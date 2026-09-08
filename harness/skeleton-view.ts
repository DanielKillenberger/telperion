import * as THREE from "three";

import { treeCore, type Family, type TreePreset, type Timings } from "../src/browser/core";
import { materializeTree, disposeTreeGeometry, recomputeInstanceBounds } from "../src/browser/three";
import type { GrowerParams } from "./params";
import type { Clay } from "./stage";
import { toFamily } from "./family";

/* The dial-to-family composition moved to `family.ts`, which knows
   nothing about three.js; this stage is the last thing that does. Kept
   re-exported here so the old stage's own callers still reach it. */
export {
  presetToParams,
  toCanopyParams,
  toRadiusParams,
  toSkeletonParams,
  toSurfaceParams,
} from "./family";

/** Materializes owned Rust outputs for the clay stage. Botanical generation
 * stays in Rust; the dial composition stays in `family.ts`. */

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
  return build(toFamily(params), clay, withFoliage);
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

export type SpecimenView = "whole" | "bare" | "foliage-detail";

/** Detail isolates a real placed biological unit, connector included. It keeps
 * the generated scale and orientation; it does not substitute a display asset. */
export function selectSpecimenView(tree: THREE.Group, view: SpecimenView): THREE.Group {
  tree.userData.specimenView = view;
  if (view === "whole") return tree;
  const remove: THREE.Object3D[] = [];
  tree.traverse(node => {
    if (!(node instanceof THREE.Mesh)) return;
    if (view === "bare" ? node instanceof THREE.InstancedMesh : !(node instanceof THREE.InstancedMesh)) {
      remove.push(node);
    } else if (view === "foliage-detail" && node instanceof THREE.InstancedMesh && node.count > 0) {
      const matrix = new THREE.Matrix4();
      node.getMatrixAt(Math.floor(node.count / 2), matrix);
      // View along the prototype's broad face, transformed into this
      // specimen's orientation, so needles are not shown end-on.
      tree.userData.detailDirection ??= new THREE.Vector3(0.35, 0.2, 1).transformDirection(matrix);
      node.setMatrixAt(0, matrix);
      node.count = 1;
      node.instanceMatrix.needsUpdate = true;
      recomputeInstanceBounds(node);
    }
  });
  for (const node of remove) { node.removeFromParent(); disposeTreeGeometry(node); }
  // Keep comparison details alongside each other at their own scale.
  if (view === "foliage-detail" && tree.children.some(child => child instanceof THREE.Group)) {
    let right = 0;
    for (const child of tree.children) {
      const bounds = new THREE.Box3().setFromObject(child);
      if (bounds.isEmpty()) continue;
      const size = bounds.getSize(new THREE.Vector3());
      child.position.add(new THREE.Vector3(right - bounds.min.x, -bounds.min.y, -bounds.getCenter(new THREE.Vector3()).z));
      right += size.x * 1.5;
    }
  }
  return tree;
}
