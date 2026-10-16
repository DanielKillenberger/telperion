// fn-291: a hash of every output buffer a dist entry hands JS, per preset
// and seed, so two builds of dist/ compare byte for byte.
//   node hash-dist.mjs <dist-dir> <seed> [ids=all]
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';
import { resolve } from 'node:path';

const [dist, seed, ids] = process.argv.slice(2);
const { TreeEngine, PRESETS } = await import(pathToFileURL(resolve(dist, 'telperion.js')).href);
for (const preset of PRESETS.filter(p => !ids || ids.split(',').includes(p.id))) {
  const family = structuredClone(preset);
  family.skeleton.seed = Number(seed);
  const engine = await TreeEngine.create();
  const tree = engine.build(family, { surface: true, foliage: true });
  const hash = createHash('sha256');
  for (const a of [tree.surface.positions, tree.surface.normals, tree.surface.indices,
    tree.foliage.positions, tree.foliage.indices, tree.foliage.leaves]) {
    hash.update(new Uint8Array(a.buffer, a.byteOffset, a.byteLength));
  }
  console.log(`${preset.id} ${seed} ${hash.digest('hex').slice(0, 16)}`);
  engine.dispose();
}
