import test from 'node:test';
import assert from 'node:assert/strict';
import * as fs from 'fs';
import * as os from 'os';
import * as path from 'path';
import {
    PrismFlatDebugMap,
    buildSourceFileMap,
    loadFlatDebugMap,
    mapCsLineToPrsmLine,
    mapPrsmLineToCsLine,
} from '../dap-adapter';

function tempDir(prefix: string): string {
    return fs.mkdtempSync(path.join(os.tmpdir(), prefix));
}

function writeMap(dir: string, generatedName: string, map: PrismFlatDebugMap): string {
    const mapPath = path.join(dir, `${path.basename(generatedName, '.cs')}.prsm.map`);
    fs.writeFileSync(mapPath, JSON.stringify(map), 'utf8');
    return mapPath;
}

test('loadFlatDebugMap reads a valid map next to the generated cs file', () => {
    const dir = tempDir('prsm-dap-load-');
    const map: PrismFlatDebugMap = {
        version: 1,
        source: 'Player.prsm',
        generated: 'Player.cs',
        mappings: [
            { prsmLine: 5, csLine: 12 },
            { prsmLine: 6, csLine: 13 },
        ],
    };
    writeMap(dir, 'Player.cs', map);

    const csPath = path.join(dir, 'Player.cs');
    fs.writeFileSync(csPath, '// generated\n', 'utf8');

    const loaded = loadFlatDebugMap(csPath);
    assert.ok(loaded, 'expected non-null map');
    assert.equal(loaded?.mappings.length, 2);
    assert.equal(loaded?.mappings[0].prsmLine, 5);
    assert.equal(loaded?.mappings[1].csLine, 13);
});

test('loadFlatDebugMap returns null when the map is missing', () => {
    const dir = tempDir('prsm-dap-missing-');
    const csPath = path.join(dir, 'Empty.cs');
    fs.writeFileSync(csPath, '// generated\n', 'utf8');
    const loaded = loadFlatDebugMap(csPath);
    assert.equal(loaded, null);
});

test('mapPrsmLineToCsLine handles exact and best-effort matches', () => {
    const map: PrismFlatDebugMap = {
        version: 1,
        source: 'a.prsm',
        generated: 'a.cs',
        mappings: [
            { prsmLine: 5, csLine: 12 },
            { prsmLine: 8, csLine: 18 },
            { prsmLine: 10, csLine: 22 },
        ],
    };
    assert.equal(mapPrsmLineToCsLine(map, 5), 12);
    assert.equal(mapPrsmLineToCsLine(map, 8), 18);
    // Best-effort: line 7 falls between 5 and 8 — pick the larger <= 7 (5).
    assert.equal(mapPrsmLineToCsLine(map, 7), 12);
    // Past the last mapping — return the largest.
    assert.equal(mapPrsmLineToCsLine(map, 99), 22);
    // Before the first mapping — no result.
    assert.equal(mapPrsmLineToCsLine(map, 1), null);
});

test('mapCsLineToPrsmLine inverts the mapping', () => {
    const map: PrismFlatDebugMap = {
        version: 1,
        source: 'a.prsm',
        generated: 'a.cs',
        mappings: [
            { prsmLine: 5, csLine: 12 },
            { prsmLine: 8, csLine: 18 },
        ],
    };
    assert.equal(mapCsLineToPrsmLine(map, 12), 5);
    assert.equal(mapCsLineToPrsmLine(map, 18), 8);
    // Best-effort fallback for an interior line.
    assert.equal(mapCsLineToPrsmLine(map, 15), 5);
});

test('buildSourceFileMap walks the workspace and merges every map', () => {
    const root = tempDir('prsm-dap-walk-');
    fs.mkdirSync(path.join(root, 'Generated'), { recursive: true });
    fs.mkdirSync(path.join(root, 'Generated', 'Sub'), { recursive: true });

    const map1: PrismFlatDebugMap = {
        version: 1,
        source: '../../src/Player.prsm',
        generated: 'Player.cs',
        mappings: [{ prsmLine: 1, csLine: 1 }],
    };
    fs.writeFileSync(path.join(root, 'Generated', 'Player.prsm.map'), JSON.stringify(map1));
    fs.writeFileSync(path.join(root, 'Generated', 'Player.cs'), '// generated\n');

    const map2: PrismFlatDebugMap = {
        version: 1,
        source: '../../../src/Enemy.prsm',
        generated: 'Enemy.cs',
        mappings: [{ prsmLine: 1, csLine: 1 }],
    };
    fs.writeFileSync(path.join(root, 'Generated', 'Sub', 'Enemy.prsm.map'), JSON.stringify(map2));
    fs.writeFileSync(path.join(root, 'Generated', 'Sub', 'Enemy.cs'), '// generated\n');

    const built = buildSourceFileMap(root);
    const csPaths = Object.keys(built);
    assert.equal(csPaths.length, 2, `expected two cs entries, got ${csPaths.join(', ')}`);
    const playerCs = csPaths.find(p => p.endsWith('Player.cs'));
    assert.ok(playerCs, 'expected Player.cs entry');
    assert.match(built[playerCs!], /Player\.prsm$/);
});
