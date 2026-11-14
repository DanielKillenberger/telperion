import { afterEach, describe, expect, it, vi } from "vitest";

/* fn-317: a preset is its parameters. `telperion/presets` is plain data
 * that loads with no Wasm, and every generator entry takes the parameters,
 * never a preset's id. */
afterEach(() => vi.unstubAllGlobals());

describe("the presets entry", () => {
  it("loads in Node and fetches nothing", async () => {
    const fetched = vi.fn();
    vi.stubGlobal("fetch", fetched);
    const presets = await import("./presets");
    expect(presets.PRESETS.map(p => p.id)).toContain("telperion");
    expect(presets.presetById("laurelin").material.bloomFront).toEqual(presets.LAURELIN.material.bloomFront);
    expect(fetched).not.toHaveBeenCalled();
  });
});

describe("the CPU engine", () => {
  it("refuses a preset's id by name and builds its parameters; one dial changes the field", async () => {
    const { TreeEngine } = await import("./core");
    const { presetById } = await import("./presets");
    const engine = await TreeEngine.create();
    expect(() => engine.build("telperion" as never, { field: { n: 8 } })).toThrow("not a preset id: pass presetById(id)");
    const birch = presetById("silver-birch");
    const field = engine.build(birch, { field: { n: 12 } }).field!;
    const younger = engine.build({ ...birch, age: birch.age - 10 }, { field: { n: 12 } }).field!;
    expect(younger.cell).toBeLessThan(field.cell);
    engine.dispose();
  }, 120_000);
});
