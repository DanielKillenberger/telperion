// fn-291 task 2: one case through a traced dist (trace-pages.py), printing
// the Wasm memory at the end of growth and at every wood reservation of 16
// MiB or more. A failed build keeps its trace in the module; a small build
// after it in the same engine returns it.
//   node trace-case.mjs <traced-dist-dir> <id> <seed> [both|surface|foliage]
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';

const [dist, id, seed, which = 'both'] = process.argv.slice(2);
const OUTPUTS = { both: { surface: true, foliage: true }, surface: { surface: true }, foliage: { foliage: true } };
const { TreeEngine, PRESETS } = await import(pathToFileURL(resolve(dist, 'telperion.js')).href);
const family = structuredClone(PRESETS.find(p => p.id === id));
family.skeleton.seed = Number(seed);
const engine = await TreeEngine.create();
let outcome = 'built', trace;
try {
  trace = engine.build(family, OUTPUTS[which]).diagnostics.trace;
} catch (e) {
  outcome = String(e.message);
  trace = engine.build(PRESETS.find(p => p.id === 'date-palm'), { foliage: true }).diagnostics.trace;
}
console.log(JSON.stringify({ id, seed: Number(seed), outputs: which, outcome, memoryMiB: engine.memoryBytes >> 20, trace }));
