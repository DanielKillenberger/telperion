import * as THREE from "three";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import {
  DEFAULT_RADII,
  solveRadii,
  type RadiusField,
  type RadiusParams,
} from "@/lib/grower/radius";
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
 * It draws each branch as a tapered tube of circular section, which
 * is a viewer for the radius solve and nothing more. fn-11.5 sweeps
 * the real surface in the library - non-circular, rotating along its
 * length, with a root flare - and deletes this. The line separating
 * the two is deliberate: what is on screen here is the thickness
 * hierarchy and only the thickness hierarchy, because that is what
 * this task is judged on, and a proxy that started reaching for the
 * look of the surface would be answering fn-11.5's question badly
 * instead of this one honestly.
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
 *  `density` is the attractor count. `taper` is not a skeleton
 *  argument at all - thickness is solved over the skeleton once it has
 *  grown, so it travels through `toRadiusParams`. */
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
    // Persistence is a growth distance's kind of parameter rather than
    // a bias term - it is about the step, not about the field - so it
    // travels in `growth`, which is where the library keeps the rest of
    // them. Outside `torsion` on purpose: a stiff tree is stiff whether
    // or not it is writhing.
    growth: { maxTurnPerStep: params.maxTurnPerStep },
  };
}

/** The panel's `taper` dial, as the radius solve's arguments. The
 *  other two terms of the solve - how stout the trunk is and how fast
 *  a limb thins along its own length - keep the library's defaults,
 *  because no dial has been asked for and inventing panel values for
 *  them would give the harness a second opinion about what a tree
 *  looks like. */
export function toRadiusParams(params: GrowerParams): RadiusParams {
  return { ...DEFAULT_RADII, forkExponent: params.taper };
}

/** Sides on the proxy tube's cross section. Enough that a trunk beside
 *  a 1.8 m figure reads as round rather than as a prism, few enough
 *  that a 1600-attractor tree is still one cheap draw call. It is not
 *  a look decision: fn-11.5's section is not a circle at all. */
export const TUBE_SIDES = 8;

/** One tapered tube per branch, in one geometry and one draw call.
 *
 *  Each tube runs from its parent's position at `startRadius` to its
 *  own at `radius`, which is what the radius solve means by those two
 *  numbers - so along an unbranched run consecutive tubes meet at
 *  exactly the same width and the limb reads as continuous, while at a
 *  fork each limb leaves the trunk at its own width.
 *
 *  Ring orientation comes from a fixed world reference rather than
 *  from the branch it follows, so two consecutive tubes pointing
 *  nearly the same way get nearly the same ring and the joint does not
 *  visibly twist. A real swept frame, carried along the branch, is
 *  fn-11.5's. */
export function branchGeometry(
  skeleton: Skeleton,
  field: RadiusField,
): THREE.BufferGeometry {
  const edges: number[] = [];
  skeleton.nodes.forEach((node, index) => {
    if (node.parent < 0) return;
    if (node.position.distanceTo(skeleton.nodes[node.parent].position) > 0) {
      edges.push(index);
    }
  });

  const ring = TUBE_SIDES;
  const positions = new Float32Array(edges.length * ring * 2 * 3);
  const indices = new Uint32Array(edges.length * ring * 6);

  const axis = new THREE.Vector3();
  const reference = new THREE.Vector3();
  const across = new THREE.Vector3();
  const up = new THREE.Vector3();
  const offset = new THREE.Vector3();

  edges.forEach((node, edge) => {
    const from = skeleton.nodes[skeleton.nodes[node].parent].position;
    const to = skeleton.nodes[node].position;
    axis.subVectors(to, from).normalize();

    // The world axis this branch is least aligned with; crossing with
    // the one it points most nearly along would be a degenerate basis.
    const ax = Math.abs(axis.x);
    const ay = Math.abs(axis.y);
    const az = Math.abs(axis.z);
    if (ax <= ay && ax <= az) reference.set(1, 0, 0);
    else if (ay <= az) reference.set(0, 1, 0);
    else reference.set(0, 0, 1);
    across.crossVectors(axis, reference).normalize();
    up.crossVectors(axis, across);

    const base = edge * ring * 2;
    for (let side = 0; side < ring; side += 1) {
      const angle = (side / ring) * Math.PI * 2;
      offset
        .copy(across)
        .multiplyScalar(Math.cos(angle))
        .addScaledVector(up, Math.sin(angle));

      const start = (base + side) * 3;
      positions[start] = from.x + offset.x * field.startRadius[node];
      positions[start + 1] = from.y + offset.y * field.startRadius[node];
      positions[start + 2] = from.z + offset.z * field.startRadius[node];

      const end = (base + ring + side) * 3;
      positions[end] = to.x + offset.x * field.radius[node];
      positions[end + 1] = to.y + offset.y * field.radius[node];
      positions[end + 2] = to.z + offset.z * field.radius[node];
    }

    for (let side = 0; side < ring; side += 1) {
      const next = (side + 1) % ring;
      const at = (edge * ring + side) * 6;
      indices[at] = base + side;
      indices[at + 1] = base + next;
      indices[at + 2] = base + ring + side;
      indices[at + 3] = base + next;
      indices[at + 4] = base + ring + next;
      indices[at + 5] = base + ring + side;
    }
  });

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.BufferAttribute(positions, 3));
  geometry.setIndex(new THREE.BufferAttribute(indices, 1));
  // Shared ring vertices, so this smooths around the tube and leaves
  // the joints between tubes as the only hard edges.
  geometry.computeVertexNormals();
  return geometry;
}

/** The subject the stage draws: this tree, in clay, with thickness. */
export function buildTree(params: GrowerParams, clay: Clay): THREE.Mesh {
  const skeletonParams = toSkeletonParams(params);
  const skeleton = growSkeleton(skeletonParams);
  const field = solveRadii(
    skeleton,
    skeletonParams.envelope,
    toRadiusParams(params),
  );
  const tree = new THREE.Mesh(branchGeometry(skeleton, field), clay.surface);
  tree.name = "grower-tree";
  return tree;
}
