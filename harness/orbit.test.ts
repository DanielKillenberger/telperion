import { describe, expect, it } from "vitest";

import { eyeOf, orbitOf, push, turn, type Point } from "./orbit";

/* The camera moves when it is asked to and at no other time, and it
   moves where it was asked. Neither claim needs a GPU: an orbit is
   three numbers and the arithmetic that keeps them inside the room. */

const TARGET: Point = [0, 11, 0];
const EYE: Point = [12, 18, 20];

function distance(a: Point, b: Point): number {
  return Math.hypot(a[0] - b[0], a[1] - b[1], a[2] - b[2]);
}

describe("orbitOf", () => {
  it("stands exactly where the pose it was read from stood", () => {
    const eye = eyeOf(orbitOf(EYE, TARGET));
    for (const axis of [0, 1, 2]) expect(eye[axis]).toBeCloseTo(EYE[axis], 9);
  });

  it("takes its travel limits from the framed distance", () => {
    // Every subject is orbited in proportion to its own size: a
    // sapling and a 400 m Telperion need the same room and wildly
    // different metres.
    const orbit = orbitOf(EYE, TARGET);
    expect(orbit.closest).toBeCloseTo(orbit.distance / 16, 9);
    expect(orbit.furthest).toBeCloseTo(orbit.distance * 8, 9);
  });

  it("lifts an eye that was handed to it below the horizon", () => {
    // A pose solved for a subject centred well above the ground can
    // put the eye under it; the room's floor is where it lands.
    const orbit = orbitOf([10, 0, 0], [0, 10, 0]);
    expect(orbit.elevation).toBeGreaterThan(0);
    expect(eyeOf(orbit)[1]).toBeGreaterThan(10);
  });
});

describe("turn", () => {
  it("walks round the subject without changing how far off it stands", () => {
    const orbit = orbitOf(EYE, TARGET);
    const turned = turn(orbit, 200, 0);
    expect(distance(eyeOf(turned), TARGET)).toBeCloseTo(orbit.distance, 9);
    expect(turned.yaw).not.toBeCloseTo(orbit.yaw, 6);
  });

  it("comes back to where it started after a full circuit", () => {
    const orbit = orbitOf(EYE, TARGET);
    const round = turn(orbit, 800, 0);
    const eye = eyeOf(round);
    for (const axis of [0, 1, 2]) expect(eye[axis]).toBeCloseTo(EYE[axis], 6);
  });

  it("never lets the eye under the ground or onto the pole", () => {
    let orbit = orbitOf(EYE, TARGET);
    for (const drag of [-4000, 4000, -9999, 9999]) {
      orbit = turn(orbit, 0, drag);
      expect(eyeOf(orbit)[1]).toBeGreaterThan(TARGET[1]);
      // At the pole the up direction stops being a direction and the
      // picture rolls; the margin is what keeps a horizontal offset.
      expect(Math.hypot(eyeOf(orbit)[0], eyeOf(orbit)[2])).toBeGreaterThan(0);
    }
  });
});

describe("push", () => {
  it("moves the eye along the line it was already looking down", () => {
    const orbit = orbitOf(EYE, TARGET);
    const pushed = push(orbit, 3);
    expect(pushed.distance).toBeGreaterThan(orbit.distance);
    expect(pushed.yaw).toBe(orbit.yaw);
    expect(pushed.elevation).toBe(orbit.elevation);
  });

  it("stops at the limits rather than passing through the subject", () => {
    const orbit = orbitOf(EYE, TARGET);
    expect(push(orbit, -400).distance).toBeCloseTo(orbit.closest, 9);
    expect(push(orbit, 400).distance).toBeCloseTo(orbit.furthest, 9);
  });

  it("is reversible, so a wheel scrolled back lands where it left", () => {
    const orbit = orbitOf(EYE, TARGET);
    expect(push(push(orbit, 5), -5).distance).toBeCloseTo(orbit.distance, 9);
  });
});
