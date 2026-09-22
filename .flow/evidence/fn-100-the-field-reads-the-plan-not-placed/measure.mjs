// fn-100 measurements through the Wasm binding, no browser.
//
//   node measure.mjs time  <label>=<wasm> ...   R2: field-only builds with a
//       64^3 batch query, median of five, for oak, birch and spruce at seeds
//       1 and 7; peak linear memory and field bytes; and, on the last wasm
//       given, the surface-plus-foliage build for comparison.
//   node measure.mjs wood  <base wasm> <candidate wasm>   R3: the wood bit of
//       a 64^3 batch query on eight trees, byte for byte.
//
// Raw rows go to stdout as JSON; the caller keeps them under raw/.
import { readFileSync } from 'node:fs';
import { createHash } from 'node:crypto';

const [mode, ...rest] = process.argv.slice(2);
const SPECIES = ['oregon-white-oak', 'silver-birch', 'norway-spruce'];
const N = 64;

async function engine(path) {
  const { instance } = await WebAssembly.instantiate(readFileSync(path), { env: { now: () => performance.now() } });
  const x = instance.exports;
  const meta = () => JSON.parse(Buffer.from(x.memory.buffer, x.metadata_ptr(), x.metadata_len()).toString());
  const family = id => { if (x.catalogue()) throw Error('catalogue'); return meta().find(e => e.id === id).family; };
  const build = (fam, outputs) => {
    const request = Buffer.from(JSON.stringify({ family: fam, outputs }));
    if (x.request_alloc(request.length)) throw Error(JSON.stringify(meta()));
    new Uint8Array(x.memory.buffer, x.request_ptr(), request.length).set(request);
    const t = performance.now();
    if (x.build()) throw Error(JSON.stringify(meta()));
    return { ms: performance.now() - t, meta: meta() };
  };
  const query = (revision, cells) => {
    if (x.query_alloc(cells.length / 4)) throw Error(JSON.stringify(meta()));
    new Float64Array(x.memory.buffer, x.query_ptr(), cells.length).set(cells);
    const t = performance.now();
    if (x.query(revision)) throw Error(JSON.stringify(meta()));
    return { ms: performance.now() - t, flags: new Uint8Array(x.memory.buffer, x.buffer_ptr(8), x.buffer_len(8)).slice() };
  };
  return { x, meta, family, build, query, memory: () => x.memory.buffer.byteLength, release: () => x.release() };
}

function grid(bounds) {
  const step = Math.max(...bounds.max.map((v, i) => v - bounds.min[i])) / N;
  const cells = new Float64Array(N ** 3 * 4);
  let i = 0;
  for (let a = 0; a < N; a++) for (let b = 0; b < N; b++) for (let c = 0; c < N; c++)
    cells.set([bounds.min[0] + (a + .5) * step, bounds.min[1] + (b + .5) * step, bounds.min[2] + (c + .5) * step, step / 2], i++ * 4);
  return cells;
}
const median = v => { const s = [...v].sort((a, b) => a - b); return s[(s.length - 1) >> 1]; };
const subjects = () => SPECIES.flatMap(id => [1, 7].map(seed => ({ id, seed })));

if (mode === 'time') {
  const rows = [];
  for (const [k, arg] of rest.entries()) {
    const [label, path] = arg.split('=');
    for (const { id, seed } of subjects()) {
      for (const outputs of k === rest.length - 1 ? [{ field: true }, { surface: true, foliage: true }] : [{ field: true }]) {
        const e = await engine(path);
        const fam = e.family(id); fam.skeleton.seed = seed;
        const samples = [];
        let memory = 0, fieldBytes = 0, stages = null, timings = null;
        for (let s = 0; s < 5; s++) {
          const built = e.build(fam, outputs);
          let queryMs = 0;
          if (outputs.field) queryMs = e.query(built.meta.revision, grid(built.meta.fieldBounds)).ms;
          samples.push({ buildMs: built.ms, queryMs, totalMs: built.ms + queryMs });
          memory = Math.max(memory, e.memory());
          fieldBytes = built.meta.fieldBytes; stages = built.meta.stages; timings = built.meta.timings;
          e.release();
        }
        const row = { label, id, seed, outputs: Object.keys(outputs).join('+'), medianTotalMs: median(samples.map(s => s.totalMs)),
          medianBuildMs: median(samples.map(s => s.buildMs)), medianQueryMs: median(samples.map(s => s.queryMs)), peakMemoryBytes: memory, fieldBytes, stages, timings, samples };
        rows.push(row);
        console.error(`${label} ${id} seed ${seed} ${row.outputs}: median ${row.medianTotalMs.toFixed(0)} ms (build ${row.medianBuildMs.toFixed(0)} + query ${row.medianQueryMs.toFixed(0)}), peak ${(memory / 1048576).toFixed(0)} MB, field ${(fieldBytes / 1048576).toFixed(1)} MB`);
      }
    }
  }
  console.log(JSON.stringify(rows));
} else if (mode === 'wood') {
  const [basePath, candidatePath] = rest;
  const trees = [...subjects(), { id: 'ordinary', seed: null }, { id: 'telperion', seed: null }];
  const rows = [];
  for (const { id, seed } of trees) {
    const base = await engine(basePath), candidate = await engine(candidatePath);
    const fam = base.family(id); if (seed !== null) fam.skeleton.seed = seed;
    const b = base.build(fam, { field: true });
    const cells = grid(b.meta.fieldBounds);
    const c = candidate.build(fam, { field: true });
    const wood = flags => flags.map(f => f & 1);
    const hash = a => createHash('sha256').update(a).digest('hex').slice(0, 16);
    const bw = wood(base.query(b.meta.revision, cells).flags), cw = wood(candidate.query(c.meta.revision, cells).flags);
    const identical = bw.length === cw.length && bw.every((v, i) => v === cw[i]);
    const row = { id, seed, cells: bw.length, woodCells: bw.reduce((n, v) => n + v, 0), baseHash: hash(bw), candidateHash: hash(cw), identical,
      baseFieldBytes: b.meta.fieldBytes, candidateFieldBytes: c.meta.fieldBytes, candidateSource: c.meta.stages.fieldSource };
    rows.push(row);
    console.error(`${id} seed ${seed ?? 'preset'}: wood cells ${row.woodCells} of ${row.cells}, ${identical ? 'identical' : 'DIFFERENT'} (${row.baseHash})`);
  }
  console.log(JSON.stringify(rows));
} else {
  throw Error('mode: time | wood');
}
