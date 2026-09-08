import {
  createRenderer,
  type FrameStats,
  type Renderer,
  type Submitted,
  type TimingReport,
  type View,
} from "../src/browser/render";

import { eyeOf, orbitOf, push, turn, type Orbit } from "./orbit";

/* ------------------------------------------------------------------ *
 * THE CLAY ROOM, ON THE RUST RENDERER
 *
 * The same contract the three.js stage had, over a canvas the Rust
 * renderer draws: hand it a tree, tell it which view, and it keeps
 * drawing. The panel owns the dials and this owns everything about
 * the picture - the frame loop, the pointer, and when the camera is
 * allowed to move.
 *
 * The camera rule is the old room's, unchanged, and it is the one
 * thing here worth stating twice: the camera moves when it is asked
 * to and at no other time. A camera that re-frames whenever the tree
 * changes size follows the height dial around the room while it is
 * being dragged, which reads as the room moving rather than the tree
 * growing. So the latch lives here rather than in the component: the
 * component outlives its own stage, and a latch held there would be
 * spent on a stage that no longer exists.
 * ------------------------------------------------------------------ */

export interface Stage {
  /** Replaces the tree with the one these parameters describe. Throws
   *  the generator's or the renderer's own message; the tree already on
   *  the canvas stays where it is. */
  setTree(family: string): Submitted;
  setView(view: View): void;
  /** Frames the tree that is on the canvas now. */
  frame(): void;
  /** Frames it only while this stage is still waiting to be framed - on
   *  its first tree, or on the next one after `frameNext`. */
  frameIfWaiting(): void;
  /** Asks for the next tree to arrive to be framed, for a caller that
   *  has just requested one and cannot frame it yet. */
  frameNext(): void;
  /** What the renderer drew on its last frame. */
  stats(): FrameStats;
  /** Runs the GPU timing session. The frame loop stands aside for it, so
   *  what is measured is the session's own frames and nothing else. */
  timing(): Promise<TimingReport>;
  dispose(): void;
}

export interface StageOptions {
  /** Where a failure inside the frame loop goes. The loop stops on one:
   *  a renderer that cannot draw will not draw sixty times a second. */
  onError(message: string): void;
}

/** How many notches one wheel event is worth. Browsers report pixels,
 *  lines or pages depending on the device; a notch is about a line. */
const WHEEL_NOTCH = 53;

export async function createStage(
  canvas: HTMLCanvasElement,
  options: StageOptions,
): Promise<Stage> {
  const renderer: Renderer = await createRenderer(canvas);

  let orbit: Orbit | null = null;
  let waitingToFrame = true;
  let measuring = false;
  let disposed = false;

  const look = (next: Orbit): void => {
    orbit = next;
    renderer.setCamera({ position: eyeOf(next), target: next.target });
  };

  const frame = (): void => {
    const pose = renderer.hero();
    if (pose === null) return;
    waitingToFrame = false;
    look(orbitOf(pose.position, pose.target));
  };

  /* Drawing is the loop's, always: the panel never calls `frame` on the
     renderer itself, so a dial move and a camera move cost the same one
     frame each. */
  let handle = requestAnimationFrame(function tick(): void {
    handle = requestAnimationFrame(tick);
    if (measuring) return;
    try {
      renderer.frame();
    } catch (error) {
      cancelAnimationFrame(handle);
      options.onError(String(error));
    }
  });

  let dragging: number | null = null;
  let last: [number, number] = [0, 0];
  const onPointerDown = (event: PointerEvent): void => {
    if (orbit === null) return;
    dragging = event.pointerId;
    last = [event.clientX, event.clientY];
    canvas.setPointerCapture(event.pointerId);
  };
  const onPointerMove = (event: PointerEvent): void => {
    if (dragging !== event.pointerId || orbit === null) return;
    look(turn(orbit, event.clientX - last[0], event.clientY - last[1]));
    last = [event.clientX, event.clientY];
  };
  const onPointerUp = (event: PointerEvent): void => {
    if (dragging !== event.pointerId) return;
    dragging = null;
    canvas.releasePointerCapture(event.pointerId);
  };
  const onWheel = (event: WheelEvent): void => {
    if (orbit === null) return;
    event.preventDefault();
    look(push(orbit, event.deltaY / WHEEL_NOTCH));
  };

  canvas.addEventListener("pointerdown", onPointerDown);
  canvas.addEventListener("pointermove", onPointerMove);
  canvas.addEventListener("pointerup", onPointerUp);
  canvas.addEventListener("pointercancel", onPointerUp);
  canvas.addEventListener("wheel", onWheel, { passive: false });

  return {
    setTree: (family) => renderer.setTree(family),
    setView: (view) => {
      renderer.setView(view);
      // A view is a different subject - one leaf at generated scale is
      // centimetres where the tree was metres - so it is framed rather
      // than looked at from where the whole tree was.
      frame();
    },
    frame,
    frameIfWaiting: () => {
      if (waitingToFrame) frame();
    },
    frameNext: () => {
      waitingToFrame = true;
    },
    stats: () => renderer.stats(),
    timing: async () => {
      measuring = true;
      try {
        return await renderer.timing();
      } finally {
        measuring = false;
      }
    },
    dispose: () => {
      if (disposed) return;
      disposed = true;
      cancelAnimationFrame(handle);
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointermove", onPointerMove);
      canvas.removeEventListener("pointerup", onPointerUp);
      canvas.removeEventListener("pointercancel", onPointerUp);
      canvas.removeEventListener("wheel", onWheel);
      renderer.dispose();
    },
  };
}
