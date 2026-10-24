// fn-198 task 3: the main module (telperion.wasm) building an engine preset
// at an age, in Node: status, message, time and memory, or the trap.
//   node main-module.mjs <preset-id> <age> [outputs-json]
import { readFileSync } from 'node:fs';

const [id, age, outputs] = process.argv.slice(2);
const { instance } = await WebAssembly.instantiate(
  readFileSync('src/browser/telperion.wasm'), { env: { now: () => performance.now() } });
const core = instance.exports;
const text = () => new TextDecoder().decode(new Uint8Array(core.memory.buffer, core.metadata_ptr(), core.metadata_len()));
core.catalogue();
const family = JSON.parse(text()).find((e) => e.id === id).family;
family.age = Number(age);
const request = { family, ...(outputs ? { outputs: JSON.parse(outputs) } : {}) };
const bytes = new TextEncoder().encode(JSON.stringify(request));
if (core.request_alloc(bytes.length) !== 0) throw Error(text());
new Uint8Array(core.memory.buffer, core.request_ptr(), bytes.length).set(bytes);
const started = performance.now();
let status;
try { status = core.build(); } catch (e) { status = `trap: ${e.message}`; }
const meta = typeof status === 'number' ? text().slice(0, 300) : '';
console.log(JSON.stringify({ id, age: Number(age), status, ms: Math.round(performance.now() - started),
  memoryMB: Math.round(core.memory.buffer.byteLength / 1e6), meta }));
