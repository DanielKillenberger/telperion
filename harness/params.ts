/* ------------------------------------------------------------------ *
 * THE DIALS THE OWNER TURNS
 *
 * The harness's half of the generator contract. The panel renders
 * whatever is in SLIDERS and hands the generator a GrowerParams; it
 * knows nothing else about either. skeleton-view.ts translates these
 * into the generator's own arguments, so a new dial is added here and
 * the panel picks it up for free.
 *
 * Every name here points at a mechanism the spec already committed to:
 * `height` and `spread` are the authored envelope, the five bias dials
 * are the growth bias field's own five terms under their own names,
 * `taper` is the radius solve's fork exponent, `density` is how many
 * attractors the envelope gets. Nothing is a knob invented for the
 * panel's sake.
 *
 * The radius solve's other two terms - how stout the trunk is and how
 * fast a limb thins along its length - are library parameters with
 * documented defaults and no dial, on the same footing as the growth
 * distances in grow.ts: nothing has asked to turn them live yet, and
 * each is one line in SLIDERS the day something does.
 *
 * The bias dials are five and not one, because a single number mixes
 * qualities that are independent - a tree that leans is a different
 * tree from one that wanders, and amplitude and wavelength are the
 * difference between a slow S-curve and a corkscrew. Their names and
 * units are the library's, unchanged, so there is no translation to
 * drift.
 *
 * `torsion` sits above the three of them that are departures from
 * vertical and scales all three at once, which is the "straight to
 * writhing in one move" the spec asks for. It is a master over the
 * shape those five describe, not a sixth quality: at 0 the tree is
 * dead straight whatever the others say, at 1 it is exactly what they
 * say, and it runs to 2 for a look the individual dials would have to
 * be walked to one at a time.
 * ------------------------------------------------------------------ */

import { DEFAULT_RADII } from "@/lib/grower/radius";
import { DEFAULT_MAX_TURN_PER_STEP } from "@/lib/grower/skeleton/colonize";
import { DEFAULT_BIAS } from "@/lib/grower/torsion";

/** Seeds are unsigned 32-bit integers, and nothing else is a seed. */
export const SEED_MAX = 0xff_ff_ff_ff;

export interface GrowerParams {
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** Envelope height, in metres. The tree is judged at human scale. */
  height: number;
  /** Envelope half-width as a fraction of height, so the crown's width
   *  over the tree's height is twice this. The range spans real trees:
   *  0.12 is a narrow upright Telperion at about a quarter as wide as
   *  tall, 0.65 a broad domed Laurelin at about one and a third. Above
   *  that a crown stops reading as a tree and starts reading as a
   *  hedge. */
  spread: number;
  /** Master over `lean`, `writheAmplitude` and `spiralRate`: 0 leaves
   *  the tree dead straight, 1 is the three of them as dialled. */
  torsion: number;
  /** Upward pull on every growth step. */
  gravitropism: number;
  /** Steady departure from vertical: horizontal metres per metre climbed. */
  lean: number;
  /** How far the centreline strays from its mean path, as a fraction
   *  of height. */
  writheAmplitude: number;
  /** The length of one bend, as a fraction of height. */
  writheWavelength: number;
  /** Turns about the trunk axis over the tree's full height. */
  spiralRate: number;
  /** Directional persistence: how far one growth step may turn from
   *  the step before it, in degrees. Bending stiffness - low is a limb
   *  that commits to a direction, high is one that follows whatever is
   *  nearest. */
  maxTurnPerStep: number;
  /** How thickly the envelope is populated: branch count, not leaves. */
  density: number;
  /** The radius solve's fork exponent: what a fork does to thickness,
   *  and so the contrast between trunk and twig. 2 conserves
   *  cross-sectional area exactly. */
  taper: number;
}

export interface SliderSpec {
  key: Exclude<keyof GrowerParams, "seed">;
  label: string;
  min: number;
  max: number;
  step: number;
  /** Suffix shown next to the value. Empty for a bare ratio. */
  unit: string;
}

export const SLIDERS: readonly SliderSpec[] = [
  { key: "height", label: "height", min: 4, max: 60, step: 0.5, unit: "m" },
  { key: "spread", label: "spread", min: 0.12, max: 0.65, step: 0.01, unit: "" },
  // The bias dials run well past what looks good. The owner has to be
  // able to see where too much is, or the usable range sits at the
  // ceiling and reads as a limit rather than as a choice.
  { key: "torsion", label: "torsion", min: 0, max: 2, step: 0.01, unit: "x" },
  { key: "gravitropism", label: "gravitropism", min: 0, max: 1.3, step: 0.01, unit: "" },
  { key: "lean", label: "lean", min: 0, max: 0.5, step: 0.01, unit: "" },
  { key: "writheAmplitude", label: "writhe", min: 0, max: 0.25, step: 0.01, unit: "" },
  { key: "writheWavelength", label: "bend length", min: 0.18, max: 1.2, step: 0.01, unit: "" },
  { key: "spiralRate", label: "spiral", min: 0, max: 6, step: 0.1, unit: "" },
  // Stiffness, and the one dial that is a rail as well as a look: past
  // about 90 a step can turn back on the one before it and the crown
  // starts drawing the sawtooth fn-11.8 was about, so the dial stops
  // where the rail does rather than showing the owner a range whose top
  // end is a bug.
  { key: "maxTurnPerStep", label: "turn limit", min: 5, max: 90, step: 1, unit: "deg/step" },
  { key: "density", label: "density", min: 0, max: 1, step: 0.01, unit: "" },
  // The fork exponent, under the name the owner already turns. Below
  // 2 a fork sheds more than area and the tree runs from a heavy
  // trunk to threads; above 3 the limbs stop thinning enough to read
  // as limbs. The dial spans both sides of that so the good range is
  // visibly a choice.
  { key: "taper", label: "taper", min: 1.4, max: 3.6, step: 0.05, unit: "n" },
];

export const DEFAULT_PARAMS: GrowerParams = {
  seed: 1,
  height: 24,
  spread: 0.3,
  torsion: 1,
  gravitropism: DEFAULT_BIAS.gravitropism,
  lean: DEFAULT_BIAS.lean,
  writheAmplitude: DEFAULT_BIAS.writheAmplitude,
  writheWavelength: DEFAULT_BIAS.writheWavelength,
  spiralRate: DEFAULT_BIAS.spiralRate,
  maxTurnPerStep: DEFAULT_MAX_TURN_PER_STEP,
  density: 0.5,
  taper: DEFAULT_RADII.forkExponent,
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

/** A fresh seed, drawn from the platform CSPRNG so consecutive rerolls
 *  are not neighbours in a weak sequence the eye can learn. */
export function randomSeed(): number {
  const buffer = new Uint32Array(1);
  crypto.getRandomValues(buffer);
  return buffer[0];
}

/** A range input hands back a string, and a slider spec is the only
 *  authority on what that string is allowed to mean. Total on purpose:
 *  anything unreadable falls back to the default for that dial rather
 *  than propagating a NaN into the scene graph. */
export function readSlider(spec: SliderSpec, raw: string): number {
  const value = Number.parseFloat(raw);
  if (!Number.isFinite(value)) return DEFAULT_PARAMS[spec.key];
  return Math.min(spec.max, Math.max(spec.min, value));
}
