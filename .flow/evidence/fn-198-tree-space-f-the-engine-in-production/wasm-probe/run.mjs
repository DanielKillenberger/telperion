import { readFile } from 'node:fs/promises';
const bytes = await readFile(new URL('target/wasm32-unknown-unknown/release/wasm_probe.wasm', import.meta.url));
const mod = await WebAssembly.compile(bytes);
const names = ['palm', 'beech', 'oak', 'spruce'];
const [which, age, prep] = process.argv.slice(2).map(Number);
const inst = await WebAssembly.instantiate(mod, { env: { now: () => performance.now() } });
const e = inst.exports;
const t = performance.now();
let code;
try { code = which === 9 ? e.fine() : e.run(which, age, prep); } catch (err) { code = `trap: ${err.message}`; }
const out = new Float64Array(e.memory.buffer, e.out_ptr(), 16);
const mb = v => (v / 1e6).toFixed(0);
console.log(JSON.stringify({ species: names[which] ?? 'fine', age, code, wall_ms: Math.round(performance.now() - t),
  grow_ms: Math.round(out[1]), phytomers: out[3], nodes: out[4],
  mem_start_MB: mb(out[0]), mem_after_grow_MB: mb(out[2]), mem_after_convert_MB: mb(out[5]), mem_after_expand_MB: mb(out[6]),
  mem_after_rings_MB: mb(out[7]), mem_after_stations_MB: mb(out[8]), mem_after_curve_or_mesh_MB: mb(out[9]), gpu_branch: out[10], prep_ms: Math.round(out[11]),
  final_memory_MB: mb(e.memory.buffer.byteLength) }));
