import type { Family, TreePreset } from "../src/browser/core";

import type { GrowerParams } from "./params";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS THE GENERATOR'S OWN ARGUMENTS
 *
 * What the panel holds becomes what the generator takes. Every catalogue
 * row travels under its own name in `params.family`, and the seed box is
 * the one translation here. Nothing here draws anything, and nothing
 * here knows what will.
 * ------------------------------------------------------------------ */

/** A preset's parameters, as the panel's dials: the family as authored
 *  and its seed on the seed box. Exact: `toFamily` applied to the result
 *  is the preset's family. */
export function presetToParams(preset: TreePreset): GrowerParams {
  const { id: _id, name: _name, note: _note, ...family } = preset;
  return {
    family: structuredClone(family),
    seed: family.skeleton.seed,
  };
}

/** The dials as the generator reads them: the family with the seed.
 *
 *  The renderer parses this with the core's own schema, so the keys are
 *  the schema's keys and an unknown one is refused rather than ignored. */
export function toFamily(params: GrowerParams): Family {
  const family = structuredClone(params.family);
  family.skeleton.seed = params.seed;
  return family;
}

/** The same family as the text the renderer is handed. A non-finite
 *  dial would otherwise travel as `null` and come back as the schema's
 *  general type complaint, which says nothing about which dial it was. */
export function familyJson(params: GrowerParams): string {
  return JSON.stringify(toFamily(params), (key, value: unknown) => {
    if (typeof value === "number" && !Number.isFinite(value)) {
      throw Error(`Tree parameter "${key}" is not a finite number`);
    }
    return value;
  });
}
