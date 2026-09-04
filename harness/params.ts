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
 * `taper` feeds the radius solve, `density` is how many attractors the
 * envelope gets. Nothing is a knob invented for the panel's sake.
 *
 * The bias dials are five and not one on purpose. A single "torsion"
 * number mixes qualities that are independent - a tree that leans is a
 * different tree from one that wanders, and amplitude and wavelength
 * are the difference between a slow S-curve and a corkscrew - so it
 * cannot be art-directed. Their names and units are the library's,
 * unchanged, so skeleton-view.ts hands them straight over and there is
 * no translation to drift.
 * ------------------------------------------------------------------ */

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
  /** How thickly the envelope is populated: branch count, not leaves. */
  density: number;
  /** Radius falloff through a fork, 0.5 (abrupt) to 1 (none). */
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
  { key: "gravitropism", label: "gravitropism", min: 0, max: 1.3, step: 0.01, unit: "" },
  { key: "lean", label: "lean", min: 0, max: 0.5, step: 0.01, unit: "" },
  { key: "writheAmplitude", label: "writhe", min: 0, max: 0.25, step: 0.01, unit: "" },
  { key: "writheWavelength", label: "bend length", min: 0.08, max: 1.2, step: 0.01, unit: "" },
  { key: "spiralRate", label: "spiral", min: 0, max: 6, step: 0.1, unit: "" },
  { key: "density", label: "density", min: 0, max: 1, step: 0.01, unit: "" },
  { key: "taper", label: "taper", min: 0.5, max: 1, step: 0.01, unit: "" },
];

export const DEFAULT_PARAMS: GrowerParams = {
  seed: 1,
  height: 24,
  spread: 0.3,
  gravitropism: DEFAULT_BIAS.gravitropism,
  lean: DEFAULT_BIAS.lean,
  writheAmplitude: DEFAULT_BIAS.writheAmplitude,
  writheWavelength: DEFAULT_BIAS.writheWavelength,
  spiralRate: DEFAULT_BIAS.spiralRate,
  density: 0.5,
  taper: 0.78,
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
