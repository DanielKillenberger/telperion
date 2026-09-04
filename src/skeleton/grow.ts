import * as THREE from "three";

import { sampleEnvelope, type Envelope } from "../envelope";
import { createRng } from "../rng";
import { colonize, type GrowthConfig, type Skeleton } from "./colonize";

/* ------------------------------------------------------------------ *
 * THE GENERATOR'S FRONT DOOR
 *
 * Seed and envelope in, skeleton out, and nothing else in the loop.
 * The same arguments return the same nodes in the same order on any
 * machine: the only source of chance is `createRng(seed)`, and
 * colonization downstream of it is deterministic.
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
  /** Overrides for any growth distance; the rest come from
   *  `defaultGrowth(envelope)`. */
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
  const growth = { ...defaultGrowth(params.envelope), ...params.growth };
  return colonize(attractors, new THREE.Vector3(0, 0, 0), growth);
}
