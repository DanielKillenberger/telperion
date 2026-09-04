import * as THREE from "three";

import { DEFAULT_ENVELOPE, type Envelope } from "@/lib/grower/envelope";
import { transportFrames } from "@/lib/grower/mesh/frames";
import { branchPaths } from "@/lib/grower/mesh/paths";
import type { RadiusField } from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";

/* ------------------------------------------------------------------ *
 * THE SURFACE
 *
 * One swept skin over the whole tree. Three things are being asked of
 * it and they are independent of each other:
 *
 *   CONTINUITY. The surface has no gaps, anywhere, at any bend. It is
 *   swept along runs (paths.ts) with one ring per node shared by the
 *   segment below and the segment above, oriented by a transported
 *   frame (frames.ts), with the radius interpolated between rings
 *   rather than stepped per segment. Every run is closed at both ends,
 *   so the mesh is a union of closed shells with no boundary edge in
 *   it - a hole is not a thing this can produce, rather than a thing
 *   it has been tuned not to produce. Forks are the hard half: a child
 *   run starts sunk back INSIDE its parent's volume and swollen where
 *   it leaves, so it emerges through the parent's skin instead of
 *   butting against it. No boolean, no stitching, and nothing that can
 *   pinch or invert.
 *
 *   THE PLAITED SECTION. The cross section is not a circle - it is a
 *   lobed profile, and the profile rotates about the branch axis as
 *   the sweep advances. That is the braided, rope-like trunk in the
 *   Garland reference, and it is the reason to sweep a section at all
 *   rather than extrude a tube. It is the SURFACE twisting, and it is
 *   a different mechanism from fn-11.3's growth bias, which bends the
 *   CENTRELINE. Both are required and neither substitutes for the
 *   other: a trunk with spiral bias and a circular section is a bent
 *   pipe, and a straight trunk with a rotating section is a drill bit.
 *
 *   THE ROOT FLARE. A trunk that meets the ground at its own trunk
 *   radius reads as a pole stuck in soil. Real trunks widen into their
 *   roots, and the widening is what makes a tree look like it is
 *   holding itself up.
 *
 * Every quality above is a named parameter with a documented range,
 * because fn-11.7 authors Telperion and Laurelin as two of these and
 * anything constant in here is a difference between the two trees that
 * cannot be authored.
 *
 * Radii are the fn-11.4 field's, unchanged: the lobes modulate around
 * it and its mean over the section is exactly the field's radius. The
 * only other thing that touches a radius is the flare, which is a
 * stated multiplier that decays to nothing above the ground.
 *
 * Pure and deterministic in its arguments. No seed reaches here.
 * ------------------------------------------------------------------ */

export interface SurfaceParams {
  /** Vertices around one cross section. This is the surface's
   *  resolution and its cost: triangles scale with it exactly. Below
   *  about 8 a lobed section reads as a faceted prism; above about 20
   *  a twig is spending vertices on a shape a millimetre across. 12 is
   *  a trunk that reads as round-ish and braided beside a 1.8 m
   *  figure. Rounded to an integer, held to 3 or more. */
  radialSegments: number;
  /** How many lobes the cross section has: the number of strands the
   *  limb reads as. 0 is a circle - the thing every other procedural
   *  tree extrudes. 3 to 7 is where a trunk reads as plaited rather
   *  than as merely dented. Rounded to an integer. */
  lobes: number;
  /** How deep the lobes cut, as a fraction of the radius: 0.1 is a
   *  softly fluted trunk, 0.3 is rope. The section's mean radius is
   *  the radius solve's whatever this is, so deepening the lobes does
   *  not fatten the tree. Held below 1, where a lobe would reach the
   *  centreline and the section would fold through itself. */
  lobeDepth: number;
  /** Turns of the cross section about its own axis over the envelope's
   *  full height - the rate the plait winds at. Signed: negative winds
   *  the other way. Around 1 to 3 reads as a slow braid. It is sampled
   *  once per growth step, so past about 6 the winding turns faster
   *  than the skeleton is subdivided and reads as chatter rather than
   *  as a twist.
   *
   *  This is the SURFACE's rotation. `BiasParams.spiralRate` is the
   *  centreline's. They are separate mechanisms with separate dials on
   *  purpose. */
  twistRate: number;
  /** How much wider the trunk is where it meets the ground, as a
   *  multiple of its radius there. 1 is no flare at all; 2 to 3 is a
   *  trunk that spreads into its roots. Held at 1 or above - a base
   *  narrower than the trunk is not a flare. */
  flareRadius: number;
  /** The height over which the flare decays, as a fraction of the
   *  envelope's height, in e-foldings: at 0.02 on a 24 m tree the
   *  flare is gone within about a metre and a half. Small is a sharp
   *  buttress, large is a trunk that is conical for its whole lower
   *  half. Stated against the ground plane rather than along the
   *  trunk, because it is what happens where wood meets earth. */
  flareFalloff: number;
  /** How far the flared base sinks below the ground, as a fraction of
   *  the envelope's height. It exists so the trunk's end cap is buried
   *  rather than showing as a flat disc at y=0, so anything above zero
   *  does the job; larger only wastes triangles underground. */
  flareDepth: number;
  /** How far back into its parent a child run starts, in parent radii
   *  at the fork. This is what makes a fork continuous: the child's
   *  first ring is inside the parent's solid, so the child emerges
   *  through the parent's skin instead of meeting it. 0 starts the
   *  child exactly on the parent's centreline, which is already inside
   *  it; above about 1 the child starts behind the parent's surface on
   *  the far side and pokes out backwards. */
  forkSocket: number;
  /** How much wider a child is where it leaves the fork, as a multiple
   *  of its own radius there. This is the fillet: real branches thicken
   *  into the limb they leave. It decays over a distance of one parent
   *  radius, so a junction's fillet is scaled by the junction's own
   *  size rather than by a length in metres. 1 is no fillet. */
  forkSwell: number;
}

/** The surface everything else is a departure from: a twelve-sided
 *  five-lobed section winding one and a half turns over the tree's
 *  height, on a trunk that doubles its width into the ground. Not
 *  either of the Two Trees - fn-11.7 authors those. */
export const DEFAULT_SURFACE: SurfaceParams = {
  radialSegments: 12,
  lobes: 5,
  lobeDepth: 0.16,
  twistRate: 1.5,
  flareRadius: 2.1,
  flareFalloff: 0.022,
  flareDepth: 0.004,
  forkSocket: 0.5,
  forkSwell: 1.35,
};

/** Rails on the parameters, none of them art direction. Each one is
 *  the boundary past which the arithmetic stops describing a surface:
 *  a section needs three sides to enclose anything, a lobe at or past
 *  the full radius folds the section through its own centreline, and a
 *  flare below 1 is a pinch. */
const MIN_RADIAL_SEGMENTS = 3;
const MAX_RADIAL_SEGMENTS = 64;
const MAX_LOBES = 24;
const MAX_LOBE_DEPTH = 0.9;
const MIN_FLARE_FALLOFF = 1e-4;

const TWO_PI = Math.PI * 2;

/** A closed triangle soup, in the form three.js wants it. Positions
 *  and indices only: normals come from the winding, which is uniform
 *  and outward everywhere, and materials are the consumer's business. */
export interface SurfaceMesh {
  /** xyz per vertex, in metres. */
  positions: Float32Array;
  /** Three vertex indices per triangle, wound counter-clockwise seen
   *  from outside the surface. */
  indices: Uint32Array;
  triangles: number;
  vertices: number;
}

/** One ring's worth of the sweep: where it sits, how wide it is, and
 *  how far along the tree it is (which is what the twist is a function
 *  of, so that the plait carries on through a fork instead of
 *  restarting on every limb). */
interface Sample {
  position: THREE.Vector3;
  radius: number;
  distance: number;
}

const held = (value: number, fallback: number): number =>
  Number.isFinite(value) ? value : fallback;

/**
 * Skins `skeleton` with one continuous surface.
 *
 * Guarantees, all structural rather than tuned:
 *   - no boundary edges: every edge is shared by exactly two triangles,
 *     wound in opposite directions, so there is no gap and no hole
 *     anywhere in the mesh;
 *   - one ring per node along a run, shared by the segments either
 *     side of it, so no joint can open at a bend;
 *   - every triangle has area, and every vertex is finite;
 *   - identical arguments give an identical mesh, vertex for vertex.
 */
export function buildSurface(
  skeleton: Skeleton,
  field: RadiusField,
  envelope: Envelope,
  params: SurfaceParams,
): SurfaceMesh {
  const nodes = skeleton.nodes;
  const paths = branchPaths(skeleton, field);
  if (paths.length === 0) {
    return {
      positions: new Float32Array(0),
      indices: new Uint32Array(0),
      triangles: 0,
      vertices: 0,
    };
  }

  const height = Math.max(
    1e-6,
    held(envelope.height, DEFAULT_ENVELOPE.height),
  );
  const segments = Math.min(
    MAX_RADIAL_SEGMENTS,
    Math.max(
      MIN_RADIAL_SEGMENTS,
      Math.round(held(params.radialSegments, DEFAULT_SURFACE.radialSegments)),
    ),
  );
  const lobes = Math.min(
    MAX_LOBES,
    Math.max(0, Math.round(held(params.lobes, DEFAULT_SURFACE.lobes))),
  );
  const lobeDepth = Math.min(
    MAX_LOBE_DEPTH,
    Math.max(0, held(params.lobeDepth, DEFAULT_SURFACE.lobeDepth)),
  );
  const twistRate = held(params.twistRate, DEFAULT_SURFACE.twistRate);
  const flareRadius = Math.max(
    1,
    held(params.flareRadius, DEFAULT_SURFACE.flareRadius),
  );
  const flareFalloff =
    Math.max(
      MIN_FLARE_FALLOFF,
      held(params.flareFalloff, DEFAULT_SURFACE.flareFalloff),
    ) * height;
  const flareDepth =
    Math.max(0, held(params.flareDepth, DEFAULT_SURFACE.flareDepth)) * height;
  const forkSocket = Math.max(
    0,
    held(params.forkSocket, DEFAULT_SURFACE.forkSocket),
  );
  const forkSwell = Math.max(
    1,
    held(params.forkSwell, DEFAULT_SURFACE.forkSwell),
  );

  /* Distance from the root to every node, along the tree. The twist
     phase is a function of it, so the plait runs continuously out of
     the trunk and into every limb rather than starting again at each
     fork. Parent indices are lower than their children's, so one
     forward pass is enough. */
  const distance = new Float64Array(nodes.length);
  for (let i = 1; i < nodes.length; i += 1) {
    const parent = nodes[i].parent;
    if (parent < 0) continue;
    distance[i] =
      distance[parent] + nodes[parent].position.distanceTo(nodes[i].position);
  }

  /** The flare, as a multiplier on a radius at height `y`. Above the
   *  falloff it is 1 to floating point, so this is a ground effect
   *  applied by a rule rather than a special case for the trunk. */
  const flare = (y: number): number =>
    1 + (flareRadius - 1) * Math.exp(-Math.max(0, y) / flareFalloff);

  const positions: number[] = [];
  const indices: number[] = [];
  let vertices = 0;

  for (const path of paths) {
    const samples: Sample[] = [];

    if (path.trunk) {
      /* The trunk's own end, buried. Without it the sweep starts with
         a flat disc lying in the ground plane, which is the thing the
         task calls out by name. */
      const root = nodes[path.nodes[0]].position;
      samples.push({
        position: new THREE.Vector3(root.x, root.y - flareDepth, root.z),
        radius: field.radius[path.nodes[0]] * flare(root.y),
        distance: 0,
      });
      for (const node of path.nodes) {
        samples.push({
          position: nodes[node].position.clone(),
          radius: field.radius[node] * flare(nodes[node].position.y),
          distance: distance[node],
        });
      }
    } else {
      /* A child run. It starts back inside the parent, along its own
         axis - the fork node is on the parent's centreline, so any
         step shorter than the parent's radius from there is still
         inside the parent's solid - and it leaves the fork swollen,
         decaying over one parent radius. Together those are what make
         the junction read as a limb dividing rather than as two tubes
         crossing. */
      const attach = path.nodes[0];
      const first = path.nodes[1];
      const parentRadius = field.radius[attach];
      const away = nodes[first].position
        .clone()
        .sub(nodes[attach].position)
        .normalize();
      const swell = (along: number): number =>
        1 + (forkSwell - 1) * Math.exp(-along / Math.max(1e-9, parentRadius));

      samples.push({
        position: nodes[attach].position
          .clone()
          .addScaledVector(away, -forkSocket * parentRadius),
        radius: field.startRadius[first] * forkSwell * flare(nodes[attach].position.y),
        distance: distance[attach],
      });
      for (let i = 1; i < path.nodes.length; i += 1) {
        const node = path.nodes[i];
        const position = nodes[node].position;
        samples.push({
          position: position.clone(),
          radius:
            field.radius[node] *
            swell(distance[node] - distance[attach]) *
            flare(position.y),
          distance: distance[node],
        });
      }
    }

    if (samples.length < 2) continue;

    const frames = transportFrames(samples.map((sample) => sample.position));
    const base = vertices;

    for (let i = 0; i < samples.length; i += 1) {
      const { position, radius } = samples[i];
      const { normal, binormal } = frames[i];
      // The section's own rotation, in radians at this point of the
      // sweep. This one term is the whole plait.
      const phase = TWO_PI * twistRate * (samples[i].distance / height);
      for (let k = 0; k < segments; k += 1) {
        const angle = (k / segments) * TWO_PI;
        const profile = 1 + lobeDepth * Math.cos(lobes * (angle + phase));
        const width = radius * profile;
        const cos = Math.cos(angle);
        const sin = Math.sin(angle);
        positions.push(
          position.x + (normal.x * cos + binormal.x * sin) * width,
          position.y + (normal.y * cos + binormal.y * sin) * width,
          position.z + (normal.z * cos + binormal.z * sin) * width,
        );
      }
    }
    vertices += samples.length * segments;

    /* The skin between consecutive rings. Both triangles are wound so
       that their normal points away from the centreline, which is what
       makes every normal on the surface an outward one - there is no
       per-face decision to get wrong. */
    for (let i = 0; i < samples.length - 1; i += 1) {
      const lower = base + i * segments;
      const upper = lower + segments;
      for (let k = 0; k < segments; k += 1) {
        const next = (k + 1) % segments;
        indices.push(lower + k, lower + next, upper + k);
        indices.push(lower + next, upper + next, upper + k);
      }
    }

    /* Both ends closed, by a fan to a vertex at the ring's own centre.
       That is what makes each run a closed shell, and a union of
       closed shells is a surface with nothing to see through. The
       trunk's lower cap is underground and every other one is either a
       twig tip a few millimetres across or a child run's back end,
       which is inside its parent. */
    const bottomCentre = vertices;
    positions.push(
      samples[0].position.x,
      samples[0].position.y,
      samples[0].position.z,
    );
    const last = samples[samples.length - 1].position;
    const topCentre = vertices + 1;
    positions.push(last.x, last.y, last.z);
    vertices += 2;

    const topRing = base + (samples.length - 1) * segments;
    for (let k = 0; k < segments; k += 1) {
      const next = (k + 1) % segments;
      indices.push(bottomCentre, base + next, base + k);
      indices.push(topCentre, topRing + k, topRing + next);
    }
  }

  return {
    positions: Float32Array.from(positions),
    indices: Uint32Array.from(indices),
    triangles: indices.length / 3,
    vertices,
  };
}
