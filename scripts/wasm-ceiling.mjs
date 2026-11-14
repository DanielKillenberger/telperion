import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';

/* Where 32-bit Wasm memory runs out, measured as the browser grows a tree:
 * the package's own entry (dist/telperion.js, as scripts/test-dist.mjs
 * loads it) on a core built without the kept-phytomer refusal, which no
 * shipped module is:
 *
 *   CARGO_TARGET_DIR=target/ceiling cargo build --release \
 *     --target wasm32-unknown-unknown -p telperion-wasm --features measure-ceiling
 *   node scripts/wasm-ceiling.mjs target/ceiling/wasm32-unknown-unknown/release/telperion_wasm.wasm
 *
 * Each case is its own process, because Wasm memory only grows: its
 * `memory.buffer.byteLength` after the build is its high-water mark. A case
 * builds, traps (a RuntimeError: the memory ran out), or is refused by name.
 * Each species is stepped by ten years from its preset age to its first
 * failure, then bisected to the year (fn-198 BROWSER.md's walk); the Two
 * Trees are built at their preset ages on seeds 42, 1 and 7. Builds
 * nothing itself; takes up to 4 GB a case and minutes a species. */
const dist = new URL('../dist/', import.meta.url);
const SPECIES = ['norway-spruce', 'european-beech', 'oregon-white-oak'];
const TWO_TREES = ['telperion', 'laurelin'];
const SEEDS = [42, 1, 7];
const STEP = 10;
const OLDEST = 400;

/** One case in this process: the tree's outcome, kept count and memory. */
async function one(module, id, age, seed) {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const family = structuredClone(PRESETS.find(p => p.id === id));
  if (age !== undefined) family.age = age;
  family.skeleton.seed = seed;
  const engine = await TreeEngine.create(new WebAssembly.Module(readFileSync(module)));
  const started = performance.now();
  let outcome = 'built';
  try {
    engine.build(family, { surface: true, foliage: true });
  } catch (e) {
    if (e instanceof WebAssembly.RuntimeError) outcome = 'trapped';
    else outcome = `refused: ${e instanceof Error ? e.message : e}`;
  }
  // The export reads no engine state, so it answers after a trap; zero is
  // a tree that never finished growing.
  const kept = engine['exports'].measured_kept() >>> 0;
  const bytes = engine.memoryBytes;
  const seconds = (performance.now() - started) / 1000;
  return { id, age: family.age, seed, kept: kept || null, bytes, outcome, seconds };
}

/** One case in a child process of its own. */
function child(module, id, age, seed) {
  const args = [fileURLToPath(import.meta.url), '--case', module, id, String(age ?? ''), String(seed)];
  try {
    const out = execFileSync(process.execPath, args, { encoding: 'utf8', maxBuffer: 1 << 20 });
    return JSON.parse(out.trim().split('\n').at(-1));
  } catch (e) {
    return { id, age, seed, kept: null, bytes: null, outcome: `died: ${String(e.stderr ?? e).trim().split('\n').at(-1)}`, seconds: null };
  }
}

function row(r) {
  const per = r.kept && r.outcome === 'built' ? Math.round(r.bytes / r.kept) : '';
  const n = v => v === null || v === undefined ? 'unknown' : v.toLocaleString('en');
  console.log(`| ${r.id} | ${r.age} | ${r.seed} | ${n(r.kept)} | ${n(r.bytes)} | ${r.outcome} | ${per} |`);
  console.error(JSON.stringify(r));
  return r;
}

/** The largest age that builds and the first that does not, at seed 42. */
function edge(module, id) {
  const built = age => row(child(module, id, age, 42)).outcome === 'built';
  let low = row(child(module, id, undefined, 42));
  if (low.outcome !== 'built') throw Error(`${id} fails at its preset age ${low.age}`);
  let lowAge = low.age, high;
  for (let age = lowAge + STEP; age <= OLDEST; age += STEP) {
    if (!built(age)) { high = age; break; }
    lowAge = age;
  }
  if (high === undefined) return console.log(`| ${id} | builds to ${OLDEST} |||||`);
  while (high - lowAge > 1) {
    const mid = Math.floor((lowAge + high) / 2);
    if (built(mid)) lowAge = mid; else high = mid;
  }
}

const args = process.argv.slice(2);
if (args[0] === '--case') {
  const [, module, id, age, seed] = args;
  console.log(JSON.stringify(await one(module, id, age === '' ? undefined : Number(age), Number(seed))));
} else {
  const module = args[0];
  if (!module) throw Error('usage: node scripts/wasm-ceiling.mjs <measurement telperion_wasm.wasm> [species...]');
  const only = args.slice(1);
  console.log('| Tree | Age | Seed | Kept phytomers | Wasm bytes | Outcome | Bytes per kept |');
  console.log('|---|--:|--:|--:|--:|---|--:|');
  for (const id of SPECIES.filter(s => !only.length || only.includes(s))) edge(module, id);
  if (!only.length || only.includes('two-trees')) {
    for (const id of TWO_TREES) for (const seed of SEEDS) row(child(module, id, undefined, seed));
  }
}
