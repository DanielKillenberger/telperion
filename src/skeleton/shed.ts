import { DEFAULT_CULL, type CullParams } from "../canopy/cull";
import { DEFAULT_ENVELOPE, distanceToProfile, envelopeMaxRadius, envelopeProfile, envelopeRadiusAt, type Envelope } from "../envelope";
import type { Skeleton, SkeletonNode } from "./colonize";
import type { TwiggedSkeleton } from "./twigs";

/* ------------------------------------------------------------------ *
 * SHEDDING: THE SHELL RULE, ONE LEVEL UP
 *
 * A real crown is a shell because twigs in deep shade die, and a
 * generated one that keeps every twig it grew is carrying, and then
 * skinning and leafing, the wood the camera never sees. The leaf
 * culler already measures how far under the authored envelope a leaf
 * sits and thins the interior by it; this is the same rule applied to
 * the twigs the local pass appended, at the same cost, before the
 * radius solve and the surface ever see them. Not a light-competition
 * simulation - the shell this library already has, applied to wood.
 *
 * ONLY TWIGS ARE SHED. Colonization's nodes are the structure that
 * fills the envelope and reaches for light; shedding those would move
 * the outline, which is the whole of what the eye reads a tree by. The
 * twig pass appends after the nodes it was given, so "a twig" is an
 * index at or past where the first pass ended.
 *
 * A TWIG SURVIVES ON THE FIRST NODE OF ITS SUBTREE THAT IS NOT
 * DEFINITELY INTERIOR - the culler's every-vertex rule, with a node's
 * descendants standing in for a blade's vertices. A twig that starts
 * inside the shell and grows out to the light is outline; classifying
 * it by its base would remove exactly the boundary case the
 * conservatism argument is about. And a kept node keeps its parent,
 * which is what keeps `parent < self` and every parent present.
 *
 * THE PREDICATE FAILS SAFE, as the culler's does: `!(depth > shell)`
 * keeps, so a NaN from a degenerate envelope or a non-finite position
 * keeps a twig rather than deleting it; and where the envelope has no
 * width - below the crown base, above the tip - the profile's radius
 * is zero, depth is at most zero, and nothing is shed. A shell of the
 * crown's whole half-width sheds nothing; a shell of zero sheds every
 * twig not poking through the silhouette, and that is the reason the
 * default is the culler's own measured shell and not zero.
 * ------------------------------------------------------------------ */

/** The shell twigs are kept in: the leaf culler's, by construction.
 *  One rule, one number - a twig shed at a different depth from the
 *  leaves it would have borne is two opinions about where the crown's
 *  interior begins. */
export const DEFAULT_SHED: CullParams = DEFAULT_CULL;

/* NaN is the one that has to be named separately - `Math.max`
   propagates it rather than clamping it. */
const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

/**
 * Removes the twigs of `skeleton` that sit deep inside `envelope`.
 *
 * `from` is the index the twig pass started appending at: every node
 * below it is colonization's and is kept whatever its depth. Pure -
 * neither argument is touched - and the result carries the kept nodes
 * in their original order with parents re-indexed, so the skeleton
 * that comes back is one skeleton with one parent invariant and no
 * downstream stage can tell a twig was ever there.
 *
 * A skeleton with no twigs, a degenerate envelope, or a non-finite
 * position all come back with their nodes intact rather than emptied.
 */
export function shedTwigs(
  skeleton: Skeleton | TwiggedSkeleton,
  from: number,
  envelope: Envelope,
  params: CullParams = DEFAULT_SHED,
): TwiggedSkeleton {
  const nodes = skeleton.nodes;
  const records = "branchId" in skeleton ? skeleton : undefined;
  const status = { levelCapped: records?.levelCapped ?? false,
    nodeCapped: records?.nodeCapped ?? false,
    ...(records?.refused ? { refused: records.refused } : {}) };

  const first = Math.max(1, Math.floor(held(from, nodes.length)));
  /* Shedding removes nodes at or after `first` only, so the colonization
     prefix is intact and the crossover the thickness solve keys on is
     exactly `first`. It is carried through rather than dropped: a plain
     `{ nodes }` here made the solve read the whole tree as limb. */
  if (first >= nodes.length) return { nodes: nodes.slice(), crossover: nodes.length, branchId: new Int32Array(), baseRadius: new Float64Array(), twig: new Uint8Array(), ...status };

  const maxRadius = envelopeMaxRadius(envelope);
  const shell =
    Math.max(0, held(params.shellDepth, DEFAULT_SHED.shellDepth)) *
    (Number.isFinite(maxRadius)
      ? maxRadius
      : envelopeMaxRadius(DEFAULT_ENVELOPE));
  const profile = envelopeProfile(envelope);

  /* Which twigs are definitely interior, on their own position. The
     radial slack is the cheap upper bound on depth - the surface is
     never further away than straight out sideways - so a node the
     slack alone keeps never pays for the curve. */
  const keep = new Uint8Array(nodes.length);
  for (let i = 0; i < first; i += 1) keep[i] = 1;
  for (let i = first; i < nodes.length; i += 1) {
    const { x, y, z } = nodes[i].position;
    const r = Math.hypot(x, z);
    const slack = envelopeRadiusAt(envelope, y) - r;
    if (!(slack > shell) || !(distanceToProfile(profile, r, y) > shell)) {
      keep[i] = 1;
    }
  }
  /* A kept node keeps its line back to the wood it grew from. Parents
     precede children, so one backward pass carries "kept" up every
     lineage before the lineage's own base is decided. */
  for (let i = nodes.length - 1; i >= first; i -= 1) {
    if (keep[i] === 1) keep[nodes[i].parent] = 1;
  }

  const index = new Int32Array(nodes.length).fill(-1);
  const out: SkeletonNode[] = [];
  for (let i = 0; i < nodes.length; i += 1) {
    if (keep[i] === 0) continue;
    index[i] = out.length;
    const node = nodes[i];
    out.push(
      i < first ? node : { position: node.position, parent: index[node.parent] },
    );
  }
  const branchId = new Int32Array(out.length - first);
  const baseRadius = new Float64Array(out.length - first);
  const twig = new Uint8Array(out.length - first);
  for (let i = first; i < nodes.length; i++) {
    if (index[i] < 0) continue;
    const at = index[i] - first;
    const source = i - (records?.crossover ?? first);
    branchId[at] = records ? index[records.branchId[source]] : index[i];
    baseRadius[at] = records?.baseRadius[source] ?? 0;
    twig[at] = records?.twig[source] ?? 0;
  }
  return { nodes: out, crossover: first, branchId, baseRadius, twig, ...status };
}
