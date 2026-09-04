import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_PARAMS } from "./params";
import { buildPlaceholderTree } from "./placeholder";

/* The panel's promise is that a seed change and a slider move produce a
   different tree without a reload. The browser is where that is judged;
   these are the parts of it a box with no GPU can still hold to
   account - geometry comes out, the seed decides it, and the height
   dial actually reaches the geometry. */

const material = new THREE.MeshStandardMaterial();

function box(seedOrParams: number | typeof DEFAULT_PARAMS): THREE.Box3 {
  const params =
    typeof seedOrParams === "number"
      ? { ...DEFAULT_PARAMS, seed: seedOrParams }
      : seedOrParams;
  const mesh = buildPlaceholderTree(params, material);
  const geometry = mesh.geometry;
  geometry.computeBoundingBox();
  return geometry.boundingBox!;
}

describe("buildPlaceholderTree", () => {
  it("produces geometry with vertices", () => {
    const mesh = buildPlaceholderTree(DEFAULT_PARAMS, material);
    expect(mesh.geometry.getAttribute("position").count).toBeGreaterThan(0);
  });

  it("is deterministic in the seed", () => {
    const a = box(7);
    const b = box(7);
    expect(a.min.toArray()).toEqual(b.min.toArray());
    expect(a.max.toArray()).toEqual(b.max.toArray());
  });

  it("a different seed grows a different tree", () => {
    expect(box(7).max.toArray()).not.toEqual(box(8).max.toArray());
  });

  it("the height dial reaches the geometry", () => {
    const short = box({ ...DEFAULT_PARAMS, height: 8 });
    const tall = box({ ...DEFAULT_PARAMS, height: 48 });
    expect(tall.max.y).toBeGreaterThan(short.max.y * 3);
  });
});
