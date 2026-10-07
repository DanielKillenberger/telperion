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

// The engine presets grow through the engine (fn-198); the others are
// refused by name on both sides.
const NONE = ['ordinary', 'telperion', 'laurelin'];
const ENGINE = ['european-beech', 'oregon-white-oak', 'norway-spruce', 'date-palm', 'silver-birch'];

async function wasmBuild(id: string): Promise<[number, Buffer]> {
  const { instance } = await WebAssembly.instantiate(
    readFileSync('src/browser/telperion.wasm'), { env: { now: () => 0 } });
  const core = instance.exports as Core;
  const request = new TextEncoder().encode(JSON.stringify({ family: id, outputs: { structure: true } }));
  expect(core.request_alloc(request.length)).toBe(0);
  new Uint8Array(core.memory.buffer, core.request_ptr(), request.length).set(request);
  const status = core.build();
  if (status !== 0) return [status, Buffer.alloc(0)];
  return [0, Buffer.concat([6, 7].map(slot => Buffer.from(new Uint8Array(
    core.memory.buffer, core.buffer_ptr(slot), core.buffer_len(slot) * (slot === 6 ? 8 : 4)))))];
}

for (const id of NONE) {
  test(`native/wasm refuse a preset with no engine species: ${id}`, async () => {
    const [status] = await wasmBuild(id);
    expect(status).not.toBe(0);
    expect(() => execFileSync('target/release/examples/node_buffer', [id], { stdio: 'pipe' })).toThrow();
  }, 120_000);
}

// The engine's transcendentals are std's, so native and Wasm agree on the
// tree's topology to the bit and on its floats to rounding (2.5e-14 m at
// most, measured), not to the bit as the pinned core's do (fn-11). Bit
// parity waits on the host's decision to pin the engine's math (fn-198).
const ROUNDING = 1e-12;

for (const id of ENGINE) {
  test(`native/wasm topology bytes and positions to rounding: ${id}`, async () => {
    const [status, wasm] = await wasmBuild(id);
    expect(status).toBe(0);
    const native = execFileSync('target/release/examples/node_buffer', [id], { maxBuffer: 512 * 1024 * 1024 });
    expect(native.length).toBe(wasm.length);
    const floats = (b: Buffer, n: number) => new Float64Array(b.buffer.slice(b.byteOffset, b.byteOffset + n * 8));
    const nodes = wasm.length / (6 * 8 + 3 * 4);
    const [w, n] = [floats(wasm, nodes * 6), floats(native, nodes * 6)];
    let worst = 0;
    for (let i = 0; i < w.length; i++) worst = Math.max(worst, Math.abs(w[i] - n[i]));
    console.info(`${id}: ${nodes} nodes, widest gap ${worst.toExponential(2)} m`);
    expect(worst).toBeLessThan(ROUNDING);
    expect(wasm.subarray(nodes * 48).equals(native.subarray(nodes * 48))).toBe(true);
  }, 300_000);
}
