import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_BIAS } from "@/lib/grower/torsion";

import { DEFAULT_PARAMS, type GrowerParams } from "./params";
import {
  buildSkeletonLines,
  skeletonGeometry,
  toSkeletonParams,
} from "./skeleton-view";

/* The panel's promise is that a seed change and a slider move produce a
   different tree without a reload. The browser is where that is judged;
   these are the parts of it a box with no GPU can still hold to
   account - lines come out, the seed decides them, and the dials that
   claim to reach the generator do. */

const clay = {
  surface: new THREE.MeshStandardMaterial(),
  line: new THREE.LineBasicMaterial(),
};

function lines(overrides: Partial<GrowerParams> = {}): THREE.LineSegments {
  return buildSkeletonLines({ ...DEFAULT_PARAMS, ...overrides }, clay);
}

function positions(mesh: THREE.LineSegments): Float32Array {
  return mesh.geometry.getAttribute("position").array as Float32Array;
}

describe("toSkeletonParams", () => {
  it("hands the bias dials over under the library's own names", () => {
    const mapped = toSkeletonParams({
      ...DEFAULT_PARAMS,
      gravitropism: 0.9,
      lean: 0.21,
      writheAmplitude: 0.13,
      writheWavelength: 0.31,
      spiralRate: 2.5,
    });
    expect(mapped.bias).toEqual({
      gravitropism: 0.9,
      lean: 0.21,
      writheAmplitude: 0.13,
      writheWavelength: 0.31,
      spiralRate: 2.5,
    });
  });

  it("defaults every bias dial to the library's own default", () => {
    // The panel is not allowed a second opinion about what a tree looks
    // like out of the box; fn-11.7's presets are the library's business.
    expect(toSkeletonParams(DEFAULT_PARAMS).bias).toEqual(DEFAULT_BIAS);
  });

  it("passes the envelope dials straight through", () => {
    const mapped = toSkeletonParams({
      ...DEFAULT_PARAMS,
      seed: 9,
      height: 31,
      spread: 0.42,
    });
    expect(mapped.seed).toBe(9);
    expect(mapped.envelope.height).toBe(31);
    expect(mapped.envelope.spread).toBe(0.42);
  });

  it("turns density into an attractor count", () => {
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, density: 0 }).attractors).toBe(
      250,
    );
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, density: 1 }).attractors).toBe(
      1600,
    );
    const middle = toSkeletonParams({ ...DEFAULT_PARAMS, density: 0.5 });
    expect(middle.attractors).toBeGreaterThan(250);
    expect(middle.attractors).toBeLessThan(1600);
  });
});

describe("skeletonGeometry", () => {
  it("draws one segment per branch and nothing for the root", () => {
    const geometry = skeletonGeometry({
      nodes: [
        { position: new THREE.Vector3(0, 0, 0), parent: -1 },
        { position: new THREE.Vector3(0, 1, 0), parent: 0 },
        { position: new THREE.Vector3(1, 2, 0), parent: 1 },
      ],
    });
    expect(geometry.getAttribute("position").count).toBe(4);
    expect([...(geometry.getAttribute("position").array as Float32Array)]).toEqual(
      [0, 0, 0, 0, 1, 0, 0, 1, 0, 1, 2, 0],
    );
  });

  it("draws nothing for a skeleton that never grew", () => {
    const geometry = skeletonGeometry({
      nodes: [{ position: new THREE.Vector3(0, 0, 0), parent: -1 }],
    });
    expect(geometry.getAttribute("position").count).toBe(0);
  });
});

describe("buildSkeletonLines", () => {
  it("produces line segments the stage can draw", () => {
    const mesh = lines();
    expect(mesh).toBeInstanceOf(THREE.LineSegments);
    expect(mesh.material).toBe(clay.line);
    expect(positions(mesh).length).toBeGreaterThan(0);
    // Two endpoints per segment, three floats each.
    expect(positions(mesh).length % 6).toBe(0);
  });

  it("is deterministic in the seed", () => {
    expect([...positions(lines({ seed: 7 }))]).toEqual([
      ...positions(lines({ seed: 7 })),
    ]);
  });

  it("a different seed grows a different tree", () => {
    expect([...positions(lines({ seed: 7 }))]).not.toEqual([
      ...positions(lines({ seed: 8 })),
    ]);
  });

  it("the height dial reaches the geometry", () => {
    const short = lines({ height: 8 });
    const tall = lines({ height: 48 });
    short.geometry.computeBoundingBox();
    tall.geometry.computeBoundingBox();
    expect(tall.geometry.boundingBox!.max.y).toBeGreaterThan(
      short.geometry.boundingBox!.max.y * 3,
    );
  });

  it("the spread dial reaches the silhouette", () => {
    const widest = (spread: number): number => {
      const geometry = lines({ spread }).geometry;
      geometry.computeBoundingBox();
      const box = geometry.boundingBox!;
      return Math.max(box.max.x, box.max.z, -box.min.x, -box.min.z);
    };
    expect(widest(1.2)).toBeGreaterThan(widest(0.2) * 3);
  });

  it("every bias dial reaches the geometry", () => {
    /* The panel promised for two tasks that torsion would land in
       fn-11.3 and reached nothing in the meantime. Five dials now, one
       per term of the growth bias field, and each one has to move the
       tree on its own - a dial that renders and does nothing is worse
       than no dial. */
    const straight = {
      gravitropism: 0,
      lean: 0,
      writheAmplitude: 0,
      spiralRate: 0,
    };
    const flat = [...positions(lines(straight))];
    for (const dial of [
      { gravitropism: 0.9 },
      { lean: 0.4 },
      { writheAmplitude: 0.15, writheWavelength: 0.5 },
      { writheAmplitude: 0.15, writheWavelength: 0.1 },
      { writheAmplitude: 0.15, spiralRate: 4 },
    ]) {
      expect([...positions(lines({ ...straight, ...dial }))]).not.toEqual(flat);
    }
  });

  it("the density dial reaches the branch count", () => {
    expect(positions(lines({ density: 1 })).length).toBeGreaterThan(
      positions(lines({ density: 0 })).length * 2,
    );
  });
});
