import { readFile, writeFile, mkdir, access } from 'node:fs/promises';
import { createHash, randomInt } from 'node:crypto';
import { spawn } from 'node:child_process';
import { tmpdir } from 'node:os';
import { resolve, join } from 'node:path';
import { fileURLToPath } from 'node:url';

/* ------------------------------------------------------------------ *
 * SPECIES QA, ON THE NATIVE RENDERER
 *
 * Two halves that answer different questions and never stand in for
 * each other. The measurement half runs the core's own example and
 * gates the numbers against the frozen profiles. The capture half asks
 * the headless renderer for stills at the hero pose - one process per
 * still, a real GPU, no browser and no page anywhere in the path.
 *
 * Neither half awards a visual verdict. The runner exits 1 while the
 * inspection is unassessed, even when every PNG is on disk.
 * ------------------------------------------------------------------ */

const args = process.argv.slice(2);
const option = name => { const i = args.indexOf(name); return i < 0 ? undefined : args[i + 1]; };
if (args.includes('--help')) {
  console.log(`Species QA (run from repository root; mature presets, no generation caps).
  --draw-seeds                Record fresh seeds once, before generation/tuning
  --seeds FILE                Default .flow/evidence/fn9/seeds.json
  --output DIR                Default new directory under OS temp
  --measure-only              Native 24-seed measurements per species
  --capture-only              Reuse measurements in --output; do not regenerate
  --case ID                   Capture only this case (partial, never protocol pass)
  --timeout-ms N              Per native/capture process limit (default 300000)
Build both examples first: npm run species:qa does it. Stills come from
target/release/examples/headless, which needs a GPU that is not a software
fallback; it names the condition on stderr and exits non-zero otherwise.
Every required specimen is captured whole, bare and as a single leaf, at the
renderer's own hero pose. JSON records the command, the adapter line it printed,
parameters and hashes, with numeric/visual/owner fields kept separate.
Exit 1 for failed/missing required evidence or unassessed visual results.
Human inspection goes in REPORT.md; this runner never awards visual approval.`);
  process.exit(0);
}
const known = new Set(['--draw-seeds', '--seeds', '--output', '--measure-only', '--capture-only', '--case', '--timeout-ms']);
for (let i = 0; i < args.length; i++) {
  if (!known.has(args[i])) throw Error(`Unknown option ${args[i]}`);
  if (['--seeds', '--output', '--case', '--timeout-ms'].includes(args[i])) {
    if (!args[++i] || args[i].startsWith('--')) throw Error('Missing option value');
  }
}
const timeout = Number(option('--timeout-ms') ?? 300000);
if (!Number.isSafeInteger(timeout) || timeout < 1000) throw Error('Invalid timeout');

/** What the headless still is rendered at, and how it is read back. The
 *  size is the old capture rig's, so the stills stay comparable with the
 *  images already in the fn9 record. */
const SIZE = '960x720';
const MEASURE = 'target/release/examples/species_measure';
const HEADLESS = 'target/release/examples/headless';
/** Whole tree, wood alone, and one placed element at generated scale -
 *  the three the renderer draws. */
const VIEWS = ['whole', 'bare', 'leaf'];

const sha = bytes => createHash('sha256').update(bytes).digest('hex');
const json = async path => JSON.parse(await readFile(path, 'utf8'));
const save = (path, value) => writeFile(path, JSON.stringify(value, null, 2) + '\n');
async function command(program, argv, limit = timeout) {
  return new Promise(resolveResult => {
    const child = spawn(program, argv, { stdio: ['ignore', 'pipe', 'pipe'], detached: process.platform !== 'win32' });
    let stdout = '', stderr = '', expired = false;
    child.stdout.on('data', x => { stdout += x; }); child.stderr.on('data', x => { stderr += x; });
    const timer = setTimeout(() => {
      expired = true;
      try { if (process.platform === 'win32') child.kill('SIGKILL'); else process.kill(-child.pid, 'SIGKILL'); } catch { /* already exited */ }
    }, limit);
    child.on('error', error => { clearTimeout(timer); resolveResult({ code: -1, error: String(error), stdout, stderr }); });
    child.on('close', code => { clearTimeout(timer); resolveResult({ code, expired, stdout, stderr }); });
  });
}

/** One still. The renderer reports what it drew on its own last line, and
 *  a still with no triangles in it is a failure however cleanly the
 *  process exited. */
async function capture(job) {
  const argv = ['--preset', job.preset, '--seed', String(job.seed), '--view', job.view, '--size', SIZE, '--out', job.png];
  const run = await command(HEADLESS, argv);
  const report = run.stdout.trim().split('\n').at(-1) ?? '';
  const drawn = /(\d+) triangles and (\d+) instances drawn in (\d+) calls/.exec(report);
  if (run.code !== 0 || drawn === null || Number(drawn[1]) === 0) {
    return { ...job, capture_status: 'fail', command: [HEADLESS, ...argv], report, error: run };
  }
  return { ...job, capture_status: 'pass', command: [HEADLESS, ...argv], report,
    drawn: { triangles: Number(drawn[1]), instances: Number(drawn[2]), calls: Number(drawn[3]) },
    pngSha256: sha(await readFile(job.png)), visual_status: 'unassessed', owner_feedback: null };
}

const seedPath = resolve(option('--seeds') ?? '.flow/evidence/fn9/seeds.json');
const profiles = await json('.flow/evidence/fn9/profiles.json');
if (args.includes('--draw-seeds')) {
  const manifest = { drawn_at: new Date().toISOString(), calibration_commit: (await command('git', ['rev-parse', 'HEAD'])).stdout.trim(), method: 'OS cryptographic random u32; reject only fixed/duplicate seeds', fixed: profiles.protocol.fixed_seeds, fresh: {} };
  for (const profile of profiles.profiles) {
    const used = new Set(manifest.fixed), fresh = [];
    while (fresh.length < 12) { const seed = randomInt(0, 2 ** 32); if (!used.has(seed)) { used.add(seed); fresh.push(seed); } }
    manifest.fresh[profile.id] = fresh;
  }
  await writeFile(seedPath, JSON.stringify(manifest, null, 2) + '\n', { flag: 'wx' });
  console.log(seedPath); process.exit(0);
}
if (args.includes('--measure-only') && args.includes('--capture-only')) throw Error('Conflicting modes');
const seeds = await json(seedPath);
if (JSON.stringify(seeds.fixed) !== JSON.stringify(profiles.protocol.fixed_seeds)) throw Error('Fixed seed protocol mismatch');
for (const p of profiles.profiles) {
  const fresh = seeds.fresh[p.id];
  if (!Array.isArray(fresh) || fresh.length !== 12 || new Set([...seeds.fixed, ...fresh]).size !== 24 || fresh.some(s => !Number.isInteger(s) || s < 0 || s >= 2 ** 32)) throw Error(`Invalid fresh seeds for ${p.id}`);
}
const out = resolve(option('--output') ?? join(tmpdir(), `telperion-species-${Date.now()}`));
await mkdir(out, { recursive: true }); console.log(out);
if (!args.includes('--capture-only')) {
  for (const p of profiles.profiles) for (const seed of [...seeds.fixed, ...seeds.fresh[p.id]]) {
    let exists = false;
    try { await access(join(out, `${p.id}-${seed}.jsonl`)); exists = true; } catch { /* new output */ }
    if (exists) throw Error('Existing numeric evidence: use a new output directory or --capture-only');
  }
  await writeFile(join(out, 'measurement-lock.json'), JSON.stringify({ started: new Date().toISOString() }), { flag: 'wx' });
}
await save(join(out, 'seeds.json'), seeds);
const cases = profiles.profiles.flatMap(p => ['fixed', 'fresh'].flatMap(group => (group === 'fixed' ? seeds.fixed : seeds.fresh[p.id]).map((seed, index) => ({ id: `${p.id}-${seed}`, preset: p.id, seed, group, required: index < 3 }))));
for (const c of cases) {
  const path = join(out, `${c.id}.jsonl`);
  if (!args.includes('--capture-only')) {
    // Native output refuses overwrite; explicit replay uses a new output dir.
    const run = await command(MEASURE, ['--case', `${c.id}:${c.preset}:${c.preset}:${c.seed}`, '--output', path]);
    await save(join(out, `${c.id}-process.json`), run);
  }
  try {
    const events = (await readFile(path, 'utf8')).split('\n').slice(0, -1).map(JSON.parse);
    c.numeric = events.findLast(e => (e.event === 'completed' || e.event === 'failed') && e.case === `${c.id}:${c.preset}:${c.preset}:${c.seed}`) ?? { numeric_status: 'unassessed', reason: 'No completed case' };
  } catch (error) { c.numeric = { numeric_status: 'unassessed', reason: String(error) }; }
  // Retain the original width counterexample even after its numeric repair.
  c.required ||= c.id === 'norway-spruce-4250668600' || c.numeric.numeric_status !== 'pass';
  c.visual_status = 'unassessed'; c.owner_feedback = null;
  console.log(c.id, c.numeric.numeric_status ?? 'failed');
}
await save(join(out, 'numeric.json'), cases);
if (args.includes('--measure-only')) { process.exit(cases.every(c => c.numeric.numeric_status === 'pass') ? 0 : 1); }
/* The supplementary presets have no drawn seed of their own, and the
   headless target names its seed rather than inheriting one, so they are
   captured at the protocol's first fixed seed. */
const subjects = [...cases.filter(c => c.required), ...['ordinary', 'telperion', 'laurelin'].map(preset => ({ id: preset, preset, seed: seeds.fixed[0] }))];
if (option('--case') && !subjects.some(c => c.id === option('--case'))) throw Error('Unknown or non-required capture case');
/* What rendered these stills: the renderer's sources, the binary built
   from them, and the profiles the cases came out of. */
const sourceFiles = (await command('git', ['ls-files', 'crates/telperion-render', 'crates/telperion-core/src'])).stdout.trim().split('\n');
const sourceHashes = Object.fromEntries(await Promise.all(sourceFiles.map(async path => [path, sha(await readFile(path))])));
const provenance = { sourceHashes, sourceSha256: sha(JSON.stringify(sourceHashes)), commit: (await command('git', ['rev-parse', 'HEAD'])).stdout.trim(), binarySha256: sha(await readFile(HEADLESS)), profilesSha256: sha(await readFile('.flow/evidence/fn9/profiles.json')), runnerSha256: sha(await readFile(fileURLToPath(import.meta.url))), size: SIZE };
await save(join(out, 'provenance.json'), provenance);
const jobs = subjects.flatMap(c => VIEWS.map(view => ({ id: c.id, preset: c.preset, seed: c.seed, view, provenance, png: join(out, `${c.id}-${view}.png`), result: join(out, `${c.id}-${view}.json`), capture_status: 'pending', visual_status: 'unassessed', owner_feedback: null })));
const suffix = option('--case') ? `-${option('--case')}` : '';
const capturesPath = join(out, `captures${suffix}.json`);
await save(join(out, `capture-plan${suffix}.json`), jobs);
for (const [index, job] of jobs.entries()) {
  if (option('--case') && job.id !== option('--case')) continue;
  try {
    // Reuse only a matching successful receipt with its PNG still beside it.
    const old = await json(job.result);
    if (old.id === job.id && old.view === job.view && old.seed === job.seed && old.capture_status === 'pass'
      && old.provenance?.sourceSha256 === provenance.sourceSha256 && old.provenance?.binarySha256 === provenance.binarySha256
      && old.provenance?.runnerSha256 === provenance.runnerSha256 && old.pngSha256 === sha(await readFile(job.png))) {
      jobs[index] = old; await save(capturesPath, jobs); continue;
    }
    await save(job.result + `.previous-${Date.now()}`, old);
  } catch { /* capture not available */ }
  const done = await capture(job);
  jobs[index] = done;
  await save(job.result, done);
  await save(capturesPath, jobs);
  console.log(done.id, done.view, done.capture_status, done.report);
}
await save(join(out, `summary${suffix}.json`), { protocol_status: 'unassessed', reason: 'Human trait inspection required; see REPORT.md. Missing/failed images never pass.', partial: !!option('--case'), numeric: cases.map(c => ({ id: c.id, status: c.numeric.numeric_status ?? 'unassessed' })), captures: jobs.map(j => ({ id: j.id, view: j.view, status: j.capture_status, path: j.png })), owner_feedback: null });
process.exitCode = 1;
