// Throwaway prototype: can the solved tree alone (the `structure` output, no
// surface, no placed leaves) drive a black-and-white voxel tree? Wood cells come
// from tapered segments, foliage cells from twig nodes within one leaf reach.
// Run: node experiments/voxel-field/voxels.mjs [resolution] [keep,keep,...]
// keep is the fraction of twig-bearing branches whose sprays become foliage:
// whole branches drop out, so the crown opens in clumps instead of thinning evenly.
// FIELD=1 reads the core's field (fn-100) instead of the structure export: one
// batch query over the grid, wood and foliage bits and the wood radius per
// cell, and the owning limb id as the clump coin; LIMB_ORDER=<n> asks the field
// for limbs at that lateral order (the family's clumpSystemOrder otherwise).
// Every run logs its clump count, the distinct coins the crown was dropped by.
// WASM=<path> reads another build of the binding.
import { readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { deflateSync } from 'node:zlib';

const root = new URL('../../', import.meta.url);
const out = new URL('out/', import.meta.url);
const N = Number(process.argv[2] ?? 64);
const CLUMP = Number(process.argv[4] ?? 0.1);
const KEEPS = (process.argv[3] ?? '0.5,0.3,0.15').split(',').map(Number);
const SPECIES = ['oregon-white-oak', 'silver-birch', 'norway-spruce'];
const WOOD = 1, LEAF = 2;
const FIELD = process.env.FIELD === '1';
const LIMB_ORDER = process.env.LIMB_ORDER === undefined ? undefined : Number(process.env.LIMB_ORDER);
// THIN=coin|density|mass: how the field's foliage cells are thinned to a keep
// fraction. coin drops whole limb systems; density keeps the twig-densest
// cells; mass keeps the cells nearest to drawn limb wood. Limbs always draw.
const THIN = process.env.THIN ?? 'coin';
// WOOD_CUT: wood draws where the cell's thickest wood is at least this many cells.
const WOOD_CUT = Number(process.env.WOOD_CUT ?? 0.2);

const { instance } = await WebAssembly.instantiate(
  readFileSync(process.env.WASM ? new URL(process.env.WASM, `file://${process.cwd()}/`) : new URL('src/browser/telperion.wasm', root)),
  { env: { now: () => performance.now() } },
);
const x = instance.exports;
const meta = () => JSON.parse(Buffer.from(x.memory.buffer, x.metadata_ptr(), x.metadata_len()).toString());

function family(id) {
  if (x.catalogue()) throw Error('catalogue');
  return meta().find(e => e.id === id).family;
}

function structure(id) {
  const request = Buffer.from(JSON.stringify({ family: id, outputs: { structure: true } }));
  if (x.request_alloc(request.length)) throw Error(JSON.stringify(meta()));
  new Uint8Array(x.memory.buffer, x.request_ptr(), request.length).set(request);
  if (x.build()) throw Error(JSON.stringify(meta()));
  return {
    values: new Float64Array(x.memory.buffer, x.buffer_ptr(6), x.buffer_len(6)).slice(),
    topology: new Uint32Array(x.memory.buffer, x.buffer_ptr(7), x.buffer_len(7)).slice(),
  };
}

/// The field over a grid of N cells along the tree's longest axis: one batch
/// query, then the bits, the wood radius and the limb id of every cell.
function field(id) {
  const field = LIMB_ORDER === undefined ? true : { limbOrder: LIMB_ORDER };
  const request = Buffer.from(JSON.stringify({ family: id, outputs: { field } }));
  if (x.request_alloc(request.length)) throw Error(JSON.stringify(meta()));
  new Uint8Array(x.memory.buffer, x.request_ptr(), request.length).set(request);
  if (x.build()) throw Error(JSON.stringify(meta()));
  const m = meta();
  const { min, max } = m.fieldBounds;
  const cell = Math.max(max[0] - min[0], max[1] - min[1], max[2] - min[2]) / (N - 2);
  const origin = min.map((v, a) => (v + max[a]) / 2 - (N * cell) / 2);
  origin[1] = min[1] - cell;
  const cells = new Float64Array(N * N * N * 4);
  for (let k = 0; k < N; k++) for (let j = 0; j < N; j++) for (let i = 0; i < N; i++)
    cells.set([origin[0] + (i + .5) * cell, origin[1] + (j + .5) * cell, origin[2] + (k + .5) * cell, cell / 2], ((k * N + j) * N + i) * 4);
  if (x.query_alloc(N * N * N)) throw Error(JSON.stringify(meta()));
  new Float64Array(x.memory.buffer, x.query_ptr(), cells.length).set(cells);
  if (x.query(m.revision)) throw Error(JSON.stringify(meta()));
  return {
    flags: new Uint8Array(x.memory.buffer, x.buffer_ptr(8), x.buffer_len(8)).slice(),
    limbs: new Uint32Array(x.memory.buffer, x.buffer_ptr(20), x.buffer_len(20)).slice(),
    leaves: new Float32Array(x.memory.buffer, x.buffer_ptr(19), x.buffer_len(19)).slice(),
    radius: new Float32Array(x.memory.buffer, x.buffer_ptr(25), x.buffer_len(25)).slice(),
    cell, stages: m.stages, planned: m.leavesPlanned, buildMs: m.timings.coreMs,
  };
}

/// Cells from the field, as the structure pass draws them: wood where the
/// thickest wood reaching the cell is at least a fifth of it, through any
/// foliage; foliage elsewhere where the field reports it and the owning limb
/// keeps its clump. Thinner wood is the twigs, and a dropped clump takes them.
function voxelizeField({ flags, limbs, leaves, radius, cell }, keep) {
  const grid = new Uint8Array(N * N * N);
  const coins = new Set();
  let wood = 0, leaf = 0;
  const limb = n => (flags[n] & 1) && radius[n] >= cell * WOOD_CUT;
  for (let n = 0; n < grid.length; n++) if (limb(n)) { grid[n] = WOOD; wood++; }
  if (THIN === 'coin') {
    for (let n = 0; n < grid.length; n++) if (!limb(n) && flags[n] & 2) {
      coins.add(limbs[n]);
      if (kept(limbs[n], keep)) { grid[n] = LEAF; leaf++; }
    }
    return { grid, cell, wood, leaf, clumps: coins.size };
  }
  // A score per foliage cell; the top `keep` fraction survives.
  const foliage = [];
  for (let n = 0; n < grid.length; n++) if (!limb(n) && flags[n] & 2) foliage.push(n);
  let score;
  if (THIN === 'density') score = n => leaves[n];
  else if (THIN === 'tuft') {
    // Each limb system keeps a rounded mass about its own centre: a cell
    // scores by how deep it sits in its system, so the fringe between
    // neighbouring systems goes first and the branch to each mass shows.
    const at = n => [n % N, Math.floor(n / N) % N, Math.floor(n / (N * N))];
    const sum = new Map();
    for (const n of foliage) {
      const [i, j, k] = at(n);
      const e = sum.get(limbs[n]) ?? { i: 0, j: 0, k: 0, c: 0 };
      e.i += i; e.j += j; e.k += k; e.c++; sum.set(limbs[n], e);
    }
    const spread = new Map();
    const d = n => { const [i, j, k] = at(n); const e = sum.get(limbs[n]); return Math.hypot(i - e.i / e.c, j - e.j / e.c, k - e.k / e.c); };
    for (const n of foliage) { const e = spread.get(limbs[n]) ?? { s: 0, c: 0 }; e.s += d(n); e.c++; spread.set(limbs[n], e); }
    score = n => { const e = spread.get(limbs[n]); return -d(n) / (e.s / e.c + 1e-9); };
  } else if (THIN === 'gap') {
    // Each limb system keeps its own shape and loses the cells nearest a
    // neighbouring system first: gaps open along the boundaries between
    // systems, the crown's outer surface stays, and a hanging curtain stays
    // a curtain. The score is the grid distance to the nearest cell of a
    // different system, by breadth-first search from the boundaries.
    const dist = new Int32Array(N * N * N).fill(-1);
    const isLeaf = new Uint8Array(N * N * N);
    for (const n of foliage) isLeaf[n] = 1;
    const at = n => [n % N, Math.floor(n / N) % N, Math.floor(n / (N * N))];
    const steps = [[1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]];
    const neighbours = n => { const [i, j, k] = at(n); const out = []; for (const [di, dj, dk] of steps) { const a = i + di, b = j + dj, c = k + dk; if (a >= 0 && b >= 0 && c >= 0 && a < N && b < N && c < N) out.push((c * N + b) * N + a); } return out; };
    let queue = [];
    for (const n of foliage) if (neighbours(n).some(m => isLeaf[m] && limbs[m] !== limbs[n])) { dist[n] = 0; queue.push(n); }
    while (queue.length) {
      const next = [];
      for (const n of queue) for (const m of neighbours(n)) if (isLeaf[m] && dist[m] === -1 && limbs[m] === limbs[n]) { dist[m] = dist[n] + 1; next.push(m); }
      queue = next;
    }
    // A system with no neighbour never erodes; it scores as its deepest cell would.
    score = n => (dist[n] === -1 ? N : dist[n]) + leaves[n] * 1e-6;
  } else {
    // Grid distance to the nearest drawn limb cell, by breadth-first search.
    const dist = new Int32Array(N * N * N).fill(-1);
    let queue = [];
    for (let n = 0; n < grid.length; n++) if (grid[n] === WOOD) { dist[n] = 0; queue.push(n); }
    while (queue.length) {
      const next = [];
      for (const n of queue) {
        const i = n % N, j = Math.floor(n / N) % N, k = Math.floor(n / (N * N));
        for (const [di, dj, dk] of [[1,0,0],[-1,0,0],[0,1,0],[0,-1,0],[0,0,1],[0,0,-1]]) {
          const a = i + di, b = j + dj, c = k + dk;
          if (a < 0 || b < 0 || c < 0 || a >= N || b >= N || c >= N) continue;
          const m = (c * N + b) * N + a;
          if (dist[m] === -1) { dist[m] = dist[n] + 1; next.push(m); }
        }
      }
      queue = next;
    }
    // Nearer limbs score higher; ties broken by density so masses stay full.
    score = n => -dist[n] + leaves[n] * 1e-6;
  }
  foliage.sort((a, b) => score(b) - score(a));
  const survivors = Math.round(foliage.length * keep);
  for (let s = 0; s < survivors; s++) { grid[foliage[s]] = LEAF; leaf++; }
  for (const n of foliage) coins.add(limbs[n]);
  return { grid, cell, wood, leaf, clumps: coins.size };
}

/// Stable per-branch coin: the same branch run always answers the same.
function kept(branch, keep) {
  let h = Math.imul(branch ^ 0x9e3779b9, 0x85ebca6b);
  h = Math.imul(h ^ (h >>> 13), 0xc2b2ae35);
  return ((h ^ (h >>> 16)) >>> 0) / 2 ** 32 < keep;
}

/// Cubic grid of N cells along the tree's longest axis, centred on its bounds.
function voxelize({ values, topology }, keep) {
  const count = topology.length / 3;
  const min = [Infinity, Infinity, Infinity], max = [-Infinity, -Infinity, -Infinity];
  for (let i = 0; i < count; i++) for (let a = 0; a < 3; a++) {
    min[a] = Math.min(min[a], values[i * 6 + a]);
    max[a] = Math.max(max[a], values[i * 6 + a]);
  }
  const cell = Math.max(max[0] - min[0], max[1] - min[1], max[2] - min[2]) / (N - 2);
  const origin = min.map((m, a) => (m + max[a]) / 2 - (N * cell) / 2);
  origin[1] = min[1] - cell;
  const grid = new Uint8Array(N * N * N);
  const twigs = new Uint16Array(N * N * N);
  const index = (i, j, k) => (k * N + j) * N + i;
  const clampCell = v => Math.max(0, Math.min(N - 1, v));
  const cellOf = (p, a) => Math.floor((p - origin[a]) / cell);

  // Marks every cell whose centre lies within `pad` of the tapered segment.
  const sweep = (a, b, ra, rb, pad, mark) => {
    const r = Math.max(ra, rb) + pad;
    const lo = [0, 1, 2].map(n => clampCell(cellOf(Math.min(a[n], b[n]) - r, n)));
    const hi = [0, 1, 2].map(n => clampCell(cellOf(Math.max(a[n], b[n]) + r, n)));
    const d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    const dd = d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
    for (let k = lo[2]; k <= hi[2]; k++) for (let j = lo[1]; j <= hi[1]; j++) for (let i = lo[0]; i <= hi[0]; i++) {
      const c = [origin[0] + (i + .5) * cell, origin[1] + (j + .5) * cell, origin[2] + (k + .5) * cell];
      const q = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
      const t = dd > 0 ? Math.max(0, Math.min(1, (q[0] * d[0] + q[1] * d[1] + q[2] * d[2]) / dd)) : 0;
      const e = [q[0] - d[0] * t, q[1] - d[1] * t, q[2] - d[2] * t];
      const limit = ra + (rb - ra) * t + pad;
      if (e[0] * e[0] + e[1] * e[1] + e[2] * e[2] <= limit * limit) mark(index(i, j, k));
    }
  };

  const at = i => [values[i * 6], values[i * 6 + 1], values[i * 6 + 2]];
  const coins = new Set();
  for (let i = 0; i < count; i++) {
    const parent = topology[i * 3];
    if (parent === 0xffffffff) continue;
    const a = at(parent), b = at(i), end = values[i * 6 + 3], start = values[i * 6 + 4];
    if (topology[i * 3 + 2] === 2) {
      let bearer = parent;
      // Climb to the limb thick enough to own a clump: CLUMP cells of radius.
      while (values[bearer * 6 + 3] < cell * CLUMP && topology[bearer * 3] !== 0xffffffff) bearer = topology[bearer * 3];
      coins.add(topology[bearer * 3 + 1]);
      if (!kept(topology[bearer * 3 + 1], keep)) continue;
      // A twig is thinner than any cell: it votes for the cells it passes
      // through and never pads outward, so gaps between sprays stay open.
      const steps = Math.max(1, Math.ceil(Math.hypot(b[0] - a[0], b[1] - a[1], b[2] - a[2]) / (cell * 0.5)));
      let last = -1;
      for (let s = 0; s <= steps; s++) {
        const p = [0, 1, 2].map(n => a[n] + (b[n] - a[n]) * s / steps);
        const n = index(...p.map((v, axis) => clampCell(cellOf(v, axis))));
        if (n !== last) twigs[last = n]++;
      }
    } else if (Math.max(start, end) >= cell * 0.2) {
      sweep(a, b, start, end, cell * 0.5, n => { grid[n] = WOOD; });
    }
  }
  let wood = 0, leaf = 0;
  for (let n = 0; n < grid.length; n++) {
    if (grid[n] === WOOD) wood++;
    else if (twigs[n] > 0) { grid[n] = LEAF; leaf++; }
  }
  return { grid, cell, wood, leaf, clumps: coins.size };
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
SPECIES.forEach((id, column) => {
  let t = performance.now();
  const tree = FIELD ? field(id) : structure(id);
  const growMs = performance.now() - t;
  if (FIELD) console.log(`${id}: field ${JSON.stringify(tree.stages)}, ${tree.planned} stations planned, core ${tree.buildMs.toFixed(0)} ms, grid query ${growMs.toFixed(0)} ms in all`);
  KEEPS.forEach((keep, row) => {
    t = performance.now();
    const v = FIELD ? voxelizeField(tree, keep) : voxelize(tree, keep);
    const voxelMs = performance.now() - t;
    const image = render(v.grid, SIZE);
    for (let y = 0; y < SIZE; y++) sheet.set(image.subarray(y * SIZE, (y + 1) * SIZE), (row * SIZE + y) * width + column * SIZE);
    console.log(`${id} keep ${keep}: grow ${growMs.toFixed(0)} ms, voxelize ${voxelMs.toFixed(0)} ms, wood ${v.wood}, foliage ${v.leaf}, ${v.clumps} clumps, ${deflateSync(v.grid).length} B deflated`);
  });
});
mkdirSync(out, { recursive: true });
const suffix = FIELD ? `-field${LIMB_ORDER === undefined ? '' : `-order${LIMB_ORDER}`}${THIN === 'coin' ? '' : `-${THIN}`}` : '';
const file = new URL(`sheet-${N}-keep${suffix}.png`, out);
writeFileSync(file, png(sheet, width, SIZE * KEEPS.length));
console.log(file.pathname);
