import * as THREE from "three";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import {
  buildSurface,
  DEFAULT_SURFACE,
  type SurfaceParams,
} from "@/lib/grower/mesh/surface";
import { solveRadii, type RadiusParams } from "@/lib/grower/radius";
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
 * The surface itself is the library's, and all of it: growing the
 * skeleton, solving the radii and sweeping the skin are three library
 * calls, and what is left here is the translation either side of them
 * plus the cost of the build, which the panel reports. The tapered
 * proxy tubes this file used to draw are gone - they were a viewer for
 * the radius solve while there was no surface to look at, and every
 * gap the owner screenshotted was theirs.
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
      crownBase: params.crownBase,
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

/** The panel's three thickness dials, as the radius solve's arguments.
 *  All three carry the library's own names and units, so there is
 *  nothing here to translate and nothing to drift.
 *
 *  `trunkRadius` is the one that answers "is this a big tree": the
 *  library states it as a fraction of height and does not scale it
 *  with height, so a 60 m tree is exactly as slender in proportion as
 *  a 4 m one until somebody says otherwise. Saying otherwise is the
 *  dial, and later the preset. */
export function toRadiusParams(params: GrowerParams): RadiusParams {
  return {
    forkExponent: params.taper,
    trunkRadius: params.trunkRadius,
    lengthTaper: params.lengthTaper,
  };
}


/** The panel's four surface dials, as the sweep's arguments. The rest
 *  of `SurfaceParams` - how finely the section is sampled, how deep a
 *  child sockets into its parent and how much it swells leaving it,
 *  how far the flare decays and how far it sinks - keep the library's
 *  defaults, on the same footing as the growth distances in grow.ts:
 *  they are structure and cost rather than look, nothing has asked to
 *  turn them live, and each is one line in SLIDERS the day something
 *  does.
 *
 *  `twistRate` is the surface's rotation and it is deliberately not
 *  scaled by the panel's `torsion` master: `torsion` gathers the three
 *  terms that bend the CENTRELINE, and the plait is a different
 *  mechanism that happens to the skin. Folding them together would
 *  make one dial mean two things, which is the thing the spec's
 *  parameter principle exists to stop. */
export function toSurfaceParams(params: GrowerParams): SurfaceParams {
  return {
    ...DEFAULT_SURFACE,
    lobes: params.lobes,
    lobeDepth: params.lobeDepth,
    twistRate: params.twistRate,
    flareRadius: params.flareRadius,
  };
}

/** What the surface cost to build, for the panel to report. The owner
 *  is entitled to know what a dial just spent: the swept skin is
 *  allowed to cost more than the fn-11.4 viewer did, but not silently. */
export interface TreeStats {
  triangles: number;
  vertices: number;
  /** Skeleton nodes, which is what the density dial moves and what
   *  every other number here scales with. */
  nodes: number;
  /** Wall-clock milliseconds for the whole build: grow, solve, sweep. */
  buildMs: number;
}

/** The subject the stage draws: this tree, in clay, skinned.
 *
 *  Deterministic in `params` - same seed and dials, same mesh, vertex
 *  for vertex - because every stage of it is. */
export function buildTree(
  params: GrowerParams,
  clay: Clay,
): { tree: THREE.Mesh; stats: TreeStats } {
  const started = performance.now();
  const skeletonParams = toSkeletonParams(params);
  const skeleton = growSkeleton(skeletonParams);
  const field = solveRadii(
    skeleton,
    skeletonParams.envelope,
    toRadiusParams(params),
  );
  const surface = buildSurface(
    skeleton,
    field,
    skeletonParams.envelope,
    toSurfaceParams(params),
  );

  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute(
    "position",
    new THREE.BufferAttribute(surface.positions, 3),
  );
  geometry.setIndex(new THREE.BufferAttribute(surface.indices, 1));
  /* Shared ring vertices carry a shared normal, so this smooths around
     the section and along the sweep in one pass. There are no joints
     left for it to smooth over: the surface has none. */
  geometry.computeVertexNormals();

  const tree = new THREE.Mesh(geometry, clay.surface);
  tree.name = "grower-tree";
  return {
    tree,
    stats: {
      triangles: surface.triangles,
      vertices: surface.vertices,
      nodes: skeleton.nodes.length,
      buildMs: performance.now() - started,
    },
  };
}
