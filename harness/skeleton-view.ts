import * as THREE from "three";

import {
  buildSurface,
  DEFAULT_SURFACE,
  type SurfaceParams,
} from "../src/mesh/surface";
import type { TreePreset } from "../src/presets";
import { solveRadii, type RadiusParams } from "../src/radius";
import { growSkeleton, type SkeletonParams } from "../src/skeleton/grow";

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
    /* Every member of `Envelope`, named. Spreading the default and
       overriding three of them was fine while the other two were
       constants nobody could reach; now that they are dials, an
       envelope assembled by spread would silently drop whichever term
       the panel forgot to list, which is exactly the drift the
       preset round-trip test exists to catch. */
    envelope: {
      height: params.height,
      spread: params.spread,
      crownBase: params.crownBase,
      fullness: params.fullness,
      shoulder: params.shoulder,
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
  /** What the subject costs the renderer in draw calls: one per
   *  renderable in it. The canopy's claim is that a whole crown is one
   *  of these, so this is the number that claim is read off. */
  drawCalls: number;
  /** Instanced copies across the subject. One today would be a
   *  surprise - nothing here instances yet - and the field exists
   *  because the number it will carry is the one the canopy is judged
   *  by. */
  instances: number;
  /** Wall-clock milliseconds for the whole build: grow, solve, sweep. */
  buildMs: number;
}

/** What an object costs to draw, counted off the object graph.
 *
 *  Counted here rather than read from `renderer.info.render` for two
 *  reasons, and neither is convenience. The renderer's number is the
 *  whole SCENE's - ground disc, scale figure and all - so it answers a
 *  different question from "what did this subject cost"; and there is
 *  no renderer in the test runner, so a subject's draw count read off
 *  the renderer is a number nothing can hold to account. The stage
 *  reports the renderer's own figure beside this one, and the two
 *  differing by the room's fixtures is the expected reading.
 *
 *  Draws, not geometries: two instanced meshes sharing one geometry
 *  are two draw calls, because that is what the GPU is asked for. What
 *  must not be double counted is a shared geometry's TRIANGLES, and
 *  those are summed per built tree from the surface that produced
 *  them, never per instance. */
export function countDraws(object: THREE.Object3D): {
  drawCalls: number;
  instances: number;
} {
  let drawCalls = 0;
  let instances = 0;
  object.traverse((node) => {
    // InstancedMesh extends Mesh, so it is asked about first: one draw
    // whatever its count, and the count is the instances.
    if (node instanceof THREE.InstancedMesh) {
      drawCalls += 1;
      instances += node.count;
    } else if (
      node instanceof THREE.Mesh ||
      node instanceof THREE.Line ||
      node instanceof THREE.Points
    ) {
      drawCalls += 1;
    }
  });
  return { drawCalls, instances };
}

/** A preset's parameters, as the panel's dials.
 *
 *  The dials and a preset are two ways of writing down the same
 *  argument triple, so this is a rename and not a translation: every
 *  member of `GrowerParams` comes from the preset and nothing is
 *  invented here. `torsion` is the one term with no counterpart, and
 *  it is 1 by definition - a preset states the three bias terms it
 *  wants, and the master that scales them is a convenience for
 *  dragging, not part of the tree.
 *
 *  Exact in both directions: `toSkeletonParams`, `toRadiusParams` and
 *  `toSurfaceParams` applied to the result reproduce the preset
 *  member for member. That round trip is asserted, because a panel
 *  that silently dropped one of a preset's terms would show the owner
 *  a tree nobody authored. */
export function presetToParams(preset: TreePreset): GrowerParams {
  const { envelope, bias } = preset.skeleton;
  return {
    seed: preset.skeleton.seed,
    height: envelope.height,
    spread: envelope.spread,
    crownBase: envelope.crownBase,
    fullness: envelope.fullness,
    shoulder: envelope.shoulder,
    torsion: 1,
    gravitropism: bias.gravitropism,
    lean: bias.lean,
    writheAmplitude: bias.writheAmplitude,
    writheWavelength: bias.writheWavelength,
    spiralRate: bias.spiralRate,
    maxTurnPerStep: preset.skeleton.growth.maxTurnPerStep,
    density: (preset.skeleton.attractors - ATTRACTORS_MIN) /
      (ATTRACTORS_MAX - ATTRACTORS_MIN),
    taper: preset.radii.forkExponent,
    trunkRadius: preset.radii.trunkRadius,
    lengthTaper: preset.radii.lengthTaper,
    lobes: preset.surface.lobes,
    lobeDepth: preset.surface.lobeDepth,
    twistRate: preset.surface.twistRate,
    flareRadius: preset.surface.flareRadius,
  };
}

/** The one build. Skeleton, radii, surface, mesh - and the only thing
 *  that varies between one tree and another is the three argument
 *  objects handed in, which is the spec's acceptance test stated as a
 *  function signature: Telperion and Laurelin reach this with
 *  different numbers and by no other difference. */
function build(
  skeletonParams: SkeletonParams,
  radii: RadiusParams,
  surfaceParams: SurfaceParams,
  clay: Clay,
): { tree: THREE.Mesh; stats: TreeStats } {
  const started = performance.now();
  const skeleton = growSkeleton(skeletonParams);
  const field = solveRadii(skeleton, skeletonParams.envelope, radii);
  const surface = buildSurface(
    skeleton,
    field,
    skeletonParams.envelope,
    surfaceParams,
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
  const draws = countDraws(tree);
  return {
    tree,
    stats: {
      triangles: surface.triangles,
      vertices: surface.vertices,
      nodes: skeleton.nodes.length,
      drawCalls: draws.drawCalls,
      instances: draws.instances,
      buildMs: performance.now() - started,
    },
  };
}

/** The subject the stage draws: this tree, in clay, skinned.
 *
 *  Deterministic in `params` - same seed and dials, same mesh, vertex
 *  for vertex - because every stage of it is. */
export function buildTree(
  params: GrowerParams,
  clay: Clay,
): { tree: THREE.Mesh; stats: TreeStats } {
  return build(
    toSkeletonParams(params),
    toRadiusParams(params),
    toSurfaceParams(params),
    clay,
  );
}

/** One named tree, exactly as authored. Straight from the preset's own
 *  three objects rather than round-tripped through the dials, so what
 *  the owner judges is what the library file says and not what the
 *  panel could express of it. */
export function buildPreset(
  preset: TreePreset,
  clay: Clay,
): { tree: THREE.Mesh; stats: TreeStats } {
  return build(preset.skeleton, preset.radii, preset.surface, clay);
}

/** The gap between two trees standing side by side, as a fraction of
 *  the wider one's crown. Enough air that the two silhouettes are read
 *  as two trees rather than as one thicket, and no more - the whole
 *  point of the comparison is that they are close enough to judge
 *  against each other in one glance. */
const COMPARISON_GAP = 0.18;

/** The spec's acceptance test, standing on the ground together: every
 *  preset in a row, each built by `buildPreset` and moved sideways.
 *  Nothing about a tree changes here but where it stands - the
 *  comparison is a translation and not a second way of growing a tree.
 *
 *  Laid out by crown width rather than at a fixed pitch, because the
 *  trees being compared differ in width by a factor of three and a
 *  fixed pitch would either overlap the broad one or strand the narrow
 *  one. Centred on the origin so the stage's own framing sees a
 *  balanced subject. */
export function buildComparison(
  presets: readonly TreePreset[],
  clay: Clay,
): { group: THREE.Group; stats: TreeStats } {
  const group = new THREE.Group();
  group.name = "grower-comparison";

  const built = presets.map((preset) => ({
    preset,
    ...buildPreset(preset, clay),
  }));

  const widths = built.map(
    ({ preset }) =>
      2 * preset.skeleton.envelope.height * preset.skeleton.envelope.spread,
  );
  const gap = Math.max(0, ...widths) * COMPARISON_GAP;
  const span =
    widths.reduce((total, width) => total + width, 0) +
    gap * Math.max(0, widths.length - 1);

  let cursor = -span / 2;
  built.forEach(({ tree }, index) => {
    tree.position.x = cursor + widths[index] / 2;
    cursor += widths[index] + gap;
    group.add(tree);
  });

  return {
    group,
    stats: {
      triangles: built.reduce((total, one) => total + one.stats.triangles, 0),
      vertices: built.reduce((total, one) => total + one.stats.vertices, 0),
      nodes: built.reduce((total, one) => total + one.stats.nodes, 0),
      /* Draws and instances sum the same way the rest do, because two
         trees standing side by side really are two subjects' worth of
         work: each carries its own surface and, when the canopy lands,
         its own instanced crown. What is NOT summed twice is a
         geometry either of them might share - that would be counting
         one buffer's triangles per copy of it, and triangles come from
         the surface each tree built rather than from the graph. */
      drawCalls: built.reduce((total, one) => total + one.stats.drawCalls, 0),
      instances: built.reduce((total, one) => total + one.stats.instances, 0),
      buildMs: built.reduce((total, one) => total + one.stats.buildMs, 0),
    },
  };
}
