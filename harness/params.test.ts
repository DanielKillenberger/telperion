import { describe, expect, it } from "vitest";

import { DEFAULT_PARAMS, SEED_MAX, growthFromQuery, normalizeSeed } from "./params";

describe("growthFromQuery", () => {
  it.each([
    ["?growth=1", true],
    ["?species=silver-birch&seed=1&growth=1", true],
    ["", false],
    ["?species=silver-birch&seed=1", false],
    ["?growth=0", false],
    ["?growth=true", false],
  ])("reads %s as growth=%s", (search, expected) => {
    expect(growthFromQuery(search)).toBe(expected);
  });
});

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

describe("carried native controls", () => {
  it("keeps the Ordinary baseline natural and every carried trait a number", () => {
    expect(DEFAULT_PARAMS.family.skeleton.bias.supernatural.enabled).toBe(false);
    /* The habit is the Ordinary row of the trait table, twenty numbers
       and no tag. Written out rather than compared to itself: the panel
       shows the owner these values, and a row the core moves under it is
       a different default tree than the one the harness was tuned on. */
    expect(DEFAULT_PARAMS.family.skeleton.habit).toEqual({
      reachProbeSteps: 96,
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
      stems: 1,
      stemDivergence: 0,
      stemLean: 0,
      stemLeanSpread: 0,
      stemForkHeight: 0,
    });
    // The leaf and the way it sits on its shoot are rows as well.
    expect(DEFAULT_PARAMS.family.element).toMatchObject({ lobeCount: 0, lobeDepth: 0, sectionRoundness: 0 });
    expect(DEFAULT_PARAMS.family.canopy).toMatchObject({ forwardLean: 0, leanRise: 0, surfaceContact: 0 });
    // Short shoots are canopy rows the panel renders as its own controls,
    // neutral at a spacing of zero: no table grows one until it says so.
    expect(DEFAULT_PARAMS.family.canopy).toMatchObject({
      shortShootSpacing: 0, shortShootRadius: 0.15, shortShootLength: 0.04,
      shortShootLeaves: 3, shortShootSpread: 45,
    });
    // So is the gap between limb systems, neutral at none.
    expect(DEFAULT_PARAMS.family.canopy).toMatchObject({ limbClumping: 0 });
    expect(DEFAULT_PARAMS.family.element).not.toHaveProperty("anatomy");
    expect(DEFAULT_PARAMS.family.canopy).not.toHaveProperty("attachment");
  });
});
