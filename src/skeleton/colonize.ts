import * as THREE from "three";

/* ------------------------------------------------------------------ *
 * SPACE COLONIZATION
 *
 * Branches compete for a cloud of attractor points and grow toward
 * whichever ones they are closest to. Runions et al., "Modeling Trees
 * with a Space Colonization Algorithm" (2007), with no embellishment.
 *
 * The reason the spec chose this over an L-system is control, not
 * novelty: the attractor cloud comes from an authored envelope, so the
 * silhouette is a design decision and only the branching is generated.
 * Recursive-branching-with-jitter is self-similar and statistically
 * uniform by construction, which is why every procgen tree built that
 * way reads as the same tree.
 *
 * Nothing in here is random. Given the same attractors and the same
 * config it emits the same nodes in the same order, every time; all
 * the chance in the generator lives in where the attractors were
 * scattered. Ties go to the earlier node, so growth order decides
 * nothing the geometry has not already decided.
 *
 * The one thing it has an opinion about beyond the attractors is
 * whatever `config.bias` hands back. Direction is not the attractors'
 * business - they say where to go, not how to get there - so the field
 * that says a tree grows up, leans, and writhes on the way lives in
 * torsion.ts and reaches every step through that one hook.
 *
 * The output is topology only - positions and parents. Thickness is
 * fn-11.4's and a surface is fn-11.5's.
 * ------------------------------------------------------------------ */

/** Dot product above which two unit directions are the same direction.
 *  A few thousandths of a degree apart: this catches a repeat of an
 *  identical computation, never two branches a fork could tell
 *  apart. */
const SAME_DIRECTION = 1 - 1e-9;

/** Bends one growth step. Given the node the step leaves from and the
 *  direction colonization chose, it returns the direction actually
 *  taken, as a unit vector.
 *
 *  Pure in its arguments, which is what keeps the skeleton a pure
 *  function of the generator's inputs - and it must never return a
 *  direction that reverses the one it was given, or the trunk's climb
 *  stops gaining height. torsion.ts holds itself to both. */
export type GrowthBias = (
  position: THREE.Vector3,
  direction: THREE.Vector3,
) => THREE.Vector3;

export interface SkeletonNode {
  /** Position in metres, with the root at the origin on the ground. */
  position: THREE.Vector3;
  /** Index of the node this one grew from; -1 at the root. Always
   *  lower than this node's own index, so a single forward pass over
   *  `nodes` always sees a parent before its children. */
  parent: number;
}

export interface Skeleton {
  nodes: SkeletonNode[];
}

export interface GrowthConfig {
  /** How far a node reaches for attractors, in metres. Wide gives
   *  long sweeping limbs, narrow gives a dense twiggy thicket. */
  influenceRadius: number;
  /** An attractor this close to any node has been reached, and is
   *  retired so nothing else grows toward it. In metres. Keep it at
   *  or above `stepDistance` or growth converges slowly. */
  killDistance: number;
  /** The length of one growth step, in metres. This is the resolution
   *  of the whole skeleton. */
  stepDistance: number;
  /** How high the trunk climbs before any branching is allowed, in
   *  metres. Below it the authored envelope has no width at all, so a
   *  branch down there is outside the silhouette however plausible it
   *  looks on its own. */
  trunkHeight: number;
  /** Hard bound on node count. A stop, not a target. */
  maxNodes: number;
  /** Bends every step this run takes, the trunk's climb included.
   *  Absent is the unbiased algorithm: straight up the trunk, and
   *  wherever the attractors say after that. */
  bias?: GrowthBias;
}

/**
 * Grows a skeleton from `start` into `attractors`.
 *
 * Returns a lone root node when there is nothing to grow toward or the
 * config forbids growth; the caller gets a well-formed skeleton in
 * every case rather than an exception to handle.
 */
export function colonize(
  attractors: readonly THREE.Vector3[],
  start: THREE.Vector3,
  config: GrowthConfig,
): Skeleton {
  const nodes: SkeletonNode[] = [{ position: start.clone(), parent: -1 }];
  const count = attractors.length;
  const step = config.stepDistance;
  if (count === 0 || step <= 0 || config.maxNodes < 2) return { nodes };

  const influenceSq = config.influenceRadius * config.influenceRadius;
  const killSq = config.killDistance * config.killDistance;

  // Per-attractor state, held as parallel typed arrays because this is
  // the inner loop of the whole generator.
  const alive = new Uint8Array(count).fill(1);
  const nearest = new Int32Array(count).fill(-1);
  const nearestSq = new Float64Array(count).fill(Number.POSITIVE_INFINITY);
  let scanned = 0;

  /* Folds every node added since the last call into each attractor's
     running nearest, then retires the attractors that nearest has
     reached. Nodes are only ever appended and never moved, so a
     running minimum is exact - rescanning the whole tree each round
     would return the same answer for O(nodes x attractors) more work,
     which is the difference between the panel answering a slider drag
     and stalling on it. */
  const settle = (): void => {
    for (let a = 0; a < count; a += 1) {
      if (alive[a] === 0) continue;
      const target = attractors[a];
      for (let n = scanned; n < nodes.length; n += 1) {
        const distance = nodes[n].position.distanceToSquared(target);
        if (distance < nearestSq[a]) {
          nearestSq[a] = distance;
          nearest[a] = n;
        }
      }
      if (nearestSq[a] <= killSq) alive[a] = 0;
    }
    scanned = nodes.length;
  };

  const anyInReach = (): boolean => {
    for (let a = 0; a < count; a += 1) {
      if (alive[a] === 1 && nearestSq[a] <= influenceSq) return true;
    }
    return false;
  };

  settle();

  /* Reach. The root stands on the ground and the crown starts well
     above it, so at the outset nothing is within reach of anything and
     the algorithm proper has nothing to do. The trunk climbs straight
     up to the bare-trunk height, and then on until the lowest
     attractors come into range - but never past the highest one, which
     is what stops an unreachable crown from growing a flagpole all the
     way to maxNodes.

     Climbing to `trunkHeight` is not the same as climbing until
     something is in reach, and stopping at the latter alone was a bug:
     the influence radius is several times the step, so the lowest
     attractors come into range of the trunk long before it has reached
     the crown, and the tree forks at half its intended trunk height -
     out in the open, where the envelope says it has no width. */
  let ceiling = Number.NEGATIVE_INFINITY;
  for (const attractor of attractors) ceiling = Math.max(ceiling, attractor.y);
  const up = new THREE.Vector3(0, 1, 0);

  while (
    nodes.length < config.maxNodes &&
    nodes[nodes.length - 1].position.y <= ceiling &&
    (nodes[nodes.length - 1].position.y < config.trunkHeight || !anyInReach())
  ) {
    const tip = nodes.length - 1;
    // The climb runs through the bias field like every other step, which
    // is what stops the bare trunk being a mathematically straight line.
    // The field promises never to reverse `up`, so the loop still gains
    // height every round and still terminates on its own conditions.
    const heading = config.bias
      ? config.bias(nodes[tip].position, up)
      : up;
    nodes.push({
      position: nodes[tip].position.clone().addScaledVector(heading, step),
      parent: tip,
    });
    settle();
  }

  // Grow. Each round, every attractor in reach pulls on its nearest
  // node; every node that was pulled on grows one step along the sum
  // of those pulls. That sum is the entire reason a fork happens: a
  // node caught between two clusters splits the difference until one
  // of them is closer to a child than to it.
  const pull = new Map<number, THREE.Vector3>();
  const directions = new Map<number, THREE.Vector3[]>();
  while (nodes.length < config.maxNodes) {
    pull.clear();
    for (let a = 0; a < count; a += 1) {
      if (alive[a] === 0 || nearestSq[a] > influenceSq) continue;
      const parent = nearest[a];
      /* Nothing below the bare-trunk height may branch, whoever is
         nearest. The climb loop above only decides where the trunk
         stops; it does not stop a low trunk node from being some
         attractor's nearest and sprouting a limb out in the open, where
         the envelope has no width at all. That used to hold by luck -
         on a dead straight trunk the tip was nearest to everything in
         reach - and the luck ran out the moment the bias field was
         allowed to bend the trunk toward one side of the crown. An
         attractor whose only candidate is down there goes unreached,
         which is the truth about it. */
      if (nodes[parent].position.y < config.trunkHeight) continue;
      const direction = attractors[a].clone().sub(nodes[parent].position);
      const length = direction.length();
      if (length === 0) continue;
      direction.divideScalar(length);
      const sum = pull.get(parent);
      if (sum === undefined) pull.set(parent, direction);
      else sum.add(direction);
    }
    if (pull.size === 0) break;

    // Sorted by parent index, so a new node's index is decided by
    // where it grew from and never by the order the attractors
    // happened to be scanned in.
    const grown = [...pull.entries()].sort((a, b) => a[0] - b[0]);
    const before = nodes.length;
    for (const [parent, sum] of grown) {
      if (nodes.length >= config.maxNodes) break;
      const length = sum.length();
      if (length === 0) continue; // pulls cancelled exactly; no direction
      const chosen = sum.divideScalar(length);
      const direction = config.bias
        ? config.bias(nodes[parent].position, chosen)
        : chosen;

      /* A node grows in any one direction exactly once, ever. Without
         this the algorithm does not terminate: an attractor whose pull
         is outvoted by the rest of the sum leaves its nearest node
         unchanged - the child grew away from it - so the same node
         recomputes the same sum and emits the same child again, and
         again. Left unguarded that is not a slow tree, it is the whole
         node budget spent on one duplicated point.

         Blocking the repeat rather than retiring the attractor is what
         keeps forks: when the child does take the outvoting attractors
         with it, what is left pulling the parent is a different set,
         so the sum is a different direction and the second branch is
         allowed out. Only a genuinely unchanged sum is refused, and an
         attractor stuck behind one goes unreached, which is the truth
         about it. */
      const already = directions.get(parent);
      if (already?.some((prior) => prior.dot(direction) > SAME_DIRECTION)) {
        continue;
      }
      if (already === undefined) directions.set(parent, [direction.clone()]);
      else already.push(direction.clone());

      nodes.push({
        position: nodes[parent].position
          .clone()
          .addScaledVector(direction, step),
        parent,
      });
    }
    if (nodes.length === before) break;
    settle();
  }

  return { nodes };
}
