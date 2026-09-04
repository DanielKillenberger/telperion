import { readFileSync } from "node:fs";
import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ELEMENT } from "../canopy/element";
import { DEFAULT_ENVELOPE } from "../envelope";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import { solveRadii } from "../radius";
import { createGrowthBias, DEFAULT_BIAS, NO_BIAS } from "../torsion";
import type { GrowthConfig, Skeleton } from "./colonize";
import { growReport, growSkeleton, resolveGrowth, type SkeletonParams } from "./grow";
import {
  branchTwigs,
  DEFAULT_TWIGS,
  MAX_TWIG_CHILDREN,
  MAX_TWIG_LEVELS,
  resolveTwigs,
  type TwigParams,
} from "./twigs";

/* The second pass, held to what the first pass hands it: one skeleton,
   the same invariant, the same field and the same turn limit, and a
   tree that is the tree it was until someone asks for orders. */

const DEG = 180 / Math.PI;

function signature(skeleton: Skeleton): string {
  return JSON.stringify(
    skeleton.nodes.map((node) => [
      node.position.x,
      node.position.y,
      node.position.z,
      node.parent,
    ]),
  );
}

/** How many children each node has. */
function childCounts(skeleton: Skeleton): Int32Array {
  const counts = new Int32Array(skeleton.nodes.length);
  for (const node of skeleton.nodes) if (node.parent >= 0) counts[node.parent] += 1;
  return counts;
}

/** Tip indices: every node past the root that nothing grew from. */
function tips(skeleton: Skeleton): number[] {
  const counts = childCounts(skeleton);
  const found: number[] = [];
  for (let i = 1; i < skeleton.nodes.length; i += 1) if (counts[i] === 0) found.push(i);
  return found;
}

/** The unit direction of the step that made node `index`. */
function arrival(skeleton: Skeleton, index: number): THREE.Vector3 {
  const node = skeleton.nodes[index];
  return node.position.clone().sub(skeleton.nodes[node.parent].position).normalize();
}

/** Mean unit step over nodes `from` (inclusive) to `to` (exclusive). */
function meanStep(skeleton: Skeleton, from: number, to: number): THREE.Vector3 {
  const mean = new THREE.Vector3();
  for (let i = Math.max(1, from); i < to; i += 1) mean.add(arrival(skeleton, i));
  return mean.divideScalar(Math.max(1, to - from));
}

/** Orders below the base: 0 for a base node, parent's plus one after. */
function orders(skeleton: Skeleton, baseCount: number): Int32Array {
  const order = new Int32Array(skeleton.nodes.length);
  for (let i = baseCount; i < skeleton.nodes.length; i += 1) {
    order[i] = order[skeleton.nodes[i].parent] + 1;
  }
  return order;
}

/** A tree with `twigs` on top of `tree`, with the node ceiling lifted
 *  so the orders asked for are measured rather than truncated. */
function grown(tree: SkeletonParams, twigs: Partial<TwigParams>): Skeleton {
  return growSkeleton({
    ...tree,
    twigs: { ...tree.twigs, ...twigs },
    growth: { ...tree.growth, maxNodes: 4_000_000 },
  });
}

const params: SkeletonParams = { seed: 1, envelope: DEFAULT_ENVELOPE, attractors: 900 };

describe("branchTwigs", () => {
  it("continues from every tip colonization left, into one skeleton", () => {
    /* R1. The base comes back first and untouched - the same node
       objects in the same order - and every node after it hangs off a
       base tip. One order of a leader and a lateral is at most two
       children per tip, and at least the leader on every tip whose
       continuation stays above the bare-trunk line. */
    const base = growSkeleton({ ...params, bias: NO_BIAS });
    const config = resolveGrowth({ ...params, bias: NO_BIAS });
    const twigged = branchTwigs(base, config, resolveTwigs({ levels: 1 }));

    expect(twigged.nodes.slice(0, base.nodes.length)).toEqual(base.nodes);
    const baseTips = tips(base);
    const appended = twigged.nodes.length - base.nodes.length;
    expect(appended).toBeGreaterThan(baseTips.length);
    expect(appended).toBeLessThanOrEqual(baseTips.length * 2);
    const isTip = new Set(baseTips);
    for (let i = base.nodes.length; i < twigged.nodes.length; i += 1) {
      expect(isTip.has(twigged.nodes[i].parent)).toBe(true);
    }
    const counts = childCounts(twigged);
    const bare = baseTips.filter((tip) => counts[tip] === 0);
    expect(bare.length).toBeLessThan(baseTips.length * 0.05);
  });

  it.each([
    ["Telperion", TELPERION.skeleton],
    ["Laurelin", LAURELIN.skeleton],
  ] as const)("keeps parent before child over the whole of %s, six orders down", (_name, tree) => {
    /* The invariant radius.ts, paths.ts and surface.ts all do a single
       forward pass on. Asserted over every node, not only the appended
       ones, so a pass that reordered the base would fail here too. */
    const skeleton = grown(tree, { levels: 6 });
    expect(skeleton.nodes[0].parent).toBe(-1);
    for (let i = 1; i < skeleton.nodes.length; i += 1) {
      const parent = skeleton.nodes[i].parent;
      expect(parent).toBeGreaterThanOrEqual(0);
      expect(parent).toBeLessThan(i);
    }
    expect(skeleton.nodes.length).toBeGreaterThan(growSkeleton(tree).nodes.length * 4);
  });

  it("inherits the terminal tangent through limitTurn, and never reseeds it", () => {
    /* At the handoff and at every order below it, a twig step turns no
       further from the step before it than the run's own limit - the
       same limit every colonization step is held to, on the tree with
       the stiffest one. And with no field and no lateral, the leader
       out of a tip is that tip's own tangent to the bit: nothing is
       re-chosen at the seam. */
    const tree = TELPERION.skeleton;
    const limit = tree.growth.maxTurnPerStep;
    const skeleton = grown(tree, { levels: 4 });
    const baseCount = growSkeleton(tree).nodes.length;
    const order = orders(skeleton, baseCount);
    let handoffs = 0;
    for (let i = baseCount; i < skeleton.nodes.length; i += 1) {
      const parent = skeleton.nodes[i].parent;
      const turn = Math.acos(
        Math.min(1, arrival(skeleton, i).dot(arrival(skeleton, parent))),
      ) * DEG;
      expect(turn).toBeLessThanOrEqual(limit + 1e-6);
      if (order[i] === 1) handoffs += 1;
    }
    expect(handoffs).toBeGreaterThan(100);

    const base = growSkeleton({ ...tree, bias: NO_BIAS });
    const config = { ...resolveGrowth({ ...tree, bias: NO_BIAS }), bias: undefined };
    const leaders = branchTwigs(base, config, resolveTwigs({ levels: 1, children: 1 }));
    for (let i = base.nodes.length; i < leaders.nodes.length; i += 1) {
      const parent = leaders.nodes[i].parent;
      expect(arrival(leaders, i).distanceTo(arrival(leaders, parent))).toBeLessThan(1e-12);
    }
  });

  it.each([
    ["at rest", {}, 1, 0.6],
    ["with no taper", { taper: 1 }, 1, 1],
    ["at half an internode", { internode: 0.5 }, 0.5, 0.6],
  ] as const)("shrinks each order by the taper law, %s", (_name, twigs, internode, taper) => {
    /* The fine orders' own law: order k is `internode` steps long at
       the handoff and `taper` of the order above after that, whatever
       the field does to its direction. Thickness is the radius solve's
       and runs after this pass; length and fork count are what this
       pass can author, and below the crossover the solve applies its
       own steeper law to what it is handed. */
    const tree = params;
    const step = resolveGrowth(tree).stepDistance;
    const skeleton = grown(tree, { levels: 5, ...twigs });
    const baseCount = growSkeleton(tree).nodes.length;
    const order = orders(skeleton, baseCount);
    const seen = new Int32Array(6);
    for (let i = baseCount; i < skeleton.nodes.length; i += 1) {
      const node = skeleton.nodes[i];
      const length = node.position.distanceTo(skeleton.nodes[node.parent].position);
      const expected = step * internode * taper ** (order[i] - 1);
      expect(Math.abs(length - expected)).toBeLessThan(1e-9);
      seen[order[i]] += 1;
    }
    for (let k = 1; k <= 5; k += 1) expect(seen[k]).toBeGreaterThan(0);
  });

  it("stops on the level cap, a named parameter, and on the node ceiling", () => {
    /* Never on a shrinking search radius: the deepest order below the
       base is exactly the cap, the cap's rail is `MAX_TWIG_LEVELS`,
       and the run's node ceiling is the stop it was for colonization. */
    const baseCount = growSkeleton(params).nodes.length;
    for (const levels of [1, 3, 7]) {
      const order = orders(grown(params, { levels }), baseCount);
      expect(Math.max(...Array.from(order))).toBe(levels);
    }
    expect(resolveTwigs({ levels: 99 }).levels).toBe(MAX_TWIG_LEVELS);
    expect(resolveTwigs({ levels: -3 }).levels).toBe(0);

    const capped = growSkeleton({
      ...params,
      twigs: { levels: 6 },
      growth: { maxNodes: baseCount + 100 },
    });
    /* Shedding runs inside growSkeleton after the twig pass, so the
       finished tree is smaller than the ceiling it hit; the ceiling is
       asserted where the count is whole, on the report. */
    expect(capped.nodes.length).toBeLessThanOrEqual(baseCount + 100);
    const report = growReport({
      ...params,
      twigs: { ...params.twigs, levels: 4 },
      growth: { maxNodes: baseCount + 100 },
    });
    expect(report.skeleton.nodes.length + report.shed).toBe(baseCount + 100);
    expect(report.capped).toBe(true);
  });

  it.each([
    ["Telperion", TELPERION],
    ["Laurelin", LAURELIN],
  ] as const)("reaches leaf scale on %s at eight orders, from today's 0.15", (_name, preset) => {
    /* R3, measured rather than rounded: a leaf's length as a multiple
       of the diameter of the wood it attaches to, the median terminal
       diameter from the radius solve the tree actually runs, against
       the leaf the preset actually places. Today the leaf is 0.15 of
       the wood on Telperion and 0.16 on Laurelin; at eight orders of
       the resting twig it is 1.6 and 1.2 - past one on both, which is
       the botanical relationship rather than a round number. Six
       orders is 0.86 and 0.70, so eight is where both cross. */
    const tree = preset.skeleton;
    const leaf = DEFAULT_ELEMENT.length * preset.canopy.size;
    const ratio = (levels: number): number => {
      const skeleton = grown(tree, { levels });
      const field = solveRadii(skeleton, tree.envelope, preset.radii);
      const radii = tips(skeleton).map((tip) => field.radius[tip]).sort((a, b) => a - b);
      return leaf / (2 * radii[radii.length >> 1]);
    };
    const today = ratio(0);
    expect(today).toBeGreaterThan(0.1);
    expect(today).toBeLessThan(0.2);
    expect(ratio(8)).toBeGreaterThan(1);
  });

  it("checks each whorl for sibling collisions", () => {
    /* Two children within half the branching angle the tree can make
       are one twig. At a zero angle every lateral coincides with the
       leader and is dropped, so a leader-and-lateral tree is the
       leader-only tree byte for byte; and at rest no node carries two
       twigs closer than the separation. */
    expect(signature(grown(params, { levels: 4, angle: 0 }))).toBe(
      signature(grown(params, { levels: 4, children: 1 })),
    );

    const tree = TELPERION.skeleton;
    const separation = Math.min(tree.twigs.angle, tree.growth.maxTurnPerStep) / 2;
    const skeleton = grown(tree, { levels: 4, children: 4 });
    const baseCount = growSkeleton(tree).nodes.length;
    const siblings: number[][] = Array.from({ length: skeleton.nodes.length }, () => []);
    for (let i = baseCount; i < skeleton.nodes.length; i += 1) {
      siblings[skeleton.nodes[i].parent].push(i);
    }
    let whorls = 0;
    for (const whorl of siblings) {
      if (whorl.length < 2) continue;
      whorls += 1;
      for (let a = 0; a < whorl.length; a += 1) {
        for (let b = a + 1; b < whorl.length; b += 1) {
          const apart = Math.acos(
            Math.min(1, arrival(skeleton, whorl[a]).dot(arrival(skeleton, whorl[b]))),
          ) * DEG;
          expect(apart).toBeGreaterThanOrEqual(separation - 1e-6);
        }
      }
    }
    expect(whorls).toBeGreaterThan(100);
  });

  it("builds the same twigs twice from one seed, and iterates arrays only", () => {
    const twigs = { levels: 5 };
    const bias = { ...DEFAULT_BIAS, writheAmplitude: 0.18, spiralRate: 3 };
    expect(signature(growSkeleton({ ...params, bias, twigs }))).toBe(
      signature(growSkeleton({ ...params, bias, twigs })),
    );
    expect(signature(growSkeleton({ ...params, bias, twigs }))).not.toBe(
      signature(growSkeleton({ ...params, bias, twigs, seed: 2 })),
    );
    /* Not a style rule: hash order is stable within a run and this
       would pass anyway. It is the dependency that surfaces once in
       twenty builds on somebody else's machine, and the only cheap
       guard is that the construct is not there. */
    const source = readFileSync(new URL("twigs.ts", import.meta.url), "utf8");
    expect(source).not.toMatch(/new (Map|Set|WeakMap|WeakSet)\b/);
  });

  it.each([
    ["Telperion", TELPERION.skeleton],
    ["Laurelin", LAURELIN.skeleton],
    ["the default envelope", params],
  ] as const)("at zero orders, %s is the tree it was before the pass existed", (_name, tree) => {
    /* R7, and R1's error case with it. Twigs left out, twigs stated at
       rest, and a level count that is not a number are one tree byte
       for byte - the last through the same `held` rail every stage
       uses - so any tree that differs from today's does so because
       someone asked for orders. The base's own nodes are returned as
       they are, not copied. */
    const { twigs: _stated, ...unstated } = tree;
    const today = signature(growSkeleton(unstated));
    expect(signature(growSkeleton({ ...unstated, twigs: DEFAULT_TWIGS }))).toBe(today);
    expect(signature(growSkeleton({ ...unstated, twigs: { levels: Number.NaN } }))).toBe(today);
    expect(signature(growSkeleton({ ...unstated, twigs: { levels: 0, children: Number.POSITIVE_INFINITY } }))).toBe(today);

    const base = growSkeleton(unstated);
    const same = branchTwigs(base, resolveGrowth(unstated), resolveTwigs());
    expect(same.nodes).toEqual(base.nodes);
    expect(same.nodes[1]).toBe(base.nodes[1]);
  });

  it("bends every twig through the field that bends the limbs: lean, driven hard", () => {
    /* R9. One term at an extreme - lean at 0.5, the rest off - and the
       twigs move with the limbs. The limb with nothing else pulling on
       it is the trunk, so the trunk's mean step says which way the
       tree leans; the field's own effect on the twigs is isolated by
       running the pass twice on one base, under the leaning field and
       under the zero field, and taking the difference of the mean twig
       steps. That difference points the way the trunk leans, and it is
       most of a step's worth - measured, 0.65 on both trees. A straight
       twig on a leaning limb scores zero here. (The limbs' own mean
       step is not the reference because colonization spends more steps
       fighting the lean to reach the attractors upwind of it, so the
       crown's mean step points against the lean; the trunk does not.) */
    for (const tree of [TELPERION.skeleton, params]) {
      const lean = { ...NO_BIAS, lean: 0.5 };
      const base = growSkeleton({ ...tree, bias: lean });
      const crownBase = tree.envelope.height * tree.envelope.crownBase;
      const trunkTop = base.nodes.findIndex((node) => node.position.y > crownBase);
      const trunk = meanStep(base, 1, trunkTop);
      const leans = new THREE.Vector3(trunk.x, 0, trunk.z);
      expect(leans.length()).toBeGreaterThan(0.3);
      leans.normalize();

      const twigs = resolveTwigs({ levels: 4 });
      const leaning = { ...resolveGrowth({ ...tree, bias: lean }), maxNodes: 1e6 };
      const still = { ...leaning, bias: createGrowthBias(tree.envelope, tree.seed, NO_BIAS) };
      const under = (config: GrowthConfig): THREE.Vector3 => {
        const grownTree = branchTwigs(base, config, twigs);
        return meanStep(grownTree, base.nodes.length, grownTree.nodes.length);
      };
      const shift = under(leaning).sub(under(still));
      const sideways = new THREE.Vector3(shift.x, 0, shift.z);
      expect(sideways.dot(leans)).toBeGreaterThan(0.4);
      expect(sideways.dot(leans) / sideways.length()).toBeGreaterThan(0.95);
    }
  });

  it("makes every term at zero the unbiased recursion, byte for byte", () => {
    /* R9's error case, so the field's effect below the crossover is
       attributable: the field with every term off is `direction` made
       unit, which is exactly what the unbiased pass does to its own
       wanted direction. One base, three fields. Colonization does not
       hold this - it hands the field a direction it never re-unitises
       itself - which is why it is the twig pass, not growSkeleton, that
       is asserted. */
    const base = growSkeleton({ ...params, growth: { bias: undefined } });
    const config = { ...resolveGrowth(params), maxNodes: 1e6 };
    const twigs = resolveTwigs({ levels: 5 });
    const zero = branchTwigs(
      base,
      { ...config, bias: createGrowthBias(params.envelope, params.seed, NO_BIAS) },
      twigs,
    );
    const none = branchTwigs(base, { ...config, bias: undefined }, twigs);
    expect(zero.nodes.length).toBeGreaterThan(base.nodes.length * 4);
    expect(signature(zero)).toBe(signature(none));
    expect(signature(branchTwigs(base, config, twigs))).not.toBe(signature(none));
  });

  it("puts no twig below the bare-trunk line", () => {
    /* Zero width is not no constraint: the envelope has no width below
       the crown base, and a twig from a tip near it, heading down, is
       outside the authored silhouette however plausible on its own. */
    for (const tree of [TELPERION.skeleton, LAURELIN.skeleton, params]) {
      const trunkHeight = tree.envelope.height * tree.envelope.crownBase;
      const skeleton = grown({ ...tree, bias: { ...DEFAULT_BIAS, gravitropism: 0 } }, { levels: 6 });
      const baseCount = growSkeleton({ ...tree, bias: { ...DEFAULT_BIAS, gravitropism: 0 } }).nodes.length;
      for (let i = baseCount; i < skeleton.nodes.length; i += 1) {
        expect(skeleton.nodes[i].position.y).toBeGreaterThanOrEqual(trunkHeight);
      }
    }
  });

  it("yields no tips from a skeleton with fewer than two nodes", () => {
    // R1's other error case: returned as it came, never thrown.
    const config = resolveGrowth(params);
    const twigs = resolveTwigs({ levels: 4 });
    expect(branchTwigs({ nodes: [] }, config, twigs).nodes).toHaveLength(0);
    const root = { nodes: [{ position: new THREE.Vector3(), parent: -1 }] };
    expect(branchTwigs(root, config, twigs).nodes).toEqual(root.nodes);
    expect(growSkeleton({ ...params, envelope: { ...DEFAULT_ENVELOPE, spread: 0 }, twigs })
      .nodes).toHaveLength(1);
  });

  it("rests on botanical defaults, and holds every dial to its rail", () => {
    /* The resting values, pinned so a change to one is a change someone
       made; the sources are beside them in twigs.ts. And the rails:
       non-finite is the default, out of range is the nearer end. */
    expect(DEFAULT_TWIGS).toEqual({
      levels: 0,
      children: 2,
      angle: 45,
      divergence: 137.508,
      internode: 1,
      taper: 0.6,
    });
    expect(
      resolveTwigs({
        levels: Number.NaN,
        children: Number.POSITIVE_INFINITY,
        angle: Number.NaN,
        divergence: Number.NEGATIVE_INFINITY,
        internode: Number.NaN,
        taper: Number.NaN,
      }),
    ).toEqual(DEFAULT_TWIGS);
    expect(resolveTwigs()).toEqual(DEFAULT_TWIGS);
    const railed = resolveTwigs({
      levels: 2.6,
      children: 0,
      angle: 120,
      divergence: -99.5,
      internode: 0,
      taper: 2,
    });
    expect(railed).toEqual({
      levels: 3,
      children: 1,
      angle: 90,
      divergence: -99.5,
      internode: 1e-3,
      taper: 1,
    });
    expect(resolveTwigs({ children: 50 }).children).toBe(MAX_TWIG_CHILDREN);
    expect(resolveTwigs({ taper: 0 }).taper).toBe(0.05);
    expect(resolveTwigs({ internode: 100 }).internode).toBe(8);
  });
});
