import { describe, expect, it } from "vitest";
import { DEFAULT_ELEMENT } from "../canopy/element";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import { solveRadii } from "../radius";
import { growSkeleton, type SkeletonParams } from "./grow";
import { MAX_TWIG_LEVELS } from "./twigs";
import {
  branchLength, childRadius, DEFAULT_BRANCH_LAW, DEFAULT_TWIG_ANATOMY,
  generationsUntilTwig,
} from "./law";

const bare = (tree: SkeletonParams): SkeletonParams => ({
  ...tree, twigs: { ...tree.twigs, levels: 0 },
});
const median = (values: number[]): number => {
  const sorted = values.slice().sort((a, b) => a - b);
  const middle = Math.floor(sorted.length / 2);
  return sorted.length % 2 ? sorted[middle] : (sorted[middle - 1] + sorted[middle]) / 2;
};

/* An explicit tip-only topology for the proof, before any shedding or
   collision rejection: three internodes along the leader, two laterals
   (at one and two thirds), and one terminal twig continuing the leader.
   Both laterals take the full length ratio, so omitting positional taper
   overestimates them. A twig adds one fixed internode. Colonization-limb
   laterals are task 4's separate budget. */
function estimateNodes(radius: number): number {
  if (radius <= DEFAULT_TWIG_ANATOMY.diameter / 2) return 1;
  return 4 + 2 * estimateNodes(childRadius(
    radius, DEFAULT_BRANCH_LAW.lengthRatio, DEFAULT_BRANCH_LAW.ratioPower,
  ));
}

describe("the branch law", () => {
  it("anchors elastic similarity in metres and keeps the twig independent of tree size", () => {
    expect(branchLength(7.4)).toBeCloseTo(148, 10);
    expect(branchLength(0.395)).toBeCloseTo(20.9816014211964, 10);
    expect(branchLength(8) / branchLength(1)).toBeCloseTo(4, 12);
    expect(DEFAULT_TWIG_ANATOMY).toEqual({ diameter: 0.005, internodeLength: 0.02, stationsPerInternode: 1 });
    expect(DEFAULT_ELEMENT.length / DEFAULT_TWIG_ANATOMY.diameter).toBe(24);
  });

  it("uses the child's length ratio without dividing the parent's area among siblings", () => {
    expect(childRadius(2, 0.4, 1.3)).toBeCloseTo(2 * 0.4 ** 1.3, 12);
    expect(childRadius(2, 1, 1.3)).toBe(2);
  });

  it("counts lateral reductions, including the exact twig threshold and a reported cap", () => {
    const twig = DEFAULT_TWIG_ANATOMY.diameter / 2;
    expect(generationsUntilTwig(twig)).toEqual({ generations: 0, capped: false });
    expect(generationsUntilTwig(twig / 2)).toEqual({ generations: 0, capped: false });
    expect(generationsUntilTwig(twig * 2, { lengthRatio: 0.5, ratioPower: 1 }))
      .toEqual({ generations: 1, capped: false });
    expect(generationsUntilTwig(twig * 2 ** MAX_TWIG_LEVELS, { lengthRatio: 0.5, ratioPower: 1 }))
      .toEqual({ generations: MAX_TWIG_LEVELS, capped: false });
    expect(generationsUntilTwig(1, { lengthRatio: 1 }))
      .toEqual({ generations: MAX_TWIG_LEVELS, capped: true });
    expect(generationsUntilTwig(1, { ratioPower: 0 }))
      .toEqual({ generations: MAX_TWIG_LEVELS, capped: true });
  });

  it("holds non-finite inputs and pins finite departures to the documented rails", () => {
    expect(branchLength(NaN)).toBe(0);
    expect(branchLength(-1)).toBe(0);
    expect(branchLength(1, { lengthScale: Infinity })).toBe(branchLength(1));
    expect(branchLength(1, { lengthScale: -1 })).toBe(1e-6);
    expect(childRadius(Infinity, 0.4, 1.3)).toBe(0);
    expect(childRadius(1, NaN, NaN)).toBe(childRadius(1, 0.4, 1.3));
    expect(childRadius(1, -1, 100)).toBeCloseTo(0.05 ** 8, 15);
    expect(childRadius(1, 2, -1)).toBe(1);
    expect(generationsUntilTwig(1, { twigDiameter: NaN })).toEqual(generationsUntilTwig(1));
    expect(generationsUntilTwig(1, { twigDiameter: -1 })).toEqual(generationsUntilTwig(1, { twigDiameter: 1e-6 }));
  });

  it.each([TELPERION, LAURELIN])("measures every zero-order $name handoff before the pass changes", (preset) => {
    const skeleton = growSkeleton(bare(preset.skeleton));
    const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);
    const children = new Uint32Array(skeleton.nodes.length);
    for (let i = 1; i < skeleton.nodes.length; i += 1) children[skeleton.nodes[i].parent] += 1;
    const handoffs = [];
    for (let i = 1; i < skeleton.nodes.length; i += 1) {
      if (children[i] !== 0) continue;
      const radius = field.radius[i];
      const depth = generationsUntilTwig(radius);
      expect(depth.capped).toBe(false);
      expect(depth.generations).toBeLessThanOrEqual(MAX_TWIG_LEVELS);
      handoffs.push({ node: i, radius, diameter: 2 * radius,
        length: branchLength(radius), ...depth, nodes: estimateNodes(radius) });
    }
    expect(handoffs.length).toBeGreaterThan(0);
    const total = skeleton.nodes.length + handoffs.reduce((sum, tip) => sum + tip.nodes, 0);
    expect(total).toBeLessThan(250_000);
    const length = median(handoffs.map((tip) => tip.length));
    if (preset === TELPERION) {
      expect(length).toBeGreaterThan(15);
      expect(length).toBeLessThan(30);
    }
    process.stdout.write(JSON.stringify({ preset: preset.id, colonizationNodes: skeleton.nodes.length,
      tips: handoffs.length, medianDiameter: median(handoffs.map((tip) => tip.diameter)),
      medianLength: length, minLength: Math.min(...handoffs.map((tip) => tip.length)),
      maxLength: Math.max(...handoffs.map((tip) => tip.length)),
      minGenerations: Math.min(...handoffs.map((tip) => tip.generations)),
      maxGenerations: Math.max(...handoffs.map((tip) => tip.generations)), estimatedNodes: total,
      handoffs,
    }) + "\n");
  });
});
