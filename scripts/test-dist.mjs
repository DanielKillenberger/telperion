import { execFileSync } from 'node:child_process';
import { readdir } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';

/* The package as Node loads it: dist/telperion.js and dist/field.js import
 * with no Wasm source given, so each entry reads its module from beside
 * itself through a `file:` URL, which is the README's first example. Runs on
 * a built dist and builds nothing itself.
 *
 * Every shipped preset at seeds 1, 7 and 42 builds through telperion.js
 * with surface and foliage, each case in its own process, because Wasm
 * memory only grows: a case's `memoryBytes` is its own high-water mark, held
 * at or under 3.5 GiB of the 32-bit binding's 4 GiB (fn-291). The 24 cases
 * take about four minutes on the owner's desk (fn-291 PROBE.md). One more
 * case must be refused by name before its wood is allocated. */
const dist = new URL('../dist/', import.meta.url);
const LIMIT = 3.5 * 2 ** 30;
const SEEDS = [1, 7, 42];

/** One case: build it, check its wood chunks and its memory, report a row. */
async function one(id, seed) {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const family = structuredClone(PRESETS.find(p => p.id === id));
  family.skeleton.seed = seed;
  const engine = await TreeEngine.create();
  const started = performance.now();
  const tree = engine.build(family, { surface: true, foliage: true });
  const seconds = (performance.now() - started) / 1000;
  const nodes = tree.diagnostics.nodes;
  const { chunks } = tree.surface;
  if (!(nodes > 0) || chunks.length === 0 || chunks.length !== tree.diagnostics.surfaceChunks) throw Error('built no tree');
  // The wood in chunks: none past 65,536 vertices, two oct components a
  // vertex, every 16-bit index inside its own chunk.
  for (const [k, c] of chunks.entries()) {
    const vertices = c.positions.length / 3;
    const whole = vertices > 0 && vertices <= 65536 && c.normals.length === vertices * 2 && c.indices.length > 0;
    if (!whole || !c.indices.every(i => i < vertices)) throw Error(`wood chunk ${k} is malformed`);
  }
  // The flowers beside the leaves (fn-294): present exactly where the core
  // names their reference, three words a floret and a whole flower mesh.
  const organ = tree.organ, reference = tree.diagnostics.organReference;
  if (Boolean(organ) !== (reference !== null)) throw Error('flowers and their reference disagree');
  if (organ && (organ.leaves.length === 0 || organ.leaves.length % 3 || !organ.indices.every(i => i < organ.positions.length / 3))) throw Error('flowers are malformed');
  const memory = engine.memoryBytes;
  if (memory > LIMIT) throw Error(`Wasm memory ${mib(memory)} MiB is over the ${mib(LIMIT)} MiB bar`);
  return { nodes, chunks: chunks.length, memory, seconds };
}

/** A wood 32-bit Wasm cannot hold (fn-291 R2f): telperion at seed 7 with
 * 64 sides a ring must be refused by name, before its wood is allocated,
 * so the memory stays far below the 4 GiB it would otherwise reach. */
async function refused() {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const family = structuredClone(PRESETS.find(p => p.id === 'telperion'));
  family.skeleton.seed = 7;
  family.surface.radialSegments = 64;
  const engine = await TreeEngine.create();
  let said;
  try {
    engine.build(family, { surface: true, foliage: true });
  } catch (e) {
    said = e instanceof Error ? e.message : String(e);
  }
  if (said === undefined) throw Error('built a wood it should have refused');
  const named = /^Tree core: the wood needs \d+ bytes and \d+ are left in 32-bit Wasm memory; .*device entry.*radialSegments/;
  if (!named.test(said)) throw Error(`not refused by name: ${said}`);
  const memory = engine.memoryBytes;
  if (memory > LIMIT) throw Error(`refused only after Wasm memory reached ${mib(memory)} MiB`);
  return { said, memory };
}

const mib = bytes => Math.round(bytes / 2 ** 20).toLocaleString('en');
const args = process.argv.slice(2);
if (args[0] === '--refused') {
  try {
    console.log(JSON.stringify(await refused()));
  } catch (e) {
    console.error(`telperion.js: telperion seed 7 at 64 sides: ${e instanceof Error ? e.message : e}`);
    process.exitCode = 1;
  }
} else if (args[0] === '--case') {
  const [id, seed] = [args[1], Number(args[2])];
  try {
    console.log(JSON.stringify(await one(id, seed)));
  } catch (e) {
    console.error(`telperion.js: ${id} seed ${seed}: ${e instanceof Error ? e.message : e}`);
    process.exitCode = 1;
  }
} else {
  const present = new Set(await readdir(dist));
  for (const file of ['telperion.js', 'telperion.wasm', 'telperion-render.wasm', 'field.js']) {
    if (!present.has(file)) throw Error(`dist/${file} is missing; run npm run build first`);
  }
  const { PRESETS, decodeNormals } = await import(new URL('telperion.js', dist));
  if (typeof decodeNormals !== 'function') throw Error('telperion.js does not export decodeNormals');
  const failures = [];
  for (const { id } of PRESETS) {
    for (const seed of SEEDS) {
      try {
        const out = execFileSync(process.execPath, [fileURLToPath(import.meta.url), '--case', id, String(seed)], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
        const row = JSON.parse(out.trim().split('\n').pop());
        console.log(`telperion.js: ${id} seed ${seed}: ${row.nodes} nodes, ${row.chunks} wood chunks, ${mib(row.memory)} MiB, ${row.seconds.toFixed(1)} s`);
      } catch (e) {
        const said = String(e.stderr ?? '').trim().split('\n').findLast(l => l.startsWith('telperion.js:'));
        const failure = said ?? `telperion.js: ${id} seed ${seed}: the case process ${e.signal ? `was killed by ${e.signal}` : `exited ${e.status}`}`;
        console.error(failure);
        failures.push(failure);
      }
    }
  }
  try {
    const out = execFileSync(process.execPath, [fileURLToPath(import.meta.url), '--refused'], { encoding: 'utf8', stdio: ['ignore', 'pipe', 'pipe'] });
    const row = JSON.parse(out.trim().split('\n').pop());
    console.log(`telperion.js: telperion seed 7 at 64 sides: refused at ${mib(row.memory)} MiB: ${row.said}`);
  } catch (e) {
    const said = String(e.stderr ?? '').trim().split('\n').findLast(l => l.startsWith('telperion.js:'));
    const failure = said ?? `telperion.js: telperion seed 7 at 64 sides: the case process ${e.signal ? `was killed by ${e.signal}` : `exited ${e.status}`}`;
    console.error(failure);
    failures.push(failure);
  }
  if (failures.length) throw Error(`${failures.length} of ${PRESETS.length * SEEDS.length + 1} cases failed:\n${failures.join('\n')}`);
  // The field grows on the device alone (fn-257): Node has no WebGPU, so it
  // rejects by name, never growing on the CPU.
  const { growField } = await import(new URL('field.js', dist));
  const refused = await growField('silver-birch', 7, 36).then(() => 'grown', e => String(e.message));
  if (!refused.includes('WebGPU is not available')) throw Error(`field.js in Node: ${refused}`);
  const side2 = await growField('silver-birch', 7, 2).then(() => 'grown', e => String(e.message));
  if (!side2.includes('field grid side')) throw Error(`field.js in Node, a side of two: ${side2}`);
  console.log(`dist in Node ${process.version}: telperion.js built all ${PRESETS.length * SEEDS.length} cases from the Wasm beside it, each at or under ${mib(LIMIT)} MiB; field.js refused without WebGPU and a side of two by name.`);
}
