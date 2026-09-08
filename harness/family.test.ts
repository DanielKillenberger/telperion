import { describe, expect, it } from "vitest";

import { ORDINARY, PRESETS as CATALOGUE, TWO_TREES } from "../src/browser/core";

import { DEFAULT_PARAMS, SLIDERS } from "./params";
import {
  presetToParams,
  toCanopyParams,
  toFamily,
  familyJson,
  toRadiusParams,
  toSkeletonParams,
  toSurfaceParams,
} from "./family";

/* The panel's half of the generator contract, on its own: what a dial
   move becomes before anything draws it. No GPU and no renderer here -
   this is the translation, and the translation is where a dropped term
   would silently show the owner a tree nobody authored. */

const PRESETS = TWO_TREES;
const DEFAULT_ENVELOPE = ORDINARY.skeleton.envelope;
const DEFAULT_SURFACE = ORDINARY.surface;
const DEFAULT_RADII = ORDINARY.radii;
const DEFAULT_BIAS = ORDINARY.skeleton.bias;

describe("toSkeletonParams", () => {
  it("hands the bias dials over under the library's own names", () => {
    const mapped = toSkeletonParams({
      ...DEFAULT_PARAMS,
      gravitropism: 0.9,
      lean: 0.21,
      writheAmplitude: 0.13,
      writheWavelength: 0.31,
      spiralRate: 2.5,
    });
    expect(mapped.bias).toEqual({
      gravitropism: 0.9,
      lean: 0.21,
      supernatural: {
        enabled: DEFAULT_PARAMS.supernaturalEnabled,
        writheAmplitude: 0.13,
        writheWavelength: 0.31,
        spiralRate: 2.5,
      },
    });
  });

  it("defaults every bias dial to the library's own default", () => {
    // The panel is not allowed a second opinion about what a tree looks
    // like out of the box; fn-11.7's presets are the library's business.
    expect(toSkeletonParams(DEFAULT_PARAMS).bias).toEqual(DEFAULT_BIAS);
  });

  it("scales supernatural bending without changing botanical lean", () => {
    /* One move from straight to writhing. Gravitropism is outside it:
       a tree that wants to grow up still wants to when it is not
       twisting, and folding it in would make torsion 0 a tree with no
       opinion about direction at all. */
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, lean: 0.2, torsion: 0 }).bias.lean).toBe(0.2);
    const straight = toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 0 });
    expect(straight.bias).toEqual({
      gravitropism: DEFAULT_BIAS.gravitropism,
      lean: DEFAULT_BIAS.lean,
      supernatural: {
        enabled: DEFAULT_PARAMS.supernaturalEnabled,
        writheAmplitude: 0,
        writheWavelength: DEFAULT_BIAS.supernatural.writheWavelength,
        spiralRate: 0,
      },
    });

    const doubled = toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 2 });
    expect(doubled.bias).toEqual({
      gravitropism: DEFAULT_BIAS.gravitropism,
      lean: DEFAULT_BIAS.lean,
      supernatural: {
        enabled: DEFAULT_PARAMS.supernaturalEnabled,
        writheAmplitude: DEFAULT_BIAS.supernatural.writheAmplitude * 2,
        writheWavelength: DEFAULT_BIAS.supernatural.writheWavelength,
        spiralRate: DEFAULT_BIAS.supernatural.spiralRate * 2,
      },
    });
  });

  it("passes the envelope dials straight through", () => {
    const mapped = toSkeletonParams({
      ...DEFAULT_PARAMS,
      seed: 9,
      height: 31,
      spread: 0.42,
    });
    expect(mapped.seed).toBe(9);
    expect(mapped.envelope.height).toBe(31);
    expect(mapped.envelope.spread).toBe(0.42);
  });

  it("hands the turn limit over as the growth argument it is", () => {
    // Not a bias term: persistence is about the step, not about the
    // field, so it travels in `growth` under the library's own name.
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, maxTurnPerStep: 18 }).growth)
      .toEqual({ maxTurnPerStep: 18 });
    // And it is outside torsion - a stiff tree is stiff whether or not
    // it is writhing.
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 0 }).growth).toEqual({
      maxTurnPerStep: DEFAULT_PARAMS.maxTurnPerStep,
    });
  });

  it("hands the growth step over under the library's own name", () => {
    // A sibling of the attractor count, not a growth override: the
    // library derives the distances from it.
    const mapped = toSkeletonParams({ ...DEFAULT_PARAMS, step: 0.011 });
    expect(mapped.step).toBe(0.011);
    expect(mapped.growth).toEqual({ maxTurnPerStep: DEFAULT_PARAMS.maxTurnPerStep });
  });

  it("hands branch anatomy and the law over under the library's own names", () => {
    // Every member of `twigs` named, so the panel cannot drop one; and
    // the lateral count reaches the tree and adds branches.
    const mapped = toSkeletonParams({ ...DEFAULT_PARAMS, laterals: 2 });
    expect(mapped.twigs).toEqual({
      twig: { length: DEFAULT_PARAMS.twigLength, diameter: DEFAULT_PARAMS.twigDiameter, internodeLength: DEFAULT_PARAMS.twigStationLength,
        stationsPerInternode: DEFAULT_PARAMS.twigStations, bearingDiameter: DEFAULT_PARAMS.twigBearing },
      ratioPower: DEFAULT_PARAMS.ratioPower,
      limbRadius: DEFAULT_PARAMS.limbRadius,
      reach: DEFAULT_PARAMS.reach,
      laterals: 2,
      angle: DEFAULT_PARAMS.twigAngle,
      divergence: DEFAULT_PARAMS.twigDivergence,
      internodeFactor: DEFAULT_PARAMS.internodeFactor,
      angleVariation: DEFAULT_PARAMS.angleVariation,
      vigourVariation: DEFAULT_PARAMS.vigourVariation,
      lengthRatio: DEFAULT_PARAMS.lengthRatio,
    });
    expect(mapped.twigs).not.toHaveProperty("levels");
  });

  it("turns density into an attractor count", () => {
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, density: 0 }).attractors).toBe(
      250,
    );
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, density: 1 }).attractors).toBe(
      1600,
    );
    const middle = toSkeletonParams({ ...DEFAULT_PARAMS, density: 0.5 });
    expect(middle.attractors).toBeGreaterThan(250);
    expect(middle.attractors).toBeLessThan(1600);
  });
});

describe("toSurfaceParams", () => {
  it("hands the four surface dials over under the library's names", () => {
    const mapped = toSurfaceParams({
      ...DEFAULT_PARAMS,
      lobes: 7,
      lobeDepth: 0.23,
      twistRate: -2.5,
      flareRadius: 3.1,
    });
    expect(mapped).toEqual({
      ...DEFAULT_SURFACE,
      lobes: 7,
      lobeDepth: 0.23,
      twistRate: -2.5,
      flareRadius: 3.1,
    });
  });

  it("leaves the surface twist outside the torsion master", () => {
    /* `torsion` gathers the three terms that bend the CENTRELINE. The
       plait is the skin winding about that path - a different
       mechanism, and one dial meaning both is exactly what the spec's
       parameter principle rules out. */
    const straight = { ...DEFAULT_PARAMS, torsion: 0, twistRate: 2 };
    expect(toSurfaceParams(straight).twistRate).toBe(2);
    expect(toSkeletonParams(straight).bias?.supernatural.spiralRate).toBe(0);
  });
});

describe("toRadiusParams", () => {
  it("hands the taper dial over as the fork exponent", () => {
    expect(toRadiusParams({ ...DEFAULT_PARAMS, taper: 2.6 }).forkExponent).toBe(
      2.6,
    );
  });

  it("hands the solve's other two terms over as dials", () => {
    /* They used to be defaults with no dial, and the note here said
       the day one of them got a dial it would get one here rather than
       a number invented in the harness. This is that day: how stout a
       tree is at the ground is what makes a 60 m tree read as 60 m,
       and the library will not do it on its own. */
    const mapped = toRadiusParams({
      ...DEFAULT_PARAMS,
      trunkRadius: 0.032,
      lengthTaper: 0.9,
    });
    expect(mapped.trunkRadius).toBe(0.032);
    expect(mapped.lengthTaper).toBe(0.9);
    // Out of the box the panel still has no opinion of its own.
    expect(toRadiusParams(DEFAULT_PARAMS)).toEqual(DEFAULT_RADII);
  });

  it("hands the bare-trunk height over as the envelope term it is", () => {
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, crownBase: 0.45 }).envelope)
      .toMatchObject({ crownBase: 0.45 });
    expect(toSkeletonParams(DEFAULT_PARAMS).envelope.crownBase).toBe(
      DEFAULT_ENVELOPE.crownBase,
    );
  });
});

describe("presetToParams", () => {
  it.each(CATALOGUE.map((preset) => [preset.id, preset] as const))(
    "%s round-trips through the dials, term for term",
    (_id, preset) => {
      const dialled = presetToParams(preset);
      expect(toSkeletonParams(dialled)).toEqual(preset.skeleton);
      expect(toRadiusParams(dialled)).toEqual(preset.radii);
      expect(toSurfaceParams(dialled)).toEqual(preset.surface);
      expect(toCanopyParams(dialled)).toEqual(preset.canopy);
    },
  );

  it.each(PRESETS.map((preset) => [preset.id, preset] as const))(
    "%s sits inside every slider's own range",
    (_id, preset) => {
      // A preset the panel clamps on arrival is a preset the owner
      // cannot get back to after one drag of the slider it fell
      // outside of.
      const dialled = presetToParams(preset);
      for (const spec of SLIDERS) {
        expect(dialled[spec.key]).toBeGreaterThanOrEqual(spec.min);
        expect(dialled[spec.key]).toBeLessThanOrEqual(spec.max);
      }
    },
  );
});

describe("toFamily", () => {
  it("composes every dialled term onto the family the preset carried", () => {
    const params = { ...DEFAULT_PARAMS, height: 31, lobes: 7, taper: 2.6 };
    const family = toFamily(params);
    expect(family.skeleton).toEqual(toSkeletonParams(params));
    expect(family.radii).toEqual(toRadiusParams(params));
    expect(family.surface).toEqual(toSurfaceParams(params));
    expect(family.canopy).toEqual(toCanopyParams(params));
    // Every term the dials do not reach still travels, unchanged.
    expect(family.element).toEqual(params.family.element);
  });

  it.each(CATALOGUE.map((preset) => [preset.id, preset] as const))(
    "%s survives the dials as the family it was authored as",
    (_id, preset) => {
      const { id: _ignored, name: _name, note: _note, ...family } = preset;
      expect(toFamily(presetToParams(preset))).toEqual(family);
    },
  );
});

describe("familyJson", () => {
  it("is the same family, as the text the renderer parses", () => {
    expect(JSON.parse(familyJson(DEFAULT_PARAMS))).toEqual(toFamily(DEFAULT_PARAMS));
  });

  it("names the dial that is not a number rather than sending null", () => {
    // JSON would otherwise replace a non-finite value with `null`, and
    // the schema would refuse it as a type error naming nothing.
    expect(() => familyJson({ ...DEFAULT_PARAMS, height: Number.NaN })).toThrow(/height/);
  });
});
