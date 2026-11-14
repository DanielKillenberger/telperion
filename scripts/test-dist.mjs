import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
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
 * at or under 3.5 GiB of the 32-bit binding's 4 GiB (fn-291), and printed:
 * the check that every shipped preset at its own seed (42) builds in Wasm,
 * in place of the kept-phytomer ceiling (fn-369). The 24 cases take about
 * four minutes on the owner's desk (fn-291 PROBE.md). One more case must be
 * refused by name: a tree the machine cannot hold. */
const dist = new URL('../dist/', import.meta.url);
const LIMIT = 3.5 * 2 ** 30;
const REFUSED_AGE = 126;
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

/** A tree 32-bit Wasm cannot hold (fn-369 step 2): the spruce at seed 42
 * at `REFUSED_AGE`, two years past the youngest the machine refused, is
 * refused by name with the bytes it asked for and the stage, and the next
 * request on the same engine builds on a fresh instance. */
async function refused() {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const spruce = PRESETS.find(p => p.id === 'norway-spruce');
  const family = structuredClone(spruce);
  family.age = REFUSED_AGE;
  const engine = await TreeEngine.create();
  let said;
  try {
    engine.build(family, { surface: true, foliage: true });
  } catch (e) {
    said = e instanceof Error ? e.message : String(e);
  }
  if (said === undefined) throw Error('built a tree it should have refused');
  const named = /^Tree core: the [\w' ]+ needs (\d+) bytes and the machine did not give them(?: \((\d+) bytes live\))?$/.exec(said);
  if (!named) throw Error(`not refused by name: ${said}`);
  const live = named[2] === undefined ? null : Number(named[2]);
  const next = engine.build(spruce, { surface: true, foliage: true });
  if (!(next.diagnostics.nodes > 0)) throw Error('the next request built no tree');
  return { said, live, next: next.diagnostics.nodes, memory: engine.memoryBytes };
}

const mib = bytes => Math.round(bytes / 2 ** 20).toLocaleString('en');
const args = process.argv.slice(2);
if (args[0] === '--refused') {
  try {
    console.log(JSON.stringify(await refused()));
  } catch (e) {
    console.error(`telperion.js: norway-spruce seed 42 at ${REFUSED_AGE}: ${e instanceof Error ? e.message : e}`);
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
    console.log(`telperion.js: norway-spruce seed 42 at ${REFUSED_AGE}: ${row.said}; the next request built ${row.next} nodes in ${mib(row.memory)} MiB`);
  } catch (e) {
    const said = String(e.stderr ?? '').trim().split('\n').findLast(l => l.startsWith('telperion.js:'));
    const failure = said ?? `telperion.js: norway-spruce seed 42 at ${REFUSED_AGE}: the case process ${e.signal ? `was killed by ${e.signal}` : `exited ${e.status}`}`;
    console.error(failure);
    failures.push(failure);
  }
  if (failures.length) throw Error(`${failures.length} of ${PRESETS.length * SEEDS.length + 1} cases failed:\n${failures.join('\n')}`);
  // The field grows on the CPU (fn-353): in Node, without WebGPU, it reads
  // out the grid the main entry's build reads out, from the Wasm beside it.
  const { growField } = await import(new URL('field.js', dist));
  const { presetById: preset } = await import(new URL('presets.js', dist));
  const { TreeEngine } = await import(new URL('telperion.js', dist));
  const birch = preset('silver-birch');
  const grid = await growField(birch, 36);
  const built = (await TreeEngine.create()).build(birch, { field: { n: 36 } }).field;
  if (grid.n !== 36 || !grid.answer.flags.some(f => f & 1) || JSON.stringify(grid) !== JSON.stringify(built)) throw Error('field.js in Node: the grid is not the main entry\'s');
  // The presets are plain data (fn-317 R3): their entry imports nothing,
  // so loading it fetches no Wasm.
  const presets = readFileSync(new URL('presets.js', dist), 'utf8');
  if (/\bimport\b|\.wasm/.test(presets)) throw Error('presets.js imports or names a module');
  const side2 = await growField(birch, 2).then(() => 'grown', e => String(e.message));
  if (!side2.includes('field grid side')) throw Error(`field.js in Node, a side of two: ${side2}`);
  console.log(`dist in Node ${process.version}: telperion.js built all ${PRESETS.length * SEEDS.length} cases from the Wasm beside it, each at or under ${mib(LIMIT)} MiB; field.js grew the birch's grid without WebGPU and refused a side of two by name.`);
}
