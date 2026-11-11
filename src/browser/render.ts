import { WebRenderer, type WebGrowth } from "./render/telperion_render.js";
import { loadModule } from "./module";
import type { Family } from "./presets";
import { wire } from "./wire";

/* ------------------------------------------------------------------ *
 * THE RENDERER, AS A PAGE HOLDS IT
 *
 * The Rust renderer draws the canvas; this owns the two things that
 * are the page's own business and not the renderer's - how many
 * device pixels the canvas is asked for, and when that number
 * changes. Everything else is a call straight through.
 *
 * No geometry crosses this boundary. A family goes in as the one wire
 * text every entry hands the cores (`wire`), and counts come back as text; the tree itself is generated and uploaded
 * inside the module's own memory, so there is no view over wasm memory
 * here to detach when that memory grows.
 * ------------------------------------------------------------------ */

/** The cap the clay room has always drawn at. Above two device pixels
 *  per CSS pixel the fill cost doubles again for a difference the owner
 *  cannot see on a tree in flat clay. */
const MAX_PIXEL_RATIO = 2;

/** Metres, Y up, right-handed - the renderer's own frame. */
export type Point = readonly [number, number, number];

export interface Submitted {
  woodVertices: number;
  woodTriangles: number;
  foliageInstances: number;
  bounds: { min: Point; max: Point };
}

export interface FrameStats {
  drawCalls: number;
  /** The room's and the leaves' triangles: the wood's are the device's own
   *  count, which `woodCounted: false` marks as left out (fn-208). */
  triangles: number;
  woodCounted: boolean;
  instances: number;
}

export interface Pose {
  position: Point;
  target: Point;
}

/** The timing session's record, as the Rust report writes it, field for
 *  field. The GPU percentiles exist only on a valid verdict, so nothing
 *  in an invalid record can be read as a number that passed. The wall
 *  clock is the page's own and not the GPU's, so an orbit reports it even
 *  where there was no GPU clock to read. */
export interface TimingReport {
  adapter: string;
  driver: string;
  backend: string;
  conditioning: number;
  warmup: number;
  measured: number;
  samples: number;
  multisample: number;
  caster_triangles: number;
  caster_instances: number;
  verdict: "valid" | "unavailable" | "disjoint" | "contended";
  reason?: string;
  p50_ms?: number;
  p95_ms?: number;
  /** The selection compute pass that decides what the vegetation pass draws. */
  selection_p50_ms?: number;
  selection_p95_ms?: number;
  shadow_p50_ms?: number;
  shadow_p95_ms?: number;
  /** Measured frames that reused the sun's map: each costs zero in the
   *  shadow numbers above rather than going unmeasured (fn-246). */
  shadow_cached?: number;
  /** Three passes added frame by frame and then ranked: a frame's own cost. */
  total_p50_ms?: number;
  total_p95_ms?: number;
  /** What each level drew, coarsest first, and last the bucket of leaves
   *  no level drew - which approximated nothing, so it has no deviation.
   *  Read back only where a session can read the counters. */
  levels?: { deviation_m: number | null; instances_p50: number }[];
  /** Frame to frame on the page's own animation clock, over an orbit. */
  wall_frames?: number;
  wall_p50_ms?: number;
  wall_p95_ms?: number;
  wall_max_ms?: number;
}

export type View = "whole" | "bare" | "leaf" | "clay";

/** The sun, the sky and the ground, as the renderer states them. The row is
 *  defined in Rust and read back out of the renderer, so nothing on this side
 *  keeps a second copy of its fields or of their default values: a field the
 *  row grows appears in the panel with no line here. */
export type SceneRow = Record<string, number>;

/** Devices created minus devices disposed, on the window, for the soak
 *  test to read. React's development double-mount builds one renderer
 *  and disposes it before the live one, so "one canvas, one device" is
 *  a claim only a counter that survives both mounts can check. */
declare global {
  interface Window {
    telperionLiveDevices?: number;
  }
}

function countDevice(step: 1 | -1): void {
  window.telperionLiveDevices = (window.telperionLiveDevices ?? 0) + step;
}

/** A tree growing on the CPU, on the page's thread (fn-270 R6, R7;
 *  fn-353): begun for its family's age, grown on to any age within it,
 *  and drawn at any age in its last year. Each call runs to its end
 *  before it returns; the tree on the canvas stays until a draw replaces
 *  it. */
export interface Growth {
  /** The whole years grown. */
  committed(): number;
  /** The most years it grows: the family's age. */
  horizon(): number;
  /** Grows on to `age` years and returns the whole years grown. An age
   *  past the horizon, a year or more back, or not finite throws by name
   *  before anything moves; a failure spends the tree. */
  growTo(age: number): number;
  /** Puts the tree drawn at `at`, within its last year, on the canvas in
   *  place of the tree there. */
  draw(at: number): Submitted;
  /** Lets the growth state go. */
  dispose(): void;
}

/** One canvas drawn by the Rust renderer, sized to the display and
 *  disposed with the page element it belongs to. */
export interface Renderer {
  setTree(family: Family): Submitted;
  /** The wood's surfacing programs, compiled by the browser in the
   *  background from the moment the renderer exists; the wood is drawn
   *  once they have settled. Rejects with the program the browser
   *  refused, or the device lost meanwhile, by name. */
  programs: Promise<void>;
  /** The family's tree before its first year, growing on the CPU
   *  (`Growth`). Throws the core's words for a family it cannot grow. */
  beginGrowth(family: Family): Growth;
  setView(view: View): void;
  /** The sun, sky and ground the next frame is drawn under. */
  scene(): SceneRow;
  /** Stands the sun somewhere else, on top of the row already set. Throws the
   *  renderer's own words for a value it will not have, and the sky the
   *  renderer is drawing under stays exactly where it was. */
  setScene(row: SceneRow): void;
  /** Puts the camera at the judging pose for what is on the canvas, and
   *  reports where that left it. Null before a tree has been submitted. */
  hero(): Pose | null;
  setCamera(pose: Pose): void;
  frame(): void;
  stats(): FrameStats;
  timing(): Promise<TimingReport>;
  /** The same session while the camera makes one full turn about the pose
   *  it stands at, ending with ten seconds of frames drawn the way this
   *  loop draws them - which is where the record's wall numbers come from. */
  orbit(): Promise<TimingReport>;
  dispose(): void;
}

/** Loads the module once per page and puts a renderer on this canvas.
 *  Rejects with the renderer's own words when there is no WebGPU, only a
 *  software adapter, or the device is refused. */
export async function createRenderer(canvas: HTMLCanvasElement): Promise<Renderer> {
  await loadModule();

  /* Writing a canvas's size clears its picture even at the same value, and
     the renderer redraws only on a change (fn-246), so an unchanged size is
     never written. */
  const resize = (): void => {
    const ratio = Math.min(window.devicePixelRatio, MAX_PIXEL_RATIO);
    const width = Math.max(1, Math.round((canvas.clientWidth || 1) * ratio));
    const height = Math.max(1, Math.round((canvas.clientHeight || 1) * ratio));
    if (canvas.width !== width) canvas.width = width;
    if (canvas.height !== height) canvas.height = height;
  };
  resize();

  const renderer = (await WebRenderer.create(canvas)) as WebRenderer;
  countDevice(1);
  let disposed = false;
  /* Compiled now, before any tree is asked for: a synchronous compile on
     the first build held Chrome's GPU process until its watchdog ended it
     (fn-243). */
  const programs = (renderer.preparePrograms() as Promise<unknown>).then(() => undefined);
  programs.catch(() => undefined);

  const observer = new ResizeObserver(() => {
    if (disposed) return;
    resize();
    renderer.resize(canvas.width, canvas.height);
  });
  observer.observe(canvas);

  return {
    setTree: (family) => JSON.parse(renderer.setTree(wire(family))) as Submitted,
    programs,
    beginGrowth: (family) => {
      const handle: WebGrowth = renderer.beginGrowth(wire(family));
      return {
        committed: () => handle.committed(),
        horizon: () => handle.horizon(),
        growTo: (age) => renderer.growTo(handle, age),
        draw: (at) => JSON.parse(renderer.drawGrowth(handle, at)) as Submitted,
        dispose: () => handle.free(),
      };
    },
    setView: (view) => renderer.setView(view),
    scene: () => JSON.parse(renderer.scene()) as SceneRow,
    setScene: (row) => renderer.setScene(JSON.stringify(row)),
    hero: () => JSON.parse(renderer.heroCamera()) as Pose | null,
    setCamera: ({ position, target }) =>
      renderer.setCamera(position[0], position[1], position[2], target[0], target[1], target[2]),
    frame: () => renderer.frame(),
    stats: () => JSON.parse(renderer.stats()) as FrameStats,
    timing: async () => JSON.parse((await renderer.timing()) as string) as TimingReport,
    orbit: async () => JSON.parse((await renderer.orbit()) as string) as TimingReport,
    dispose: () => {
      if (disposed) return;
      disposed = true;
      observer.disconnect();
      renderer.dispose();
      renderer.free();
      countDevice(-1);
    },
  };
}
