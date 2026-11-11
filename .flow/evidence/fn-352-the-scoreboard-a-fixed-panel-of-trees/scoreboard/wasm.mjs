// fn-329: every preset's tree time in the package's Wasm, on the CPU in Node.
// fn-352: copied from fn-329 probes/wasm.mjs, unchanged below.
// Usage: node wasm.mjs <dist dir> <fidelity,...>; prints one JSON line per tree.
// Each tree: one cold build, then five warm builds; a build is the package's
// `build(family, { field: { n: 36 } })`, timed by wall clock around the call.
import { pathToFileURL } from "node:url";
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const [dist, fidelities] = [resolve(process.argv[2]), process.argv[3].split(",").map(Number)];
const { TreeEngine } = await import(pathToFileURL(join(dist, "telperion.js")).href);
const { CATALOGUE_IDS, presetById } = await import(pathToFileURL(join(dist, "presets.js")).href).then(m => ({
  presetById: m.presetById,
  CATALOGUE_IDS: ["ordinary", "oregon-white-oak", "norway-spruce", "european-beech", "silver-birch", "date-palm", "telperion", "laurelin"],
}));
const engine = await TreeEngine.create(await readFile(join(dist, "telperion.wasm")));
const WARM = 5;
const median = xs => [...xs].sort((a, b) => a - b)[xs.length >> 1];

for (const fidelity of fidelities) {
  for (const id of CATALOGUE_IDS) {
    for (const seed of [1, 1234, 5678]) {
      const family = presetById(id);
      family.skeleton.seed = seed;
      family.fidelity = fidelity;
      const samples = [];
      let timings = [];
      try {
        for (let k = 0; k <= WARM; k++) {
          const start = performance.now();
          const tree = engine.build(family, { field: { n: 36 } });
          const ms = performance.now() - start;
          if (k > 0) { samples.push(ms); timings.push(tree.diagnostics.timings); }
          engine.release();
        }
      } catch (e) {
        console.log(JSON.stringify({ id, seed, fidelity, refused: String(e) }));
        continue;
      }
      const stage = key => median(timings.map(t => t[key]));
      const stages = Object.fromEntries(Object.keys(timings[0]).map(k => [k, +stage(k).toFixed(2)]));
      console.log(JSON.stringify({ id, seed, fidelity, age: family.age, ms: +median(samples).toFixed(2), samples: samples.map(x => +x.toFixed(2)), stages }));
    }
  }
}
engine.dispose();
