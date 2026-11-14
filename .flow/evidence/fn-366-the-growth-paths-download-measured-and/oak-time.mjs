import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync } from 'node:fs';
import { resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

/* The oak at 40 in Wasm under Node, through the package's own entry
 * (dist/telperion.js), on a module given by path (fn-366, LEVERS.md).
 *
 *   node oak-time.mjs <dist dir> <module.wasm> [runs = 3]
 *
 * Each run is its own process. A run times one field request (n = 36, the
 * growth path's request) and then digests the tree a second request builds
 * (its structure, topology and field grid), so a lever that moves the tree
 * shows as a moved digest. Prints one JSON line a run, then the median. */
const [, self, distArg, wasmArg, runsArg] = process.argv;
if (distArg === '--run') {
  const [dist, wasm] = [process.argv[3], process.argv[4]];
  const { TreeEngine, presetById } = await import(pathToFileURL(resolve(dist, 'telperion.js')).href);
  const oak = structuredClone(presetById('oregon-white-oak'));
  if (oak.age !== 40) throw Error(`the oak preset is ${oak.age}, not 40`);
  const engine = await TreeEngine.create(readFileSync(wasm));
  const started = performance.now();
  engine.build(oak, { field: { n: 36 } });
  const ms = performance.now() - started;
  const tree = engine.build(oak, { structure: true, field: { n: 36 } });
  const hash = createHash('sha256');
  hash.update(new Uint8Array(tree.structure.values.buffer));
  hash.update(new Uint8Array(tree.structure.topology.buffer));
  hash.update(JSON.stringify(tree.field));
  console.log(JSON.stringify({ ms: Math.round(ms), nodes: tree.diagnostics.nodes, digest: hash.digest('hex').slice(0, 16) }));
} else {
  const runs = Number(runsArg ?? 3);
  const rows = [];
  for (let i = 0; i < runs; i++) {
    const out = execFileSync(process.execPath, [fileURLToPath(pathToFileURL(self)), '--run', distArg, wasmArg], { encoding: 'utf8' });
    const row = JSON.parse(out.trim().split('\n').pop());
    rows.push(row);
    console.log(JSON.stringify({ at: new Date().toISOString(), ...row }));
  }
  const ms = rows.map(r => r.ms).sort((a, b) => a - b);
  const digests = new Set(rows.map(r => r.digest));
  console.log(`median ${ms[Math.floor(ms.length / 2)]} ms of [${ms.join(', ')}]; digest ${[...digests].join(' / ')}`);
}
