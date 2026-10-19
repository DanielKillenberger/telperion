import { execFile, spawn } from 'node:child_process';
import { appendFile, readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

/* One scorecard row on a card no other session is using (fn-259 R4, host
 * rule 2026-10-19). Wraps any probe command, either version's:
 *
 *   node scripts/gpu-row.mjs <rows.jsonl> -- <probe command...>
 *
 * The row holds ${XDG_RUNTIME_DIR}/telperion-heavy.lock from before its
 * first sample to after its last, so no `telperion-heavy` job is admitted
 * meanwhile; it lets it go while it waits. It starts only when no other
 * `telperion-heavy-*` unit runs and no process whose command line names
 * telperion is on the card. While the
 * probe runs it samples `nvidia-smi --query-compute-apps` every 250 ms; a
 * process that appears on the card, was not there at the start and is not
 * in the probe's process tree is foreign, and makes the row `valid: false`,
 * as a probe that exits non-zero or prints no JSON does.
 * The processes on the card at the start (the desktop's clients) are
 * written into the row with their memory and do not invalidate it: a
 * compute job outside Telperion already running at the start is the
 * residual this rule leaves. Appends one JSON line: the probe's own JSON
 * under `probe` (null when it printed none), its exit code, the GPU state
 * at the start and the end, the foreign processes and `valid`.
 */
const run = promisify(execFile);
const SAMPLE_MS = 250;
const WAIT_MS = 5_000;

/** @typedef {{ pid: number, mib: number, command: string }} App */

/** Every process on the card with its memory and command line. @returns {Promise<App[]>} */
async function apps() {
  const { stdout } = await run('nvidia-smi', [
    '--query-compute-apps=pid,used_memory',
    '--format=csv,noheader,nounits',
  ]);
  const rows = stdout.trim().split('\n').filter(Boolean);
  return Promise.all(
    rows.map(async row => {
      const [pid, mib] = row.split(',').map(s => Number(s.trim()));
      return { pid, mib, command: await commandOf(pid) };
    }),
  );
}

async function commandOf(/** @type {number} */ pid) {
  const raw = await readFile(`/proc/${pid}/cmdline`, 'latin1').catch(() => '');
  return raw.replaceAll('\0', ' ').trim().slice(0, 160) || '(exited before read)';
}

/** The card's memory and use, and every process on it. */
async function gpuState() {
  const { stdout } = await run('nvidia-smi', [
    '--query-gpu=memory.free,memory.used,memory.total,utilization.gpu',
    '--format=csv,noheader,nounits',
  ]);
  const [free, used, total, util] = stdout.trim().split(',').map(s => Number(s.trim()));
  return { free_mib: free, used_mib: used, total_mib: total, utilization: util, apps: await apps() };
}

/** Whether `pid` descends from `root` (or is it). */
async function descends(/** @type {number} */ pid, /** @type {number} */ root) {
  for (let at = pid; at > 1; ) {
    if (at === root) return true;
    const stat = await readFile(`/proc/${at}/stat`, 'latin1').catch(() => '');
    // The parent is the second field after the parenthesised name.
    const parent = Number(stat.slice(stat.lastIndexOf(')') + 2).split(' ')[1]);
    if (!parent) return false;
    at = parent;
  }
  return false;
}

/** Other `telperion-heavy-*` units than the one this row runs in. */
async function otherHeavyUnits() {
  const own = (await readFile('/proc/self/cgroup', 'latin1')).match(/telperion-heavy-\d+\.scope/)?.[0];
  const { stdout } = await run('systemctl', [
    '--user', 'list-units', '--plain', '--no-legend', 'telperion-heavy-*',
  ]);
  return stdout.split('\n').map(l => l.split(' ')[0]).filter(u => u && u !== own);
}

/** Takes the heavy lock; resolves once it is held, with the call that lets it go. */
async function lock() {
  const path = `${process.env.XDG_RUNTIME_DIR ?? '/tmp'}/telperion-heavy.lock`;
  const holder = spawn('flock', [path, 'sh', '-c', 'echo held; exec cat >/dev/null'], {
    stdio: ['pipe', 'pipe', 'inherit'],
  });
  await new Promise((resolve, reject) => {
    holder.stdout.once('data', resolve);
    holder.once('exit', code => reject(new Error(`flock exited ${code} before the lock was held`)));
  });
  return () => holder.stdin.end();
}

/** What keeps a row from starting: other heavy units and telperion processes on the card. */
async function busy() {
  const ours = (await apps()).filter(a => a.command.includes('telperion'));
  return [...(await otherHeavyUnits()), ...ours.map(a => `pid ${a.pid}`)];
}

/**
 * Takes the lock once the card is quiet. While it is not, the lock is let
 * go, so two admitted rows never wait on each other, and taken again to
 * check once more.
 */
async function quietLock() {
  for (let waited = 0; ; waited += WAIT_MS) {
    const release = await lock();
    const blocking = await busy();
    if (!blocking.length) return release;
    release();
    if (waited % 30_000 === 0) {
      const at = new Date().toTimeString().slice(0, 8);
      console.error(`gpu-row: waiting for ${blocking.join(', ')} ${at}`);
    }
    await new Promise(r => setTimeout(r, WAIT_MS));
  }
}

/**
 * Runs the probe, sampling the card while it runs and once more at the
 * end; resolves with its output, exit code, foreign processes and the
 * card's state at the end.
 */
async function probe(/** @type {string[]} */ command, /** @type {Set<number>} */ present) {
  const child = spawn(command[0], command.slice(1), { stdio: ['ignore', 'pipe', 'inherit'] });
  let out = '';
  child.stdout.on('data', d => (out += d));
  /** @type {Map<number, App>} */
  const foreign = new Map();
  /** @type {Set<number>} */
  const own = new Set();
  const classify = async (/** @type {App[]} */ seen) => {
    for (const app of seen) {
      if (present.has(app.pid) || own.has(app.pid)) continue;
      if (child.pid && (await descends(app.pid, child.pid))) own.add(app.pid);
      else foreign.set(app.pid, { ...app, mib: Math.max(app.mib, foreign.get(app.pid)?.mib ?? 0) });
    }
  };
  let sampling = false;
  const sample = async () => {
    if (sampling) return;
    sampling = true;
    await classify(await apps().catch(() => []));
    sampling = false;
  };
  const timer = setInterval(sample, SAMPLE_MS);
  const exit = await new Promise(resolve => child.once('close', code => resolve(code ?? 1)));
  clearInterval(timer);
  while (sampling) await new Promise(r => setTimeout(r, 10));
  const end = await gpuState();
  await classify(end.apps);
  return { out, exit, foreign: [...foreign.values()], end };
}

function parse(/** @type {string} */ out) {
  const last = out.trim().split('\n').at(-1);
  try {
    return last ? JSON.parse(last) : null;
  } catch {
    return null;
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const [rows, dashes, ...command] = process.argv.slice(2);
  if (!rows || dashes !== '--' || !command.length) {
    console.error('usage: node scripts/gpu-row.mjs <rows.jsonl> -- <probe command...>');
    process.exit(2);
  }
  const release = await quietLock();
  try {
    const start = await gpuState();
    const result = await probe(command, new Set(start.apps.map(a => a.pid)));
    const measured = parse(result.out);
    const row = {
      command: command.join(' '),
      exit: result.exit,
      valid: result.exit === 0 && measured !== null && result.foreign.length === 0,
      foreign: result.foreign,
      gpu_start: start,
      gpu_end: result.end,
      probe: measured,
    };
    await appendFile(rows, `${JSON.stringify(row)}\n`);
    console.error(`gpu-row: valid ${row.valid}, exit ${row.exit}, ${row.foreign.length} foreign`);
    process.exitCode = result.exit;
  } finally {
    release();
  }
}
