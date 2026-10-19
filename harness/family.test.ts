import { describe, expect, it } from "vitest";

import { PRESETS } from "../src/browser/core";

import { familyJson, presetToParams, toFamily } from "./family";
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

describe("every row the build reads", () => {
  it("reaches the family from its own control", () => {
    /* A sentinel per row, so a row that happened to hold the value the
       control writes cannot pass as reached. Stems and the side-branch
       orders were the rows the owner could not reach (fn-159). */
    const owned = new Set(["/skeleton/seed"]);
    const rows = shownRows();
    expect(rows.map(p => p.path)).toEqual(expect.arrayContaining(
      ["/radii/trunkRadius", "/canopy/rosetteFronds", "/supernatural/enabled"]));
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
    const family = writeRow(DEFAULT_PARAMS.family, "/radii/trunkRadius", Number.NaN);
    expect(() => familyJson({ ...DEFAULT_PARAMS, family })).toThrow(/trunkRadius/);
  });
});
