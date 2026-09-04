import * as THREE from "three";

import type { Envelope } from "@/lib/grower/envelope";
import { createNoise } from "@/lib/grower/noise";
import { createRng } from "@/lib/grower/rng";
import type { GrowthBias } from "@/lib/grower/skeleton/colonize";

/* ------------------------------------------------------------------ *
 * THE GROWTH BIAS FIELD
 *
 * Space colonization on its own grows toward attractors and believes
 * nothing else. That is why the skeleton it returns has no idea a tree
 * wants to grow up - measured on the fn-11.2 output at defaults, 22.2%
 * of every growth step went downward, branches wandering back down
 * through their own crown - and why its trunk was 14 nodes in a
 * mathematically straight line. Both are the same missing thing: the
 * algorithm had no opinion about direction, only about destination.
 *
 * This is that opinion. Every growth step in the generator, the trunk's
 * climb included, is handed to `createGrowthBias` and comes back bent.
 *
 * Five named terms, and they are named because they are independent and
 * an art director wants them independently. One lumped "torsion" number
 * cannot say both "leans hard but grows straight" and "stands upright
 * but writhes", and those are two different trees. fn-11.7 authors
 * Telperion and Laurelin as two `BiasParams`, so these five values are
 * exactly what will differ between them.
 *
 *   gravitropism    - the upward pull. Strongest low, weakest at the
 *                     tips, which is how a real tree carries it: the
 *                     leader is dead vertical and the outer twigs droop.
 *   lean            - a steady departure from vertical. The tree's mean
 *                     path is a straight leaning line, not the y axis,
 *                     and everything below is measured against it.
 *   writheAmplitude - how far the centreline strays from that mean
 *                     path, as a fraction of the envelope's height.
 *   writheWavelength- how long one bend is. Amplitude and wavelength
 *                     are the difference between one slow S-curve and a
 *                     corkscrew, and one number cannot express both.
 *   spiralRate      - turns about the mean path over the tree's full
 *                     height. This is the "wrung out" term.
 *
 * Everything is expressed as a fraction of envelope height or as turns
 * over it, so a 4 m tree and a 60 m tree writhe the same way at
 * different sizes - the same reason grow.ts derives its distances that
 * way.
 * ------------------------------------------------------------------ */

const TAU = Math.PI * 2;

export interface BiasParams {
  /** Upward pull on every growth step, 0 (none) to about 1 (the tree
   *  ignores its attractors and grows at the sky). Negative is not a
   *  tree and is clamped away. */
  gravitropism: number;
  /** Steady departure from vertical: horizontal metres per metre
   *  climbed, so 0.1 is a lean of about six degrees. The compass
   *  bearing it leans along comes from the seed. */
  lean: number;
  /** How far the centreline may stray from its mean path, as a
   *  fraction of the envelope's height. */
  writheAmplitude: number;
  /** The length of one bend, as a fraction of the envelope's height.
   *  Large is one slow sweep over the whole tree; small is a tighter
   *  wave. Not arbitrarily small: a bend the growth step cannot sample
   *  is raised to the field's own floor of `MIN_STEPS_PER_BEND` steps,
   *  because below that it is not a shorter bend, it is a sawtooth. */
  writheWavelength: number;
  /** Turns about the mean path over the envelope's full height. Held
   *  to the same sampling floor as `writheWavelength`: a spiral that
   *  turns faster than the growth can follow is a zigzag. */
  spiralRate: number;
}

/** The tree the rest are a departure from: upright, leaning barely at
 *  all, with a slow single sweep and one turn of spiral over its
 *  height. Not either of the Two Trees - fn-11.7 authors those. */
export const DEFAULT_BIAS: BiasParams = {
  gravitropism: 0.7,
  lean: 0.05,
  writheAmplitude: 0.07,
  writheWavelength: 0.45,
  spiralRate: 1.2,
};

/** A skeleton grown with this is exactly the fn-11.2 skeleton: every
 *  term off, no opinion about direction at all. It is what the
 *  downward-step baseline is measured against, so it has to stay a
 *  reachable configuration rather than a comment. */
export const NO_BIAS: BiasParams = {
  gravitropism: 0,
  lean: 0,
  writheAmplitude: 0,
  writheWavelength: DEFAULT_BIAS.writheWavelength,
  spiralRate: 0,
};

/** The share of `gravitropism` still acting at the crown tip. Not zero:
 *  most of the steps that dive back through the crown are up in the
 *  outer crown, and a tip with no upward pull at all is exactly where
 *  they come from - dropping this to 0.3 costs three points of downward
 *  steps on its own. Not one either: a crown whose tips pull up as hard
 *  as its trunk does closes into a fist. */
const GRAVITROPISM_TIP = 0.55;

/** Hard ceiling on the combined sideways bias, as a multiple of the
 *  unit growth direction. It is a stability rail, not art direction:
 *  below 1 the biased direction can never reverse the step it was given,
 *  which is what guarantees the trunk's climb keeps gaining height and
 *  so can neither stall against `maxNodes` nor turn back at the ground.
 *  A 0.9 rail still allows a 42-degree kink on a single step. */
const WRITHE_CEILING = 0.9;

/** The finest bend the field will hand out, in growth steps per full
 *  bend. A wave is only a wave if it is sampled often enough to be
 *  one: at the panel's shortest bend length the field was being read
 *  3.6 times per period, and a sinusoid sampled that coarsely is a
 *  sawtooth by construction - it moved the median turn angle between
 *  consecutive steps from 12 degrees to 30 and put 20 outright
 *  reversals into one crown.
 *
 *  Nyquist's floor is 2 samples per period, and that only guarantees
 *  the frequency survives; 8 is what a bend needs to read to the eye
 *  as a curve rather than as a kink. The clamp lives here rather than
 *  in the panel's slider range because the panel is not the only
 *  caller, and an aliased field should be reachable by none of them.
 *  It binds the spiral as well as the writhe: both are waves in
 *  height, and the same growth step samples both. */
export const MIN_STEPS_PER_BEND = 8;

/** How hard the centreline is pulled back onto its mean path once it
 *  has strayed the full amplitude. Above the sideways terms it fights
 *  (about 1.0 combined at the defaults), so stray settles below the
 *  budget instead of winding away from it. This is what turns the
 *  spiral into a bounded writhe rather than a helix that walks the
 *  trunk off its own base. */
const RESTORE_GAIN = 2;

/** Radius, as a fraction of the stray budget, over which the spiral
 *  term crosses from "push the axis sideways" to "swirl around it".
 *  Exactly on the mean path there is no direction to swirl about, and
 *  this is the blend that covers that. */
const SWIRL_BLEND = 0.5;

/** Where the mean path stops being the trunk's business. Above the
 *  crown base the attractors are what hold a branch in place and a pull
 *  back toward the trunk axis would just close the crown up, so the
 *  restoring term fades out across the bottom of the crown. */
const RESTORE_FADE = 1.3;

function smoothstep(edge0: number, edge1: number, x: number): number {
  if (edge1 <= edge0) return x < edge0 ? 0 : 1;
  const t = Math.min(1, Math.max(0, (x - edge0) / (edge1 - edge0)));
  return t * t * (3 - 2 * t);
}

/**
 * The bias field for one tree.
 *
 * Deterministic in `seed` and `envelope` and pure in its arguments, so
 * the skeleton grown through it is still a pure function of the
 * generator's inputs. The seed reaches two things and nothing else: the
 * bearing the tree leans along, and the noise field it wanders in.
 */
export function createGrowthBias(
  envelope: Envelope,
  seed: number,
  params: BiasParams,
): GrowthBias {
  // A stream of its own, from a seed derived from the caller's. Drawing
  // from the attractor sampler's stream would make the shape of the
  // crown depend on how many numbers the field happened to want.
  const rng = createRng((seed ^ 0x5b_f0_3d_11) >>> 0);
  const bearing = rng.next() * TAU;
  const phase = rng.next() * TAU;
  const noise = createNoise((seed ^ 0x1f_83_d9_ab) >>> 0);

  const height = Math.max(1e-6, envelope.height);
  const gravitropism = Math.max(0, params.gravitropism);
  const lean = Math.max(0, params.lean);
  const amplitude = Math.max(0, params.writheAmplitude);
  const askedWavelength = Math.max(1e-3, params.writheWavelength);
  const askedSpiralRate = Math.max(0, params.spiralRate);

  const leanDirection = new THREE.Vector3(
    Math.cos(bearing),
    0,
    Math.sin(bearing),
  );

  /* Both sideways gains are the slope a path needs to reach the stray
     budget at the bend length it was given: a centreline of amplitude A
     and wavelength L has a maximum slope of 2*pi*A/L, and a helix of
     radius A making `n` turns over height H has one of 2*pi*n*A/H. The
     numbers the panel hands over are therefore shape, and the strength
     that produces that shape is derived rather than dialled separately. */
  const strayLimit = amplitude * height;
  let wavelength = askedWavelength;
  let spiralRate = askedSpiralRate;
  let curlLength = wavelength * height;
  let curlGain = (TAU * amplitude) / wavelength;
  let spiralGain = TAU * spiralRate * amplitude;

  /* Retuned to the step the caller is actually taking, so that neither
     wave is asked for a bend the growth is too coarse to draw. Both
     periods are lengths up the tree - `wavelength` as a fraction of
     height, one turn of spiral as height/spiralRate - so both floors
     fall straight out of MIN_STEPS_PER_BEND. Growth takes one step
     length for a whole run, so this recomputes once and then does
     nothing. */
  let sampledStep = Number.NaN;
  const retune = (step: number): void => {
    if (step === sampledStep || !(step > 0)) return;
    sampledStep = step;
    wavelength = Math.max(askedWavelength, (MIN_STEPS_PER_BEND * step) / height);
    const maxTurns = height / (MIN_STEPS_PER_BEND * step);
    spiralRate = Math.min(askedSpiralRate, maxTurns);
    curlLength = wavelength * height;
    curlGain = (TAU * amplitude) / wavelength;
    spiralGain = TAU * spiralRate * amplitude;
  };

  const crownBase = height * envelope.crownBase;

  // Scratch, reused every call: this runs once per node grown.
  const stray = new THREE.Vector3();
  const swirl = new THREE.Vector3();
  const tangent = new THREE.Vector3();
  const helix = new THREE.Vector3();
  const writhe = new THREE.Vector3();
  const curl = new THREE.Vector3();
  const biased = new THREE.Vector3();

  return (
    position: THREE.Vector3,
    direction: THREE.Vector3,
    stepDistance: number,
  ): THREE.Vector3 => {
    retune(stepDistance);
    const t = Math.min(1, Math.max(0, position.y / height));

    // Where the mean path is at this height, and how far off it we are.
    const meanX = leanDirection.x * lean * position.y;
    const meanZ = leanDirection.z * lean * position.y;
    stray.set(position.x - meanX, 0, position.z - meanZ);
    const strayed = stray.length();

    writhe.set(0, 0, 0);

    // Lean: match the slope of the mean path, everywhere in the tree.
    writhe.addScaledVector(leanDirection, lean);

    // Spiral about the mean path. On the path itself there is no
    // direction to swirl around, so the term starts as a horizontal
    // vector that rotates with height - which is what puts a corkscrew
    // into a trunk that has nothing else to bend it - and crosses over
    // to a true tangential swirl as the branch moves outward.
    if (spiralGain > 0) {
      const theta = TAU * (spiralRate * t) + phase;
      helix.set(Math.cos(theta), 0, Math.sin(theta));
      if (strayed > 1e-9) {
        tangent.set(-stray.z, 0, stray.x).divideScalar(strayed);
        const blend = smoothstep(0, SWIRL_BLEND * strayLimit, strayed);
        swirl.lerpVectors(helix, tangent, blend);
        // Anti-parallel halfway through the blend cancels exactly; the
        // tangential end is the one that means something off the axis.
        if (swirl.length() > 1e-6) swirl.normalize();
        else swirl.copy(tangent);
      } else {
        swirl.copy(helix);
      }
      writhe.addScaledVector(swirl, spiralGain);
    }

    // Curl noise: the structured wander. Divergence-free, so branches
    // sweep past one another instead of piling into the same knot.
    if (curlGain > 0) {
      noise.curl(position, curlLength, curl);
      const length = curl.length();
      if (length > 1e-9) writhe.addScaledVector(curl, curlGain / length);
    }

    // Restore. The trunk has no attractors to hold it, so without this
    // the spiral would wind it off its own base; in the crown the
    // attractors do the holding and a pull toward the axis would only
    // close the crown up, so it fades out across the bottom of the crown.
    if (strayLimit > 0 && strayed > 1e-9) {
      const trunkness =
        1 - smoothstep(crownBase, crownBase * RESTORE_FADE, position.y);
      if (trunkness > 0) {
        const over = Math.min(1, strayed / strayLimit);
        writhe.addScaledVector(
          stray,
          (-RESTORE_GAIN * over * over * trunkness) / strayed,
        );
      }
    }

    writhe.clampLength(0, WRITHE_CEILING);

    // Gravitropism last and outside the ceiling: it is the one term that
    // may never be traded away, and it only ever adds height.
    const lift =
      gravitropism * (GRAVITROPISM_TIP + (1 - GRAVITROPISM_TIP) * (1 - t));

    biased.copy(direction).add(writhe);
    biased.y += lift;

    // The ceiling makes this unreachable from a unit `direction`, but
    // the contract is "returns a unit vector" and a NaN escaping into
    // the skeleton would be silent. Cheap, once per node.
    const length = biased.length();
    if (length < 1e-9) return direction.clone().normalize();
    return biased.clone().divideScalar(length);
  };
}
