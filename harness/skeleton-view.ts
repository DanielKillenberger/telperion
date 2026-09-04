import * as THREE from "three";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";
import { growSkeleton, type SkeletonParams } from "@/lib/grower/skeleton/grow";

import type { GrowerParams } from "./params";
import type { Clay } from "./stage";

/* ------------------------------------------------------------------ *
 * THE PANEL'S HALF OF THE CONTRACT
 *
 * Two jobs, both of them translation. Turn the dials the owner is
 * looking at into the arguments the generator takes, and turn the
 * skeleton it returns into something the clay room can draw. The
 * generator itself knows about neither the panel nor the stage, which
 * is what "the generator is a standalone library" has to mean in
 * practice.
 *
 * It draws as lines because there is nothing to shade yet: fn-11.4
 * solves the radii and fn-11.5 sweeps a surface over them. What is on
 * screen now is the branching structure and only the branching
 * structure, which is the thing this task is judged on.
 * ------------------------------------------------------------------ */

/** What the density dial spans, in attractors. The floor is a tree
 *  with a readable handful of limbs rather than a bare fork; the
 *  ceiling is where the crown stops gaining structure and starts
 *  gaining only cost. */
const ATTRACTORS_MIN = 250;
const ATTRACTORS_MAX = 1600;

/** The panel's dials, as the generator's arguments.
 *
 *  `height` and `spread` are the authored envelope and go straight
 *  through, and so do the five bias dials - they carry the library's
 *  own names and units, so there is nothing here to translate and
 *  nothing to drift. `torsion` is the one dial that is not a library
 *  term: it scales the three that are departures from vertical, so one
 *  move takes the tree from straight to writhing without walking three
 *  sliders. Gravitropism is deliberately outside it - a tree that wants
 *  to grow up still wants to when it is not twisting.
 *  `density` is the attractor count. `taper` reaches
 *  nothing yet and the panel says so: it is fn-11.4's radius solve, and
 *  wiring it to something that merely looks related would be worse than
 *  leaving it visibly inert. */
export function toSkeletonParams(params: GrowerParams): SkeletonParams {
  return {
    seed: params.seed,
    envelope: {
      ...DEFAULT_ENVELOPE,
      height: params.height,
      spread: params.spread,
    },
    attractors: Math.round(
      ATTRACTORS_MIN + params.density * (ATTRACTORS_MAX - ATTRACTORS_MIN),
    ),
    bias: {
      gravitropism: params.gravitropism,
      lean: params.lean * params.torsion,
      writheAmplitude: params.writheAmplitude * params.torsion,
      writheWavelength: params.writheWavelength,
      spiralRate: params.spiralRate * params.torsion,
    },
  };
}

/** One line segment per branch, in one geometry and one draw call. */
export function skeletonGeometry(skeleton: Skeleton): THREE.BufferGeometry {
  const edges = skeleton.nodes.reduce(
    (count, node) => (node.parent >= 0 ? count + 1 : count),
    0,
  );
  const positions = new Float32Array(edges * 6);

  let at = 0;
  for (const node of skeleton.nodes) {
    if (node.parent < 0) continue;
    const from = skeleton.nodes[node.parent].position;
    positions[at] = from.x;
    positions[at + 1] = from.y;
    positions[at + 2] = from.z;
    positions[at + 3] = node.position.x;
    positions[at + 4] = node.position.y;
    positions[at + 5] = node.position.z;
    at += 6;
  }

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  return geometry;
}

/** The subject the stage draws: this tree's skeleton, in clay. */
export function buildSkeletonLines(
  params: GrowerParams,
  clay: Clay,
): THREE.LineSegments {
  const skeleton = growSkeleton(toSkeletonParams(params));
  const lines = new THREE.LineSegments(skeletonGeometry(skeleton), clay.line);
  lines.name = "grower-skeleton";
  return lines;
}
