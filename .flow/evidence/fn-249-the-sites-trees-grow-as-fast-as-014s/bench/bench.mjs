// fn-249 R5: the site's request in one Wasm instance (one session), grows in
// sequence so the first is cold and the rest warm. Per grow: grow ms, and per
// n the grid, query and voxelize ms, with a SHA-256 over the bounds and every
// field answer (flags, wood radii, leaves, limbs) for the byte-identity check.
// argv: wasm species seeds(comma) ns(comma) cut wood rule keep
import { readFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';
import { grid, voxelize } from './voxelize.js';
const [wasm, species, seeds, ns, cut, wood, rule, keep] = process.argv.slice(2);
const mod = await WebAssembly.compile(await readFile(wasm));
const inst = await WebAssembly.instantiate(mod, { prof: { now: () => performance.now() } });
const e = inst.exports;
const check = (s) => { if (s) throw Error(new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.error_ptr(), e.error_len()))); };
const id = new TextEncoder().encode(species);
const A = [1, 4, 4, 4];
let k = 0;
for (const seed of seeds.split(',').map(Number)) {
  const out = { wasm, species, seed, order: k++, cut: +cut };
  let t = performance.now();
  check(e.species_alloc(id.length));
  new Uint8Array(e.memory.buffer, e.species_ptr(), id.length).set(id);
  check(e.grow.length === 3 ? e.grow(seed, 0xffffffff, +cut) : e.grow(seed, 0xffffffff));
  out.grow_ms = performance.now() - t;
  const b = new Float64Array(e.memory.buffer, e.bounds_ptr(), e.bounds_len());
  const bounds = { min: [b[0], b[1], b[2]], max: [b[3], b[4], b[5]] };
  out.per_n = {};
  for (const n of ns.split(',').map(Number)) {
    const r = {};
    t = performance.now();
    const g = grid(bounds, n);
    r.grid_ms = performance.now() - t;
    t = performance.now();
    check(e.query_alloc(g.cells.length / 4));
    new Float64Array(e.memory.buffer, e.query_ptr(), g.cells.length).set(g.cells);
    check(e.query(e.revision()));
    const sl = (s) => e.memory.buffer.slice(e.answer_ptr(s), e.answer_ptr(s) + e.answer_len(s) * A[s]);
    const raw = [0, 1, 2, 3].map(sl);
    const answer = { flags: new Uint8Array(raw[0]), woodRadius: new Float32Array(raw[1]), leaves: new Float32Array(raw[2]), limbs: new Uint32Array(raw[3]) };
    r.query_ms = performance.now() - t;
    t = performance.now();
    const cubes = voxelize(g, answer, { woodCutoff: g.cell * +wood, thin: { rule, keep: +keep } });
    r.voxelize_ms = performance.now() - t;
    r.end_ms = out.grow_ms + r.grid_ms + r.query_ms + r.voxelize_ms;
    const h = createHash('sha256');
    h.update(new Uint8Array(new Float64Array(b).buffer));
    for (const x of raw) h.update(new Uint8Array(x));
    r.hash = h.digest('hex').slice(0, 16);
    r.cubes = cubes.wood.length + cubes.foliage.length;
    out.per_n[n] = r;
  }
  console.log(JSON.stringify(out));
}
