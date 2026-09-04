import * as THREE from "three";
import { describe, expect, it } from "vitest";

import type { Envelope } from "../envelope";
import { branchPaths } from "../mesh/paths";
import { buildSurface, type SurfaceParams } from "../mesh/surface";
import { LAURELIN, TELPERION, type TreePreset } from "../presets";
import { solveRadii, type RadiusField } from "../radius";
import type { Skeleton } from "./colonize";
import { growSkeleton } from "./grow";
import type { TwiggedSkeleton } from "./twigs";

/* THE CROSSOVER IS AN INVARIANT, NOT AN INTERFACE (R2).

   Radius, direction and taper are continuous where colonization hands
   over to the local rules, and "continuous" means exactly three things:
   the parent-to-child radius ratio at the crossover lies inside the
   range that ratio takes over the ten fork generations above it; the
   direction change is within the turn limit growth already enforces;
   and the taper rate, radius change per unit length, is within a stated
   tolerance of the rate immediately above. Nothing is added by comment.

   Every sample is taken several generations either side of the seam,
   never only on the boundary edge, and every sample is first held to
   the envelope's own local dimensions - the crown has width where the
   sample sits, and the wood sampled is a stated fraction of that width
   - so a region where both sides have collapsed to nothing fails here
   rather than passing as "no difference". A tree with no crossover in
   its grown range is reported as untested, not as continuous.

   The surface half asserts on the mesh as drawn, at every vertex of
   the junction ring against the lobed, finitely sampled parent - a
   test that checks a ring's centre tests placement, not containment. */

const DEG = 180 / Math.PI;

/** Fork generations above a crossover node the range is taken over:
 *  the AC's own number. */
const GENERATIONS_ABOVE = 10;
/** Orders below the crossover that are sampled beside the boundary
 *  edge itself. Four is past the reach of one step across the seam,
 *  and a tree at rest still has every one of them. */
const ORDERS_BELOW = 4;
/** The finest wood any sample may be, as a fraction of the envelope's
 *  height - the dimension every radius in the library is authored in.
 *  Measured at the resting twig: the finest wood within four orders of
 *  the seam is 2.4e-4 of height on Telperion and 5.3e-4 on Laurelin,
 *  and colonization's own finest is 2.7e-3 and 4.4e-3; a solve that
 *  had collapsed to the trunk floor lands at 4e-6. The floor sits a
 *  decade under the wood and above the collapse, so a seam where both
 *  sides have gone to nothing fails before any ratio is formed. The
 *  crown's half-width at the sample's height is not the scale, on
 *  purpose: twigs and the tips they leave cross the crown's top, where
 *  the authored width is zero, and wood there is wood. */
const MIN_WOOD = 1e-5;
/** Taper tolerance at the seam, in degrees of drawn half-angle, on the
 *  ninetieth percentile of runs. Measured tight on the seam edge alone,
 *  which is the edge the criterion is about: Telperion median 1.3, p90
 *  7.75, worst 19.9; Laurelin median 1.4, p90 7.44, worst 42.5. The
 *  worst cases are blunt tips handing a limb's radius to twigs, which
 *  is fn-4's spec. An earlier version averaged the seam edge with the
 *  four below it and read p90 5.6 and 5.3, and the audit showed a x0.5
 *  step at the seam hiding inside that mean; 8 on the seam edge alone
 *  is the number the shipped law measures at. Its reach is bounded by
 *  twig radii: half-angles at twig scale are small, so by mutation this
 *  bound catches a x0.25 taper jump along the fine-order edges and not
 *  a x0.5 one, and a twigTaper of 2.0 does not move it at all because
 *  that exponent scales a length ratio near one at rest. The seam's
 *  radius bound and the junction-ring bound are the load-bearing ones;
 *  this is the backstop for a taper failure they would not see. */
const TAPER_TOLERANCE_DEG = 8;
/** How far under the law's own share a seam edge may sit. Set from the
 *  tight measurement on both presets (printed with any failure); a
 *  thinning of x3.7 at the seam, which the range test admitted, lands at
 *  0.27 of the share and fails here. */
const SEAM_SHARE_FLOOR = 0.8;
/** Slack on a per-vertex containment, in units of the vertex's own
 *  magnitude: the mesh stores float32, whose spacing at 100 m from the
 *  origin is 7.6e-6 m, and a 6 cm ring drawn there carries that error
 *  in every coordinate. Four ulps. */
const VERTEX_SLACK = 2 ** -22;

/** A preset with `levels` twig orders under it and the node ceiling
 *  lifted, so the orders asked for are the orders measured. */
function grown(preset: TreePreset, levels: number): Skeleton {
  const tree = preset.skeleton;
  return growSkeleton({
    ...tree,
    twigs: { ...tree.twigs, levels },
    growth: { ...tree.growth, maxNodes: 4_000_000 },
  });
}

/** Where the seam is, or why there is none to test. A skeleton the
 *  twig pass never marked, or one it marked at its own end, has no
 *  crossover inside its grown range: there is no discontinuity to
 *  find, and a pass over zero samples is not a pass. */
function seam(
  skeleton: Skeleton,
): { crossover: number } | { untested: string } {
  if (!("crossover" in skeleton)) {
    return { untested: "the skeleton carries no crossover: not built by the twig pass" };
  }
  const { crossover, nodes } = skeleton as TwiggedSkeleton;
  if (!(crossover >= 1 && crossover < nodes.length)) {
    return {
      untested: `crossover ${crossover} is outside the grown range of ${nodes.length} nodes: no twig orders`,
    };
  }
  return { crossover };
}

function childrenOf(skeleton: Skeleton): number[][] {
  const children: number[][] = skeleton.nodes.map(() => []);
  skeleton.nodes.forEach((node, index) => {
    if (node.parent >= 0) children[node.parent].push(index);
  });
  return children;
}

/** Order below the crossover: 0 above it, the parent's plus one below. */
function ordersOf(skeleton: Skeleton, crossover: number): Int32Array {
  const order = new Int32Array(skeleton.nodes.length);
  for (let i = crossover; i < skeleton.nodes.length; i += 1) {
    order[i] = order[skeleton.nodes[i].parent] + 1;
  }
  return order;
}

/** Edges (child indices) within `GENERATIONS_ABOVE` fork generations
 *  above any crossover node: walked up from every colonization tip
 *  that has twigs, counting forks passed. */
function edgesAbove(skeleton: Skeleton, crossover: number, envelope: Envelope): Set<number> {
  const children = childrenOf(skeleton);
  const crownBase = envelope.height * envelope.crownBase;
  const above = new Set<number>();
  for (let i = crossover; i < skeleton.nodes.length; i += 1) {
    const tip = skeleton.nodes[i].parent;
    if (tip >= crossover) continue;
    let at = tip;
    let forks = 0;
    // The generations above are the crown's: the bare trunk under it
    // has no forks and no width, and is not a generation of anything.
    while (at > 0 && forks < GENERATIONS_ABOVE) {
      const parent = skeleton.nodes[at].parent;
      if (skeleton.nodes[parent].position.y < crownBase) break;
      above.add(at);
      if (children[parent].length > 1) forks += 1;
      at = parent;
    }
  }
  return above;
}

/** The sample held to the envelope's dimensions before any comparison
 *  is made on it: the envelope has a height, and the radii on both ends
 *  of the edge are a stated fraction of it at least. This is what makes
 *  a region where the wood has gone to nothing fail rather than pass. */
function held(
  envelope: Envelope,
  skeleton: Skeleton,
  field: RadiusField,
  child: number,
): void {
  const parent = skeleton.nodes[child].parent;
  expect(envelope.height).toBeGreaterThan(0);
  for (const value of [field.radius[parent], field.startRadius[child], field.radius[child]]) {
    expect(Number.isFinite(value)).toBe(true);
    expect(value / envelope.height, `wood at edge into node ${child}`).toBeGreaterThan(MIN_WOOD);
  }
}

function arrival(skeleton: Skeleton, index: number): THREE.Vector3 {
  const node = skeleton.nodes[index];
  return node.position.clone().sub(skeleton.nodes[node.parent].position).normalize();
}

const presets = [
  ["Telperion", TELPERION],
  ["Laurelin", LAURELIN],
] as const;

describe("the crossover: radius", () => {
  it.each(presets)("on %s, the ratio at the seam stays inside the range the ten generations above take", (_name, preset) => {
    const skeleton = grown(preset, 8);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const { crossover } = at;
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const order = ordersOf(skeleton, crossover);

    /* The range above, over every edge within ten fork generations of
       a colonization tip that has twigs, each edge held to the local
       width first. */
    let low = Number.POSITIVE_INFINITY;
    let high = 0;
    const above = edgesAbove(skeleton, crossover, envelope);
    for (const child of above) {
      held(envelope, skeleton, field, child);
      const ratio = field.startRadius[child] / field.radius[skeleton.nodes[child].parent];
      low = Math.min(low, ratio);
      high = Math.max(high, ratio);
    }
    expect(above.size).toBeGreaterThan(500);
    expect(low).toBeGreaterThan(0);
    expect(high).toBeLessThanOrEqual(1 + 1e-12);

    /* The seam and the orders below it: the boundary edge (order 1)
       and every edge down to ORDERS_BELOW, each inside that range. */
    /* The range above is the crown's loosest fork, and the audit showed
       a deliberate x3.7 thinning at the seam sliding under it. So the
       seam edge itself is bound to the law it must obey: a child of a
       parent with k children leaves it at k^(-1/n) of the parent's
       radius, thinned only by its own length ratio, which at rest is
       one step against one step. Above SEAM_SHARE_FLOOR of that share
       and never over it. Measured tight on both presets: the minimum
       seam ratio over share is printed with the failure so the floor
       is set from the trees, not the other way round. */
    const kids = childrenOf(skeleton);
    const exponent = preset.radii.forkExponent;
    const seen = new Int32Array(ORDERS_BELOW + 1);
    let seamOverShareMin = Number.POSITIVE_INFINITY;
    for (let i = crossover; i < skeleton.nodes.length; i += 1) {
      if (order[i] > ORDERS_BELOW) continue;
      held(envelope, skeleton, field, i);
      const parent = skeleton.nodes[i].parent;
      const ratio = field.startRadius[i] / field.radius[parent];
      if (order[i] === 1) {
        const share = kids[parent].length ** (-1 / exponent);
        seamOverShareMin = Math.min(seamOverShareMin, ratio / share);
        expect(ratio / share, `seam edge into node ${i}: ratio ${ratio.toFixed(4)} against share ${share.toFixed(4)}`).toBeGreaterThanOrEqual(SEAM_SHARE_FLOOR);
        expect(ratio, `seam edge into node ${i}`).toBeLessThanOrEqual(share * (1 + 1e-9));
      } else {
        expect(ratio, `order ${order[i]} edge into node ${i}`).toBeGreaterThanOrEqual(low);
        expect(ratio, `order ${order[i]} edge into node ${i}`).toBeLessThanOrEqual(high);
      }
      seen[order[i]] += 1;
    }
    expect(seamOverShareMin, `tightest seam ratio over share: ${seamOverShareMin.toFixed(4)}`).toBeGreaterThanOrEqual(SEAM_SHARE_FLOOR);
    for (let k = 1; k <= ORDERS_BELOW; k += 1) expect(seen[k]).toBeGreaterThan(100);
  });
});

describe("the crossover: direction", () => {
  it.each(presets)("on %s, no step across or beside the seam turns further than the run's limit", (_name, preset) => {
    const skeleton = grown(preset, ORDERS_BELOW);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const { crossover } = at;
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const limit = preset.skeleton.growth.maxTurnPerStep;
    const above = edgesAbove(skeleton, crossover, envelope);

    const turn = (child: number): number => {
      const parent = skeleton.nodes[child].parent;
      // The root has no arrival; the trunk's first step is unturned.
      if (parent === 0) return 0;
      return Math.acos(Math.min(1, arrival(skeleton, child).dot(arrival(skeleton, parent)))) * DEG;
    };
    let sampled = 0;
    for (const child of above) {
      held(envelope, skeleton, field, child);
      expect(turn(child)).toBeLessThanOrEqual(limit + 1e-6);
      sampled += 1;
    }
    for (let i = crossover; i < skeleton.nodes.length; i += 1) {
      held(envelope, skeleton, field, i);
      expect(turn(i), `twig node ${i}`).toBeLessThanOrEqual(limit + 1e-6);
      sampled += 1;
    }
    expect(sampled).toBeGreaterThan(1000);
  });
});

describe("the crossover: taper", () => {
  it.each(presets)("on %s, a run's drawn slope below the seam is within tolerance of the slope above it", (_name, preset) => {
    /* The rate the surface draws along a run - the ring-to-ring slope
       between consecutive nodes, as a taper half-angle - averaged over
       the orders below the seam on each run that crosses it, against
       the same average over the ten fork generations above on that
       run. Measured before the tolerance was chosen, on both presets
       at the resting twig; the largest difference and the count of
       runs over the tight tolerance are in the assertion messages. */
    const skeleton = grown(preset, 8);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const { crossover } = at;
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const order = ordersOf(skeleton, crossover);
    const children = childrenOf(skeleton);

    const slope = (from: number, to: number): number => {
      const length = skeleton.nodes[from].position.distanceTo(skeleton.nodes[to].position);
      return Math.atan((field.radius[from] - field.radius[to]) / length) * DEG;
    };

    let runs = 0;
    let worst = 0;
    const differences: number[] = [];
    for (const path of branchPaths(skeleton, field)) {
      const nodes = path.nodes;
      const cross = nodes.findIndex((node, k) => k > 0 && node >= crossover && nodes[k - 1] < crossover);
      if (cross < 0) continue;
      // Above: back from the seam node until ten forks have been passed.
      let sumAbove = 0;
      let countAbove = 0;
      let forks = 0;
      const crownBase = envelope.height * envelope.crownBase;
      for (let k = cross - 1; k > 0 && forks < GENERATIONS_ABOVE; k -= 1) {
        if (skeleton.nodes[nodes[k - 1]].position.y < crownBase) break;
        held(envelope, skeleton, field, nodes[k]);
        sumAbove += slope(nodes[k - 1], nodes[k]);
        countAbove += 1;
        if (children[nodes[k - 1]].length > 1) forks += 1;
      }
      if (countAbove === 0) continue;
      // Below: the seam edge and the orders after it along this run.
      /* The seam edge alone. Averaging it with the orders after it let a
         x0.5 step at the seam hide inside the mean; the audit executed
         that and the p90 stayed under tolerance. The edge that crosses
         the method boundary is the one the criterion is about. */
      held(envelope, skeleton, field, nodes[cross]);
      const difference = Math.abs(slope(nodes[cross - 1], nodes[cross]) - sumAbove / countAbove);
      differences.push(difference);
      worst = Math.max(worst, difference);
      runs += 1;
    }
    expect(runs).toBeGreaterThan(100);
    differences.sort((a, b) => a - b);
    const over = differences.filter((d) => d > TAPER_TOLERANCE_DEG).length;
    const ninetieth = differences[Math.floor(runs * 0.9)];
    expect(
      ninetieth,
      `${over} of ${runs} runs over ${TAPER_TOLERANCE_DEG} deg; median ${differences[runs >> 1].toFixed(3)}, p90 ${ninetieth.toFixed(3)}, worst ${worst.toFixed(3)} deg`,
    ).toBeLessThanOrEqual(TAPER_TOLERANCE_DEG);
  });
});

describe("the crossover: the surface as drawn", () => {
  /** Sampling the section exactly as surface.ts resolves it. */
  function segmentsOf(params: SurfaceParams): number {
    return Math.min(64, Math.max(3, Math.round(params.lobes) * 4, Math.round(params.radialSegments)));
  }

  it.each(presets)("on %s, every vertex of every junction ring at the seam is inside the parent as drawn", (_name, preset) => {
    const skeleton = grown(preset, ORDERS_BELOW);
    const at = seam(skeleton);
    if ("untested" in at) throw new Error(at.untested);
    const { crossover } = at;
    const envelope = preset.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, preset.radii);
    const surface = preset.surface;
    const segments = segmentsOf(surface);
    const mesh = buildSurface(skeleton, field, envelope, surface);
    const order = ordersOf(skeleton, crossover);
    const aboveSeam = edgesAbove(skeleton, crossover, envelope);
    const flare = (y: number): number =>
      1 + (surface.flareRadius - 1) * Math.exp(-Math.max(0, y) / (surface.flareFalloff * envelope.height));

    /* Walk the mesh in the order surface.ts lays it out: one ring per
       sample, the trunk's buried base first, then two cap centres per
       run. A child run's first ring is its socket at the fork node. */
    const vertex = (index: number): THREE.Vector3 =>
      new THREE.Vector3(mesh.positions[index * 3], mesh.positions[index * 3 + 1], mesh.positions[index * 3 + 2]);
    let offset = 0;
    let junctions = 0;
    let checked = 0;
    for (const path of branchPaths(skeleton, field)) {
      const rings = path.nodes.length + (path.trunk && surface.flareDepth > 0 ? 1 : 0);
      if (!path.trunk) {
        const attach = path.nodes[0];
        const first = path.nodes[1];
        /* Junctions at the seam and beside it: a colonization tip
           forking into a twig, and twig whorls for ORDERS_BELOW under
           it, plus the last forks above along the way to the tip. */
        const atSeam = attach < crossover && first >= crossover;
        const below = attach >= crossover && order[attach] < ORDERS_BELOW;
        const above = attach < crossover && aboveSeam.has(attach);
        if (atSeam || below || above) {
          held(envelope, skeleton, field, first);
          const centre = skeleton.nodes[attach].position;
          const inscribed =
            field.radius[attach] * flare(centre.y) * (1 - surface.lobeDepth) * Math.cos(Math.PI / segments);
          const slack = centre.length() * VERTEX_SLACK;
          for (let k = 0; k < segments; k += 1) {
            const distance = vertex(offset + k).distanceTo(centre);
            expect(distance, `run at node ${attach} vertex ${k}`).toBeLessThanOrEqual(inscribed + slack);
            checked += 1;
          }
          if (atSeam) junctions += 1;
        }
      }
      offset += rings * segments + 2;
    }
    expect(offset).toBe(mesh.vertices);
    expect(junctions).toBeGreaterThan(50);
    expect(checked).toBeGreaterThan(junctions * segments);
  });
});

describe("the crossover: the error case", () => {
  it("reports a tree whose crossover is outside its grown range as untested, not continuous", () => {
    const rested = grown(TELPERION, 0);
    expect(seam(rested)).toEqual({
      untested: `crossover ${rested.nodes.length} is outside the grown range of ${rested.nodes.length} nodes: no twig orders`,
    });
    const bare: Skeleton = { nodes: rested.nodes };
    expect(seam(bare)).toEqual({
      untested: "the skeleton carries no crossover: not built by the twig pass",
    });
    // A ceiling that stops the pass before its first twig is the same case.
    const capped = growSkeleton({
      ...TELPERION.skeleton,
      twigs: { ...TELPERION.skeleton.twigs, levels: 3 },
      growth: { ...TELPERION.skeleton.growth, maxNodes: rested.nodes.length },
    });
    expect("untested" in seam(capped)).toBe(true);
    expect("crossover" in seam(grown(TELPERION, 1))).toBe(true);
  });
});
