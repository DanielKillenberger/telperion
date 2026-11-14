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
 * nothing itself; takes up to 4 GB a case and minutes a species.
 *
 * `node scripts/wasm-ceiling.mjs <module> --allocations` prints, a JSON line
 * a case, the measurement allocator's records for the trees each side of a
 * trap and the Two Trees at seed 42: the failed request, every block of
 * 16 MiB or more with its stage, and the large blocks live at the high-water
 * mark. `--reuse` builds the oak at 40 then Telperion, and Telperion
 * twice, each pair on one engine, the memory read after each build. */
const dist = new URL('../dist/', import.meta.url);
const SPECIES = ['norway-spruce', 'european-beech', 'oregon-white-oak'];
const TWO_TREES = ['telperion', 'laurelin'];
const SEEDS = [42, 1, 7];
const STEP = 10;
const OLDEST = 400;
/** `--allocations`: the trees each side of a trap, and the Two Trees. */
const TRACED = [['norway-spruce', 88], ['norway-spruce', 89], ['european-beech', 121], ['european-beech', 122], ['telperion'], ['laurelin']];
/** `--reuse`: trees built one after another in one instance. */
const REUSED = [[['oregon-white-oak', 40], ['telperion']], [['telperion'], ['telperion']]];

/** Each tree of `sequence` built in turn on one engine, in a process of its
 * own: its outcome and the memory after it. */
function reused(module, sequence) {
  const args = [fileURLToPath(import.meta.url), '--sequence', module, JSON.stringify(sequence)];
  const out = execFileSync(process.execPath, args, { encoding: 'utf8', maxBuffer: 1 << 20 });
  return JSON.parse(out.trim().split('\n').at(-1));
}

/** The stage tags of `telperion_space::measure`; growth tags its year. */
const STAGES = { 0: 'before', 1001: 'settle', 1002: 'order', 1003: 'establish', 1004: 'convert', 1005: 'clothe', 1006: 'rings', 1007: 'faces', 1008: 'prepare', 1009: 'foliage', 1010: 'organ', 1011: 'structure', 1012: 'curve', 1013: 'handout' };
const stage = tag => STAGES[tag] ?? `growth ${tag}`;

/** The measurement allocator's records (crates/telperion-wasm/src/measured.rs). */
function allocations(e) {
  const words = kind => Array.from(new Uint32Array(e.memory.buffer, e.measured_words(kind) >>> 0, e.measured_words_len(kind)));
  const [high, highStage, highPages, live, pages, seen, , ...f] = words(0);
  const groups = (w, n) => Array.from({ length: w.length / n }, (_, k) => w.slice(k * n, k * n + n));
  return {
    high, highStage: stage(highStage), memoryAtHigh: highPages * 65536, liveAfter: live, memoryAfter: pages * 65536, largeEvents: seen,
    failed: f[5] ? { size: f[0], old: f[1], realloc: f[1] > 0, stage: stage(f[2]), live: f[3], memory: f[4] * 65536 } : null,
    events: groups(words(1), 4).map(([size, old, tag, at]) => ({ size, old, stage: stage(tag), at })),
    atHigh: groups(words(2), 3).map(([at, size, tag]) => ({ at, size, stage: stage(tag) })).sort((a, b) => b.size - a.size),
  };
}

/** One case in this process: the tree's outcome, kept count and memory. */
async function one(module, id, age, seed) {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const family = structuredClone(PRESETS.find(p => p.id === id));
  if (age !== undefined) family.age = age;
  family.skeleton.seed = seed;
  const engine = await TreeEngine.create(new WebAssembly.Module(readFileSync(module)));
  // The loader replaces a trapped instance (fn-369 step 2); the trapped
  // one's records are read first, through exports that touch no engine
  // state, by wrapping its private replacement for this measurement alone.
  const records = e => ({ kept: e.measured_kept() >>> 0, bytes: e.memory.buffer.byteLength, ...(e.measured_words ? { allocations: allocations(e) } : {}) });
  let trapped;
  const replace = Object.getPrototypeOf(engine).trapped;
  engine.trapped = function (error) {
    if (error instanceof WebAssembly.RuntimeError) trapped = records(this.exports);
    return replace.call(this, error);
  };
  const started = performance.now();
  let outcome = 'built';
  try {
    engine.build(family, { surface: true, foliage: true });
  } catch (e) {
    const said = e instanceof Error ? e.message : String(e);
    outcome = trapped ? `trapped: ${said}` : `refused: ${said}`;
  }
  const seconds = (performance.now() - started) / 1000;
  const { kept, ...rest } = trapped ?? records(engine['exports']);
  return { id, age: family.age, seed, kept: kept || null, outcome, seconds, ...rest };
}

/** One case in a child process of its own. */
function child(module, id, age, seed) {
  const args = [fileURLToPath(import.meta.url), '--case', module, id, String(age ?? ''), String(seed)];
  try {
    const out = execFileSync(process.execPath, args, { encoding: 'utf8', maxBuffer: 1 << 26 });
    return JSON.parse(out.trim().split('\n').at(-1));
  } catch (e) {
    return { id, age, seed, kept: null, bytes: null, outcome: `died: ${String(e.stderr ?? e).trim().split('\n').at(-1)}`, seconds: null };
  }
}

function row(r) {
  delete r.allocations;
  const per = r.kept && r.outcome === 'built' ? Math.round(r.bytes / r.kept) : '';
  const n = v => v === null || v === undefined ? 'unknown' : v.toLocaleString('en');
  console.log(`| ${r.id} | ${r.age} | ${r.seed} | ${n(r.kept)} | ${n(r.bytes)} | ${r.outcome} | ${per} |`);
  console.error(JSON.stringify(r));
  return r;
}

/** The largest age that builds and the first that does not, at seed 42. */
function edge(module, id, from) {
  const built = age => row(child(module, id, age, 42)).outcome === 'built';
  let low = row(child(module, id, from, 42));
  if (low.outcome !== 'built') return console.log(`| ${id} | fails at ${low.age}, where its walk starts; start it younger |||||`);
  let lowAge = low.age, high, kept = low.kept, falls = 0;
  for (let age = lowAge + STEP; age <= OLDEST; age += STEP) {
    const r = row(child(module, id, age, 42));
    if (r.outcome !== 'built') { high = age; break; }
    lowAge = age;
    // Past its peak a tree keeps fewer each step: two falls in a row end
    // the walk, the light thinning it rather than the memory stopping it.
    falls = r.kept < kept ? falls + 1 : 0;
    kept = r.kept;
    if (falls === 2) return console.log(`| ${id} | builds to ${age}, its kept count falling |||||`);
  }
  if (high === undefined) return console.log(`| ${id} | builds to ${OLDEST} |||||`);
  while (high - lowAge > 1) {
    const mid = Math.floor((lowAge + high) / 2);
    if (built(mid)) lowAge = mid; else high = mid;
  }
}

const args = process.argv.slice(2);
if (args[0] === '--sequence') {
  const { TreeEngine, PRESETS } = await import(new URL('telperion.js', dist));
  const engine = await TreeEngine.create(new WebAssembly.Module(readFileSync(args[1])));
  const rows = [];
  for (const [id, age] of JSON.parse(args[2])) {
    const family = structuredClone(PRESETS.find(p => p.id === id));
    if (age !== undefined) family.age = age;
    let outcome = 'built';
    try {
      engine.build(family, { surface: true, foliage: true });
    } catch (e) {
      outcome = `failed: ${e instanceof Error ? e.message : e}`;
    }
    rows.push({ id, age: family.age, outcome, memory: engine.memoryBytes });
  }
  console.log(JSON.stringify(rows));
} else if (args[0] === '--case') {
  const [, module, id, age, seed] = args;
  console.log(JSON.stringify(await one(module, id, age === '' ? undefined : Number(age), Number(seed))));
} else {
  const module = args[0];
  if (!module) throw Error('usage: node scripts/wasm-ceiling.mjs <measurement telperion_wasm.wasm> [species...]');
  if (args[1] === '--reuse') {
    for (const sequence of REUSED) console.log(JSON.stringify({ sequence: reused(module, sequence) }));
    process.exit();
  }
  if (args[1] === '--allocations') {
    for (const [id, age] of TRACED) console.log(JSON.stringify(child(module, id, age, 42)));
    process.exit();
  }
  const only = args.slice(1);
  console.log('| Tree | Age | Seed | Kept phytomers | Wasm bytes | Outcome | Bytes per kept |');
  console.log('|---|--:|--:|--:|--:|---|--:|');
  // A species may be named with the age its walk starts at: `norway-spruce@80`.
  const walks = only.length ? only.filter(s => s !== 'two-trees').map(s => s.split('@')) : SPECIES.map(s => [s]);
  for (const [id, from] of walks) edge(module, id, from === undefined ? undefined : Number(from));
  if (!only.length || only.includes('two-trees')) {
    for (const id of TWO_TREES) for (const seed of SEEDS) row(child(module, id, undefined, seed));
  }
}
