// A black-and-white voxel tree from the slim field entry point (fn-101): the
// tree grows in the slim Wasm, one batch query answers a cubic grid, and the
// example voxelizer turns the answers into cube lists. Every rule and dial
// here is the voxelizer's; this script only chooses them, draws the sheet
// and logs the counts.
//
// Run: node experiments/voxel-field/voxels.mjs [resolution] [keep,keep,...]
//   LIMB_ORDER=<n>  limbs named at that lateral order (the family's otherwise)
//   THIN=coin|density|mass|tuft|gap  the thinning rule (coin)
//   WOOD_CUT=<cells>  wood draws at this many cells of radius (0.2)
//   CUT_METRES=1  the keeps and WOOD_CUT are metres of radius, not cells
//   WOOD_ONLY=1  no foliage; each row's keep is the wood cutoff instead
//   SEED=<n>  the seed every species grows at (42, the families' own)
//   WASM=<path>  another build of the slim binding
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { deflateSync } from 'node:zlib';

const root = new URL('../../', import.meta.url);
const { growField, compileField } = await import(new URL('src/field/index.ts', root));
const { grid, voxelize } = await import(new URL('src/field/voxelize.ts', root));
const out = new URL('out/', import.meta.url);
const N = Number(process.argv[2] ?? 64);
const KEEPS = (process.argv[3] ?? '0.5,0.3,0.15').split(',').map(Number);
const SPECIES = ['oregon-white-oak', 'silver-birch', 'norway-spruce'];
const WOOD = 1, LEAF = 2;
const LIMB_ORDER = process.env.LIMB_ORDER === undefined ? undefined : Number(process.env.LIMB_ORDER);
const THIN = process.env.THIN ?? 'coin';
const WOOD_CUT = Number(process.env.WOOD_CUT ?? 0.2);
const WOOD_ONLY = process.env.WOOD_ONLY === '1';
const CUT_METRES = process.env.CUT_METRES === '1';
const SEED = Number(process.env.SEED ?? 42);
const module = await compileField(readFileSync(process.env.WASM ? new URL(process.env.WASM, `file://${process.cwd()}/`) : new URL('src/field/telperion-field.wasm', root)));

/// The grid the voxelizer defines over the tree's bounds, answered in one batch.
async function field(id) {
  const tree = await growField(id, SEED, { limbOrder: LIMB_ORDER, source: module });
  const g = grid(tree.bounds, N);
  const answer = tree.query(g.cells);
  tree.release();
  return { grid: g, answer };
}

/// One row's cubes on the grid: a cell byte per cell for the renderer.
function cubes({ grid: g, answer }, keep) {
  const cut = WOOD_ONLY ? keep : WOOD_CUT;
  const dials = { woodCutoff: CUT_METRES ? cut : g.cell * cut, thin: WOOD_ONLY ? undefined : { rule: THIN, keep } };
  const v = voxelize(g, answer, dials);
  const cells = new Uint8Array(N * N * N);
  for (const i of v.wood) cells[i] = WOOD;
  for (const i of v.foliage) cells[i] = LEAF;
  return { grid: cells, wood: v.wood.length, leaf: v.foliage.length, clumps: v.clumps };
}

/// Orthographic ray march from an isometric direction; one face tone per axis,
/// black lines wherever the voxel or the face changes between pixels.
function render(grid, size) {
  const pixels = new Uint8Array(size * size).fill(255);
  const ids = new Int32Array(size * size).fill(-1);
  const norm = v => { const l = Math.hypot(...v); return v.map(c => c / l); };
  const dir = norm([-1, -0.8, -1]);
  const right = norm([1, 0, -1]);
  const up = norm([dir[1] * right[2] - dir[2] * right[1], dir[2] * right[0] - dir[0] * right[2], dir[0] * right[1] - dir[1] * right[0]]).map(c => -c);
  const span = N * 1.35;
  const tone = { [WOOD]: [40, 0, 20], [LEAF]: [255, 205, 235] };
  for (let py = 0; py < size; py++) for (let px = 0; px < size; px++) {
    const u = (px / size - .5) * span, v = (.5 - py / size) * span;
    const p = [0, 1, 2].map(a => N / 2 + right[a] * u + up[a] * v - dir[a] * N * 2);
    let tEnter = 0, tExit = Infinity;
    for (let a = 0; a < 3; a++) {
      const t0 = (0 - p[a]) / dir[a], t1 = (N - p[a]) / dir[a];
      tEnter = Math.max(tEnter, Math.min(t0, t1));
      tExit = Math.min(tExit, Math.max(t0, t1));
    }
    if (tEnter >= tExit) continue;
    const s = p.map((c, a) => c + dir[a] * (tEnter + 1e-6));
    const c = s.map(v => Math.max(0, Math.min(N - 1, Math.floor(v))));
    const step = dir.map(Math.sign);
    const delta = dir.map(d => Math.abs(1 / d));
    const next = s.map((v, a) => ((step[a] > 0 ? c[a] + 1 - v : v - c[a]) * delta[a]));
    let face = 1;
    while (c.every(v => v >= 0 && v < N)) {
      const kind = grid[(c[2] * N + c[1]) * N + c[0]];
      if (kind) {
        pixels[py * size + px] = tone[kind][face];
        ids[py * size + px] = ((c[2] * N + c[1]) * N + c[0]) * 3 + face;
        break;
      }
      face = next[0] < next[1] ? (next[0] < next[2] ? 0 : 2) : (next[1] < next[2] ? 1 : 2);
      c[face] += step[face];
      next[face] += delta[face];
    }
  }
  const lined = pixels.slice();
  for (let y = 0; y < size - 1; y++) for (let xx = 0; xx < size - 1; xx++) {
    const n = y * size + xx;
    if (ids[n] !== ids[n + 1] || ids[n] !== ids[n + size]) lined[n] = 0;
  }
  return lined;
}

function png(gray, width, height) {
  const crcTable = Array.from({ length: 256 }, (_, n) => {
    for (let k = 0; k < 8; k++) n = n & 1 ? 0xedb88320 ^ (n >>> 1) : n >>> 1;
    return n >>> 0;
  });
  const chunk = (type, data) => {
    const body = Buffer.concat([Buffer.from(type), data]);
    let c = 0xffffffff;
    for (const b of body) c = crcTable[(c ^ b) & 255] ^ (c >>> 8);
    const len = Buffer.alloc(4), crc = Buffer.alloc(4);
    len.writeUInt32BE(data.length);
    crc.writeUInt32BE((c ^ 0xffffffff) >>> 0);
    return Buffer.concat([len, body, crc]);
  };
  const header = Buffer.alloc(13);
  header.writeUInt32BE(width, 0);
  header.writeUInt32BE(height, 4);
  header.set([8, 0, 0, 0, 0], 8);
  const rows = Buffer.alloc((width + 1) * height);
  for (let y = 0; y < height; y++) rows.set(gray.subarray(y * width, (y + 1) * width), y * (width + 1) + 1);
  return Buffer.concat([
    Buffer.from([137, 80, 78, 71, 13, 10, 26, 10]),
    chunk('IHDR', header), chunk('IDAT', deflateSync(rows)), chunk('IEND', Buffer.alloc(0)),
  ]);
}

const SIZE = 520;
const width = SIZE * SPECIES.length;
const sheet = new Uint8Array(width * SIZE * KEEPS.length).fill(255);
for (const [column, id] of SPECIES.entries()) {
  let t = performance.now();
  const tree = await field(id);
  const growMs = performance.now() - t;
  console.log(`${id}: grown and queried over ${N}^3 cells in ${growMs.toFixed(0)} ms`);
  KEEPS.forEach((keep, row) => {
    t = performance.now();
    const v = cubes(tree, keep);
    const voxelMs = performance.now() - t;
    const image = render(v.grid, SIZE);
    for (let y = 0; y < SIZE; y++) sheet.set(image.subarray(y * SIZE, (y + 1) * SIZE), (row * SIZE + y) * width + column * SIZE);
    console.log(`${id} keep ${keep}: voxelize ${voxelMs.toFixed(0)} ms, wood ${v.wood}, foliage ${v.leaf}, ${v.clumps} clumps, ${deflateSync(v.grid).length} B deflated`);
  });
}
mkdirSync(out, { recursive: true });
const suffix = `-field${LIMB_ORDER === undefined ? '' : `-order${LIMB_ORDER}`}${THIN === 'coin' ? '' : `-${THIN}`}${WOOD_ONLY ? '-woodonly' : ''}${CUT_METRES ? '-metres' : ''}`;
const file = new URL(`sheet-${N}-keep${suffix}.png`, out);
writeFileSync(file, png(sheet, width, SIZE * KEEPS.length));
console.log(file.pathname);
