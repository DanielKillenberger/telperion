import {
  DEFAULT_ENVELOPE,
  envelopeMaxRadius,
  envelopeRadiusAt,
  type Envelope,
} from "../envelope";
import type { ElementMesh } from "./element";
import type { Canopy } from "./place";

/* ------------------------------------------------------------------ *
 * CULLING TO A SHELL
 *
 * A real tree sheds the leaves that never see light, and a generated
 * one that fills its crown solid is paying for the most expensive
 * elements it owns: the ones the camera cannot see. Thinning the
 * interior is free performance right up to the moment it eats the
 * outline - and the outline is the whole of what the eye reads a tree
 * by - so everything below leans one way. An element is removed only
 * on a definite verdict that it is deep inside; anything undecidable
 * is kept.
 *
 * THE SHELL IS THE AUTHORED ENVELOPE, offset inward. `envelopeRadiusAt`
 * already answers "how wide is the tree at this height", it is the
 * function containment and attractor sampling are both defined by, and
 * a second notion of the same surface would be free to drift from the
 * first. So the shape is not restated here: it is sampled into the
 * profile curve of the same solid of revolution and depth is the
 * distance from a point to that curve, in the half-plane the profile
 * is drawn in.
 *
 * DEPTH IS DISTANCE TO THE SURFACE, NOT SLACK IN THE RADIUS, and the
 * difference is most of what this file gets right. A radial reading
 * calls a leaf hanging under the middle of the crown deeply interior,
 * because the crown is wide there - and then culling it lifts the
 * underside of the tree, which is outline, in full view, and exactly
 * what must not move. Measured to the surface, that leaf is a metre
 * under the crown's own belly and stays. The same argument holds
 * upside down under the dome. Measured on Telperion at one shell
 * depth, reading depth to the surface rather than to the radius
 * halved what culling cost the outline - 15.5% of the silhouette down
 * to 8.0% - and that is what lets the shell be shallow enough to
 * remove anything at all.
 *
 * EVERY VERTEX, NEVER THE CENTROID. An element is a blade with real
 * extent, and a blade straddling the shell boundary has its centre
 * inside it and its tip out in the light. Classifying by the centre
 * removes exactly the elements that are half on the silhouette - the
 * boundary case, which is the one the whole conservatism argument is
 * about. So every vertex of the element is transformed by that
 * element's own instance matrix and the element survives on the first
 * one that is not definitely interior.
 *
 * THE PREDICATE IS WRITTEN TO FAIL SAFE. `!(depth > shell)` keeps,
 * rather than `depth <= shell` keeps, and the difference is every case
 * where the arithmetic produced a NaN: a non-finite transform, a
 * degenerate envelope, a height the profile has no answer for. NaN
 * compares false against everything, so the negated form keeps those
 * elements and the plain form would silently delete them. An empty
 * canopy is a bug that reaches the screen as nothing at all, which is
 * the hardest kind to see.
 *
 * WHERE THE ENVELOPE HAS NO WIDTH IT CULLS NOTHING. Below the crown
 * base and above the tip the profile's radius is zero, so depth there
 * is at most zero and every element is kept. That is deliberate and it
 * is the same lesson the skeleton learned the hard way: a predicate
 * written only in terms of a width is vacuous exactly where the width
 * is zero, and the only safe direction for it to be vacuous in is the
 * one that keeps geometry.
 * ------------------------------------------------------------------ */

export interface CullParams {
  /** How deep beneath the authored envelope foliage is kept, as a
   *  fraction of the envelope's widest half-width - so a 148 m tree
   *  and a 24 m one keep a shell of the same proportion. Everything
   *  deeper than this, by every one of its vertices, is interior and
   *  goes.
   *
   *  Zero keeps only what pokes through the envelope itself, and is
   *  too aggressive to be honest: the branches do not reach the
   *  authored surface everywhere, so the outline is drawn by foliage
   *  sitting some way under it. At 1 the shell is as thick as the
   *  crown is wide and nothing is ever removed. */
  shellDepth: number;
}

/** The shell every canopy is thinned to unless a preset says
 *  otherwise: measured, not guessed.
 *
 *  It is the shallowest shell that leaves the silhouette inside the
 *  raster's own resolution - `silhouettePixelFloor`, one pixel at each
 *  end of every column - from all six directions the canopy is judged
 *  from, on both presets, at their own density and at twice it. On the
 *  sparse end the nearest shallower step measured, 0.4, comes within
 *  two percent of that floor on one view of Telperion, which is a
 *  margin a change to placement would erase; 0.45 clears it by two and
 *  a half times.
 *
 *  What it buys is honest and worth stating: about a sixth of the
 *  elements on a canopy that fills its envelope, and very little on an
 *  open one. Placing foliage on distal shoots already puts most of it
 *  near the crown's surface, so a well-placed canopy is most of the
 *  way to being a shell before this file sees it. The elements this
 *  removes are the ones that grew on inner wood - the botanically
 *  wrong ones, and the ones the camera never sees. */
export const DEFAULT_CULL: CullParams = {
  shellDepth: 0.45,
};

/* NaN is the one that has to be named separately - `Math.max`
   propagates it rather than clamping it. Deliberately local: the same
   rail appears in radius.ts, surface.ts, element.ts and place.ts, and
   it is duplicated per call site rather than shared. */
const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

/** A canopy with nothing in it, built fresh each time so a caller that
 *  writes into one result cannot reach another's. */
const empty = (): Canopy => ({ matrices: new Float32Array(0), count: 0 });

/** How finely the authored profile is sampled into the curve depth is
 *  measured against. The profile is smooth and this is a chord
 *  approximation of it: at 128 samples the chord sits under the true
 *  curve by well under a millimetre on either preset, which is three
 *  orders below the shell it is used to measure. */
const PROFILE_SAMPLES = 128;

/** The envelope's own profile curve: the outline of the solid of
 *  revolution in the half-plane it is turned in, as `r, y` pairs from
 *  the crown base up to the tip.
 *
 *  Sampled from `envelopeRadiusAt` rather than derived a second time,
 *  so there is exactly one description of the shape in the library and
 *  a degenerate envelope produces a degenerate curve here rather than
 *  a disagreement. */
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

/** Distance from `r, y` to the nearest point of `profile`, in metres.
 *
 *  Plain minimum over the chords: the profile has 128 of them and this
 *  runs per vertex, but the caller settles the common case on the
 *  radial slack alone and only reaches here for a point that is a
 *  candidate for removal. */
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

/** How many whole instances a canopy actually carries: the smaller of
 *  what it claims and what its buffer holds. */
function instanceCount(canopy: Canopy): number {
  const claimed = Number.isFinite(canopy.count) ? Math.floor(canopy.count) : 0;
  return Math.max(0, Math.min(claimed, Math.floor(canopy.matrices.length / 16)));
}

/**
 * Removes the elements of `canopy` that sit deep inside the envelope.
 *
 * Pure: neither argument is touched, and the result is a fresh buffer
 * carrying the kept transforms in their original order, float for
 * float, so a culled canopy is as deterministic as the canopy it came
 * from.
 *
 * An element is removed only when every one of its vertices, under
 * that element's own instance transform, is definitely deeper than the
 * shell. A canopy with no elements, a degenerate envelope, or a
 * transform carrying a non-finite number all come back with their
 * geometry intact rather than emptied.
 */
export function cullCanopy(
  canopy: Canopy,
  element: ElementMesh,
  envelope: Envelope,
  params: CullParams,
): Canopy {
  const count = instanceCount(canopy);
  if (count === 0) return empty();

  const positions = element.positions;
  const vertices = (positions.length / 3) | 0;
  if (vertices === 0) {
    return {
      matrices: canopy.matrices.slice(0, count * 16),
      count,
    };
  }

  const maxRadius = envelopeMaxRadius(envelope);
  const shell =
    Math.max(0, held(params.shellDepth, DEFAULT_CULL.shellDepth)) *
    (Number.isFinite(maxRadius)
      ? maxRadius
      : envelopeMaxRadius(DEFAULT_ENVELOPE));

  const profile = envelopeProfile(envelope);
  const out = new Float32Array(count * 16);
  const m = canopy.matrices;
  let kept = 0;

  for (let i = 0; i < count; i += 1) {
    const o = i * 16;
    let keep = false;

    for (let v = 0; v < vertices; v += 1) {
      const x = positions[v * 3];
      const y = positions[v * 3 + 1];
      const z = positions[v * 3 + 2];

      const wx = m[o] * x + m[o + 4] * y + m[o + 8] * z + m[o + 12];
      const wy = m[o + 1] * x + m[o + 5] * y + m[o + 9] * z + m[o + 13];
      const wz = m[o + 2] * x + m[o + 6] * y + m[o + 10] * z + m[o + 14];

      /* How far under the authored silhouette this vertex sits. The
         radial slack is the cheap upper bound on it - the surface is
         never further away than straight out sideways - so a vertex
         the slack alone already keeps never pays for the curve. */
      const r = Math.hypot(wx, wz);
      const slack = envelopeRadiusAt(envelope, wy) - r;
      if (!(slack > shell)) {
        keep = true;
        break;
      }
      if (!(distanceToProfile(profile, r, wy) > shell)) {
        keep = true;
        break;
      }
    }

    if (!keep) continue;
    out.set(m.subarray(o, o + 16), kept * 16);
    kept += 1;
  }

  return { matrices: out.slice(0, kept * 16), count: kept };
}
