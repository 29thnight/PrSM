/**
 * Stack trace navigator for PrSM source-map workflow.
 *
 * Parses Unity / .NET stack trace lines that reference generated .cs files
 * or already-remapped `.prsm` runtime frames, then opens the original source.
 *
 * Supported stack-trace formats
 * ─────────────────────────────
 *   Unity:        (at Assets/Generated/Foo.cs:42)
 *   .NET:         in C:\path\to\Foo.cs:line 42
 *   plain:        Foo.cs:42
 *   remapped:     Assets/Foo.prsm(8,10): error [PrSMRuntime] ...
 *   remapped dot: at Foo.Update() in Assets/Foo.prsm:line 8 [PrSM col 10]
 */

import * as path from 'path';
import * as vscode from 'vscode';
import {
    parsePrismLocationText,
    parseStackTraceText,
    resolveStackTraceLocations,
    type ResolvedFrame,
} from './stack-trace-support';

// ─────────────────────────────────────────────────────────────────────────────
// VS Code command handler
// ─────────────────────────────────────────────────────────────────────────────

/**
 * `prsm.openFromStackTrace` command implementation.
 *
 * Uses the active editor selection when text is selected; otherwise prompts
 * the user to paste a stack-trace snippet. Finds already-remapped `.prsm`
 * locations or generated `.cs` references, resolves them, and opens the
 * first successful `.prsm` location.
 * If multiple frames resolve successfully a QuickPick is shown.
 */
export async function openFromStackTraceCommand(workspaceRoots: string[]): Promise<void> {
    let input: string | undefined;

    const editor = vscode.window.activeTextEditor;
    if (editor && !editor.selection.isEmpty) {
        input = editor.document.getText(editor.selection);
    }

    if (!input || input.trim() === '') {
        input = await vscode.window.showInputBox({
            prompt: 'Paste a Unity / .NET stack trace (one or more lines)',
            placeHolder: '(at Assets/Generated/PlayerController.cs:42)',
            ignoreFocusOut: true,
        });
    }

    if (!input) {
        return;
    }

    const directPrsmLocations = parsePrismLocationText(input);
    const generatedFrames = parseStackTraceText(input);
    if (directPrsmLocations.length === 0 && generatedFrames.length === 0) {
        vscode.window.showWarningMessage('No PrSM or generated C# file references found in the pasted text.');
        return;
    }

    const resolved = resolveStackTraceLocations(input, workspaceRoots);

    if (resolved.length === 0) {
        if (directPrsmLocations.length > 0) {
            vscode.window.showWarningMessage('No PrSM source files found for the referenced location(s).');
        } else {
            vscode.window.showWarningMessage(
                'No PrSM source map found for the referenced .cs file(s). Compile the workspace first.',
            );
        }
        return;
    }

    let target: ResolvedFrame;

    if (resolved.length === 1) {
        target = resolved[0];
    } else {
        const items = resolved.map(r => ({
            label: `$(go-to-file) ${path.basename(r.prsmPath)}:${r.prsmLine}`,
            description: r.rawLine,
            detail: r.prsmPath,
            resolved: r,
        }));

        const picked = await vscode.window.showQuickPick(items, {
            placeHolder: 'Multiple PrSM locations found — select one to open',
        });

        if (!picked) {
            return;
        }
        target = picked.resolved;
    }

    const uri = vscode.Uri.file(target.prsmPath);
    const doc = await vscode.workspace.openTextDocument(uri);
    const line = Math.max(0, target.prsmLine - 1);
    const col = Math.max(0, target.prsmCol - 1);
    const range = new vscode.Range(line, col, line, col);

    await vscode.window.showTextDocument(doc, {
        preview: false,
        selection: range,
    });
}
