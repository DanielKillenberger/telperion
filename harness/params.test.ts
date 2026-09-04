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
