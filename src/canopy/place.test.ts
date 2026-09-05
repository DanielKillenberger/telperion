import { readFileSync } from "node:fs";
import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE } from "../envelope";
import { branchPaths } from "../mesh/paths";
import { buildSurface, DEFAULT_SURFACE } from "../mesh/surface";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import { DEFAULT_RADII, solveRadii, type RadiusField } from "../radius";
import type { Skeleton } from "../skeleton/colonize";
import { DEFAULT_TWIG_ANATOMY } from "../skeleton/law";
import type { TwiggedSkeleton } from "../skeleton/twigs";
import { growSkeleton } from "../skeleton/grow";
import { buildCanopy, DEFAULT_CANOPY, type CanopyParams } from "./place";
import { shoots } from "./shoots";

/* ------------------------------------------------------------------ *
 * Whether the canopy reads as foliage is judged in clay and not here.
 * What a box with no GPU can hold to account is the mechanism: that
 * the spiral is a spiral, that the clump is at the tip, that each bias
 * moves what it says it moves, and above all that the whole thing is a
 * pure function of the seed.
 *
 * Determinism gets more than the usual attention because it fails in
 * the worst possible way: intermittently, on a machine that is not
 * yours. Nothing in the repo tested repeated builds before this, so
 * the pattern starts here rather than being copied.
 *
 * Most of the geometry below is measured on a straight vertical shoot,
 * where the answers are exact rather than approximate: the frames are
 * constant along it, the petiole offset is horizontal so arc length is
 * height, and an unbiased element axis is horizontal to the last bit.
 * ------------------------------------------------------------------ */

const at = (x: number, y: number, z: number, parent: number) => ({
  position: new THREE.Vector3(x, y, z),
  parent,
});

/** A straight vertical run four metres long, standing five metres off
 *  the tree's own axis so "outward" has a direction to mean. */
const straight: Skeleton = {
  nodes: [
    at(5, 0, 0, -1),
    at(5, 1, 0, 0),
    at(5, 2, 0, 1),
    at(5, 3, 0, 2),
    at(5, 4, 0, 3),
  ],
};

/** A trunk that forks into a thick limb and a thin one. */
const forked: Skeleton = {
  nodes: [
    at(0, 0, 0, -1),
    at(0, 1, 0, 0),
    at(0, 2, 0, 1),
    at(1, 3, 0, 2),
    at(-1, 3, 0, 2),
    at(2, 4, 0, 3),
    at(-2, 3.5, 0, 4),
  ],
};

const radii = (skeleton: Skeleton): RadiusField =>
  solveRadii(skeleton, DEFAULT_ENVELOPE, DEFAULT_RADII);

/** The whole tree is young wood, no scatter, no bias: the bare
 *  mechanism, so every angle below is exact. */
const bare: CanopyParams = {
  ...DEFAULT_CANOPY,
  shootRadius: 1,
  spacing: 0.02,
  clump: 0,
  outward: 0,
  upward: 0,
  scatter: 0,
  sizeVariation: 0,
};

/** The whole of a small fixture is young wood. The default threshold
 *  is stated against a trunk radius, and these fixtures have too few
 *  forks to run anywhere near thin enough for it - a real tree does
 *  that over a dozen forks, and `tree` below is the one that has to
 *  answer for the default. */
const dense: CanopyParams = { ...DEFAULT_CANOPY, shootRadius: 1 };

/** A real tree, grown once for the tests the fixtures cannot carry:
 *  the ones about the stated defaults, and the one about leaving the
 *  four stages before this untouched. */
const tree = (() => {
  const skeleton = growSkeleton(TELPERION.skeleton);
  return {
    skeleton,
    field: solveRadii(skeleton, TELPERION.skeleton.envelope, TELPERION.radii),
    envelope: TELPERION.skeleton.envelope,
  };
})();

function build(
  skeleton: Skeleton,
  params: CanopyParams,
  seed = 7,
  envelope = DEFAULT_ENVELOPE,
) {
  return buildCanopy(skeleton, radii(skeleton), envelope, seed, params);
}

/** One element's basis and origin, read back out of the packed
 *  column-major transforms. */
function element(matrices: Float32Array, index: number) {
  const m = new THREE.Matrix4().fromArray(matrices, index * 16);
  const basis = m.elements;
  return {
    side: new THREE.Vector3(basis[0], basis[1], basis[2]),
    axis: new THREE.Vector3(basis[4], basis[5], basis[6]),
    face: new THREE.Vector3(basis[8], basis[9], basis[10]),
    position: new THREE.Vector3(basis[12], basis[13], basis[14]),
  };
}

describe("shoots", () => {
  it("takes the young-wood end of a terminal run, through branchPaths", () => {
    /* The runs are branchPaths' own decomposition and every one of
       them already ends at a tip; a shoot is the thin end of one. So
       every shoot has to be a suffix of a run rather than some second
       walk over the raw nodes. */
    const field = radii(forked);
    const runs = branchPaths(forked, field);
    // 0.7 of the trunk: this fixture's tips sit at 0.63 of it, so the
    // threshold cuts a proper suffix out of every run rather than
    // taking the whole of one.
    const found = shoots(forked, field, field.radius[0] * 0.7);

    expect(found.length).toBe(branchPaths(forked, field).length);
    for (const shoot of found) {
      expect(shoot.nodes.length).toBeGreaterThanOrEqual(2);
      const run = runs.find(
        (candidate) =>
          candidate.nodes[candidate.nodes.length - 1] ===
          shoot.nodes[shoot.nodes.length - 1],
      );
      expect(run).toBeDefined();
      expect(run?.nodes.slice(-shoot.nodes.length)).toEqual(shoot.nodes);
      // A run ends where the wood runs out, which is what "terminal"
      // means: nothing in the skeleton grows out of the last node.
      const tip = shoot.nodes[shoot.nodes.length - 1];
      expect(forked.nodes.some((node) => node.parent === tip)).toBe(false);
    }
  });

  it("inherits the zero-length fold rather than re-deriving it", () => {
    // A repeated node is drawn at its parent's position. branchPaths
    // resolves that in its stands[] pass; a shoot carrying the folded
    // index would be a second opinion about where the wood is.
    const doubled: Skeleton = {
      nodes: [at(0, 0, 0, -1), at(0, 0, 0, 0), at(0, 1, 0, 1)],
    };
    const field = radii(doubled);
    expect(shoots(doubled, field, field.radius[0])).toEqual([{ nodes: [0, 2] }]);
  });

  it("finds nothing on wood that never got thin", () => {
    const field = radii(forked);
    expect(shoots(forked, field, 0)).toEqual([]);
  });
});

describe("buildCanopy", () => {
  it("turns each element by the divergence angle, along the shoot", () => {
    /* Phyllotaxis is the whole reason placement is not one element per
       attachment frame. Ninety degrees rather than the golden angle
       because a right angle is checkable exactly: consecutive
       elements are perpendicular, and every second one is reversed. */
    const canopy = build(straight, { ...bare, divergence: 90 });
    expect(canopy.count).toBeGreaterThan(4);

    for (let i = 1; i < canopy.count; i += 1) {
      const previous = element(canopy.matrices, i - 1).axis.normalize();
      const current = element(canopy.matrices, i).axis.normalize();
      expect(previous.dot(current)).toBeCloseTo(0, 6);
      if (i >= 2) {
        const before = element(canopy.matrices, i - 2).axis.normalize();
        expect(before.dot(current)).toBeCloseTo(-1, 6);
      }
    }
  });

  it("gathers a clump into the last stretch of every shoot", () => {
    const span = 0.25;
    const spaced = build(straight, { ...bare, clump: 0 });
    const clumped = build(straight, { ...bare, clump: 4, clumpSpan: span });

    // One shoot, so the clump is exactly four more elements.
    expect(clumped.count).toBe(spaced.count + 4);

    /* The shoot runs from y=0 to y=4 and the petiole offset is
       horizontal, so arc length along it IS height. The clump is the
       last four emitted. */
    for (let i = clumped.count - 4; i < clumped.count; i += 1) {
      expect(element(clumped.matrices, i).position.y).toBeGreaterThanOrEqual(
        4 * (1 - span) - 1e-6,
      );
    }
  });

  it.each([
    ["outward", "x" as const, { outward: 1 }],
    ["upward", "y" as const, { upward: 1 }],
  ])("turns elements %s when the bias asks", (_name, component, bias) => {
    /* Unbiased, the axis is radial about a vertical shoot: no vertical
       component at all, and a full spiral's worth of horizontal ones
       that cancel. Either bias has to move its own component and
       nothing else has to move it. */
    const neutral = build(straight, bare);
    const biased = build(straight, { ...bare, ...bias });
    expect(biased.count).toBe(neutral.count);

    const mean = (canopy: typeof neutral) => {
      let total = 0;
      for (let i = 0; i < canopy.count; i += 1) {
        total += element(canopy.matrices, i).axis.normalize()[component];
      }
      return total / canopy.count;
    };

    expect(Math.abs(mean(neutral))).toBeLessThan(0.2);
    expect(mean(biased)).toBeGreaterThan(0.5);
  });

  it.each([
    ["a skeleton with one node", { nodes: [at(0, 0, 0, -1)] }],
    ["a skeleton with no nodes", { nodes: [] }],
    // Two nodes, but the second stands where the first does, so the
    // fold leaves the root with no children and there is no run.
    [
      "a skeleton with no terminal runs",
      { nodes: [at(0, 0, 0, -1), at(0, 0, 0, 0)] },
    ],
  ])("yields an empty canopy for %s, without throwing", (_name, skeleton) => {
    const canopy = build(skeleton as Skeleton, DEFAULT_CANOPY);
    expect(canopy.count).toBe(0);
    expect(canopy.matrices).toHaveLength(0);
  });

  it("yields an empty canopy when no wood is thin enough to bear it", () => {
    const canopy = build(forked, { ...DEFAULT_CANOPY, shootRadius: 0 });
    expect(canopy.count).toBe(0);
    expect(canopy.matrices).toHaveLength(0);
  });

  it("falls back to the defaults for every non-finite parameter", () => {
    /* The panel can send through a NaN, and a NaN transform reaches
       the screen as an element that is silently not drawn. Every dial
       is pinned on the same rail the other stages use. */
    const nonFinite: CanopyParams = {
      shootRadius: Number.NaN,
      spacing: Number.NaN,
      divergence: Number.POSITIVE_INFINITY,
      clump: Number.NaN,
      clumpSpan: Number.NEGATIVE_INFINITY,
      outward: Number.NaN,
      upward: Number.NaN,
      scatter: Number.NaN,
      size: Number.NaN,
      sizeVariation: Number.NaN,
    };
    const fallback = buildCanopy(
      tree.skeleton,
      tree.field,
      { ...tree.envelope, height: Number.NaN },
      3,
      nonFinite,
    );
    // The envelope height falls back the same way, so the stated side
    // states the height the rail would have chosen.
    const stated = buildCanopy(
      tree.skeleton,
      tree.field,
      { ...tree.envelope, height: DEFAULT_ENVELOPE.height },
      3,
      DEFAULT_CANOPY,
    );

    expect(fallback.count).toBe(stated.count);
    expect(fallback.count).toBeGreaterThan(0);
    expect(Array.from(fallback.matrices)).toEqual(Array.from(stated.matrices));
  });

  it("emits a similarity transform per element, all of it finite", () => {
    /* An element is placed, turned and sized - never sheared and never
       mirrored, either of which would show as a leaf inside out. The
       three columns stay perpendicular and equal in length, and that
       length stays inside the size band. */
    const canopy = build(forked, dense);
    expect(canopy.count).toBeGreaterThan(0);
    expect(canopy.matrices).toHaveLength(canopy.count * 16);

    const size = DEFAULT_CANOPY.size;
    for (let i = 0; i < canopy.count; i += 1) {
      const { side, axis, face, position } = element(canopy.matrices, i);
      for (const value of [...side.toArray(), ...position.toArray()]) {
        expect(Number.isFinite(value)).toBe(true);
      }
      const scale = axis.length();
      expect(scale).toBeCloseTo(side.length(), 5);
      expect(scale).toBeCloseTo(face.length(), 5);
      expect(scale).toBeGreaterThanOrEqual(
        size * (1 - DEFAULT_CANOPY.sizeVariation),
      );
      expect(scale).toBeLessThan(size * (1 + DEFAULT_CANOPY.sizeVariation));
      expect(side.dot(axis) / (scale * scale)).toBeCloseTo(0, 5);
      expect(axis.dot(face) / (scale * scale)).toBeCloseTo(0, 5);
      // Right-handed: side is axis crossed into face, not against it.
      expect(
        axis.clone().cross(face).normalize().dot(side.clone().normalize()),
      ).toBeCloseTo(1, 5);
    }
  });

  it("scales the element it is given, not the tree it is on", () => {
    /* `size` multiplies the element's own authored dimensions. It is
       not a fraction of envelope height, and this test is here because
       it once was: placement and the element each stated a scale
       convention, the two disagreed, and nothing composed them until
       the draw. A leaf does not grow because its tree is tall - a
       148 m tree carrying a leaf sized off its own height would be
       carrying 1.5 m fronds - so the same dials on a tall tree and a
       short one give the same scale, and only `size` moves it. */
    const scaleOf = (height: number, size: number): number => {
      const canopy = build(
        forked,
        { ...dense, size, sizeVariation: 0 },
        7,
        { ...DEFAULT_ENVELOPE, height },
      );
      expect(canopy.count).toBeGreaterThan(0);
      return element(canopy.matrices, 0).axis.length();
    };

    expect(scaleOf(24, 1)).toBeCloseTo(1, 6);
    expect(scaleOf(148, 1)).toBeCloseTo(1, 6);
    expect(scaleOf(148, 2.5)).toBeCloseTo(2.5, 6);
  });
});

describe("determinism", () => {
  it("builds the same canopy twice from one seed", () => {
    const first = build(forked, dense, 11);
    const second = build(forked, dense, 11);
    expect(first.count).toBe(second.count);
    expect(first.count).toBeGreaterThan(0);
    expect(Array.from(first.matrices)).toEqual(Array.from(second.matrices));
  });

  it("gives two seeds two canopies, so two trees do not correlate", () => {
    const one = build(forked, dense, 11);
    const other = build(forked, dense, 12);
    expect(Array.from(one.matrices)).not.toEqual(Array.from(other.matrices));
  });

  it("iterates arrays, never a Map or a Set", () => {
    /* Not a style rule. Hash order is stable within a run and this
       would pass anyway; it is the kind of dependency that surfaces
       once in twenty builds on somebody else's machine, and the only
       cheap guard against it is that the construct is not there. */
    for (const file of ["place.ts", "shoots.ts"]) {
      const source = readFileSync(new URL(file, import.meta.url), "utf8");
      expect(source).not.toMatch(/new (Map|Set|WeakMap|WeakSet)\b/);
    }
  });

  it("leaves the stages before it byte-identical", () => {
    /* The canopy is a fifth stage the other four never call, which is
       what keeps their own comparisons valid. It also has to leave
       their inputs alone: a canopy that normalised a node's position
       in place would move the surface under it. */
    const { skeleton, field } = tree;
    const before = buildSurface(
      skeleton,
      field,
      TELPERION.skeleton.envelope,
      DEFAULT_SURFACE,
    );
    const nodes = skeleton.nodes.map((node) => node.position.toArray());
    const radiusBefore = Array.from(field.radius);

    const canopy = buildCanopy(
      skeleton,
      field,
      TELPERION.skeleton.envelope,
      TELPERION.skeleton.seed,
      TELPERION.canopy,
    );
    expect(canopy.count).toBeGreaterThan(0);

    expect(skeleton.nodes.map((node) => node.position.toArray())).toEqual(nodes);
    expect(Array.from(field.radius)).toEqual(radiusBefore);
    const after = buildSurface(
      skeleton,
      field,
      TELPERION.skeleton.envelope,
      DEFAULT_SURFACE,
    );
    expect(Array.from(after.positions)).toEqual(Array.from(before.positions));
  }, 60_000);
});

describe("the two trees", () => {
  it.each([
    ["Telperion", TELPERION],
    ["Laurelin", LAURELIN],
  ])("%s states its canopy in full", (_name, preset) => {
    // Every term stated, none inherited: the preset convention, which
    // a canopy the presets did not name would have broken.
    for (const term of Object.keys(DEFAULT_CANOPY) as (keyof CanopyParams)[]) {
      expect(Number.isFinite(preset.canopy[term])).toBe(true);
    }
  });

  it.each([
    ["Telperion", TELPERION],
    ["Laurelin", LAURELIN],
  ])("%s carries foliage, and none of it below its crown", (_name, preset) => {
    /* The shoot threshold is the one term here that can silently find
       nothing - it is measured against a trunk radius the preset does
       not state directly - so both trees are checked for a canopy that
       exists, sits inside the crown, and does not clothe the bare
       trunk the references make so much of. */
    const skeleton = growSkeleton(preset.skeleton);
    const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);
    const canopy = buildCanopy(
      skeleton,
      field,
      preset.skeleton.envelope,
      preset.skeleton.seed,
      preset.canopy,
    );

    expect(canopy.count).toBeGreaterThan(500);
    const base =
      preset.skeleton.envelope.height * preset.skeleton.envelope.crownBase;
    // Counted rather than asserted per element: the shipped crowns
    // carry hundreds of thousands, and one expectation each is what
    // made this test time out rather than fail.
    let below = 0;
    for (let i = 0; i < canopy.count; i += 1) {
      if (element(canopy.matrices, i).position.y <= base) below += 1;
    }
    expect(below).toBe(0);
  }, 60_000);

  it("thins the whole shoot when it saturates, never bares the tip", () => {
    /* The per-shoot cap is a stop, and a stop has to take from
       everywhere. Stations accumulate from the shoot's base, so a cap
       on the walk's loop condition truncates the DISTAL end: it strips
       the growing tip - this season's leaves, and where the clump goes
       - and leaves the old wood behind it fully clothed. That inverts
       what a shoot means, and it is reachable from the panel by
       widening `shootRadius` and tightening `spacing`. */
    const saturating: CanopyParams = {
      ...dense,
      spacing: 0.001,
      clump: 8,
      clumpSpan: 0.25,
      scatter: 0,
      sizeVariation: 0,
    };
    /* Spacing is a fraction of envelope height and floored as one, so
       a short envelope over this four-metre shoot is what drives the
       station count past the cap: 0.001 x 4 m asks for a thousand. */
    const canopy = build(straight, saturating, 7, {
      ...DEFAULT_ENVELOPE,
      height: 4,
    });

    // The shoot runs the fixture's full height; foliage must reach it.
    const tip = straight.nodes[straight.nodes.length - 1].position.y;
    const foot = straight.nodes[0].position.y;
    let highest = -Infinity;
    for (let i = 0; i < canopy.count; i += 1) {
      highest = Math.max(highest, element(canopy.matrices, i).position.y);
    }

    expect(canopy.count).toBeGreaterThan(400);
    // Within one widened step of the tip, not stranded down the stem.
    expect(highest).toBeGreaterThan(foot + (tip - foot) * 0.95);
  });

  it("keeps every transform finite however large `size` is asked to be", () => {
    /* `held` catches a NaN and not a finite enormity. Unrailed, a huge
       multiplier arrives through the float32 multiply as Infinity; the
       culler's fail-safe predicate then KEEPS every one of those
       elements, and the instanced mesh's bounds take the room's
       framing with them. `buildCanopy` is a public export, so the
       panel's own slider range is not the bound that matters. */
    for (const size of [1e6, 1e30, 1e39, Number.MAX_VALUE]) {
      const canopy = build(forked, { ...dense, size });
      expect(canopy.count).toBeGreaterThan(0);
      for (let i = 0; i < canopy.matrices.length; i += 1) {
        expect(Number.isFinite(canopy.matrices[i])).toBe(true);
      }
    }
  });
});

describe("twig anatomy placement", () => {
  const twigged: TwiggedSkeleton = {
    nodes: [at(0, 0, 0, -1), at(0, 1, 0, 0), at(0, 1.04, 0, 1), at(1, 2, 0, 1)],
    crossover: 2, branchId: new Int32Array([2, 3]),
    baseRadius: new Float64Array([0.0025, 0.1]), endRadius: new Float64Array([0.0025, 0.1]), twig: new Uint8Array([1, 0]),
    levelCapped: false, nodeCapped: false,
  };
  const field: RadiusField = {
    radius: new Float64Array([1, 0.5, 0.0025, 0.1]),
    startRadius: new Float64Array([1, 1, 0.0025, 0.1]),
  };

  it("places only marked edges at fixed internodes, on their own wood surface", () => {
    const params = { ...bare, shootRadius: 0, clump: 64 };
    const anatomy = { ...DEFAULT_TWIG_ANATOMY, stationsPerInternode: 2 };
    const canopy = buildCanopy(twigged, field, DEFAULT_ENVELOPE, 7, params, anatomy);
    expect(canopy.count).toBe(4);
    expect(element(canopy.matrices, 0).axis.dot(element(canopy.matrices, 1).axis))
      .toBeCloseTo(-1, 6);
    for (let i = 0; i < canopy.count; i++) {
      const point = element(canopy.matrices, i).position;
      expect(point.y).toBeCloseTo(1 + Math.floor(i / 2) * 0.02, 6);
      expect(Math.hypot(point.x, point.z)).toBeCloseTo(0.0025, 7);
    }
    expect(buildCanopy(twigged, field, { ...DEFAULT_ENVELOPE, height: 500 }, 7, params, anatomy))
      .toEqual(canopy);
    expect(buildCanopy(twigged, field, DEFAULT_ENVELOPE, 7, params,
      { ...anatomy, internodeLength: 0.01 }).count).toBe(8);
    expect(buildCanopy({ ...twigged, twig: new Uint8Array(2) }, field,
      DEFAULT_ENVELOPE, 7, params, anatomy).count).toBe(0);
  });

  it("walks the full single-edge twig with continuous phyllotaxis and a distal budget", () => {
    const tree = { ...twigged, nodes: [...twigged.nodes] };
    tree.nodes[2] = at(0, 1.25, 0, 1);
    const params = { ...bare, divergence: 90 };
    const canopy = buildCanopy(tree, field, DEFAULT_ENVELOPE, 7, params, DEFAULT_TWIG_ANATOMY);
    expect(canopy.count).toBe(13);
    for (let i = 0; i < canopy.count; i++) {
      expect(element(canopy.matrices, i).position.y).toBeCloseTo(1 + i * 0.02, 6);
      if (i > 0) expect(element(canopy.matrices, i).axis.dot(element(canopy.matrices, i - 1).axis)).toBeCloseTo(0, 6);
    }
    const crowded = buildCanopy(tree, field, DEFAULT_ENVELOPE, 7, params,
      { ...DEFAULT_TWIG_ANATOMY, internodeLength: 0.00001 });
    expect(crowded.count).toBe(512);
    expect(element(crowded.matrices, 511).position.y).toBeGreaterThan(1.249);
  });

  it("retains the shoot rule without anatomy or without twig records", () => {
    const plain = { nodes: twigged.nodes };
    const expected = buildCanopy(plain, field, DEFAULT_ENVELOPE, 7, bare);
    expect(expected.count).toBeGreaterThan(0);
    expect(buildCanopy(twigged, field, DEFAULT_ENVELOPE, 7, bare)).toEqual(expected);
    expect(buildCanopy(plain, field, DEFAULT_ENVELOPE, 7, bare, DEFAULT_TWIG_ANATOMY))
      .toEqual(expected);
  });
});
