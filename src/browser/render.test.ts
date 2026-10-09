import { afterEach, beforeEach, expect, it, vi } from "vitest";

/* fn-243 R4: the page's renderer over a WebRenderer whose program
   compilation the test holds - resolving late, rejected for a named
   program, or ended by a lost device - so the pending and failure states
   are proved without a browser. */

function deferred() {
  let resolve!: () => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<void>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

const fake = vi.hoisted(() => ({
  programs: undefined as Promise<void> | undefined,
  prepared: 0,
  builds: [] as string[],
}));

vi.mock("./render/telperion_render.js", () => ({
  default: async () => undefined,
  WebRenderer: {
    create: async () => ({
      preparePrograms: () => { fake.prepared += 1; return fake.programs; },
      setTreeGpu: async (family: string) => { fake.builds.push(family); return "{}"; },
      resize: () => undefined,
      dispose: () => undefined,
      free: () => undefined,
    }),
  },
}));

const { createRenderer } = await import("./render");
const canvas = { clientWidth: 4, clientHeight: 4, width: 0, height: 0 } as unknown as HTMLCanvasElement;
const flush = async () => { await new Promise(resolve => setTimeout(resolve, 0)); };

beforeEach(() => {
  vi.stubGlobal("window", { devicePixelRatio: 1 });
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
  fake.prepared = 0;
  fake.builds = [];
});
afterEach(() => { vi.unstubAllGlobals(); });

it("starts compiling at creation and holds a build until the programs resolve", async () => {
  const programs = deferred();
  fake.programs = programs.promise;
  const renderer = await createRenderer(canvas);
  expect(fake.prepared).toBe(1);
  const built = renderer.setTreeGpu("beech");
  await flush();
  expect(fake.builds).toEqual([]);
  programs.resolve();
  await built;
  await renderer.setTreeGpu("spruce");
  expect(fake.builds).toEqual(["beech", "spruce"]);
  expect(fake.prepared).toBe(1);
});

it.each([
  "the browser refused the GPU program `grow`: internal: the shader failed to compile",
  "the GPU device was lost: Unknown: A valid external Instance reference no longer exists.",
])("a preparation that fails (%s) rejects every build by name, and none starts", async message => {
  const programs = deferred();
  fake.programs = programs.promise;
  const renderer = await createRenderer(canvas);
  const built = renderer.setTreeGpu("beech");
  programs.reject(new Error(message));
  await expect(renderer.programs).rejects.toThrow(message);
  await expect(built).rejects.toThrow(message);
  await expect(renderer.setTreeGpu("spruce")).rejects.toThrow(message);
  expect(fake.builds).toEqual([]);
});

it("a resize notice that keeps the canvas's pixel size does not reset it (fn-246)", async () => {
  // Writing a canvas's width or height clears its picture even at the same
  // value, and the renderer draws only when something changed, so a reset
  // with no new size would leave the canvas blank.
  let notice = (): void => undefined;
  vi.stubGlobal("ResizeObserver", class {
    constructor(callback: () => void) { notice = callback; }
    observe() {}
    disconnect() {}
  });
  const writes: string[] = [];
  const size = { width: 0, height: 0 };
  const watched = {
    clientWidth: 4,
    clientHeight: 4,
    get width() { return size.width; },
    set width(v: number) { writes.push(`width=${v}`); size.width = v; },
    get height() { return size.height; },
    set height(v: number) { writes.push(`height=${v}`); size.height = v; },
  } as unknown as HTMLCanvasElement;
  fake.programs = Promise.resolve();
  await createRenderer(watched);
  expect(writes).toEqual(["width=4", "height=4"]);
  notice();
  expect(writes).toEqual(["width=4", "height=4"]);
});
