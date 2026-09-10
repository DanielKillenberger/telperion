import { describe, expect, it } from "vitest";

import {
  DEFAULT_PARAMS,
  SEED_MAX,
  SLIDERS,
  normalizeSeed,
  readSlider,
} from "./params";

describe("normalizeSeed", () => {
  it.each([
    ["0", 0],
    ["1", 1],
    ["4294967295", SEED_MAX],
  ])("accepts %s as a seed", (raw, expected) => {
    expect(normalizeSeed(raw)).toBe(expected);
  });

  it.each([
    ["", "empty"],
    [" 12", "leading space"],
    ["12 ", "trailing space"],
    ["1e3", "exponent"],
    ["0x4", "hex"],
    ["-1", "negative"],
    ["1.5", "fractional"],
    ["4294967296", "above uint32"],
    ["99999999999", "longer than ten digits"],
    ["seed", "not a number"],
  ])("rejects %s (%s)", (raw) => {
    expect(normalizeSeed(raw)).toBeNull();
  });
});

describe("readSlider", () => {
  const height = SLIDERS.find((s) => s.key === "height")!;

  it("passes an in-range value through", () => {
    expect(readSlider(height, "18.5")).toBe(18.5);
  });

  it("clamps below the minimum", () => {
    expect(readSlider(height, "-40")).toBe(height.min);
  });

  it("clamps above the maximum", () => {
    expect(readSlider(height, "1000")).toBe(height.max);
  });

  it.each(["", "abc", "NaN", "Infinity"])(
    "falls back to the default for %p",
    (raw) => {
      expect(readSlider(height, raw)).toBe(DEFAULT_PARAMS.height);
    },
  );

  it("puts the growth step beside density, in the skeleton's stage", () => {
    /* SLIDERS runs in pipeline order and a dial's place states where
       it acts: the step answers the attractors density scatters, so it
       follows density and stays inside the skeleton group rather than
       opening one of its own. */
    const keys = SLIDERS.map((spec) => spec.key);
    const step = SLIDERS[keys.indexOf("step")];
    expect(keys.indexOf("step")).toBe(keys.indexOf("density") + 1);
    expect(step.group).toBeUndefined();
    const groupBefore = SLIDERS.slice(0, keys.indexOf("step"))
      .map((spec) => spec.group)
      .filter((group) => group !== undefined)
      .pop();
    expect(groupBefore).toBe("skeleton");
  });

  it("every slider's default sits inside its own range", () => {
    for (const spec of SLIDERS) {
      const value = DEFAULT_PARAMS[spec.key];
      expect(value).toBeGreaterThanOrEqual(spec.min);
      expect(value).toBeLessThanOrEqual(spec.max);
    }
  });
});


describe("branch-law controls", () => {
  it("offers the law rails and retires depth and fallback canopy controls", () => {
    const branch = SLIDERS.filter((s) =>
      ["lengthRatio", "ratioPower", "internodeFactor", "angleVariation", "vigourVariation", "laterals", "limbRadius"].includes(s.key));
    expect(branch.map(({ key, min, max, step }) => ({ key, min, max, step }))).toEqual([
      { key: "lengthRatio", min: 0.05, max: 1, step: 0.01 },
      { key: "ratioPower", min: 0, max: 8, step: 0.05 },
      { key: "internodeFactor", min: 0.05, max: 32, step: 0.05 },
      { key: "angleVariation", min: 0, max: 90, step: 1 },
      { key: "vigourVariation", min: 0, max: 0.95, step: 0.01 },
      { key: "laterals", min: 0, max: 7, step: 1 },
      { key: "limbRadius", min: 0, max: 1, step: 0.005 },
    ]);
    for (const key of ["twigLevels", "twigThinning", "shootRadius", "spacing", "clump", "clumpSpan"]) {
      expect(SLIDERS.map((s) => s.key)).not.toContain(key);
    }
    expect(DEFAULT_PARAMS).not.toHaveProperty("twigLevels");
  });
});

describe("carried native controls", () => {
  it("keeps the Ordinary baseline natural and every carried trait a number", () => {
    expect(DEFAULT_PARAMS.supernaturalEnabled).toBe(false);
    /* The habit is the Ordinary row of the trait table, fifteen numbers
       and no tag. Written out rather than compared to itself: the panel
       shows the owner these values, and a row the core moves under it is
       a different default tree than the one the harness was tuned on. */
    expect(DEFAULT_PARAMS.family.skeleton.habit).toEqual({
      apicalDominance: 0.5,
      whorlStrength: 0.3,
      leaderInternode: 1.5,
      lateralsPerStation: 3,
      lateralPitch: 60,
      pitchVariation: 15,
      risePrimary: 0.05,
      riseSecondary: 0,
      crookedness: 12,
      lateralSpacing: 0.9,
      lateralLengthRatio: 0.4,
      lateralOrders: 3,
      attractorWeight: 1,
      twigTipTaper: 1,
      sheddingThreshold: 0.45,
    });
    // The leaf and the way it sits on its shoot are rows as well.
    expect(DEFAULT_PARAMS.family.element).toMatchObject({ lobeCount: 0, lobeDepth: 0, sectionRoundness: 0 });
    expect(DEFAULT_PARAMS.family.canopy).toMatchObject({ forwardLean: 0, leanRise: 0, surfaceContact: 0 });
    expect(DEFAULT_PARAMS.family.element).not.toHaveProperty("anatomy");
    expect(DEFAULT_PARAMS.family.canopy).not.toHaveProperty("attachment");
    expect(SLIDERS.map(s => s.key)).not.toContain("family");
    expect(SLIDERS.map(s => s.key)).not.toContain("supernaturalEnabled");
  });
});
