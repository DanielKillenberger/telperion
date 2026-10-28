import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

/* `compileField()` compiles the module beside it once, as 0.1.4 did, and
 * keeps no failed compile (fn-319 R4). The file and the compiler are stood
 * in for, so each test counts the compiles its calls start. */
const source = vi.fn(async () => new Uint8Array(8));
vi.mock("../wasm-source", () => ({ wasmSource: () => source() }));
vi.mock("../browser/render/telperion_render.js", () => ({ WebField: {} }));
vi.mock("../browser/module", () => ({ loadModule: async () => undefined }));

const module = {} as WebAssembly.Module;
let compile: ReturnType<typeof vi.spyOn>;
beforeEach(() => {
  vi.resetModules();
  source.mockClear();
  compile = vi.spyOn(WebAssembly, "compile").mockResolvedValue(module);
});
afterEach(() => compile.mockRestore());

describe("compileField", () => {
  it("resolves two concurrent default calls, and a later one, to one compiled module", async () => {
    const { compileField } = await import("./index");
    const [a, b] = await Promise.all([compileField(), compileField()]);
    expect(a).toBe(module);
    expect(b).toBe(a);
    expect(await compileField()).toBe(a);
    expect(compile).toHaveBeenCalledTimes(1);
    expect(source).toHaveBeenCalledTimes(1);
  });

  it("keeps no rejected compile: the next default call compiles again", async () => {
    const { compileField } = await import("./index");
    const broken = Error("CompileError: bad magic");
    compile.mockRejectedValueOnce(broken);
    await expect(compileField()).rejects.toBe(broken);
    expect(await compileField()).toBe(module);
    expect(compile).toHaveBeenCalledTimes(2);
  });

  it("returns a Module as given and compiles an explicit source on every call", async () => {
    const { compileField } = await import("./index");
    const given = new WebAssembly.Module(new Uint8Array([0, 97, 115, 109, 1, 0, 0, 0]));
    expect(await compileField(given)).toBe(given);
    const bytes = new Uint8Array(8);
    await compileField(bytes);
    await compileField(bytes);
    expect(compile).toHaveBeenCalledTimes(2);
    expect(compile).toHaveBeenLastCalledWith(bytes);
    expect(source).not.toHaveBeenCalled();
  });
});
