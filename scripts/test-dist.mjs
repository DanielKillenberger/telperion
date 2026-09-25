import { readdir } from 'node:fs/promises';

/* The package as Node loads it: dist/telperion.js and dist/field.js import
 * with no Wasm source given, so each entry reads its module from beside
 * itself through a `file:` URL, which is the README's first example. Runs on
 * a built dist and builds nothing itself. */
const dist = new URL('../dist/', import.meta.url);
const present = new Set(await readdir(dist));
for (const file of ['telperion.js', 'telperion.wasm', 'telperion-render.wasm', 'field.js', 'telperion-field.wasm']) {
  if (!present.has(file)) throw Error(`dist/${file} is missing; run npm run build first`);
}
const { TreeEngine, TELPERION, PRESETS } = await import(new URL('telperion.js', dist));
const { growField } = await import(new URL('field.js', dist));
const engine = await TreeEngine.create();
const family = structuredClone(TELPERION);
family.skeleton.seed = 7;
const tree = engine.build(family, { surface: true, foliage: true });
const nodes = tree.diagnostics.nodes;
if (!(nodes > 0) || !(tree.surface.positions.length > 0)) throw Error('the main entry built no tree');
engine.dispose();
const field = await growField('silver-birch', 7);
const { min, max } = field.bounds;
const answer = field.query(new Float64Array([(min[0] + max[0]) / 2, (min[1] + max[1]) / 2, (min[2] + max[2]) / 2, Math.max(...max.map((v, a) => v - min[a])) / 2]));
if (answer.flags[0] !== 3) throw Error(`the whole-tree cell holds wood and foliage, got ${answer.flags[0]}`);
field.release();
// Every shipped preset grows through the slim entry (fn-150): a preset the
// field package cannot serve does not ship.
for (const preset of PRESETS) {
  const tree = await growField(preset.id, 1);
  if (tree.bounds.max[1] <= tree.bounds.min[1]) throw Error(`field.js grew ${preset.id} empty`);
  tree.release();
}
console.log(`dist in Node ${process.version}: telperion.js built Telperion to ${nodes} nodes, field.js grew silver-birch (wood radius ${answer.woodRadius[0].toFixed(3)} m), both from the Wasm beside them.`);
