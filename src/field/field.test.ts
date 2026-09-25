import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";
import { growField, compileField, NO_LIMB } from "./index";
import { grid } from "./voxelize";
import { TreeEngine, presetById, PRESETS } from "../browser/core";

/* The slim entry point in Node (R5): the module's bytes passed in, one
 * query answered. Beside it, the answers held to the full binding's on the
 * same species, seed and limb order, so the slim chain is the field the
 * harness shows and not a second one. */
const source = readFileSync("src/field/telperion-field.wasm");

describe("the slim field entry point", () => {
  it("grows a species at a seed and answers a grid, byte for byte the full binding's", async () => {
    const module = await compileField(source);
    const tree = await growField("ordinary", 5, { limbOrder: 1, source: module });
    expect(tree.species).toBe("ordinary");
    expect(tree.seed).toBe(5);
    expect(tree.bounds.max[1]).toBeGreaterThan(tree.bounds.min[1]);
    const g = grid(tree.bounds, 12);
    const slim = tree.query(g.cells);
    expect(slim.flags.length).toBe(12 ** 3);
    expect(slim.flags.some(f => f & 1)).toBe(true);
    expect(slim.flags.some(f => f & 2)).toBe(true);
    expect(slim.limbs.some(l => l !== NO_LIMB)).toBe(true);

    const engine = await TreeEngine.create(readFileSync("src/browser/telperion.wasm"));
    const family = presetById("ordinary");
    family.skeleton.seed = 5;
    const full = engine.build(family, { field: { limbOrder: 1 } }).field!.query(g.cells);
    expect(full.flags).toEqual(slim.flags);
    expect(full.woodRadius).toEqual(slim.woodRadius);
    expect(full.leaves).toEqual(slim.leaves);
    expect(full.limbs).toEqual(slim.limbs);
    engine.dispose();

    const again = await growField("ordinary", 5, { limbOrder: 1, source: module });
    expect(again.query(g.cells)).toEqual(slim);
    tree.release();
    again.release();
    expect(() => tree.query(g.cells)).toThrow("released");
  });

  it("loads its Wasm from beside itself in Node when no source is given, as the main entry does", async () => {
    const tree = await growField("ordinary", 5, { limbOrder: 1 });
    expect(tree.bounds.max[1]).toBeGreaterThan(tree.bounds.min[1]);
    tree.release();
    const engine = await TreeEngine.create();
    expect(engine.build(presetById("ordinary"), { structure: true }).diagnostics.nodes).toBeGreaterThan(0);
    engine.dispose();
  });

  it("grows every shipped preset, byte for byte the main entry's field, bounds included (fn-150 R2, R3, R5)", async () => {
    const module = await compileField(source);
    const engine = await TreeEngine.create(readFileSync("src/browser/telperion.wasm"));
    for (const preset of PRESETS) {
      for (const seed of [1, 4242]) {
        const tree = await growField(preset.id, seed, { source: module });
        const family = presetById(preset.id);
        family.skeleton.seed = seed;
        const field = engine.build(family, { field: true }).field!;
        expect(field.bounds, `${preset.id} ${seed}`).toEqual(tree.bounds);
        const g = grid(tree.bounds, 10);
        const slim = tree.query(g.cells), full = field.query(g.cells);
        expect(slim.flags.some(f => f & 2), `${preset.id} ${seed} foliage`).toBe(true);
        expect(full.flags, `${preset.id} ${seed}`).toEqual(slim.flags);
        expect(full.woodRadius).toEqual(slim.woodRadius);
        expect(full.leaves).toEqual(slim.leaves);
        expect(full.limbs).toEqual(slim.limbs);
        tree.release();
      }
    }
    engine.dispose();
  });

  it("grows the date palm at every seed the owner tried (fn-150 R1)", async () => {
    const module = await compileField(source);
    for (const seed of [1, 7, 1407, 4242]) {
      const tree = await growField("date-palm", seed, { source: module });
      const answer = tree.query(grid(tree.bounds, 8).cells);
      expect(answer.flags.some(f => f & 1), `seed ${seed} wood`).toBe(true);
      expect(answer.flags.some(f => f & 2), `seed ${seed} fronds`).toBe(true);
      tree.release();
    }
  });

  it("refuses what the core refuses, whole", async () => {
    await expect(growField("no-such-tree", 1, { source })).rejects.toThrow("preset identity");
    await expect(growField("european-beech", 1, { source })).rejects.toThrow("preset identity");
    await expect(growField("ordinary", 1.5, { source })).rejects.toThrow("seed");
    await expect(growField("ordinary", 2 ** 32, { source })).rejects.toThrow("seed");
    await expect(growField("ordinary", 1, { limbOrder: -1, source })).rejects.toThrow("limbOrder");
    const tree = await growField("ordinary", 1, { source });
    expect(() => tree.query(new Float64Array(5))).toThrow("packed");
    expect(() => tree.query(new Float64Array([0, 1, 0, 0.5, 0, 2, 0, -0.1]))).toThrow("field query");
    expect(() => tree.query(new Float64Array([0, 1, 0, 0.5, Number.NaN, 2, 0, 0.1]))).toThrow("field query");
    expect(tree.query(new Float64Array([0, 1, 0, 0.5])).flags.length).toBe(1);
    tree.release();
  });
});
