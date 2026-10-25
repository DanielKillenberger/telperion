/* An example voxelizer beside the field entry point: the grid every field
 * is read to, and the cube lists a black-and-white voxel tree draws from a
 * field's answers. A consumer imports it, copies it, or writes its own; the
 * field in ./index is the entry point, this is one reading of its answers.
 * It draws nothing.
 *
 * The rules are the ones experiments/voxel-field settled under fn-100: wood
 * draws where the cell's thickest wood is at least the cutoff, through any
 * foliage; foliage draws elsewhere where the field reports it, thinned to a
 * keep fraction by one of five rules. Thinner wood is the twigs, and a
 * dropped clump takes them. Flowers (fn-316) draw where the field counts
 * them and no wood draws, in place of the foliage there: a cell with leaves
 * keeps its flowers where its foliage survives the thinning, and a cell with
 * flowers alone, which no limb owns, always keeps them. */
import type { Bounds, FieldGrid } from "./index";

/** The grid a field is read to (fn-257): `n` cells a side, `cell` metres
 * each, `n - 2` of them spanning the box's longest axis, centred on the box
 * in x and z with one empty cell below its lowest point. Cell `i + n * (j +
 * n * k)` is centred at `origin + (i + 0.5, j + 0.5, k + 0.5) * cell`. The
 * CPU field, the device field and this file read this one definition. */
export interface Grid { n: number; cell: number; origin: [number, number, number] }
/** `n` where it is a side a grid may have: a whole number from 3 to 256. */
export function side(n: number): number {
  if (!Number.isInteger(n) || n < 3 || n > 256) throw RangeError(`field grid side = ${n}: a whole number from 3 to 256`);
  return n;
}
/** The grid of `n` cells a side over `bounds`. */
export function grid(bounds: Bounds, n: number): Grid {
  side(n);
  const { min, max } = bounds;
  const cell = Math.max(max[0] - min[0], max[1] - min[1], max[2] - min[2]) / (n - 2);
  const origin: [number, number, number] = [(min[0] + max[0]) / 2 - (n * cell) / 2, min[1] - cell, (min[2] + max[2]) / 2 - (n * cell) / 2];
  return { n, cell, origin };
}
/** The centre of cell `index` in metres. */
export function centre({ n, cell, origin }: Grid, index: number): [number, number, number] {
  return [origin[0] + ((index % n) + 0.5) * cell, origin[1] + (Math.floor(index / n) % n + 0.5) * cell, origin[2] + (Math.floor(index / (n * n)) + 0.5) * cell];
}

/** How foliage cells thin to the keep fraction. `coin` drops whole limb
 * systems by a stable coin per limb id. `density` keeps the cells with the
 * most leaf stations. `mass` keeps the cells nearest to drawn wood. `tuft`
 * keeps each limb system's rounded mass about its own centre. `gap` erodes
 * each system where it meets a neighbouring one, so the crown's outer
 * surface stays and a hanging curtain stays a curtain. */
export type ThinRule = "coin" | "density" | "mass" | "tuft" | "gap";
export interface Thinning { rule: ThinRule; keep: number }
export interface Dials {
  /** Wood draws where the cell's thickest wood radius is at least this, in
   * metres; the owner's reading was about 2 cm for a broadleaf and 1 cm
   * for the spruce. */
  woodCutoff: number;
  /** Left out, no foliage draws: the wood alone. */
  thin?: Thinning;
}
/** Cell indices to draw as wood, as foliage and as flowers, each cell in one
 * of them at most, and the number of distinct limb systems the foliage was
 * thinned over. */
export interface Cubes { wood: Uint32Array; foliage: Uint32Array; flowers: Uint32Array; clumps: number }
const WOOD = 1, LEAF = 2, FLOWER = 4;

/** The cubes a field draws as: `field` is a grid with its answers, as
 * `growField` returns it. */
export function voxelize(field: FieldGrid, dials: Dials): Cubes {
  const { n, answer } = field;
  const count = n * n * n;
  if (answer.flags.length !== count) throw RangeError(`the answer has ${answer.flags.length} cells, the grid ${count}`);
  if (!(dials.woodCutoff >= 0)) throw RangeError("woodCutoff is a radius in metres");
  const { flags, limbs, leaves, woodRadius } = answer;
  const drawn = new Uint8Array(count);
  const wood: number[] = [];
  const limb = (i: number) => (flags[i] & 1) !== 0 && woodRadius[i] >= dials.woodCutoff;
  for (let i = 0; i < count; i++) if (limb(i)) { drawn[i] = WOOD; wood.push(i); }
  if (dials.thin === undefined) return { wood: Uint32Array.from(wood), foliage: new Uint32Array(0), flowers: new Uint32Array(0), clumps: 0 };
  const { rule, keep } = dials.thin;
  if (!(keep >= 0 && keep <= 1)) throw RangeError("keep is a fraction");
  const coins = new Set<number>();
  const foliage: number[] = [];
  for (let i = 0; i < count; i++) if (!limb(i) && flags[i] & 2) { foliage.push(i); coins.add(limbs[i]); }
  const kept = rule === "coin" ? foliage.filter(i => coin(limbs[i], keep)) : thinned(field, foliage, drawn, rule, keep);
  const survived = new Set(kept);
  const flowers: number[] = [];
  for (let i = 0; i < count; i++) if (!limb(i) && flags[i] & FLOWER && (!(flags[i] & LEAF) || survived.has(i))) flowers.push(i);
  const bloom = new Set(flowers);
  return { wood: Uint32Array.from(wood), foliage: Uint32Array.from(kept.filter(i => !bloom.has(i))), flowers: Uint32Array.from(flowers), clumps: coins.size };
}
/** The foliage cells `rule` ranks highest, `keep` of them. */
function thinned(field: FieldGrid, foliage: number[], drawn: Uint8Array, rule: Exclude<ThinRule, "coin">, keep: number): number[] {
  const score = SCORES[rule](field, foliage, drawn, field.answer.limbs, field.answer.leaves);
  const ranked = foliage.map((i, at) => ({ i, at })).sort((a, b) => score[b.at] - score[a.at]);
  return ranked.slice(0, Math.round(foliage.length * keep)).map(r => r.i);
}
/** A stable coin per limb id: the same limb always answers the same. */
export function coin(limb: number, keep: number): boolean {
  let h = Math.imul(limb ^ 0x9e3779b9, 0x85ebca6b);
  h = Math.imul(h ^ (h >>> 13), 0xc2b2ae35);
  return ((h ^ (h >>> 16)) >>> 0) / 2 ** 32 < keep;
}

type Score = (grid: Grid, foliage: number[], drawn: Uint8Array, limbs: Uint32Array, leaves: Float32Array) => Float64Array;
const at = (n: number, i: number): [number, number, number] => [i % n, Math.floor(i / n) % n, Math.floor(i / (n * n))];
const STEPS = [[1, 0, 0], [-1, 0, 0], [0, 1, 0], [0, -1, 0], [0, 0, 1], [0, 0, -1]] as const;
function neighbours(n: number, index: number): number[] {
  const [i, j, k] = at(n, index);
  const out: number[] = [];
  for (const [di, dj, dk] of STEPS) {
    const a = i + di, b = j + dj, c = k + dk;
    if (a >= 0 && b >= 0 && c >= 0 && a < n && b < n && c < n) out.push((c * n + b) * n + a);
  }
  return out;
}
/** Grid distance from the seeds by breadth-first search, -1 where unreached;
 * `open` says which cells the search may enter. */
function distances(n: number, seeds: number[], open: (from: number, to: number) => boolean): Int32Array {
  const dist = new Int32Array(n * n * n).fill(-1);
  let queue = seeds;
  for (const s of seeds) dist[s] = 0;
  while (queue.length) {
    const next: number[] = [];
    for (const from of queue) for (const to of neighbours(n, from)) if (dist[to] === -1 && open(from, to)) { dist[to] = dist[from] + 1; next.push(to); }
    queue = next;
  }
  return dist;
}
const SCORES: Record<Exclude<ThinRule, "coin">, Score> = {
  density: (_grid, foliage, _drawn, _limbs, leaves) => Float64Array.from(foliage, i => leaves[i]),
  // Nearer drawn wood scores higher; ties broken by density so masses stay full.
  mass: ({ n }, foliage, drawn, _limbs, leaves) => {
    const seeds: number[] = [];
    for (let i = 0; i < drawn.length; i++) if (drawn[i] === WOOD) seeds.push(i);
    const dist = distances(n, seeds, () => true);
    return Float64Array.from(foliage, i => -dist[i] + leaves[i] * 1e-6);
  },
  // A cell scores by how deep it sits in its own system, so the fringe
  // between neighbouring systems goes first and the branch to each mass shows.
  tuft: ({ n }, foliage, _drawn, limbs) => {
    const sum = new Map<number, { i: number; j: number; k: number; c: number }>();
    for (const index of foliage) {
      const [i, j, k] = at(n, index);
      const e = sum.get(limbs[index]) ?? { i: 0, j: 0, k: 0, c: 0 };
      e.i += i; e.j += j; e.k += k; e.c++;
      sum.set(limbs[index], e);
    }
    const depth = (index: number) => { const [i, j, k] = at(n, index); const e = sum.get(limbs[index])!; return Math.hypot(i - e.i / e.c, j - e.j / e.c, k - e.k / e.c); };
    const spread = new Map<number, { s: number; c: number }>();
    for (const index of foliage) { const e = spread.get(limbs[index]) ?? { s: 0, c: 0 }; e.s += depth(index); e.c++; spread.set(limbs[index], e); }
    return Float64Array.from(foliage, index => { const e = spread.get(limbs[index])!; return -depth(index) / (e.s / e.c + 1e-9); });
  },
  // The grid distance to the nearest cell of a different system; a system
  // with no neighbour never erodes and scores as its deepest cell would.
  gap: ({ n }, foliage, _drawn, limbs, leaves) => {
    const isLeaf = new Uint8Array(n * n * n);
    for (const i of foliage) isLeaf[i] = 1;
    const seeds = foliage.filter(i => neighbours(n, i).some(m => isLeaf[m] && limbs[m] !== limbs[i]));
    const dist = distances(n, seeds, (from, to) => isLeaf[to] === 1 && limbs[to] === limbs[from]);
    return Float64Array.from(foliage, i => (dist[i] === -1 ? n : dist[i]) + leaves[i] * 1e-6);
  },
};
