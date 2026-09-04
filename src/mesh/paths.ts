import type { RadiusField } from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";

/* ------------------------------------------------------------------ *
 * THE SKELETON, CUT INTO RUNS
 *
 * The skeleton is a tree of nodes, and the fn-11.4 viewer drew it one
 * edge at a time: a tapered cylinder per edge, each with its own two
 * flat ends. Wherever the centreline bent, the two ends meeting at a
 * node stopped covering each other and the joint opened - the rings
 * and the see-through gaps the owner screenshotted. Nothing about that
 * is fixable at the edge level, because two independently oriented
 * discs never meet.
 *
 * So the surface is not swept over edges. It is swept over runs: a
 * continuous chain of nodes from an attachment point to a tip, with
 * ONE ring per node, shared by the segment below it and the segment
 * above it. A shared ring cannot open, because there is only one of
 * it. That is the whole of the continuity fix along a limb, and it is
 * structural rather than tuned - no epsilon, no overlap, no fudge.
 *
 * Which child continues the run and which one starts a new one is
 * decided by thickness: the thickest child is the same limb carrying
 * on, and everything else is a branch off it. That is what the radius
 * solve already means, so the surface inherits the fork rule rather
 * than forming a second opinion about which limb is the leader. Ties
 * go to the lower node index, so the decomposition is deterministic in
 * the skeleton alone.
 *
 * Every run past the first begins AT the fork node it leaves, not at
 * the child. The run therefore starts inside the parent limb's own
 * volume, which is what gives surface.ts somewhere to socket the child
 * so that a fork has no gap to show. The alternative - starting the
 * child at its own node - leaves the gap between the child's first
 * ring and the parent's surface, which is the fork half of exactly the
 * defect being fixed.
 * ------------------------------------------------------------------ */

/** Two nodes closer than this are the same point, as a multiple of the
 *  tree's own scale. Colonization can emit a step of zero length, and
 *  a zero-length segment is a ring of degenerate triangles - so those
 *  nodes are folded into their parent and their children re-parented
 *  onto it, which drops the degeneracy without dropping the subtree. */
const SAME_POINT = 1e-9;

export interface BranchPath {
  /** Node indices along one continuous run, in growth order.
   *  `nodes[0]` is where the run attaches - the skeleton's root for
   *  the trunk, the fork node for every other run. At least two long;
   *  a run with nothing to sweep is never emitted. */
  nodes: number[];
  /** True for the one run that starts at the skeleton's root and
   *  follows the thickest child at every fork: the trunk. Every other
   *  run leaves a fork and has to be socketed into its parent. */
  trunk: boolean;
}

/**
 * Cuts `skeleton` into the runs the surface is swept over.
 *
 * Pure and deterministic in the skeleton and the radius field. Every
 * edge of the skeleton appears in exactly one run, so the surface
 * covers the whole tree and covers no part of it twice.
 *
 * Returns an empty array for a skeleton that never grew - one node has
 * no run in it, and the caller gets an empty mesh rather than an
 * exception.
 */
export function branchPaths(
  skeleton: Skeleton,
  field: RadiusField,
): BranchPath[] {
  const nodes = skeleton.nodes;
  const count = nodes.length;
  if (count < 2) return [];

  /* Zero-length edges collapse onto their parent. `stands[i]` is the
     node index that node i's position is actually drawn at: itself,
     or the ancestor it is a duplicate of. Node indices rise away from
     the root, so one forward pass resolves every alias before it is
     used. */
  const stands = new Int32Array(count);
  const children: number[][] = Array.from({ length: count }, () => []);
  for (let i = 1; i < count; i += 1) {
    const parent = nodes[i].parent;
    if (parent < 0) {
      stands[i] = i;
      continue;
    }
    const at = stands[parent];
    if (nodes[i].position.distanceTo(nodes[at].position) > SAME_POINT) {
      stands[i] = i;
      children[at].push(i);
    } else {
      stands[i] = at;
    }
  }

  /** The child that carries the limb on: the thickest where it leaves
   *  the fork, ties to the lower index. */
  const leader = (of: number): number => {
    const kids = children[of];
    let best = kids[0];
    for (let k = 1; k < kids.length; k += 1) {
      if (field.startRadius[kids[k]] > field.startRadius[best]) best = kids[k];
    }
    return best;
  };

  const paths: BranchPath[] = [];
  /* Breadth-first over runs, seeded with the trunk. Each seed is the
     node the run attaches to plus the child it takes first; the side
     children found while walking a run become seeds of their own, in
     the order they were grown. */
  const seeds: { attach: number; first: number }[] = [];
  if (children[0].length > 0) seeds.push({ attach: 0, first: leader(0) });
  for (const kid of children[0]) {
    if (kid !== seeds[0]?.first) seeds.push({ attach: 0, first: kid });
  }

  for (let s = 0; s < seeds.length; s += 1) {
    const seed = seeds[s];
    const run = [seed.attach, seed.first];
    let at = seed.first;
    while (children[at].length > 0) {
      const next = leader(at);
      for (const kid of children[at]) {
        if (kid !== next) seeds.push({ attach: at, first: kid });
      }
      run.push(next);
      at = next;
    }
    paths.push({ nodes: run, trunk: s === 0 });
  }

  return paths;
}
