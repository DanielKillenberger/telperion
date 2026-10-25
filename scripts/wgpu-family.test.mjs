import { readFile } from 'node:fs/promises';
import { describe, expect, it } from 'vitest';

/* The wgpu family comes from the fork at one rev (fn-311 R2,
 * docs/wgpu-fork.md). Cargo.lock records every target's resolution at once,
 * so a family crate taken twice, from crates.io or from another rev shows up
 * here whichever target pulls it in. */
const root = new URL('../', import.meta.url);
const lock = await readFile(new URL('Cargo.lock', root), 'utf8');
const toml = await readFile(new URL('Cargo.toml', root), 'utf8');
const NAMED = ['wgpu', 'wgpu-core', 'wgpu-hal', 'wgpu-types', 'naga', 'wgpu-naga-bridge'];
const FORK = 'https://github.com/DanielKillenberger/wgpu';

/** The rev every entry of the root patch table names; more than one is a fault. */
function pinned(manifest) {
  const table = /^\[patch\.crates-io\]\n([\s\S]*?)(?=^\[|(?![\s\S]))/m.exec(manifest)?.[1] ?? '';
  const revs = new Set([...table.matchAll(/rev = "([0-9a-f]+)"/g)].map(m => m[1]));
  if (revs.size !== 1) throw Error(`the patch table names ${revs.size} revs, not one`);
  return [...revs][0];
}

/** Every way the lock's family entries break the one-of-each rule. */
function faults(lockText, rev) {
  const entries = lockText.split('[[package]]\n').slice(1).map(block => ({
    name: /^name = "([^"]+)"/m.exec(block)?.[1],
    source: /^source = "([^"]+)"/m.exec(block)?.[1] ?? 'the workspace',
  }));
  const family = entries.filter(e => /^(wgpu|naga)(-|$)/.test(e.name));
  const wanted = `git+${FORK}?rev=${rev}#${rev}`;
  const found = [];
  for (const name of new Set([...NAMED, ...family.map(e => e.name)])) {
    const own = family.filter(e => e.name === name);
    if (own.length !== 1) found.push(`${name}: ${own.length} entries, not one`);
    for (const e of own.filter(e => e.source !== wanted)) found.push(`${name}: from ${e.source}`);
  }
  return found;
}

describe('the wgpu family', () => {
  const rev = pinned(toml);

  it('is pinned to a full sha and taken once each, from the fork at that rev', () => {
    expect(rev).toMatch(/^[0-9a-f]{40}$/);
    expect(faults(lock, rev)).toEqual([]);
  });

  it.each([
    ['crates.io', 'source = "registry+https://github.com/rust-lang/crates.io-index"', 'naga: from registry+'],
    ['another rev', `source = "git+${FORK}?rev=${'0'.repeat(40)}#${'0'.repeat(40)}"`, `naga: from git+${FORK}?rev=000`],
  ])('fails an entry from %s', (_, source, said) => {
    const edited = lock.replace(/(name = "naga"\nversion = "[^"]+"\n)source = "[^"]+"/, `$1${source}`);
    expect(faults(edited, rev)).toEqual([expect.stringContaining(said)]);
  });

  it('fails a family crate taken twice', () => {
    const naga = /\[\[package\]\]\nname = "naga"\n[\s\S]*?\n\n/.exec(lock)[0];
    expect(faults(lock + '\n' + naga, rev)).toEqual(['naga: 2 entries, not one']);
  });
});
