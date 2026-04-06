import test from 'node:test';
import assert from 'node:assert/strict';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import {
    parsePrismLocationText,
    parseStackTraceText,
    resolvePrismPath,
    resolveStackTraceLocations,
} from '../stack-trace-support';

function tempDir(prefix: string): string {
    return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

test('parsePrismLocationText extracts diagnostic headers and remapped frames', () => {
    const locations = parsePrismLocationText(
        'Assets/TestScript.prsm(7,5): error [PrSMRuntime] DivideByZeroException\n'
        + '[PrSM] Remapped runtime stack trace from generated PrSM C#\n'
        + 'TestScript.Awake () (at Assets/TestScript.prsm:7) [PrSM col 5]\n'
        + 'at TestScript.Awake() in Assets/TestScript.prsm:line 7 [PrSM col 5]',
    );

    assert.equal(locations.length, 1);
    assert.equal(locations[0].prsmPath, 'Assets/TestScript.prsm');
    assert.equal(locations[0].lineNumber, 7);
    assert.equal(locations[0].colNumber, 5);
});

test('resolvePrismPath resolves remapped PrSM paths relative to workspace roots', () => {
    const root = tempDir('prsm-stack-trace-prsm-');
    const sourceFile = path.join(root, 'Assets', 'TestScript.prsm');
    fs.mkdirSync(path.dirname(sourceFile), { recursive: true });
    fs.writeFileSync(sourceFile, 'component TestScript : MonoBehaviour {}\n');

    assert.equal(resolvePrismPath('Assets/TestScript.prsm', [root]), sourceFile);

    fs.rmSync(root, { recursive: true, force: true });
});

test('resolveStackTraceLocations prefers already-remapped PrSM locations', () => {
    const root = tempDir('prsm-stack-trace-direct-');
    const sourceFile = path.join(root, 'Assets', 'Player.prsm');
    fs.mkdirSync(path.dirname(sourceFile), { recursive: true });
    fs.writeFileSync(sourceFile, 'component Player : MonoBehaviour {}\n');

    const resolved = resolveStackTraceLocations(
        'Assets/Player.prsm(8,10): error [PrSMRuntime] NullReferenceException: sample\n'
        + '[PrSM] Remapped runtime stack trace from generated PrSM C#\n'
        + 'Player.Update() (at Assets/Player.prsm:8) [PrSM col 10]',
        [root],
    );

    assert.equal(resolved.length, 1);
    assert.equal(resolved[0].prsmPath, sourceFile);
    assert.equal(resolved[0].prsmLine, 8);
    assert.equal(resolved[0].prsmCol, 10);

    fs.rmSync(root, { recursive: true, force: true });
});

test('resolveStackTraceLocations maps generated C# frames through source maps', () => {
    const root = tempDir('prsm-stack-trace-generated-');
    const sourceFile = path.join(root, 'Assets', 'Player.prsm');
    const generatedFile = path.join(root, 'Packages', 'com.prsm.generated', 'Runtime', 'Player.cs');
    const sourceMapFile = path.join(root, 'Packages', 'com.prsm.generated', 'Runtime', 'Player.prsmmap.json');

    fs.mkdirSync(path.dirname(sourceFile), { recursive: true });
    fs.mkdirSync(path.dirname(generatedFile), { recursive: true });
    fs.writeFileSync(sourceFile, 'component Player : MonoBehaviour {}\n');
    fs.writeFileSync(generatedFile, '// generated\n');
    fs.writeFileSync(sourceMapFile, JSON.stringify({
        version: 1,
        source_file: 'Assets/Player.prsm',
        generated_file: 'Packages/com.prsm.generated/Runtime/Player.cs',
        declaration: {
            kind: 'type',
            name: 'Player',
            qualified_name: 'Player',
            source_span: { line: 1, col: 11, end_line: 1, end_col: 16 },
            generated_span: { line: 7, col: 1, end_line: 23, end_col: 1 },
            generated_name_span: { line: 7, col: 14, end_line: 7, end_col: 19 },
        },
        members: [
            {
                kind: 'function',
                name: 'Update',
                qualified_name: 'Player.Update',
                source_span: { line: 8, col: 10, end_line: 8, end_col: 15 },
                generated_span: { line: 18, col: 1, end_line: 22, end_col: 5 },
                generated_name_span: { line: 18, col: 17, end_line: 18, end_col: 22 },
            },
        ],
    }));

    const resolved = resolveStackTraceLocations(
        'Player.Update() (at Packages/com.prsm.generated/Runtime/Player.cs:19)',
        [root],
    );

    assert.equal(resolved.length, 1);
    assert.equal(resolved[0].prsmPath, sourceFile);
    assert.equal(resolved[0].prsmLine, 8);
    assert.equal(resolved[0].prsmCol, 10);

    fs.rmSync(root, { recursive: true, force: true });
});

test('parseStackTraceText extracts generated C# frames from Unity and .NET formats', () => {
    const frames = parseStackTraceText(
        'Player.Update() (at Packages/com.prsm.generated/Runtime/Player.cs:19)\n'
        + 'at Player.Update() in C:/Project/Packages/com.prsm.generated/Runtime/Player.cs:line 19',
    );

    assert.equal(frames.length, 2);
    assert.equal(frames[0].csPath, 'Packages/com.prsm.generated/Runtime/Player.cs');
    assert.equal(frames[0].lineNumber, 19);
    assert.equal(frames[1].csPath, 'C:/Project/Packages/com.prsm.generated/Runtime/Player.cs');
    assert.equal(frames[1].lineNumber, 19);
});