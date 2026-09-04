import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { transportFrames } from "@/lib/grower/mesh/frames";

/* The frame is what decides where the cross section's angle zero sits,
   and every visible failure of a swept surface that is not a gap is a
   failure of the frame: a ring that spins between one node and the
   next shows as a twist in the skin that no parameter asked for. */

/** An S-curve in the xz plane, which passes through an inflection at
 *  its middle: the case that flips a Frenet frame and the reason this
 *  module exists. */
function sCurve(count: number): THREE.Vector3[] {
  return Array.from({ length: count }, (unused, i) => {
    const t = (i / (count - 1)) * Math.PI * 2;
    return new THREE.Vector3(Math.sin(t) * 2, i * 0.5, 0);
  });
}

describe("transportFrames", () => {
  it("returns an orthonormal right-handed basis at every point", () => {
    for (const frame of transportFrames(sCurve(24))) {
      expect(frame.tangent.length()).toBeCloseTo(1, 12);
      expect(frame.normal.length()).toBeCloseTo(1, 12);
      expect(frame.binormal.length()).toBeCloseTo(1, 12);
      expect(frame.tangent.dot(frame.normal)).toBeCloseTo(0, 12);
      expect(frame.tangent.dot(frame.binormal)).toBeCloseTo(0, 12);
      expect(
        frame.tangent.clone().cross(frame.normal).dot(frame.binormal),
      ).toBeCloseTo(1, 12);
    }
  });

  it("carries the frame through an inflection without flipping it", () => {
    /* The whole claim, measured: the frame never turns further between
       two points than the path itself does. A Frenet frame fails this
       by 180 degrees at the inflection in the middle of this curve, and
       that flip is a visible twist artifact in the surface. */
    const frames = transportFrames(sCurve(48));
    let worstPath = 0;
    let worstFrame = 0;
    for (let i = 1; i < frames.length; i += 1) {
      worstPath = Math.max(
        worstPath,
        frames[i - 1].tangent.angleTo(frames[i].tangent),
      );
      worstFrame = Math.max(
        worstFrame,
        frames[i - 1].normal.angleTo(frames[i].normal),
      );
    }
    expect(worstPath).toBeGreaterThan(0.05);
    expect(worstFrame).toBeLessThanOrEqual(worstPath + 1e-9);
  });

  it("has no run to frame with fewer than two points", () => {
    expect(transportFrames([])).toEqual([]);
    expect(transportFrames([new THREE.Vector3()])).toEqual([]);
  });

  it("survives a repeated point rather than emitting NaN", () => {
    // Colonization can hand back a step of zero length; a frame built
    // by normalising it would put NaN into every vertex downstream.
    const frames = transportFrames([
      new THREE.Vector3(0, 0, 0),
      new THREE.Vector3(0, 1, 0),
      new THREE.Vector3(0, 1, 0),
      new THREE.Vector3(0, 2, 0),
    ]);
    for (const frame of frames) {
      for (const axis of [frame.tangent, frame.normal, frame.binormal]) {
        expect(Number.isFinite(axis.x + axis.y + axis.z)).toBe(true);
      }
    }
  });
});
