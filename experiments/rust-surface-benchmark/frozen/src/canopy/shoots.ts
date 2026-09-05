import { branchPaths } from "../mesh/paths";
import type { RadiusField } from "../radius";
import type { Skeleton } from "../skeleton/colonize";

/* ------------------------------------------------------------------ *
 * SHOOTS: WHERE FOLIAGE IS ENTITLED TO GROW
 *
 * A shoot is a terminal run - a continuous chain of nodes ending at a
 * tip that nothing grows out of. That is not a second decomposition of
 * the skeleton: it is `branchPaths`' own. Every run it emits walks the
 * thickest child until there is no child left, so every run already
 * ends at a tip, and the terminal-run rule is satisfied by taking its
 * output rather than by re-walking the raw nodes. Re-walking is also
 * what would force this file to re-derive the zero-length-edge folding
 * that `branchPaths` resolves in its `stands[]` pass, and two opinions
 * about which node a position is drawn at is exactly the kind of drift
 * that shows up as leaves floating off the wood.
 *
 * A run is not a shoot along its whole length, though. The trunk run
 * of a tree this size starts as a column seven metres through, and a
 * leaf on it would be a leaf growing out of bark that has not carried
 * a bud in decades. Real trees bear foliage on young wood, so the
 * shoot is the distal part of the run where the wood is still thin
 * enough - one named parameter, `maxRadius`, and no other rule.
 *
 * The radius solve guarantees thickness never increases from root to
 * tip, so "thin enough" is a suffix of the run and one backward scan
 * finds where it starts. A run whose own tip is already too thick
 * carries no shoot at all, which is the honest answer for a limb that
 * never got fine.
 * ------------------------------------------------------------------ */

export interface Shoot {
  /** Node indices along the shoot, in growth order, the tip last.
   *  Always at least two long: a shoot of one node has no direction to
   *  place anything along, so it is widened by one node rather than
   *  emitted degenerate. */
  nodes: number[];
}

/**
 * The shoots of `skeleton`: the young-wood end of every terminal run.
 *
 * Pure and deterministic in the skeleton and the radius field, and
 * array-backed throughout - the order shoots come out in is
 * `branchPaths`' order, which is a function of the skeleton alone.
 *
 * Returns an empty array for a skeleton with no runs in it (fewer than
 * two nodes, or a root nothing grew from), and for one whose every
 * tip is thicker than `maxRadius`. A caller gets an empty canopy
 * rather than an exception.
 */
export function shoots(
  skeleton: Skeleton,
  field: RadiusField,
  maxRadius: number,
): Shoot[] {
  const runs = branchPaths(skeleton, field);
  const found: Shoot[] = [];

  for (let r = 0; r < runs.length; r += 1) {
    const nodes = runs[r].nodes;
    const last = nodes.length - 1;
    /* Stated as "not thicker" rather than "thinner or equal" so a
       non-finite radius or threshold falls out here instead of
       propagating into a transform. */
    if (!(field.radius[nodes[last]] <= maxRadius)) continue;

    let first = last;
    while (first > 0 && field.radius[nodes[first - 1]] <= maxRadius) {
      first -= 1;
    }
    // One node is a point, not a run. Every path `branchPaths` emits
    // is at least two long, so there is always a node to step back to.
    if (first === last) first -= 1;

    found.push({ nodes: nodes.slice(first) });
  }

  return found;
}
