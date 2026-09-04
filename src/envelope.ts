import * as THREE from "three";

import type { Rng } from "./rng";

/* ------------------------------------------------------------------ *
 * THE AUTHORED SILHOUETTE
 *
 * This is the half of the generator that is a design decision rather
 * than a generated one, and the inversion is the whole point of the
 * spec's approach. Recursive branching emits a silhouette as a side
 * effect and it is always a blob. Here the silhouette is stated first
 * and the branching is what gets generated inside it.
 *
 * It is a solid of revolution: one radius profile turned about the
 * trunk axis. Two half-superellipses meet at the widest point, so
 * `shoulder` alone carries the difference the references keep showing
 * between the two trees - low is a pointed, upright Telperion, high is
 * a broad, square-shouldered, domed Laurelin. fn-11.7 is where those
 * two envelopes get authored; this is the shape language they are
 * written in.
 *
 * Nothing here knows about branches, seeds, materials or scenes. It
 * answers one question - is this point inside, and how wide is the
 * tree at this height - and scatters attractors in the volume.
 * ------------------------------------------------------------------ */

export interface Envelope {
  /** Total height in metres, ground to crown tip. */
  height: number;
  /** Fraction of `height` below which there is no crown: bare trunk. */
  crownBase: number;
  /** Widest half-width as a fraction of `height`. */
  spread: number;
  /** Where the widest point sits within the crown, 0 at its base and
   *  1 at the tip. Low is bottom-heavy and spreading, high is a crown
   *  that carries its mass up top. */
  fullness: number;
  /** Profile exponent. 1 is a straight-sided cone, 2 an ellipse, and
   *  above that the shoulders square off into a dome. */
  shoulder: number;
}

/** The shape everything else is a departure from: a 24 m tree with a
 *  bare lower third and a crown that is widest just below its middle.
 *  Not either of the Two Trees - those are fn-11.7's to author. */
export const DEFAULT_ENVELOPE: Envelope = {
  height: 24,
  crownBase: 0.3,
  spread: 0.3,
  fullness: 0.45,
  shoulder: 2.2,
};

/** How many rejection-sampling attempts each requested point is
 *  allowed. The envelope fills roughly a third of its bounding prism,
 *  so this is slack of about twenty times over - it exists to bound a
 *  degenerate envelope, not to be reached. */
const ATTEMPTS_PER_POINT = 64;

/** Half-width at the widest point, in metres. */
export function envelopeMaxRadius(envelope: Envelope): number {
  return envelope.height * envelope.spread;
}

/** Half-width at height `y` metres, and zero everywhere outside the
 *  crown - below its base, above the tip, or on a degenerate
 *  envelope. Containment and sampling are both defined by this one
 *  function, so there is no second description of the shape to drift
 *  from the first. */
export function envelopeRadiusAt(envelope: Envelope, y: number): number {
  const base = envelope.height * envelope.crownBase;
  const span = envelope.height - base;
  if (span <= 0) return 0;

  const t = (y - base) / span;
  if (t <= 0 || t >= 1) return 0;

  const maxRadius = envelopeMaxRadius(envelope);
  if (maxRadius <= 0) return 0;

  // Guarded away from the ends so the two halves below both have a
  // non-zero run to cover; a widest point exactly at the crown base or
  // exactly at the tip is a degenerate profile, not a useful one.
  const fullness = Math.min(0.999, Math.max(0.001, envelope.fullness));
  const shoulder = Math.max(0.1, envelope.shoulder);

  if (t < fullness) {
    const u = t / fullness;
    return maxRadius * Math.pow(1 - Math.pow(1 - u, shoulder), 1 / shoulder);
  }
  const v = (t - fullness) / (1 - fullness);
  return maxRadius * Math.pow(1 - Math.pow(v, shoulder), 1 / shoulder);
}

/** Whether `point` lies in the solid, within `tolerance` metres.
 *
 *  The height check is not redundant with the radius one. The radius is
 *  zero on the axis outside the crown, so a containment test written as
 *  a radius comparison alone answers "inside" for the entire trunk axis
 *  extended to infinity - the ground beneath the tree and the sky above
 *  its tip included. */
export function envelopeContains(
  envelope: Envelope,
  point: THREE.Vector3,
  tolerance = 0,
): boolean {
  if (point.y < -tolerance || point.y > envelope.height + tolerance) {
    return false;
  }
  const radius = envelopeRadiusAt(envelope, point.y) + tolerance;
  return Math.hypot(point.x, point.z) <= radius;
}

/**
 * Scatters up to `count` attractor points uniformly through the
 * envelope's volume, drawing from `rng` and nothing else.
 *
 * Rejection sampling against the bounding prism, which is uniform in
 * the volume by construction. Sampling the profile directly would be
 * faster and would bunch points wherever the profile is narrow, which
 * is exactly the bias that makes a crown look like it was drawn by an
 * algorithm.
 *
 * Returns fewer than `count` points only when the attempt budget runs
 * out, which in practice means the envelope has no volume to fill.
 */
export function sampleEnvelope(
  envelope: Envelope,
  count: number,
  rng: Rng,
): THREE.Vector3[] {
  const points: THREE.Vector3[] = [];
  if (count <= 0) return points;

  const maxRadius = envelopeMaxRadius(envelope);
  const base = envelope.height * envelope.crownBase;
  if (maxRadius <= 0 || envelope.height <= base) return points;

  const attempts = count * ATTEMPTS_PER_POINT;
  for (let i = 0; i < attempts && points.length < count; i += 1) {
    const x = rng.range(-maxRadius, maxRadius);
    const y = rng.range(base, envelope.height);
    const z = rng.range(-maxRadius, maxRadius);
    const radius = envelopeRadiusAt(envelope, y);
    if (x * x + z * z <= radius * radius) {
      points.push(new THREE.Vector3(x, y, z));
    }
  }
  return points;
}
