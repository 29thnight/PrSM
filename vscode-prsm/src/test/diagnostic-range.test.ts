import test from 'node:test';
import assert from 'node:assert/strict';
import { codepointColToUtf16Index, normalizeDiagnosticRange } from '../diagnostic-range';

test('normalizeDiagnosticRange uses explicit end coordinates', () => {
    const range = normalizeDiagnosticRange(
        { line: 2, col: 3, endLine: 2, endCol: 8 },
        [5, 12, 4],
    );

    assert.deepEqual(range, {
        startLine: 1,
        startCol: 2,
        endLine: 1,
        endCol: 7,
    });
});

test('normalizeDiagnosticRange widens zero-width spans to one character when possible', () => {
    const range = normalizeDiagnosticRange(
        { line: 1, col: 2, endLine: 1, endCol: 2 },
        [6],
    );

    assert.deepEqual(range, {
        startLine: 0,
        startCol: 1,
        endLine: 0,
        endCol: 2,
    });
});

test('normalizeDiagnosticRange clamps coordinates to document bounds', () => {
    const range = normalizeDiagnosticRange(
        { line: 10, col: 20, endLine: 11, endCol: 30 },
        [3, 4],
    );

    assert.deepEqual(range, {
        startLine: 1,
        startCol: 4,
        endLine: 1,
        endCol: 4,
    });
});

// Issue #80: codepoint-to-UTF16 conversion for LSP position reporting.
// For ASCII the two encodings agree, but surrogate-pair characters
// count as 2 UTF-16 units per codepoint.
test('codepointColToUtf16Index returns identity for ASCII', () => {
    // Column 1 is codepoint-0 → utf16 index 0.
    assert.equal(codepointColToUtf16Index('hello world', 1), 0);
    // Column 6 is codepoint-5 → utf16 index 5.
    assert.equal(codepointColToUtf16Index('hello world', 6), 5);
});

test('codepointColToUtf16Index converts surrogate pairs to UTF-16 units', () => {
    // 😀 is a surrogate pair in UTF-16 (2 code units, 1 codepoint).
    // Line: "a😀b" — codepoints ['a', '😀', 'b'], utf16 positions [0, 1, 3, 4].
    // Column 3 (0-based cp=2) is 'b', which starts at utf16 index 3.
    const line = 'a😀b';
    assert.equal(codepointColToUtf16Index(line, 1), 0); // 'a'
    assert.equal(codepointColToUtf16Index(line, 2), 1); // 😀 starts at utf16 1
    assert.equal(codepointColToUtf16Index(line, 3), 3); // 'b' starts at utf16 3
});

test('codepointColToUtf16Index handles CJK identifiers', () => {
    // 한글 is two CJK characters, each one UTF-16 code unit.
    const line = 'val 한글: Int';
    // column 5 (0-based cp 4) is '한' — utf16 index 4.
    assert.equal(codepointColToUtf16Index(line, 5), 4);
    // column 6 is '글' — utf16 index 5.
    assert.equal(codepointColToUtf16Index(line, 6), 5);
});