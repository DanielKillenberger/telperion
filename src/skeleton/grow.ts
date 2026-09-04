import * as THREE from "three";

import { envelopeRadiusAt, sampleEnvelope, type Envelope } from "../envelope";
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
 * bare fork and the large one a solid mat. `step` is the one of them
 * that is a dial: the growth step as a fraction of height, which is
 * how finely the tree answers its attractors and so how far down it
 * branches. Kill distance follows it at a fixed ratio and the search
 * radius is derived from it, so one number moves the three together
 * and their ratios stay the art direction this file states. Every
 * distance is still an argument in metres through `growth`, which
 * overrides any of them individually for a caller with distances of
 * its own.
 * ------------------------------------------------------------------ */

export interface SkeletonParams {
  /** uint32. Varies the detail; it does not gamble on the outcome. */
  seed: number;
  /** The authored silhouette the branching has to fill. */
  envelope: Envelope;
  /** How many attractors the envelope gets - branch count, not
   *  leaves. More is a denser, finer tree, not a bigger one. */
  attractors: number;
  /** The growth step as a fraction of envelope height: the branching
   *  depth dial. Attractors decide where the tree is asked to grow and
   *  the step decides how finely it answers, so a finer step is a tree
   *  that keeps forking further down - finer wood, more tips - and not
   *  a smaller one. Kill distance moves with it at two steps; the
   *  search radius is derived from it by `influenceRadiusFor` rather
   *  than scaled with it, so shrinking the step cannot blind the
   *  search.
   *
   *  Defaults to `DEFAULT_STEP`, 0.022, which every tree before this
   *  dial existed was grown at; the useful rail runs down to about
   *  0.003, where Telperion is 14,100 nodes on 1,200 tips against 808
   *  on 133 at the default, and Laurelin's widest panel setting is
   *  42,000 nodes. Non-finite falls back to the default. The node
   *  ceiling scales with it - see `defaultGrowth`. */
  step?: number;
  /** Overrides for any of the growth bias field's five terms; the rest
   *  come from `DEFAULT_BIAS`. This is the shape of the preset fn-11.7
   *  authors Telperion and Laurelin as. */
  bias?: Partial<BiasParams>;
  /** Overrides for any growth distance, in metres; the rest come from
   *  `defaultGrowth(envelope, attractors, step)`. A `bias` given here
   *  wins over the field built from `SkeletonParams.bias`, which is
   *  the escape hatch for a caller with a field of its own. */
  growth?: Partial<GrowthConfig>;
}

/** The default growth step as a fraction of envelope height. About two
 *  per cent is fine enough that a limb reads as a curve, and it is the
 *  step every tree was grown at before the step was a dial, so a tree
 *  at this value is the tree it always was. */
export const DEFAULT_STEP = 0.022;
/** Kill distance in steps. Two stops a branch orbiting an attractor it
 *  has already arrived at. */
const KILL_STEPS = 2;
/** Search radius in steps: wide enough for long sweeping limbs. This
 *  is the art direction every tree grown so far was grown under, and
 *  the spacing floor below is added under it, never in place of it. */
const INFLUENCE_STEPS = 9;
/** Search radius floor, in attractor spacings. See `influenceRadiusFor`
 *  for what this number is and why it is not larger. */
const INFLUENCE_SPACINGS = 1.2;
/** The node ceiling at the default step. A stop, not a target: a tree
 *  that wants more nodes than this at today's step has been asked for
 *  something the panel should not be asking for. It is stated at the
 *  default step and scaled by the step's inverse in `defaultGrowth`,
 *  because the nodes a crown takes go roughly as one over the step -
 *  measured, each halving costs 1.7 to 2.2 times the nodes - so a
 *  fixed ceiling would either truncate the fine end of the rail or be
 *  no stop at all at the coarse end. */
const NODE_BUDGET = 8000;
/** Midpoint-rule samples for the crown volume. The profile is smooth
 *  between its ends and the volume only sets a floor, so this is far
 *  more than the derivation needs; it is fixed so that the same
 *  envelope always integrates to the same number. */
const VOLUME_SAMPLES = 256;

/** The crown's volume in cubic metres, integrated from the same
 *  profile that scatters the attractors. Not a bounding box: the
 *  profile is the whole point of the envelope, and a spire and a dome
 *  of the same height and spread hold very different amounts of air. */
function crownVolume(envelope: Envelope): number {
  const base = envelope.height * envelope.crownBase;
  const span = envelope.height - base;
  if (!(span > 0)) return 0;
  let sum = 0;
  for (let i = 0; i < VOLUME_SAMPLES; i += 1) {
    const y = base + ((i + 0.5) / VOLUME_SAMPLES) * span;
    const radius = envelopeRadiusAt(envelope, y);
    sum += radius * radius;
  }
  return Math.PI * sum * (span / VOLUME_SAMPLES);
}

/**
 * How far a node reaches for attractors, in metres, for a growth step
 * of `stepDistance` into a crown scattered with `attractors` points.
 *
 * Nine steps, as it has always been - and never less than 1.2 attractor
 * spacings, where the spacing is the cube root of the crown's volume
 * per attractor: the distance to the next one, in expectation, when
 * they are scattered uniformly through it.
 *
 * The floor exists because a radius tied to the step alone is blind to
 * how far apart the attractors are, and a finer step shrinks the search
 * until the next attractor is out of reach. That is not a sparser tree,
 * it is no tree: a tip that reaches its attractor and sees no other
 * stops, and once no tip sees anything colonization ends with the crown
 * untouched. Measured on Telperion at a 0.44 m step with 1,600
 * attractors, nine steps is 3.96 m against a 4.96 m spacing, and the
 * tree is 169 nodes, three tips, and 0.2% of its attractors reached,
 * all of it within five metres of the crown base. Every step below
 * today's is that regime, which is why the floor has to hold before a
 * depth dial can exist.
 *
 * Why 1.2 and not more. A sphere of 1.2 spacings holds about seven
 * attractors in expectation, and the forward half of it - the half a
 * tip can still turn toward - holds three or four; the odds that an
 * exhausted tip early in growth sees nothing ahead are a few per cent
 * per event, and a floor with real margin against that would sit
 * nearer two spacings. It does not, because the floor must not move a
 * single tree grown at today's step: the search radius has to resolve
 * to nine steps exactly at every configuration this library's tests
 * and presets state, so that any later change is attributable to a
 * dial someone moved. The sparsest of those - a 50 m envelope 140 m
 * wide with 900 attractors - has nine steps at 1.229 spacings, and the
 * floor stops just under it. The presets sit far above: Telperion's
 * nine steps are 5.9 spacings, Laurelin's 2.3.
 *
 * Both are fractions of height for a fixed attractor count - the
 * volume goes as the cube of height and the spacing as its cube root -
 * so the radius still scales with the tree, and a sapling and a
 * landmark with the same count branch the same way.
 */
export function influenceRadiusFor(
  envelope: Envelope,
  stepDistance: number,
  attractors: number,
): number {
  const bySteps = stepDistance * INFLUENCE_STEPS;
  const volume = crownVolume(envelope);
  if (!(attractors > 0) || !(volume > 0)) return bySteps;
  const spacing = Math.cbrt(volume / attractors);
  return Math.max(bySteps, spacing * INFLUENCE_SPACINGS);
}

/** NaN is the one value `Math.max` and `Math.min` pass through rather
 *  than pin, and a NaN step is a tree that never grows. */
const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

/** Growth distances for `envelope`, all proportional to its height: a
 *  step of `step` times it, a kill distance of two steps, a search
 *  radius of nine steps or 1.2 attractor spacings, whichever is wider -
 *  `influenceRadiusFor` carries the derivation and the reason for it -
 *  and a node ceiling of `NODE_BUDGET` at the default step, growing as
 *  the step shrinks so that the whole rail fits under it.
 *
 *  `attractors` is how many the envelope is scattered with, which the
 *  spacing floor is derived from. Left out, the radius is nine steps
 *  alone, which is exact at today's step and the collapse below it;
 *  `growSkeleton` always passes the count it scattered. `step` is the
 *  branching depth as `SkeletonParams.step` states it, held to
 *  `DEFAULT_STEP` when it is not a number. */
export function defaultGrowth(
  envelope: Envelope,
  attractors = 0,
  step = DEFAULT_STEP,
): GrowthConfig {
  const fraction = held(step, DEFAULT_STEP);
  const stepDistance = envelope.height * fraction;
  return {
    stepDistance,
    killDistance: stepDistance * KILL_STEPS,
    influenceRadius: influenceRadiusFor(envelope, stepDistance, attractors),
    // The envelope's own bare-trunk height: below it the silhouette
    // has no width, so nothing may branch there.
    trunkHeight: envelope.height * envelope.crownBase,
    maxNodes: Math.round(NODE_BUDGET * (DEFAULT_STEP / fraction)),
  };
}

/** The growth configuration `growSkeleton` runs `params` under: the
 *  derived distances, the bias field built from `params.bias`, and
 *  then `params.growth` over both. Exported so a caller can read the
 *  ceiling a skeleton was grown under and say whether it was reached,
 *  rather than re-deriving the merge and drifting from it.
 *
 *  `scattered` is the attractor count the spacing floor is derived
 *  from; `growSkeleton` passes the count it actually placed, which is
 *  `params.attractors` except in an envelope with no volume. */
export function resolveGrowth(
  params: SkeletonParams,
  scattered = params.attractors,
): GrowthConfig {
  const bias = createGrowthBias(params.envelope, params.seed, {
    ...DEFAULT_BIAS,
    ...params.bias,
  });
  return {
    ...defaultGrowth(params.envelope, scattered, params.step),
    bias,
    ...params.growth,
  };
}

/** Grows one skeleton. Deterministic in `params`. */
export function growSkeleton(params: SkeletonParams): Skeleton {
  const rng = createRng(params.seed);
  const attractors = sampleEnvelope(params.envelope, params.attractors, rng);
  return colonize(
    attractors,
    new THREE.Vector3(0, 0, 0),
    resolveGrowth(params, attractors.length),
  );
}
