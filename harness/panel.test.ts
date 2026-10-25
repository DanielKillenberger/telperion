import { describe, expect, it } from "vitest";

import { presetById } from "../src/browser/core";
import { GROUPS } from "../src/browser/parameters.generated";

import { presetToParams } from "./family";
import { changed, controls, matches, reset, resetAll, type Control } from "./panel";
import { writeRow } from "./rows";
import { moveAll } from "./space";

const palm = presetToParams(presetById("date-palm"));
const groups = controls(palm.preset);

function control(id: string): Control {
  const found = groups.flatMap(g => g.controls).find(c => c.id === id);
  if (found === undefined) throw Error(`no control ${id}`);
  return found;
}

describe("the panel's groups", () => {
  it("come in the catalogue's order, size first and the engine's internals last", () => {
    expect(groups.map(g => g.group)).toEqual(GROUPS.map(g => g.id));
    expect(groups[0].title).toBe("size and age");
    expect(groups.at(-1)?.group).toBe("advanced");
  });

  it("place the palm's lean in form, its height in size and the budgets in advanced", () => {
    expect(control("/space/form.elevation").group).toBe("form");
    expect(control("/space/internode").group).toBe("size");
    expect(control("/canopy/maxInstances").group).toBe("advanced");
    expect(control("/canopy/rosetteFronds").group).toBe("leaves");
  });

  it("leave the seed and the age to their own controls", () => {
    const ids = groups.flatMap(g => g.controls).map(c => c.id);
    expect(ids).not.toContain("/age");
    expect(ids).not.toContain("/skeleton/seed");
  });
});

describe("the filter", () => {
  it("finds a control by every word of its label, unit, meaning or name", () => {
    expect(matches(control("/space/form.elevation"), "elevation horizontal")).toBe(true);
    expect(matches(control("/canopy/rosetteFronds"), "rosette")).toBe(true);
    expect(matches(control("/canopy/rosetteFronds"), "rosette bark")).toBe(false);
    expect(matches(control("/canopy/rosetteFronds"), "")).toBe(true);
  });
});

describe("a change from the preset", () => {
  const fronds = control("/canopy/rosetteFronds");
  const lean = control("/space/form.elevation");

  it("is marked on the control that moved and no other", () => {
    const family = writeRow(palm.family, "/canopy/rosetteFronds", 30);
    expect(changed(fronds, family, palm.preset)).toBe(true);
    expect(changed(lean, family, palm.preset)).toBe(false);
  });

  it("goes back by its own reset, and a group's", () => {
    const moved = moveAll(palm.preset, writeRow(palm.family, "/canopy/rosetteFronds", 30),
      lean.kind === "space" ? lean.setting : (() => { throw Error("an engine setting"); })(), -0.3);
    expect(changed(lean, moved, palm.preset)).toBe(true);
    const back = reset(lean, moved, palm.preset);
    expect(changed(lean, back, palm.preset)).toBe(false);
    expect(changed(fronds, back, palm.preset)).toBe(true);
    const all = groups.flatMap(g => g.controls);
    expect(resetAll(all, moved, palm.preset)).toEqual(palm.family);
  });
});
