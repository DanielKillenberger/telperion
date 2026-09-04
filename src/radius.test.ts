import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE, type Envelope } from "@/lib/grower/envelope";
import {
  DEFAULT_RADII,
  solveRadii,
  type RadiusParams,
} from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";
import { growSkeleton } from "@/lib/grower/skeleton/grow";

/* The radius solve makes three claims that a box with no GPU can hold
   to account, and they are the whole of the task's acceptance: the
   fork rule holds at every junction, thickness never grows toward the
   tips, and no radius is ever zero or negative whatever it is handed.
   The fourth - that the exponent visibly changes the taper - is the
   owner's to judge on screen, but "visibly" has a measurable floor and
   that floor is tested here. */

/** A real tree, small enough that the whole file runs in a blink. */
function grownSkeleton(seed = 7, attractors = 400): Skeleton {
  return growSkeleton({ seed, envelope: DEFAULT_ENVELOPE, attractors });
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

  it("keeps the same proportions at any envelope height", () => {
    // The same tree at two sizes: positions scaled by fifteen and the
    // envelope with them. Everything the solve does is a fraction of
    // height, so the radii have to come back scaled by fifteen too - a
    // 4 m tree is not a 60 m tree's twig.
    const small = solveRadii(forked(4), { ...DEFAULT_ENVELOPE, height: 4 }, params());
    const large = solveRadii(forked(60), { ...DEFAULT_ENVELOPE, height: 60 }, params());
    small.radius.forEach((value, index) => {
      expect(large.radius[index] / value).toBeCloseTo(15, 9);
    });
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
