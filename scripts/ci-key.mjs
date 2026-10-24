import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { pathToFileURL } from 'node:url';

/* The receipt key of a test suite: a hash over the git objects of everything
 * the suite reads, the toolchain file among them, and the cargo profile it
 * runs under. A green run saves an Actions cache entry under the key and the
 * next run whose inputs hash to it skips the suite. The key reads the commit,
 * never the working tree, so it is the same wherever the commit is checked out.
 *
 *   node scripts/ci-key.mjs rust-core
 */
const root = new URL('../', import.meta.url);

const SHARED = [
  'Cargo.toml', 'Cargo.lock', 'rust-toolchain.toml', '.config/nextest.toml',
  '.github/workflows/tests.yml', '.github/actions/suite/action.yml',
  '.github/actions/nextest/action.yml', '.github/actions/wasm-bindgen/action.yml',
  'scripts/ci-key.mjs',
];
const CORE = 'crates/telperion-core';
const RENDER = 'crates/telperion-render';
const WASM = 'crates/telperion-wasm';
// The tree space's engine: the core and the renderer depend on it, so every
// suite that builds either lists it (ci-key.test.mjs checks the graph).
const SPACE = 'crates/telperion-space';
// What the core crate's tests read beyond the crate: the migration fixtures
// and the species profiles and benchmark protocol under the evidence tree.
const CORE_TESTS = [
  'tests/migration',
  '.flow/evidence/fn9/profiles.json', '.flow/evidence/fn34/profiles.json',
  '.flow/evidence/fn19/protocol.json',
  // The identity, sweep and growth-reference tests read the catalogue's pins.
  'catalogue',
];

export const SUITES = {
  'rust-core': { profile: 'ci', inputs: [CORE, SPACE, ...CORE_TESTS] },
  'rust-render': { profile: 'ci', inputs: [CORE, RENDER, SPACE] },
  'rust-wasm': { profile: 'ci', inputs: [CORE, WASM, SPACE] },
  // The isolation test walks the generation crates and the browser source,
  // and the citation parser is tested on the fn-11 research section.
  'rust-jev': {
    profile: 'ci',
    inputs: ['crates/telperion-jev', CORE, RENDER, WASM, SPACE, 'src', '.flow/specs/fn-11-growth-over-time.md'],
  },
  // npm test and npm run typecheck: the pretest builds the three wasm crates.
  node: {
    profile: 'release',
    inputs: [
      CORE, RENDER, WASM, SPACE, 'src', 'harness', 'tests',
      'catalogue', '.gitattributes',
      'scripts/build-wasm.mjs', 'scripts/build-render.mjs',
      'scripts/catalogue-check.mjs', 'scripts/catalogue-pages.mjs',
      'scripts/artifact-budgets.mjs', 'scripts/artifact-budgets.json', 'scripts/artifact-budgets.test.mjs',
      'scripts/ci-key.test.mjs', 'scripts/wgpu-family.test.mjs',
      'package.json', 'package-lock.json',
      'tsconfig.json', 'tsconfig.build.json', 'vite.config.ts', 'vitest.config.ts',
    ],
  },
};

/** A suite's key, each input path read as its git object by `object`. */
export function key(suite, object) {
  const spec = SUITES[suite];
  const lines = [...SHARED, ...spec.inputs].sort().map(path => `${path} ${object(path)}`);
  lines.push(`profile ${spec.profile}`);
  const hash = createHash('sha256').update(lines.join('\n')).digest('hex');
  return `receipt-${suite}-${spec.profile}-${hash}`;
}

function head(path) {
  try {
    return execFileSync('git', ['rev-parse', '--verify', '-q', `HEAD:${path}`], { cwd: root, encoding: 'utf8' }).trim();
  } catch {
    console.error(`${path} is not in HEAD; a suite input that vanished is a key script to fix, never a skip`);
    process.exit(1);
  }
}

if (import.meta.url === pathToFileURL(process.argv[1] ?? '').href) {
  const suite = process.argv[2];
  if (SUITES[suite] === undefined) {
    console.error(`usage: node scripts/ci-key.mjs <${Object.keys(SUITES).join('|')}>`);
    process.exit(2);
  }
  console.log(key(suite, head));
}
