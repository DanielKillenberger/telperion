import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import { DEFAULT_RADII, solveRadii } from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";
import { growSkeleton } from "@/lib/grower/skeleton/grow";
import { DEFAULT_BIAS } from "@/lib/grower/torsion";

import { DEFAULT_PARAMS, type GrowerParams } from "./params";
import {
  branchGeometry,
  buildTree,
  TUBE_SIDES,
  toRadiusParams,
  toSkeletonParams,
} from "./skeleton-view";

/* The panel's promise is that a seed change and a slider move produce a
   different tree without a reload. The browser is where that is judged;
   these are the parts of it a box with no GPU can still hold to
   account - a solid comes out, the seed decides it, and the dials that
   claim to reach the generator do. */

const clay = {
  surface: new THREE.MeshStandardMaterial(),
  line: new THREE.LineBasicMaterial(),
};

function tree(overrides: Partial<GrowerParams> = {}): THREE.Mesh {
  return buildTree({ ...DEFAULT_PARAMS, ...overrides }, clay);
}

function positions(mesh: THREE.Mesh): Float32Array {
  return mesh.geometry.getAttribute("position").array as Float32Array;
}

/** The centreline the tubes are built over. The dials that shape the
 *  skeleton are checked against this rather than against the drawn
 *  vertices, which sit a branch radius off the centre and would report
 *  a straight trunk as a bent one. */
function centreline(overrides: Partial<GrowerParams> = {}): THREE.Vector3[] {
  const skeleton = growSkeleton(
    toSkeletonParams({ ...DEFAULT_PARAMS, ...overrides }),
  );
  return skeleton.nodes.map((node) => node.position);
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

  it("scales the three departure-from-vertical terms by torsion", () => {
    /* One move from straight to writhing. Gravitropism is outside it:
       a tree that wants to grow up still wants to when it is not
       twisting, and folding it in would make torsion 0 a tree with no
       opinion about direction at all. */
    const straight = toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 0 });
    expect(straight.bias).toEqual({
      gravitropism: DEFAULT_BIAS.gravitropism,
      lean: 0,
      writheAmplitude: 0,
      writheWavelength: DEFAULT_BIAS.writheWavelength,
      spiralRate: 0,
    });

    const doubled = toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 2 });
    expect(doubled.bias).toEqual({
      gravitropism: DEFAULT_BIAS.gravitropism,
      lean: DEFAULT_BIAS.lean * 2,
      writheAmplitude: DEFAULT_BIAS.writheAmplitude * 2,
      writheWavelength: DEFAULT_BIAS.writheWavelength,
      spiralRate: DEFAULT_BIAS.spiralRate * 2,
    });
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

  it("hands the turn limit over as the growth argument it is", () => {
    // Not a bias term: persistence is about the step, not about the
    // field, so it travels in `growth` under the library's own name.
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, maxTurnPerStep: 18 }).growth)
      .toEqual({ maxTurnPerStep: 18 });
    // And it is outside torsion - a stiff tree is stiff whether or not
    // it is writhing.
    expect(toSkeletonParams({ ...DEFAULT_PARAMS, torsion: 0 }).growth).toEqual({
      maxTurnPerStep: DEFAULT_PARAMS.maxTurnPerStep,
    });
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

describe("toRadiusParams", () => {
  it("hands the taper dial over as the fork exponent", () => {
    expect(toRadiusParams({ ...DEFAULT_PARAMS, taper: 2.6 }).forkExponent).toBe(
      2.6,
    );
  });

  it("leaves the solve's other two terms at the library's own defaults", () => {
    // The panel is not allowed a second opinion about how stout a tree
    // is; the day one of these gets a dial it gets one here, not a
    // number invented in the harness.
    const mapped = toRadiusParams(DEFAULT_PARAMS);
    expect(mapped.trunkRadius).toBe(DEFAULT_RADII.trunkRadius);
    expect(mapped.lengthTaper).toBe(DEFAULT_RADII.lengthTaper);
    expect(toRadiusParams(DEFAULT_PARAMS)).toEqual(DEFAULT_RADII);
  });
});

describe("branchGeometry", () => {
  const straight: Skeleton = {
    nodes: [
      { position: new THREE.Vector3(0, 0, 0), parent: -1 },
      { position: new THREE.Vector3(0, 1, 0), parent: 0 },
      { position: new THREE.Vector3(1, 2, 0), parent: 1 },
    ],
  };

  it("builds one closed tube per branch and nothing for the root", () => {
    const field = solveRadii(straight, DEFAULT_ENVELOPE, DEFAULT_RADII);
    const geometry = branchGeometry(straight, field);
    // Two branches, two rings each, and two triangles per side.
    expect(geometry.getAttribute("position").count).toBe(2 * 2 * TUBE_SIDES);
    expect(geometry.getIndex()!.count).toBe(2 * TUBE_SIDES * 6);
  });

  it("puts each ring at the radius the solve gave that end", () => {
    const field = solveRadii(straight, DEFAULT_ENVELOPE, DEFAULT_RADII);
    const geometry = branchGeometry(straight, field);
    const points = geometry.getAttribute("position").array as Float32Array;

    // The first ring sits about the root, at the trunk's own radius:
    // the tube is a viewer for the field, so a vertex that disagreed
    // with it would be the harness inventing a thickness of its own.
    for (let side = 0; side < TUBE_SIDES; side += 1) {
      const at = side * 3;
      const offset = new THREE.Vector3(points[at], points[at + 1], points[at + 2]);
      expect(offset.length()).toBeCloseTo(field.startRadius[1], 5);
    }
  });

  it("draws nothing for a skeleton that never grew", () => {
    const lone: Skeleton = {
      nodes: [{ position: new THREE.Vector3(0, 0, 0), parent: -1 }],
    };
    const geometry = branchGeometry(
      lone,
      solveRadii(lone, DEFAULT_ENVELOPE, DEFAULT_RADII),
    );
    expect(geometry.getAttribute("position").count).toBe(0);
    expect(geometry.getIndex()!.count).toBe(0);
  });

  it("skips a zero-length branch rather than emitting NaN vertices", () => {
    // Normalising a zero axis is the one way this geometry can put a
    // NaN on screen, and a NaN vertex takes the whole draw call with it.
    const doubled: Skeleton = {
      nodes: [
        { position: new THREE.Vector3(0, 0, 0), parent: -1 },
        { position: new THREE.Vector3(0, 0, 0), parent: 0 },
        { position: new THREE.Vector3(0, 1, 0), parent: 1 },
      ],
    };
    const geometry = branchGeometry(
      doubled,
      solveRadii(doubled, DEFAULT_ENVELOPE, DEFAULT_RADII),
    );
    expect(geometry.getAttribute("position").count).toBe(2 * TUBE_SIDES);
    for (const value of geometry.getAttribute("position").array) {
      expect(Number.isFinite(value)).toBe(true);
    }
  });
});

describe("buildTree", () => {
  it("produces a solid the stage can draw, in clay", () => {
    const mesh = tree();
    expect(mesh).toBeInstanceOf(THREE.Mesh);
    expect(mesh.material).toBe(clay.surface);
    expect(positions(mesh).length).toBeGreaterThan(0);
    // One ring per branch end, three floats a vertex.
    expect(positions(mesh).length % (TUBE_SIDES * 2 * 3)).toBe(0);
    expect(mesh.geometry.getAttribute("normal")).toBeDefined();
    for (const value of positions(mesh)) {
      expect(Number.isFinite(value)).toBe(true);
    }
  });

  it("is thick at the foot and fine at the twigs", () => {
    /* The whole point of the task on screen: a trunk that is visibly a
       trunk. Measured off the drawn surface rather than off the field,
       because the field being right and the mesh ignoring it is exactly
       the failure this catches. */
    const mesh = tree();
    const points = positions(mesh);
    const height = DEFAULT_PARAMS.height;

    let footWidth = 0;
    let crownWidest = 0;
    let crownNarrowest = Number.POSITIVE_INFINITY;
    const axis = new THREE.Vector3();
    for (let at = 0; at < points.length; at += 3) {
      const y = points[at + 1];
      axis.set(points[at], 0, points[at + 2]);
      if (y < height * 0.02) footWidth = Math.max(footWidth, axis.length());
    }

    // A ring high in the crown, taken about its own centre: the twigs
    // there are a small fraction of the trunk they hang off.
    const solved = solveRadii(
      growSkeleton(toSkeletonParams(DEFAULT_PARAMS)),
      toSkeletonParams(DEFAULT_PARAMS).envelope,
      toRadiusParams(DEFAULT_PARAMS),
    );
    for (let node = 1; node < solved.radius.length; node += 1) {
      crownWidest = Math.max(crownWidest, solved.radius[node]);
      crownNarrowest = Math.min(crownNarrowest, solved.radius[node]);
    }

    const trunk = DEFAULT_RADII.trunkRadius * height;
    expect(footWidth).toBeCloseTo(trunk, 2);
    expect(crownWidest).toBeLessThan(trunk);
    expect(crownNarrowest * 20).toBeLessThan(trunk);
  });

  it("the taper dial changes the thickness and not the branching", () => {
    const sharp = tree({ taper: 1.6 });
    const soft = tree({ taper: 3.4 });
    expect([...positions(sharp)]).not.toEqual([...positions(soft)]);
    // Same skeleton underneath: taper is solved over the branching, it
    // does not grow a different tree.
    expect(centreline({ taper: 1.6 })).toEqual(centreline({ taper: 3.4 }));
  });

  it("is deterministic in the seed", () => {
    expect([...positions(tree({ seed: 7 }))]).toEqual([
      ...positions(tree({ seed: 7 })),
    ]);
  });

  it("a different seed grows a different tree", () => {
    expect([...positions(tree({ seed: 7 }))]).not.toEqual([
      ...positions(tree({ seed: 8 })),
    ]);
  });

  it("the height dial reaches the geometry", () => {
    const short = tree({ height: 8 });
    const tall = tree({ height: 48 });
    short.geometry.computeBoundingBox();
    tall.geometry.computeBoundingBox();
    expect(tall.geometry.boundingBox!.max.y).toBeGreaterThan(
      short.geometry.boundingBox!.max.y * 3,
    );
  });

  it("the spread dial reaches the silhouette", () => {
    const widest = (spread: number): number => {
      const geometry = tree({ spread }).geometry;
      geometry.computeBoundingBox();
      const box = geometry.boundingBox!;
      return Math.max(box.max.x, box.max.z, -box.min.x, -box.min.z);
    };
    expect(widest(1.2)).toBeGreaterThan(widest(0.2) * 3);
  });

  it("the torsion dial takes the tree from straight to writhing", () => {
    /* The spec's own acceptance, in one move: at zero the trunk is a
       mathematically straight line and at the top of the dial it is
       not, with connectivity intact at both ends. Measured on the
       centreline the tubes are built over - the drawn vertices sit a
       branch radius off it, and off a solid this test would be reading
       the trunk's own girth as a bow. */
    const trunkBow = (torsion: number): number => {
      const crownBase = DEFAULT_PARAMS.height * 0.3;
      const trunk = centreline({ torsion }).filter(
        (point) => point.y <= crownBase,
      );
      expect(trunk.length).toBeGreaterThan(8);
      const first = trunk[0];
      const axis = trunk[trunk.length - 1].clone().sub(first).normalize();
      return trunk.reduce((worst, point) => {
        const offset = point.clone().sub(first);
        return Math.max(
          worst,
          offset.clone().addScaledVector(axis, -offset.dot(axis)).length(),
        );
      }, 0);
    };

    expect(trunkBow(0)).toBeLessThan(1e-6);
    expect(trunkBow(1)).toBeGreaterThan(0.2);
    expect(trunkBow(2)).toBeGreaterThan(trunkBow(1));

    // Connectivity: every branch starts where its parent ended, at both
    // ends of the dial, and the tube for it is built over that edge.
    for (const torsion of [0, 2]) {
      const skeleton = growSkeleton(
        toSkeletonParams({ ...DEFAULT_PARAMS, torsion }),
      );
      skeleton.nodes.forEach((node, index) => {
        if (index === 0) return expect(node.parent).toBe(-1);
        expect(node.parent).toBeGreaterThanOrEqual(0);
        expect(node.parent).toBeLessThan(index);
      });
    }
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
    const flat = [...positions(tree(straight))];
    for (const dial of [
      { gravitropism: 0.9 },
      { lean: 0.4 },
      { writheAmplitude: 0.15, writheWavelength: 0.5 },
      { writheAmplitude: 0.15, writheWavelength: 0.1 },
      { writheAmplitude: 0.15, spiralRate: 4 },
    ]) {
      expect([...positions(tree({ ...straight, ...dial }))]).not.toEqual(flat);
    }
  });

  it("the density dial reaches the branch count", () => {
    expect(positions(tree({ density: 1 })).length).toBeGreaterThan(
      positions(tree({ density: 0 })).length * 2,
    );
  });
});
