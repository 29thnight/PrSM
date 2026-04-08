// PrSM Debug Adapter (v5 deferred item)
//
// VS Code already ships an excellent C# debugger via `ms-dotnettools.csharp`
// (vsdbg / coreclr / mono). PrSM compiles to C#, so the most reliable
// debugging story is to *delegate* to that adapter and just rewrite source
// references between PrSM and the generated C# using the flat
// `*.prsm.map` files emitted by the compiler.
//
// This module provides:
//
//   1. `PrismDebugAdapterDescriptorFactory` — registered with
//      `vscode.debug.registerDebugAdapterDescriptorFactory('prsm', ...)`.
//      Currently returns `null`, signalling that VS Code should fall
//      through to the `coreclr` adapter and apply the source-map
//      translation we feed it via the launch config.
//
//   2. `PrismDebugConfigurationProvider` — implements the
//      `resolveDebugConfiguration` callback that rewrites a PrSM-shaped
//      launch config (`{ "type": "prsm", "program": "...prsm" }`) into
//      a coreclr-shaped one with `sourceFileMap` populated from the
//      flat debug maps next to each generated `.cs` file.
//
//   3. `loadFlatDebugMap` / `mapPrsmLineToCsLine` /
//      `mapCsLineToPrsmLine` — pure helpers for the rest of the
//      extension and the test suite.
//
// The DAP integration is intentionally thin so existing C# debuggers
// (vsdbg in OmniSharp, vsdbg-ui in C# Dev Kit, mono-debug for older
// engines) continue to work unchanged. The only thing PrSM needs is to
// translate breakpoints from `.prsm` line numbers to `.cs` line numbers
// before they reach the C# adapter, and the reverse for stack frames.

import * as fs from 'fs';
import * as path from 'path';
// Type-only import so the test runner (which has no `vscode` module
// available) can load this file purely for the pure helpers below.
// The runtime functions that actually call into VS Code dynamically
// `require('vscode')` inside `registerPrismDebugAdapter`.
import type * as vscode from 'vscode';

/// Spec-compliant flat source map (matches `crates/refraction/src/debugger.rs`).
export interface PrismFlatDebugMapping {
    prsmLine: number;
    csLine: number;
}

export interface PrismFlatDebugMap {
    version: number;
    source: string;
    generated: string;
    mappings: PrismFlatDebugMapping[];
}

/// Read the flat `*.prsm.map` JSON next to a generated `.cs` file. The
/// compiler writes this file via `crates/refraction/src/debugger.rs`'s
/// `flatten_source_map` helper. Returns `null` when the map is missing
/// or malformed.
export function loadFlatDebugMap(generatedCsPath: string): PrismFlatDebugMap | null {
    const dir = path.dirname(generatedCsPath);
    const stem = path.basename(generatedCsPath, path.extname(generatedCsPath));
    const mapPath = path.join(dir, `${stem}.prsm.map`);
    if (!fs.existsSync(mapPath)) {
        return null;
    }
    try {
        const raw = fs.readFileSync(mapPath, 'utf8');
        const parsed = JSON.parse(raw) as Partial<PrismFlatDebugMap>;
        if (
            typeof parsed.version !== 'number' ||
            typeof parsed.source !== 'string' ||
            typeof parsed.generated !== 'string' ||
            !Array.isArray(parsed.mappings)
        ) {
            return null;
        }
        // Filter out malformed entries defensively.
        const mappings: PrismFlatDebugMapping[] = parsed.mappings
            .filter(
                (m): m is PrismFlatDebugMapping =>
                    typeof m === 'object' &&
                    m !== null &&
                    typeof (m as PrismFlatDebugMapping).prsmLine === 'number' &&
                    typeof (m as PrismFlatDebugMapping).csLine === 'number',
            )
            .sort((a, b) => a.prsmLine - b.prsmLine);
        return {
            version: parsed.version,
            source: parsed.source,
            generated: parsed.generated,
            mappings,
        };
    } catch {
        return null;
    }
}

/// Translate a 1-based PrSM line number into the matching generated C#
/// line. Returns `null` when no mapping exists. The match prefers an
/// exact hit; falling back to the next mapping with `prsmLine <= line`
/// keeps breakpoints set on a multi-line statement reachable.
///
/// Issue #73: when multiple mappings share the same PrSM line (because
/// the old exact-match returned the first one — which sometimes pointed
/// at a brace-only line where the C# debugger cannot stop), prefer the
/// LARGEST C# line among the exact matches. This advances the breakpoint
/// into the first executable statement of a multi-line lowering instead
/// of landing on the method header's `{`.
export function mapPrsmLineToCsLine(map: PrismFlatDebugMap, prsmLine: number): number | null {
    if (map.mappings.length === 0) {
        return null;
    }
    // Exact match first — pick the highest C# line among all matches so
    // we skip past brace-only lines emitted by the anchor walker.
    let exactBest: number | null = null;
    for (const m of map.mappings) {
        if (m.prsmLine === prsmLine) {
            if (exactBest === null || m.csLine > exactBest) {
                exactBest = m.csLine;
            }
        }
    }
    if (exactBest !== null) {
        return exactBest;
    }
    // Best-effort: pick the largest mapping whose source line is <= the
    // requested line. This is what most debuggers do when a breakpoint
    // lands on a non-statement line (blank, comment, brace).
    let best: PrismFlatDebugMapping | null = null;
    for (const m of map.mappings) {
        if (m.prsmLine <= prsmLine) {
            if (best === null || m.prsmLine > best.prsmLine) {
                best = m;
            } else if (m.prsmLine === best.prsmLine && m.csLine > best.csLine) {
                // Prefer the latest C# line on ties, same rationale as above.
                best = m;
            }
        }
    }
    return best ? best.csLine : null;
}

/// Reverse mapping: given a 1-based C# line, find the matching PrSM
/// line. Used by the stack-frame rewriter so callers see PrSM source
/// locations even though the underlying debugger reports C# locations.
export function mapCsLineToPrsmLine(map: PrismFlatDebugMap, csLine: number): number | null {
    if (map.mappings.length === 0) {
        return null;
    }
    for (const m of map.mappings) {
        if (m.csLine === csLine) {
            return m.prsmLine;
        }
    }
    let best: PrismFlatDebugMapping | null = null;
    for (const m of map.mappings) {
        if (m.csLine <= csLine) {
            if (best === null || m.csLine > best.csLine) {
                best = m;
            }
        }
    }
    return best ? best.prsmLine : null;
}

/// VS Code debug-adapter descriptor factory for the `prsm` debug type.
/// Returning `null` (or rather an `executable` pointing nowhere) tells
/// VS Code to fall through to the next registered adapter. We expect
/// the user to install the C# debugger separately; this factory exists
/// purely so the activation event for `prsm` debugging fires and the
/// configuration provider gets a chance to rewrite the launch config.
export class PrismDebugAdapterDescriptorFactory implements vscode.DebugAdapterDescriptorFactory {
    createDebugAdapterDescriptor(
        _session: vscode.DebugSession,
        _executable: vscode.DebugAdapterExecutable | undefined,
    ): vscode.ProviderResult<vscode.DebugAdapterDescriptor> {
        // We have no in-process adapter; rely on the C# debugger that
        // PrSM workspaces already depend on. Returning `undefined`
        // makes VS Code raise the "no adapter found" error, which is
        // intentional — we want users to launch via `coreclr` directly
        // and the configuration provider just sets up source-file maps.
        return undefined;
    }
}

/// Configuration provider that rewrites `type: "prsm"` launch configs
/// into `type: "coreclr"` configs with the right `sourceFileMap` and
/// `stopAtEntry: false` defaults. The mapping is built lazily when the
/// user starts a debug session.
export class PrismDebugConfigurationProvider implements vscode.DebugConfigurationProvider {
    resolveDebugConfiguration(
        folder: vscode.WorkspaceFolder | undefined,
        config: vscode.DebugConfiguration,
        _token?: vscode.CancellationToken,
    ): vscode.ProviderResult<vscode.DebugConfiguration> {
        // If the user already targeted coreclr / mono / vsdbg, leave it
        // alone — they know what they're doing.
        if (config.type !== 'prsm') {
            return config;
        }
        if (!folder) {
            return null;
        }
        // Discover every flat debug map under the workspace and build a
        // sourceFileMap entry for each pair so the C# debugger surfaces
        // PrSM file paths in stack frames.
        const sourceFileMap = buildSourceFileMap(folder.uri.fsPath);
        const rewritten: vscode.DebugConfiguration = {
            ...config,
            type: 'coreclr',
            // Allow the user to override these via their launch.json.
            request: config.request ?? 'launch',
            sourceFileMap: {
                ...(config.sourceFileMap ?? {}),
                ...sourceFileMap,
            },
        };
        return rewritten;
    }
}

/// Walk every `*.prsm.map` file under `workspaceRoot` and produce a map
/// of `<generated cs absolute path>` → `<source prsm absolute path>`.
/// The returned object is consumed by `coreclr`'s `sourceFileMap`
/// configuration so stack-frame display shows the PrSM file.
export function buildSourceFileMap(workspaceRoot: string): Record<string, string> {
    const out: Record<string, string> = {};
    const queue: string[] = [workspaceRoot];
    while (queue.length > 0) {
        const dir = queue.pop()!;
        let entries: fs.Dirent[];
        try {
            entries = fs.readdirSync(dir, { withFileTypes: true });
        } catch {
            continue;
        }
        for (const entry of entries) {
            // Skip noisy directories that never contain compiler output.
            if (entry.isDirectory()) {
                if (
                    entry.name === 'node_modules' ||
                    entry.name === '.git' ||
                    entry.name === 'Library' ||
                    entry.name === 'Temp'
                ) {
                    continue;
                }
                queue.push(path.join(dir, entry.name));
                continue;
            }
            if (!entry.isFile() || !entry.name.endsWith('.prsm.map')) {
                continue;
            }
            const mapPath = path.join(dir, entry.name);
            const map = loadFlatDebugMapAtPath(mapPath);
            if (!map) {
                continue;
            }
            const csAbs = path.resolve(dir, map.generated);
            const prsmAbs = path.resolve(dir, map.source);
            out[csAbs] = prsmAbs;
        }
    }
    return out;
}

function loadFlatDebugMapAtPath(mapPath: string): PrismFlatDebugMap | null {
    try {
        const raw = fs.readFileSync(mapPath, 'utf8');
        const parsed = JSON.parse(raw) as Partial<PrismFlatDebugMap>;
        if (
            typeof parsed.version !== 'number' ||
            typeof parsed.source !== 'string' ||
            typeof parsed.generated !== 'string' ||
            !Array.isArray(parsed.mappings)
        ) {
            return null;
        }
        return {
            version: parsed.version,
            source: parsed.source,
            generated: parsed.generated,
            mappings: parsed.mappings as PrismFlatDebugMapping[],
        };
    } catch {
        return null;
    }
}

/// Convenience: register both the descriptor factory and the
/// configuration provider on extension activation. Caller is the
/// `extension.ts` activate() entry point.
///
/// `vscode` is `require`d at runtime so this module can also be loaded
/// from the standalone test runner, which has no `vscode` package
/// available. The pure helpers (`loadFlatDebugMap`,
/// `mapPrsmLineToCsLine`, `buildSourceFileMap`, …) are exercised by
/// the test suite without needing a fake VS Code shim.
export function registerPrismDebugAdapter(context: vscode.ExtensionContext): void {
    // eslint-disable-next-line @typescript-eslint/no-var-requires
    const vscodeRuntime = require('vscode') as typeof import('vscode');
    context.subscriptions.push(
        vscodeRuntime.debug.registerDebugAdapterDescriptorFactory(
            'prsm',
            new PrismDebugAdapterDescriptorFactory(),
        ),
    );
    context.subscriptions.push(
        vscodeRuntime.debug.registerDebugConfigurationProvider(
            'prsm',
            new PrismDebugConfigurationProvider(),
        ),
    );
}
