// One tree's field, as a consumer would read it through `telperion/field`,
// drawn three ways while the camera turns about it. Every style starts from a
// grid of cells the field answered in one batch query; nothing else about the
// tree is read.
//   smooth  a density from the answered flags and leaf counts, lightly
//           blurred, ray-marched as a smooth surface: voxels as a web page
//           draws them
//   blocks  whole cubes read by the example voxelizer, each face given a
//           procedural pixel texture: a block game's trees, with no game's
//           assets
//   points  a point cloud: points scattered in each answered cell, as many as
//           the cell's leaf stations say, the wood cells in bark
// Run from the evidence folder (frames are split across processes):
//   node --import ./video/draw/ts-resolve.ts video/draw/consumers.ts <species>
//        <seed> <style> <outdir> <WxH> <frames> <azimuth> <sweep> [first last]
import { mkdirSync, readFileSync } from 'node:fs';
import { spawn } from 'node:child_process';
import { availableParallelism } from 'node:os';
import { fileURLToPath } from 'node:url';
import { compileField, growField, type FieldAnswer } from '../../../../../src/field/index.ts';
import { grid, voxelize, type Grid } from '../../../../../src/field/voxelize.ts';
import { Camera, Canvas, ease, fit, normalise, dot, type Rgb, type Vec3 } from './canvas.ts';

type Style = 'smooth' | 'blocks' | 'points';
const RESOLUTION: Record<Style, number> = { smooth: 88, blocks: 36, points: 120 };
const BACKGROUND: Rgb = [0.012, 0.014, 0.02];
const SUN = normalise([0.5, 0.8, 0.35]);
const WOOD = 1, LEAF = 2;

interface Field { grid: Grid; answer: FieldAnswer; height: number; across: number; centre: Vec3 }

async function field(species: string, seed: number, n: number): Promise<Field> {
  const wasm = new URL('../../../../../src/field/telperion-field.wasm', import.meta.url);
  const source = await compileField(readFileSync(fileURLToPath(wasm)));
  const tree = await growField(species, seed, { source });
  const g = grid(tree.bounds, n);
  const answer = tree.query(g.cells);
  const { min, max } = tree.bounds;
  tree.release();
  return { grid: g, answer, height: max[1] - min[1],
    across: Math.max(max[0] - min[0], max[2] - min[2]),
    centre: [(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2] };
}

/// Cheap stable noise in [0, 1) from integers.
function hash(...values: number[]): number {
  let h = 2166136261;
  for (const v of values) h = Math.imul(h ^ (v | 0), 16777619) ^ (h >>> 13);
  return ((h >>> 0) % 10007) / 10007;
}

/// Where a ray enters and leaves the grid's box, or nothing.
function clip(g: Grid, o: Vec3, d: Vec3): [number, number] | undefined {
  let near = 0, far = Infinity;
  for (let a = 0; a < 3; a++) {
    const lo = (g.origin[a] - o[a]) / d[a], hi = (g.origin[a] + g.n * g.cell - o[a]) / d[a];
    near = Math.max(near, Math.min(lo, hi));
    far = Math.min(far, Math.max(lo, hi));
  }
  return near < far ? [near, far] : undefined;
}

// --- blocks ------------------------------------------------------------

function blockCells(f: Field): Uint8Array {
  const { grid: g, answer } = f;
  const cubes = voxelize(g, answer, { woodCutoff: g.cell * 0.18, thin: { rule: 'gap', keep: 0.55 } });
  const cells = new Uint8Array(g.n ** 3);
  for (const i of cubes.foliage) cells[i] = LEAF;
  for (const i of cubes.wood) cells[i] = WOOD;
  return cells;
}

const BARK: Rgb = [0.2, 0.12, 0.06], LEAVES: Rgb = [0.12, 0.3, 0.07];

/// A face's pixel texture: eight texels a side, bark in vertical grain and
/// leaves in speckle, every face lit by the side it faces and its rim darker.
function texel(kind: number, cell: number, axis: number, u: number, v: number): Rgb {
  const tu = Math.floor(u * 8), tv = Math.floor(v * 8);
  const rim = tu === 0 || tv === 0 || tu === 7 || tv === 7 ? 0.8 : 1;
  const base = kind === WOOD ? BARK : LEAVES;
  const n = kind === WOOD ? 0.7 + 0.5 * hash(cell, tu, axis) * (0.8 + 0.4 * hash(tv >> 2, tu))
    : 0.55 + 0.9 * hash(cell, tu, tv, axis);
  const light = (axis === 1 ? 1.0 : axis === 0 ? 0.78 : 0.62) * rim;
  return [base[0] * n * light, base[1] * n * light, base[2] * n * light];
}

function blocks(f: Field, cells: Uint8Array, camera: Camera): Canvas {
  const { grid: g } = f;
  const canvas = new Canvas(camera.width, camera.height, BACKGROUND);
  for (let y = 0; y < camera.height; y++) {
    for (let x = 0; x < camera.width; x++) {
      const [o, d] = camera.ray(x + 0.5, y + 0.5);
      const span = clip(g, o, d);
      if (!span) continue;
      const p: Vec3 = [0, 1, 2].map((a) => (o[a] + d[a] * (span[0] + 1e-6) - g.origin[a]) / g.cell) as Vec3;
      const c = p.map((v) => Math.min(g.n - 1, Math.max(0, Math.floor(v))));
      const step = d.map((v) => (v > 0 ? 1 : -1));
      const next = [0, 1, 2].map((a) => ((c[a] + (d[a] > 0 ? 1 : 0) - p[a]) / d[a]) * g.cell);
      const delta = d.map((v) => Math.abs(g.cell / v));
      let t = span[0], axis = 1;
      for (let guard = 0; guard < g.n * 3; guard++) {
        const index = (c[2] * g.n + c[1]) * g.n + c[0];
        const kind = cells[index];
        if (kind) {
          const hit = [0, 1, 2].map((a) => (o[a] + d[a] * t - g.origin[a]) / g.cell);
          const [ua, va] = axis === 0 ? [2, 1] : axis === 1 ? [0, 2] : [0, 1];
          const colour = texel(kind, index, axis, hit[ua] - Math.floor(hit[ua]),
            hit[va] - Math.floor(hit[va]));
          canvas.blend(x, y, colour, 1);
          break;
        }
        axis = next[0] < next[1] ? (next[0] < next[2] ? 0 : 2) : (next[1] < next[2] ? 1 : 2);
        t = span[0] + next[axis];
        if (t > span[1]) break;
        c[axis] += step[axis];
        next[axis] += delta[axis];
        if (c[axis] < 0 || c[axis] >= g.n) break;
      }
    }
  }
  return canvas;
}

// --- smooth ------------------------------------------------------------

interface Density { value: Float32Array; wood: Float32Array }

function density(f: Field): Density {
  const { grid: g, answer } = f;
  const n = g.n, size = n ** 3;
  const raw = new Float32Array(size), wood = new Float32Array(size);
  for (let i = 0; i < size; i++) {
    const foliage = answer.flags[i] & 2 ? Math.min(1, 0.45 + answer.leaves[i] / 40) : 0;
    const w = answer.woodRadius[i] > g.cell * 0.2 ? 1 : 0;
    raw[i] = Math.max(foliage, w);
    wood[i] = w;
  }
  const blur = (source: Float32Array): Float32Array => {
    let a = source;
    for (let axis = 0; axis < 3; axis++) {
      const b = new Float32Array(size);
      const stride = axis === 0 ? 1 : axis === 1 ? n : n * n;
      for (let i = 0; i < size; i++) {
        const coord = Math.floor(i / stride) % n;
        const lo = coord > 0 ? a[i - stride] : 0, hi = coord < n - 1 ? a[i + stride] : 0;
        b[i] = 0.25 * lo + 0.5 * a[i] + 0.25 * hi;
      }
      a = b;
    }
    return a;
  };
  return { value: blur(raw), wood: blur(wood) };
}

function sample(g: Grid, field: Float32Array, p: Vec3): number {
  const n = g.n;
  const x = p[0] - 0.5, y = p[1] - 0.5, z = p[2] - 0.5;
  const i = Math.floor(x), j = Math.floor(y), k = Math.floor(z);
  if (i < 0 || j < 0 || k < 0 || i >= n - 1 || j >= n - 1 || k >= n - 1) return 0;
  const fx = x - i, fy = y - j, fz = z - k;
  const at = (a: number, b: number, c: number) => field[((k + c) * n + j + b) * n + i + a];
  const lerp = (a: number, b: number, t: number) => a + (b - a) * t;
  return lerp(
    lerp(lerp(at(0, 0, 0), at(1, 0, 0), fx), lerp(at(0, 1, 0), at(1, 1, 0), fx), fy),
    lerp(lerp(at(0, 0, 1), at(1, 0, 1), fx), lerp(at(0, 1, 1), at(1, 1, 1), fx), fy), fz);
}

const ISO = 0.32;
const SMOOTH_WOOD: Rgb = [0.34, 0.24, 0.16], SMOOTH_LEAF: Rgb = [0.3, 0.52, 0.22];

function smooth(f: Field, d: Density, camera: Camera): Canvas {
  const { grid: g } = f;
  const canvas = new Canvas(camera.width, camera.height, BACKGROUND);
  const toGrid = (w: Vec3): Vec3 => [0, 1, 2].map((a) => (w[a] - g.origin[a]) / g.cell) as Vec3;
  for (let y = 0; y < camera.height; y++) {
    for (let x = 0; x < camera.width; x++) {
      const [o, dir] = camera.ray(x + 0.5, y + 0.5);
      const span = clip(g, o, dir);
      if (!span) continue;
      const step = g.cell * 0.4;
      let t = span[0], prev = 0, hit = -1;
      for (; t < span[1]; t += step) {
        const v = sample(g, d.value, toGrid([o[0] + dir[0] * t, o[1] + dir[1] * t, o[2] + dir[2] * t]));
        if (v >= ISO) {
          hit = t - step * ((v - ISO) / Math.max(v - prev, 1e-6));
          break;
        }
        prev = v;
      }
      if (hit < 0) continue;
      const p = toGrid([o[0] + dir[0] * hit, o[1] + dir[1] * hit, o[2] + dir[2] * hit]);
      const e = 0.6;
      const grad = normalise([0, 1, 2].map((a) => {
        const lo = [...p] as Vec3, hi = [...p] as Vec3;
        lo[a] -= e; hi[a] += e;
        return sample(g, d.value, lo) - sample(g, d.value, hi);
      }) as Vec3);
      const lambert = Math.max(0, dot(grad, SUN));
      const rim = (1 - Math.abs(dot(grad, dir))) ** 3 * 0.35;
      const w = Math.min(1, sample(g, d.wood, p) / Math.max(sample(g, d.value, p), 1e-6));
      const light = 0.22 + 0.95 * lambert + rim;
      const colour: Rgb = [0, 1, 2].map((c) =>
        (SMOOTH_LEAF[c] + (SMOOTH_WOOD[c] - SMOOTH_LEAF[c]) * w) * light) as Rgb;
      canvas.blend(x, y, colour, 1);
    }
  }
  return canvas;
}

// --- points ------------------------------------------------------------

function cloud(f: Field): Float32Array {
  const { grid: g, answer } = f;
  const out: number[] = [];
  for (let i = 0; i < g.n ** 3; i++) {
    const wood = answer.woodRadius[i] > g.cell * 0.08;
    const count = wood ? 2 : answer.flags[i] & 2 ? Math.min(4, 1 + Math.floor(answer.leaves[i] / 12)) : 0;
    const cx = i % g.n, cy = Math.floor(i / g.n) % g.n, cz = Math.floor(i / g.n ** 2);
    for (let k = 0; k < count; k++) {
      out.push(g.origin[0] + (cx + hash(i, k, 1)) * g.cell, g.origin[1] + (cy + hash(i, k, 2)) * g.cell,
        g.origin[2] + (cz + hash(i, k, 3)) * g.cell, wood ? 1 : 0);
    }
  }
  return Float32Array.from(out);
}

function points(f: Field, p: Float32Array, camera: Camera): Canvas {
  const canvas = new Canvas(camera.width, camera.height, BACKGROUND);
  const top = f.centre[1] + f.height / 2, bottom = f.centre[1] - f.height / 2;
  for (let i = 0; i < p.length; i += 4) {
    const s = camera.project([p[i], p[i + 1], p[i + 2]]);
    if (s[2] <= 0) continue;
    const up = (p[i + 1] - bottom) / (top - bottom);
    const colour: Rgb = p[i + 3] ? [0.8, 0.55, 0.32] : [0.35 + 0.4 * up, 0.8, 0.7 - 0.25 * up];
    canvas.disc(s[0], s[1], 0.7, colour, 0.4);
  }
  return canvas;
}

// --- driver ------------------------------------------------------------

async function render(args: string[]): Promise<void> {
  const [species, seed, style, out, size, frames, azimuth, sweep, first, last] = args;
  const kind = style as Style;
  const [width, height] = size.split('x').map(Number);
  const count = Number(frames);
  mkdirSync(out, { recursive: true });
  const f = await field(species, Number(seed), RESOLUTION[kind]);
  const cells = kind === 'blocks' ? blockCells(f) : undefined;
  const dense = kind === 'smooth' ? density(f) : undefined;
  const cloudPoints = kind === 'points' ? cloud(f) : undefined;
  for (let frame = Number(first); frame <= Number(last); frame++) {
    const t = count > 1 ? frame / (count - 1) : 0;
    // Far enough that the crown fits across the frame at any turn, not only
    // the height.
    const distance = Math.max(fit(f.height, 0.84, 34), fit(f.across * height / width, 0.94, 34));
    const camera = new Camera(f.centre, Number(azimuth) + Number(sweep) * ease(t), 14,
      distance, 34, width, height);
    const canvas = cells ? blocks(f, cells, camera) : dense ? smooth(f, dense, camera)
      : points(f, cloudPoints!, camera);
    canvas.write(`${out}/frame-${String(frame + 1).padStart(4, '0')}.png`);
  }
}

async function main(): Promise<void> {
  const args = process.argv.slice(2);
  if (args.length < 8 || !['smooth', 'blocks', 'points'].includes(args[2])) {
    throw new Error('usage: consumers.ts <species> <seed> <smooth|blocks|points> <outdir> '
      + '<WxH> <frames> <azimuth> <sweep> [first last]');
  }
  if (args.length === 10) return render(args);
  const count = Number(args[5]), workers = Math.min(availableParallelism() - 2, count);
  const jobs = Array.from({ length: workers }, (_, w) => {
    const first = Math.floor((w * count) / workers), last = Math.floor(((w + 1) * count) / workers) - 1;
    return new Promise<void>((done, fail) => {
      const child = spawn(process.execPath, [...process.execArgv, fileURLToPath(import.meta.url), ...args.slice(0, 8),
        String(first), String(last)], { stdio: 'inherit' });
      child.on('exit', (code) => (code === 0 ? done() : fail(new Error(`worker ${w}: ${code}`))));
    });
  });
  await Promise.all(jobs);
}

await main();
