import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE, type Envelope } from "./envelope";
import {
  DEFAULT_RADII,
  solveRadii,
  type RadiusParams,
} from "./radius";
import { LAURELIN, TELPERION } from "./presets/two-trees";
import { colonize, type Skeleton } from "./skeleton/colonize";
import { sampleEnvelope } from "./envelope";
import { createRng } from "./rng";
import { resolveGrowth } from "./skeleton/grow";
import { growSkeleton } from "./skeleton/grow";
import type { TwiggedSkeleton } from "./skeleton/twigs";

/* The radius solve makes three claims that a box with no GPU can hold
   to account, and they are the whole of the task's acceptance: the
   fork rule holds at every junction, thickness never grows toward the
   tips, and no radius is ever zero or negative whatever it is handed.
   The fourth - that the exponent visibly changes the taper - is the
   owner's to judge on screen, but "visibly" has a measurable floor and
   that floor is tested here. */

/** A real tree, small enough that the whole file runs in a blink. */
function grownSkeleton(seed = 7, attractors = 400): Skeleton {
  const tree = { seed, envelope: DEFAULT_ENVELOPE, attractors };
  return colonize(sampleEnvelope(tree.envelope, attractors, createRng(seed)), new THREE.Vector3(), resolveGrowth(tree));
}

/** A straight unbranched run of `steps` edges over `length` metres.
 *  Nothing forks, so it isolates the length taper from the fork rule. */
function chain(length: number, steps: number): Skeleton {
  const nodes = [];
  for (let i = 0; i <= steps; i += 1) {
    nodes.push({
      position: new THREE.Vector3(0, (length * i) / steps, 0),
      parent: i - 1,
    });
  }
  return { nodes };
}

/** A trunk that forks into two limbs, each of which runs on. Every
 *  position is a multiple of `scale`, so the same shape can be asked
 *  for at two sizes. */
function forked(scale: number): Skeleton {
  const at = (x: number, y: number, z: number, parent: number) => ({
    position: new THREE.Vector3(x * scale, y * scale, z * scale),
    parent,
  });
  return {
    nodes: [
      at(0, 0, 0, -1),
      at(0, 0.5, 0, 0),
      at(0.2, 0.8, 0, 1),
      at(-0.3, 0.75, 0.1, 1),
      at(0.4, 1, 0, 2),
      at(-0.5, 0.95, 0.2, 3),
    ],
  };
}

function childrenOf(skeleton: Skeleton): number[][] {
  const children: number[][] = skeleton.nodes.map(() => []);
  skeleton.nodes.forEach((node, index) => {
    if (node.parent >= 0) children[node.parent].push(index);
  });
  return children;
}

function params(overrides: Partial<RadiusParams> = {}): RadiusParams {
  return { ...DEFAULT_RADII, ...overrides };
}

describe("solveRadii - the fork rule", () => {
  it("conserves cross-sectional area through every fork", () => {
    // The acceptance criterion in its own terms: at the default
    // exponent of 2 the rule is the area statement itself.
    const skeleton = grownSkeleton();
    const field = solveRadii(skeleton, DEFAULT_ENVELOPE, params());
    const children = childrenOf(skeleton);

    let forks = 0;
    children.forEach((kids, index) => {
      if (kids.length < 2) return;
      forks += 1;
      const parentArea = field.radius[index] ** 2;
      const childArea = kids.reduce(
        (sum, kid) => sum + field.startRadius[kid] ** 2,
        0,
      );
      // Tolerance is floating-point summation and nothing else: the
      // solve is exact by construction, so a loose tolerance here
      // would only be hiding a wrong one.
      expect(Math.abs(parentArea - childArea) / parentArea).toBeLessThan(1e-12);
    });
    expect(forks).toBeGreaterThan(20);
  });

  it.each([1, 1.6, 2, 2.5, 3.4])(
    "holds r_parent^n = sum r_child^n at exponent %s",
    (forkExponent) => {
      const skeleton = grownSkeleton();
      const field = solveRadii(
        skeleton,
        DEFAULT_ENVELOPE,
        params({ forkExponent }),
      );
      const children = childrenOf(skeleton);

      children.forEach((kids, index) => {
        if (kids.length === 0) return;
        const parent = field.radius[index] ** forkExponent;
        const sum = kids.reduce(
          (total, kid) => total + field.startRadius[kid] ** forkExponent,
          0,
        );
        expect(Math.abs(parent - sum) / parent).toBeLessThan(1e-12);
      });
    },
  );

  it("clamps an exponent below 1, where a parent would outweigh its children", () => {
    const skeleton = grownSkeleton();
    const clamped = solveRadii(skeleton, DEFAULT_ENVELOPE, params({ forkExponent: 0.2 }));
    const rail = solveRadii(skeleton, DEFAULT_ENVELOPE, params({ forkExponent: 1 }));
    expect([...clamped.radius]).toEqual([...rail.radius]);
  });
});

describe("solveRadii - monotonicity", () => {
  it("never thickens from a parent toward its child", () => {
    const skeleton = grownSkeleton();
    const field = solveRadii(skeleton, DEFAULT_ENVELOPE, params());

    const children = childrenOf(skeleton);
    skeleton.nodes.forEach((node, index) => {
      if (node.parent < 0) return;
      /* Both halves of the edge. The fork sheds thickness down to
         startRadius - by nothing at all where a node has one child,
         which is the rule stating that an unforked branch carries the
         same plumbing - and the length taper sheds from there to the
         node itself, which is what makes the whole run strictly
         thinner toward the tip. */
      expect(field.radius[node.parent]).toBeGreaterThanOrEqual(
        field.startRadius[index],
      );
      expect(field.startRadius[index]).toBeGreaterThan(field.radius[index]);
    });

    // And at a real fork the parent is strictly thicker than either
    // limb leaving it: that is the visible hierarchy.
    children.forEach((kids, index) => {
      if (kids.length < 2) return;
      for (const kid of kids) {
        expect(field.radius[index]).toBeGreaterThan(field.startRadius[kid]);
      }
    });
  });

  it("is a cylinder along an unbranched run when the length taper is off", () => {
    const field = solveRadii(chain(12, 24), DEFAULT_ENVELOPE, params({ lengthTaper: 0 }));
    const trunk = DEFAULT_RADII.trunkRadius * DEFAULT_ENVELOPE.height;
    for (const value of field.radius) expect(value).toBeCloseTo(trunk, 12);
  });

  it("clamps a negative length taper rather than thickening toward the tip", () => {
    const field = solveRadii(chain(12, 24), DEFAULT_ENVELOPE, params({ lengthTaper: -2 }));
    const trunk = DEFAULT_RADII.trunkRadius * DEFAULT_ENVELOPE.height;
    for (const value of field.radius) expect(value).toBeCloseTo(trunk, 12);
  });
});

describe("solveRadii - the parameters are the whole of the look", () => {
  it("puts the root at exactly the trunk radius it was asked for", () => {
    const envelope: Envelope = { ...DEFAULT_ENVELOPE, height: 41 };
    const field = solveRadii(
      grownSkeleton(),
      envelope,
      params({ trunkRadius: 0.035 }),
    );
    expect(field.radius[0]).toBeCloseTo(0.035 * 41, 12);
    // The root has no branch behind it, so both ends of it are the same.
    expect(field.startRadius[0]).toBe(field.radius[0]);
  });

  it("keeps the same proportions at every height on the dial", () => {
    /* The same tree at five sizes: positions scaled with the envelope
       each time. Everything the solve does is a fraction of height, so
       every radius has to come back scaled by exactly the same factor
       - a 4 m tree is not a 400 m tree's twig. The heights are the
       height dial's own range end to end, because that is the range
       the claim is made over and 60 m stopped being the top of it. */
    const at = (height: number) =>
      solveRadii(forked(height), { ...DEFAULT_ENVELOPE, height }, params());
    const sapling = at(4);
    for (const height of [24, 60, 150, 400]) {
      const grown = at(height);
      sapling.radius.forEach((value, index) => {
        expect(
          grown.radius[index] / value,
          `node ${index} at ${height} m`,
        ).toBeCloseTo(height / 4, 9);
      });
    }
  });

  it("changes the taper visibly when the exponent moves, and only the taper", () => {
    const skeleton = grownSkeleton();
    const children = childrenOf(skeleton);
    const tips = children
      .map((kids, index) => (kids.length === 0 ? index : -1))
      .filter((index) => index >= 0);
    const meanTip = (radii: Float64Array): number =>
      tips.reduce((sum, index) => sum + radii[index], 0) / tips.length;

    const sharp = solveRadii(skeleton, DEFAULT_ENVELOPE, params({ forkExponent: 2 }));
    const soft = solveRadii(skeleton, DEFAULT_ENVELOPE, params({ forkExponent: 3 }));

    // A higher exponent sheds less at each fork, so the twigs come out
    // stouter under a trunk that has not moved: the dial is taper and
    // nothing else.
    expect(meanTip(soft.radius) / meanTip(sharp.radius)).toBeGreaterThan(1.5);
    expect(soft.radius[0]).toBeCloseTo(sharp.radius[0], 12);
  });

  it("states the length taper as a rate over distance, not per node", () => {
    // The same run at two resolutions. If this were per-step, a
    // skeleton grown at a finer step would come out systematically
    // thinner and the dial would mean something different on every
    // tree.
    const length = DEFAULT_ENVELOPE.height * 0.5;
    const coarse = solveRadii(chain(length, 2), DEFAULT_ENVELOPE, params());
    const fine = solveRadii(chain(length, 40), DEFAULT_ENVELOPE, params());

    const expected =
      DEFAULT_RADII.trunkRadius *
      DEFAULT_ENVELOPE.height *
      Math.exp(-DEFAULT_RADII.lengthTaper * 0.5);
    expect(coarse.radius[2]).toBeCloseTo(expected, 12);
    expect(fine.radius[40]).toBeCloseTo(expected, 12);
  });
});

describe("solveRadii - no zero or negative radii, whatever it is handed", () => {
  const cases: Array<[string, Skeleton, Envelope, RadiusParams]> = [
    ["a grown tree at defaults", grownSkeleton(), DEFAULT_ENVELOPE, params()],
    ["a lone root", { nodes: [{ position: new THREE.Vector3(), parent: -1 }] }, DEFAULT_ENVELOPE, params()],
    ["a zero-height envelope", grownSkeleton(), { ...DEFAULT_ENVELOPE, height: 0 }, params()],
    ["a zero trunk radius", grownSkeleton(), DEFAULT_ENVELOPE, params({ trunkRadius: 0 })],
    ["a negative trunk radius", grownSkeleton(), DEFAULT_ENVELOPE, params({ trunkRadius: -3 })],
    ["a zero exponent", grownSkeleton(), DEFAULT_ENVELOPE, params({ forkExponent: 0 })],
    ["a negative exponent", grownSkeleton(), DEFAULT_ENVELOPE, params({ forkExponent: -2 })],
    ["a huge length taper", grownSkeleton(), DEFAULT_ENVELOPE, params({ lengthTaper: 50 })],
  ];

  it.each(cases)("stays finite and positive with %s", (_label, skeleton, envelope, radii) => {
    const field = solveRadii(skeleton, envelope, radii);
    expect(field.radius.length).toBe(skeleton.nodes.length);
    for (const value of field.radius) {
      expect(Number.isFinite(value)).toBe(true);
      expect(value).toBeGreaterThan(0);
    }
    for (const value of field.startRadius) {
      expect(Number.isFinite(value)).toBe(true);
      expect(value).toBeGreaterThan(0);
    }
  });

  it("returns empty fields for an empty skeleton rather than throwing", () => {
    const field = solveRadii({ nodes: [] }, DEFAULT_ENVELOPE, params());
    expect(field.radius.length).toBe(0);
    expect(field.startRadius.length).toBe(0);
  });
});

describe("solveRadii - the branch generations below the crossover", () => {
  /** Full generations, or a prefix-only fixture for the colonization solve. */
  const twigged = (preset: typeof TELPERION, withBranches: boolean, lengthRatio = preset.skeleton.twigs.lengthRatio): TwiggedSkeleton => {
    const tree = growSkeleton({ ...preset.skeleton,
      twigs: { ...preset.skeleton.twigs, lengthRatio },
      growth: { ...preset.skeleton.growth, maxNodes: 4_000_000 },
    }, preset.radii);
    return withBranches ? tree : { ...tree, nodes: tree.nodes.slice(0, tree.crossover),
      branchId: new Int32Array(), baseRadius: new Float64Array(), endRadius: new Float64Array(), twig: new Uint8Array() };
  };

  it.each([
    ["Telperion", TELPERION],
    ["Laurelin", LAURELIN],
  ] as const)("leaves the trunk-to-limb field of %s byte for byte where it was, across branch length ratios", (_name, preset) => {
    const rested = twigged(preset, false);
    const envelope = preset.skeleton.envelope;
    const today = solveRadii({ nodes: rested.nodes }, envelope, preset.radii);
    const atRest = solveRadii(rested, envelope, preset.radii);
    expect(rested.crossover).toBe(rested.nodes.length);
    expect([...atRest.radius]).toEqual([...today.radius]);
    expect([...atRest.startRadius]).toEqual([...today.startRadius]);

    for (const lengthRatio of [0.25, 0.4]) {
      const grown = twigged(preset, true, lengthRatio);
      const field = solveRadii(grown, envelope, preset.radii);
      expect(grown.crossover).toBe(rested.nodes.length);
      expect([...field.radius.subarray(0, grown.crossover)]).toEqual([...today.radius]);
      expect([...field.startRadius.subarray(0, grown.crossover)]).toEqual([...today.startRadius]);
    }
  });

  it("reads branch bases, tapers internodes, and keeps the twig anatomy at both ends", () => {
    const skeleton = twigged(TELPERION, true);
    const envelope = TELPERION.skeleton.envelope;
    const field = solveRadii(skeleton, envelope, TELPERION.radii);
    let checked = 0;
    for (let i = skeleton.crossover; i < skeleton.nodes.length; i++) {
      const record = i - skeleton.crossover;
      const parent = skeleton.nodes[i].parent;
      const expected = skeleton.branchId[record] === i
        ? skeleton.baseRadius[record] : field.radius[parent];
      expect(field.startRadius[i]).toBe(expected);
      expect(field.radius[i]).toBe(skeleton.endRadius[record]);
      expect(field.radius[i]).toBeLessThanOrEqual(expected);
      checked++;
    }
    expect(checked).toBeGreaterThan(1000);
  });

  it.each([TELPERION, LAURELIN])("draws the same twig diameter across heights and handoff radii ($name)", (preset) => {
    for (const height of [24, preset.skeleton.envelope.height]) {
      for (const trunkRadius of [0.01, preset.radii.trunkRadius]) {
        const envelope = { ...preset.skeleton.envelope, height };
        const radii = { ...preset.radii, trunkRadius };
        const skeleton = growSkeleton({ ...preset.skeleton, envelope }, radii);
        const field = solveRadii(skeleton, envelope, radii);
        let twigs = 0;
        for (let i = skeleton.crossover; i < skeleton.nodes.length; i++) {
          if (!skeleton.twig[i - skeleton.crossover]) continue;
          expect(2 * field.startRadius[i]).toBe(preset.skeleton.twigs.twig.diameter);
          expect(2 * field.radius[i]).toBe(preset.skeleton.twigs.twig.diameter);
          twigs++;
        }
        expect(twigs).toBeGreaterThan(0);
      }
    }
  });

  it("keeps long branch internodes monotone and terminal twig radii constant", () => {
    const preset = LAURELIN;
    const skeleton = growSkeleton({
      ...preset.skeleton,
      twigs: { ...preset.skeleton.twigs, internodeFactor: 32 },
      growth: { ...preset.skeleton.growth, maxNodes: 4_000_000 },
    }, preset.radii) as TwiggedSkeleton;
    const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);
    expect(skeleton.crossover).toBeLessThan(skeleton.nodes.length);
    for (let i = skeleton.crossover; i < skeleton.nodes.length; i += 1) {
      const parent = skeleton.nodes[i].parent;
      // The fixed twig is the stated exception: a branch whose tapered end
      // has fallen a hair under 2.5 mm still bears a 5 mm twig.
      if (skeleton.twig[i - skeleton.crossover]) continue;
      expect(field.radius[parent]).toBeGreaterThanOrEqual(field.startRadius[i]);
      if (skeleton.twig[i - skeleton.crossover]) expect(field.startRadius[i]).toBe(field.radius[i]);
      else expect(field.startRadius[i]).toBeGreaterThan(field.radius[i]);
      expect(field.radius[i]).toBeGreaterThan(0);
    }
  });
});
