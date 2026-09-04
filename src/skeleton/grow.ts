import * as THREE from "three";

import { sampleEnvelope, type Envelope } from "../envelope";
import { createRng } from "../rng";
import {
  colonize,
  type GrowthConfig,
  type Skeleton,
} from "./colonize";
import {
  createGrowthBias,
  DEFAULT_BIAS,
  type BiasParams,
} from "../torsion";

/* ------------------------------------------------------------------ *
 * THE GENERATOR'S FRONT DOOR
 *
 * Seed and envelope in, skeleton out, and nothing else in the loop.
 * The same arguments return the same nodes in the same order on any
 * machine: the only source of chance is `createRng(seed)`, and
 * colonization downstream of it is deterministic.
 *
 * Direction is the other half of the arguments. `bias` is the growth
 * bias field - gravitropism, lean, writhe and spiral - and it defaults
 * to something on rather than to nothing, because a tree with no
 * opinion about direction is the fn-11.2 output: 22.2% of its growth
 * steps went downward and its trunk was a straight line. `NO_BIAS` is
 * the way back to that, and it is what the baseline is measured with.
 *
 * The growth distances default to fractions of the envelope's height
 * rather than to absolute metres, so a 4 m tree and a 60 m tree get
 * the same branching character instead of the small one coming out a
 * bare fork and the large one a solid mat. Every one of them is still
 * an argument: `growth` overrides any of them individually, which is
 * how kill distance and influence radius are exposed for art
 * direction without adding a dial nobody has asked for yet.
 * ------------------------------------------------------------------ */

export interface SkeletonParams {
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** The authored silhouette the branching has to fill. */
  envelope: Envelope;
  /** How many attractors the envelope gets - branch count, not
   *  leaves. More is a denser, finer tree, not a bigger one. */
  attractors: number;
  /** Overrides for any of the growth bias field's five terms; the rest
   *  come from `DEFAULT_BIAS`. This is the shape of the preset fn-11.7
   *  authors Telperion and Laurelin as. */
  bias?: Partial<BiasParams>;
  /** Overrides for any growth distance; the rest come from
   *  `defaultGrowth(envelope)`. A `bias` given here wins over the field
   *  built from `SkeletonParams.bias`, which is the escape hatch for a
   *  caller with a field of its own. */
  growth?: Partial<GrowthConfig>;
}

/** Growth distances for `envelope`, all but `maxNodes` proportional to
 *  its height. The ratios are the art direction: a step of about two
 *  per cent of height is fine enough that a limb reads as a curve, a
 *  kill distance of two steps stops a branch orbiting an attractor it
 *  has already arrived at, and an influence radius of nine steps is
 *  wide enough for long sweeping limbs and narrow enough that the
 *  crown does not collapse to a single mast. */
export function defaultGrowth(envelope: Envelope): GrowthConfig {
  const step = envelope.height * 0.022;
  return {
    stepDistance: step,
    killDistance: step * 2,
    influenceRadius: step * 9,
    // The envelope's own bare-trunk height: below it the silhouette
    // has no width, so nothing may branch there.
    trunkHeight: envelope.height * envelope.crownBase,
    // A stop, not a target: a tree that wants more nodes than this has
    // been asked for something the panel should not be asking for.
    maxNodes: 8000,
  };
}

/** Grows one skeleton. Deterministic in `params`. */
export function growSkeleton(params: SkeletonParams): Skeleton {
  const rng = createRng(params.seed);
  const attractors = sampleEnvelope(params.envelope, params.attractors, rng);
  const bias = createGrowthBias(params.envelope, params.seed, {
    ...DEFAULT_BIAS,
    ...params.bias,
  });
  const growth = { ...defaultGrowth(params.envelope), bias, ...params.growth };
  return colonize(attractors, new THREE.Vector3(0, 0, 0), growth);
}
