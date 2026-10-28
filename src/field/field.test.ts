import { beforeAll, describe, expect, it } from "vitest";
import { FieldError, growField, prepareField } from "./index";
import { ORDINARY, TreeEngine } from "../browser/core";
import { presetById } from "../browser/presets";
import type { Family } from "../browser/presets";

/* The field entry point in Node (fn-257 R3a): Node has no WebGPU, so every
 * growth rejects with a coded `FieldError` (fn-317 R4), and nothing falls
 * back to a CPU growth. Its input contract is checked before any device is
 * asked for. The answers themselves are tested natively, on the device,
 * against the CPU's reference field (`resident/field_tests.rs`). */
const BEECH = presetById("european-beech");

describe("the field entry point without WebGPU", () => {
  it("rejects a growth with a coded error, never growing on the CPU", async () => {
    // Each refusal is caught as it is made: a growth loads the module before
    // it asks for a device, so the three settle at different times.
    const grown = [growField(BEECH, 24), growField(BEECH, 24, { limbOrder: 1 }), prepareField()];
    for (const refused of await Promise.all(grown.map(g => g.catch((e: unknown) => e)))) {
      expect(refused).toBeInstanceOf(FieldError);
      expect(refused).toMatchObject({ code: "webgpu-unavailable", name: "FieldError" });
      expect(String((refused as Error).message)).toContain("WebGPU is not available");
    }
  });

  it("refuses a preset's id and a malformed request before it asks for a device", async () => {
    await expect(growField("european-beech" as unknown as Family, 24)).rejects.toThrow("not a preset id: pass presetById(id)");
    await expect(growField(BEECH, 24, { limbOrder: -1 })).rejects.toThrow("limbOrder");
    await expect(growField({ ...BEECH, age: Number.NaN }, 24)).rejects.toThrow("finite");
    for (const n of [2, 257, 24.5, Number.NaN]) await expect(growField(BEECH, n)).rejects.toThrow("field grid side");
  });

  it("refuses a family the CPU refuses with the CPU's message before it asks for a device (fn-319 R2)", async () => {
    const scatter = structuredClone(ORDINARY);
    scatter.canopy.scatter = -1;
    const young = structuredClone(ORDINARY);
    young.age = 1.5;
    const both = structuredClone(scatter);
    both.age = 1.5;
    for (const [family, reason] of [[scatter, "scatter"], [young, "age = 1.5"], [both, "age = 1.5"]] as const) {
      const core = (() => { try { engine.build(family, { field: { n: 12 } }); } catch (e) { return (e as Error).message; } })();
      expect(core).toBe(`Tree core: ${reason}`);
      const refused = await growField(family, 12).catch((e: unknown) => e);
      expect(refused).not.toBeInstanceOf(FieldError);
      expect((refused as Error).message).toBe(`the generator rejected the tree: invalid input: ${reason}`);
    }
  });
});

let engine: TreeEngine;
beforeAll(async () => { engine = await TreeEngine.create(); });
