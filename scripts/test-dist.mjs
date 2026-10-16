import { readdir } from 'node:fs/promises';

/* The package as Node loads it: dist/telperion.js and dist/field.js import
 * with no Wasm source given, so each entry reads its module from beside
 * itself through a `file:` URL, which is the README's first example. Runs on
 * a built dist and builds nothing itself. */
const dist = new URL('../dist/', import.meta.url);
const present = new Set(await readdir(dist));
for (const file of ['telperion.js', 'telperion.wasm', 'telperion-render.wasm', 'field.js']) {
  if (!present.has(file)) throw Error(`dist/${file} is missing; run npm run build first`);
}
const { TreeEngine, TELPERION } = await import(new URL('telperion.js', dist));
const { growField } = await import(new URL('field.js', dist));
const engine = await TreeEngine.create();
const family = structuredClone(TELPERION);
family.skeleton.seed = 7;
const tree = engine.build(family, { surface: true, foliage: true });
const nodes = tree.diagnostics.nodes;
const { chunks } = tree.surface;
if (!(nodes > 0) || chunks.length !== tree.diagnostics.surfaceChunks) throw Error('the main entry built no tree');
// The wood in chunks: none past 65,536 vertices, two oct components a
// vertex, every 16-bit index inside its own chunk.
for (const [k, c] of chunks.entries()) {
  const vertices = c.positions.length / 3;
  const whole = vertices > 0 && vertices <= 65536 && c.normals.length === vertices * 2;
  if (!whole || !c.indices.every(i => i < vertices)) throw Error(`telperion.js: Telperion seed 7, wood chunk ${k} is malformed`);
}
engine.dispose();
// The field grows on the device alone (fn-257): Node has no WebGPU, so it
// rejects by name, never growing on the CPU.
const refused = await growField('silver-birch', 7).then(() => 'grown', e => String(e.message));
if (!refused.includes('WebGPU is not available')) throw Error(`field.js in Node: ${refused}`);
console.log(`dist in Node ${process.version}: telperion.js built Telperion to ${nodes} nodes in ${chunks.length} wood chunks from the Wasm beside it; field.js refused without WebGPU.`);
