/* ------------------------------------------------------------------ *
 * THE DIALS THE OWNER TURNS
 *
 * The harness's half of the generator contract. The panel hands the
 * generator a GrowerParams and knows nothing else about it: the family
 * under the catalogue's own names, every row of it a control
 * (`rows.ts`), the seed on its own box, and the two named transforms
 * `family.ts` applies, density and torsion. A row the catalogue adds is
 * a control without a line here.
 * ------------------------------------------------------------------ */

import { ORDINARY, type Family } from "../src/browser/core";
import { presetToParams } from "./family";

/** Seeds are unsigned 32-bit integers, and nothing else is a seed. */
export const SEED_MAX = 0xff_ff_ff_ff;

export interface GrowerParams {
  /** The family under the generator's own names: every catalogue row. */
  family: Family;
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** The attractor count as a share of the density dial's span. */
  density: number;
  /** Master over the supernatural bending: 1 is the rows as dialled. */
  torsion: number;
}

export const DEFAULT_PARAMS: GrowerParams = {
  ...presetToParams({ ...ORDINARY, id: "ordinary", name: "Ordinary", note: "" }),
  seed: 1, density: 0.5,
};

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

/** Growth is a hidden feature (owner, 2026-09-18): the harness draws the
 *  mature tree, the same one the stills and the protocol build, unless the
 *  page was opened with `?growth=1`. Any other value is the mature path. */
export function growthFromQuery(search: string): boolean {
  return new URLSearchParams(search).get("growth") === "1";
}

/** A fresh seed, drawn from the platform CSPRNG so consecutive rerolls
 *  are not neighbours in a weak sequence the eye can learn. */
export function randomSeed(): number {
  const buffer = new Uint32Array(1);
  crypto.getRandomValues(buffer);
  return buffer[0];
}
