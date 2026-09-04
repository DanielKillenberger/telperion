import * as THREE from "three";
import { mergeGeometries } from "three/examples/jsm/utils/BufferGeometryUtils.js";

import type { GrowerParams } from "./params";

/* ------------------------------------------------------------------ *
 * A STAND-IN, AND IT IS SUPPOSED TO LOOK LIKE ONE
 *
 * fn-11.2 through .5 build the real thing: seeded RNG, envelope
 * sampling, space colonization, a torsion field, an area-preserving
 * radius solve, swept non-circular cross sections. None of that is
 * here.
 *
 * This is recursive branching with jitter - precisely the approach the
 * spec rejects, because it is self-similar by construction and its
 * silhouette is always a blob. It exists so the harness has something
 * to regenerate on the first day, so the owner can see a seed change
 * and a slider move do something, and so the panel-to-generator
 * contract is exercised before there is a generator. Delete it whole
 * when the skeleton lands; do not grow it.
 * ------------------------------------------------------------------ */

/** xorshift32. Enough for a stand-in; fn-11.2 owns the real one. */
function rng(seed: number): () => number {
  let state = (seed >>> 0) === 0 ? 0x9e_37_79_b9 : seed >>> 0;
  return () => {
    state ^= state << 13;
    state ^= state >>> 17;
    state ^= state << 5;
    state >>>= 0;
    return state / 0x1_00_00_00_00;
  };
}

const DEPTH = 6;
const UP = new THREE.Vector3(0, 1, 0);

/**
 * Builds a stand-in tree from `params`, merged into a single mesh so
 * the harness stays at one draw call and one geometry to dispose.
 * Deterministic in `params.seed`.
 */
export function buildPlaceholderTree(
  params: GrowerParams,
  material: THREE.Material,
): THREE.Mesh {
  const random = rng(params.seed);
  const parts: THREE.BufferGeometry[] = [];

  // Trunk radius from height, so the tree reads at any scale, and from
  // spread, so a wide crown gets a stout base rather than a flagpole.
  const rootRadius = params.height * 0.035 * (0.75 + params.spread * 0.5);
  const rootLength = params.height * 0.3;

  const grow = (
    origin: THREE.Vector3,
    direction: THREE.Vector3,
    length: number,
    radius: number,
    depth: number,
  ): void => {
    const tip = origin.clone().addScaledVector(direction, length);
    const tipRadius = radius * params.taper;
    parts.push(segment(origin, tip, radius, tipRadius));
    if (depth === 0) return;

    // Two children, plus a third where density asks for one. More
    // children is the only thing `density` does here.
    const children = random() < params.density ? 3 : 2;
    const childRadius = tipRadius * Math.pow(1 / children, 1 / 2.2);

    for (let i = 0; i < children; i += 1) {
      // Torsion as a rigid spiral about the trunk axis: each level is
      // turned further than the last. The real field (fn-11.3) is a
      // spiral plus curl noise, and it does not live in the branching.
      const spiral =
        (i / children) * Math.PI * 2 +
        params.torsion * (DEPTH - depth) * 1.1 +
        random() * 0.7;
      const lean = 0.35 + params.spread * 0.55 + random() * 0.2;

      const child = direction
        .clone()
        .addScaledVector(
          new THREE.Vector3(Math.cos(spiral), 0, Math.sin(spiral)),
          lean,
        )
        .addScaledVector(UP, 0.15)
        .normalize();

      grow(
        tip,
        child,
        length * (0.68 + random() * 0.12),
        childRadius,
        depth - 1,
      );
    }
  };

  grow(new THREE.Vector3(0, 0, 0), UP.clone(), rootLength, rootRadius, DEPTH);

  const merged = mergeGeometries(parts, false);
  for (const part of parts) part.dispose();
  if (merged === null) {
    throw new Error("placeholder tree: geometries failed to merge");
  }
  merged.computeVertexNormals();

  const mesh = new THREE.Mesh(merged, material);
  mesh.name = "placeholder-tree";
  return mesh;
}

/** One tapered length of branch, already placed in world space. */
function segment(
  from: THREE.Vector3,
  to: THREE.Vector3,
  radiusBottom: number,
  radiusTop: number,
): THREE.BufferGeometry {
  const axis = to.clone().sub(from);
  const length = axis.length();
  const geometry = new THREE.CylinderGeometry(
    radiusTop,
    radiusBottom,
    length,
    7,
    1,
    true,
  );
  // Cylinders are built along +Y about their own centre; move the
  // origin to the base first so the rotation below turns about the
  // joint and not about the middle of the branch.
  geometry.translate(0, length / 2, 0);
  geometry.applyQuaternion(
    new THREE.Quaternion().setFromUnitVectors(UP, axis.normalize()),
  );
  geometry.translate(from.x, from.y, from.z);
  return geometry;
}
