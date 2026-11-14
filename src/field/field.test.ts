import { afterEach, beforeAll, describe, expect, it, vi } from "vitest";
import { growField } from "./index";
import { ORDINARY, TreeEngine } from "../browser/core";
import { presetById } from "../browser/presets";
import type { Family } from "../browser/presets";

/* The field entry point grows on the CPU (fn-353 R3): in Node, which has no
 * WebGPU, and where a page or worker has it alike, it returns the grid the
 * main entry's `TreeEngine.build(family, { field })` reads out, and refuses
 * what that refuses with the same message. */
const BEECH = presetById("european-beech");

describe("the field entry point on the CPU", () => {
  it("returns the main entry's field, with and without WebGPU", async () => {
    const built = engine.build(BEECH, { field: { n: 24, limbOrder: 1 } }).field;
    expect(await growField(BEECH, 24, { limbOrder: 1 })).toEqual(built);
    vi.stubGlobal("navigator", { gpu: { requestAdapter: async () => ({}) } });
    expect(await growField(BEECH, 24, { limbOrder: 1 })).toEqual(built);
  });

  it("refuses a preset's id and a malformed request by name", async () => {
    await expect(growField("european-beech" as unknown as Family, 24)).rejects.toThrow("not a preset id: pass presetById(id)");
    await expect(growField(BEECH, 24, { limbOrder: -1 })).rejects.toThrow("limbOrder");
    await expect(growField({ ...BEECH, age: Number.NaN }, 24)).rejects.toThrow("finite");
    for (const n of [2, 257, 24.5, Number.NaN]) await expect(growField(BEECH, n)).rejects.toThrow("field grid side");
  });

  it("refuses a family the core refuses with the core's message", async () => {
    const scatter = structuredClone(ORDINARY);
    scatter.canopy.scatter = -1;
    const young = structuredClone(ORDINARY);
    young.age = 1.5;
    for (const [family, reason] of [[scatter, "scatter"], [young, "age = 1.5"]] as const) {
      await expect(growField(family, 12)).rejects.toThrow(`Tree core: ${reason}`);
    }
  });
});

let engine: TreeEngine;
beforeAll(async () => { engine = await TreeEngine.create(); });
afterEach(() => vi.unstubAllGlobals());
