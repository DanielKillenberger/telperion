import { readFile } from 'node:fs/promises';
const [dir, which, age, runs] = process.argv.slice(2);
const bytes = await readFile(`${dir}/target/wasm32-unknown-unknown/release/wprobe.wasm`);
const { instance } = await WebAssembly.instantiate(bytes, { env: { now: () => performance.now() } });
const t = [];
for (let i = 0; i < Number(runs ?? 3); i++) t.push(instance.exports.run(Number(which), Number(age)));
t.sort((a, b) => a - b);
console.log(`${dir} ${['palm','beech','oak','spruce'][which]} ${age} best ${t[0].toFixed(0)} median ${t[t.length >> 1].toFixed(0)} ms`);
