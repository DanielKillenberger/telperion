import * as THREE from "three";

import { treeCore, type Family, type TreePreset, type Timings } from "../src/browser/core";
import { materializeTree, disposeTreeGeometry } from "../src/browser/three";
type SkeletonParams = Family["skeleton"];
type RadiusParams = Family["radii"];
type SurfaceParams = Family["surface"];
type CanopyParams = Family["canopy"];

import type { GrowerParams } from "./params";
import type { Clay } from "./stage";

/** Translates viewer controls to native family parameters and materializes
 * owned Rust outputs for the clay stage. Botanical generation stays in Rust. */

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
 *  `density` is the attractor count, and `step` and the branch-law
 *  rules go through under the library's own names. Twig anatomy stays
 *  in metres. `taper` travels through `toRadiusParams`: the colonization
 *  radius solve uses it before branch growth, and the final solve
 *  preserves the appended branches' recorded local taper. */
export function toSkeletonParams(params: GrowerParams): SkeletonParams {
  return {
    ...params.family.skeleton,
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
    step: params.step,
    // Preserve every branch-law and anatomy field through preset round trips.
    twigs: {
      twig: { length: params.twigLength, diameter: params.twigDiameter, internodeLength: params.twigStationLength,
        stationsPerInternode: params.twigStations, bearingDiameter: params.twigBearing },
      ratioPower: params.ratioPower,
      limbRadius: params.limbRadius,
      reach: params.reach,
      laterals: params.laterals,
      angleVariation: params.angleVariation,
      vigourVariation: params.vigourVariation,
      angle: params.twigAngle,
      divergence: params.twigDivergence,
      internodeFactor: params.internodeFactor,
      lengthRatio: params.lengthRatio,
    },
    bias: {
      gravitropism: params.gravitropism,
      lean: params.lean * params.torsion,
      supernatural: {
        enabled: params.supernaturalEnabled,
        writheAmplitude: params.writheAmplitude * params.torsion,
        writheWavelength: params.writheWavelength,
        spiralRate: params.spiralRate * params.torsion,
      },
    },
    // Persistence is a growth distance's kind of parameter rather than
    // a bias term - it is about the step, not about the field - so it
    // travels in `growth`, which is where the library keeps the rest of
    // them. Outside `torsion` on purpose: a stiff tree is stiff whether
    // or not it is writhing.
    growth: { ...params.family.skeleton.growth, maxTurnPerStep: params.maxTurnPerStep },
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
 *  defaults, on the same footing as native growth distances:
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
    ...params.family.surface,
    lobes: params.lobes,
    lobeDepth: params.lobeDepth,
    twistRate: params.twistRate,
    flareRadius: params.flareRadius,
  };
}

/** The panel's canopy values, as the placement stage's arguments.
 *
 *  A rename and nothing else: every term carries the library's own
 *  name and unit, so there is nothing here to translate and nothing to
 *  drift - the same footing the bias and surface dials are on. All ten
 *  are stated rather than spread over `DEFAULT_CANOPY`, for the reason
 *  the envelope in `toSkeletonParams` is: a set assembled by spread
 *  silently keeps a default for whichever term the panel forgot, and a
 *  preset loaded onto the dials would then be built from a canopy
 *  nobody authored. */
export function toCanopyParams(params: GrowerParams): CanopyParams {
  return {
    ...params.family.canopy,
    shootRadius: params.shootRadius,
    spacing: params.spacing,
    divergence: params.divergence,
    clump: params.clump,
    clumpSpan: params.clumpSpan,
    outward: params.outward,
    upward: params.upward,
    scatter: params.scatter,
    size: params.size,
    sizeVariation: params.sizeVariation,
  };
}

/** What the surface cost to build, for the panel to report. The owner
 *  is entitled to know what a dial just spent: the swept skin is
 *  allowed to cost more than the fn-11.4 viewer did, but not silently. */
export interface TreeStats {
  triangles: number;
  vertices: number;
  /** Skeleton nodes, which is what the density and step dials move
   *  and what every other number here scales with. */
  nodes: number;
  /** Whether the growth stopped at its node ceiling rather than
   *  finishing the crown or the twigs. A capped tree is the ceiling's
   *  shape and not the envelope's, and `nodes` alone cannot say which
   *  it was - so the panel says it in words. */
  capped: boolean;
  attractionCapped: boolean;
  complete: boolean;
  timings: Timings & { materializationMs: number };
  /** The pass's actual generation stop, retained even after shedding. */
  levelCapped: boolean;
  /** Surviving edges leaving colonization, including tip leaders and laterals. */
  handoffs: number;
  /** Radius-law reductions to twig size for surviving handoffs. Empty is null. */
  generations: { min: number; median: number; max: number } | null;
  /** Histogram preserves the pooled median when multiple trees are compared. */
  generationCounts: number[];
  /** Handoffs whose radius law remains above twig radius at the safety cap.
   * This prediction can exceed actual stops when collisions or short runs end growth. */
  levelCappedHandoffs: number;
  /** Surviving twig marks, independent of the foliage visibility toggle. */
  twigs: number;
  /** What the subject costs the renderer in draw calls: one per
   *  renderable in it. The canopy's claim is that a whole crown is one
   *  of these, so this is the number that claim is read off. */
  drawCalls: number;
  /** Instanced copies across the subject: the canopy's elements -
   *  the leaf count, after the twigs are shed and the canopy is
   *  culled - and nothing else instances. Read beside
   *  `drawCalls` this is R4's whole claim - a crown of thousands of
   *  leaves arriving as one draw.
   *
   *  Their triangles are deliberately NOT in `triangles` above, which
   *  stays the branch surface's: a canopy's triangle bill is
   *  `instances` times the element's own count, and the renderer's own
   *  figure beside this one in the panel is what reports what was
   *  actually drawn. */
  instances: number;
  /** Leaf stations placed before canopy culling (zero with foliage off). */
  leavesPlaced: number;
  /** Wall-clock milliseconds for growth, radii, surface, foliage and meshes. */
  buildMs: number;
}

function generationRange(counts: readonly number[]): TreeStats["generations"] {
  const total = counts.reduce((sum, count) => sum + count, 0);
  if (total === 0) return null;
  const at = (rank: number): number => {
    let seen = 0;
    for (let i = 0; i < counts.length; i++) {
      seen += counts[i];
      if (seen > rank) return i;
    }
    return counts.length - 1;
  };
  return { min: at(0), median: (at(Math.floor((total - 1) / 2)) + at(Math.floor(total / 2))) / 2,
    max: at(total - 1) };
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
 *  Exact in both directions: `toSkeletonParams`, `toRadiusParams`,
 *  `toSurfaceParams` and `toCanopyParams` applied to the result
 *  reproduce the preset member for member. That round trip is
 *  asserted, because a panel that silently dropped one of a preset's
 *  terms would show the owner a tree nobody authored - and the canopy
 *  is where that would bite hardest, since Laurelin's divergence is
 *  not the golden angle the default is. */
export function presetToParams(preset: TreePreset): GrowerParams {
  const { envelope, bias } = preset.skeleton;
  const canopy = preset.canopy;
  return {
    family: structuredClone((({ id: _id, name: _name, note: _note, ...family }) => family)(preset)),
    supernaturalEnabled: bias.supernatural.enabled,
    seed: preset.skeleton.seed,
    height: envelope.height,
    spread: envelope.spread,
    crownBase: envelope.crownBase,
    fullness: envelope.fullness,
    shoulder: envelope.shoulder,
    torsion: 1,
    gravitropism: bias.gravitropism,
    lean: bias.lean,
    writheAmplitude: bias.supernatural.writheAmplitude,
    writheWavelength: bias.supernatural.writheWavelength,
    spiralRate: bias.supernatural.spiralRate,
    maxTurnPerStep: preset.skeleton.growth.maxTurnPerStep,
    density: (preset.skeleton.attractors - ATTRACTORS_MIN) /
      (ATTRACTORS_MAX - ATTRACTORS_MIN),
    step: preset.skeleton.step,
    twigLength: preset.skeleton.twigs.twig.length,
    angleVariation: preset.skeleton.twigs.angleVariation,
    vigourVariation: preset.skeleton.twigs.vigourVariation,
    twigDiameter: preset.skeleton.twigs.twig.diameter,
    twigStationLength: preset.skeleton.twigs.twig.internodeLength,
    twigStations: preset.skeleton.twigs.twig.stationsPerInternode,
    twigBearing: preset.skeleton.twigs.twig.bearingDiameter,
    ratioPower: preset.skeleton.twigs.ratioPower,
    limbRadius: preset.skeleton.twigs.limbRadius,
    reach: preset.skeleton.twigs.reach,
    laterals: preset.skeleton.twigs.laterals,
    twigAngle: preset.skeleton.twigs.angle,
    twigDivergence: preset.skeleton.twigs.divergence,
    internodeFactor: preset.skeleton.twigs.internodeFactor,
    lengthRatio: preset.skeleton.twigs.lengthRatio,
    taper: preset.radii.forkExponent,
    trunkRadius: preset.radii.trunkRadius,
    lengthTaper: preset.radii.lengthTaper,
    lobes: preset.surface.lobes,
    lobeDepth: preset.surface.lobeDepth,
    twistRate: preset.surface.twistRate,
    flareRadius: preset.surface.flareRadius,
    shootRadius: canopy.shootRadius,
    spacing: canopy.spacing,
    divergence: canopy.divergence,
    clump: canopy.clump,
    clumpSpan: canopy.clumpSpan,
    outward: canopy.outward,
    upward: canopy.upward,
    scatter: canopy.scatter,
    size: canopy.size,
    sizeVariation: canopy.sizeVariation,
  };
}

function build(
  family: Family, clay: Clay, withFoliage = true,
): { tree: THREE.Group; stats: TreeStats } {
  const started = performance.now();
  const engine = treeCore();
  try {
    const output = engine.build(family,
      { surface: true, foliage: withFoliage });
    engine.release();
    const materialization = performance.now();
    const tree = materializeTree(output, clay);
    const materializationMs = performance.now() - materialization;
    const d = output.diagnostics;
    return { tree, stats: {
      triangles: (output.surface?.indices.length ?? 0) / 3,
      vertices: (output.surface?.positions.length ?? 0) / 3,
      nodes: d.nodes, capped: d.capped, levelCapped: d.levelCapped,
      attractionCapped: d.attractionCapped, complete: d.complete,
      handoffs: d.handoffs, generationCounts: d.generationCounts,
      generations: generationRange(d.generationCounts),
      levelCappedHandoffs: d.levelCappedHandoffs, twigs: d.twigs,
      ...countDraws(tree), leavesPlaced: d.leavesPlaced,
      timings: { ...d.timings, materializationMs }, buildMs: performance.now() - started,
    } };
  } finally { engine.release(); }
}

/** The subject the stage draws: this tree, in clay, skinned.
 *
 *  Deterministic in `params` - same seed and dials, same mesh, vertex
 *  for vertex - because every stage of it is. */
export function buildTree(
  params: GrowerParams,
  clay: Clay,
  withFoliage = true,
): { tree: THREE.Group; stats: TreeStats } {
  return build(
    { ...params.family, skeleton: toSkeletonParams(params), radii: toRadiusParams(params),
      surface: toSurfaceParams(params), canopy: toCanopyParams(params) },
    clay,
    withFoliage,
  );
}

/** One named tree, exactly as authored. Straight from the preset's own
 *  three objects rather than round-tripped through the dials, so what
 *  the owner judges is what the library file says and not what the
 *  panel could express of it. */
export function buildPreset(
  preset: TreePreset,
  clay: Clay,
  withFoliage = true,
): { tree: THREE.Group; stats: TreeStats } {
  return build(
    preset,
    clay,
    withFoliage,
  );
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
  withFoliage = true,
): { group: THREE.Group; stats: TreeStats } {
  const group = new THREE.Group();
  group.name = "grower-comparison";

  const built: { preset: TreePreset; tree: THREE.Group; stats: TreeStats }[] = [];
  try {
    for (const preset of presets) built.push({ preset, ...buildPreset(preset, clay, withFoliage) });
  } catch (error) {
    for (const one of built) disposeTreeGeometry(one.tree);
    throw error;
  }

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

  const generationCounts = Array<number>(Math.max(0, ...built.map(one => one.stats.generationCounts.length))).fill(0);
  for (const one of built) {
    one.stats.generationCounts.forEach((count, generation) => {
      generationCounts[generation] += count;
    });
  }
  return {
    group,
    stats: {
      triangles: built.reduce((total, one) => total + one.stats.triangles, 0),
      vertices: built.reduce((total, one) => total + one.stats.vertices, 0),
      nodes: built.reduce((total, one) => total + one.stats.nodes, 0),
      // One capped tree is a comparison that cannot be judged.
      capped: built.some((one) => one.stats.capped),
      attractionCapped: built.some(one => one.stats.attractionCapped),
      complete: built.every(one => one.stats.complete),
      timings: {
        growthMs: built.reduce((sum, one) => sum + one.stats.timings.growthMs, 0),
        surfaceMs: built.reduce((sum, one) => sum + one.stats.timings.surfaceMs, 0),
        foliageMs: built.reduce((sum, one) => sum + one.stats.timings.foliageMs, 0),
        fieldMs: built.reduce((sum, one) => sum + one.stats.timings.fieldMs, 0),
        coreMs: built.reduce((sum, one) => sum + one.stats.timings.coreMs, 0),
        transferMs: built.reduce((sum, one) => sum + one.stats.timings.transferMs, 0),
        buildMs: built.reduce((sum, one) => sum + one.stats.timings.buildMs, 0),
        materializationMs: built.reduce((sum, one) => sum + one.stats.timings.materializationMs, 0),
      },
      levelCapped: built.some((one) => one.stats.levelCapped),
      handoffs: built.reduce((total, one) => total + one.stats.handoffs, 0),
      generations: generationRange(generationCounts),
      generationCounts,
      levelCappedHandoffs: built.reduce((total, one) => total + one.stats.levelCappedHandoffs, 0),
      twigs: built.reduce((total, one) => total + one.stats.twigs, 0),
      /* Draws and instances sum the same way the rest do, because two
         trees standing side by side really are two subjects' worth of
         work: each carries its own surface and, when the canopy lands,
         its own instanced crown. What is NOT summed twice is a
         geometry either of them might share - that would be counting
         one buffer's triangles per copy of it, and triangles come from
         the surface each tree built rather than from the graph. */
      drawCalls: built.reduce((total, one) => total + one.stats.drawCalls, 0),
      instances: built.reduce((total, one) => total + one.stats.instances, 0),
      leavesPlaced: built.reduce((total, one) => total + one.stats.leavesPlaced, 0),
      buildMs: built.reduce((total, one) => total + one.stats.buildMs, 0),
    },
  };
}
