import { describe, expect, it } from "vitest";
import { growField, prepareField } from "./index";

/* The field entry point in Node (fn-257 R3a): Node has no WebGPU, so every
 * growth rejects by name, and nothing falls back to a CPU growth. Its input
 * contract is checked before any device is asked for. The answers
 * themselves are tested natively, on the device, against the CPU's
 * reference field (`resident/field_tests.rs`). */
describe("the field entry point without WebGPU", () => {
  it("rejects a growth by name, never growing on the CPU", async () => {
    await expect(growField("european-beech", 5)).rejects.toThrow("WebGPU is not available");
    await expect(growField("european-beech", 5, { limbOrder: 1 })).rejects.toThrow("WebGPU is not available");
    await expect(prepareField()).rejects.toThrow("WebGPU is not available");
  });

  it("refuses a malformed request before it asks for a device", async () => {
    await expect(growField("european-beech", 1.5)).rejects.toThrow("seed");
    await expect(growField("european-beech", 2 ** 32)).rejects.toThrow("seed");
    await expect(growField("european-beech", 1, { limbOrder: -1 })).rejects.toThrow("limbOrder");
  });
});
