// fn-101 measurements, in Node, through the slim entry point.
//
//   node measure.mjs [wasm]   R3: from species id and seed to a fully queried
//       grid (64 cells a side, the voxelizer's grid over the field's bounds)
//       for oak, birch and spruce at seeds 1 and 7: the cold run first, module
//       compile and instantiation included, then the median of five warm runs.
//
// Raw rows go to stdout as JSON; the caller keeps them under raw/. The
// machine and the Node version are in the rows.
import { readFileSync } from 'node:fs';
import { cpus, totalmem } from 'node:os';

const root = new URL('../../../', import.meta.url);
const { growField, compileField } = await import(new URL('src/field/index.ts', root));
const { grid } = await import(new URL('src/field/voxelize.ts', root));
const path = process.argv[2] ?? new URL('src/field/telperion-field.wasm', root);
const bytes = readFileSync(path);
const N = 64;
const SPECIES = ['oregon-white-oak', 'silver-birch', 'norway-spruce'];
const median = v => { const s = [...v].sort((a, b) => a - b); return s[(s.length - 1) >> 1]; };

async function run(id, seed, source) {
  const t0 = performance.now();
  const tree = await growField(id, seed, { source });
  const t1 = performance.now();
  const answer = tree.query(grid(tree.bounds, N).cells);
  const t2 = performance.now();
  tree.release();
  return { growMs: t1 - t0, queryMs: t2 - t1, totalMs: t2 - t0, wood: answer.flags.filter(f => f & 1).length, foliage: answer.flags.filter(f => f & 2).length };
}
const rows = [];
for (const id of SPECIES) for (const seed of [1, 7]) {
  // Cold: the bytes compiled and instantiated inside the measured call.
  const t = performance.now();
  const module = await compileField(bytes);
  const compileMs = performance.now() - t;
  const cold = await run(id, seed, bytes);
  const warm = [];
  for (let s = 0; s < 5; s++) warm.push(await run(id, seed, module));
  const row = {
    id, seed, cells: N, compileMs, cold,
    warmMedianMs: median(warm.map(w => w.totalMs)), warmMedianGrowMs: median(warm.map(w => w.growMs)), warmMedianQueryMs: median(warm.map(w => w.queryMs)),
    warm, wood: cold.wood, foliage: cold.foliage,
  };
  rows.push(row);
  console.error(`${id} seed ${seed}: cold ${cold.totalMs.toFixed(0)} ms (compile ${compileMs.toFixed(0)} + grow ${cold.growMs.toFixed(0)} + query ${cold.queryMs.toFixed(0)}), warm median ${row.warmMedianMs.toFixed(0)} ms (grow ${row.warmMedianGrowMs.toFixed(0)} + query ${row.warmMedianQueryMs.toFixed(0)})`);
}
console.log(JSON.stringify({ node: process.version, cpu: cpus()[0].model, cores: cpus().length, memoryGB: Math.round(totalmem() / 2 ** 30), wasmBytes: bytes.length, rows }));
