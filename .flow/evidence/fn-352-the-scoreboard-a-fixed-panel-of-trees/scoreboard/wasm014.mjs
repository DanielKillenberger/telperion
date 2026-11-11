// fn-352 R4: the published telperion@0.1.4's Wasm times, by fn-329's protocol
// (wasm.mjs beside this file) as far as 0.1.4's API allows: each tree one cold
// build, then five warm, the median wall clock around `build`, Node, one
// thread pinned. Differences, forced by 0.1.4's API: presets come from
// `presetById` in telperion.js (0.1.4 ships no presets.js), there is no
// fidelity, and the field takes no grid size, so a build is
// `build(family, { field: true })` at 0.1.4's own field.
// Usage: node wasm014.mjs <0.1.4 package dist dir>; one JSON line per tree.
import { pathToFileURL } from "node:url";
import { readFile } from "node:fs/promises";
import { join, resolve } from "node:path";

const dist = resolve(process.argv[2]);
const { TreeEngine, presetById } = await import(pathToFileURL(join(dist, "telperion.js")).href);
const IDS = ["ordinary", "oregon-white-oak", "norway-spruce", "silver-birch", "date-palm", "telperion", "laurelin"];
const engine = await TreeEngine.create(await readFile(join(dist, "telperion.wasm")));
const WARM = 5;
const median = xs => [...xs].sort((a, b) => a - b)[xs.length >> 1];

for (const id of IDS) {
  for (const seed of [1, 1234, 5678]) {
    const family = structuredClone(presetById(id));
    family.skeleton.seed = seed;
    const samples = [];
    try {
      for (let k = 0; k <= WARM; k++) {
        const start = performance.now();
        engine.build(family, { field: true });
        const ms = performance.now() - start;
        if (k > 0) samples.push(ms);
        engine.release();
      }
    } catch (e) {
      console.log(JSON.stringify({ id, seed, fidelity: "0.1.4", refused: String(e) }));
      continue;
    }
    console.log(JSON.stringify({ id, seed, fidelity: "0.1.4", age: family.age, ms: +median(samples).toFixed(2), samples: samples.map(x => +x.toFixed(2)) }));
  }
}
engine.dispose();
