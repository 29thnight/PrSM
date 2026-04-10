use crate::hir::{HirDefinition, HirDefinitionKind, HirFile};
use crate::lexer::token::Span;
use crate::lowering::csharp_ir::{CsFile, CsMember, CsStmt};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceMapFile {
    pub version: u32,
    pub source_file: PathBuf,
    pub generated_file: PathBuf,
    /// The primary top-level type declaration. Preserved for backward
    /// compatibility with existing consumers; new code should read
    /// `declarations` to correctly handle multi-type files.
    pub declaration: Option<SourceMapAnchor>,
    /// Issue #71: every top-level type declared in this file gets its
    /// own anchor. For a single-decl file this is a 1-element vector
    /// containing the same anchor as `declaration`. For a multi-type
    /// file (e.g. `enum EnemyState` + `component EnemyAI` in one
    /// source) each declaration lands here, so stack traces can
    /// resolve the class header for any of them and the Unity
    /// importer can bind the MonoScript to the right type.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub declarations: Vec<SourceMapAnchor>,
    pub members: Vec<SourceMapAnchor>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct SourceMapAnchor {
    pub kind: String,
    pub name: String,
    pub qualified_name: String,
    pub source_span: SourceMapSpan,
    pub generated_span: Option<SourceMapSpan>,
    pub generated_name_span: Option<SourceMapSpan>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub segments: Vec<SourceMapAnchor>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
pub struct SourceMapSpan {
    pub line: u32,
    pub col: u32,
    pub end_line: u32,
    pub end_col: u32,
}

pub fn source_map_path_for_generated(generated_file: &Path) -> PathBuf {
    let stem = generated_file
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("generated");
    generated_file.with_file_name(format!("{}.prsmmap.json", stem))
}

pub fn write_source_map(
    hir_file: &HirFile,
    generated_ir: &CsFile,
    generated_file: &Path,
    generated_source: &str,
) -> Result<PathBuf, String> {
    let path = source_map_path_for_generated(generated_file);
    let map = build_source_map(hir_file, generated_ir, generated_file, generated_source);
    let json = serde_json::to_string_pretty(&map)
        .map_err(|error| format!("Cannot serialize source map {}: {}", path.display(), error))?;
    fs::write(&path, json)
        .map_err(|error| format!("Cannot write source map {}: {}", path.display(), error))?;
    Ok(path)
}

pub fn build_source_map(
    hir_file: &HirFile,
    generated_ir: &CsFile,
    generated_file: &Path,
    generated_source: &str,
) -> SourceMapFile {
    let lines = generated_source.lines().collect::<Vec<_>>();
    let mut definitions = hir_file
        .definitions
        .iter()
        .filter(|definition| is_anchor_kind(definition.kind))
        .collect::<Vec<_>>();
    definitions.sort_by(|left, right| {
        left.span
            .start
            .line
            .cmp(&right.span.start.line)
            .then(left.span.start.col.cmp(&right.span.start.col))
            .then(left.qualified_name.cmp(&right.qualified_name))
    });

    // Issue #71: collect every top-level type declaration, not just the
    // first one. Real shipped samples (e.g. `EnemyState.prsm`) declare
    // both `enum EnemyState` and `component EnemyAI` in the same file,
    // and the generated `.cs` contains both types. The old code only
    // kept the first anchor, so the second declaration was lost from
    // the source map and the Unity importer bound the MonoScript to
    // the wrong type.
    let type_definitions: Vec<&HirDefinition> = definitions
        .iter()
        .copied()
        .filter(|definition| definition.kind == HirDefinitionKind::Type)
        .collect();
    let member_definitions = definitions
        .into_iter()
        .filter(|definition| definition.kind != HirDefinitionKind::Type)
        .collect::<Vec<_>>();

    let declarations_with_anchors: Vec<(&HirDefinition, Option<GeneratedAnchor>)> = type_definitions
        .iter()
        .copied()
        .map(|definition| (definition, find_declaration_anchor(&lines, definition)))
        .collect();

    // Primary declaration — the first (by sort order) top-level type.
    // `declaration` is kept for backward compatibility with older
    // consumers that only read a single anchor.
    let primary = declarations_with_anchors.first();
    let class_header_line = primary
        .and_then(|(_, anchor)| anchor.as_ref())
        .map(|anchor| anchor.header_line)
        .unwrap_or(1);
    let class_end_line = primary
        .and_then(|(_, anchor)| anchor.as_ref())
        .map(|anchor| anchor.end_line)
        .unwrap_or(lines.len() as u32);

    let declaration = primary.map(|(definition, generated)| SourceMapAnchor {
        kind: definition.kind.as_str().to_string(),
        name: definition.name.clone(),
        qualified_name: definition.qualified_name.clone(),
        source_span: SourceMapSpan::from_span(definition.span),
        generated_span: generated.as_ref().map(|anchor| anchor.generated_span),
        generated_name_span: generated.as_ref().map(|anchor| anchor.generated_name_span),
        segments: Vec::new(),
    });

    let declarations: Vec<SourceMapAnchor> = declarations_with_anchors
        .iter()
        .map(|(definition, generated)| SourceMapAnchor {
            kind: definition.kind.as_str().to_string(),
            name: definition.name.clone(),
            qualified_name: definition.qualified_name.clone(),
            source_span: SourceMapSpan::from_span(definition.span),
            generated_span: generated.as_ref().map(|anchor| anchor.generated_span),
            generated_name_span: generated.as_ref().map(|anchor| anchor.generated_name_span),
            segments: Vec::new(),
        })
        .collect();

    let mut found_members = Vec::with_capacity(member_definitions.len());
    let mut search_from_line = class_header_line.saturating_add(1);
    for definition in &member_definitions {
        let found = find_member_anchor(&lines, definition, search_from_line, class_end_line);
        if let Some(anchor) = found {
            search_from_line = anchor.header_line.saturating_add(1);
        }
        found_members.push(found);
    }

    let members = member_definitions
        .into_iter()
        .enumerate()
        .map(|(index, definition)| {
            let generated = found_members[index].as_ref().map(|anchor| {
                let next_header_start = found_members[index + 1..]
                    .iter()
                    .flatten()
                    .map(|candidate| candidate.start_line)
                    .next();
                let raw_end_line = next_header_start
                    .map(|line| line.saturating_sub(1))
                    .unwrap_or_else(|| class_end_line.saturating_sub(1).max(anchor.start_line));
                let end_line = find_previous_content_line(&lines, raw_end_line, anchor.start_line);
                GeneratedAnchor {
                    generated_span: SourceMapSpan {
                        line: anchor.start_line,
                        col: 1,
                        end_line,
                        end_col: line_end_col(lines.get(end_line.saturating_sub(1) as usize).copied()),
                    },
                    generated_name_span: anchor.generated_name_span,
                    start_line: anchor.start_line,
                    header_line: anchor.header_line,
                    end_line,
                }
            });

            SourceMapAnchor {
                kind: definition.kind.as_str().to_string(),
                name: definition.name.clone(),
                qualified_name: definition.qualified_name.clone(),
                source_span: SourceMapSpan::from_span(definition.span),
                generated_span: generated.as_ref().map(|anchor| anchor.generated_span),
                generated_name_span: generated.as_ref().map(|anchor| anchor.generated_name_span),
                segments: generated
                    .as_ref()
                    .map(|anchor| build_member_segments(lines.as_slice(), generated_ir, definition, anchor))
                    .unwrap_or_default(),
            }
        })
        .collect();

    SourceMapFile {
        version: 1,
        source_file: hir_file.path.clone(),
        generated_file: generated_file.to_path_buf(),
        declaration,
        declarations,
        members,
    }
}

fn build_member_segments(
    lines: &[&str],
    generated_ir: &CsFile,
    definition: &HirDefinition,
    generated_anchor: &GeneratedAnchor,
) -> Vec<SourceMapAnchor> {
    let Some(CsMember::Method { body, .. }) = find_method_member(generated_ir, &generated_member_name(definition)) else {
        return Vec::new();
    };

    let mut next_segment_id = 0u32;
    let (segments, _) = collect_statement_segments(
        lines,
        body,
        generated_anchor.header_line.saturating_add(2),
        &definition.qualified_name,
        &mut next_segment_id,
    );
    segments
}

fn find_method_member<'a>(generated_ir: &'a CsFile, generated_name: &str) -> Option<&'a CsMember> {
    generated_ir
        .class
        .members
        .iter()
        .find(|member| matches!(member, CsMember::Method { name, .. } if name == generated_name))
}

fn collect_statement_segments(
    lines: &[&str],
    statements: &[CsStmt],
    start_line: u32,
    parent_qualified_name: &str,
    next_segment_id: &mut u32,
) -> (Vec<SourceMapAnchor>, u32) {
    let mut segments = Vec::new();
    let mut current_line = start_line;

    for statement in statements {
        let (segment, next_line) = build_statement_segment(
            lines,
            statement,
            current_line,
            parent_qualified_name,
            next_segment_id,
        );
        if let Some(segment) = segment {
            segments.push(segment);
        }
        current_line = next_line;
    }

    (segments, current_line)
}

fn build_statement_segment(
    lines: &[&str],
    statement: &CsStmt,
    start_line: u32,
    parent_qualified_name: &str,
    next_segment_id: &mut u32,
) -> (Option<SourceMapAnchor>, u32) {
    let (children, next_line) = match statement {
        CsStmt::If { then_body, else_body, .. } => {
            let (mut children, after_then_body) = collect_statement_segments(
                lines,
                then_body,
                start_line.saturating_add(2),
                parent_qualified_name,
                next_segment_id,
            );
            let mut next_line = after_then_body.saturating_add(1);

            if let Some(else_body) = else_body {
                let (else_children, after_else_body) = collect_statement_segments(
                    lines,
                    else_body,
                    after_then_body.saturating_add(3),
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(else_children);
                next_line = after_else_body.saturating_add(1);
            }

            (children, next_line)
        }
        CsStmt::Switch { cases, .. } => {
            let mut children = Vec::new();
            let mut line = start_line.saturating_add(2);
            for case in cases {
                let case_body_start = line.saturating_add(1);
                let (case_children, after_case_body) = collect_statement_segments(
                    lines,
                    &case.body,
                    case_body_start,
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(case_children);
                line = after_case_body;
            }
            (children, line.saturating_add(1))
        }
        CsStmt::For { body, .. } | CsStmt::ForEach { body, .. } | CsStmt::While { body, .. } => {
            let (children, after_body) = collect_statement_segments(
                lines,
                body,
                start_line.saturating_add(2),
                parent_qualified_name,
                next_segment_id,
            );
            (children, after_body.saturating_add(1))
        }
        CsStmt::Block(statements, ..) => {
            collect_statement_segments(lines, statements, start_line, parent_qualified_name, next_segment_id)
        }
        CsStmt::Raw(code, ..) => (Vec::new(), start_line.saturating_add(raw_line_count(code))),
        // Issue #72: `try { ... } catch (Ex e) { ... } finally { ... }` —
        // a 12-line block that the old code treated as a single line,
        // skewing every subsequent statement downward. Recurse into each
        // arm and add the brace/keyword lines between them.
        CsStmt::TryCatch { try_body, catches, finally_body, .. } => {
            // `try` header (`try {`) occupies 1 line; the body starts
            // on the next.
            let (mut children, after_try) = collect_statement_segments(
                lines,
                try_body,
                start_line.saturating_add(2),
                parent_qualified_name,
                next_segment_id,
            );
            // Closing brace line of the try block, then each catch.
            let mut next_line = after_try.saturating_add(1);
            for catch in catches {
                // `} catch (...) {` on one line, body starts next.
                next_line = next_line.saturating_add(1);
                let (catch_children, after_catch) = collect_statement_segments(
                    lines,
                    &catch.body,
                    next_line,
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(catch_children);
                next_line = after_catch.saturating_add(1);
            }
            if let Some(finally) = finally_body {
                // `} finally {` on one line.
                next_line = next_line.saturating_add(1);
                let (finally_children, after_finally) = collect_statement_segments(
                    lines,
                    finally,
                    next_line,
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(finally_children);
                next_line = after_finally.saturating_add(1);
            }
            (children, next_line)
        }
        // Issue #72: `using (var x = ...) { body }` — header + body +
        // closing brace.
        CsStmt::UseBlock { body, .. } => {
            let (children, after_body) = collect_statement_segments(
                lines,
                body,
                start_line.saturating_add(2),
                parent_qualified_name,
                next_segment_id,
            );
            (children, after_body.saturating_add(1))
        }
        // Issue #72: `#if COND` / `#elif` / `#else` / `#endif`. Each
        // directive is a single line, with bodies in between.
        CsStmt::Preprocessor { arms, else_body, .. } => {
            let mut children: Vec<SourceMapAnchor> = Vec::new();
            let mut next_line = start_line.saturating_add(1);
            for (idx, arm) in arms.iter().enumerate() {
                if idx > 0 {
                    // `#elif` directive on its own line.
                    next_line = next_line.saturating_add(1);
                }
                let (arm_children, after_arm) = collect_statement_segments(
                    lines,
                    &arm.body,
                    next_line,
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(arm_children);
                next_line = after_arm;
            }
            if let Some(else_stmts) = else_body {
                next_line = next_line.saturating_add(1); // `#else`
                let (else_children, after_else) = collect_statement_segments(
                    lines,
                    else_stmts,
                    next_line,
                    parent_qualified_name,
                    next_segment_id,
                );
                children.extend(else_children);
                next_line = after_else;
            }
            // `#endif` directive.
            (children, next_line.saturating_add(1))
        }
        _ => (Vec::new(), start_line.saturating_add(inline_statement_line_count(statement))),
    };

    let Some(source_span) = statement_source_span(statement) else {
        return (None, next_line);
    };

    *next_segment_id += 1;
    let segment_name = format!("stmt{}", next_segment_id);
    let end_line = next_line.saturating_sub(1).max(start_line);
    let generated_span = SourceMapSpan {
        line: start_line,
        col: 1,
        end_line,
        end_col: line_end_col(lines.get(end_line.saturating_sub(1) as usize).copied()),
    };

    (
        Some(SourceMapAnchor {
            kind: "statement".to_string(),
            name: segment_name.clone(),
            qualified_name: format!("{}#{}", parent_qualified_name, segment_name),
            source_span: SourceMapSpan::from_span(source_span),
            generated_span: Some(generated_span),
            generated_name_span: None,
            segments: children,
        }),
        next_line,
    )
}

fn raw_line_count(code: &str) -> u32 {
    code.lines().count().max(1) as u32
}

fn inline_statement_line_count(statement: &CsStmt) -> u32 {
    match statement {
        CsStmt::VarDecl { init, .. } => rendered_line_count(init),
        CsStmt::Assignment { value, .. } => rendered_line_count(value),
        CsStmt::Expr(expr, _) => rendered_line_count(expr),
        CsStmt::Return(Some(value), _) => rendered_line_count(value),
        CsStmt::YieldReturn(value, _) => rendered_line_count(value),
        // Issue #72: `throw expr;` on a single line.
        CsStmt::Throw(value, _) => rendered_line_count(value),
        // Bare statements occupying exactly one line — `break;`,
        // `continue;`, `yield break;`, bare `return;`.
        CsStmt::Break(_)
        | CsStmt::Continue(_)
        | CsStmt::YieldBreak(_)
        | CsStmt::Return(None, _) => 1,
        _ => 1,
    }
}

fn rendered_line_count(rendered: &str) -> u32 {
    rendered.lines().count().max(1) as u32
}

fn statement_source_span(statement: &CsStmt) -> Option<Span> {
    match statement {
        CsStmt::VarDecl { source_span, .. }
        | CsStmt::Assignment { source_span, .. }
        | CsStmt::If { source_span, .. }
        | CsStmt::Switch { source_span, .. }
        | CsStmt::For { source_span, .. }
        | CsStmt::ForEach { source_span, .. }
        | CsStmt::While { source_span, .. }
        | CsStmt::UseBlock { source_span, .. }
        | CsStmt::Preprocessor { source_span, .. } => *source_span,
        CsStmt::Expr(_, source_span)
        | CsStmt::Return(_, source_span)
        | CsStmt::YieldReturn(_, source_span)
        | CsStmt::YieldBreak(source_span)
        | CsStmt::Break(source_span)
        | CsStmt::Continue(source_span)
        | CsStmt::Raw(_, source_span)
        | CsStmt::Block(_, source_span)
        | CsStmt::TryCatch { source_span, .. }
        | CsStmt::Throw(_, source_span) => *source_span,
    }
}

fn is_anchor_kind(kind: HirDefinitionKind) -> bool {
    matches!(
        kind,
        HirDefinitionKind::Type
            | HirDefinitionKind::Field
            | HirDefinitionKind::Function
            | HirDefinitionKind::Coroutine
            | HirDefinitionKind::Lifecycle
            | HirDefinitionKind::EnumEntry
    )
}

fn find_declaration_anchor(lines: &[&str], definition: &HirDefinition) -> Option<GeneratedAnchor> {
    // Issue #70: try each candidate generated name in turn. For an
    // attribute declaration we need to match both `Foo` and
    // `FooAttribute` because the lowering auto-suffixes attribute
    // classes with `Attribute` when the user omits the suffix.
    for generated_name in generated_type_name_candidates(definition) {
        if let Some(anchor) = find_declaration_anchor_for_name(lines, &generated_name) {
            return Some(anchor);
        }
    }
    None
}

fn find_declaration_anchor_for_name(lines: &[&str], generated_name: &str) -> Option<GeneratedAnchor> {
    for (index, line) in lines.iter().enumerate() {
        let header_pattern = [
            format!("class {}", generated_name),
            format!("enum {}", generated_name),
            format!("struct {}", generated_name),
        ]
        .into_iter()
        .find(|pattern| line.contains(pattern));

        if header_pattern.is_none() {
            continue;
        }

        let header_line = (index + 1) as u32;
        let start_line = include_attribute_lines(lines, header_line, 1);
        let end_line = find_top_level_closing_line(lines, header_line).unwrap_or(lines.len() as u32);
        let name_col = line.find(generated_name).map(|value| value as u32 + 1)?;
        let name_end_col = name_col + generated_name.chars().count() as u32 - 1;

        return Some(GeneratedAnchor {
            generated_span: SourceMapSpan {
                line: start_line,
                col: 1,
                end_line,
                end_col: line_end_col(lines.get(end_line.saturating_sub(1) as usize).copied()),
            },
            generated_name_span: SourceMapSpan {
                line: header_line,
                col: name_col,
                end_line: header_line,
                end_col: name_end_col,
            },
            start_line,
            header_line,
            end_line,
        });
    }

    None
}

fn find_member_anchor(
    lines: &[&str],
    definition: &HirDefinition,
    start_line: u32,
    class_end_line: u32,
) -> Option<GeneratedAnchor> {
    let generated_name = generated_member_name(definition);

    for line_index in start_line.max(1)..=class_end_line {
        let Some(line) = lines.get(line_index.saturating_sub(1) as usize).copied() else {
            break;
        };

        let name_col = match definition.kind {
            HirDefinitionKind::Field => find_field_name_col(lines, line_index, &generated_name),
            HirDefinitionKind::EnumEntry => find_enum_entry_name_col(line, &generated_name),
            HirDefinitionKind::Function | HirDefinitionKind::Coroutine | HirDefinitionKind::Lifecycle => {
                find_method_name_col(lines, line_index, &generated_name, class_end_line)
            }
            _ => None,
        };

        let Some(name_col) = name_col else {
            continue;
        };

        let header_line = line_index;
        // Issue #74: for serialized-field properties the attribute
        // lives above the backing field, not the property header.
        // Use the extended walk for field anchors so `[SerializeField]`
        // ends up inside the anchor span.
        let start_line = if definition.kind == HirDefinitionKind::Field {
            include_serialize_field_attribute_lines(lines, header_line, &generated_name, start_line)
        } else {
            include_attribute_lines(lines, header_line, start_line)
        };
        let name_end_col = name_col + generated_name.chars().count() as u32 - 1;
        return Some(GeneratedAnchor {
            generated_span: SourceMapSpan {
                line: start_line,
                col: 1,
                end_line: header_line,
                end_col: line_end_col(Some(line)),
            },
            generated_name_span: SourceMapSpan {
                line: header_line,
                col: name_col,
                end_line: header_line,
                end_col: name_end_col,
            },
            start_line,
            header_line,
            end_line: header_line,
        });
    }

    None
}

/// Issue #70: for `attribute Foo(...)`, the compiler lowers to
/// `class FooAttribute : System.Attribute` when the user omits the
/// suffix. Return every candidate generated name so `find_declaration_anchor`
/// can search for each in turn. The original implementation used
/// `.next()` on a two-element array which always yielded the first
/// element, making the fallback dead code.
fn generated_type_name_candidates(definition: &HirDefinition) -> Vec<String> {
    if definition.name.ends_with("Attribute") {
        return vec![definition.name.clone()];
    }
    vec![
        definition.name.clone(),
        format!("{}Attribute", definition.name),
    ]
}

fn generated_member_name(definition: &HirDefinition) -> String {
    match definition.kind {
        HirDefinitionKind::Function | HirDefinitionKind::Coroutine => pascal_case(&definition.name),
        HirDefinitionKind::Lifecycle => match definition.name.as_str() {
            "awake" => "Awake".into(),
            "start" => "Start".into(),
            "update" => "Update".into(),
            "fixedUpdate" => "FixedUpdate".into(),
            "lateUpdate" => "LateUpdate".into(),
            "onEnable" => "OnEnable".into(),
            "onDisable" => "OnDisable".into(),
            "onDestroy" => "OnDestroy".into(),
            "onTriggerEnter" => "OnTriggerEnter".into(),
            "onTriggerExit" => "OnTriggerExit".into(),
            "onTriggerStay" => "OnTriggerStay".into(),
            "onCollisionEnter" => "OnCollisionEnter".into(),
            "onCollisionExit" => "OnCollisionExit".into(),
            "onCollisionStay" => "OnCollisionStay".into(),
            _ => definition.name.clone(),
        },
        _ => definition.name.clone(),
    }
}

fn pascal_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        None => String::new(),
        Some(c) => c.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

fn include_attribute_lines(lines: &[&str], header_line: u32, lower_bound: u32) -> u32 {
    let mut start = header_line;
    while start > lower_bound {
        let previous = lines
            .get(start.saturating_sub(2) as usize)
            .copied()
            .unwrap_or_default()
            .trim_start();
        if previous.starts_with('[') {
            start -= 1;
        } else {
            break;
        }
    }
    start
}

/// Issue #74: extended attribute-inclusion walk for serialize-field
/// property anchors. The lowering emits the `[SerializeField]`
/// attribute on a backing field *above* the public property, so a
/// plain `include_attribute_lines` walk from the property header sees
/// `private float _speed = 1.0f;` (not `[`-starting) and stops. We
/// detect that pattern and continue walking past the backing field,
/// picking up any attribute line that sits above it.
fn include_serialize_field_attribute_lines(
    lines: &[&str],
    header_line: u32,
    name: &str,
    lower_bound: u32,
) -> u32 {
    // First, consume any attribute lines directly above the property.
    let mut start = include_attribute_lines(lines, header_line, lower_bound);
    // If the line above `start` looks like a backing field for this
    // property (`private <type> _<name>`), include it and keep walking
    // up for more attributes.
    loop {
        if start <= lower_bound {
            break;
        }
        let prev_line = lines
            .get(start.saturating_sub(2) as usize)
            .copied()
            .unwrap_or_default();
        let trimmed = prev_line.trim_start();
        let backing_name_patterns = [
            format!(" _{} =", name),
            format!(" _{};", name),
        ];
        let looks_like_backing = trimmed.starts_with("private ")
            && backing_name_patterns
                .iter()
                .any(|p| prev_line.contains(p.as_str()));
        if !looks_like_backing {
            break;
        }
        // Consume the backing field line.
        start -= 1;
        // Continue up past any attributes attached to the backing field.
        start = include_attribute_lines(lines, start, lower_bound);
    }
    start
}

fn find_top_level_closing_line(lines: &[&str], header_line: u32) -> Option<u32> {
    let mut depth = 0u32;
    let mut saw_open_brace = false;

    for line_index in header_line..=lines.len() as u32 {
        let line = lines.get(line_index.saturating_sub(1) as usize)?;
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    saw_open_brace = true;
                }
                '}' => {
                    if depth == 0 {
                        continue;
                    }
                    depth -= 1;
                    if saw_open_brace && depth == 0 {
                        return Some(line_index);
                    }
                }
                _ => {}
            }
        }
    }

    None
}

fn find_method_name_col(lines: &[&str], line_index: u32, name: &str, class_end_line: u32) -> Option<u32> {
    let line = lines.get(line_index.saturating_sub(1) as usize)?.trim_end();
    // Skip attribute-only lines so the caller keeps walking forward.
    if line.trim_start().starts_with('[') {
        return None;
    }
    let pattern = format!("{}(", name);
    if !line.contains(&pattern) {
        return None;
    }
    // Issue #72: an expression-bodied method ends in `;` on the same
    // line (`public int f() => 42;`). The old check `line.ends_with(';')`
    // rejected every expression-bodied method silently. Accept the
    // header when the same line contains `=>` — that's the expression-
    // body marker.
    let is_expr_body_same_line = line.contains("=>") && line.ends_with(';');
    if line.ends_with(';') && !is_expr_body_same_line {
        return None;
    }

    // Accept one of three common header shapes:
    //   - `public void f() { ... }`                      (brace next line)
    //   - `public int f() => 42;`                        (expression body, same line)
    //   - `public int f() =>\n    42;`                   (expression body, next line)
    // For the brace case the next non-empty line must be `{`; for the
    // expression-body case it starts with `=>` or the current line
    // already contains it.
    let next = next_non_empty_line(lines, line_index.saturating_add(1), class_end_line);
    let next_opens_body = next.map(|value| value.trim()) == Some("{");
    let next_starts_expr_body = next
        .map(|value| value.trim_start().starts_with("=>"))
        .unwrap_or(false);

    if !next_opens_body && !is_expr_body_same_line && !next_starts_expr_body {
        return None;
    }

    lines[line_index.saturating_sub(1) as usize]
        .find(&pattern)
        .map(|value| value as u32 + 1)
}

fn find_field_name_col(lines: &[&str], line_index: u32, name: &str) -> Option<u32> {
    let line = lines.get(line_index.saturating_sub(1) as usize)?.trim_end();
    if line.trim_start().starts_with('[') || line.trim() == "{" || line.trim() == "}" {
        return None;
    }

    let anchored_patterns = [
        format!(" {} =", name),
        format!(" {} =>", name),
        format!(" {};", name),
        format!(" {}\r", name),
    ];
    for pattern in anchored_patterns {
        if let Some(index) = lines[line_index.saturating_sub(1) as usize].find(&pattern) {
            return Some(index as u32 + 2);
        }
    }

    let property_pattern = format!(" {}", name);
    if !line.contains(&format!("{}(", name))
        && line.contains(&property_pattern)
        && next_non_empty_line(lines, line_index.saturating_add(1), line_index.saturating_add(2))
            .map(|next| next.trim() == "{")
            .unwrap_or(false)
    {
        return lines[line_index.saturating_sub(1) as usize]
            .find(&property_pattern)
            .map(|value| value as u32 + 2);
    }

    None
}

fn find_enum_entry_name_col(line: &str, name: &str) -> Option<u32> {
    let trimmed = line.trim_start();
    if trimmed.starts_with(&format!("{},", name)) {
        let indent = (line.len() - trimmed.len()) as u32;
        return Some(indent + 1);
    }
    None
}

fn next_non_empty_line<'a>(lines: &'a [&'a str], start_line: u32, end_line: u32) -> Option<&'a str> {
    for line_index in start_line..=end_line.min(lines.len() as u32) {
        let line = lines.get(line_index.saturating_sub(1) as usize)?;
        if !line.trim().is_empty() {
            return Some(*line);
        }
    }
    None
}

fn find_previous_content_line(lines: &[&str], mut line_index: u32, minimum: u32) -> u32 {
    line_index = line_index.min(lines.len() as u32);
    while line_index > minimum {
        if let Some(line) = lines.get(line_index.saturating_sub(1) as usize) {
            if !line.trim().is_empty() {
                return line_index;
            }
        }
        line_index -= 1;
    }
    minimum
}

fn line_end_col(line: Option<&str>) -> u32 {
    line.map(|value| value.chars().count() as u32)
        .filter(|value| *value > 0)
        .unwrap_or(1)
}

impl SourceMapSpan {
    fn from_span(span: Span) -> Self {
        Self {
            line: span.start.line,
            col: span.start.col,
            end_line: span.end.line,
            end_col: span.end.col,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct GeneratedAnchor {
    generated_span: SourceMapSpan,
    generated_name_span: SourceMapSpan,
    start_line: u32,
    header_line: u32,
    end_line: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hir::{HirDefinition, HirDefinitionKind, HirFile};
    use crate::lexer::token::{Position, Span};
    use crate::lowering::csharp_ir::{CsClass, CsFile, CsMember, CsStmt};
    use crate::semantic::types::{PrismType, PrimitiveKind};

    #[test]
    fn source_map_path_uses_prsmmap_extension() {
        let path = source_map_path_for_generated(Path::new("Generated/Player.cs"));
        assert_eq!(path, PathBuf::from("Generated/Player.prsmmap.json"));
    }

    #[test]
    fn build_source_map_tracks_declaration_and_members() {
        let hir_file = HirFile {
            path: PathBuf::from("Assets/Player.prsm"),
            definitions: vec![
                definition(1, "Player", "Player", HirDefinitionKind::Type, span(1, 11, 1, 16)),
                definition(2, "speed", "Player.speed", HirDefinitionKind::Field, span(2, 15, 2, 19)),
                definition(3, "update", "Player.update", HirDefinitionKind::Lifecycle, span(4, 5, 4, 10)),
                definition(4, "jump", "Player.jump", HirDefinitionKind::Function, span(8, 10, 8, 13)),
            ],
            references: vec![],
            pattern_bindings: vec![],
            listen_sites: vec![],
        };

        let generated = r#"// <auto-generated>
// This file was generated by the refraction compiler. Do not edit manually.
// </auto-generated>

using UnityEngine;

public class Player : MonoBehaviour
{
    [SerializeField]
    private float _speed = 5.0f;
    public float speed
    {
        get => _speed;
        set => _speed = value;
    }

    private void Update()
    {
        Debug.Log(speed);
    }

    public void Jump()
    {
    }
}
"#;

        let map = build_source_map(&hir_file, &generated_ir(), Path::new("Generated/Player.cs"), generated);
        assert_eq!(map.version, 1);
        assert_eq!(map.declaration.as_ref().map(|anchor| anchor.name.as_str()), Some("Player"));
        assert_eq!(map.declaration.as_ref().and_then(|anchor| anchor.generated_name_span).map(|span| span.line), Some(7));
        assert_eq!(map.members.len(), 3);
        assert_eq!(map.members[0].name, "speed");
        assert_eq!(map.members[0].generated_name_span.map(|span| span.line), Some(11));
        assert_eq!(map.members[1].name, "update");
        assert_eq!(map.members[1].generated_name_span.map(|span| span.line), Some(17));
        assert_eq!(map.members[1].generated_name_span.map(|span| span.col), Some(18));
        assert_eq!(map.members[1].segments.len(), 1);
        assert_eq!(map.members[1].segments[0].source_span.line, 5);
        assert_eq!(map.members[1].segments[0].generated_span.map(|span| span.line), Some(19));
        assert_eq!(map.members[2].name, "jump");
        assert_eq!(map.members[2].generated_name_span.map(|span| span.line), Some(22));
    }

    fn generated_ir() -> CsFile {
        CsFile {
            header_comment: "// <auto-generated>".to_string(),
            usings: vec!["UnityEngine".to_string()],
            class: CsClass {
                attributes: vec![],
                modifiers: "public".to_string(),
                name: "Player".to_string(),
                base_class: Some("MonoBehaviour".to_string()),
                interfaces: vec![],
                where_clauses: vec![],
                members: vec![
                    CsMember::Field {
                        attributes: vec!["[SerializeField]".to_string()],
                        modifiers: "private".to_string(),
                        ty: "float".to_string(),
                        name: "_speed".to_string(),
                        init: Some("5.0f".to_string()),
                    },
                    CsMember::Property {
                        modifiers: "public".to_string(),
                        ty: "float".to_string(),
                        name: "speed".to_string(),
                        getter_expr: "_speed".to_string(),
                        setter: Some("set".to_string()),
                        setter_expr: Some("_speed".to_string()),
                    },
                    CsMember::Method {
                        attributes: vec![],
                        modifiers: "private".to_string(),
                        return_ty: "void".to_string(),
                        name: "Update".to_string(),
                        params: vec![],
                        where_clauses: vec![],
                        body: vec![CsStmt::Expr("Debug.Log(speed)".to_string(), Some(span(5, 9, 5, 24)))],
                        source_span: None,
                    },
                    CsMember::Method {
                        attributes: vec![],
                        modifiers: "public".to_string(),
                        return_ty: "void".to_string(),
                        name: "Jump".to_string(),
                        params: vec![],
                        where_clauses: vec![],
                        body: vec![],
                        source_span: None,
                    },
                ],
            },
            extra_types: vec![],
        }
    }

    fn definition(
        id: u32,
        name: &str,
        qualified_name: &str,
        kind: HirDefinitionKind,
        span: Span,
    ) -> HirDefinition {
        HirDefinition {
            id,
            name: name.to_string(),
            qualified_name: qualified_name.to_string(),
            kind,
            ty: PrismType::Primitive(PrimitiveKind::Int),
            mutable: false,
            file_path: PathBuf::from("Assets/Player.prsm"),
            span,
        }
    }

    fn span(line: u32, col: u32, end_line: u32, end_col: u32) -> Span {
        Span {
            start: Position { line, col },
            end: Position { line: end_line, col: end_col },
        }
    }

    // Issue #70: `attribute Foo(...)` lowers to `class FooAttribute :
    // System.Attribute`. The anchor walker must find the declaration
    // even though the PrSM name is `Foo`.
    #[test]
    fn generated_type_name_candidates_include_attribute_suffix() {
        let def_plain = HirDefinition {
            id: 1,
            name: "MyAttribute".into(),
            qualified_name: "MyAttribute".into(),
            kind: HirDefinitionKind::Type,
            ty: PrismType::Primitive(PrimitiveKind::Int),
            mutable: false,
            file_path: PathBuf::from("Assets/MyAttribute.prsm"),
            span: span(1, 1, 1, 5),
        };
        assert_eq!(
            generated_type_name_candidates(&def_plain),
            vec!["MyAttribute".to_string()]
        );

        let def_bare = HirDefinition {
            id: 2,
            name: "Foo".into(),
            qualified_name: "Foo".into(),
            kind: HirDefinitionKind::Type,
            ty: PrismType::Primitive(PrimitiveKind::Int),
            mutable: false,
            file_path: PathBuf::from("Assets/Foo.prsm"),
            span: span(1, 1, 1, 5),
        };
        assert_eq!(
            generated_type_name_candidates(&def_bare),
            vec!["Foo".to_string(), "FooAttribute".to_string()]
        );
    }

    // Issue #71: a file declaring multiple top-level types must
    // emit one anchor per declaration, not just the first.
    #[test]
    fn build_source_map_captures_multiple_type_declarations() {
        let hir_file = HirFile {
            path: PathBuf::from("Assets/EnemyState.prsm"),
            definitions: vec![
                definition(1, "EnemyState", "EnemyState", HirDefinitionKind::Type, span(1, 6, 1, 16)),
                definition(2, "EnemyAI", "EnemyAI", HirDefinitionKind::Type, span(5, 11, 5, 18)),
            ],
            references: vec![],
            pattern_bindings: vec![],
            listen_sites: vec![],
        };
        let generated = r#"// <auto-generated>
using UnityEngine;

public enum EnemyState
{
    Idle,
    Chase,
}

public class EnemyAI : MonoBehaviour
{
}
"#;
        // For the test we reuse the Player IR shape; only the
        // declarations field of SourceMapFile is under test.
        let map = build_source_map(&hir_file, &generated_ir(), Path::new("Generated/EnemyState.cs"), generated);
        assert_eq!(map.declarations.len(), 2);
        assert_eq!(map.declarations[0].name, "EnemyState");
        assert_eq!(map.declarations[1].name, "EnemyAI");
        // The legacy `declaration` field still points at the first
        // declaration for backward compatibility.
        assert_eq!(map.declaration.as_ref().map(|d| d.name.as_str()), Some("EnemyState"));
    }

    // Issue #74: the serialize-field attribute lives above the
    // backing field, not the public property. The extended walk
    // must include both in the anchor span.
    #[test]
    fn serialize_field_attribute_lines_reach_backing_field() {
        let lines: Vec<&str> = r#"public class Player : MonoBehaviour
{
    [SerializeField]
    private float _speed = 5.0f;
    public float speed => _speed;
}"#.lines().collect();
        // Property header sits on line 5 (1-based).
        let start = include_serialize_field_attribute_lines(&lines, 5, "speed", 1);
        // Walk back past `_speed` backing field (line 4) and the
        // `[SerializeField]` attribute (line 3) — start should land
        // on line 3.
        assert_eq!(start, 3);
    }

    // Issue #72: `throw`/`break`/`continue` inline statements count
    // as exactly one line, so subsequent statements do not drift.
    #[test]
    fn inline_line_count_handles_throw_break_continue() {
        use crate::lowering::csharp_ir::CsStmt;
        assert_eq!(inline_statement_line_count(&CsStmt::Throw("new Exception(\"oops\")".into(), None)), 1);
        assert_eq!(inline_statement_line_count(&CsStmt::Break(None)), 1);
        assert_eq!(inline_statement_line_count(&CsStmt::Continue(None)), 1);
        assert_eq!(inline_statement_line_count(&CsStmt::YieldBreak(None)), 1);
        assert_eq!(inline_statement_line_count(&CsStmt::Return(None, None)), 1);
    }
}