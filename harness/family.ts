import type { Family, TreePreset } from "../src/browser/core";
import { PARAMETERS } from "../src/browser/parameters.generated";

import type { GrowerParams } from "./params";
import { admit, readRow } from "./rows";

/* ------------------------------------------------------------------ *
 * THE DIALS, AS THE GENERATOR'S OWN ARGUMENTS
 *
 * What the panel holds becomes what the generator takes. Every catalogue
 * row travels under its own name in `params.family`; two dials are not
 * rows but named transforms over rows, and they are the only
 * translation here. Nothing here draws anything, and nothing here knows
 * what will.
 * ------------------------------------------------------------------ */

/** What the density dial spans, in attractors. The floor is a tree
 *  with a readable handful of limbs rather than a bare fork; the
 *  ceiling is where the crown stops gaining structure and starts
 *  gaining only cost. */
const ATTRACTORS_MIN = 250;
const ATTRACTORS_MAX = 1600;

/** A dial that is not a catalogue row: a named transform over rows. */
export interface Adapter {
  key: "density" | "torsion";
  /** The group whose rows it moves. */
  group: string;
  meaning: string;
  /** The row it stands in for, whose own control it replaces. */
  owns?: string;
  /** The rows it writes: their bounds hold it, their dormancy is its own. */
  moves: readonly string[];
  min: number;
  max: number;
  step: number;
}

export const ADAPTERS: readonly Adapter[] = [
  {
    key: "density", group: "/skeleton", owns: "/skeleton/attractors", moves: ["/skeleton/attractors"],
    meaning: `The attractor count as a share of ${ATTRACTORS_MIN} to ${ATTRACTORS_MAX}: how thickly the envelope is populated, branch count and not leaves.`,
    min: 0, max: 1, step: 0.01,
  },
  {
    key: "torsion", group: "/skeleton/bias/supernatural",
    moves: ["/skeleton/bias/supernatural/writheAmplitude", "/skeleton/bias/supernatural/spiralRate"],
    meaning: "Master over writhe amplitude and spiral rate: 0 leaves the centreline straight whatever they say, 1 is the two as dialled. Lean, gravitropism and the surface twist are outside it.",
    min: 0, max: 2, step: 0.01,
  },
];

/** Whether the adapter may take `value`: every row it moves lands inside
 *  that row's bounds, so a dial move never sends a family the wire refuses. */
export function adapterAdmits(params: GrowerParams, adapter: Adapter, value: number): boolean {
  if (!Number.isFinite(value)) return false;
  const family = toFamily({ ...params, [adapter.key]: value });
  return adapter.moves.every(path => {
    const row = PARAMETERS.find(p => p.path === path);
    const moved = readRow(family, path);
    return row !== undefined && typeof moved === "number" && admit(row, moved) === moved;
  });
}

/** The attractor count the density dial stands for. */
export function attractors(density: number): number {
  return Math.round(ATTRACTORS_MIN + density * (ATTRACTORS_MAX - ATTRACTORS_MIN));
}

/** A preset's parameters, as the panel's dials: the family as authored,
 *  its seed on the seed box, its attractor count as a density, and
 *  torsion at 1, the master that leaves the bending as the preset states
 *  it. Exact: `toFamily` applied to the result is the preset's family. */
export function presetToParams(preset: TreePreset): GrowerParams {
  const { id: _id, name: _name, note: _note, ...family } = preset;
  return {
    family: structuredClone(family),
    seed: family.skeleton.seed,
    density: (family.skeleton.attractors - ATTRACTORS_MIN) / (ATTRACTORS_MAX - ATTRACTORS_MIN),
    torsion: 1,
  };
}

/** The dials as the generator reads them: the family with the seed, the
 *  density and the torsion master applied to the rows they move.
 *
 *  The renderer parses this with the core's own schema, so the keys are
 *  the schema's keys and an unknown one is refused rather than ignored. */
export function toFamily(params: GrowerParams): Family {
  const family = structuredClone(params.family);
  family.skeleton.seed = params.seed;
  family.skeleton.attractors = attractors(params.density);
  const bending = family.skeleton.bias.supernatural;
  bending.writheAmplitude *= params.torsion;
  bending.spiralRate *= params.torsion;
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
