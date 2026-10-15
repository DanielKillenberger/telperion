/* ------------------------------------------------------------------ *
 * THE DIALS THE OWNER TURNS
 *
 * The harness's half of the generator contract. The panel hands the
 * generator a GrowerParams and knows nothing else about it: the family
 * under the catalogue's own names, every row of it a control
 * (`rows.ts`) and the seed on its own box. A row the catalogue adds is
 * a control without a line here.
 * ------------------------------------------------------------------ */

import { ORDINARY, presetById, type Family } from "../src/browser/core";
import { presetToParams } from "./family";

/** Seeds are unsigned 32-bit integers, and nothing else is a seed. */
export const SEED_MAX = 0xff_ff_ff_ff;

export interface GrowerParams {
  /** The family under the generator's own names: every catalogue row. */
  family: Family;
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
}

export const DEFAULT_PARAMS: GrowerParams = {
  ...presetToParams({ ...ORDINARY, id: "ordinary", name: "Ordinary", note: "" }),
  seed: 1,
};

/** The tree the harness opens on and resets to: the engine's beech at its
 *  preset age (fn-198 host decision 5). The Ordinary rows above stay the
 *  dials' trait baseline; the Ordinary tree, the space's centroid since
 *  fn-215, is not the tree it opens on. */
export const OPENING_PARAMS: GrowerParams = { ...presetToParams(presetById("european-beech")), seed: 1 };

/** The seed field is the one free-text surface on the panel, so it is
 *  the one place a value can arrive as anything at all. Parsed
 *  strictly: Number() would take "1e3", " 12", "0x4" and "", none of
 *  which anyone types into a seed box on purpose. Returns null when
 *  the text is not a seed, and the caller keeps the last good one. */
export function normalizeSeed(raw: string): number | null {
  if (!/^\d{1,10}$/.test(raw)) return null;
  const value = Number(raw);
  if (!Number.isInteger(value) || value > SEED_MAX) return null;
  return value;
}

/** An age in a link (`?age=`): whole years from one, strictly parsed as a
 *  seed is. Null where the text is not one, and the preset's age stands. */
export function normalizeAge(raw: string): number | null {
  if (!/^\d{1,6}$/.test(raw)) return null;
  const value = Number(raw);
  return value >= 1 ? value : null;
}

/** A fresh seed, drawn from the platform CSPRNG so consecutive rerolls
 *  are not neighbours in a weak sequence the eye can learn. */
export function randomSeed(): number {
  const buffer = new Uint32Array(1);
  crypto.getRandomValues(buffer);
  return buffer[0];
}
