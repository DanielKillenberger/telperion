import { describe, expect, it } from "vitest";

import { PRESETS } from "../src/browser/core";
import { PARAMETERS, type Parameter } from "../src/browser/parameters.generated";

import { admit, groupOf, labelOf, notch, readRow, shownRows, span } from "./rows";

function leaves(value: unknown, at: string, out: Map<string, unknown>): Map<string, unknown> {
  if (typeof value === "object" && value !== null) {
    for (const [key, inner] of Object.entries(value)) leaves(inner, `${at}/${key}`, out);
  } else out.set(at, value);
  return out;
}

function row(path: string): Parameter {
  const found = PARAMETERS.find(p => p.path === path);
  if (found === undefined) throw Error(`no row ${path}`);
  return found;
}

describe("the generated metadata", () => {
  it.each(PRESETS.map(preset => [preset.id, preset] as const))(
    "names every row of %s's wire, and only those, with its type",
    (_id, preset) => {
      const { id: _ignored, name: _name, note: _note, ...family } = preset;
      const wire = leaves(family, "", new Map());
      const paths = new Set(PARAMETERS.map(p => p.path));
      for (const path of wire.keys()) expect(paths, path).toContain(path);
      for (const p of PARAMETERS) {
        const value = readRow(family, p.path);
        if (p.optional && value === undefined) continue;
        expect(typeof value, p.path).toBe(p.kind === "switch" ? "boolean" : "number");
      }
    },
  );
});

describe("shownRows", () => {
  it("draws the mature build's rows, the growth path's only when it is open", () => {
    const mature = shownRows(false).map(p => p.path);
    expect(mature).not.toContain("/growth/workBudget");
    expect(shownRows(true).map(p => p.path)).toContain("/growth/workBudget");
    expect(mature).not.toContain("/canopy/spacing");
    expect(shownRows(true).map(p => p.path)).not.toContain("/canopy/spacing");
  });
});

describe("the control's arithmetic", () => {
  const stems = row("/skeleton/habit/stems");
  const shell = row("/shellDepth");

  it("groups and labels a row by its pointer", () => {
    expect([groupOf(stems.path), labelOf(stems.path)]).toEqual(["/skeleton/habit", "stems"]);
    expect([groupOf(shell.path), labelOf(shell.path)]).toEqual(["/", "shell depth"]);
  });

  it("spans the tuning window, widened to the value, and none where unbounded", () => {
    expect(span(shell, 0.4)).toEqual([0, 1]);
    const lateral = row("/skeleton/habit/lateralsPerStation");
    expect(span(lateral, 6)).toEqual([1, 6]);
    expect(span(row("/skeleton/envelope/height"), 20)).not.toBeNull();
    const unbounded = PARAMETERS.find(p => !p.dial && p.high === Infinity);
    expect(unbounded && span(unbounded, 1)).toBeNull();
    expect(notch(stems, [1, 6])).toBe(1);
    expect(notch(shell, [0, 1])).toBe(0.001);
  });

  it("admits a number as the row does", () => {
    expect(admit(stems, 2.6)).toBe(3);
    expect(admit(stems, 99)).toBe(stems.high);
    expect(admit(shell, -1)).toBe(0);
    expect(admit(shell, Number.NaN)).toBeNull();
    const open = PARAMETERS.find(p => p.lowOpen && !p.zero)!;
    expect(admit(open, open.low)).toBeNull();
    const zero = PARAMETERS.find(p => p.zero && p.low > 0)!;
    expect(admit(zero, 0)).toBe(0);
  });
});
