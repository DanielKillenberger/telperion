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
 * Two things here are not Runions, and they are the same bug seen from
 * two sides. A node grows toward its nearest attractors; the moment it
 * arrives between two of them the nearest set flips to the ones behind
 * it and the next step goes back the way it came. Measured on the
 * fn-11.2 skeleton that is a 101-degree reversal, and with the bias
 * field on it reaches 176.8 - a branch going forward and then almost
 * exactly backward, which is the sawtooth the owner screenshotted.
 *
 *   `maxTurnPerStep` is the stiffness a plant has and this algorithm
 *   does not: a growth step may not turn more than so far from the
 *   step that preceded it, so a reversal is not a direction the
 *   generator can express.
 *
 *   Termination is the other half, and it is the half that is easy to
 *   miss. A branch stuck between two attractors it cannot reach - the
 *   midpoint between them is further than `killDistance` from either,
 *   so neither is ever retired - will, once it can no longer turn
 *   around, simply push on into nothing. So a step that closes on none
 *   of the attractors still pulling its node ends that branch. A real
 *   branch with nowhere to grow stops, and every node it would have
 *   spent oscillating in place is a node the rest of the crown gets.
 *
 * The output is topology only - positions and parents. Thickness is
 * fn-11.4's and a surface is fn-11.5's.
 * ------------------------------------------------------------------ */

/** Dot product above which two unit directions are the same direction.
 *  A few thousandths of a degree apart: this catches a repeat of an
 *  identical computation, never two branches a fork could tell
 *  apart. */
const SAME_DIRECTION = 1 - 1e-9;

/** How far a growth step may turn from the step before it, in degrees,
 *  when the config does not say. This is bending stiffness, and it is
 *  the single number that decides whether a tree reads as stiff and
 *  upright or as whippy and drooping, so it is a parameter and not a
 *  constant - `GrowthConfig.maxTurnPerStep` overrides it per tree.
 *
 *  35 degrees over a step of about two per cent of the tree's height
 *  is a limb that can bend right around over half a dozen steps and
 *  cannot kink. */
export const DEFAULT_MAX_TURN_PER_STEP = 35;

const DEG_TO_RAD = Math.PI / 180;

/** Bends one growth step. Given the node the step leaves from, the
 *  direction colonization chose, and the length of the step about to
 *  be taken, it returns the direction actually taken, as a unit vector.
 *
 *  The step length is handed over because a field with a wavelength
 *  has to know how finely it is being sampled: a bend shorter than a
 *  few steps is a sawtooth however smooth the function behind it, and
 *  a field that is never told the step distance can be asked for one.
 *
 *  Pure in its arguments, which is what keeps the skeleton a pure
 *  function of the generator's inputs - and it must never return a
 *  direction that reverses the one it was given, or the trunk's climb
 *  stops gaining height. torsion.ts holds itself to both. */
export type GrowthBias = (
  position: THREE.Vector3,
  direction: THREE.Vector3,
  stepDistance: number,
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
  /** Directional persistence: how far one growth step may turn from
   *  the step before it, in degrees per step. Bending stiffness, under
   *  the only unit it has.
   *
   *  Small is a stiff tree that commits to a direction - about 10
   *  degrees is a leader that will not be deflected. Large is whippy
   *  and searching, following its attractors wherever they lead: past
   *  about 60 the crown starts to look loose, and at 180 the limit is
   *  off and the generator is free to reverse a branch onto itself
   *  again. Defaults to `DEFAULT_MAX_TURN_PER_STEP`.
   *
   *  It is per step, so its effect depends on `stepDistance`: the same
   *  degrees over a shorter step is a tighter curve on the ground. */
  maxTurnPerStep?: number;
  /** Bends every step this run takes, the trunk's climb included.
   *  Absent is the unbiased algorithm: straight up the trunk, and
   *  wherever the attractors say after that. */
  bias?: GrowthBias;
}

/** `wanted`, turned back until it is no more than `maxRadians` away
 *  from `from`. Both arguments are unit vectors and so is the result.
 *
 *  The turn runs along the great circle between the two, which is the
 *  only path that keeps the whole of the wanted direction that fits
 *  inside the limit - clamping a component or blending linearly would
 *  drag the result toward one or the other. */
function limitTurn(
  from: THREE.Vector3 | null,
  wanted: THREE.Vector3,
  maxRadians: number,
): THREE.Vector3 {
  // The root has no step behind it, and a limit of half a turn cannot
  // be exceeded: both mean the direction stands as chosen.
  if (from === null || maxRadians >= Math.PI) return wanted;
  const cosine = Math.min(1, Math.max(-1, from.dot(wanted)));
  const angle = Math.acos(cosine);
  if (angle <= maxRadians) return wanted;

  const sine = Math.sin(angle);
  if (sine < 1e-9) {
    /* Dead antiparallel: every great circle through the two is as
       short as every other, so there is no turn to shorten. Any
       perpendicular is as good an answer as any other and the only
       thing that matters is that the same input always picks the same
       one, or the skeleton stops being deterministic. Take the world
       axis this step is least aligned with. */
    const ax = Math.abs(from.x);
    const ay = Math.abs(from.y);
    const az = Math.abs(from.z);
    const reference =
      ax <= ay && ax <= az
        ? new THREE.Vector3(1, 0, 0)
        : ay <= az
          ? new THREE.Vector3(0, 1, 0)
          : new THREE.Vector3(0, 0, 1);
    const across = reference.cross(from).normalize();
    return from
      .clone()
      .multiplyScalar(Math.cos(maxRadians))
      .addScaledVector(across, Math.sin(maxRadians));
  }

  return from
    .clone()
    .multiplyScalar(Math.sin(angle - maxRadians) / sine)
    .addScaledVector(wanted, Math.sin(maxRadians) / sine)
    .normalize();
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
  const maxTurn =
    Math.max(0, config.maxTurnPerStep ?? DEFAULT_MAX_TURN_PER_STEP) * DEG_TO_RAD;

  /** The direction of the step that made node `index`, as a unit
   *  vector, or null at the root. Derived rather than carried: every
   *  step is `stepDistance` long, so no node's arrival direction can
   *  be lost or go stale. */
  const arrival = (index: number): THREE.Vector3 | null => {
    const parent = nodes[index].parent;
    if (parent < 0) return null;
    return nodes[index].position
      .clone()
      .sub(nodes[parent].position)
      .normalize();
  };

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
    /* The climb runs through the bias field like every other step,
       which is what stops the bare trunk being a mathematically
       straight line, and through the turn limit like every other step,
       which is what stops it kinking.
       The field promises never to reverse `up`, so every heading it
       returns gains height; and the great-circle turn between two
       directions that both gain height gains height too, the upward
       half-space being convex along the short arc. The loop therefore
       still climbs every round and still terminates on its own
       conditions. */
    const heading = limitTurn(
      arrival(tip),
      config.bias ? config.bias(nodes[tip].position, up, step) : up,
      maxTurn,
    );
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
  const heading = new Map<number, THREE.Vector3>();
  const closing = new Set<number>();
  const directions = new Map<number, THREE.Vector3[]>();
  /* Tips that have been found to have nowhere to grow. A branch does
     not go back on that: the geometry that trapped it - its own
     arrival direction against the attractors around it - is fixed the
     moment it stops, and re-deciding it every round would be the
     accordion again, one round slower. */
  const stopped = new Set<number>();
  const candidate = new THREE.Vector3();
  while (nodes.length < config.maxNodes) {
    pull.clear();
    for (let a = 0; a < count; a += 1) {
      if (alive[a] === 0 || nearestSq[a] > influenceSq) continue;
      const parent = nearest[a];
      if (stopped.has(parent)) continue;
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

    /* Decide where each node would go, before deciding whether it may.
       Both the bias field and the turn limit are between the pull and
       the step actually taken, and it is the step actually taken that
       has to close on something. */
    heading.clear();
    for (const [parent, sum] of grown) {
      const length = sum.length();
      if (length === 0) continue; // pulls cancelled exactly; no direction
      const chosen = sum.divideScalar(length);
      const direction = config.bias
        ? config.bias(nodes[parent].position, chosen, step)
        : chosen;
      heading.set(parent, limitTurn(arrival(parent), direction, maxTurn));
    }

    /* Which of those steps is progress. An attractor still pulling a
       node is one it can see and has not reached; if the step it is
       about to take gets no closer to a single one of them, the branch
       is not growing toward anything and pushing it out another node
       would be the comb. One pass over the attractors, the same shape
       as the pull above - `nearestSq[a]` is already the distance from
       that attractor to this very node. */
    closing.clear();
    for (let a = 0; a < count; a += 1) {
      if (alive[a] === 0 || nearestSq[a] > influenceSq) continue;
      const parent = nearest[a];
      if (closing.has(parent)) continue;
      const direction = heading.get(parent);
      if (direction === undefined) continue;
      candidate
        .copy(nodes[parent].position)
        .addScaledVector(direction, step);
      /* And a step that leaves the crown is not progress either,
         whatever it is closing on. The envelope has no width below the
         bare-trunk height, so a limb down there is outside the authored
         silhouette - the pull loop already refuses to grow *from* a
         node below the line, and this is the same rule for a node
         about to put its child under it. */
      if (candidate.y < config.trunkHeight) continue;
      if (candidate.distanceToSquared(attractors[a]) < nearestSq[a]) {
        closing.add(parent);
      }
    }

    const before = nodes.length;
    for (const [parent] of grown) {
      if (nodes.length >= config.maxNodes) break;
      const direction = heading.get(parent);
      if (direction === undefined) continue;
      if (!closing.has(parent)) {
        stopped.add(parent);
        continue;
      }

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
