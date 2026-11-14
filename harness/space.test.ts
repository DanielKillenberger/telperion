import { describe, expect, it } from "vitest";

import { presetById } from "../src/browser/core";
import { GROUPS, SPACE_SETTINGS, type SpaceSetting } from "../src/browser/parameters.generated";

import { presetToParams } from "./family";
import { admits, moveAll, moved, places, readSetting, stated, writeSetting } from "./space";

function setting(name: string): SpaceSetting {
  const found = SPACE_SETTINGS.find(s => s.name === name);
  if (found === undefined) throw Error(`no setting ${name}`);
  return found;
}

const palm = presetToParams(presetById("date-palm"));
const elevation = setting("form.elevation");
const internode = setting("internode");

describe("the engine's table", () => {
  it("offers every setting but the bud tables, each in a group", () => {
    const names = SPACE_SETTINGS.map(s => s.name);
    expect(names).toContain("form.elevation");
    expect(names).not.toContain("zone.lateral");
    const groups = new Set(GROUPS.map(g => g.id));
    for (const s of SPACE_SETTINGS) expect(groups.has(s.group), s.name).toBe(true);
  });
});

describe("a move on the setting's scale", () => {
  it("multiplies a magnitude above its floor and offsets an angle", () => {
    expect(moved(internode, 0.03, 2)).toBeCloseTo((0.03 + 0.001) * 2 - 0.001, 12);
    expect(moved(elevation, 0.5, 0.25)).toBeCloseTo(0.75, 12);
  });

  it("holds each value to its bounds, and an open end unmoved", () => {
    expect(moved(elevation, 1.5, 1)).toBe(elevation.high);
    expect(moved(internode, 99, 4)).toBe(internode.high);
    expect(admits(internode, 0)).toBe(false);
  });

  it("moves a kept span on the share it keeps, to never and back", () => {
    const shedding = setting("shedding");
    expect(moved(shedding, Infinity, 0.5)).toBe(1);
    expect(moved(shedding, 1, -1)).toBe(Infinity);
    expect(moved(setting("leaf_life"), 1, -1)).toBe(1);
  });
});

describe("the palm's lean", () => {
  it("moves every age the palm states, from the preset's values", () => {
    const at = places(palm.preset, elevation);
    expect(at.length).toBeGreaterThan(0);
    const family = moveAll(palm.preset, palm.family, elevation, -0.5);
    for (const place of at) {
      const from = readSetting(palm.preset, elevation, place);
      expect(readSetting(family, elevation, place)).toBeCloseTo(Math.max(elevation.low, from - 0.5), 12);
    }
    expect(moveAll(palm.preset, family, elevation, 0)).toEqual(palm.family);
  });

  it("writes one age as its row, and never as the wire's token", () => {
    const place = places(palm.preset, internode)[0];
    const family = writeSetting(palm.family, internode, place, 0.05);
    expect(readSetting(family, internode, place)).toBe(0.05);
    const shed = writeSetting(palm.family, setting("shedding"), place, Infinity);
    expect(JSON.stringify(shed.space)).toContain("\"never\"");
  });

  it("offers the zone settings only where the species states a zone", () => {
    expect(stated(palm.preset, SPACE_SETTINGS).map(s => s.name)).toContain("zone.buds");
  });
});
