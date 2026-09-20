// Where a preset's frozen numbers and its matched reference records live.
//
// A species is tuned inside one cohort, and that cohort's profiles.json is the
// set its measurements are gated against: the oak and the spruce in fn-9's, the
// beech and the birch in fn-34's. The runner used to default to fn-9's for
// every preset, so a look at the beech failed on records nothing had searched
// for until someone passed `--profiles` by hand. Resolution fills that default
// from the preset; the flag keeps its meaning and still overrides it.
import { readFileSync } from 'node:fs';
import { dirname, join, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

export const ROOT = resolve(dirname(fileURLToPath(import.meta.url)), '..');

/** Every profile set the runner knows, in the order they are searched: a cohort
 *  that re-tunes a species is listed above the one it supersedes. */
export const PROFILE_SETS = [
  '.flow/evidence/fn34/profiles.json',
  '.flow/evidence/fn9/profiles.json',
];
/** The protocol's own set, and the default for a preset no set profiles. */
export const FALLBACK_PROFILE_SET = '.flow/evidence/fn9/profiles.json';

/** A species' reference records live in its catalogue folder, the one place a
 *  species record lives. */
export const referenceFile = (catalogue, preset) => join(catalogue, preset, 'packet', 'references.json');

/** The records that ask for a matched still. A preset with no catalogue folder,
 *  or none that carries a shot block, has none. */
export function matchedRecords(catalogue, preset) {
  try {
    return JSON.parse(readFileSync(referenceFile(catalogue, preset), 'utf8')).references.filter((r) => r.shot);
  } catch {
    return [];
  }
}

/** The profile set to load for one preset. The set that profiles it wins; a
 *  preset with no matched record of its own stays on the fallback, which is
 *  where every synthetic family and the two species tuned there already sit.
 *  Records no set can gate are a mistake, and naming it costs one line. */
export function resolveProfileSet(preset, options = {}) {
  const { root = ROOT, sets = PROFILE_SETS, catalogue = join(root, 'catalogue') } = options;
  for (const set of sets) {
    const path = join(root, set);
    let listed;
    try {
      listed = JSON.parse(readFileSync(path, 'utf8')).profiles;
    } catch (error) {
      throw Error(`Unreadable profile set ${set}: ${error.message}`);
    }
    if (listed.some((profile) => profile.id === preset)) return path;
  }
  if (matchedRecords(catalogue, preset).length === 0) return join(root, FALLBACK_PROFILE_SET);
  throw Error(`No profile set carries the reference records of ${preset}; searched ${sets.join(', ')}`);
}
