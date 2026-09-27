import { execFile } from 'node:child_process';
import { mkdtemp, readdir, readFile, rm, stat } from 'node:fs/promises';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { promisify } from 'node:util';

/* Every shipped Wasm module and package script against the two limits in
 * artifact-budgets.json (docs/principles.md), measured on the package CI
 * built (npm run build). Run after the build; it builds nothing itself.
 *
 *   node scripts/artifact-budgets.mjs
 *
 * The ceiling holds everywhere. The per-PR growth limit holds on a pull
 * request: the base is the newest green master run of tests.yml whose
 * commit is an ancestor of the PR head and whose `package` artifact still
 * exists. With no such run the job warns that growth was not checked and
 * holds the ceilings alone.
 */
const root = new URL('../', import.meta.url);
const run = promisify(execFile);

/**
 * @typedef {{ path: string, ceilingBytes: number, reason: string }} Artifact
 * @typedef {{ recipe: string, growthShare: number, artifacts: Artifact[] }} Budgets
 * @typedef {Record<string, number>} Sizes
 * @typedef {{ label: string, sizes: Sizes }} Base
 * @typedef {{ id: number, sha: string, url: string }} Run
 * @typedef {{
 *   runs(): Promise<Run[]>,
 *   isAncestor(sha: string, head: string): Promise<boolean>,
 *   packageSizes(runId: number, paths: string[]): Promise<Sizes | null>,
 * }} BaseReader
 */

const percent = (/** @type {number} */ x) => `${(100 * x).toFixed(1)}%`;

/**
 * Every artifact missing, over its ceiling, or grown past the share over
 * `base` without a declaration, as one line each.
 * @param {Budgets} budgets
 * @param {Sizes} sizes
 * @param {{ base?: Base | null, declared?: Set<string> }} [pr]
 */
export function check(budgets, sizes, { base = null, declared = new Set() } = {}) {
  const fails = [];
  for (const { path, ceilingBytes, reason } of budgets.artifacts) {
    const size = sizes[path];
    if (size === undefined) {
      fails.push(`${path}: not built by the recipe (${budgets.recipe})`);
      continue;
    }
    if (size > ceilingBytes) {
      fails.push(`${path}: ${size} bytes, over its ceiling ${ceilingBytes} (${reason}); crossing a ceiling is an owner decision`);
    }
    const from = base?.sizes[path];
    if (from === undefined || declared.has(path)) continue;
    const growth = size / from - 1;
    if (growth > budgets.growthShare) {
      fails.push(`${path}: ${size} bytes, +${percent(growth)} over ${from} on the base ${base?.label}, above the ${percent(budgets.growthShare)} growth share per PR; name ${basename(path)} in the PR's Decisions section, or shrink the artifact`);
    }
  }
  return fails;
}

const basename = (/** @type {string} */ path) => path.slice(path.lastIndexOf('/') + 1);

/**
 * The artifacts a PR body's `## Decisions` section names by file name.
 * @param {Budgets} budgets
 * @param {string} body
 */
export function declaredGrowth(budgets, body) {
  const section = /^##[ \t]+Decisions[ \t]*$([\s\S]*?)(?=^##\s|(?![\s\S]))/m.exec((body ?? '').replace(/\r\n/g, '\n'))?.[1] ?? '';
  const names = (/** @type {string} */ path) =>
    new RegExp(`(^|[^\\w.-])${basename(path).replace(/\./g, '\\.')}(?![\\w-])`).test(section);
  return new Set(budgets.artifacts.map(a => a.path).filter(names));
}

/**
 * The newest green master run that is an ancestor of `head` and still holds
 * its package, measured; null when there is none.
 * @param {BaseReader} reader
 * @param {string} head
 * @param {string[]} paths
 * @returns {Promise<Base | null>}
 */
export async function findBase(reader, head, paths) {
  for (const { id, sha, url } of await reader.runs()) {
    if (!(await reader.isAncestor(sha, head))) continue;
    const sizes = await reader.packageSizes(id, paths);
    if (sizes) return { label: `${sha.slice(0, 8)} (${url})`, sizes };
  }
  return null;
}

/**
 * The live reader: tests.yml's green master runs through the GitHub API.
 * @param {string} repo
 * @returns {BaseReader}
 */
export function githubReader(repo) {
  const gh = async (/** @type {string[]} */ ...args) => (await run('gh', args, { maxBuffer: 1 << 26 })).stdout.trim();
  return {
    async runs() {
      const rows = await gh('api', `repos/${repo}/actions/workflows/tests.yml/runs?branch=master&event=push&status=success&per_page=30`,
        '--jq', '.workflow_runs[] | [.id, .head_sha, .html_url] | @tsv');
      return rows.split('\n').filter(Boolean).map(row => {
        const [id, sha, url] = row.split('\t');
        return { id: Number(id), sha, url };
      });
    },
    async isAncestor(sha, head) {
      const status = await gh('api', `repos/${repo}/compare/${sha}...${head}`, '--jq', '.status');
      return status === 'ahead' || status === 'identical';
    },
    async packageSizes(runId, paths) {
      const live = await gh('api', `repos/${repo}/actions/runs/${runId}/artifacts`,
        '--jq', '[.artifacts[] | select(.name == "package" and (.expired | not))] | length');
      if (live === '0') return null;
      const dir = await mkdtemp(join(tmpdir(), 'base-package-'));
      try {
        await gh('run', 'download', String(runId), '-R', repo, '-n', 'package', '-D', dir);
        const tarball = (await readdir(dir)).find(f => f.endsWith('.tgz'));
        if (!tarball) return null;
        await run('tar', ['-xzf', join(dir, tarball), '-C', dir, 'package/dist']);
        return await measure(paths, join(dir, 'package'));
      } finally {
        await rm(dir, { recursive: true, force: true });
      }
    },
  };
}

/**
 * The size of each path under `dir` that exists.
 * @param {string[]} paths
 * @param {string} dir
 */
async function measure(paths, dir) {
  /** @type {Sizes} */
  const sizes = {};
  for (const path of paths) {
    try {
      sizes[path] = (await stat(join(dir, path))).size;
    } catch {}
  }
  return sizes;
}

/** @returns {Promise<Budgets>} */
export async function budgets() {
  return JSON.parse(await readFile(new URL('scripts/artifact-budgets.json', root), 'utf8'));
}

/**
 * The PR's base and declarations, or null with the reason growth is not checked.
 * @param {Budgets} b
 */
async function pullRequest(b) {
  const { GITHUB_EVENT_NAME, GITHUB_REPOSITORY: repo, PR_NUMBER, PR_HEAD_SHA: head } = process.env;
  if (GITHUB_EVENT_NAME !== 'pull_request' || !repo || !PR_NUMBER || !head) {
    return { skip: 'not a pull request run; the growth limit is checked on pull requests' };
  }
  const reader = githubReader(repo);
  try {
    const base = await findBase(reader, head, b.artifacts.map(a => a.path));
    if (!base) return { skip: `no green master run that is an ancestor of ${head} still holds its package artifact` };
    const body = await run('gh', ['api', `repos/${repo}/pulls/${PR_NUMBER}`, '--jq', '.body // ""']);
    return { base, declared: declaredGrowth(b, body.stdout) };
  } catch (e) {
    return { skip: `the base lookup failed: ${e instanceof Error ? e.message.split('\n')[0] : e}` };
  }
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
  const b = await budgets();
  const sizes = await measure(b.artifacts.map(a => a.path), fileURLToPath(root));
  const pr = await pullRequest(b);
  if ('skip' in pr) {
    console.log(`::warning title=Size growth not checked::${pr.skip}; the ceilings alone were enforced`);
  } else {
    console.log(`base: ${pr.base.label}`);
  }
  for (const { path, ceilingBytes } of b.artifacts) {
    if (sizes[path] === undefined) continue;
    const from = 'base' in pr ? pr.base?.sizes[path] : undefined;
    const growth = from === undefined ? '' : `, ${sizes[path] >= from ? '+' : ''}${percent(sizes[path] / from - 1)} over base ${from}`;
    const named = 'declared' in pr && pr.declared?.has(path) ? ', growth named in Decisions' : '';
    console.log(`${path}: ${sizes[path]} bytes (ceiling ${ceilingBytes}${growth}${named})`);
  }
  const fails = check(b, sizes, 'skip' in pr ? {} : pr);
  for (const f of fails) console.error(`FAIL ${f}`);
  process.exit(fails.length ? 1 : 0);
}
