import { afterEach, beforeEach, expect, it, vi } from "vitest";

/* fn-243 R4, fn-353: the page's renderer over a WebRenderer whose creation,
   the wood's programs compiled inside it, the test holds - resolving late,
   rejected for a named program, or ended by a lost device - so the pending
   and failure states are proved without a browser. */

function deferred() {
  let resolve!: () => void;
  let reject!: (error: unknown) => void;
  const promise = new Promise<void>((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}

const fake = vi.hoisted(() => ({
  compiled: undefined as Promise<void> | undefined,
  created: 0,
  builds: [] as string[],
}));

vi.mock("./render/telperion_render.js", () => ({
  default: async () => undefined,
  WebRenderer: {
    create: async () => {
      fake.created += 1;
      await fake.compiled;
      return {
        beginGrowth: (family: string) => {
          fake.builds.push(`grow ${family}`);
          let committed = 0;
          return { committed: () => committed, horizon: () => 8, free: () => undefined, grow: (age: number) => (committed = Math.ceil(age)) };
        },
        growTo: (handle: { grow(age: number): number }, age: number) => handle.grow(age),
        drawGrowth: (_handle: unknown, at: number) => JSON.stringify({ foliageInstances: at }),
        resize: () => undefined,
        dispose: () => undefined,
        free: () => undefined,
      };
    },
  },
}));

const { createRenderer } = await import("./render");
const { OREGON_WHITE_OAK } = await import("./presets");
const { wire } = await import("./wire");
const OAK = wire(OREGON_WHITE_OAK);
const canvas = { clientWidth: 4, clientHeight: 4, width: 0, height: 0 } as unknown as HTMLCanvasElement;
const flush = async () => { await new Promise(resolve => setTimeout(resolve, 0)); };

beforeEach(() => {
  vi.stubGlobal("window", { devicePixelRatio: 1 });
  vi.stubGlobal("ResizeObserver", class { observe() {} disconnect() {} });
  fake.compiled = Promise.resolve();
  fake.created = 0;
  fake.builds = [];
});
afterEach(() => { vi.unstubAllGlobals(); });

it("resolves only once the renderer exists with the wood's programs compiled, and has nothing to prepare", async () => {
  const programs = deferred();
  fake.compiled = programs.promise;
  let settled = false;
  const created = createRenderer(canvas).then(renderer => { settled = true; return renderer; });
  await flush();
  expect(fake.created).toBe(1);
  expect(settled).toBe(false);
  programs.resolve();
  const renderer = await created;
  expect(Object.keys(renderer)).not.toContain("programs");
  expect(fake.created).toBe(1);
});

it.each([
  "the browser refused the GPU program `rings`: internal: the shader failed to compile",
  "the GPU device was lost: Unknown: A valid external Instance reference no longer exists.",
])("a compile that fails (%s) rejects the renderer by name", async message => {
  const programs = deferred();
  fake.compiled = programs.promise;
  const created = createRenderer(canvas);
  programs.reject(new Error(message));
  await expect(created).rejects.toThrow(message);
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
  await createRenderer(watched);
  expect(writes).toEqual(["width=4", "height=4"]);
  notice();
  expect(writes).toEqual(["width=4", "height=4"]);
});

it("a growth begins on the CPU, then grows and draws through its handle (fn-353)", async () => {
  const renderer = await createRenderer(canvas);
  const growth = renderer.beginGrowth(OREGON_WHITE_OAK);
  expect(fake.builds).toEqual([`grow ${OAK}`]);
  expect(growth.growTo(2.5)).toBe(3);
  expect(growth.committed()).toBe(3);
  expect(growth.draw(2.5).foliageInstances).toBe(2.5);
});

it.each(["setTree", "beginGrowth"] as const)(
  "%s refuses a preset id by wire's message, before the renderer sees it",
  async method => {
    const renderer = await createRenderer(canvas);
    await expect(async () => renderer[method]("oregon-white-oak" as never)).rejects.toThrow("not a preset id: pass presetById(id)");
    expect(fake.builds).toEqual([]);
  },
);
