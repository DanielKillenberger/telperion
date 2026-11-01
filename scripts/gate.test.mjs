import { spawnSync } from 'node:child_process';
import { chmodSync, cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync, realpathSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { afterEach, beforeEach, describe, expect, it } from 'vitest';
import { SUITES } from './ci-key.mjs';

/* The gate's skip (fn-322 R1, R2): the real gate.py and ci-key.mjs in a small
 * git repository holding every key input, with nextest, npm and cargo replaced
 * by a script that logs what the gate asked it to run. */
const scripts = dirname(fileURLToPath(import.meta.url));
const SHARED = ['Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.config/nextest.toml'];
const FAKE = `#!/bin/sh
root="$(git rev-parse --show-toplevel)"
if [ "$1" = nextest ] && [ "$2" = --version ]; then echo "cargo-nextest 0.9.145"; exit 0; fi
if [ "$1" = nextest ] && [ "$2" = list ]; then
  printf '{"rust-suites":{'
  sep=''
  for p in telperion-core telperion-render telperion-wasm telperion-space telperion-jev $EXTRA_PACKAGE; do
    printf '%s"%s":{"binary-name":"%s","binary-id":"%s","package-name":"%s","testcases":{"one":{"ignored":false},"two":{"ignored":false}}}' "$sep" "$p" "$p" "$p" "$p"
    sep=','
  done
  printf '}}'
  exit 0
fi
if [ "$1" = nextest ] && [ "$2" = store ]; then echo "  abc123  2026-11-01  1s  1 KB  1 failed  *latest"; exit 0; fi
echo "$(basename "$0") $*" >> "$root/calls.log"
if [ "$1" = nextest ] && [ "$2" = run ]; then
  mkdir -p "$root/target/nextest/gate"
  {
    echo '<testsuites>'
    prev=''
    for arg in "$@"; do
      if [ "$prev" = -p ]; then
        red=0
        case "$FAIL_ON" in *"$arg"*) red=1 ;; esac
        echo "<testsuite name=\\"$arg\\" failures=\\"$red\\" errors=\\"0\\">"
        echo "<testcase name=\\"one\\" classname=\\"$arg\\"/>"
        case "$CANCEL_IN" in
          *"$arg"*) ;;
          *) echo "<testcase name=\\"two\\" classname=\\"$arg\\"/>" ;;
        esac
        echo "</testsuite>"
      fi
      prev="$arg"
    done
    echo '</testsuites>'
  } > "$root/target/nextest/gate/junit.xml"
fi
case "$*" in
  *"$STOP_ON"*) [ -n "$STOP_ON" ] && exit 3 ;;
esac
case "$*" in
  *"$KILL_ON"*) [ -n "$KILL_ON" ] && kill -TERM "$PPID" && sleep 5 ;;
esac
case "$*" in
  *"$FAIL_ON"*) [ -n "$FAIL_ON" ] && exit 100 ;;
esac
exit 0
`;

let repo;
const git = (...args) => spawnSync('git', ['-c', 'user.name=t', '-c', 'user.email=t@t', ...args], { cwd: repo, encoding: 'utf8' });

function write(path, text) {
  const full = join(repo, path);
  mkdirSync(dirname(full), { recursive: true });
  writeFileSync(full, text);
}

function commitChange(path) {
  write(path, `changed ${Math.random()}`);
  git('add', '-A');
  git('commit', '-qm', `change ${path}`);
}

function gate(env = {}, ...args) {
  rmSync(join(repo, 'calls.log'), { force: true });
  const run = spawnSync('python3', ['scripts/gate.py', '--base', 'HEAD', ...args], {
    cwd: repo,
    encoding: 'utf8',
    env: {
      ...process.env,
      PATH: `${join(repo, 'fake')}:${process.env.PATH}`,
      CARGO_NEXTEST: join(repo, 'fake', 'cargo-nextest'),
      FAIL_ON: '',
      KILL_ON: '',
      STOP_ON: '',
      CANCEL_IN: '',
      ...env,
    },
  });
  const log = existsSync(join(repo, 'calls.log')) ? readFileSync(join(repo, 'calls.log'), 'utf8') : '';
  return { ...run, log, ran: ran(log) };
}

/** The suites a gate's calls ran, read from the commands, guards aside. */
function ran(log) {
  const lines = log.split('\n');
  const found = [];
  const suite = line => line.includes(' run ') && !line.includes(' -R ');
  if (lines.some(l => l.startsWith('npm run build'))) found.push('package');
  if (lines.some(l => suite(l) && l.includes('-p telperion-jev'))) found.push('rust-jev');
  if (lines.some(l => suite(l) && l.includes('-p telperion-wasm') && l.includes('-p telperion-space'))) found.push('rust-wasm');
  if (lines.some(l => suite(l) && l.includes('-p telperion-render'))) found.push('rust-render');
  if (lines.some(l => suite(l) && l.includes('-p telperion-core'))) found.push('rust-core');
  if (lines.some(l => l.startsWith('npm test'))) found.push('node');
  return found;
}

const ALL = ['package', 'rust-jev', 'rust-wasm', 'rust-render', 'rust-core', 'node'];

beforeEach(() => {
  repo = mkdtempSync(join(tmpdir(), 'telperion-gate-'));
  git('init', '-q');
  write('.gitignore', 'target/\ncalls.log\n__pycache__/\nfake/\n');
  write('.config/gate-budgets.toml', '');
  const ciInputs = ['.github/workflows/tests.yml', '.github/actions/suite/action.yml',
    '.github/actions/nextest/action.yml', '.github/actions/wasm-bindgen/action.yml'];
  const inputs = [...new Set(Object.values(SUITES).flatMap(s => s.inputs))];
  for (const input of [...SHARED, ...ciInputs, ...inputs]) {
    if (input.startsWith('scripts/')) continue;
    // A path with no extension stands for a directory, as the key sees it.
    write(/\.[a-z]+$/.test(input) ? input : `${input}/file`, input);
  }
  for (const script of ['gate.py', 'ci-key.mjs']) cpSync(join(scripts, script), join(repo, 'scripts', script), { recursive: true });
  for (const input of inputs.filter(i => i.startsWith('scripts/') && !/gate\.py|ci-key\.mjs/.test(i))) write(input, `// ${input}\n`);
  write('.config/nextest-user.toml', '');
  write('scripts/gate_test.py', '# the memory runner\n');
  mkdirSync(join(repo, 'fake'));
  for (const tool of ['cargo-nextest', 'npm', 'cargo']) {
    writeFileSync(join(repo, 'fake', tool), FAKE);
    chmodSync(join(repo, 'fake', tool), 0o755);
  }
  git('add', '-A');
  git('commit', '-qm', 'fixture');
});

afterEach(() => rmSync(repo, { recursive: true, force: true }));

describe('the gate reruns only what a change can affect', () => {
  it('runs every suite with no receipt, and none on a rerun, naming each key', () => {
    const first = gate();
    expect(first.status, first.stderr).toBe(0);
    expect(first.ran).toEqual(ALL);
    const second = gate();
    expect(second.status, second.stderr).toBe(0);
    expect(second.ran).toEqual([]);
    for (const suite of ['rust-core', 'rust-render', 'rust-wasm', 'rust-jev', 'node', 'package']) {
      expect(second.stdout).toMatch(new RegExp(`${suite} skipped, green receipt receipt-`));
    }
  });

  it('reruns only the suites a change reaches', () => {
    gate();
    commitChange('crates/telperion-render/file');
    expect(gate().ran).toEqual(['package', 'rust-jev', 'rust-render', 'node']);
    commitChange('catalogue/file');
    expect(gate().ran).toEqual(['package', 'rust-core', 'node']);
    commitChange('harness/file');
    expect(gate().ran).toEqual(['package', 'node']);
  });

  it('reruns the jev suite and the node checks for a browser source change', () => {
    gate();
    commitChange('src/file');
    expect(gate().ran).toEqual(['package', 'rust-jev', 'node']);
  });

  it.each(['Cargo.lock', 'rust-toolchain.toml', '.config/nextest.toml', 'Cargo.toml'])(
    'reruns everything when the shared input %s changes',
    shared => {
      gate();
      commitChange(shared);
      expect(gate().ran).toEqual(ALL);
    },
  );

  it('reruns everything when the key script changes', () => {
    gate();
    const script = join(repo, 'scripts', 'ci-key.mjs');
    writeFileSync(script, `${readFileSync(script, 'utf8')}\n// changed\n`);
    git('add', '-A');
    git('commit', '-qm', 'key');
    expect(gate().ran).toEqual(ALL);
  });

  it('runs nothing for a change only to .flow/ or a top-level Markdown file', () => {
    gate();
    commitChange('.flow/specs/other.md');
    commitChange('STRATEGY.md');
    const run = gate();
    expect(run.status, run.stderr).toBe(0);
    expect(run.log).toBe('');
  });

  it('leaves no receipt for a failed suite and runs it again, its failed set first', () => {
    const failed = gate({ FAIL_ON: '-p telperion-render' });
    expect(failed.status).toBe(1);
    expect(failed.stderr).toMatch(/failed: rust-render/);
    const next = gate();
    expect(next.status, next.stderr).toBe(0);
    expect(next.ran).toEqual(['rust-render']);
    const lines = next.log.trim().split('\n');
    expect(lines[0]).toMatch(/run -R abc123 .*-p telperion-render/);
    expect(lines[1]).not.toMatch(/ -R /);
    expect(gate().ran).toEqual([]);
  });

  it('stops at the failed set when it still fails', () => {
    gate({ FAIL_ON: '-p telperion-render' });
    const still = gate({ FAIL_ON: '-R abc123' });
    expect(still.status).toBe(1);
    expect(still.log.split('\n').filter(l => l.includes('-p telperion-render') && !l.includes(' -R '))).toEqual([]);
  });

  it('leaves no receipt for a suite interrupted mid-run', () => {
    const killed = gate({ KILL_ON: '-p telperion-render' });
    expect(killed.status).not.toBe(0);
    expect(gate().ran).toContain('rust-render');
  });

  it('leaves no green receipt behind a --full run that fails', () => {
    gate();
    expect(gate({ FAIL_ON: '-p telperion-render' }, '--full').status).toBe(1);
    expect(gate().ran).toEqual(['rust-render']);
  });

  it('removes each receipt before a --full run that is interrupted', () => {
    gate();
    gate({ KILL_ON: '-p telperion-render' }, '--full');
    expect(gate().ran).toEqual(expect.arrayContaining(['rust-jev', 'rust-render', 'rust-core']));
  });

  it('writes no receipt for a package a cancelled run left part way', () => {
    const run = gate({ FAIL_ON: '-p telperion-render', CANCEL_IN: 'telperion-core' });
    expect(run.status).toBe(1);
    expect(gate().ran).toEqual(['rust-render', 'rust-core']);
  });

  it('reruns the Rust suites when a budget or the memory runner changes', () => {
    gate();
    write('.config/gate-budgets.toml', '# tightened\n');
    git('add', '-A');
    git('commit', '-qm', 'budget');
    expect(gate().ran).toEqual(['rust-jev', 'rust-wasm', 'rust-render', 'rust-core']);
    commitChange('scripts/gate_test.py');
    expect(gate().ran).toEqual(['rust-jev', 'rust-wasm', 'rust-render', 'rust-core']);
  });

  it('reruns the node checks when the jev crate changes', () => {
    gate();
    commitChange('crates/telperion-jev/file');
    expect(gate().ran).toEqual(['package', 'rust-jev', 'node']);
  });

  it('writes no Rust receipt when nextest stops without a verdict', () => {
    const run = gate({ STOP_ON: 'run --no-fail-fast' });
    expect(run.status).toBe(1);
    expect(gate().ran).toEqual(['rust-jev', 'rust-wasm', 'rust-render', 'rust-core']);
  });

  it('runs the suites in one pool', () => {
    const lines = gate().log.split('\n').filter(l => l.includes('nextest run'));
    expect(lines.length).toBe(1);
    expect(lines[0]).toMatch(/-p telperion-jev -p telperion-wasm -p telperion-space -p telperion-render -p telperion-core/);
  });

  it('gives the driver a roomy cache of its own, unless the caller configured one', () => {
    const fake = join(repo, 'fake', 'cargo-nextest');
    const seen = join(repo, 'target', 'cache-seen');
    const body = readFileSync(fake, 'utf8').replace(
      'echo "$(basename "$0") $*" >> "$root/calls.log"',
      `echo "$(basename "$0") $*" >> "$root/calls.log"\nmkdir -p "$root/target"; echo "$__GL_SHADER_DISK_CACHE_PATH $__GL_SHADER_DISK_CACHE_SIZE" > "${seen}"`,
    );
    writeFileSync(fake, body);
    const unset = { __GL_SHADER_DISK_CACHE_PATH: undefined, __GL_SHADER_DISK_CACHE_SIZE: undefined };
    gate(unset);
    expect(readFileSync(seen, 'utf8').trim()).toBe(
      `${join(realpathSync(repo), 'target', 'gate', 'shader-cache')} ${4 * 2 ** 30}`,
    );
    commitChange('crates/telperion-core/file');
    gate({ ...unset, __GL_SHADER_DISK_CACHE_SIZE: '123' });
    expect(readFileSync(seen, 'utf8').trim()).toBe('123');
    commitChange('crates/telperion-core/other');
    gate({ ...unset, __GL_SHADER_DISK_CACHE_PATH: '/elsewhere' });
    expect(readFileSync(seen, 'utf8').trim()).toBe('/elsewhere');
  });

  it('writes no receipt for a suite the report does not mention', () => {
    const fake = join(repo, 'fake', 'cargo-nextest');
    writeFileSync(fake, readFileSync(fake, 'utf8').replace('<testsuite name=', '<other name='));
    const run = gate();
    expect(run.status).toBe(1);
    expect(gate().ran).toEqual(['rust-jev', 'rust-wasm', 'rust-render', 'rust-core']);
  });

  it('holds the node receipt back until both its commands pass', () => {
    const failed = gate({ FAIL_ON: 'run typecheck' });
    expect(failed.status).toBe(1);
    expect(gate().ran).toEqual(['node']);
  });

  it('holds the core receipt back until the core without geometry passes', () => {
    const failed = gate({ FAIL_ON: '--no-default-features --lib' });
    expect(failed.status).toBe(1);
    expect(gate().ran).toEqual(['rust-core']);
  });

  it('refuses a dirty tree by name, and allows bookkeeping under .flow/', () => {
    gate();
    write('crates/telperion-core/new.rs', 'dirty');
    const refused = gate();
    expect(refused.status).toBe(1);
    expect(refused.stderr).toContain('crates/telperion-core/new.rs');
    expect(refused.log).toBe('');
    rmSync(join(repo, 'crates/telperion-core/new.rs'));
    write('.flow/tmp-notes.md', 'bookkeeping');
    expect(gate().status).toBe(0);
  });

  it('refuses a dirty .flow/ file a key reads', () => {
    gate();
    write('.flow/evidence/fn19/protocol.json', 'dirty');
    const refused = gate();
    expect(refused.status).toBe(1);
    expect(refused.stderr).toContain('.flow/evidence/fn19/protocol.json');
  });

  it('runs every suite under --full whatever receipts exist', () => {
    gate();
    expect(gate({}, '--full').ran).toEqual(ALL);
  });

  it('fails when a workspace package is in no suite', () => {
    const run = gate({ EXTRA_PACKAGE: 'telperion-new' });
    expect(run.status).toBe(1);
    expect(run.stderr).toMatch(/telperion-new: in no suite/);
  });
});
