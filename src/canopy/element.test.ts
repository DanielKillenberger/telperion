import * as THREE from "three";
import { describe, expect, it } from "vitest";

import {
  buildElement,
  DEFAULT_ELEMENT,
  type ElementMesh,
  type ElementParams,
} from "./element";

/* ------------------------------------------------------------------ *
 * There is no GPU on this box, so nothing here claims the leaf looks
 * like a leaf. What can be held to account is the two claims the
 * canopy is built on top of.
 *
 * The first is the local frame, because placement is being written
 * against it in parallel and never reads a vertex: petiole at the
 * origin, blade along +Y, face toward +Z. If that drifts, every
 * transform placement emits is wrong by the same amount and nothing
 * downstream can tell.
 *
 * The second is the fit. "Fitted to the silhouette rather than a
 * bounding quad" is a claim about fill, so it is checked as one -
 * against the drawn triangles' own projected area, next to the quad
 * the same parameters produce, rather than against a bounding box. A
 * bounding box is the thing this element exists not to be, so a test
 * that measured one would agree with the failure it is supposed to
 * catch. For the same reason the containment checks below run over
 * every vertex, never over a centroid.
 * ------------------------------------------------------------------ */

function element(params: Partial<ElementParams> = {}): ElementMesh {
  return buildElement({ ...DEFAULT_ELEMENT, ...params });
}

function vertex(mesh: ElementMesh, index: number): THREE.Vector3 {
  return new THREE.Vector3(
    mesh.positions[index * 3],
    mesh.positions[index * 3 + 1],
    mesh.positions[index * 3 + 2],
  );
}

function corners(mesh: ElementMesh, index: number): THREE.Vector3[] {
  return [0, 1, 2].map((k) => vertex(mesh, mesh.indices[index * 3 + k]));
}

/** Twice the triangle's area vector: the cross product of two of its
 *  edges, unnormalized. Its z is twice the signed area of the
 *  triangle's shadow on the xy plane, so its sign is the winding seen
 *  from +Z and its length is the area the fragment shader pays for. */
function areaVector(mesh: ElementMesh, index: number): THREE.Vector3 {
  const [a, b, c] = corners(mesh, index);
  return new THREE.Vector3()
    .subVectors(b, a)
    .cross(new THREE.Vector3().subVectors(c, a));
}

/** The area of the leaf's shadow on the xy plane - the fill the
 *  rasterizer processes when the leaf faces the camera. */
function projectedArea(mesh: ElementMesh): number {
  let total = 0;
  for (let i = 0; i < mesh.triangles; i += 1) {
    total += areaVector(mesh, i).z / 2;
  }
  return total;
}

function smallestArea(mesh: ElementMesh): number {
  let smallest = Infinity;
  for (let i = 0; i < mesh.triangles; i += 1) {
    smallest = Math.min(smallest, areaVector(mesh, i).length() / 2);
  }
  return smallest;
}

function spans(mesh: ElementMesh, axis: 0 | 1 | 2): { min: number; max: number } {
  let min = Infinity;
  let max = -Infinity;
  for (let i = 0; i < mesh.vertices; i += 1) {
    const value = mesh.positions[i * 3 + axis];
    min = Math.min(min, value);
    max = Math.max(max, value);
  }
  return { min, max };
}

describe("buildElement", () => {
  it("authors the blade in the frame placement is written against", () => {
    const mesh = element();
    // The petiole is not near the origin, it is the origin: it is the
    // point every transform placement emits is stated against.
    expect([...vertex(mesh, 0).toArray()]).toEqual([0, 0, 0]);

    const y = spans(mesh, 1);
    expect(y.min).toBe(0);
    expect(y.max).toBe(Math.fround(DEFAULT_ELEMENT.length));

    // The tip is one vertex on the axis, not a row: an outline that
    // closed on a flat end would be a truncated blade.
    const tips = [];
    for (let i = 0; i < mesh.vertices; i += 1) {
      if (vertex(mesh, i).y === y.max) tips.push(vertex(mesh, i));
    }
    expect(tips).toHaveLength(1);
    expect(tips[0].x).toBe(0);

    // Symmetric about the midrib, vertex for vertex, because a leaf
    // that leans is a leaf placement cannot orient.
    const x = spans(mesh, 0);
    expect(x.min).toBe(-x.max);
    expect(x.max).toBeGreaterThan(0);
  });

  it("faces +z everywhere, at every vertex of every triangle", () => {
    const mesh = element();
    for (let i = 0; i < mesh.triangles; i += 1) {
      expect(areaVector(mesh, i).z).toBeGreaterThan(0);
    }

    // With the curvature dials at zero the sheet is the xy plane
    // itself, so the face normal is not merely positive in z, it is
    // exactly +Z.
    const flat = element({ cup: 0, curl: 0 });
    for (let i = 0; i < flat.triangles; i += 1) {
      // Read off the area vector rather than a normalized copy: the
      // claim is that the normal has no x and no y component at all,
      // and normalizing rounds an exact answer into a float. The
      // magnitudes are there because a component that is exactly zero
      // can still carry a sign.
      const face = areaVector(flat, i);
      expect(Math.abs(face.x)).toBe(0);
      expect(Math.abs(face.y)).toBe(0);
      expect(face.z).toBeGreaterThan(0);
    }
    expect(spans(flat, 2)).toEqual({ min: 0, max: 0 });
  });

  it("fits the silhouette rather than its bounding quad", () => {
    const mesh = element();
    const quad = element({ card: true });

    // The quad is exactly what its name says, which is what makes it
    // a fair comparison rather than a straw man.
    expect(projectedArea(quad)).toBeCloseTo(
      DEFAULT_ELEMENT.length * DEFAULT_ELEMENT.width,
      9,
    );

    /* The claim, in the units the claim is about. Imagination's figure
       for a circle on its best-fit quad is 22% of processed fragments
       wasted; a leaf outline is tighter than a circle, and this blade
       covers 0.58 of its quad - so two fifths of the fragments the
       quad would rasterize are never processed at all. 0.7 is the
       loosest ratio that is still a fit rather than a rectangle with
       its corners knocked off. */
    const fill = projectedArea(mesh) / projectedArea(quad);
    expect(fill).toBeLessThan(0.7);
    // And it is a leaf, not a splinter: an outline that collapsed
    // would pass the line above for the wrong reason.
    expect(fill).toBeGreaterThan(0.4);

    // Every vertex, not the bounding box: the blade lives inside the
    // quad it is measured against.
    const half = DEFAULT_ELEMENT.width / 2;
    for (let i = 0; i < mesh.vertices; i += 1) {
      const point = vertex(mesh, i);
      expect(Math.abs(point.x)).toBeLessThanOrEqual(Math.fround(half));
      expect(point.y).toBeGreaterThanOrEqual(0);
      expect(point.y).toBeLessThanOrEqual(Math.fround(DEFAULT_ELEMENT.length));
    }
  });

  it("draws the outline the shape parameters ask for", () => {
    /* The widest point of the DRAWN outline lands where `widestAt`
       put it, which is what says the profile reaches the vertices
       rather than being a comment about them. Finely sampled so the
       station spacing is the tolerance. */
    for (const widestAt of [0.2, 0.5, 0.8]) {
      const mesh = element({ widestAt, axialSegments: 20 });
      let widest = new THREE.Vector3();
      for (let i = 0; i < mesh.vertices; i += 1) {
        const point = vertex(mesh, i);
        if (Math.abs(point.x) > Math.abs(widest.x)) widest = point;
      }
      expect(widest.y / DEFAULT_ELEMENT.length).toBeCloseTo(widestAt, 1);
      expect(Math.abs(widest.x)).toBeCloseTo(DEFAULT_ELEMENT.width / 2, 5);
    }

    // A drawn-out tip is narrower a step below the apex than a blunt
    // one is, which is the whole of what the exponent does.
    const nearTip = (tipSharpness: number): number => {
      const mesh = element({ tipSharpness, axialSegments: 20 });
      return spans(mesh, 0).max === 0 ? 0 : Math.abs(
        vertex(mesh, mesh.vertices - 2).x,
      );
    };
    expect(nearTip(4)).toBeLessThan(nearTip(0.5));
  });

  it("cups and curls out of its own plane rather than staying paper", () => {
    const mesh = element({ cup: 0.5, curl: 0.3, axialSegments: 6 });
    // The tip carries the whole of the curl.
    expect(vertex(mesh, mesh.vertices - 1).z).toBeCloseTo(
      0.3 * DEFAULT_ELEMENT.length,
      6,
    );
    // And within a row the margins stand out of the midrib, which is
    // what makes the leaf a channel rather than a plane. Checked on
    // every row, not on the mesh's average.
    const columns = 2;
    for (let row = 0; row * (columns + 1) + 1 < mesh.vertices - 1; row += 1) {
      const at = 1 + row * (columns + 1);
      const midrib = vertex(mesh, at + 1);
      expect(vertex(mesh, at).z).toBeGreaterThan(midrib.z);
      expect(vertex(mesh, at + 2).z).toBeGreaterThan(midrib.z);
      expect(vertex(mesh, at).z).toBeCloseTo(vertex(mesh, at + 2).z, 9);
    }
  });

  it("keeps the bounding quad reachable by name and off by default", () => {
    expect(DEFAULT_ELEMENT.card).toBe(false);
    expect(element().triangles).toBeGreaterThan(2);

    const quad = element({ card: true });
    expect(quad.triangles).toBe(2);
    expect(quad.vertices).toBe(4);
    expect(spans(quad, 2)).toEqual({ min: 0, max: 0 });
    for (let i = 0; i < quad.triangles; i += 1) {
      expect(areaVector(quad, i).z).toBeGreaterThan(0);
    }

    // Truthy is not the named parameter. Anything but `true` is the
    // documented default, so a caller cannot get a card by accident.
    const sloppy = element({ card: "yes" as unknown as boolean });
    expect([...sloppy.positions]).toEqual([...element().positions]);
  });

  it("falls back to the documented defaults for a parameter that is not a number", () => {
    const defaults = element();
    const absent = [Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY];
    const keys = [
      "length",
      "width",
      "widestAt",
      "baseFullness",
      "tipSharpness",
      "cup",
      "curl",
      "axialSegments",
      "crossSegments",
    ] as const;
    for (const value of absent) {
      for (const key of keys) {
        const mesh = element({ [key]: value });
        expect([...mesh.positions]).toEqual([...defaults.positions]);
        expect([...mesh.indices]).toEqual([...defaults.indices]);
      }
    }
  });

  it("holds a hostile parameter to an element it can still build", () => {
    /* A library caller does not clamp its own dials, and the failure
       to guard against is not a crash - it is a mesh that passes
       every check by being nothing at all. Zero width is not "no
       constraint": a blade of no width is a row of degenerate
       triangles wearing a valid index buffer. */
    const hostile: Partial<ElementParams>[] = [
      { length: 0, width: 0 },
      { length: -3, width: -1, widestAt: 0, baseFullness: 0, tipSharpness: 0 },
      { widestAt: 1, cup: -9, curl: -9 },
      { axialSegments: 1, crossSegments: 1 },
      { axialSegments: 0.4, crossSegments: 3 },
      {
        length: Number.MAX_VALUE,
        width: Number.MAX_VALUE,
        widestAt: Number.MAX_VALUE,
        baseFullness: Number.MAX_VALUE,
        tipSharpness: Number.MAX_VALUE,
        cup: Number.MAX_VALUE,
        curl: Number.MAX_VALUE,
        axialSegments: Number.MAX_VALUE,
        crossSegments: Number.MAX_VALUE,
      },
      { length: Number.MIN_VALUE, width: Number.MIN_VALUE },
      // Finely sampled and drawn out to a point at both ends, so the
      // outline's own arithmetic underflows toward zero width in the
      // rows nearest the petiole and the tip - the case the blade's
      // minimum width exists for.
      {
        axialSegments: 64,
        widestAt: 0.05,
        baseFullness: 8,
        tipSharpness: 8,
        width: Number.MIN_VALUE,
      },
      { card: true, length: 0, width: Number.MAX_VALUE },
    ];
    for (const params of hostile) {
      const mesh = element(params);
      for (const value of mesh.positions) {
        expect(Number.isFinite(value)).toBe(true);
      }
      expect(mesh.triangles).toBeGreaterThan(0);
      expect(smallestArea(mesh)).toBeGreaterThan(0);
      // Extent in both directions of the sheet: a leaf that survived
      // as a line would satisfy everything above.
      expect(spans(mesh, 1).max).toBeGreaterThan(0);
      expect(spans(mesh, 0).max).toBeGreaterThan(0);
      expect(mesh.triangles * 3).toBe(mesh.indices.length);
      expect(mesh.vertices * 3).toBe(mesh.positions.length);
      for (const index of mesh.indices) {
        expect(index).toBeLessThan(mesh.vertices);
      }
    }
  });

  it("is deterministic, vertex for vertex", () => {
    expect([...element().positions]).toEqual([...element().positions]);
    expect([...element().indices]).toEqual([...element().indices]);
  });

  it("costs what an instanced canopy can afford", () => {
    /* Not a benchmark - a bound. The canopy's budget is stated at 8 to
       20 triangles per element across tens of thousands of instances,
       and the default has to sit inside the band it is budgeted in. */
    const mesh = element();
    expect(mesh.triangles).toBeGreaterThanOrEqual(8);
    expect(mesh.triangles).toBeLessThanOrEqual(20);
    expect(mesh.triangles).toBe(
      2 * DEFAULT_ELEMENT.crossSegments * (DEFAULT_ELEMENT.axialSegments - 1),
    );
    expect(mesh.triangles * 3).toBe(mesh.indices.length);
    expect(mesh.vertices * 3).toBe(mesh.positions.length);
  });
});
