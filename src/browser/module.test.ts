import { beforeEach, expect, it, vi } from "vitest";

/* fn-297 R1: the renderer's wasm-bindgen module finalises its instance
   after an await. Two renderers starting it together must instantiate it
   once, and a refused start must leave the next call free to try again. */

const glue = vi.hoisted(() => ({ inits: 0, refuse: 0 }));

vi.mock("./render/telperion_render.js", () => ({
  default: async () => {
    glue.inits += 1;
    await new Promise(resolve => setTimeout(resolve, 0));
    if (glue.refuse > 0) {
      glue.refuse -= 1;
      throw Error("the module was refused");
    }
  },
  WebRenderer: {
    create: async () => ({ resize: () => undefined, dispose: () => undefined, free: () => undefined }),
  },
}));

beforeEach(() => {
  vi.resetModules();
  vi.stubGlobal("window", { devicePixelRatio: 1 });
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
  vi.stubGlobal("navigator", { gpu: {} });
  glue.inits = 0;
  glue.refuse = 0;
});

const canvas = { clientWidth: 4, clientHeight: 4, width: 0, height: 0 } as unknown as HTMLCanvasElement;

it("instantiates once when two renderers start together", async () => {
  const { createRenderer } = await import("./render");
  await Promise.all([createRenderer(canvas), createRenderer(canvas)]);
  expect(glue.inits).toBe(1);
});

it("tries again after a refused initialisation", async () => {
  const { createRenderer } = await import("./render");
  glue.refuse = 1;
  await expect(createRenderer(canvas)).rejects.toThrow("refused");
  await createRenderer(canvas);
  await createRenderer(canvas);
  expect(glue.inits).toBe(2);
});
