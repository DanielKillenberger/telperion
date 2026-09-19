import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { beforeAll, expect, test } from 'vitest';

type Core = WebAssembly.Exports & {
  memory: WebAssembly.Memory;
  request_alloc: (length: number) => number;
  request_ptr: () => number;
  build: () => number;
  buffer_ptr: (slot: number) => number;
  buffer_len: (slot: number) => number;
};

beforeAll(() => {
  execFileSync('cargo', ['build', '--release', '-p', 'telperion-core', '--example', 'node_buffer'],
    { timeout: 120_000 });
}, 120_000);

for (const id of ['ordinary', 'oregon-white-oak', 'norway-spruce', 'telperion', 'laurelin']) {
  test(`native/wasm node bytes: ${id}`, async () => {
    const { instance } = await WebAssembly.instantiate(
      readFileSync('src/browser/telperion.wasm'), { env: { now: () => 0 } });
    const core = instance.exports as Core;
    const request = new TextEncoder().encode(JSON.stringify({ family: id, outputs: { structure: true } }));
    expect(core.request_alloc(request.length)).toBe(0);
    new Uint8Array(core.memory.buffer, core.request_ptr(), request.length).set(request);
    expect(core.build()).toBe(0);
    const wasm = Buffer.concat([6, 7].map(slot => Buffer.from(new Uint8Array(
      core.memory.buffer, core.buffer_ptr(slot), core.buffer_len(slot) * (slot === 6 ? 8 : 4)))));
    const native = execFileSync('target/release/examples/node_buffer', [id], { maxBuffer: 32 * 1024 * 1024 });
    const hash = (bytes: Uint8Array) => createHash('sha256').update(bytes).digest('hex');
    console.info(`${id} native=${hash(native)} wasm=${hash(wasm)} bytes=${native.length}/${wasm.length}`);
    expect(hash(wasm)).toBe(hash(native));
  }, 120_000);
}

for (const id of ['ordinary', 'oregon-white-oak', 'norway-spruce', 'telperion', 'laurelin']) {
  test(`native/wasm retained specimen bytes: ${id}`, async () => {
    const { TreeEngine, presetById } = await import('../src/browser/core');
    const engine = await TreeEngine.create(readFileSync('src/browser/telperion.wasm'));
    const family = { ...presetById(id), age: 20.25 };
    const specimen = engine.buildSpecimen(family);
    specimen.advance(5.5);
    const snapshot = specimen.snapshot();
    const nativeSnapshot = execFileSync('target/release/examples/node_buffer', [id, '25.75', '--export'], { maxBuffer: 512 * 1024 * 1024 });
    const imported = engine.importSpecimen({ schema: 2, data: nativeSnapshot });
    imported.advance(0.25);
    for (const age of [10.5, 20.25, 26]) {
      const read = imported.read(age);
      const wasm = Buffer.concat([read.structure.values, read.structure.topology, read.leaves]
        .map(a => Buffer.from(a.buffer, a.byteOffset, a.byteLength)));
      const native = execFileSync('target/release/examples/node_buffer', [id, String(age), '--import'], { input: snapshot.data, maxBuffer: 512 * 1024 * 1024 });
      expect(wasm.equals(native), `${id} at ${age}: ${wasm.length}/${native.length} bytes`).toBe(true);
    }
    engine.dispose();
  }, 120_000);
}
