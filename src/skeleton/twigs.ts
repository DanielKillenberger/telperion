import * as THREE from "three";

import {
  DEFAULT_MAX_TURN_PER_STEP,
  limitTurn,
  type GrowthConfig,
  type Skeleton,
  type SkeletonNode,
} from "./colonize";

/* ------------------------------------------------------------------ *
 * TWIGS: THE RECURSION BELOW THE CROSSOVER
 *
 * Space colonization stops where its attractors stop meaning anything,
 * and on a tree of this size that is about twelve fork generations
 * from the trunk with eleven still to go: its finest wood is 0.79 m
 * across and a leaf is 0.12 m long. Below that scale nothing global
 * decides anything - the crown has already filled its envelope and
 * reached for its light - so the branching continues under local
 * rules, from every tip colonization left, appending into the same
 * skeleton. One structure, two passes, and nothing downstream can tell
 * which pass made a node. Generating per branch order with a different
 * rule set per order is what Weber and Penn, SpeedTree and Runions all
 * do; what is particular here is only that the second pass inherits
 * the first's exit state instead of being seeded afresh at a seam.
 *
 * WHAT A TIP HANDS OVER. Its position, its terminal tangent - the
 * direction of the step that made it - and the resolution it was grown
 * at, the growth step, which is the only measure of vigour the first
 * pass has. Direction is never reseeded: every twig step is the
 * parent's direction, bent by the growth-bias field and then held to
 * the same turn limit every colonization step is held to, through the
 * same `limitTurn`. A tree is a cantilever, and a direction that jumps
 * at the crossover is a bending moment that jumps.
 *
 * THE BIAS FIELD SHAPES TWIGS AS IT SHAPES LIMBS. Every twig node's
 * direction goes through `config.bias` - the same function colonize
 * consults, at the parent's position and at colonization's own step -
 * so torsion, writhe, spiral, lean and gravitropism read continuously
 * from trunk to tip. A straight twig on a twisted limb passes every
 * geometric check and still breaks the tree; contorted hazel is
 * twisted to the twig. With every term of the field at zero the field
 * is exactly `direction / |direction|`, which is exactly what the
 * unbiased path does to its own wanted direction, so the field's whole
 * effect below the crossover is attributable: zero terms and no field
 * are one recursion, byte for byte.
 *
 * THE RULES, and where their resting values come from. Every one is a
 * dial that can be driven past its resting value; the resting value is
 * a real tree, so the tree at rest is real and every departure is a
 * departure from one. `levels` alone has no botanical default: it is
 * the owner's decision in clay (R8), and its default of zero orders is
 * what keeps both presets the trees they were (R7).
 *
 *   children    - 2: a leader and one lateral at every node, which is
 *                 alternate phyllotaxis, one bud per node, the commonest
 *                 arrangement in broadleaved trees (oak, elm, beech,
 *                 birch, poplar; the opposite-leaved maples and ashes
 *                 are 3 here). Corner's second rule (Corner 1949; the
 *                 architectural models of Halle, Oldeman and Tomlinson
 *                 1978 restate it) has the count of appendages fall as
 *                 ramification rises, so the count is held per node and
 *                 the shrinking is carried by `taper`. ez-tree states
 *                 children per level the same way; Weber and Penn state
 *                 them as `nBranches` per stem.
 *   angle       - 45 degrees off the parent axis for a lateral: Weber
 *                 and Penn 1995, "Creation and Rendering of Realistic
 *                 Trees", whose `nDownAngle` at the twig levels is 45 for
 *                 Quaking Aspen at levels 2 and 3 and for Black Tupelo at
 *                 level 3, inside a table that runs 20 to 60 across its
 *                 four species. The turn limit binds it the way it binds
 *                 every fork above: a preset stiffer than the angle gets
 *                 its stiffness, which is R2's direction criterion.
 *   divergence  - 137.508 degrees, the golden angle: the Fibonacci
 *                 phyllotaxis nearly every spiral-leaved plant shows
 *                 (Jean 1994, "Phyllotaxis"), the number the canopy
 *                 already places leaves on a shoot with, and Weber and
 *                 Penn's `nRotate` of 140 at every level of Aspen and
 *                 Tupelo rounded off. It is the angle the whorl turns
 *                 about the axis from one node to the next, so a leaf
 *                 never sits directly above the one two before it.
 *   internode   - 1 growth step: the first twig order continues at the
 *                 resolution of the wood it leaves, so the change of
 *                 method is invisible in segment length (R1). That is
 *                 continuity rather than botany; the botany is Corner's
 *                 first rule, that an appendage's size follows the mass
 *                 of the axis that bears it, and a shoot that continues
 *                 a tip inherits the tip's vigour.
 *   taper       - 0.6 per order: each order's internode is this fraction
 *                 of the one above. Weber and Penn's `nLength`, a child's
 *                 length relative to its parent's, is 0.6 at level 2 for
 *                 both Quaking Aspen and Black Tupelo and 0.4 at Tupelo's
 *                 level 3; Corner's second rule is the same statement
 *                 without a number. This is the fine orders' own taper
 *                 law in length and fork count: thickness is not known
 *                 during this pass. The radius solve runs after it and,
 *                 below the crossover this pass exposes, applies its own
 *                 steeper law (`RadiusParams.twigTaper`) anchored to the
 *                 parent's actual radius at the handoff - area alone gave
 *                 2 to 1 leaf to twig at eight orders where the botany
 *                 is 25 to 1.
 *
 * LEVEL-CAPPED, NOT SCALE-SEARCHED. The recursion stops on `levels`, a
 * counter sized to real twig counts - a leader-and-lateral tree adds
 * 2^(levels+1) - 2 nodes per tip, sixty-two at five orders - and on the
 * node ceiling the run already carries. Shrinking a search radius
 * toward zero is the failure mode the whole spec exists to escape.
 *
 * ONE WHORL, ONE COLLISION CHECK. Blind local replacement with no
 * neighbour awareness is the documented weakness of pure rule systems,
 * so each child is checked against the siblings already accepted at
 * its node: two that come out within half the branching angle of each
 * other - half the angle the tree can actually make, once its own
 * stiffness has bound it - after the field and the turn limit have had
 * their say, are one twig, and the later is dropped.
 *
 * NO ATTRACTORS, SO NONE OF COLONIZE'S PER-ROUND MACHINERY. Flat arrays
 * walked in one order, breadth-first by order across every tip: every
 * child's parent was appended in an earlier order, so `parent < self`
 * holds by construction, and no `Map` or `Set` is anywhere near the
 * candidate path. Determinism depends on that. No chance either - the
 * phyllotactic phase is carried along each lineage in a frame
 * transported from the tip's own tangent, and all the variation the
 * twigs show comes from the field and the limbs they continue.
 * ------------------------------------------------------------------ */

/** The one skeleton the two passes build, with the seam between them
 *  named: `crossover` is the index of the first node the twig pass
 *  appended, so `nodes[0, crossover)` are colonization's and
 *  `nodes[crossover, length)` are the twigs. Equal to `nodes.length`
 *  when the pass appended nothing - zero orders, a bare skeleton, or a
 *  node ceiling already reached - which is how a consumer tells a tree
 *  with no crossover from one whose crossover it has failed to find.
 *  The radius solve reads it to know where the fine orders' own taper
 *  law begins; nothing else downstream needs to know which pass made
 *  a node. */
export interface TwiggedSkeleton extends Skeleton {
  crossover: number;
}

export interface TwigParams {
  /** Orders of local branching appended below colonization's tips.
   *  Zero is none, and both presets ship at zero until the owner has
   *  seen the alternatives in clay. Rounded to an integer and held to
   *  0 through `MAX_TWIG_LEVELS`. */
  levels: number;
  /** Children at every node, the leader that carries the shoot on
   *  included: 2 is alternate, 3 opposite, 4 whorled. Rounded to an
   *  integer and held to 1 through `MAX_TWIG_CHILDREN`. */
  children: number;
  /** How far a lateral leaves the parent axis, in degrees. The
   *  leader leaves at zero. Held to 0 through 90; the run's turn limit
   *  binds it as it binds every fork above. */
  angle: number;
  /** How far the whorl turns about the axis from one node to the
   *  next, in degrees. 137.508 is the golden angle; 180 is distichous,
   *  90 decussate. Any finite angle. */
  divergence: number;
  /** The first order's internode as a multiple of the growth step
   *  the tip was grown at. 1 continues the tip's own resolution. Held
   *  to `MIN_INTERNODE` through `MAX_INTERNODE`. */
  internode: number;
  /** Each order's internode as a fraction of the one above. Held to
   *  `MIN_TAPER` through 1: an order that grows longer than its
   *  parent is not a taper and is not a twig.
   *  This is the twig's LENGTH per order. `RadiusParams.twigTaper` is
   *  its thickness, applied by the radius solve below the crossover. */
  taper: number;
}

/** The twigs at rest: none, and when there are some, an alternate
 *  broadleaf twig. Sources beside each term in the file header. */
export const DEFAULT_TWIGS: TwigParams = {
  levels: 0,
  children: 2,
  angle: 45,
  divergence: 137.508,
  internode: 1,
  taper: 0.6,
};

/** Twelve orders: one more than the eleven the spec counts from today's
 *  terminal wood to a 2.5 mm twig - twenty-three from the trunk, less
 *  the twelve colonization makes - so the rail's end is reachable with
 *  an order to spare. A stop, not a target. */
export const MAX_TWIG_LEVELS = 12;
/** Past eight children at a node the whorl is a brush, and with
 *  twelve orders under it the count is astronomical before the node
 *  ceiling can say so. */
export const MAX_TWIG_CHILDREN = 8;
const MAX_ANGLE = 90;
/** Below this the twig is a point and above it the first twig order
 *  is longer than the limb it leaves. */
const MIN_INTERNODE = 1e-3;
const MAX_INTERNODE = 8;
const MIN_TAPER = 0.05;
const MAX_TAPER = 1;
/** The floor on the sibling separation, in radians, where the
 *  branching angle is zero: two children in exactly the same direction
 *  are still one twig. */
const MIN_SEPARATION = 1e-6;

const DEG_TO_RAD = Math.PI / 180;

/** NaN is the one value `Math.max` and `Math.min` pass through rather
 *  than pin, so it is named separately - the same rail idiom every
 *  other stage uses. */
const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

const pinned = (value: number, low: number, high: number): number =>
  Math.min(high, Math.max(low, value));

/** `twigs` with every term stated and every rail applied. A term left
 *  out or not a number is its documented default; a term outside its
 *  range is the nearer end of it. */
export function resolveTwigs(twigs?: Partial<TwigParams>): TwigParams {
  const asked = { ...DEFAULT_TWIGS, ...twigs };
  return {
    levels: pinned(
      Math.round(held(asked.levels, DEFAULT_TWIGS.levels)),
      0,
      MAX_TWIG_LEVELS,
    ),
    children: pinned(
      Math.round(held(asked.children, DEFAULT_TWIGS.children)),
      1,
      MAX_TWIG_CHILDREN,
    ),
    angle: pinned(held(asked.angle, DEFAULT_TWIGS.angle), 0, MAX_ANGLE),
    divergence: held(asked.divergence, DEFAULT_TWIGS.divergence),
    internode: pinned(
      held(asked.internode, DEFAULT_TWIGS.internode),
      MIN_INTERNODE,
      MAX_INTERNODE,
    ),
    taper: pinned(held(asked.taper, DEFAULT_TWIGS.taper), MIN_TAPER, MAX_TAPER),
  };
}

/** A unit vector perpendicular to unit `direction`, the same one for
 *  the same input: the world axis the direction is least aligned with,
 *  crossed with it - the choice `limitTurn` makes for its antiparallel
 *  case, for the same reason. */
function perpendicular(direction: THREE.Vector3): THREE.Vector3 {
  const ax = Math.abs(direction.x);
  const ay = Math.abs(direction.y);
  const az = Math.abs(direction.z);
  const reference =
    ax <= ay && ax <= az
      ? new THREE.Vector3(1, 0, 0)
      : ay <= az
        ? new THREE.Vector3(0, 1, 0)
        : new THREE.Vector3(0, 0, 1);
  return reference.cross(direction).normalize();
}

/** `normal` carried onto the plane perpendicular to unit `direction`:
 *  its component along the direction removed, then made unit. Where
 *  the two have become parallel there is nothing to carry, and
 *  `fallback` - the binormal, which cannot also be parallel - stands
 *  in. This is what keeps the phyllotactic phase a spiral along a
 *  shoot rather than a frame that jumps at every node. */
function transport(
  normal: THREE.Vector3,
  fallback: THREE.Vector3,
  direction: THREE.Vector3,
): THREE.Vector3 {
  const carried = normal
    .clone()
    .addScaledVector(direction, -normal.dot(direction));
  if (carried.lengthSq() > 1e-12) return carried.normalize();
  return fallback
    .clone()
    .addScaledVector(direction, -fallback.dot(direction))
    .normalize();
}

/** One node awaiting its children: where it is in `nodes`, the unit
 *  direction of the step that made it, the frame its whorl is laid
 *  out in, and the phase the whorl has reached. */
interface Shoot {
  at: number;
  direction: THREE.Vector3;
  normal: THREE.Vector3;
  phase: number;
}

/**
 * Continues `skeleton` from every tip colonization left, `twigs.levels`
 * orders down, and returns the one skeleton with the twig nodes
 * appended after the nodes it was given and `crossover` naming where
 * the appending began.
 *
 * `config` is the growth configuration the skeleton was grown under:
 * the step is the resolution the tips hand over, the bias field and
 * turn limit bend every twig step as they bent every limb step, the
 * bare-trunk height is the line no twig may cross, and the node
 * ceiling is the stop it already was. Pure in its arguments and
 * deterministic: the same skeleton and the same parameters append the
 * same nodes in the same order on any machine.
 *
 * A skeleton with fewer than two nodes has no tips and comes back as
 * it was; so does one asked for zero orders. Nothing here throws.
 */
export function branchTwigs(
  skeleton: Skeleton,
  config: GrowthConfig,
  twigs: TwigParams,
): TwiggedSkeleton {
  const base = skeleton.nodes;
  const nodes: SkeletonNode[] = base.slice();
  const crossover = base.length;
  const step = config.stepDistance;
  if (base.length < 2 || twigs.levels < 1 || !(step > 0)) {
    return { nodes, crossover };
  }

  const maxTurn =
    Math.max(0, config.maxTurnPerStep ?? DEFAULT_MAX_TURN_PER_STEP) *
    DEG_TO_RAD;
  const tilt = twigs.angle * DEG_TO_RAD;
  const divergence = twigs.divergence * DEG_TO_RAD;
  const laterals = twigs.children - 1;
  /* Two siblings closer than half the branching angle are one twig -
     half the angle the tree can actually make, which is the lesser of
     the angle asked for and the turn limit, or a stiff tree whose
     laterals are all held to its cone would lose most of them to a
     threshold measured against an angle it never reaches. Compared as
     a dot product, with a floor so that two coincident directions - a
     zero angle, or the field folding both onto the same edge of the
     cone - count as a collision rather than slipping past a strict
     test by one ulp. */
  const separation = Math.cos(
    Math.max(MIN_SEPARATION, Math.min(tilt, maxTurn) / 2),
  );

  /* The tips, in index order: every node past the root that nothing
     grew from. A tip whose own edge has no length has no tangent to
     hand over and is left as it is. */
  const children = new Int32Array(base.length);
  for (let i = 1; i < base.length; i += 1) children[base[i].parent] += 1;
  let frontier: Shoot[] = [];
  for (let i = 1; i < base.length; i += 1) {
    if (children[i] !== 0) continue;
    const direction = base[i].position
      .clone()
      .sub(base[base[i].parent].position);
    if (direction.lengthSq() === 0) continue;
    direction.normalize();
    frontier.push({
      at: i,
      direction,
      normal: perpendicular(direction),
      phase: 0,
    });
  }

  const binormal = new THREE.Vector3();
  const wanted = new THREE.Vector3();
  const across = new THREE.Vector3();
  const candidate = new THREE.Vector3();
  const accepted: THREE.Vector3[] = [];

  for (let order = 1; order <= twigs.levels; order += 1) {
    const length = step * twigs.internode * twigs.taper ** (order - 1);
    const next: Shoot[] = [];

    for (let s = 0; s < frontier.length; s += 1) {
      const shoot = frontier[s];
      const from = shoot.direction;
      const position = nodes[shoot.at].position;
      const phase = shoot.phase + divergence;
      binormal.crossVectors(from, shoot.normal);
      accepted.length = 0;

      for (let c = 0; c <= laterals; c += 1) {
        if (nodes.length >= config.maxNodes) return { nodes, crossover };

        /* The leader carries on; a lateral leaves at the branching
           angle, at its place round the whorl. Both are then what
           every colonization step is: the field's opinion of that
           direction at this position and this step, held within the
           turn limit of the step that arrived here. Without a field
           the wanted direction is made unit exactly as the field
           would make it, so zero terms and no field agree to the
           bit. */
        if (c === 0) {
          wanted.copy(from);
        } else {
          const azimuth = phase + ((c - 1) * Math.PI * 2) / laterals;
          across
            .copy(shoot.normal)
            .multiplyScalar(Math.cos(azimuth))
            .addScaledVector(binormal, Math.sin(azimuth));
          wanted
            .copy(from)
            .multiplyScalar(Math.cos(tilt))
            .addScaledVector(across, Math.sin(tilt));
        }
        const heading = limitTurn(
          from,
          config.bias
            ? config.bias(position, wanted, step)
            : wanted.clone().normalize(),
          maxTurn,
        );

        let collides = false;
        for (let a = 0; a < accepted.length && !collides; a += 1) {
          collides = accepted[a].dot(heading) >= separation;
        }
        if (collides) continue;

        /* The envelope has no width below the bare-trunk height, so
           a twig down there is outside the authored silhouette - the
           same rule colonize applies to a step about to put its
           child under the line. */
        candidate.copy(position).addScaledVector(heading, length);
        if (candidate.y < config.trunkHeight) continue;

        accepted.push(heading);
        nodes.push({ position: candidate.clone(), parent: shoot.at });
        next.push({
          at: nodes.length - 1,
          direction: heading,
          normal: transport(shoot.normal, binormal, heading),
          phase,
        });
      }
    }

    frontier = next;
    if (frontier.length === 0) break;
  }

  return { nodes, crossover };
}
