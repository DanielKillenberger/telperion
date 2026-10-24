import { beforeEach, expect, it, vi } from "vitest";

/* fn-297 R1: the renderer and the field entry share one wasm-bindgen
   module, whose init finalises its instance after an await. Two entries
   starting it together must instantiate it once, and a refused start must
   leave the next call free to try again. */

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
    create: async () => ({ preparePrograms: async () => undefined, resize: () => undefined, dispose: () => undefined, free: () => undefined }),
  },
  WebField: { prepare: async () => undefined },
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
const source = new Uint8Array(8);

it("instantiates once when the renderer and the field start together", async () => {
  const { createRenderer } = await import("./render");
  const { prepareField } = await import("../field/index");
  await Promise.all([createRenderer(canvas), prepareField(source), createRenderer(canvas)]);
  expect(glue.inits).toBe(1);
});

it("tries again after a refused initialisation, from either entry", async () => {
  const { createRenderer } = await import("./render");
  const { prepareField } = await import("../field/index");
  glue.refuse = 1;
  await expect(createRenderer(canvas)).rejects.toThrow("refused");
  await createRenderer(canvas);
  await prepareField(source);
  expect(glue.inits).toBe(2);
});
