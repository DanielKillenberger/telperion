// fn-291 R1 probe: every shipped preset at its preset age through the built
// dist/ CPU entry in Node, one child process per case so Wasm memory, which
// only grows, is that case's high-water mark.
//   node probe-dist.mjs <dist-dir> <out.jsonl> [seeds=1,7,42] [ids=all]
//   node probe-dist.mjs --case <dist-dir> <id> <seed> <outputs>
// outputs: "both" | "surface" | "foliage". Each child runs under
// telperion-heavy so a case over its cap is killed, never swapping the desk.
import { execFileSync } from 'node:child_process';
import { appendFileSync } from 'node:fs';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { resolve } from 'node:path';

const OUTPUTS = { both: { surface: true, foliage: true }, surface: { surface: true }, foliage: { foliage: true } };

async function one(dist, id, seed, which) {
  const { TreeEngine, PRESETS } = await import(pathToFileURL(resolve(dist, 'telperion.js')).href);
  const preset = PRESETS.find(p => p.id === id);
  const family = structuredClone(preset);
  family.skeleton.seed = seed;
  const engine = await TreeEngine.create();
  const row = { id, seed, outputs: which, age: preset.age };
  const started = performance.now();
  try {
    const tree = engine.build(family, OUTPUTS[which]);
    const d = tree.diagnostics;
    Object.assign(row, {
      outcome: 'built', nodes: d.nodes,
      surfaceFloats: tree.surface?.positions.length ?? 0,
      surfaceIndices: tree.surface?.indices.length ?? 0,
      leaves: tree.foliage?.leaves.length ?? 0,
      timings: d.timings,
    });
  } catch (e) {
    const named = e instanceof Error && e.message.startsWith('Tree core:');
    row.outcome = named ? 'refused' : `trap:${e?.constructor?.name}`;
    row.error = String(e?.message ?? e);
  }
  row.ms = Math.round(performance.now() - started);
  row.memoryBytes = engine.memoryBytes;
  console.log(JSON.stringify(row));
}

async function sweep(dist, out, seeds, ids) {
  const { PRESETS } = await import(pathToFileURL(resolve(dist, 'telperion.js')).href);
  const self = fileURLToPath(import.meta.url);
  for (const { id } of PRESETS.filter(p => !ids || ids.includes(p.id))) {
    for (const seed of seeds) {
      const run = which => {
        let line;
        try {
          line = execFileSync('telperion-heavy', ['6', '--threads', '2', '--', 'node', self, '--case', dist, id, String(seed), which],
            { encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'], maxBuffer: 1 << 20 }).trim().split('\n').pop();
        } catch (e) {
          line = JSON.stringify({ id, seed, outputs: which, outcome: `process:${e.status ?? e.signal}`, error: String(e.stdout ?? '').slice(-400) });
        }
        appendFileSync(out, line + '\n');
        console.log(line);
        return JSON.parse(line);
      };
      const row = run('both');
      if (row.outcome !== 'built') { run('surface'); run('foliage'); }
    }
  }
}

const args = process.argv.slice(2);
if (args[0] === '--case') await one(args[1], args[2], Number(args[3]), args[4]);
else await sweep(resolve(args[0]), resolve(args[1]),
  (args[2] ?? '1,7,42').split(',').map(Number), args[3]?.split(','));
