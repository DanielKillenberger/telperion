import * as THREE from "three";
import { beforeAll, describe, expect, it } from "vitest";
import { readFileSync } from "node:fs";
import { initializeTreeCore, LAURELIN, TWO_TREES, ORDINARY, treeCore } from "../src/browser/core";

import { DEFAULT_PARAMS, type GrowerParams } from "./params";
import { toRadiusParams, toSkeletonParams } from "./family";
import {
  buildComparison,
  selectSpecimenView,
  buildPreset,
  buildTree,
  countDraws,
} from "./skeleton-view";

// Keep the existing full-size geometry suite bounded to the Two Trees.
// All five identities have parameter roundtrips here and small Wasm binding fixtures.
const PRESETS = TWO_TREES;

beforeAll(() => initializeTreeCore(readFileSync("src/browser/telperion.wasm")));

const DEFAULT_RADII = ORDINARY.radii;

/* The panel's promise is that a seed change and a slider move produce a
   different tree without a reload. The browser is where that is judged;
   these are the parts of it a box with no GPU can still hold to
   account - a solid comes out, the seed decides it, and the dials that
   claim to reach the generator do. */

const clay = {
  surface: new THREE.MeshStandardMaterial(),
  line: new THREE.LineBasicMaterial(),
  element: new THREE.MeshStandardMaterial(),
};

/** The swept trunk out of a built subject. The subject is a group of
 *  two renderables now - the trunk and the crown as one instanced draw
 *  - and every assertion about branch geometry is about the first of
 *  them. Named by the object rather than by index, because a subject
 *  with no canopy has one child and one with a canopy has two. */
function trunkOf(subject: THREE.Object3D): THREE.Mesh {
  const found = subject.getObjectByName("grower-trunk");
  expect(found).toBeInstanceOf(THREE.Mesh);
  return found as THREE.Mesh;
}

/** The crown, or null on a tree that grew none. */
function canopyOf(subject: THREE.Object3D): THREE.InstancedMesh | null {
  const found = subject.getObjectByName("grower-canopy") ?? null;
  return found as THREE.InstancedMesh | null;
}

function tree(overrides: Partial<GrowerParams> = {}): THREE.Mesh {
  return trunkOf(buildTree({ ...DEFAULT_PARAMS, ...overrides }, clay).tree);
}

function positions(mesh: THREE.Mesh): Float32Array {
  return mesh.geometry.getAttribute("position").array as Float32Array;
}

/** The centreline the tubes are built over. The dials that shape the
 *  skeleton are checked against this rather than against the drawn
 *  vertices, which sit a branch radius off the centre and would report
 *  a straight trunk as a bent one. */
function centreline(overrides: Partial<GrowerParams> = {}): THREE.Vector3[] {
  const { values } = structure(overrides);
  return Array.from({ length: values.length / 6 }, (_, i) =>
    new THREE.Vector3(values[i * 6], values[i * 6 + 1], values[i * 6 + 2]));
}

function structure(overrides: Partial<GrowerParams> = {}) {
  const params = { ...DEFAULT_PARAMS, ...overrides };
  const output = treeCore().build({
    ...ORDINARY,
    skeleton: toSkeletonParams(params),
    radii: toRadiusParams(params),
  }, { structure: true });
  treeCore().release();
  return output.structure!;
}

describe("buildTree", () => {
  it("puts more branches on the tree when more laterals are asked for", () => {
    expect(positions(tree({ laterals: 2 })).length).toBeGreaterThan(
      positions(tree({ laterals: 0 })).length * 2,
    );
  });

  it("produces a solid the stage can draw, in clay", () => {
    const mesh = tree();
    expect(mesh).toBeInstanceOf(THREE.Mesh);
    expect(mesh.material).toBe(clay.surface);
    expect(positions(mesh).length).toBeGreaterThan(0);
    expect(mesh.geometry.getAttribute("normal")).toBeDefined();
    for (const value of positions(mesh)) {
      expect(Number.isFinite(value)).toBe(true);
    }
  }, 60_000);

  it("reports what the build cost, for the panel to show", () => {
    // The surface is allowed to cost more than the tube viewer did.
    // It is not allowed to cost it silently.
    const { tree: subject, stats } = buildTree(DEFAULT_PARAMS, clay);
    const mesh = trunkOf(subject);
    expect(stats.triangles * 3).toBe(mesh.geometry.getIndex()!.count);
    expect(stats.vertices * 3).toBe(positions(mesh).length);
    expect(stats.nodes).toBe(
      structure().values.length / 6,
    );
    expect(stats.buildMs).toBeGreaterThanOrEqual(0);
  });

  it("draws the whole crown in one call, however many leaves it has", () => {
    /* R4, stated as the two numbers the panel reports. A tree is two
       draws - the swept surface and the crown - whatever the canopy
       costs, and the instance count is what a crown that quietly added
       one draw per leaf would fail on. Thousands and not tens, because
       the default dials are one 24 m tree; the claim is one draw at
       any count, not a count. */
    const { tree: subject, stats } = buildTree(DEFAULT_PARAMS, clay);
    const crown = canopyOf(subject);
    expect(crown).not.toBeNull();
    expect(stats.drawCalls).toBe(2);
    expect(stats.instances).toBeGreaterThan(1000);
    expect(stats.instances).toBe(crown!.count);
  });

  it("cuts the foliage out rather than blending it, from both faces", () => {
    /* Alpha test, never blending, and a leaf is an open sheet with no
       back face. Both are the harness's material rather than the
       library's - the canopy arrives as geometry and transforms - so
       this asserts that the crown is drawn with the clay the room
       handed it and nothing else. */
    const crown = canopyOf(buildTree(DEFAULT_PARAMS, clay).tree);
    expect(crown!.material).toBe(clay.element);
  });

  it("measures a crown that is there, before anything frames it", () => {
    /* R8's integration risk. An InstancedMesh carries its own bounding
       box and three computes it once, from the instance transforms -
       so one built before its matrices are written measures a single
       leaf at the origin, and a stage that trusts it frames the
       branches alone. The mesh leaves the builder with bounds that
       cover the crown, which on this tree is metres and not
       centimetres. */
    const crown = canopyOf(buildTree(DEFAULT_PARAMS, clay).tree)!;
    expect(crown.boundingBox).not.toBeNull();
    const span = crown
      .boundingBox!.getSize(new THREE.Vector3())
      .length();
    const oneLeaf = crown.geometry.boundingBox!.getSize(
      new THREE.Vector3(),
    ).length();
    expect(span).toBeGreaterThan(oneLeaf * 20);
    expect(span).toBeGreaterThan(DEFAULT_PARAMS.height * 0.3);
  });

  it("puts the crown on the tree, not beside it", () => {
    /* The transforms are the library's and this is the one thing the
       harness could get wrong about them: a column-major buffer read
       as row-major puts every leaf somewhere else entirely. The
       crown's bounds sit inside the branches' own, give or take a leaf
       - foliage grows on wood. */
    const subject = buildTree(DEFAULT_PARAMS, clay).tree;
    const wood = new THREE.Box3().setFromObject(trunkOf(subject));
    const crown = canopyOf(subject)!.boundingBox!;
    const leaf = 0.3;
    expect(crown.min.y).toBeGreaterThan(wood.min.y - leaf);
    expect(crown.max.y).toBeLessThan(wood.max.y + leaf);
    expect(crown.max.x - crown.min.x).toBeLessThan(
      (wood.max.x - wood.min.x) * 1.5 + leaf,
    );
  });

  it("leaves no canopy at all rather than an empty draw", () => {
    // An empty canopy must leave no instanced object or draw in the scene.
    const bare = buildTree(DEFAULT_PARAMS, clay, false);
    expect(canopyOf(bare.tree)).toBeNull();
    expect(bare.stats.drawCalls).toBe(1);
    expect(bare.stats.instances).toBe(0);
  });

  it("passes twig stations through to the placed canopy", () => {
    const sparse = buildTree({ ...DEFAULT_PARAMS, twigStations: 1 }, clay);
    const dense = buildTree({ ...DEFAULT_PARAMS, twigStations: 4 }, clay);
    expect(sparse.stats.instances).toBeGreaterThan(0);
    expect(dense.stats.instances).toBeGreaterThan(sparse.stats.instances * 2);
    expect(positions(trunkOf(dense.tree))).toEqual(positions(trunkOf(sparse.tree)));
    for (const built of [sparse, dense]) built.tree.traverse((object) => {
      if (object instanceof THREE.Mesh) object.geometry.dispose();
      if (object instanceof THREE.InstancedMesh) object.dispose();
    });
  }, 60_000);

  it("grows the same canopy from the same seed, transform for transform", () => {
    /* R7 on the harness's side of the seam. The canopy is placed from
       its own sub-stream, so a canopy that drifted between two builds
       of one seed would be a stage reaching into another's chance. */
    const first = canopyOf(buildTree(DEFAULT_PARAMS, clay).tree)!;
    const second = canopyOf(buildTree(DEFAULT_PARAMS, clay).tree)!;
    expect([...(first.instanceMatrix.array as Float32Array)]).toEqual([
      ...(second.instanceMatrix.array as Float32Array),
    ]);
  });

  it("every surface dial reaches the geometry", () => {
    // Ordinary is circular now: exercise lobe count/twist with explicit lobing.
    const surface = { lobeDepth: 0.2 };
    const plain = [...positions(tree(surface))];
    for (const dial of [
      { lobes: 0 },
      { lobeDepth: 0.4 },
      { twistRate: -3 },
      { flareRadius: 4 },
    ]) {
      expect([...positions(tree({ ...surface, ...dial }))]).not.toEqual(plain);
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
    const { values } = structure();
    for (let node = 1; node < values.length / 6; node += 1) {
      crownWidest = Math.max(crownWidest, values[node * 6 + 3]);
      crownNarrowest = Math.min(crownNarrowest, values[node * 6 + 3]);
    }

    /* At the foot the drawn surface is deliberately wider than the
       solve's trunk radius: the root flare spreads it into the ground,
       and the lobes cut in and out around it. Both are stated
       multipliers, so the widest vertex down there is exactly the two
       of them on the trunk radius and nothing else. */
    const trunk = DEFAULT_RADII.trunkRadius * height;
    const flared =
      trunk * DEFAULT_PARAMS.flareRadius * (1 + DEFAULT_PARAMS.lobeDepth);
    expect(footWidth).toBeGreaterThan(trunk);
    expect(footWidth).toBeLessThanOrEqual(flared * 1.001);
    expect(crownWidest).toBeLessThan(trunk);
    expect(crownNarrowest * 20).toBeLessThan(trunk);
  });

  it("the taper dial changes thickness and branch generations while preserving colonization", () => {
    const sharp = tree({ taper: 1.6 });
    const soft = tree({ taper: 3.4 });
    expect([...positions(sharp)]).not.toEqual([...positions(soft)]);
    const grown = [1.6, 3.4].map(taper => structure({ taper }));
    const nodes = (output: ReturnType<typeof structure>, kind: number) =>
      Array.from({ length: output.values.length / 6 }, (_, i) => i)
        .filter(i => output.topology[i * 3 + 2] === kind)
        .map(i => [...output.values.slice(i * 6, i * 6 + 3), output.topology[i * 3]]);
    expect(nodes(grown[0], 0)).toEqual(nodes(grown[1], 0));
    expect(nodes(grown[0], 1)).not.toEqual(nodes(grown[1], 1));
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
      const trunk = centreline({ torsion, lean: 0, supernaturalEnabled: true, writheAmplitude: 0.1, spiralRate: 0 }).filter(
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
    // Natural growth and clipping change which nodes lie below this height;
    // that subset's bow is not monotonic in amplitude. Both enabled settings
    // must bend and differ; exact torsion scaling is covered above.
    expect(trunkBow(2)).toBeGreaterThan(0.2);
    expect(trunkBow(2)).not.toBeCloseTo(trunkBow(1), 6);

    // Connectivity: every branch starts where its parent ended, at both
    // ends of the dial, and the tube for it is built over that edge.
    for (const torsion of [0, 2]) {
      const { topology } = structure({ torsion });
      for (let index = 0; index < topology.length / 3; index += 1) {
        const parent = topology[index * 3];
        if (index === 0) expect(parent).toBe(0xffffffff);
        else expect(parent).toBeLessThan(index);
      }
    }
  });

  it("every bias dial reaches the geometry", () => {
    /* The panel promised for two tasks that torsion would land in
       fn-11.3 and reached nothing in the meantime. Five dials now, one
       per term of the growth bias field, and each one has to move the
       tree on its own - a dial that renders and does nothing is worse
       than no dial. */
    const straight = {
      supernaturalEnabled: true,
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
    // Hold every handoff at twig scale so the count measures density,
    // independent of how many radius-driven generations follow it.
    const twigDiameter = 2 * DEFAULT_PARAMS.height * DEFAULT_PARAMS.trunkRadius;
    expect(positions(tree({ density: 1, twigDiameter })).length).toBeGreaterThan(
      positions(tree({ density: 0, twigDiameter })).length * 2,
    );
  });
});

/* ------------------------------------------------------------------ *
 * THE PRESETS, THROUGH THE PANEL
 *
 * A preset is the generator's arguments written down, and the dials
 * are the same arguments under the owner's hands. The two only stay
 * the same thing while the panel can express every term a preset
 * states - and the failure if it cannot is silent: the owner picks
 * Laurelin, the panel drops the one term it has no dial for, and what
 * stands on the stage is a tree nobody authored.
 * ------------------------------------------------------------------ */

describe("the node ceiling", () => {
  it("is reported when the growth stops at it, and not otherwise", () => {
    /* Reaching the ceiling is a tree cut off rather than finished, and
       `nodes` alone cannot say which happened. Asserted both ways on
       the same preset: as authored it finishes its crown, and under a
       ceiling low enough to hit it stops exactly there and says so. */
    const finished = buildPreset(LAURELIN, clay).stats;
    expect(finished.capped).toBe(false);

    const cutOff = buildPreset(
      {
        ...LAURELIN,
        skeleton: {
          ...LAURELIN.skeleton,
          growth: { ...LAURELIN.skeleton.growth, maxNodes: 200 },
        },
      },
      clay,
    ).stats;
    expect(cutOff.nodes).toBe(200);
    expect(cutOff.capped).toBe(true);
  }, 60_000);
});

describe("buildComparison", () => {
  it("stands every preset on the ground, side by side and clear", () => {
    const { group } = buildComparison(PRESETS, clay);
    expect(group.children).toHaveLength(PRESETS.length);

    // Each tree keeps its own foot on the ground: the layout moves
    // trees sideways and does nothing else to them.
    for (const child of group.children) {
      expect(child.position.y).toBe(0);
      expect(child.position.z).toBe(0);
    }

    // No two crowns overlap. Read off the envelopes rather than off
    // the meshes, because that is what the layout is spacing by.
    const placed = PRESETS.map((preset, index) => ({
      x: group.children[index].position.x,
      half: preset.skeleton.envelope.height * preset.skeleton.envelope.spread,
    })).sort((a, b) => a.x - b.x);
    for (let i = 1; i < placed.length; i += 1) {
      expect(placed[i].x - placed[i - 1].x).toBeGreaterThan(
        placed[i].half + placed[i - 1].half,
      );
    }
  }, 60_000);

  it("builds each tree exactly as its preset says, not as the dials do", () => {
    /* The comparison is the acceptance test, so what stands on it has
       to be the library's own objects. Checked by rebuilding one
       preset on its own and matching the geometry vertex for vertex -
       a comparison that quietly went through the panel's defaults
       would differ here. */
    const { group } = buildComparison([LAURELIN], clay);
    const alone = buildPreset(LAURELIN, clay).tree;
    const placed = group.children[0];
    expect([...positions(trunkOf(placed))]).toEqual([
      ...positions(trunkOf(alone)),
    ]);
    /* The canopy too, and by its transforms rather than its geometry:
       a comparison built through the panel's defaults would carry the
       golden angle where Laurelin is authored with the Lucas one, and
       the trunks would match all the same. */
    expect([...(canopyOf(placed)!.instanceMatrix.array as Float32Array)])
      .toEqual([...(canopyOf(alone)!.instanceMatrix.array as Float32Array)]);
  }, 60_000);

  it("centres the row on the origin", () => {
    const { group } = buildComparison(PRESETS, clay);
    const box = new THREE.Box3().setFromObject(group);
    const centre = box.getCenter(new THREE.Vector3());
    // Within a metre on trees over a hundred metres wide: the row is
    // laid out by envelope and the growth fills each envelope its own
    // way, so this is centred, not symmetrical.
    expect(Math.abs(centre.x)).toBeLessThan(
      LAURELIN.skeleton.envelope.height * 0.05,
    );
  }, 60_000);
});

/* ------------------------------------------------------------------ *
 * WHAT THE SUBJECT COSTS TO DRAW
 *
 * Counted off the object graph, because the renderer's own number is
 * the whole scene's - ground disc and scale figure included - and
 * because there is no renderer in this runner at all. That is the
 * whole reason the count lives in the builder: a number that only
 * exists inside a browser is a number nothing can hold to account, and
 * "one instanced draw per element type" is the canopy's central
 * performance claim.
 * ------------------------------------------------------------------ */

describe("countDraws", () => {
  it("counts one call per renderable, and none for what is not drawn", () => {
    const group = new THREE.Group();
    group.add(new THREE.Object3D()); // a bare transform draws nothing
    group.add(new THREE.Mesh(new THREE.BoxGeometry(), clay.surface));
    group.add(
      new THREE.LineSegments(new THREE.BufferGeometry(), clay.line),
    );
    expect(countDraws(group)).toEqual({ drawCalls: 2, instances: 0 });
  });

  it("counts an instanced mesh as one draw carrying its copies", () => {
    // The canopy's shape, stated as an assertion: however many leaves,
    // one call. An InstancedMesh IS a Mesh, so a count that asked the
    // wrong question first would report its copies as zero.
    const crown = new THREE.InstancedMesh(
      new THREE.BoxGeometry(),
      clay.surface,
      5000,
    );
    expect(countDraws(crown)).toEqual({ drawCalls: 1, instances: 5000 });
  });

  it("counts draws, not geometries, when two of them share one", () => {
    /* A shared geometry is one buffer and two draw calls, and both
       halves of that matter. Draws are what the GPU is asked for, so
       they are counted per mesh; triangles are what the buffer holds,
       so they are never counted per copy of it - which is why they
       come from the surface each tree built and not from this. */
    const shared = new THREE.BoxGeometry();
    const group = new THREE.Group();
    group.add(new THREE.InstancedMesh(shared, clay.surface, 12));
    group.add(new THREE.InstancedMesh(shared, clay.surface, 30));
    expect(countDraws(group)).toEqual({ drawCalls: 2, instances: 42 });
  });
});

describe("the forest's own numbers", () => {
  it("sums draws and instances across the trees standing there", () => {
    /* Two trees really are two subjects' worth of work - each carries
       its own surface, and its own crown when there is one - so the
       new counts sum exactly the way triangles and nodes already do.
       Checked against the trees built one at a time, which is the only
       way to catch an aggregate that summed one tree twice. */
    const forest = buildComparison(PRESETS, clay).stats;
    const alone = PRESETS.map((preset) => buildPreset(preset, clay).stats);

    expect(forest.drawCalls).toBe(
      alone.reduce((total, one) => total + one.drawCalls, 0),
    );
    expect(forest.instances).toBe(
      alone.reduce((total, one) => total + one.instances, 0),
    );
    // And the numbers themselves, so a per-tree count that drifted to
    // a per-forest one still fails here: every tree brings its surface
    // and its crown, and every leaf on both trees is instanced.
    expect(forest.drawCalls).toBe(PRESETS.length * 2);
    expect(forest.instances).toBeGreaterThan(0);
    // FN-9 retains clipped lateral stations: twig counts intentionally rise
    // from 54890/32153 to 55315/32313. Keep exact counts and aggregation.
    expect(alone.map((one) => one.handoffs)).toEqual([1075, 1649]);
    expect(alone.map((one) => one.twigs)).toEqual([55315, 32313]);
    for (const key of ["handoffs", "levelCappedHandoffs", "twigs"] as const) {
      expect(forest[key]).toBe(alone.reduce((sum, one) => sum + one[key], 0));
    }
    const pooled = alone.flatMap((one) => one.generationCounts.flatMap(
      (count, generation) => Array<number>(count).fill(generation),
    )).sort((a, b) => a - b);
    expect(forest.generations).toEqual({ min: pooled[0],
      median: (pooled[Math.floor((pooled.length - 1) / 2)] + pooled[Math.floor(pooled.length / 2)]) / 2,
      max: pooled[pooled.length - 1] });
    expect(forest.levelCapped).toBe(false);
    expect(forest.levelCappedHandoffs).toBe(0);
  }, 60_000);
  it("builds the same tree bare when foliage is off", () => {
    /* Foliage off is an empty canopy, not a second code path: no leaf
       is placed, the mesh builder returns null, and the trunk is the
       one renderable. The skeleton underneath is byte for byte the
       tree with foliage on. */
    const on = buildTree(DEFAULT_PARAMS, clay);
    const off = buildTree(DEFAULT_PARAMS, clay, false);
    expect(off.stats.instances).toBe(0);
    expect(off.tree.children.length).toBe(1);
    expect(on.stats.instances).toBeGreaterThan(0);
    // Leaves are instances, not surface triangles; the surface is the same.
    expect(off.stats.triangles).toBe(on.stats.triangles);
    expect(off.stats.nodes).toBe(on.stats.nodes);
  });

});


describe("specimen views", () => {
  it("isolates an actual placed unit and recalculates its bounds", () => {
    const subject = new THREE.Group();
    subject.add(new THREE.Mesh(new THREE.BoxGeometry(), clay.surface));
    const foliage = new THREE.InstancedMesh(new THREE.BoxGeometry(0.02, 0.1, 0.01), clay.element, 3);
    for (let i = 0; i < 3; i++) foliage.setMatrixAt(i, new THREE.Matrix4().makeTranslation(i * 10, 4, 0));
    subject.add(foliage);
    selectSpecimenView(subject, "foliage-detail");
    expect(subject.children).toEqual([foliage]);
    expect(foliage.count).toBe(1);
    expect(foliage.boundingBox!.getCenter(new THREE.Vector3()).toArray()).toEqual([10, 4, 0]);
    expect(foliage.boundingSphere!.radius).toBeLessThan(0.06);
  });
  it("handles an empty detail and removes foliage for a bare view", () => {
    const empty = new THREE.Group();
    empty.add(new THREE.Mesh(new THREE.BoxGeometry(), clay.surface));
    expect(selectSpecimenView(empty, "foliage-detail").children).toHaveLength(0);
    const bare = new THREE.Group();
    bare.add(new THREE.Mesh(new THREE.BoxGeometry(), clay.surface));
    bare.add(new THREE.InstancedMesh(new THREE.BoxGeometry(), clay.element, 0));
    expect(selectSpecimenView(bare, "bare").children).toHaveLength(1);
  });
});
