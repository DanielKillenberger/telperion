import { describe, expect, it } from "vitest";

import { PRESETS } from "../src/browser/core";

import { ADAPTERS, adapterAdmits, attractors, familyJson, presetToParams, toFamily } from "./family";
import { DEFAULT_PARAMS } from "./params";
import { readRow, shownRows, writeRow } from "./rows";

/* The panel's half of the generator contract, on its own: what a dial
   move becomes before anything draws it. No GPU and no renderer here -
   this is the translation, and the translation is where a dropped term
   would silently show the owner a tree nobody authored. */

describe("presetToParams", () => {
  it.each(PRESETS.map(preset => [preset.id, preset] as const))(
    "%s survives the dials as the family it was authored as",
    (_id, preset) => {
      const { id: _ignored, name: _name, note: _note, ...family } = preset;
      expect(toFamily(presetToParams(preset))).toEqual(family);
    },
  );
});

describe("density", () => {
  it("is the attractor count over its span, and owns that row", () => {
    expect(ADAPTERS.find(a => a.key === "density")?.owns).toBe("/skeleton/attractors");
    expect(toFamily({ ...DEFAULT_PARAMS, density: 0 }).skeleton.attractors).toBe(250);
    expect(toFamily({ ...DEFAULT_PARAMS, density: 1 }).skeleton.attractors).toBe(1600);
    expect(attractors(0.5)).toBe(925);
  });
});

describe("torsion", () => {
  it("scales the supernatural bending and nothing else", () => {
    /* One move from straight to writhing. Lean and gravitropism are
       outside it: a tree that wants to grow up still wants to when it is
       not twisting. The surface twist winds the skin, not the path. */
    const bent = writeRow(writeRow(DEFAULT_PARAMS.family, "/skeleton/bias/supernatural/writheAmplitude", 0.1),
      "/skeleton/bias/supernatural/spiralRate", 2);
    const params = { ...DEFAULT_PARAMS, family: bent };
    const doubled = toFamily({ ...params, torsion: 2 });
    const straight = toFamily({ ...params, torsion: 0 });
    expect([doubled.skeleton.bias.supernatural.writheAmplitude, doubled.skeleton.bias.supernatural.spiralRate])
      .toEqual([0.2, 4]);
    expect([straight.skeleton.bias.supernatural.writheAmplitude, straight.skeleton.bias.supernatural.spiralRate])
      .toEqual([0, 0]);
    const untouched = (family: typeof bent) => ({
      ...family,
      skeleton: { ...family.skeleton, bias: { ...family.skeleton.bias,
        supernatural: { ...family.skeleton.bias.supernatural, writheAmplitude: 0, spiralRate: 0 } } },
    });
    expect(untouched(doubled)).toEqual(untouched(toFamily(params)));
  });
});

describe("an adapter's bounds", () => {
  it("are the bounds of the rows it moves", () => {
    const [density, torsion] = ["density", "torsion"].map(key => ADAPTERS.find(a => a.key === key)!);
    expect(adapterAdmits(DEFAULT_PARAMS, density, 0.5)).toBe(true);
    expect(adapterAdmits(DEFAULT_PARAMS, density, -1)).toBe(false);
    expect(adapterAdmits(DEFAULT_PARAMS, density, Number.NaN)).toBe(false);
    const bent = writeRow(DEFAULT_PARAMS.family, "/skeleton/bias/supernatural/writheAmplitude", 0.1);
    expect(adapterAdmits({ ...DEFAULT_PARAMS, family: bent }, torsion, 2)).toBe(true);
    expect(adapterAdmits({ ...DEFAULT_PARAMS, family: bent }, torsion, -1)).toBe(false);
  });
});

describe("every row the build reads", () => {
  it("reaches the family from its own control or its adapter", () => {
    /* A sentinel per row, so a row that happened to hold the value the
       control writes cannot pass as reached. Stems and the side-branch
       orders were the rows the owner could not reach (fn-159). */
    const owned = new Set(["/skeleton/seed", ...ADAPTERS.map(a => a.owns)]);
    const rows = shownRows(false);
    expect(rows.map(p => p.path)).toEqual(expect.arrayContaining(
      ["/skeleton/habit/stems", "/skeleton/habit/lateralOrders", "/skeleton/bias/supernatural/enabled"]));
    for (const p of rows.filter(p => !owned.has(p.path))) {
      const now = readRow(DEFAULT_PARAMS.family, p.path);
      const sentinel = p.kind === "switch" ? now !== true : (typeof now === "number" ? now : 0) + 1;
      const family = writeRow(DEFAULT_PARAMS.family, p.path, sentinel);
      expect(readRow(toFamily({ ...DEFAULT_PARAMS, family }), p.path), p.path).toBe(sentinel);
    }
    expect(toFamily({ ...DEFAULT_PARAMS, seed: 7 }).skeleton.seed).toBe(7);
  });
});

describe("familyJson", () => {
  it("is the same family, as the text the renderer parses", () => {
    expect(JSON.parse(familyJson(DEFAULT_PARAMS))).toEqual(toFamily(DEFAULT_PARAMS));
  });

  it("names the row that is not a number rather than sending null", () => {
    const family = writeRow(DEFAULT_PARAMS.family, "/skeleton/envelope/height", Number.NaN);
    expect(() => familyJson({ ...DEFAULT_PARAMS, family })).toThrow(/height/);
  });
});
