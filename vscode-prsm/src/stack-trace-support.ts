import * as fs from 'fs';
import * as path from 'path';
import {
    findSourceAnchorForGeneratedPosition,
    readGeneratedSourceMap,
    resolveSourceMapSourcePath,
} from './generated-source-map';

export interface StackFrame {
    csPath: string;
    lineNumber: number;
    rawLine: string;
}

export interface PrismLocation {
    prsmPath: string;
    lineNumber: number;
    colNumber: number;
    rawLine: string;
}

export interface ResolvedFrame {
    rawLine: string;
    prsmPath: string;
    prsmLine: number;
    prsmCol: number;
}

const UNITY_PATTERN = /\(at\s+(.+\.cs):(\d+)\)/gi;
const DOTNET_PATTERN = /\bin\s+(.+\.cs):line\s+(\d+)/gi;
const BARE_PATTERN = /\b([A-Za-z0-9_./\\-]+\.cs):(\d+)/gi;

const DIAGNOSTIC_LOCATION_PATTERN = /^(?<path>.*?\.prsm)\((?<line>\d+),(?<col>\d+)\):/i;
const PRISM_FRAME_PATTERN = /\(at\s+(?<path>.*?\.prsm):(?<line>\d+)\)\s+\[PrSM col\s+(?<col>\d+)\]/i;
const DOTNET_PRISM_FRAME_PATTERN = /\sin\s+(?<path>.*?\.prsm):line\s+(?<line>\d+)\s+\[PrSM col\s+(?<col>\d+)\]/i;

export function parseStackTraceText(text: string): StackFrame[] {
    const results: StackFrame[] = [];
    const seen = new Set<string>();

    function addMatch(rawLine: string, csPath: string, lineStr: string): void {
        const lineNumber = parsePositiveInt(lineStr);
        if (!lineNumber) {
            return;
        }

        const key = `${csPath}:${lineNumber}`;
        if (seen.has(key)) {
            return;
        }

        seen.add(key);
        results.push({ csPath, lineNumber, rawLine: rawLine.trim() });
    }

    for (const rawLine of text.split(/\r?\n/)) {
        let matched = false;

        for (const match of rawLine.matchAll(UNITY_PATTERN)) {
            addMatch(rawLine, match[1], match[2]);
            matched = true;
        }
        for (const match of rawLine.matchAll(DOTNET_PATTERN)) {
            addMatch(rawLine, match[1], match[2]);
            matched = true;
        }
        if (!matched) {
            for (const match of rawLine.matchAll(BARE_PATTERN)) {
                addMatch(rawLine, match[1], match[2]);
            }
        }
    }

    return results;
}

export function parsePrismLocationText(text: string): PrismLocation[] {
    const results: PrismLocation[] = [];
    const seen = new Set<string>();

    for (const rawLine of text.split(/\r?\n/)) {
        const match = rawLine.match(DIAGNOSTIC_LOCATION_PATTERN)
            ?? rawLine.match(DOTNET_PRISM_FRAME_PATTERN)
            ?? rawLine.match(PRISM_FRAME_PATTERN);
        const groups = match?.groups;
        if (!groups) {
            continue;
        }

        const lineNumber = parsePositiveInt(groups.line);
        const colNumber = parsePositiveInt(groups.col);
        if (!lineNumber || !colNumber) {
            continue;
        }

        const prsmPath = groups.path.trim();
        const key = `${prsmPath}:${lineNumber}:${colNumber}`;
        if (seen.has(key)) {
            continue;
        }

        seen.add(key);
        results.push({
            prsmPath,
            lineNumber,
            colNumber,
            rawLine: rawLine.trim(),
        });
    }

    return results;
}

export function resolveFrameCsPath(
    frame: StackFrame,
    workspaceRoots: string[],
    fsLike: typeof fs = fs,
): string | null {
    const { csPath } = frame;

    if (path.isAbsolute(csPath) && fsLike.existsSync(csPath)) {
        return path.normalize(csPath);
    }

    const candidates: string[] = [];
    for (const root of workspaceRoots) {
        candidates.push(path.join(root, csPath));
        candidates.push(path.join(root, 'Assets', 'Generated', path.basename(csPath)));
        candidates.push(path.join(root, 'Assets', path.basename(csPath)));
        candidates.push(path.join(root, 'Generated', path.basename(csPath)));
    }

    for (const candidate of candidates) {
        if (fsLike.existsSync(candidate)) {
            return path.normalize(candidate);
        }
    }

    return null;
}

export function resolvePrismPath(
    prsmPath: string,
    workspaceRoots: string[],
    fsLike: typeof fs = fs,
): string | null {
    if (path.isAbsolute(prsmPath) && fsLike.existsSync(prsmPath)) {
        return path.normalize(prsmPath);
    }

    const candidates = [
        ...workspaceRoots.map(root => path.join(root, prsmPath)),
        path.resolve(prsmPath),
    ];

    for (const candidate of candidates) {
        if (fsLike.existsSync(candidate)) {
            return path.normalize(candidate);
        }
    }

    return null;
}

export function resolveFrameToPrsm(
    csAbsPath: string,
    lineNumber: number,
    workspaceRoots: string[],
    rawLine = '',
): ResolvedFrame | null {
    const sourceMap = readGeneratedSourceMap(csAbsPath);
    if (!sourceMap) {
        return null;
    }

    const anchor = findSourceAnchorForGeneratedPosition(sourceMap, lineNumber, 1);
    if (!anchor || !anchor.source_span) {
        return null;
    }

    const prsmPath = resolveSourceMapSourcePath(csAbsPath, sourceMap, workspaceRoots);
    if (!prsmPath) {
        return null;
    }

    return {
        rawLine,
        prsmPath,
        prsmLine: anchor.source_span.line,
        prsmCol: anchor.source_span.col,
    };
}

export function resolveStackTraceLocations(
    text: string,
    workspaceRoots: string[],
    fsLike: typeof fs = fs,
): ResolvedFrame[] {
    const resolved: ResolvedFrame[] = [];

    for (const location of parsePrismLocationText(text)) {
        const absolutePrsmPath = resolvePrismPath(location.prsmPath, workspaceRoots, fsLike);
        if (!absolutePrsmPath) {
            continue;
        }

        resolved.push({
            rawLine: location.rawLine,
            prsmPath: absolutePrsmPath,
            prsmLine: location.lineNumber,
            prsmCol: location.colNumber,
        });
    }

    for (const frame of parseStackTraceText(text)) {
        const absoluteCsPath = resolveFrameCsPath(frame, workspaceRoots, fsLike);
        if (!absoluteCsPath) {
            continue;
        }

        const mapped = resolveFrameToPrsm(absoluteCsPath, frame.lineNumber, workspaceRoots, frame.rawLine);
        if (mapped) {
            resolved.push(mapped);
        }
    }

    return dedupeResolvedFrames(resolved);
}

function dedupeResolvedFrames(frames: ResolvedFrame[]): ResolvedFrame[] {
    const deduped: ResolvedFrame[] = [];
    const seen = new Set<string>();

    for (const frame of frames) {
        const key = `${path.normalize(frame.prsmPath)}:${frame.prsmLine}:${frame.prsmCol}`;
        if (seen.has(key)) {
            continue;
        }

        seen.add(key);
        deduped.push(frame);
    }

    return deduped;
}

function parsePositiveInt(text: string | undefined): number | null {
    const value = Number.parseInt(text ?? '', 10);
    if (!Number.isFinite(value) || value < 1) {
        return null;
    }

    return value;
}