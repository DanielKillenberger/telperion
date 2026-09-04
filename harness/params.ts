/* ------------------------------------------------------------------ *
 * THE DIALS THE OWNER TURNS
 *
 * The harness's half of the generator contract. The panel renders
 * whatever is in SLIDERS and hands the generator a GrowerParams; it
 * knows nothing else about either. When fn-11.2 onwards replace the
 * placeholder with the real space-colonization grower, this file is
 * where a new dial is added and the panel picks it up for free.
 *
 * Every name here points at a mechanism the spec already committed to:
 * `height` and `spread` are the authored envelope, `torsion` is the
 * spiral/curl bias, `taper` feeds the radius solve, `density` is how
 * many attractors the envelope gets. Nothing is a knob invented for
 * the panel's sake.
 * ------------------------------------------------------------------ */

/** Seeds are unsigned 32-bit integers, and nothing else is a seed. */
export const SEED_MAX = 0xff_ff_ff_ff;

export interface GrowerParams {
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** Envelope height, in metres. The tree is judged at human scale. */
  height: number;
  /** Envelope half-width as a fraction of height. Low is Telperion-ish
   *  and upright, high is Laurelin-ish and domed. */
  spread: number;
  /** Spiral bias around the trunk axis, 0 (straight) to 1 (wrung out). */
  torsion: number;
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
  { key: "spread", label: "spread", min: 0.15, max: 1.6, step: 0.01, unit: "" },
  { key: "torsion", label: "torsion", min: 0, max: 1, step: 0.01, unit: "" },
  { key: "density", label: "density", min: 0, max: 1, step: 0.01, unit: "" },
  { key: "taper", label: "taper", min: 0.5, max: 1, step: 0.01, unit: "" },
];

export const DEFAULT_PARAMS: GrowerParams = {
  seed: 1,
  height: 24,
  spread: 0.75,
  torsion: 0.35,
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
