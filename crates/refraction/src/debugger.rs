//! Debugger Integration helpers (v4 Section 30, `dx.debugger`).
//!
//! The compiler already produces a rich source-map (`source_map.rs`) that the
//! VS Code extension can consume, but the format the spec mandates in
//! §30.2 is the simpler line-pair shape:
//!
//! ```json
//! {
//!     "version": 1,
//!     "source": "src/Player.prsm",
//!     "generated": "Generated/Player.cs",
//!     "mappings": [
//!         { "prsmLine": 5, "csLine": 12 },
//!         { "prsmLine": 6, "csLine": 13 }
//!     ]
//! }
//! ```
//!
//! This module flattens the existing rich `SourceMapFile` into that shape,
//! exposes a stable variable rename helper for the debugger adapter, and
//! provides a `DebugAdapterInfo` struct that the editor can read on startup —
//! the actual DAP server is out of scope for this phase, but the entry point
//! is wired so an external adapter can be plugged in later without touching
//! the compiler again.

use crate::source_map::{SourceMapAnchor, SourceMapFile, SourceMapSpan};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Spec-compliant flat source map (v4 §30.2).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugSourceMap {
    pub version: u32,
    pub source: String,
    pub generated: String,
    pub mappings: Vec<DebugMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugMapping {
    #[serde(rename = "prsmLine")]
    pub prsm_line: u32,
    #[serde(rename = "csLine")]
    pub cs_line: u32,
}

/// DAP adapter discovery info written alongside generated source. The
/// extension reads this on startup so it knows which adapter to launch and
/// where to find the source maps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DebugAdapterInfo {
    pub adapter_kind: String,
    pub language: String,
    pub source_map_glob: String,
    pub generated_glob: String,
    pub step_filter_patterns: Vec<String>,
}

impl Default for DebugAdapterInfo {
    fn default() -> Self {
        DebugAdapterInfo {
            adapter_kind: "vscode-cs".into(),
            language: "prsm".into(),
            source_map_glob: "**/*.prsmmap.json".into(),
            generated_glob: "**/*.cs".into(),
            step_filter_patterns: default_step_filters(),
        }
    }
}

/// Default skip-list for §30.3.3 — compiler-generated boilerplate the user
/// should never step through.
pub fn default_step_filters() -> Vec<String> {
    vec![
        "__opt_*".into(),
        "__cached_*".into(),
        "__prev_*".into(),
        "*PoolFactory_*".into(),
        "*StateMachineDispatch".into(),
        "*Singleton.Awake".into(),
    ]
}

/// Build a flat mapping from a rich `SourceMapFile`. Each member anchor is
/// exploded into one mapping per line of its source span, paired with the
/// matching line of its generated span. When a member has no `generated_span`
/// it is silently skipped.
pub fn flatten_source_map(map: &SourceMapFile) -> DebugSourceMap {
    let mut mappings: Vec<DebugMapping> = Vec::new();
    // Issue #71: iterate every top-level type declaration. Fall back
    // to the legacy `declaration` field for source maps produced by
    // older compilers (or JSON blobs deserialized without the new
    // `declarations` field).
    if !map.declarations.is_empty() {
        for decl in &map.declarations {
            push_anchor_mappings(decl, &mut mappings);
        }
    } else if let Some(decl) = &map.declaration {
        push_anchor_mappings(decl, &mut mappings);
    }
    for member in &map.members {
        push_anchor_mappings(member, &mut mappings);
    }
    mappings.sort_by(|a, b| a.prsm_line.cmp(&b.prsm_line).then(a.cs_line.cmp(&b.cs_line)));
    mappings.dedup();

    DebugSourceMap {
        version: 1,
        source: map.source_file.to_string_lossy().to_string(),
        generated: map.generated_file.to_string_lossy().to_string(),
        mappings,
    }
}

fn push_anchor_mappings(anchor: &SourceMapAnchor, out: &mut Vec<DebugMapping>) {
    if let Some(generated) = &anchor.generated_span {
        push_pair(anchor.source_span, *generated, out);
    }
    if let Some(name_gen) = &anchor.generated_name_span {
        push_pair(anchor.source_span, *name_gen, out);
    }
    for segment in &anchor.segments {
        push_anchor_mappings(segment, out);
    }
}

fn push_pair(src: SourceMapSpan, gen: SourceMapSpan, out: &mut Vec<DebugMapping>) {
    let src_lines: Vec<u32> = (src.line..=src.end_line.max(src.line)).collect();
    let gen_lines: Vec<u32> = (gen.line..=gen.end_line.max(gen.line)).collect();
    let src_count = src_lines.len();
    let gen_count = gen_lines.len();
    if src_count == 0 || gen_count == 0 {
        out.push(DebugMapping {
            prsm_line: src.line,
            cs_line: gen.line,
        });
        return;
    }

    // Issue #73: when a one-line PrSM statement lowers to multi-line C#
    // (or vice versa), the old `min(src, gen)` stopped after the first
    // pair and left the remaining lines unreachable from the debugger.
    // Emit one mapping per generated line, repeating the last PrSM line
    // when we run out (so every generated line has a PrSM backreference).
    // Similarly, emit one mapping per source line when the source span
    // is longer than the generated span (the reverse direction).
    let pair_count = src_count.max(gen_count);
    for i in 0..pair_count {
        let src_line = src_lines.get(i).copied().unwrap_or_else(|| {
            *src_lines.last().unwrap_or(&src.line)
        });
        let cs_line = gen_lines.get(i).copied().unwrap_or_else(|| {
            *gen_lines.last().unwrap_or(&gen.line)
        });
        out.push(DebugMapping {
            prsm_line: src_line,
            cs_line,
        });
    }
}

/// Compute the path of the flat debug map next to the generated `.cs` file.
pub fn debug_map_path_for_generated(generated_file: &Path) -> PathBuf {
    let stem = generated_file
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("generated");
    generated_file.with_file_name(format!("{}.prsm.map", stem))
}

/// Reverse mapping for §30.3.2: when a debugger sees a generated identifier
/// like `_prsm_d` it can ask the compiler for the user-facing name. The
/// compiler emits a small lookup table per file.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct VariableNameTable {
    pub entries: Vec<VariableNameEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct VariableNameEntry {
    pub generated: String,
    pub original: String,
}

impl VariableNameTable {
    pub fn add(&mut self, generated: impl Into<String>, original: impl Into<String>) {
        self.entries.push(VariableNameEntry {
            generated: generated.into(),
            original: original.into(),
        });
    }

    pub fn lookup<'a>(&'a self, generated: &str) -> Option<&'a str> {
        self.entries
            .iter()
            .find(|entry| entry.generated == generated)
            .map(|entry| entry.original.as_str())
    }
}

/// Should the debugger step into a generated symbol? Returns `false` for
/// names that match any of the configured filter patterns (glob-ish: `*` is
/// the only metachar). Used by the VS Code extension via JSON-RPC.
pub fn should_step_into(symbol: &str, filters: &[String]) -> bool {
    !filters.iter().any(|pattern| glob_match(pattern, symbol))
}

fn glob_match(pattern: &str, value: &str) -> bool {
    // Tiny glob: only `*` (any-chars) and literal segments. Sufficient for
    // the small skip list shipped by default — keeps us free of an external
    // dependency.
    let parts: Vec<&str> = pattern.split('*').collect();
    let mut cursor = 0usize;
    for (i, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if i == 0 && !pattern.starts_with('*') {
            if !value[cursor..].starts_with(part) {
                return false;
            }
            cursor += part.len();
            continue;
        }
        if i == parts.len() - 1 && !pattern.ends_with('*') {
            return value[cursor..].ends_with(part);
        }
        if let Some(found) = value[cursor..].find(part) {
            cursor += found + part.len();
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::token::{Position, Span};
    use crate::source_map::{SourceMapAnchor, SourceMapFile, SourceMapSpan};

    fn span_pair(sl: u32, el: u32) -> SourceMapSpan {
        SourceMapSpan {
            line: sl,
            col: 1,
            end_line: el,
            end_col: 1,
        }
    }

    fn rich_map() -> SourceMapFile {
        let player_decl = SourceMapAnchor {
            kind: "Type".into(),
            name: "Player".into(),
            qualified_name: "Player".into(),
            source_span: span_pair(1, 30),
            generated_span: Some(span_pair(5, 90)),
            generated_name_span: None,
            segments: vec![],
        };
        SourceMapFile {
            version: 1,
            source_file: PathBuf::from("src/Player.prsm"),
            generated_file: PathBuf::from("Generated/Player.cs"),
            declaration: Some(player_decl.clone()),
            declarations: vec![player_decl],
            members: vec![SourceMapAnchor {
                kind: "Func".into(),
                name: "Update".into(),
                qualified_name: "Player.Update".into(),
                source_span: span_pair(10, 12),
                generated_span: Some(span_pair(20, 22)),
                generated_name_span: None,
                segments: vec![],
            }],
        }
    }

    #[test]
    fn flatten_source_map_emits_one_pair_per_line() {
        let map = rich_map();
        let flat = flatten_source_map(&map);
        assert_eq!(flat.version, 1);
        assert_eq!(flat.source, "src/Player.prsm");
        assert_eq!(flat.generated, "Generated/Player.cs");
        // Declaration spans 1..=30 mapped to 5..=90 → first line pair (1, 5).
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 1 && m.cs_line == 5));
        // Member spans 10..=12 mapped to 20..=22 → expect 3 pairs.
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 10 && m.cs_line == 20));
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 11 && m.cs_line == 21));
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 12 && m.cs_line == 22));
    }

    #[test]
    fn debug_map_path_uses_dot_prsm_dot_map_extension() {
        let path = debug_map_path_for_generated(&PathBuf::from("Generated/Player.cs"));
        assert!(path.to_string_lossy().ends_with("Player.prsm.map"));
    }

    #[test]
    fn variable_name_table_lookup() {
        let mut table = VariableNameTable::default();
        table.add("_prsm_d", "damage");
        table.add("__hp", "hp");
        assert_eq!(table.lookup("_prsm_d"), Some("damage"));
        assert_eq!(table.lookup("__hp"), Some("hp"));
        assert_eq!(table.lookup("missing"), None);
    }

    #[test]
    fn should_step_into_skips_optimizer_temps() {
        let filters = default_step_filters();
        assert!(!should_step_into("__opt_cached_label_text", &filters));
        assert!(!should_step_into("__opt_prev_label_text_hp", &filters));
        assert!(should_step_into("Update", &filters));
        assert!(should_step_into("ComputeDamage", &filters));
    }

    // Issue #73: push_pair must emit one mapping per generated line
    // even when the source span is shorter. This keeps breakpoints on
    // multi-line lowerings reachable.
    #[test]
    fn push_pair_emits_one_mapping_per_generated_line_when_span_shorter() {
        let src = SourceMapSpan { line: 10, col: 1, end_line: 10, end_col: 1 };
        let gen = SourceMapSpan { line: 20, col: 1, end_line: 23, end_col: 1 };
        let mut out = Vec::new();
        push_pair(src, gen, &mut out);
        assert_eq!(out.len(), 4);
        // Every generated line 20..=23 must be present.
        for cs_line in 20..=23 {
            assert!(out.iter().any(|m| m.cs_line == cs_line));
        }
        // They all map back to the single source line.
        assert!(out.iter().all(|m| m.prsm_line == 10));
    }

    // Issue #73: conversely, a multi-line source span that lowers to
    // one generated line must still emit one mapping per source line.
    #[test]
    fn push_pair_emits_one_mapping_per_source_line_when_gen_shorter() {
        let src = SourceMapSpan { line: 5, col: 1, end_line: 7, end_col: 1 };
        let gen = SourceMapSpan { line: 20, col: 1, end_line: 20, end_col: 1 };
        let mut out = Vec::new();
        push_pair(src, gen, &mut out);
        assert_eq!(out.len(), 3);
        for prsm_line in 5..=7 {
            assert!(out.iter().any(|m| m.prsm_line == prsm_line));
        }
        assert!(out.iter().all(|m| m.cs_line == 20));
    }

    // Issue #71: flatten_source_map walks every declaration in the
    // `declarations` vector, not just the primary one.
    #[test]
    fn flatten_source_map_walks_all_declarations() {
        let decl_a = SourceMapAnchor {
            kind: "Type".into(),
            name: "A".into(),
            qualified_name: "A".into(),
            source_span: span_pair(1, 3),
            generated_span: Some(span_pair(10, 12)),
            generated_name_span: None,
            segments: vec![],
        };
        let decl_b = SourceMapAnchor {
            kind: "Type".into(),
            name: "B".into(),
            qualified_name: "B".into(),
            source_span: span_pair(5, 7),
            generated_span: Some(span_pair(20, 22)),
            generated_name_span: None,
            segments: vec![],
        };
        let map = SourceMapFile {
            version: 1,
            source_file: PathBuf::from("src/Pair.prsm"),
            generated_file: PathBuf::from("Generated/Pair.cs"),
            declaration: Some(decl_a.clone()),
            declarations: vec![decl_a, decl_b],
            members: vec![],
        };
        let flat = flatten_source_map(&map);
        // Both declaration's source→generated pairs should appear.
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 1 && m.cs_line == 10));
        assert!(flat.mappings.iter().any(|m| m.prsm_line == 5 && m.cs_line == 20));
    }

    #[test]
    fn glob_match_basic_patterns() {
        assert!(glob_match("__opt_*", "__opt_cached_x"));
        assert!(glob_match("*Singleton.Awake", "PlayerSingleton.Awake"));
        assert!(glob_match("*StateMachineDispatch", "FooStateMachineDispatch"));
        assert!(!glob_match("__opt_*", "Update"));
    }

    #[test]
    fn debug_adapter_info_default_has_filters() {
        let info = DebugAdapterInfo::default();
        assert_eq!(info.adapter_kind, "vscode-cs");
        assert!(info.step_filter_patterns.iter().any(|p| p == "__opt_*"));
        assert!(!info.source_map_glob.is_empty());
    }

    #[test]
    fn flat_map_serializes_to_spec_keys() {
        let map = rich_map();
        let flat = flatten_source_map(&map);
        let json = serde_json::to_string(&flat).expect("serialize");
        assert!(json.contains("\"prsmLine\""));
        assert!(json.contains("\"csLine\""));
        assert!(json.contains("\"version\":1"));
    }

    fn _unused_span_helper() -> Span {
        Span {
            start: Position { line: 1, col: 1 },
            end: Position { line: 1, col: 1 },
        }
    }
}
