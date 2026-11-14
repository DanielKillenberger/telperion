// fn-317 R4c: the voxelizer on both executors' named answers. Reads the
// grids `every_preset_s_named_fields_are_written_for_the_voxelizer` wrote
// (raw/fields, ignored) and prints, for every preset, side and thinning
// rule, the cubes drawn and how many differ between the CPU and the device.
// node .flow/evidence/fn-317-every-generator-entry-takes-a-trees/cubes.mjs
import { readdirSync, readFileSync } from 'node:fs';
import { voxelize } from '../../../src/field/voxelize.ts';

const dir = new URL('raw/fields/', import.meta.url);
const grid = g => ({ n: g.n, cell: g.cell, origin: g.origin, answer: {
  flags: Uint8Array.from(g.flags), woodRadius: Float32Array.from(g.woodRadius), leaves: Float32Array.from(g.leaves),
  limbs: Uint32Array.from(g.limbs), flowers: Float32Array.from(g.flowers) } });
const cubes = c => new Set([...[...c.wood].map(i => `w${i}`), ...[...c.foliage].map(i => `f${i}`), ...[...c.flowers].map(i => `b${i}`)]);
console.log('| Preset | Side | Rule | CPU cubes | moved | moved share |');
console.log('|---|--:|---|--:|--:|--:|');
for (const file of readdirSync(dir).sort()) {
  const { cpu, device } = JSON.parse(readFileSync(new URL(file, dir), 'utf8'));
  for (const rule of ['coin', 'density', 'mass', 'tuft', 'gap']) {
    const dials = { woodCutoff: 0.02, thin: { rule, keep: 0.7 } };
    const [a, b] = [cubes(voxelize(grid(cpu), dials)), cubes(voxelize(grid(device), dials))];
    const moved = [...a].filter(k => !b.has(k)).length + [...b].filter(k => !a.has(k)).length;
    console.log(`| ${file.replace(/-\d+\.json$/, '')} | ${cpu.n} | ${rule} | ${a.size} | ${moved} | ${(moved / a.size).toFixed(4)} |`);
  }
}
