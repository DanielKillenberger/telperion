import { DEFAULT_ENVELOPE, type Envelope } from "./envelope";
import type { Skeleton } from "./skeleton/colonize";
import type { TwiggedSkeleton } from "./skeleton/twigs";

/* ------------------------------------------------------------------ *
 * THE RADIUS SOLVE
 *
 * Colonization says where every limb goes. The branch pass records
 * the thickness its law assigned. Drawn as lines that reads as wire, not as a tree - a
 * trunk and a twig are the same one pixel - and the thickness
 * hierarchy is the single largest thing standing between the two.
 *
 * Above the crossover, a fork's parent cross-sectional area equals
 * the sum of its children's.
 * That is da Vinci's observation, and it generalises to
 *
 *     r_parent^n = sum over children of r_child^n
 *
 * with n = 2 the area statement itself. It is worth doing properly
 * because a wrong junction is the thing the eye picks out of CG trees
 * instantly: branches that keep the parent's thickness read as pipes
 * welded on, and branches that shed too much read as string. Real
 * wood conserves the plumbing it has to carry.
 *
 * Three named terms, because thickness is three independent qualities
 * and an art director wants them independently. fn-11.7 authors
 * Telperion and Laurelin as two `RadiusParams`, so anything constant
 * in here is a difference between the two trees nobody can author.
 *
 *   trunkRadius  - how stout the tree is at the ground. The whole
 *                  field is normalised to it, so it is stated
 *                  directly rather than emerging from the branch
 *                  count.
 *   forkExponent - the n above: how much thickness a fork sheds, and
 *                  therefore the contrast between trunk and twig.
 *   lengthTaper  - how much a limb thins along its own length between
 *                  forks. Without it an unbranched run is a cylinder,
 *                  and the bare trunk below the crown is the longest
 *                  unbranched run in the tree.
 *
 * The solve runs from the tips down. Every tip is given the same
 * relative thickness, the rule accumulates down to the root, and the
 * whole field is then scaled so the root is exactly `trunkRadius`.
 * The recurrence is homogeneous of degree one - scaling every radius
 * by the same factor is still a solution - so that normalisation is
 * exact and free, and it buys the property that makes the exponent a
 * usable dial: moving it changes the trunk-to-twig contrast and
 * leaves the tree's own girth where the owner put it. Solving from a
 * fixed twig upward instead couples the two, and the tree changes
 * size when you ask it to change taper.
 *
 * The trunk radius is a fraction of envelope height. The branch pass
 * receives the solved field in metres; its fixed twig anatomy never
 * scales with the envelope.
 *
 * BELOW THE CROSSOVER THE PASS OWNS THE BRANCH LAW. Its records name
 * each branch, its assigned base radius and each internode's distal
 * radius on the local profile. A branch starts at its allocation;
 * continuing internodes start at their parent's solved radius. The
 * local pass tapers over the actual branch run, independently of the
 * envelope-height-normalized taper retained above the crossover.
 * Twigs keep their stated diameter at both ends (tip fraction 1):
 * neither envelope height nor internode length rescales their anatomy.
 * The fork solve above the crossover never reads appended children.
 * ------------------------------------------------------------------ */

export interface RadiusParams {
  /** Radius at the base of the trunk, as a fraction of envelope
   *  height. An ordinary 24 m tree is about 0.02 - a metre through at
   *  the foot. The Two Trees references sit nearer 0.04, which beside
   *  the harness's 1.8 m figure is the difference between a tree and
   *  a monument. */
  trunkRadius: number;
  /** The exponent of the fork rule. 2 is cross-sectional area
   *  conserved exactly. Below it a fork sheds more than area and the
   *  tree runs from a heavy trunk to fine twigs; above it a fork
   *  sheds less and the limbs stay stout further out. Measured real
   *  trees sit between 2 and 3. Clamped at 1, below which a parent
   *  would come out thicker than all its children laid side by side. */
  forkExponent: number;
  /** How fast a limb thins where nothing branches off it, in
   *  e-foldings per envelope height: at 1 a limb running the tree's
   *  whole height without a fork would be e times thicker at its base
   *  than at its end. It is stated as a rate over distance rather
   *  than per node, so the answer does not change when the skeleton
   *  is grown at a finer step. */
  lengthTaper: number;
}

/** The tree the rest are a departure from: an ordinary trunk, forks
 *  that conserve area exactly, and a limb that loses about a fifth of
 *  its thickness over a third of the tree's height. Not either of the
 *  Two Trees - fn-11.7 authors those. */
export const DEFAULT_RADII: RadiusParams = {
  trunkRadius: 0.02,
  forkExponent: 2,
  lengthTaper: 0.6,
};

/** Floor under the trunk radius, as a fraction of envelope height. A
 *  stability rail, not art direction: the whole field is a positive
 *  multiple of this one number, so a floor above zero is what makes
 *  "no zero or negative radii" true by construction rather than by
 *  inspection of the caller. A tenth of a millimetre on a 24 m tree. */
const MIN_TRUNK_RADIUS = 4e-6;

/** Floor under the fork exponent. Also a rail: at n below 1 the rule
 *  hands a parent more girth than all its children put together,
 *  which is not a taper of any kind. n = 1 is the thickest defensible
 *  reading - radius, rather than area, conserved through the fork. */
const MIN_FORK_EXPONENT = 1;

/** Ceiling on the fork exponent. Above about 8 a fork is arithmetically
 *  indistinguishable from "the parent is exactly its thickest child",
 *  so there is no look above it to reach - and an unbounded exponent
 *  is an unbounded power of the relative field, which is how this
 *  overflows to Infinity. */
const MAX_FORK_EXPONENT = 8;

/** The most thickness the length taper may shed between the root and
 *  any one node, in e-foldings. A rail, not art direction: a twig at
 *  e^-12 of the trunk is three microns on a metre-thick trunk, so
 *  nothing anyone could want to look at reaches it - and without a
 *  bound, a skeleton handed an envelope it was not grown in (a
 *  degenerate height, say) drives `exp` straight to Infinity and the
 *  whole field comes back NaN. The cap is on the accumulated shed
 *  from the root rather than on one step, so it is still a function
 *  of distance alone and subdividing an edge cannot change it. */
const MAX_TAPER_SHED = 12;

/** Radii for one skeleton, in metres, indexed by node.
 *
 *  Two numbers per node because a branch has two ends and the fork
 *  rule is stated at the junction, not at the far end of the limb.
 *  A surface that used `radius` alone would step outward at every
 *  node where a limb tapers, and a fork test written on it would be
 *  testing the taper as well as the rule. */
export interface RadiusField {
  /** Radius at the node's own position. */
  radius: Float64Array;
  /** Radius where this node's branch leaves its parent. Equal to
   *  `radius` at the root, which has no branch behind it, and always
   *  at least `radius[i]` elsewhere - the difference is the length
   *  taper over that one step. */
  startRadius: Float64Array;
}

/**
 * Solves the radius field for `skeleton` inside `envelope`.
 *
 * Pure and deterministic: no seed reaches this, and the same skeleton
 * and parameters give the same radii on any machine.
 *
 * Guarantees, all of them structural rather than tuned:
 *   - every radius is finite and strictly positive;
 *   - `startRadius[i] >= radius[i]` on every edge; branch internodes
 *     taper with distance, while a twig keeps its fixed anatomy;
 *   - at every node above the crossover, `radius^forkExponent` is the
 *     sum of its children's `startRadius^forkExponent`, to
 *     floating-point;
 *   - the field above the crossover is byte for byte the field of the
 *     same skeleton cut off there, so twig orders never move a limb.
 *
 * `skeleton.crossover`, when the twig pass set it, is the index of the
 * first node that pass appended; a skeleton without one has no twigs
 * and the whole of it is solved by the fork rule.
 *
 * Relies on the skeleton invariant that a node's parent index is
 * lower than its own, which lets one backward pass see every child
 * before its parent, and one forward pass every parent before its
 * children.
 */
export function solveRadii(
  skeleton: Skeleton | TwiggedSkeleton,
  envelope: Envelope,
  params: RadiusParams,
): RadiusField {
  const nodes = skeleton.nodes;
  const count = nodes.length;
  const radius = new Float64Array(count);
  const startRadius = new Float64Array(count);
  if (count === 0) return { radius, startRadius };

  /* Where the fork rule stops and the fine orders' law begins. Not a
     number, or past the end, is a skeleton with no twigs; below one
     would make the root a twig with no parent to take its radius
     from, and a root is never appended. Truncated, because a fraction
     of a node is not a place. */
  const asked = "crossover" in skeleton ? skeleton.crossover : count;
  const crossover = Number.isFinite(asked)
    ? Math.min(count, Math.max(1, Math.trunc(asked)))
    : count;

  /* Every number that reaches the arithmetic is pinned to its own
     range first. NaN is the one that has to be named separately -
     `Math.max` propagates it rather than clamping it - and a NaN
     radius is the kind of thing that reaches the screen as an
     invisible branch rather than as an error. */
  const held = (value: number, fallback: number): number =>
    Number.isFinite(value) ? value : fallback;

  const height = Math.max(1e-6, held(envelope.height, DEFAULT_ENVELOPE.height));
  const exponent = Math.min(
    MAX_FORK_EXPONENT,
    Math.max(
      MIN_FORK_EXPONENT,
      held(params.forkExponent, DEFAULT_RADII.forkExponent),
    ),
  );
  // Negative is a branch that thickens toward its tip. Clamped away
  // rather than honoured: monotonicity is a promise of this function,
  // and a caller who wants that shape wants a different function.
  const taper = Math.max(0, held(params.lengthTaper, DEFAULT_RADII.lengthTaper));
  const trunk =
    Math.max(
      MIN_TRUNK_RADIUS,
      held(params.trunkRadius, DEFAULT_RADII.trunkRadius),
    ) * height;
  /* Forward pass: how much thickness the length taper has shed by the
     time it reaches each node, in e-foldings from the root. Held as
     an accumulated total rather than applied step by step so that it
     can be capped without the cap depending on how finely the branch
     happens to be subdivided. Node indices rise away from the root,
     so one forward pass sees every parent before its children. */
  const shed = new Float64Array(count);
  for (let i = 1; i < count; i += 1) {
    const parent = nodes[i].parent;
    if (parent < 0) continue;
    const length = nodes[parent].position.distanceTo(nodes[i].position);
    shed[i] = Math.min(
      MAX_TAPER_SHED,
      shed[parent] + (taper * length) / height,
    );
  }

  /* Relative radii, with every tip at 1. `carried` is the running sum
     of the children's contributions to the fork rule at each node -
     the whole solve is that one accumulator, read once per node on
     the way down and never revisited. It runs over the nodes above
     the crossover only: a colonization tip with twigs under it is
     still a tip to the fork rule, which is what keeps the limbs where
     they were. */
  const relative = new Float64Array(count);
  const relativeStart = new Float64Array(count);
  const carried = new Float64Array(count);

  for (let i = crossover - 1; i >= 0; i -= 1) {
    // A node nothing grew from is a tip, and every tip is the same
    // thickness: the tree's fineness is then the branch count, which
    // is what the density dial already means.
    relative[i] = carried[i] > 0 ? carried[i] ** (1 / exponent) : 1;

    const parent = nodes[i].parent;
    if (parent < 0) {
      relativeStart[i] = relative[i];
      continue;
    }
    // The thickness the taper shed climbing this edge, put back on
    // the way down. Continuous in distance rather than per-step, so
    // subdividing an edge cannot change the thickness at its ends.
    relativeStart[i] = relative[i] * Math.exp(shed[i] - shed[parent]);
    carried[parent] += relativeStart[i] ** exponent;
  }

  // Normalise on the root. Every relative radius is at least 1 and
  // the root's is the largest of them, so this is a positive finite
  // scale in every case, the lone-root skeleton included.
  const scale = trunk / relative[0];
  for (let i = 0; i < crossover; i += 1) {
    radius[i] = relative[i] * scale;
    startRadius[i] = relativeStart[i] * scale;
  }
  if (crossover === count) return { radius, startRadius };

  const branches = skeleton as TwiggedSkeleton;
  for (let i = crossover; i < count; i += 1) {
    const parent = nodes[i].parent;
    const record = i - crossover;
    startRadius[i] = branches.branchId[record] === i
      ? branches.baseRadius[record]
      : radius[parent];
    radius[i] = branches.endRadius[record];
  }

  return { radius, startRadius };
}
