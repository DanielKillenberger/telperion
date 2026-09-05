import * as THREE from "three";

import { envelopeRadiusAt, sampleEnvelope, type Envelope } from "../envelope";
import { solveRadii, DEFAULT_RADII, type RadiusParams, type RadiusField } from "../radius";
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
import { branchLength, childRadius, internodeLength } from "./law";
import { shedTwigs } from "./shed";
import { branchTwigs, resolveTwigs, MAX_TWIG_LEVELS, type TwigParams, type TwiggedSkeleton } from "./twigs";

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
 *
 * Colonization runs first, its radius field is solved under the caller's
 * thickness parameters, then branchTwigs continues its tips under the
 * branch law. Fixed terminal twigs end the recursion. The shell rule
 * sheds interior subtrees before later stages read the geometry and
 * the pass's records.
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
  /** Branch anatomy and allometry below the crossover. Unstated terms
   * use DEFAULT_TWIGS; generation count follows the solved radius. */
  twigs?: Partial<TwigParams>;
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
/** Search radius floor, in attractor spacings. Measured, not chosen:
 *  1.2 was the largest floor that left every test fixture byte-identical
 *  and it still starved 3 of 108 seed-by-cell draws, because colonize
 *  ends the tree the moment no tip sees an attractor while living ones
 *  remain. 2.0 starves 0 of 297 with per-event odds near 1e-7. The
 *  trees it moves are two off-panel fixtures and Laurelin dialled to the
 *  panel's sparsest density; both presets as shipped sit above it, at
 *  5.9 and 2.3 spacings, so nine steps still binds for them. See
 *  `influenceRadiusFor`. */
const INFLUENCE_SPACINGS = 2.0;
/** Hard safety ceiling shared by colonization and branch growth. */
const NODE_CEILING = 250_000;

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
 * Nine steps, as it has always been - and never less than 2.0 attractor
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
 * Why 2.0, and why it was 1.2 first. A sphere of 1.2 spacings holds
 * about seven attractors in expectation, and the forward half of it -
 * the half a tip can still turn toward - holds three or four; the odds
 * that an exhausted tip early in growth sees nothing ahead are a few
 * per cent per event, and measured over 108 seed-by-cell draws on the
 * panel's range, 1.2 starved three of them. 1.2 was chosen anyway at
 * first because it was the largest floor that left every test fixture
 * byte-identical: the sparsest, a 50 m envelope 140 m wide with 900
 * attractors, has nine steps at 1.229 spacings. That was the wrong
 * thing to hold. The byte-identity that matters is the two presets as
 * shipped, and they sit far above the floor - Telperion's nine steps
 * are 5.9 spacings, Laurelin's 2.3 - so the floor was raised to 2.0,
 * where 297 draws starve none and the per-event odds are near 1e-7.
 * The two fixtures it moves, and Laurelin dialled to the panel's
 * sparsest density, are stated in the tests rather than left to drift.
 * The mechanism fix - a tip with nothing in reach keeping on while
 * living attractors remain - was measured and rejected: Laurelin's
 * shipped tree ends through exactly that branch and would change.
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
 *  radius of nine steps or 2.0 attractor spacings, whichever is wider -
 *  `influenceRadiusFor` carries the derivation and the reason for it -
 *  and the shared `NODE_CEILING` for the two growth passes.
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
  _twigs?: Partial<TwigParams>,
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
    maxNodes: NODE_CEILING,
  };
}

/** N = branch internodes + laterals * N_child + one twig edge.
 * The estimate uses the same radius and length at every handoff as the
 * pass, before collision and trunk rejection. Resolution adds nodes
 * along existing branches without multiplying their offspring. */
function twigHeadroom(skeleton: Skeleton, field: RadiusField, config: GrowthConfig, twigs: TwigParams): number {
  const twigNodes = 1;
  const lowRatio = Math.max(0.05, twigs.lengthRatio * (1 - twigs.vigourVariation));
  const highRatio = Math.min(1, twigs.lengthRatio * (1 + twigs.vigourVariation));
  const nodesFor = (radius: number, length: number, generation = 0, minimumRadius = radius): number => {
    if (radius <= twigs.twig.diameter / 2 || length < twigs.twig.internodeLength) return twigNodes;
    if (generation >= MAX_TWIG_LEVELS) return 0;
    // The last order: a twig at every station, no lateral branches, and
    // stations no closer than a twig's length.
    if (radius <= twigs.twig.bearingDiameter / 2) {
      const stations = Math.max(1, Math.round(length /
        internodeLength(minimumRadius, length, twigs.internodeFactor, twigs.twig.length)));
      return Math.min(NODE_CEILING, stations + stations);
    }
    const internodes = Math.max(1, Math.round(length /
      internodeLength(minimumRadius, length, twigs.internodeFactor, twigs.twig.internodeLength)));
    const offspring = twigs.laterals === 0 ? 0 : twigs.laterals * nodesFor(
      childRadius(radius, highRatio, twigs.ratioPower), length * highRatio, generation + 1,
      childRadius(minimumRadius, lowRatio, twigs.ratioPower));
    return Math.min(NODE_CEILING, internodes + offspring + twigNodes);
  };
  const children = new Int32Array(skeleton.nodes.length);
  for (let i = 1; i < children.length; i++) children[skeleton.nodes[i].parent]++;
  let estimate = 0;
  for (let i = 1; i < children.length && estimate < NODE_CEILING; i++) {
    if (skeleton.nodes[i].position.y < config.trunkHeight) continue;
    const radius = field.radius[i];
    const length = branchLength(radius);
    if (children[i] === 0) estimate += nodesFor(radius, length);
    if (radius < twigs.limbRadius * field.radius[0]) {
      estimate += twigs.laterals * nodesFor(
        childRadius(radius, highRatio, twigs.ratioPower), length * highRatio, 1,
        childRadius(radius, lowRatio, twigs.ratioPower));
    }
  }
  return Math.min(NODE_CEILING, estimate);
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
    ...defaultGrowth(params.envelope, scattered, params.step, params.twigs),
    bias,
    ...params.growth,
  };
}

/** One skeleton and what growing it cost in nodes: whether growth
 *  stopped at its ceiling rather than finishing, and how many twigs
 *  the shell rule shed. Both are facts the finished skeleton cannot
 *  carry - a shed tree is smaller than the ceiling it hit - and a
 *  caller that reports the build reads them here rather than
 *  re-deriving them wrong. */
export interface GrowthReport {
  /** Carries `crossover`, the index where the twig pass began, because
   *  the thickness solve keys on it: a consumer that rebuilt a plain
   *  `{ nodes }` would have every twig solved as a limb, silently. */
  skeleton: TwiggedSkeleton;
  /** Whether the node ceiling stopped growth before the crown, or the
   *  twig pass, was finished. A capped tree is the ceiling's shape and
   *  not the envelope's, and it is reported rather than truncated
   *  silently. */
  capped: boolean;
  /** Whether branch growth reached its generation safety cap. */
  levelCapped: boolean;
  /** Twig nodes removed by the shell rule. */
  shed: number;
}

/** Grows one skeleton and reports the growth: colonization, then the
 *  twigs from its tips, then the shell rule over the twigs.
 *  Deterministic in `params` and `radii`. */
export function growReport(params: SkeletonParams, radii: RadiusParams = DEFAULT_RADII): GrowthReport {
  const rng = createRng(params.seed);
  const attractors = sampleEnvelope(params.envelope, params.attractors, rng);
  const config = resolveGrowth(params, attractors.length);
  const colonized = colonize(attractors, new THREE.Vector3(0, 0, 0), config);
  const field = solveRadii(colonized, params.envelope, radii);
  const twigs = resolveTwigs(params.twigs);
  const maxNodes = Math.min(config.maxNodes, NODE_CEILING,
    colonized.nodes.length + twigHeadroom(colonized, field, config, twigs));
  const twigged = branchTwigs(colonized, field, { ...config, maxNodes }, twigs, params.seed);
  const skeleton = shedTwigs(twigged, colonized.nodes.length, params.envelope);
  return {
    skeleton,
    capped: colonized.nodes.length >= config.maxNodes || twigged.nodeCapped,
    levelCapped: twigged.levelCapped,
    shed: twigged.nodes.length - skeleton.nodes.length,
  };
}

/** Grows one skeleton: colonization, the twigs from its tips, and the
 *  shell rule over the twigs. Deterministic in `params` and `radii`. */
export function growSkeleton(params: SkeletonParams, radii: RadiusParams = DEFAULT_RADII): TwiggedSkeleton {
  return growReport(params, radii).skeleton;
}
