import { readFile } from 'node:fs/promises';
const [dir, which, age, runs] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(await readFile(`${dir}/target/wasm32-unknown-unknown/release/wprobe.wasm`), { env: { now: () => performance.now() } });
const e = instance.exports;
let best = null;
for (let r = 0; r < Number(runs ?? 2); r++) {
  const len = e.staged(Number(which), Number(age));
  const names = new TextDecoder().decode(new Uint8Array(e.memory.buffer, e.names_ptr(), len)).split('\n');
  const sums = new Float64Array(e.memory.buffer, e.sums_ptr(), names.length);
  const total = sums.reduce((a, b) => a + b, 0);
  if (!best || total < best.total) best = { total, rows: names.map((n, i) => [n, sums[i]]) };
}
console.log(`total ${best.total.toFixed(1)}`);
for (const [n, v] of best.rows) if (v >= 0.05) console.log(`  ${n} ${v.toFixed(1)}`);
