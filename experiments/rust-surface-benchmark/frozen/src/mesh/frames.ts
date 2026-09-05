import * as THREE from "three";

/* ------------------------------------------------------------------ *
 * PARALLEL TRANSPORT FRAMES
 *
 * A swept surface needs an orientation at every point of the path it
 * follows: the tangent says which way the branch is going, and some
 * pair of perpendiculars says where "angle zero" of the cross section
 * sits. The tangent is forced. The perpendiculars are not, and the
 * choice between them is the difference between a surface that reads
 * as one continuous limb and one that twists visibly for no reason.
 *
 * The naive answer is a fresh basis per point, built by crossing the
 * tangent with a fixed world axis. That is what the fn-11.4 viewer
 * did, and it is stable only while the branch keeps pointing roughly
 * the same way: the moment the tangent crosses the reference axis the
 * whole ring spins, and a limb that bends past vertical shows a hard
 * twist at the crossing.
 *
 * The other textbook answer is the Frenet frame - normal along the
 * curvature vector, binormal perpendicular to both. It is worse. At
 * an inflection the curvature vector passes through zero and flips to
 * the other side, so the frame flips with it, and the surface shows a
 * 180-degree kink at exactly the place fn-11.3's writhe puts the most
 * of them.
 *
 * So: parallel transport. Choose one perpendicular at the start, and
 * at every later point rotate the previous one by the smallest
 * rotation that carries the previous tangent onto the current one.
 * That rotation has no component about the tangent, so the frame
 * accumulates no roll of its own - the only rotation the cross section
 * ever gets about its own axis is the one the surface parameters ask
 * for. It has no singularity, no inflection to trip over, and it is a
 * pure function of the points, which is what keeps the mesh
 * deterministic in the seed.
 * ------------------------------------------------------------------ */

export interface Frame {
  /** Unit vector along the path. */
  tangent: THREE.Vector3;
  /** Unit vector perpendicular to the tangent: the cross section's
   *  angle zero. Carried from the previous frame, never rebuilt. */
  normal: THREE.Vector3;
  /** `tangent` cross `normal`. Completes a right-handed basis. */
  binormal: THREE.Vector3;
}

/** Below this the two directions handed to a rotation are the same
 *  direction and there is nothing to rotate. */
const FLAT = 1e-9;

/** The world axis `direction` is least aligned with. Crossing with the
 *  one it points most nearly along would be a degenerate basis, and
 *  picking by magnitude is deterministic where picking by a first
 *  success is not. */
function leastAligned(direction: THREE.Vector3): THREE.Vector3 {
  const ax = Math.abs(direction.x);
  const ay = Math.abs(direction.y);
  const az = Math.abs(direction.z);
  if (ax <= ay && ax <= az) return new THREE.Vector3(1, 0, 0);
  if (ay <= az) return new THREE.Vector3(0, 1, 0);
  return new THREE.Vector3(0, 0, 1);
}

/**
 * One frame per point of `points`, carried along the path.
 *
 * Tangents at the interior points are the mean of the two segments
 * meeting there rather than either one of them, so a ring sits square
 * to the corner it is on - the miter, not the segment before it. That
 * is what lets consecutive segments share a ring at all: one ring, one
 * orientation, both segments agreeing on it.
 *
 * Pure and deterministic. Returns an empty array for fewer than two
 * points, which is not a path.
 */
export function transportFrames(points: readonly THREE.Vector3[]): Frame[] {
  const count = points.length;
  if (count < 2) return [];

  /* Segment directions first: there are count-1 of them, and every
     tangent is built out of them. A zero-length segment cannot give a
     direction, so it inherits the one before it; the caller is
     expected to have dropped repeated points, and this is the rail
     under that rather than a second opinion about it. */
  const segments: THREE.Vector3[] = [];
  for (let i = 0; i < count - 1; i += 1) {
    const step = points[i + 1].clone().sub(points[i]);
    if (step.lengthSq() > 0) segments.push(step.normalize());
    else segments.push((segments[i - 1] ?? new THREE.Vector3(0, 1, 0)).clone());
  }

  const tangents: THREE.Vector3[] = [];
  for (let i = 0; i < count; i += 1) {
    if (i === 0) tangents.push(segments[0].clone());
    else if (i === count - 1) tangents.push(segments[count - 2].clone());
    else {
      const mean = segments[i - 1].clone().add(segments[i]);
      // A dead reversal has no bisector; the outgoing segment is the
      // only defensible answer and it keeps the frame moving forward.
      tangents.push(
        mean.lengthSq() > FLAT ? mean.normalize() : segments[i].clone(),
      );
    }
  }

  const frames: Frame[] = [];
  let normal = leastAligned(tangents[0]).cross(tangents[0]).normalize();
  const rotation = new THREE.Quaternion();

  for (let i = 0; i < count; i += 1) {
    const tangent = tangents[i];
    if (i > 0) {
      rotation.setFromUnitVectors(tangents[i - 1], tangent);
      normal = normal.clone().applyQuaternion(rotation);
    }
    /* Re-square it against the tangent every step. The rotation is
       exact in theory; in floating point a few hundred of them in a
       row accumulate a drift out of the perpendicular plane, and a
       ring built on a normal that is not perpendicular is an ellipse
       that grows down the branch. */
    normal.addScaledVector(tangent, -normal.dot(tangent));
    if (normal.lengthSq() <= FLAT) {
      normal = leastAligned(tangent).cross(tangent);
    }
    normal.normalize();
    frames.push({
      tangent: tangent.clone(),
      normal: normal.clone(),
      binormal: tangent.clone().cross(normal).normalize(),
    });
  }

  return frames;
}
