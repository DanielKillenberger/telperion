import * as THREE from "three";
import { describe, expect, it } from "vitest";

import type { Envelope } from "../envelope";
import { branchPaths } from "../mesh/paths";
import { buildSurface, type SurfaceParams } from "../mesh/surface";
import { LAURELIN, TELPERION, type TreePreset } from "../presets";
import { solveRadii, type RadiusField } from "../radius";
import type { Skeleton } from "./colonize";
import { growSkeleton } from "./grow";
import { childRadius } from "./law";
import type { TwiggedSkeleton } from "./twigs";

const DEG = 180 / Math.PI;
const GENERATIONS_ABOVE = 10;
const GENERATIONS_BELOW = 4;
const MIN_WOOD = 1e-5;
// Measured on the seam edge alone: Telperion p90 6.2855 degrees,
// Laurelin 5.1131; medians 3.8215/2.8755, worst 16.1611/33.9539.
// Seven degrees is tighter than fn-5's eight-degree bound. Radius and
// per-internode rate assertions independently bind the mechanism.
const TAPER_TOLERANCE_DEG = 7;
// Four float32 ulps at the vertex's distance from the origin.
const VERTEX_SLACK = 2 ** -22;

function grown(preset: TreePreset): TwiggedSkeleton {
  return growSkeleton(preset.skeleton, preset.radii);
}

/** Each edge crossing from colonization into the pass is a separate seam.
 * Empty selections are untested, including a pass stopped at its ceiling. */
function seam(skeleton: Skeleton): { crossover: number; handoffs: number[] } | { untested: string } {
  if (!("crossover" in skeleton)) return { untested: "the skeleton carries no crossover: not built by the twig pass" };
  const { crossover, nodes } = skeleton as TwiggedSkeleton;
  const handoffs: number[] = [];
  for (let i = Math.max(1, crossover); i < nodes.length; i++) {
    if (nodes[i].parent >= 0 && nodes[i].parent < crossover) handoffs.push(i);
  }
  return handoffs.length ? { crossover, handoffs } : { untested: "no handoff edges in the grown range" };
}

function childrenOf(skeleton: Skeleton): number[][] {
  const children: number[][] = skeleton.nodes.map(() => []);
  skeleton.nodes.forEach((node, i) => {
    if (node.parent >= 0) children[node.parent].push(i);
  });
  return children;
}

/** Internodes keep their generation; only a new branch increments it. */
function generationsOf(skeleton: TwiggedSkeleton): Int32Array {
  const generation = new Int32Array(skeleton.nodes.length);
  for (let i = skeleton.crossover; i < skeleton.nodes.length; i++) {
    generation[i] = generation[skeleton.nodes[i].parent] + Number(skeleton.branchId[i - skeleton.crossover] === i);
  }
  return generation;
}

function edgesAbove(skeleton: Skeleton, tip: number, crossover: number, children: number[][], envelope: Envelope): number[] {
  const above: number[] = [];
  let forks = 0;
  for (let at = tip; at > 0 && forks < GENERATIONS_ABOVE; at = skeleton.nodes[at].parent) {
    const parent = skeleton.nodes[at].parent;
    if (skeleton.nodes[parent].position.y < envelope.height * envelope.crownBase) break;
    above.push(at);
    if (children[parent].filter(i => i < crossover).length > 1) forks++;
  }
  return above;
}

function edgesBelow(handoff: number, children: number[][], generations: Int32Array): number[] {
  const below = [handoff];
  for (let k = 0; k < below.length; k++) {
    for (const child of children[below[k]]) {
      if (generations[child] <= GENERATIONS_BELOW) below.push(child);
    }
  }
  return below;
}

function held(envelope: Envelope, skeleton: Skeleton, field: RadiusField, child: number): void {
  expect(envelope.height).toBeGreaterThan(0);
  const parent = skeleton.nodes[child].parent;
  for (const value of [field.radius[parent], field.startRadius[child], field.radius[child]]) {
    expect(Number.isFinite(value)).toBe(true);
    expect(value / envelope.height, `wood at edge into node ${child}`).toBeGreaterThan(MIN_WOOD);
  }
}

function arrival(skeleton: Skeleton, i: number): THREE.Vector3 {
  return skeleton.nodes[i].position.clone().sub(skeleton.nodes[skeleton.nodes[i].parent].position).normalize();
}

const presets = [["Telperion", TELPERION], ["Laurelin", LAURELIN]] as const;

describe("the crossover: every handoff and the generations either side", () => {
  it.each(presets)("on %s, radius ratios obey the law and the ten-generation range", (_name, preset) => {
    const skeleton = grown(preset);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const children = childrenOf(skeleton);
    const generations = generationsOf(skeleton);
    const law = preset.skeleton.twigs;
    const share = childRadius(1, law.lengthRatio, law.ratioPower);
    const seen = new Int32Array(GENERATIONS_BELOW + 1);
    // Preserve fn-5's crown range, sampling each handoff's own lineage.
    // A lineage with only forks has no continuation ratio of one; the
    // range therefore includes all sampled lineages, as it did before.
    const above = new Set<number>();
    for (const handoff of at.handoffs) {
      const lineage = edgesAbove(skeleton, skeleton.nodes[handoff].parent, at.crossover, children, envelope);
      expect(lineage.length, `lineage above ${handoff}`).toBeGreaterThan(0);
      for (const child of lineage) above.add(child);
    }
    const ratios = [...above].map(child => {
      held(envelope, skeleton, field, child);
      return field.startRadius[child] / field.radius[skeleton.nodes[child].parent];
    });
    const low = Math.min(...ratios), high = Math.max(...ratios);
    expect(above.size).toBeGreaterThan(500);
    expect(low).toBeGreaterThan(0);
    expect(high).toBeLessThanOrEqual(1 + 1e-12);
    for (const handoff of at.handoffs) {
      for (const child of edgesBelow(handoff, children, generations)) {
        held(envelope, skeleton, field, child);
        const record = child - at.crossover;
        const parent = skeleton.nodes[child].parent;
        const ratio = field.startRadius[child] / field.radius[parent];
        if (skeleton.twig[record]) {
          // Fixed anatomy is the sole exception to the branch radius law.
          expect(field.startRadius[child]).toBe(law.twig.diameter / 2);
          expect(field.radius[child]).toBe(law.twig.diameter / 2);
          continue;
        }
        expect(ratio, `handoff ${handoff}, branch edge ${child}`).toBeGreaterThanOrEqual(low - 1e-12);
        expect(ratio, `handoff ${handoff}, branch edge ${child}`).toBeLessThanOrEqual(high + 1e-12);
        if (child === handoff) {
          expect(ratio).toBeGreaterThanOrEqual(share);
          const tip = children[parent].every(i => i >= at.crossover);
          const expected = tip ? field.radius[parent]
            : childRadius(field.radius[parent], law.lengthRatio, law.ratioPower);
          expect(field.startRadius[child]).toBe(expected);
        } else if (skeleton.branchId[record] === child) {
          expect(field.startRadius[child]).toBe(childRadius(skeleton.baseRadius[parent - at.crossover], law.lengthRatio, law.ratioPower));
        } else {
          expect(field.startRadius[child]).toBe(field.radius[parent]);
        }
        seen[generations[child]]++;
      }
    }
    expect(at.handoffs.length).toBeGreaterThan(100);
    for (let k = 1; k <= GENERATIONS_BELOW; k++) expect(seen[k]).toBeGreaterThan(100);
  });

  it.each(presets)("on %s, every step across and beside each handoff stays within the turn limit", (_name, preset) => {
    const skeleton = grown(preset);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const children = childrenOf(skeleton);
    const generations = generationsOf(skeleton);
    const sampled = new Set<number>();
    for (const handoff of at.handoffs) {
      const above = edgesAbove(skeleton, skeleton.nodes[handoff].parent, at.crossover, children, envelope);
      const below = edgesBelow(handoff, children, generations);
      expect(above.length).toBeGreaterThan(0);
      expect(below[0]).toBe(handoff);
      for (const child of [...above, ...below]) sampled.add(child);
    }
    for (const child of sampled) {
      held(envelope, skeleton, field, child);
      const parent = skeleton.nodes[child].parent;
      const turn = parent === 0 ? 0 : Math.acos(THREE.MathUtils.clamp(arrival(skeleton, child).dot(arrival(skeleton, parent)), -1, 1)) * DEG;
      expect(turn, `edge ${child}`).toBeLessThanOrEqual(preset.skeleton.growth.maxTurnPerStep + 1e-6);
    }
    expect(sampled.size).toBeGreaterThan(1000);
  });

  it.each(presets)("on %s, the seam taper stays within the measured drawn-angle tolerance", (name, preset) => {
    const skeleton = grown(preset);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const children = childrenOf(skeleton);
    const generations = generationsOf(skeleton);
    const slope = (child: number) => {
      const parent = skeleton.nodes[child].parent;
      const length = skeleton.nodes[parent].position.distanceTo(skeleton.nodes[child].position);
      return Math.atan((field.radius[parent] - field.radius[child]) / length) * DEG;
    };
    const differences: number[] = [];
    for (const handoff of at.handoffs) {
      const above = edgesAbove(skeleton, skeleton.nodes[handoff].parent, at.crossover, children, envelope);
      expect(above.length).toBeGreaterThan(0);
      for (const child of above) held(envelope, skeleton, field, child);
      const mean = above.reduce((sum, child) => sum + slope(child), 0) / above.length;
      differences.push(Math.abs(slope(handoff) - mean));
      // The seam alone is measured, never diluted by averaging below it.
      // Below it, check the rate along every sampled branch internode.
      for (const child of edgesBelow(handoff, children, generations)) {
        const record = child - at.crossover;
        const parent = skeleton.nodes[child].parent;
        const length = skeleton.nodes[parent].position.distanceTo(skeleton.nodes[child].position);
        const rate = Math.log(field.startRadius[child] / field.radius[child]) / length;
        expect(rate, `internode ${child}`).toBeCloseTo(skeleton.twig[record] ? 0 : preset.radii.lengthTaper / envelope.height, 10);
      }
    }
    differences.sort((a, b) => a - b);
    const p90 = differences[Math.floor(differences.length * 0.9)];
    const measurement = `${name}: ${differences.length} handoffs, median ${differences[differences.length >> 1]}, p90 ${p90}, worst ${differences.at(-1)} degrees`;
    console.log(measurement);
    expect(p90, measurement).toBeLessThanOrEqual(TAPER_TOLERANCE_DEG);
  });
});

describe("the crossover: the surface as drawn", () => {
  function segmentsOf(params: SurfaceParams): number {
    return Math.min(64, Math.max(3, Math.round(params.lobes) * 4, Math.round(params.radialSegments)));
  }

  it.each(presets)("on %s, every handoff has a shared ring or a contained socket, and nearby forks contain every vertex", (_name, preset) => {
    const skeleton = grown(preset);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const surface = preset.surface;
    const segments = segmentsOf(surface);
    const mesh = buildSurface(skeleton, field, envelope, surface);
    const generations = generationsOf(skeleton);
    const children = childrenOf(skeleton);
    const aboveSeam = new Set(at.handoffs.flatMap(i => edgesAbove(skeleton, skeleton.nodes[i].parent, at.crossover, children, envelope)));
    const handoffs = new Set(at.handoffs);
    const checkedHandoffs = new Set<number>();
    const flare = (y: number) => 1 + (surface.flareRadius - 1) * Math.exp(-Math.max(0, y) / (surface.flareFalloff * envelope.height));
    const vertex = (i: number) => new THREE.Vector3(mesh.positions[i * 3], mesh.positions[i * 3 + 1], mesh.positions[i * 3 + 2]);
    let offset = 0, checked = 0;
    for (const path of branchPaths(skeleton, field)) {
      const buried = Number(path.trunk && surface.flareDepth > 0);
      const rings = path.nodes.length + buried;
      const attach = path.nodes[0], first = path.nodes[1];
      if (!path.trunk) {
        const atSeam = handoffs.has(first);
        const below = attach >= at.crossover && generations[attach] <= GENERATIONS_BELOW;
        const above = attach < at.crossover && aboveSeam.has(attach);
        if (atSeam || below || above) {
          held(envelope, skeleton, field, first);
          const centre = skeleton.nodes[attach].position;
          const inscribed = field.radius[attach] * flare(centre.y) * (1 - surface.lobeDepth) * Math.cos(Math.PI / segments);
          const slack = centre.length() * VERTEX_SLACK;
          for (let k = 0; k < segments; k++) {
            expect(vertex(offset + k).distanceTo(centre), `socket ${attach}, vertex ${k}`).toBeLessThanOrEqual(inscribed + slack);
            checked++;
          }
          if (atSeam) checkedHandoffs.add(first);
        }
      }
      // A tip leader continues an existing mesh run. Its two incident
      // segments use the same ring, whose radius includes the run's swell.
      let along = 0;
      for (let k = 1; k < path.nodes.length; k++) {
        const parent = path.nodes[k - 1], child = path.nodes[k];
        if (handoffs.has(child) && k > 1) {
          const centre = skeleton.nodes[parent].position;
          const swell = path.trunk ? 1 : 1 + (surface.forkSwell - 1) * Math.exp(-along / Math.max(1e-9, field.radius[attach]));
          const radius = field.radius[parent] * flare(centre.y) * swell;
          const slack = centre.length() * VERTEX_SLACK;
          const ring = offset + (k - 1 + buried) * segments;
          for (let j = 0; j < segments; j++) {
            const distance = vertex(ring + j).distanceTo(centre);
            expect(distance).toBeGreaterThanOrEqual(radius * (1 - surface.lobeDepth) - slack);
            expect(distance).toBeLessThanOrEqual(radius * (1 + surface.lobeDepth) + slack);
            checked++;
          }
          checkedHandoffs.add(child);
        }
        along += skeleton.nodes[parent].position.distanceTo(skeleton.nodes[child].position);
      }
      offset += rings * segments + 2;
    }
    expect(offset).toBe(mesh.vertices);
    expect([...checkedHandoffs].sort((a, b) => a - b)).toEqual(at.handoffs);
    expect(checkedHandoffs.size).toBeGreaterThan(100);
    expect(checked).toBeGreaterThan(checkedHandoffs.size * segments);
  });
});

describe("the crossover: the error case", () => {
  it("reports an empty handoff list as untested, never continuous", () => {
    const full = grown(TELPERION);
    const rested = { ...full, nodes: full.nodes.slice(0, full.crossover) };
    expect(seam(rested)).toEqual({ untested: "no handoff edges in the grown range" });
    expect(seam({ nodes: rested.nodes })).toEqual({ untested: "the skeleton carries no crossover: not built by the twig pass" });
    const capped = growSkeleton({ ...TELPERION.skeleton,
      growth: { ...TELPERION.skeleton.growth, maxNodes: rested.nodes.length },
    }, TELPERION.radii);
    expect("untested" in seam(capped)).toBe(true);
    expect("handoffs" in seam(full)).toBe(true);
  });
});
