import { readFileSync } from "node:fs";

import { describe, expect, it } from "vitest";

import { solveRoom } from "./stage";

/* ------------------------------------------------------------------ *
 * THE ROOM, AND THE LATCH THAT PLACES THE CAMERA IN IT
 *
 * The stage itself needs a GPU and a canvas, and this box has neither,
 * so what is testable is the part that is arithmetic: the room's own
 * numbers. That is not a consolation prize - the room's numbers are
 * where its one invariant lives. The room may never move the camera,
 * and `OrbitControls` clamps the orbit radius to `minDistance` and
 * `maxDistance` on every `update()`, so writing either on the wrong
 * side of where the camera stands moves it.
 *
 * The bug this closes was invisible from the growing direction, which
 * is the only direction anyone had tried: a room that only ever grows
 * never crosses the camera. So every assertion below that matters runs
 * the subject DOWN.
 * ------------------------------------------------------------------ */

const NEAR = 0.1;

/** The height dial's own range, and the subject's longest side at each
 *  of those heights. A 400 m tree measures 228.9 x 401.4 x 221.3, so
 *  the span runs a little over the height; the crown is what is being
 *  fitted, not the dial. */
const SPANS = [401, 150, 66, 60, 33, 5] as const;

describe("solveRoom", () => {
  it("never pulls the camera in when the subject shrinks", () => {
    /* The reported failure, replayed: grow to 400 m, scroll out to
       2408 m, then drag the height down. At 66 m the reach hits its
       400 m floor, and a `maxDistance` of 400 written under a camera
       2408 m out is a 2000 m jump on the very next frame - once for
       every intermediate value the slider passes through. */
    const radius = 2408;
    for (const span of SPANS) {
      const room = solveRoom(span, radius, NEAR);
      expect(room.maxDistance, `${span} m subject`).toBeGreaterThanOrEqual(
        radius,
      );
      expect(room.minDistance, `${span} m subject`).toBeLessThanOrEqual(radius);
    }
  });

  it("never pushes the camera out when it is right in on the subject", () => {
    // The other side of the same rule: an eye on a twig, a metre out,
    // and then the subject grows under it.
    const radius = 0.4;
    for (const span of [...SPANS].reverse()) {
      const room = solveRoom(span, radius, NEAR);
      expect(room.minDistance, `${span} m subject`).toBeLessThanOrEqual(radius);
      expect(room.maxDistance, `${span} m subject`).toBeGreaterThanOrEqual(
        radius,
      );
    }
  });

  it("sees the whole ground disc from wherever the camera stands", () => {
    // The disc reaches `reach` from the origin and the camera is
    // `radius` out from a target inside it, so the far plane has to
    // clear their sum or the world ends in front of the camera - which
    // is what a far plane sized off the room alone does to a camera
    // the shrinking room has left far outside it.
    for (const span of SPANS) {
      for (const radius of [0.4, 60, 2408]) {
        const room = solveRoom(span, radius, NEAR);
        expect(
          room.far,
          `${span} m subject, camera ${radius} m out`,
        ).toBeGreaterThanOrEqual(radius + room.reach);
      }
    }
  });

  it("grows the room to hold a bigger subject", () => {
    // The reason the room is refitted at all: a tree grown to 400 m
    // must not still be living in the room its 24 m predecessor was
    // framed in, unable to be zoomed out to.
    const small = solveRoom(33, 60, NEAR);
    const large = solveRoom(401, 60, NEAR);
    expect(large.reach).toBeGreaterThan(small.reach * 5);
    expect(large.maxDistance).toBeGreaterThan(401);
    expect(large.far).toBeGreaterThan(large.reach);
  });

  it("holds a subject that measured nothing at all", () => {
    /* A build that yields no renderable geometry measures a zero span,
       and a room solved from it used to set `minDistance` to zero -
       which lets the orbit radius collapse onto its own target, with
       no scroll direction that gets back out of it. */
    const room = solveRoom(0, 12, NEAR);
    expect(room.minDistance).toBeGreaterThanOrEqual(NEAR);
    expect(room.maxDistance).toBeGreaterThan(0);
    expect(room.far).toBeGreaterThan(0);
  });

  it("never lets the orbit inside the near plane", () => {
    // Nothing is visible closer than the near plane, so an orbit floor
    // below it only buys a way to lose the subject.
    for (const span of SPANS) {
      expect(solveRoom(span, 1000, NEAR).minDistance).toBeGreaterThanOrEqual(
        NEAR,
      );
    }
  });
});

/* The framing latch cannot be exercised from here: it belongs to a
   stage, a stage needs a WebGL context, and the double-mount that
   spends it needs React and a DOM. Both are out of this runner's reach
   by design - "pure units only", vitest.config.ts. So what is asserted
   is the structure that makes the bug unreachable, which is the thing
   that actually went wrong: the latch was a component-lifetime ref
   while the stage was not, so React's development double-mount - every
   load of the dev route, since the route exists in no other mode -
   framed stage A, disposed it, built stage B and found the latch
   already spent. B's camera never moved, and `controls.update()`
   clamped its zero radius up to `minDistance` with phi at 0: twelve
   centimetres above the root, looking straight down.

   Tie the latch to the stage and the failure has nowhere to live. */
describe("the framing latch belongs to the stage", () => {
  const source = (file: string): string =>
    readFileSync(new URL(file, import.meta.url), "utf8");

  it("is declared inside createStage, so each stage has its own", () => {
    const stage = source("./stage.ts");
    const declaration = stage.indexOf("let waitingToFrame");
    const factory = stage.indexOf("export function createStage");
    expect(declaration, "stage.ts declares no framing latch").toBeGreaterThan(
      -1,
    );
    expect(
      declaration,
      "the framing latch is module-scoped: every stage in the process would share one",
    ).toBeGreaterThan(factory);
    expect(stage.split("let waitingToFrame").length - 1).toBe(1);
  });

  it("is never held by the component that builds the stage", () => {
    const component = source("./GrowerDev.tsx");
    expect(
      component,
      "the component asks the stage to frame; it does not decide whether to",
    ).toContain("frameIfWaiting");
    expect(
      component,
      "a boolean ref in the component outlives the stage it was latched for",
    ).not.toMatch(/useRef(<boolean.*?>)?\((false|true)\)/);
  });
});
