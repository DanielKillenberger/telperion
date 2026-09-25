import { execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';

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
const FIELD = 'crates/telperion-field';
// What the core crate's tests read beyond the crate: the migration fixtures
// and the species profiles and benchmark protocol under the evidence tree.
const CORE_TESTS = [
  'tests/migration',
  '.flow/evidence/fn9/profiles.json', '.flow/evidence/fn34/profiles.json',
  '.flow/evidence/fn19/protocol.json',
  // The identity, sweep and growth-reference tests read the catalogue's pins.
  'catalogue',
];

const SUITES = {
  'rust-core': { profile: 'ci', inputs: [CORE, ...CORE_TESTS] },
  'rust-render': { profile: 'ci', inputs: [CORE, RENDER] },
  'rust-wasm': { profile: 'ci', inputs: [CORE, WASM, FIELD] },
  // The isolation test walks the generation crates and the browser source,
  // the citation parser is tested on the fn-11 research section, and the
  // principles guards read every crate and the package's exports.
  'rust-jev': {
    profile: 'ci',
    inputs: ['crates/telperion-jev', CORE, RENDER, WASM, FIELD, 'src', '.flow/specs/fn-11-growth-over-time.md', 'package.json'],
  },
  // npm test and npm run typecheck: the pretest builds the three wasm crates.
  node: {
    profile: 'release',
    inputs: [
      CORE, RENDER, WASM, FIELD, 'src', 'harness', 'tests',
      'catalogue', '.gitattributes',
      'scripts/build-wasm.mjs', 'scripts/build-render.mjs',
      'scripts/catalogue-check.mjs', 'scripts/catalogue-pages.mjs',
      'package.json', 'package-lock.json',
      'tsconfig.json', 'tsconfig.build.json', 'vite.config.ts', 'vitest.config.ts',
    ],
  },
};

const suite = process.argv[2];
const spec = SUITES[suite];
if (spec === undefined) {
  console.error(`usage: node scripts/ci-key.mjs <${Object.keys(SUITES).join('|')}>`);
  process.exit(2);
}
function object(path) {
  try {
    return execFileSync('git', ['rev-parse', '--verify', '-q', `HEAD:${path}`], { cwd: root, encoding: 'utf8' }).trim();
  } catch {
    console.error(`${path} is not in HEAD; a suite input that vanished is a key script to fix, never a skip`);
    process.exit(1);
  }
}
const lines = [...SHARED, ...spec.inputs].sort().map(path => `${path} ${object(path)}`);
lines.push(`profile ${spec.profile}`);
const hash = createHash('sha256').update(lines.join('\n')).digest('hex');
console.log(`receipt-${suite}-${spec.profile}-${hash}`);
