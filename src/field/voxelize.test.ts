import { describe, expect, it } from "vitest";
import { ORDINARY, TreeEngine } from "../browser/core";
import type { FieldAnswer, FieldGrid } from "./index";
import { NO_LIMB } from "./index";
import { centre, coin, grid, voxelize, type ThinRule } from "./voxelize";

/* The example voxelizer on a fixture grid of four cells a side, every rule
 * and dial hand-checked. Cell index is i + 4 * (j + 4 * k). */
const N = 4;
const BOUNDS = { min: [-1, 0, -1] as [number, number, number], max: [1, 4, 1] as [number, number, number] };
function fixture(): FieldAnswer {
  const count = N ** 3;
  const a = { flags: new Uint8Array(count), woodRadius: new Float32Array(count), leaves: new Float32Array(count), limbs: new Uint32Array(count).fill(NO_LIMB) };
  const wood = (i: number, radius: number) => { a.flags[i] |= 1; a.woodRadius[i] = radius; };
  const leaf = (i: number, limb: number, leaves: number) => { a.flags[i] |= 2; a.limbs[i] = limb; a.leaves[i] = leaves; };
  wood(0, 0.05); // the trunk
  wood(1, 0.01); // a twig under foliage
  wood(4, 0.03); // a limb inside the crown
  leaf(4, 7, 9);
  leaf(1, 7, 1); leaf(5, 7, 3); leaf(9, 7, 2); leaf(17, 7, 0.5); // limb 7, a column with one cell behind it
  leaf(2, 9, 5); leaf(6, 9, 4); leaf(10, 9, 1); // limb 9, the column beside it
  return a;
}
const G = grid(BOUNDS, N);
const field = (answer: FieldAnswer = fixture()): FieldGrid => ({ ...G, answer });

describe("the grid", () => {
  it("spans the longest axis with n - 2 cells, centred, one cell below the lowest point", () => {
    expect(G.cell).toBe(2);
    expect(G.origin).toEqual([-4, -2, -4]);
    expect(centre(G, 0)).toEqual([-3, -1, -3]);
    expect(centre(G, N ** 3 - 1)).toEqual([3, 5, 3]);
    expect(centre(G, 1 + 4 * (2 + 4 * 3))).toEqual([-1, 3, 3]);
    for (const n of [2, 257, 4.5]) expect(() => grid(BOUNDS, n)).toThrow("field grid side");
  });
});

describe("the grid a field is read to", () => {
  it("is the core's grid for the same box: its side, cell, origin and cell order", async () => {
    const engine = await TreeEngine.create();
    const family = structuredClone(ORDINARY);
    family.skeleton.seed = 42; family.skeleton.attractors = 500;
    const read = (n: number) => engine.build(family, { structure: true, field: { n } });
    const coarse = read(8).field!;
    // The box the core read over, as much of it as its grid depends on: the
    // longest axis, the centre in x and z and the lowest point.
    const span = coarse.cell * (coarse.n - 2), low = coarse.origin[1] + coarse.cell;
    const [x, z] = [0, 2].map(a => coarse.origin[a] + (coarse.n * coarse.cell) / 2);
    const box = { min: [x - span / 2, low, z - span / 2] as [number, number, number], max: [x + span / 2, low + span, z + span / 2] as [number, number, number] };
    const fine = read(24), field = fine.field!, g = grid(box, 24);
    expect(g.n).toBe(field.n);
    expect(g.cell).toBeCloseTo(field.cell, 12);
    for (const a of [0, 1, 2]) expect(g.origin[a]).toBeCloseTo(field.origin[a], 12);
    // Every node stands in its own wood, so the cell the grid's order names
    // for it holds wood; the order with x and z exchanged misses some.
    const values = fine.structure!.values, wood = (index: number) => field.answer.flags[index] & 1;
    let exchanged = 0;
    for (let node = 0; node < values.length / 6; node++) {
      const [i, j, k] = [0, 1, 2].map(a => Math.floor((values[node * 6 + a] - g.origin[a]) / g.cell));
      expect(wood(i + g.n * (j + g.n * k)), `node ${node}`).toBe(1);
      exchanged += 1 - wood(k + g.n * (j + g.n * i));
    }
    expect(exchanged).toBeGreaterThan(0);
    engine.dispose();
  });
});

describe("the wood cutoff", () => {
  it("draws wood at or above the cutoff in metres, whatever foliage covers it", () => {
    expect([...voxelize(field(), { woodCutoff: 0.02 }).wood]).toEqual([0, 4]);
    expect([...voxelize(field(), { woodCutoff: 0.04 }).wood]).toEqual([0]);
    expect([...voxelize(field(), { woodCutoff: 0 }).wood]).toEqual([0, 1, 4]);
    const wood = voxelize(field(), { woodCutoff: 0.02 });
    expect(wood.foliage.length).toBe(0);
    expect(wood.clumps).toBe(0);
  });
  it("refuses a mismatched answer and a bad dial", () => {
    expect(() => voxelize(field({ ...fixture(), flags: new Uint8Array(3) }), { woodCutoff: 0.02 })).toThrow("cells");
    expect(() => voxelize(field(), { woodCutoff: -1 })).toThrow("metres");
    expect(() => voxelize(field(), { woodCutoff: 0.02, thin: { rule: "coin", keep: 2 } })).toThrow("fraction");
  });
});

describe("the thinning rules", () => {
  const thin = (rule: ThinRule, keep: number) => voxelize(field(), { woodCutoff: 0.02, thin: { rule, keep } });
  it("counts the limb systems the foliage was thinned over and never draws foliage over drawn wood", () => {
    for (const rule of ["coin", "density", "mass", "tuft", "gap"] as const) {
      const cubes = thin(rule, 1);
      expect(cubes.clumps).toBe(2);
      expect([...cubes.foliage].sort((a, b) => a - b)).toEqual([1, 2, 5, 6, 9, 10, 17]);
      expect([...cubes.wood]).toEqual([0, 4]);
    }
  });
  it("coin drops whole limb systems by a stable coin per limb", () => {
    expect(thin("coin", 0).foliage.length).toBe(0);
    // Each limb's coin lands at one fraction; a keep between the two keeps
    // the limb whose coin is lower and drops the other whole.
    const landing = (limb: number) => { let lo = 0, hi = 1; for (let s = 0; s < 40; s++) { const mid = (lo + hi) / 2; if (coin(limb, mid)) hi = mid; else lo = mid; } return hi; };
    const [seven, nine] = [landing(7), landing(9)];
    expect(seven).not.toBe(nine);
    const keep = (seven + nine) / 2;
    expect([...thin("coin", keep).foliage]).toEqual(seven < nine ? [1, 5, 9, 17] : [2, 6, 10]);
  });
  it("density keeps the cells with the most leaf stations", () => {
    expect([...thin("density", 0.5).foliage]).toEqual([2, 6, 5, 9]);
  });
  it("mass keeps the cells nearest drawn wood, ties to the denser", () => {
    expect([...thin("mass", 0.5).foliage]).toEqual([5, 1, 2, 6]);
  });
  it("tuft keeps each system's cells nearest its own centre, scaled by its spread", () => {
    expect([...thin("tuft", 0.5).foliage]).toEqual([6, 5, 1, 17]);
  });
  it("gap erodes a system where it meets another and keeps its interior", () => {
    expect([...thin("gap", 0.5).foliage]).toEqual([17, 2, 6, 5]);
  });
});
