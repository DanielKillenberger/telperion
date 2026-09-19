import { readFileSync } from 'node:fs';
import { expect, test } from 'vitest';
import { TreeEngine, ORDINARY } from '../src/browser/core';

test('specimen handles recover, own reads and snapshots, and expire on replacement', async () => {
  const engine = await TreeEngine.create(readFileSync('src/browser/telperion.wasm'));
  const family = structuredClone(ORDINARY);
  family.age = 10.25;
  family.skeleton.envelope.height = 4;
  family.skeleton.attractors = 40;
  const s = engine.buildSpecimen(family, 100);
  const read = s.read(), saved = read.structure.values.slice();
  expect(read.envelope.crownBase).toBeGreaterThanOrEqual(0);
  const bytes = s.snapshot();
  expect(bytes.schema).toBe(3);
  expect(() => engine.buildSpecimen({ ...family, age: -1 })).toThrow(/age/);
  expect(s.read().structure.values).toEqual(saved);
  expect(() => s.advance(-1)).toThrow(/-1/);
  expect(() => s.read(20)).toThrow(/frontier/);
  expect(() => s.read(-1)).toThrow(/age/);
  const change = s.advance(1);
  expect(change.frontier).toBe(11.25);
  const after = s.read();
  const key = (id: typeof read.placements[number]) => `${id.shoot.birth}:${id.station}`;
  const leaves = new Map(read.placements.map((id, i) => [key(id), Array.from(read.leaves.slice(i * 3, i * 3 + 3))]));
  for (const id of change.changes.shed_placements) leaves.delete(key(id));
  for (const p of [...change.changes.born_placements, ...change.changes.moved_placements]) leaves.set(key(p.identity), [...p.leaf]);
  expect(leaves.size).toBe(after.placements.length);
  // Packed words are the storage on both sides of the record, so the applied
  // chronicle reconciles with a fresh read bit for bit.
  expect(new Uint32Array(after.placements.flatMap(id => leaves.get(key(id))!))).toEqual(after.leaves);
  for (const run of [...change.changes.born_runs, ...change.changes.resized_runs]) {
    for (const node of run.nodes) {
      const i = after.nodes.findIndex(id => id.birth === node.identity.birth);
      expect(i).toBeGreaterThanOrEqual(0);
      expect(Array.from(after.structure.values.slice(i * 6, i * 6 + 6))).toEqual([
        node.position.x, node.position.y, node.position.z, ...node.radii,
      ]);
    }
  }
  expect(s.changes(10.25, 11.25)).toEqual(change.changes);
  expect(read.structure.values).toEqual(saved);
  const imported = engine.importSpecimen(bytes);
  expect(() => s.read()).toThrow(/handle/);
  expect(imported.read().structure.values).toEqual(saved);
  const corrupt = { ...bytes, data: new Uint8Array([1, 2, 3]) };
  expect(() => engine.importSpecimen(corrupt)).toThrow(/snapshot/);
  expect(imported.read().structure.values).toEqual(saved);
  bytes.data.fill(0);
  expect(imported.read().structure.values).toEqual(saved);
  const earlier = imported.read(4.5);
  const fresh = engine.buildSpecimen({ ...family, age: 4.5 });
  expect(fresh.read()).toEqual(earlier);
  const capped = engine.buildSpecimen(family, 1);
  expect(() => capped.read(1)).toThrow(/cap/);
  engine.release();
  expect(() => capped.read()).toThrow(/handle/);
  expect(read.structure.values).toEqual(saved);
  const last = engine.buildSpecimen(family);
  engine.dispose();
  expect(() => last.read()).toThrow(/disposed/);
}, 120_000);
