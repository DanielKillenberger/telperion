import { DEFAULT_CULL, type CullParams } from "../canopy/cull";
import {
  DEFAULT_ENVELOPE,
  envelopeMaxRadius,
  envelopeRadiusAt,
  type Envelope,
} from "../envelope";
import type { Skeleton, SkeletonNode } from "./colonize";

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

/** How finely the authored profile is sampled into the curve depth is
 *  measured against - the culler's figure, for the culler's reason:
 *  the chord sits under the true curve by well under a millimetre. */
const PROFILE_SAMPLES = 128;

/* The profile curve and the distance to it restate `cull.ts` line for
   line rather than importing them, because the culler keeps them
   private; lifting both onto the envelope module is the follow-up that
   ends the duplicate, and until then the two are held equal by the
   test that runs the culler's fixture through this pass. */

/** The envelope's own profile curve as `r, y` pairs from the crown
 *  base to the tip, sampled from `envelopeRadiusAt` so there is one
 *  description of the shape. */
function envelopeProfile(envelope: Envelope): Float64Array {
  const out = new Float64Array((PROFILE_SAMPLES + 1) * 2);
  const base = envelope.height * envelope.crownBase;
  const span = envelope.height - base;
  for (let i = 0; i <= PROFILE_SAMPLES; i += 1) {
    const y = base + (span * i) / PROFILE_SAMPLES;
    out[i * 2] = envelopeRadiusAt(envelope, y);
    out[i * 2 + 1] = y;
  }
  return out;
}

/** Distance from `r, y` to the nearest point of `profile`, in metres. */
function distanceToProfile(
  profile: Float64Array,
  r: number,
  y: number,
): number {
  let best = Infinity;
  for (let i = 0; i + 3 < profile.length; i += 2) {
    const ar = profile[i];
    const ay = profile[i + 1];
    const br = profile[i + 2];
    const by = profile[i + 3];
    const dr = br - ar;
    const dy = by - ay;
    const lengthSq = dr * dr + dy * dy;
    let t = lengthSq > 0 ? ((r - ar) * dr + (y - ay) * dy) / lengthSq : 0;
    t = t < 0 ? 0 : t > 1 ? 1 : t;
    const er = r - (ar + dr * t);
    const ey = y - (ay + dy * t);
    const distance = Math.hypot(er, ey);
    if (distance < best) best = distance;
  }
  return best;
}

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
  skeleton: Skeleton,
  from: number,
  envelope: Envelope,
  params: CullParams = DEFAULT_SHED,
): Skeleton {
  const nodes = skeleton.nodes;
  const first = Math.max(1, Math.floor(held(from, nodes.length)));
  if (first >= nodes.length) return { nodes: nodes.slice() };

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
  return { nodes: out };
}
