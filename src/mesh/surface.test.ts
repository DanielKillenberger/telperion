import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import {
  buildSurface,
  DEFAULT_SURFACE,
  type SurfaceMesh,
  type SurfaceParams,
} from "@/lib/grower/mesh/surface";
import { DEFAULT_RADII, solveRadii } from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";
import { growSkeleton, type SkeletonParams } from "@/lib/grower/skeleton/grow";
import { DEFAULT_BIAS, NO_BIAS } from "@/lib/grower/torsion";

/* ------------------------------------------------------------------ *
 * There is no browser on this box and no eye on this file, so nothing
 * here claims the surface looks like anything. What it can hold to
 * account is structure, and structure is where the reported defect
 * lives: the fn-11.4 viewer's rings and see-through joints are, stated
 * without reference to a picture, boundary edges - places where the
 * mesh simply stops and you are looking at the inside of the far side.
 *
 * So the load-bearing assertion is that the mesh has none. Every
 * directed edge appears exactly once and its reverse appears exactly
 * once, which says at the same time that the surface is closed (no
 * gaps to see through) and that its winding is consistent (no inverted
 * normal). A positive signed volume then says the consistent winding
 * is the outward one. Those three together are the owner's "gaps",
 * "seams" and "inside out" in a form a box with no GPU can check.
 * ------------------------------------------------------------------ */

function surface(
  skeleton: Skeleton,
  params: Partial<SurfaceParams> = {},
  envelope = DEFAULT_ENVELOPE,
): SurfaceMesh {
  return buildSurface(
    skeleton,
    solveRadii(skeleton, envelope, DEFAULT_RADII),
    envelope,
    { ...DEFAULT_SURFACE, ...params },
  );
}

function grown(params: Partial<SkeletonParams> = {}): Skeleton {
  return growSkeleton({
    seed: 1,
    envelope: DEFAULT_ENVELOPE,
    attractors: 500,
    ...params,
  });
}

const at = (x: number, y: number, z: number, parent: number) => ({
  position: new THREE.Vector3(x, y, z),
  parent,
});

/** A bare vertical trunk, three nodes, no fork. */
const straight: Skeleton = {
  nodes: [at(0, 0, 0, -1), at(0, 4, 0, 0), at(0, 8, 0, 1)],
};

function triangle(mesh: SurfaceMesh, index: number): THREE.Vector3[] {
  return [0, 1, 2].map((k) => {
    const at3 = mesh.indices[index * 3 + k] * 3;
    return new THREE.Vector3(
      mesh.positions[at3],
      mesh.positions[at3 + 1],
      mesh.positions[at3 + 2],
    );
  });
}

/** Directed edges that have no opposite: the boundary of the mesh. A
 *  gap, a hole and a see-through joint are all this number above zero,
 *  and an inverted triangle shows up as a directed edge used twice. */
function boundary(mesh: SurfaceMesh): { open: number; repeated: number } {
  const seen = new Set<string>();
  let repeated = 0;
  for (let t = 0; t < mesh.triangles; t += 1) {
    const tri = [0, 1, 2].map((k) => mesh.indices[t * 3 + k]);
    for (let k = 0; k < 3; k += 1) {
      const key = `${tri[k]}:${tri[(k + 1) % 3]}`;
      if (seen.has(key)) repeated += 1;
      seen.add(key);
    }
  }
  let open = 0;
  for (const key of seen) {
    const [a, b] = key.split(":");
    if (!seen.has(`${b}:${a}`)) open += 1;
  }
  return { open, repeated };
}

/** Six times the volume the winding encloses. Positive is outward. */
function signedVolume(mesh: SurfaceMesh): number {
  let total = 0;
  for (let t = 0; t < mesh.triangles; t += 1) {
    const [a, b, c] = triangle(mesh, t);
    total += a.dot(b.clone().cross(c));
  }
  return total;
}

function smallestArea(mesh: SurfaceMesh): number {
  let smallest = Number.POSITIVE_INFINITY;
  for (let t = 0; t < mesh.triangles; t += 1) {
    const [a, b, c] = triangle(mesh, t);
    const area =
      b.clone().sub(a).cross(c.clone().sub(a)).length() / 2;
    smallest = Math.min(smallest, area);
  }
  return smallest;
}

/** The ring drawn at sample `index` of the first run, as (angle,
 *  radius) about the trunk axis. Only meaningful for `straight`, whose
 *  frames are known: a vertical tangent gives normal +z and binormal
 *  +x, so the angle the section was built at is atan2(x, z). */
function ring(mesh: SurfaceMesh, index: number, segments: number) {
  return Array.from({ length: segments }, (unused, k) => {
    const at3 = (index * segments + k) * 3;
    const x = mesh.positions[at3];
    const z = mesh.positions[at3 + 2];
    return { angle: Math.atan2(x, z), radius: Math.hypot(x, z) };
  });
}

describe("buildSurface", () => {
  /* The bias extremes the acceptance names. The gaps the owner
     screenshotted were worst down the trunk, where the bend is
     strongest, so a surface that is only continuous on a straight tree
     has not answered anything. */
  const configs: [string, Partial<SkeletonParams>][] = [
    ["no bias at all", { bias: NO_BIAS }],
    ["the default tree", { bias: DEFAULT_BIAS }],
    [
      "writhe and spiral at the top of their dials",
      {
        bias: {
          ...DEFAULT_BIAS,
          writheAmplitude: 0.25,
          writheWavelength: 0.18,
          spiralRate: 6,
          lean: 0.5,
        },
      },
    ],
    ["the turn limit off", { growth: { maxTurnPerStep: 90 } }],
  ];

  for (const [what, params] of configs) {
    it(`has no gap, seam or inverted face with ${what}`, () => {
      const mesh = surface(grown(params));
      expect(mesh.triangles).toBeGreaterThan(1000);
      // Closed and consistently wound: no boundary edge anywhere, and
      // no directed edge shared by two faces.
      expect(boundary(mesh)).toEqual({ open: 0, repeated: 0 });
      // ...and the consistent winding is the outward one.
      expect(signedVolume(mesh)).toBeGreaterThan(0);
      // No pinch: every triangle has area to it.
      expect(smallestArea(mesh)).toBeGreaterThan(0);
      for (const value of mesh.positions) {
        expect(Number.isFinite(value)).toBe(true);
      }
    });
  }

  it("shares one ring per node between the segments either side", () => {
    /* The fn-11.4 viewer drew a tube per edge: two rings per edge, four
       for this skeleton, and the two at the middle node were free to
       part company. One ring per node cannot part company with itself,
       and the vertex count is where that shows. */
    const segments = 8;
    const mesh = surface(straight, { radialSegments: segments });
    // Three nodes plus the buried base ring, and one centre vertex per
    // end cap.
    expect(mesh.vertices).toBe(4 * segments + 2);

    const distinct = new Set<string>();
    for (let v = 0; v < mesh.vertices; v += 1) {
      distinct.add(
        `${mesh.positions[v * 3]},${mesh.positions[v * 3 + 1]},${mesh.positions[v * 3 + 2]}`,
      );
    }
    // No duplicate vertices at the joints: there are no joints.
    expect(distinct.size).toBe(mesh.vertices);
  });

  it("takes its radii from the field and interpolates them", () => {
    /* The mean radius of a ring is the solve's radius at that node,
       exactly - the lobes modulate around it and do not fatten it - and
       consecutive rings differ by the taper over one step rather than
       stepping at the node. */
    const skeleton = straight;
    const field = solveRadii(skeleton, DEFAULT_ENVELOPE, DEFAULT_RADII);
    const segments = 24;
    const mesh = surface(skeleton, {
      radialSegments: segments,
      flareRadius: 1,
      lobeDepth: 0.3,
    });

    for (const [sample, node] of [
      [1, 0],
      [2, 1],
      [3, 2],
    ]) {
      const radii = ring(mesh, sample, segments).map((v) => v.radius);
      const mean = radii.reduce((sum, r) => sum + r, 0) / segments;
      expect(mean).toBeCloseTo(field.radius[node], 6);
      // Non-circular: the section is not the circle everyone else
      // extrudes.
      expect(Math.max(...radii) - Math.min(...radii)).toBeGreaterThan(
        field.radius[node] * 0.5,
      );
    }

    const circular = surface(skeleton, {
      radialSegments: segments,
      flareRadius: 1,
      lobes: 0,
    });
    const flat = ring(circular, 2, segments).map((v) => v.radius);
    // Six places, not twelve: the vertices are stored as float32, so
    // "the same radius all the way round" is only ever true to about a
    // part in ten million.
    expect(Math.max(...flat) - Math.min(...flat)).toBeCloseTo(0, 6);
  });

  it("winds the section along the length at the stated rate", () => {
    /* The plait. `twistRate` is turns of the section about its own axis
       over the envelope's height, and this reads the phase straight off
       the vertices: for a lobed section the first Fourier coefficient
       at the lobe frequency has the phase as its argument. */
    const segments = 24;
    const lobes = 5;
    const twistRate = 1.5;
    const mesh = surface(straight, {
      radialSegments: segments,
      lobes,
      lobeDepth: 0.25,
      flareRadius: 1,
      twistRate,
    });

    const phase = (sample: number): number => {
      let real = 0;
      let imaginary = 0;
      for (const { angle, radius } of ring(mesh, sample, segments)) {
        real += radius * Math.cos(lobes * angle);
        imaginary -= radius * Math.sin(lobes * angle);
      }
      return Math.atan2(imaginary, real) / lobes;
    };

    // Samples 1 and 3 are the nodes at y=0 and y=8 - eight metres of
    // sweep on a 24 m envelope, so one and a half turns over the height
    // is half a turn of section between them.
    const turned = phase(3) - phase(1);
    const expected = (2 * Math.PI * twistRate * 8) / DEFAULT_ENVELOPE.height;
    const period = (2 * Math.PI) / lobes;
    // Modulo the lobe period: a five-lobed section repeats every fifth
    // of a turn, so that is the finest statement the vertices support.
    const off = (((turned - expected) % period) + period) % period;
    expect(Math.min(off, period - off)).toBeCloseTo(0, 6);

    const still = surface(straight, {
      radialSegments: segments,
      lobes,
      lobeDepth: 0.25,
      flareRadius: 1,
      twistRate: 0,
    });
    const base = ring(still, 1, segments).map((v) => v.radius);
    const top = ring(still, 3, segments).map((v) => v.radius);
    // At rate zero the section does not rotate at all: every ring's
    // lobes sit at the same angles.
    for (let k = 0; k < segments; k += 1) {
      expect(top[k] / base[k]).toBeCloseTo(top[0] / base[0], 6);
    }
  });

  it("flares into the ground instead of ending on a flat disc at y=0", () => {
    const flareRadius = 2.5;
    const mesh = surface(straight, { flareRadius, lobeDepth: 0, lobes: 0 });
    const field = solveRadii(straight, DEFAULT_ENVELOPE, DEFAULT_RADII);
    const segments = DEFAULT_SURFACE.radialSegments;

    // The ring at the ground is the trunk's radius times the flare.
    for (const { radius } of ring(mesh, 1, segments)) {
      expect(radius).toBeCloseTo(field.radius[0] * flareRadius, 6);
    }
    // The end cap is below the ground, so there is no disc to see.
    let lowest = Number.POSITIVE_INFINITY;
    for (let v = 1; v < mesh.positions.length; v += 3) {
      lowest = Math.min(lowest, mesh.positions[v]);
    }
    expect(lowest).toBeLessThan(0);
    // And it is a ground effect, not a trunk effect: gone by the top.
    const high = ring(mesh, 3, segments)[0].radius;
    expect(high).toBeCloseTo(field.radius[2], 6);
  });

  it("sockets a child inside its parent so a fork cannot open", () => {
    /* The fork half of the continuity fix, measured where it matters:
       the child's first ring sits on the parent's centreline side of
       its own surface, so the child emerges through the parent's skin
       rather than meeting it end to end. */
    const forked: Skeleton = {
      nodes: [
        at(0, 0, 0, -1),
        at(0, 4, 0, 0),
        at(2, 6, 0, 1),
        at(-2, 6, 0, 1),
      ],
    };
    const field = solveRadii(forked, DEFAULT_ENVELOPE, DEFAULT_RADII);
    const segments = DEFAULT_SURFACE.radialSegments;
    const mesh = surface(forked, { flareRadius: 1 });

    // Two runs: the trunk with its buried base, then the side limb.
    // The trunk run comes first: its buried base ring, its three
    // nodes, and its two cap centres. The side run starts after it.
    const sideRun = 4 * segments + 2;
    const fork = forked.nodes[1].position;
    const centre = new THREE.Vector3();
    for (let k = 0; k < segments; k += 1) {
      const at3 = (sideRun + k) * 3;
      centre.add(
        new THREE.Vector3(
          mesh.positions[at3],
          mesh.positions[at3 + 1],
          mesh.positions[at3 + 2],
        ),
      );
    }
    centre.divideScalar(segments);
    // Started back inside the parent's solid, not out at its surface.
    expect(centre.distanceTo(fork)).toBeLessThan(field.radius[1]);
    expect(centre.distanceTo(fork)).toBeGreaterThan(0);
    expect(boundary(mesh)).toEqual({ open: 0, repeated: 0 });
  });

  it("is deterministic, vertex for vertex", () => {
    const skeleton = grown();
    expect([...surface(skeleton).positions]).toEqual([
      ...surface(skeleton).positions,
    ]);
  });

  it("has nothing to skin in a skeleton that never grew", () => {
    const lone: Skeleton = { nodes: [at(0, 0, 0, -1)] };
    const mesh = surface(lone);
    expect(mesh.triangles).toBe(0);
    expect(mesh.vertices).toBe(0);
  });

  it("holds a hostile parameter to a surface it can still build", () => {
    // The panel clamps its own dials; a library caller does not, and a
    // section of two sides or a lobe deeper than the radius is a fold
    // through the centreline rather than a look.
    const mesh = surface(grown(), {
      radialSegments: 1,
      lobes: -4,
      lobeDepth: 8,
      flareRadius: 0,
      flareFalloff: 0,
      forkSocket: -1,
      forkSwell: -3,
      twistRate: Number.NaN,
    });
    expect(boundary(mesh)).toEqual({ open: 0, repeated: 0 });
    expect(smallestArea(mesh)).toBeGreaterThan(0);
    for (const value of mesh.positions) {
      expect(Number.isFinite(value)).toBe(true);
    }
  });

  it("costs what the panel says it costs", () => {
    /* Not a benchmark - a bound. The swept surface is allowed to cost
       more than the fn-11.4 viewer's 21k triangles, and the panel
       reports what it cost, but an order more than that would mean
       something is being drawn that nobody asked for. */
    const mesh = surface(grown({ attractors: 925 }));
    expect(mesh.triangles).toBeLessThan(210_000);
    expect(mesh.triangles * 3).toBe(mesh.indices.length);
    expect(mesh.vertices * 3).toBe(mesh.positions.length);
  });
});
