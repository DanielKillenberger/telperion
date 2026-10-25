import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

/* The field's device refusals (fn-317 R4b), the module stood in for: its
 * coded refusals reach the caller as `FieldError`s with the entry's own
 * codes, and a module that found no adapter is asked once more only where
 * the browser, asked directly, has one. */
const grow = vi.fn();
vi.mock("../browser/module", () => ({ loadModule: async () => undefined }));
vi.mock("../browser/render/telperion_render.js", () => ({ WebField: { grow: (...a: unknown[]) => grow(...a), prepare: async () => undefined } }));
const { FieldError, growField } = await import("./index");
const { presetById } = await import("../browser/presets");

const refusal = (code: string, message: string) => Object.assign(Error(message), { code });
const field = { n: () => 3, cell: () => 1, origin: () => [0, 0, 0], answers: () => new Uint32Array(5 * 27), free: () => undefined };
let adapter: unknown;
beforeEach(() => {
  grow.mockReset();
  vi.stubGlobal("navigator", { gpu: { requestAdapter: async () => adapter } });
});
afterEach(() => vi.unstubAllGlobals());

describe("the field's device refusals", () => {
  it("asks the module once more where its first request found no adapter and the browser has one", async () => {
    adapter = {};
    grow.mockRejectedValueOnce(refusal("webgpu-unavailable", "WebGPU is unavailable: No suitable graphics adapter found")).mockResolvedValueOnce(field);
    const tree = await growField(presetById("ordinary"), 3);
    expect(tree.n).toBe(3);
    expect(grow).toHaveBeenCalledTimes(2);
  });

  it("refuses with the entry's code where the browser has no adapter either, asking once", async () => {
    adapter = null;
    grow.mockRejectedValue(refusal("webgpu-unavailable", "WebGPU is unavailable: No suitable graphics adapter found"));
    const refused = await growField(presetById("ordinary"), 3).catch((e: unknown) => e);
    expect(refused).toBeInstanceOf(FieldError);
    expect(refused).toMatchObject({ code: "webgpu-unavailable", message: "WebGPU is unavailable: No suitable graphics adapter found" });
    expect(grow).toHaveBeenCalledTimes(1);
  });

  it("refuses a lost device with its code and does not ask again", async () => {
    adapter = {};
    grow.mockRejectedValue(refusal("device-lost", "the GPU device was lost: Destroyed"));
    await expect(growField(presetById("ordinary"), 3)).rejects.toMatchObject({ name: "FieldError", code: "device-lost" });
    expect(grow).toHaveBeenCalledTimes(1);
  });

  it("passes another refusal through as the module made it", async () => {
    const core = Error("field grid side = 300");
    grow.mockRejectedValue(core);
    await expect(growField(presetById("ordinary"), 3)).rejects.toBe(core);
  });

  it("hands the module the parameters, never an id", async () => {
    grow.mockResolvedValue(field);
    const telperion = presetById("telperion");
    await growField(telperion, 3, { limbOrder: 2 });
    const [parameters, n, order] = grow.mock.calls[0];
    expect([n, order]).toEqual([3, 2]);
    const sent = JSON.parse(parameters as string);
    expect(sent.id ?? sent.name ?? sent.note).toBeUndefined();
    expect(sent.skeleton.seed).toBe(telperion.skeleton.seed);
  });
});
