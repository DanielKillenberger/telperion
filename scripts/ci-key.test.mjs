import { execFileSync } from 'node:child_process';
import { relative } from 'node:path';
import { fileURLToPath } from 'node:url';
import { describe, expect, it } from 'vitest';
import { SUITES, key } from './ci-key.mjs';

/* A suite's key must move when any crate it builds changes (fn-294 R3).
 * The workspace's own dependency graph, read from `cargo metadata`, says
 * which crates a crate builds; every suite that lists a crate lists each
 * crate that crate depends on, dev and build dependencies among them. */
const root = fileURLToPath(new URL('../', import.meta.url));
const metadata = JSON.parse(execFileSync('cargo', ['metadata', '--no-deps', '--format-version', '1'], { cwd: root, encoding: 'utf8' }));
const dir = manifest => relative(root, manifest).replace(/\/Cargo\.toml$/, '');
const byName = Object.fromEntries(metadata.packages.map(p => [p.name, p]));
/** The workspace crates a crate builds, itself excluded, transitively. */
function builds(crate, seen = new Set()) {
  for (const d of byName[crate].dependencies) {
    if (d.path && byName[d.name] && !seen.has(d.name) && d.name !== crate) {
      seen.add(d.name);
      builds(d.name, seen);
    }
  }
  return seen;
}
const crates = Object.fromEntries(metadata.packages.map(p => [dir(p.manifest_path), p.name]));

describe('suite keys', () => {
  it('list every workspace crate a listed crate builds', () => {
    const missing = [];
    for (const [suite, { inputs }] of Object.entries(SUITES)) {
      for (const path of inputs.filter(p => crates[p])) {
        for (const dep of builds(crates[path])) {
          const depPath = dir(byName[dep].manifest_path);
          if (!inputs.includes(depPath)) missing.push(`${suite}: ${path} builds ${depPath}`);
        }
      }
    }
    expect(missing).toEqual([]);
  });

  it('move when only the tree space changes, for every suite that builds it', () => {
    const space = 'crates/telperion-space';
    const dependent = Object.keys(SUITES).filter(s => SUITES[s].inputs.some(p => p === space || (crates[p] && builds(crates[p]).has(crates[space]))));
    expect(dependent.sort()).toEqual(['node', 'rust-core', 'rust-jev', 'rust-render', 'rust-wasm']);
    for (const suite of dependent) {
      const before = key(suite, () => 'a');
      const after = key(suite, path => (path === space ? 'b' : 'a'));
      expect(after, suite).not.toBe(before);
    }
  });
});
