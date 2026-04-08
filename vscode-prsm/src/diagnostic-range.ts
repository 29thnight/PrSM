export interface PrismDiagnosticRangeInput {
    line: number;
    col: number;
    endLine?: number;
    endCol?: number;
}

export interface NormalizedRange {
    startLine: number;
    startCol: number;
    endLine: number;
    endCol: number;
}

export function normalizeDiagnosticRange(
    entry: PrismDiagnosticRangeInput,
    lineLengths: number[],
): NormalizedRange {
    const safeLengths = lineLengths.length > 0 ? lineLengths : [0];
    const maxLine = safeLengths.length - 1;

    const startLine = clamp(entry.line - 1, 0, maxLine);
    const startCol = clamp(entry.col - 1, 0, safeLengths[startLine]);

    let endLine = clamp((entry.endLine ?? entry.line) - 1, 0, maxLine);
    let endCol = clamp((entry.endCol ?? entry.col) - 1, 0, safeLengths[endLine]);

    if (endLine < startLine || (endLine === startLine && endCol <= startCol)) {
        endLine = startLine;
        endCol = Math.min(safeLengths[startLine], startCol + 1);
        if (endCol <= startCol && safeLengths[startLine] === 0) {
            endCol = startCol;
        }
    }

    return { startLine, startCol, endLine, endCol };
}

/**
 * Issue #80: the compiler reports diagnostic columns in Unicode
 * codepoints, but VS Code expects UTF-16 code units (as per the LSP
 * spec). For ASCII-only source the two agree; for CJK identifiers and
 * surrogate-pair characters (emoji, etc.) the positions drift.
 * Convert a 1-based codepoint column into a 0-based UTF-16 code-unit
 * index against the given line text. Returns the length of the line
 * (in code units) for overflowing input to keep clamping consistent.
 */
export function codepointColToUtf16Index(lineText: string, codepointCol1Based: number): number {
    if (codepointCol1Based <= 1) {
        return 0;
    }
    // `codepointCol1Based - 1` is the number of FULL codepoints that
    // precede the cursor. Walk `lineText` counting one codepoint per
    // surrogate-pair skip.
    const target = codepointCol1Based - 1;
    let codepoints = 0;
    let utf16 = 0;
    for (const _char of lineText) {
        if (codepoints >= target) {
            break;
        }
        codepoints += 1;
        // Iterating the string with `for...of` yields each codepoint;
        // measure the UTF-16 length of the iterated segment.
        utf16 += _char.length;
    }
    return utf16;
}

function clamp(value: number, min: number, max: number): number {
    return Math.min(Math.max(value, min), max);
}