import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { expect, test } from 'vitest';

type Core = WebAssembly.Exports & {
  memory: WebAssembly.Memory;
  request_alloc: (length: number) => number;
  request_ptr: () => number;
  build: () => number;
  buffer_ptr: (slot: number) => number;
  buffer_len: (slot: number) => number;
};

// Every preset grows through the engine (fn-198, fn-213, fn-218, fn-219).
const ENGINE = [
  'european-beech',
  'oregon-white-oak',
  'norway-spruce',
  'date-palm',
  'silver-birch',
  'ordinary',
  'laurelin',
  'telperion',
];

async function wasmBuild(id: string): Promise<[number, Buffer]> {
  const { instance } = await WebAssembly.instantiate(
    readFileSync('src/browser/telperion.wasm'), { env: { now: () => 0 } });
  const core = instance.exports as Core;
  const request = new TextEncoder().encode(JSON.stringify({ family: id, outputs: { structure: true } }));
  expect(core.request_alloc(request.length)).toBe(0);
  new Uint8Array(core.memory.buffer, core.request_ptr() >>> 0, request.length).set(request);
  const status = core.build();
  if (status !== 0) return [status, Buffer.alloc(0)];
  return [0, Buffer.concat([6, 7].map(slot => Buffer.from(new Uint8Array(
    core.memory.buffer, core.buffer_ptr(slot) >>> 0, core.buffer_len(slot) * (slot === 6 ? 8 : 4)))))];
}

// The engine's transcendentals are the pinned libm the core reads (fn-11;
// fn-198 host decision 10), so native and Wasm grow every engine tree to
// the bit: positions, radii and topology alike.
for (const id of ENGINE) {
  test(`native/wasm node buffers byte-identical: ${id}`, async () => {
    const [status, wasm] = await wasmBuild(id);
    expect(status).toBe(0);
    const native = execFileSync('target/release/examples/node_buffer', [id], { maxBuffer: 512 * 1024 * 1024 });
    const digest = (b: Buffer) => createHash('sha256').update(b).digest('hex').slice(0, 16);
    console.info(`${id}: ${wasm.length} bytes, wasm ${digest(wasm)}, native ${digest(native)}`);
    expect(native.length).toBe(wasm.length);
    expect(wasm.equals(native)).toBe(true);
  }, 300_000);
}
