import * as THREE from "three";

import { DEFAULT_ENVELOPE, type Envelope } from "../envelope";
import { transportFrames } from "../mesh/frames";
import type { RadiusField } from "../radius";
import { createRng } from "../rng";
import type { Skeleton } from "../skeleton/colonize";
import { shoots } from "./shoots";

/* ------------------------------------------------------------------ *
 * PLACEMENT
 *
 * Where every foliage element sits, which way it faces and how big it
 * is - and nothing else. This file never sees an element's vertices.
 * It emits transforms against the element's stated local frame -
 * petiole at the origin, axis along +Y, face normal along +Z - which
 * is the whole of the contract between the geometry and the placement,
 * and the reason the two are built at the same time rather than one
 * after the other.
 *
 * WHY NOT ONE ELEMENT PER ATTACHMENT FRAME. Because that is uniform
 * random distribution one level down, and it reads as spray paint. A
 * real canopy is organised: leaves come off a shoot in a spiral, and
 * the spiral has a constant divergence angle because that is what
 * stops a leaf sitting directly above the one two before it. The
 * golden angle is the limit of that argument and the default here,
 * but it is a parameter, because a whorled or decussate tree is a
 * different number in the same mechanism and not a different code
 * path.
 *
 * THREE THINGS DECIDE AN ELEMENT'S DIRECTION, and each is a dial:
 *   - the phyllotactic angle about the shoot, which is the spiral;
 *   - `outward`, which turns it away from the tree's own axis,
 *     because a leaf pointing back into the crown is a leaf in shade;
 *   - `upward`, which turns it toward the sky for the same reason.
 * Then `scatter` is the admission that no real leaf obeys any of it
 * exactly. Without it a canopy reads as a diagram of a canopy.
 *
 * CLUMPING AT THE END is the other half of "not one per frame". A
 * shoot's growing tip carries this season's leaves and they are
 * bunched there; the older wood behind it carries what is left. So
 * spacing walks the whole shoot and the clump is dropped into its
 * last stretch on top.
 *
 * CHANCE COMES FROM ONE SUB-STREAM, XOR-derived from the caller's
 * seed the way the bias field's is. Two things follow. The canopy
 * cannot be moved by anything else drawing a different number of
 * randoms, and adding it leaves the skeleton's stream and the noise
 * field's - which take the seed raw - byte for byte where they were.
 * Two trees standing on one stage carry two seeds, so their canopies
 * do not correlate.
 *
 * DETERMINISM IS ARRAY-BACKED ITERATION. There is no `Map` and no
 * `Set` anywhere below: shoots arrive in `branchPaths` order, stations
 * are built into an array in the order they are walked, and the
 * stream is drawn from in that one order. Hash order is how this kind
 * of thing fails once in twenty runs.
 * ------------------------------------------------------------------ */

/** The canopy's own stream, XOR-derived from the tree's seed. The bias
 *  field is the only other XOR-derived stream and it takes
 *  `0x5b_f0_3d_11`; the skeleton and the noise field take the caller's
 *  seed raw. A constant none of them uses is what leaves every stream
 *  that existed before the canopy emitting exactly what it emitted
 *  before. */
const CANOPY_STREAM = 0x2c_9e_1a_7f;

const TAU = Math.PI * 2;
const DEG = Math.PI / 180;

/** Below this a vector has no direction worth taking. */
const TINY = 1e-12;

/** Floors and ceilings on the dials. Each one is a value the mechanism
 *  cannot mean rather than a taste: spacing below the floor is an
 *  unbounded element count on a finite shoot, and a size variation of
 *  one is an element scaled to nothing. */
const MIN_SPACING = 1e-3;
/** A floor in metres as well as in fraction. The fraction alone bounds
 *  nothing when the envelope is short: at a height of zero it lets
 *  spacing collapse to nanometres and the element count run to the
 *  per-shoot cap on every shoot. */
const MIN_SPACING_METRES = 1e-4;
const MAX_CLUMP = 64;
const MAX_SCATTER = 90;
const MAX_SIZE_VARIATION = 0.9;
/** `size` multiplies vertices stored as float32. `held` catches a NaN
 *  but not a finite enormity: 1e39 through the multiply arrives as
 *  Infinity, the culler's fail-safe predicate then KEEPS every one of
 *  those elements, and the instanced mesh's bounds take the room's
 *  framing with them. The element caps its own metres for this reason;
 *  the multiplier needs the same. */
const MAX_SIZE = 1e3;

/** A stop, not a target. A shoot asking for more elements than this
 *  has been handed a spacing the panel should not be offering, and the
 *  honest failure is a thinner shoot rather than a hung tab. */
const MAX_PER_SHOOT = 512;

export interface CanopyParams {
  /** The wood at or below this fraction of the trunk's own radius
   *  bears foliage. Everything thicker is bark. Around 0.1 puts the
   *  canopy on the last few growth steps of every limb; at 1 the whole
   *  tree is a shoot, trunk included. Held to 0 through 1. */
  shootRadius: number;
  /** Distance along a shoot between successive elements, as a fraction
   *  of envelope height - like every other length in the library, so a
   *  148 m tree and a 24 m one come out in the same proportion. Held
   *  above a thousandth of the height and above a tenth of a
   *  millimetre, whichever is larger; a shoot that still asks for more
   *  stations than the per-shoot cap allows is thinned along its whole
   *  length rather than truncated at the tip. */
  spacing: number;
  /** The phyllotactic divergence angle, in degrees: how far round the
   *  shoot each element sits from the one before it. 137.508 is the
   *  golden angle, which is the spiral almost every plant uses; 180 is
   *  alternate, 90 decussate. */
  divergence: number;
  /** Elements gathered at the growing tip, on top of what `spacing`
   *  already puts there. This is the difference between a shoot that
   *  reads as evenly beaded and one that reads as a spray. Rounded to
   *  an integer and held to 0 through 64, and reserved out of the
   *  per-shoot budget before the walk spends it. */
  clump: number;
  /** The stretch at the tip the clump is gathered into, as a fraction
   *  of the shoot's own length. Held to 0 through 1. */
  clumpSpan: number;
  /** How far an element turns away from the tree's vertical axis, 0 to
   *  1. Zero leaves it sticking straight off the shoot. */
  outward: number;
  /** How far an element turns toward the sky, 0 to 1. */
  upward: number;
  /** Random spread about the direction the three terms above ask for,
   *  in degrees. Zero is a diagram; a canopy needs some. Held to 0
   *  through 90. */
  scatter: number;
  /** Multiplier on the element's own authored size. The element owns
   *  its absolute dimensions - a leaf is a leaf whatever the tree is
   *  doing, and a blade sized as a fraction of envelope height would
   *  make a 148 m tree carry 1.5 m fronds - so this stage scales what
   *  it is given rather than deciding how big a leaf is. 1 is the
   *  element at the size it was authored. Held to 0 through 1000: a
   *  finite enormity multiplies into float32 as Infinity, and the
   *  culler's fail-safe predicate keeps rather than drops those. */
  size: number;
  /** Random variation of that multiplier, 0 to 1: at 0.3 elements run
   *  from 70% to 130% of `size`. Held to 0 through 0.9, since 1 is an
   *  element scaled to nothing. */
  sizeVariation: number;
}

/** A canopy of ordinary leaves on an ordinary tree. Every term is a
 *  starting point rather than an answer - the two presets state their
 *  own - and the values are what the `held` rail falls back to when a
 *  panel sends through a NaN. */
export const DEFAULT_CANOPY: CanopyParams = {
  shootRadius: 0.12,
  spacing: 0.006,
  divergence: 137.508,
  clump: 5,
  clumpSpan: 0.3,
  outward: 0.6,
  upward: 0.35,
  scatter: 18,
  size: 1,
  sizeVariation: 0.35,
};

export interface Canopy {
  /** One 4x4 transform per element, column-major and packed end to
   *  end: 16 floats each, in `THREE.Matrix4.elements` order, which is
   *  what an `InstancedMesh` reads straight off. The library emits
   *  this and the consumer owns the draw. */
  matrices: Float32Array;
  /** How many elements are in `matrices`. */
  count: number;
}

/** A canopy with nothing in it. Built fresh each time rather than
 *  shared, so a caller that writes into one result cannot reach
 *  another's. */
const empty = (): Canopy => ({ matrices: new Float32Array(0), count: 0 });

const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

const clamp = (value: number, low: number, high: number): number =>
  Math.min(high, Math.max(low, value));

/**
 * Places one tree's foliage.
 *
 * Pure and deterministic in its arguments: the same skeleton, field,
 * envelope, seed and parameters give the same `matrices`, float for
 * float, on any machine. Neither the skeleton nor the radius field is
 * touched - the canopy is a stage of its own and the four before it
 * never call it, which is what keeps their existing comparisons valid.
 *
 * Returns an empty canopy, rather than throwing, for a skeleton with
 * no runs in it and for one whose wood is everywhere too thick to bear
 * foliage.
 */
export function buildCanopy(
  skeleton: Skeleton,
  field: RadiusField,
  envelope: Envelope,
  seed: number,
  params: CanopyParams,
): Canopy {
  const nodes = skeleton.nodes;
  if (nodes.length < 2 || field.radius.length < nodes.length) return empty();

  /* Every number that reaches the arithmetic is pinned to its own
     range first, and NaN is named separately because `Math.max`
     propagates it rather than clamping it. A NaN transform reaches the
     screen as an element that is silently not drawn. */
  const height = Math.max(
    1e-6,
    held(envelope.height, DEFAULT_ENVELOPE.height),
  );
  const trunkRadius = field.radius[0];
  const maxRadius =
    (Number.isFinite(trunkRadius) ? trunkRadius : 0) *
    clamp(held(params.shootRadius, DEFAULT_CANOPY.shootRadius), 0, 1);
  const spacing = Math.max(
    MIN_SPACING_METRES,
    Math.max(MIN_SPACING, held(params.spacing, DEFAULT_CANOPY.spacing)) *
      height,
  );
  const divergence =
    held(params.divergence, DEFAULT_CANOPY.divergence) * DEG;
  const clump = Math.round(
    clamp(held(params.clump, DEFAULT_CANOPY.clump), 0, MAX_CLUMP),
  );
  const clumpSpan = clamp(
    held(params.clumpSpan, DEFAULT_CANOPY.clumpSpan),
    0,
    1,
  );
  const outward = clamp(held(params.outward, DEFAULT_CANOPY.outward), 0, 1);
  const upward = clamp(held(params.upward, DEFAULT_CANOPY.upward), 0, 1);
  const scatter =
    clamp(held(params.scatter, DEFAULT_CANOPY.scatter), 0, MAX_SCATTER) * DEG;
  const size = clamp(held(params.size, DEFAULT_CANOPY.size), 0, MAX_SIZE);
  const sizeVariation = clamp(
    held(params.sizeVariation, DEFAULT_CANOPY.sizeVariation),
    0,
    MAX_SIZE_VARIATION,
  );

  const found = shoots(skeleton, field, maxRadius);
  if (found.length === 0) return empty();

  const rng = createRng((held(seed, 0) ^ CANOPY_STREAM) >>> 0);
  const out: number[] = [];

  /* Reused across every element. Allocating a Vector3 per leaf on a
     canopy of tens of thousands is the kind of garbage that shows up
     as a hitch on the build. */
  const point = new THREE.Vector3();
  const axis = new THREE.Vector3();
  const face = new THREE.Vector3();
  const side = new THREE.Vector3();
  const outDirection = new THREE.Vector3();
  const jitterAxis = new THREE.Vector3();
  const jitter = new THREE.Quaternion();
  const up = new THREE.Vector3(0, 1, 0);

  for (let s = 0; s < found.length; s += 1) {
    const shootNodes = found[s].nodes;

    const points: THREE.Vector3[] = [];
    for (let i = 0; i < shootNodes.length; i += 1) {
      points.push(nodes[shootNodes[i]].position);
    }
    const frames = transportFrames(points);

    /* Arc length to every node of the shoot. Elements are spaced along
       the wood rather than per node, so a shoot grown at a finer step
       does not come out with more leaves on it. */
    const along: number[] = [0];
    for (let i = 1; i < points.length; i += 1) {
      along.push(along[i - 1] + points[i].distanceTo(points[i - 1]));
    }
    const length = along[along.length - 1];
    if (!(length > 0)) continue;

    /* Stations first, in one array, in the order the stream will be
       drawn against them: the walk up the shoot, then the clump at its
       tip. The phyllotactic index runs across both, so the clump
       carries on the spiral instead of restarting it. */
    /* The clump is reserved out of the budget before the walk spends
       it, and the walk's step widens to cover the whole shoot rather
       than stopping partway up. Both are the same bug: stations
       accumulate from the base, so a cap applied to the loop condition
       truncates the DISTAL end - it strips the growing tip, which is
       where this season's leaves are and where the clump was going to
       go, and leaves the old wood behind it fully clothed. That is the
       exact inverse of what the shoot means, and it is reachable from
       the panel by widening `shootRadius` and tightening `spacing`.
       Saturation has to thin the whole shoot instead. */
    const walkBudget = Math.max(1, MAX_PER_SHOOT - clump);
    const step = Math.max(spacing, length / walkBudget);

    const stations: number[] = [];
    for (
      let distance = 0;
      distance < length && stations.length < walkBudget;
      distance += step
    ) {
      stations.push(distance);
    }
    for (let c = 0; c < clump && stations.length < MAX_PER_SHOOT; c += 1) {
      stations.push(length * (1 - clumpSpan * rng.next()));
    }

    for (let k = 0; k < stations.length; k += 1) {
      const distance = stations[k];

      // Which segment the station falls in, and how far along it.
      let segment = points.length - 2;
      while (segment > 0 && along[segment] > distance) segment -= 1;
      const span = along[segment + 1] - along[segment];
      const t = span > TINY ? (distance - along[segment]) / span : 0;

      point.copy(points[segment]).lerp(points[segment + 1], t);
      const wood =
        field.radius[shootNodes[segment]] * (1 - t) +
        field.radius[shootNodes[segment + 1]] * t;

      /* The spiral: the frame at the foot of the segment is the one
         "angle zero" is measured from. A leaf does not need a smoother
         basis than the wood it grows out of. */
      const frame = frames[segment];
      const turn = k * divergence;
      const cos = Math.cos(turn);
      const sin = Math.sin(turn);
      axis
        .copy(frame.normal)
        .multiplyScalar(cos)
        .addScaledVector(frame.binormal, sin);

      // The petiole sits on the wood's surface, not on its centreline.
      point.addScaledVector(axis, Number.isFinite(wood) ? wood : 0);

      // Out of the crown, and up toward the light.
      outDirection.set(point.x, 0, point.z);
      if (outDirection.lengthSq() > TINY) {
        axis.addScaledVector(outDirection.normalize(), outward);
      }
      axis.y += upward;
      if (axis.lengthSq() <= TINY) {
        axis
          .copy(frame.normal)
          .multiplyScalar(cos)
          .addScaledVector(frame.binormal, sin);
      }
      axis.normalize();

      /* The face turns toward the sky as far as it can without leaving
         the axis: world up with its axial component taken out. On a
         leaf already pointing straight up that is nothing, and the
         shoot's own tangent is the only remaining opinion. */
      face.copy(up).addScaledVector(axis, -axis.y);
      if (face.lengthSq() <= TINY) {
        face.copy(frame.tangent).addScaledVector(axis, -frame.tangent.dot(axis));
      }
      if (face.lengthSq() <= TINY) {
        face.copy(frame.normal).addScaledVector(axis, -frame.normal.dot(axis));
      }
      face.normalize();
      side.copy(axis).cross(face).normalize();

      /* One rotation about a uniformly drawn axis, up to `scatter`.
         Rotating the whole basis keeps it orthonormal, which a
         per-vector nudge would not. */
      if (scatter > 0) {
        const z = rng.range(-1, 1);
        const phi = rng.range(0, TAU);
        const ring = Math.sqrt(Math.max(0, 1 - z * z));
        jitterAxis.set(ring * Math.cos(phi), z, ring * Math.sin(phi));
        jitter.setFromAxisAngle(jitterAxis, scatter * rng.next());
        axis.applyQuaternion(jitter);
        face.applyQuaternion(jitter);
        side.applyQuaternion(jitter);
      }

      const scale = size * (1 + sizeVariation * rng.range(-1, 1));

      // Column-major, the order `InstancedMesh.instanceMatrix` reads.
      out.push(
        side.x * scale,
        side.y * scale,
        side.z * scale,
        0,
        axis.x * scale,
        axis.y * scale,
        axis.z * scale,
        0,
        face.x * scale,
        face.y * scale,
        face.z * scale,
        0,
        point.x,
        point.y,
        point.z,
        1,
      );
    }
  }

  return { matrices: Float32Array.from(out), count: out.length / 16 };
}
