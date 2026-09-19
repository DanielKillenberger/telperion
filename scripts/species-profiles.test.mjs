// Which frozen profile set the species runner loads when nobody names one.
// The records a preset is gated against live in one cohort's set, and an agent
// that has to know which one pays for the lookup twice: once in a failed run,
// once in reading the evidence tree. These cases pin the resolution, the
// fallback and the flag that still wins.
import { describe, expect, it } from 'vitest';
import { execFileSync } from 'node:child_process';
import { readFileSync } from 'node:fs';
import { join } from 'node:path';

import { ROOT, FALLBACK_PROFILE_SET, matchedRecords, resolveProfileSet } from './species-profiles.mjs';

const FN9 = join(ROOT, '.flow/evidence/fn9/profiles.json');
const FN34 = join(ROOT, '.flow/evidence/fn34/profiles.json');
const catalogue = join(ROOT, 'catalogue');
const listed = (path) => JSON.parse(readFileSync(path, 'utf8')).profiles.map((profile) => profile.id);

/** Every preset the runner can be handed, read off the native tables rather
 *  than restated: the listed catalogue, and the tables still being judged,
 *  which are unlisted in the browser and reached here by name. */
function shippedPresets() {
  const source = readFileSync(join(ROOT, 'crates/telperion-core/src/params.rs'), 'utf8');
  const table = (name) => {
    const body = new RegExp(`pub const ${name}:[^=]+=\\s*&\\[([\\s\\S]*?)\\];`).exec(source)?.[1];
    const ids = [...(body ?? '').matchAll(/\(\s*\d+,\s*"([a-z-]+)"/g)].map((entry) => entry[1]);
    if (ids.length === 0) throw Error(`No ${name} preset table in params.rs`);
    return ids;
  };
  return [...table('CATALOGUE'), ...table('IN_WORK')];
}

/** Every one of them and the set its numbers are gated against. A preset that
 *  arrives without a line here fails the last case, which is the coverage the
 *  spec asks for. */
const EXPECTED = {
  'ordinary': FN9,
  'oregon-white-oak': FN9,
  'norway-spruce': FN9,
  'european-beech': FN34,
  'silver-birch': FN34,
  'telperion': FN9,
  'laurelin': FN9,
};

describe('the resolved default', () => {
  it('sends the beech and the birch to the cohort that carries their records', () => {
    for (const preset of ['european-beech', 'silver-birch']) {
      expect(resolveProfileSet(preset)).toBe(FN34);
      expect(listed(FN34)).toContain(preset);
      expect(matchedRecords(catalogue, preset).length).toBeGreaterThan(0);
    }
  });

  it("leaves the oak and the spruce on the protocol's own set", () => {
    for (const preset of ['oregon-white-oak', 'norway-spruce']) {
      expect(resolveProfileSet(preset)).toBe(FN9);
      expect(listed(FN9)).toContain(preset);
      // Unchanged: neither species asks for a matched still, so resolution
      // cannot have moved a record under either of them.
      expect(matchedRecords(catalogue, preset)).toEqual([]);
    }
  });

  it('falls back for a preset no profile set profiles', () => {
    expect(FALLBACK_PROFILE_SET).toBe('.flow/evidence/fn9/profiles.json');
    for (const preset of ['ordinary', 'telperion', 'laurelin']) {
      expect(listed(FN9)).not.toContain(preset);
      expect(matchedRecords(catalogue, preset)).toEqual([]);
      expect(resolveProfileSet(preset)).toBe(FN9);
    }
  });

  it('names the preset and the sets searched when no set carries its records', () => {
    const narrowed = () => resolveProfileSet('european-beech', { sets: ['.flow/evidence/fn9/profiles.json'] });
    expect(narrowed).toThrow(/european-beech/);
    expect(narrowed).toThrow(/\.flow\/evidence\/fn9\/profiles\.json/);
  });

  it('names a set the table lists and the tree does not carry', () => {
    expect(() => resolveProfileSet('european-beech', { sets: ['.flow/evidence/fn0/profiles.json'] }))
      .toThrow(/\.flow\/evidence\/fn0\/profiles\.json/);
  });

  it('covers every shipped preset', () => {
    expect(Object.keys(EXPECTED).sort()).toEqual(shippedPresets().sort());
    for (const [preset, path] of Object.entries(EXPECTED)) expect(resolveProfileSet(preset)).toBe(path);
  });
});

describe('the flag', () => {
  // The override is the runner's own wiring, so it is read off the runner: the
  // beech under the fn-9 set has no matched record, which only the flag can
  // arrange. Nothing here reaches the GPU.
  it('still wins over the resolved default', () => {
    let failure;
    try {
      execFileSync('node', ['tests/species.mjs', '--quick', 'european-beech',
        '--profiles', '.flow/evidence/fn9/profiles.json'], { cwd: ROOT, encoding: 'utf8', stdio: 'pipe' });
    } catch (error) { failure = error; }
    expect(failure?.status).not.toBe(0);
    expect(failure?.stderr).toContain('No matched reference records for european-beech');
  }, 60_000);
});
