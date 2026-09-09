import init, { WebRenderer } from "./render/telperion_render.js";
import wasmUrl from "./render/telperion_render_bg.wasm?url";

/* ------------------------------------------------------------------ *
 * THE RENDERER, AS A PAGE HOLDS IT
 *
 * The Rust renderer draws the canvas; this owns the two things that
 * are the page's own business and not the renderer's - how many
 * device pixels the canvas is asked for, and when that number
 * changes. Everything else is a call straight through.
 *
 * No geometry crosses this boundary. Parameters go in as text and
 * counts come back as text; the tree itself is generated and uploaded
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
  triangles: number;
  instances: number;
}

export interface Pose {
  position: Point;
  target: Point;
}

/** The timing session's record, as the Rust report writes it. The
 *  percentiles exist only on a valid verdict, so nothing in an invalid
 *  record can be read as a number that passed. */
export interface TimingReport {
  adapter: string;
  driver: string;
  backend: string;
  conditioning: number;
  warmup: number;
  measured: number;
  samples: number;
  verdict: "valid" | "unavailable" | "disjoint" | "contended";
  reason?: string;
  p50_ms?: number;
  p95_ms?: number;
}

export type View = "whole" | "bare" | "leaf";

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

let loading: Promise<void> | undefined;

/** One canvas drawn by the Rust renderer, sized to the display and
 *  disposed with the page element it belongs to. */
export interface Renderer {
  setTree(family: string): Submitted;
  setView(view: View): void;
  /** Puts the camera at the judging pose for what is on the canvas, and
   *  reports where that left it. Null before a tree has been submitted. */
  hero(): Pose | null;
  setCamera(pose: Pose): void;
  frame(): void;
  stats(): FrameStats;
  timing(): Promise<TimingReport>;
  dispose(): void;
}

/** Loads the module once per page and puts a renderer on this canvas.
 *  Rejects with the renderer's own words when there is no WebGPU, only a
 *  software adapter, or the device is refused. */
export async function createRenderer(canvas: HTMLCanvasElement): Promise<Renderer> {
  loading ??= init({ module_or_path: wasmUrl }).then(() => undefined);
  await loading;

  const resize = (): void => {
    const ratio = Math.min(window.devicePixelRatio, MAX_PIXEL_RATIO);
    canvas.width = Math.max(1, Math.round((canvas.clientWidth || 1) * ratio));
    canvas.height = Math.max(1, Math.round((canvas.clientHeight || 1) * ratio));
  };
  resize();

  const renderer = (await WebRenderer.create(canvas)) as WebRenderer;
  countDevice(1);
  let disposed = false;

  const observer = new ResizeObserver(() => {
    if (disposed) return;
    resize();
    renderer.resize(canvas.width, canvas.height);
  });
  observer.observe(canvas);

  return {
    setTree: (family) => JSON.parse(renderer.setTree(family)) as Submitted,
    setView: (view) => renderer.setView(view),
    hero: () => JSON.parse(renderer.heroCamera()) as Pose | null,
    setCamera: ({ position, target }) =>
      renderer.setCamera(position[0], position[1], position[2], target[0], target[1], target[2]),
    frame: () => renderer.frame(),
    stats: () => JSON.parse(renderer.stats()) as FrameStats,
    timing: async () => JSON.parse((await renderer.timing()) as string) as TimingReport,
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
