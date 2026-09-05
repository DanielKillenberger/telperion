import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE, sampleEnvelope } from "../envelope";
import { TELPERION } from "../presets/two-trees";
import { solveRadii } from "../radius";
import { resolveGrowth } from "../skeleton/grow";
import { colonize } from "../skeleton/colonize";
import { createRng } from "../rng";
import { cullCanopy, DEFAULT_CULL } from "./cull";
import { buildElement, DEFAULT_ELEMENT } from "./element";
import { buildCanopy, type Canopy } from "./place";
import {
  screenFor,
  silhouetteChange,
  silhouetteOf,
  silhouettePixelFloor,
  SILHOUETTE_VIEWS,
} from "./silhouette";

/* ------------------------------------------------------------------ *
 * Culling is a bet that the elements it removes were never on screen,
 * and the only way to hold that bet to account without a GPU is to
 * draw the outline twice and compare it. So the silhouette helper is
 * the instrument here and the culler is what is being measured.
 *
 * THE FIXTURE HAS TO BE WRONG ON PURPOSE. A canopy placed the way the
 * presets place it is most of the way to being a shell already -
 * foliage grows on distal shoots and distal shoots are near the
 * crown's surface - so a culler tested against one would look like it
 * works while doing almost nothing. The tree below is grown with every
 * limb bearing foliage, trunk included, at twice the preset's density:
 * the botanically wrong canopy that fills its envelope solid, which is
 * the one culling exists for and the one where a culler that does
 * nothing has nowhere to hide.
 *
 * TWO CULLERS ARE BEING EXCLUDED, and each has a test named for it: a
 * culler that removes nothing, which the count-falls assertion kills,
 * and a culler that removes everything, which the count-remains and
 * the silhouette assertions both kill. A test that only checked the
 * count fell would pass on the second one.
 *
 * THE TOLERANCE IS THE RASTER'S OWN RESOLUTION and nothing softer. It
 * is not a slack band chosen to fit the culler; it is the smallest
 * change the measurement can distinguish from where it happened to put
 * its pixel boundaries, and the culler's one dial was set against it
 * rather than the other way round. A tolerance sized by the thing it
 * protects rather than by the failure it corrects is how this repo
 * lost its orbit pivot once already.
 * ------------------------------------------------------------------ */

const element = buildElement(DEFAULT_ELEMENT);

/** A tree carrying the canopy culling exists to fix: every limb
 *  bearing foliage down to the trunk, at twice the preset's density,
 *  filling the envelope rather than clothing it. */
const filled = (() => {
  const preset = TELPERION;
  // Colonization alone: the shipped preset's twigs are already shed to
  // a shell, and a canopy placed on a shell is not the filled crown
  // this fixture exists to be.
  const tree = preset.skeleton;
  const attractors = sampleEnvelope(tree.envelope, tree.attractors, createRng(tree.seed));
  const skeleton = colonize(attractors, new THREE.Vector3(), resolveGrowth(tree, attractors.length));
  const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);
  const envelope = preset.skeleton.envelope;
  return {
    envelope,
    canopy: buildCanopy(skeleton, field, envelope, 7, {
      ...preset.canopy,
      spacing: preset.canopy.spacing / 2,
      shootRadius: 1,
    }),
  };
})();

const cull = (canopy: Canopy, envelope = filled.envelope, params = DEFAULT_CULL) =>
  cullCanopy(canopy, element, envelope, params);

/** A canopy built by hand out of whole transforms. */
const packed = (transforms: THREE.Matrix4[]): Canopy => {
  const matrices = new Float32Array(transforms.length * 16);
  for (let i = 0; i < transforms.length; i += 1) {
    matrices.set(transforms[i].elements, i * 16);
  }
  return { matrices, count: transforms.length };
};

/** One element's transform: petiole at `position`, blade running along
 *  `along`, scaled by `scale`. The basis is right-handed and
 *  orthogonal, like the ones placement emits. */
const at = (
  position: THREE.Vector3,
  along: THREE.Vector3,
  scale: number,
): THREE.Matrix4 => {
  const axis = along.clone().normalize();
  const face = new THREE.Vector3(0, 1, 0);
  if (Math.abs(face.dot(axis)) > 0.9) face.set(0, 0, 1);
  face.addScaledVector(axis, -face.dot(axis)).normalize();
  const side = new THREE.Vector3().crossVectors(axis, face).normalize();
  return new THREE.Matrix4()
    .makeBasis(
      side.multiplyScalar(scale),
      axis.clone().multiplyScalar(scale),
      face.multiplyScalar(scale),
    )
    .setPosition(position);
};

/** Where the whole of one element's geometry ends up, averaged - the
 *  centroid a cheaper classifier would test instead of the vertices. */
const centroidOf = (canopy: Canopy, index: number): THREE.Vector3 => {
  const m = new THREE.Matrix4().fromArray(canopy.matrices, index * 16);
  const sum = new THREE.Vector3();
  const vertices = element.positions.length / 3;
  for (let v = 0; v < vertices; v += 1) {
    sum.add(
      new THREE.Vector3(
        element.positions[v * 3],
        element.positions[v * 3 + 1],
        element.positions[v * 3 + 2],
      ).applyMatrix4(m),
    );
  }
  return sum.divideScalar(vertices);
};

describe("cullCanopy", () => {
  it("removes a substantial part of a canopy that fills its envelope", () => {
    const culled = cull(filled.canopy);

    // The fixture's own premise: enough elements that there is an
    // interior to speak of.
    expect(filled.canopy.count).toBeGreaterThan(2000);
    // A culler that removes nothing dies here.
    expect(culled.count).toBeLessThan(filled.canopy.count * 0.9);
  });

  it("leaves a canopy behind", () => {
    const culled = cull(filled.canopy);

    // A culler that removes everything dies here - and the count alone
    // would not have caught it, which is why this is its own test.
    expect(culled.count).toBeGreaterThan(0);
    expect(culled.count).toBeGreaterThan(filled.canopy.count / 2);
  });

  it("leaves the silhouette where it was, from every judged direction", () => {
    const culled = cull(filled.canopy);

    for (let v = 0; v < SILHOUETTE_VIEWS.length; v += 1) {
      const screen = screenFor(element, filled.canopy, SILHOUETTE_VIEWS[v]);
      const before = silhouetteOf(element, filled.canopy, screen);
      const after = silhouetteOf(element, culled, screen);

      /* An outline that is not there cannot be shown to be unchanged.
         Both guards are the same lesson the envelope learned: a
         predicate written over a width is vacuous where the width is
         zero, and this one is judged over a shape big enough to have
         a boundary worth arguing about. */
      expect(before.area).toBeGreaterThan(10_000);
      expect(before.columns).toBeGreaterThan(100);
      expect(after.area).toBeGreaterThan(0);

      expect(silhouetteChange(before, after).fraction).toBeLessThanOrEqual(
        silhouettePixelFloor(before),
      );
    }
  });

  it("classifies by every vertex, never by the centroid", () => {
    /* One blade nearly five metres long, lying along +x at the widest
       height of a 24 m envelope: its petiole is on the axis, deep
       inside, and its tip reaches into the shell. Every vertex is what
       decides it. */
    const y = DEFAULT_ENVELOPE.height * 0.615;
    const straddling = packed([
      at(new THREE.Vector3(0, y, 0), new THREE.Vector3(1, 0, 0), 40),
    ]);
    /* The same element shrunk to a speck and parked at that blade's
       own centroid: the single point a centroid classifier would test
       in its place. */
    const centre = packed([
      at(centroidOf(straddling, 0), new THREE.Vector3(1, 0, 0), 0.01),
    ]);

    expect(cull(centre, DEFAULT_ENVELOPE).count).toBe(0);
    expect(cull(straddling, DEFAULT_ENVELOPE).count).toBe(1);
  });

  it("keeps foliage hanging under the crown's own belly", () => {
    /* Just inside the crown base and close to the trunk. The crown is
       wide at that height, so read radially this leaf is four metres
       from the nearest wood and deeply interior - but it is under a
       metre from the underside of the envelope, in full view of
       anything looking up at the tree, and culling it would lift the
       whole bottom edge of the silhouette. Depth is distance to the
       surface, not slack in the radius, and this is the difference. */
    const belly = packed([
      at(new THREE.Vector3(0.2, 8, 0), new THREE.Vector3(1, 0, 0), 0.01),
    ]);

    expect(cull(belly, DEFAULT_ENVELOPE).count).toBe(1);
  });

  it("keeps foliage where the envelope has no width to judge it by", () => {
    // Below the crown base and above the tip the profile's radius is
    // zero, and a depth test written over that width alone says
    // nothing there. It has to say "keep".
    const outside = packed([
      at(new THREE.Vector3(0, 1, 0), new THREE.Vector3(1, 0, 0), 1),
      at(
        new THREE.Vector3(0, DEFAULT_ENVELOPE.height + 1, 0),
        new THREE.Vector3(1, 0, 0),
        1,
      ),
    ]);

    expect(cull(outside, DEFAULT_ENVELOPE).count).toBe(2);
  });

  it("keeps an element whose transform carries a non-finite number", () => {
    const deep = at(
      new THREE.Vector3(0, DEFAULT_ENVELOPE.height * 0.615, 0),
      new THREE.Vector3(1, 0, 0),
      1,
    );
    expect(cull(packed([deep]), DEFAULT_ENVELOPE).count).toBe(0);

    const broken = packed([deep]);
    broken.matrices[13] = Number.NaN;

    // An element that cannot be classified is not thereby interior.
    expect(cull(broken, DEFAULT_ENVELOPE).count).toBe(1);
  });

  it("falls back to the stated shell for a non-finite depth", () => {
    const stated = cull(filled.canopy, filled.envelope, DEFAULT_CULL);

    for (const depth of [Number.NaN, Infinity, -Infinity]) {
      const held = cull(filled.canopy, filled.envelope, { shellDepth: depth });
      expect(held.count).toBe(stated.count);
      expect(held.matrices).toEqual(stated.matrices);
    }
  });

  it("yields an empty canopy for an empty one, without throwing", () => {
    const culled = cull({ matrices: new Float32Array(0), count: 0 });
    expect(culled.count).toBe(0);
    expect(culled.matrices.length).toBe(0);
  });

  it("keeps what it keeps unchanged, in order, float for float", () => {
    const culled = cull(filled.canopy);

    // Every kept transform is one of the originals, in the order
    // placement emitted them: culling selects, it never rewrites.
    let source = 0;
    for (let i = 0; i < culled.count; i += 1) {
      while (
        source < filled.canopy.count &&
        !filled.canopy.matrices
          .subarray(source * 16, source * 16 + 16)
          .every((value, k) => value === culled.matrices[i * 16 + k])
      ) {
        source += 1;
      }
      expect(source).toBeLessThan(filled.canopy.count);
      source += 1;
    }

    // And it is a pure function of what it was given.
    expect(cull(filled.canopy).matrices).toEqual(culled.matrices);
  });
});

describe("silhouette", () => {
  it("finds nothing lost when a canopy is compared with itself", () => {
    const screen = screenFor(element, filled.canopy, SILHOUETTE_VIEWS[0]);
    const before = silhouetteOf(element, filled.canopy, screen);

    const change = silhouetteChange(before, before);
    expect(before.area).toBeGreaterThan(0);
    expect(change.lost).toBe(0);
    expect(change.fraction).toBe(0);
  });

  it("calls a comparison against nothing a total loss, not a pass", () => {
    const screen = screenFor(element, filled.canopy, SILHOUETTE_VIEWS[0]);
    const before = silhouetteOf(element, filled.canopy, screen);
    const nothing = silhouetteOf(
      element,
      { matrices: new Float32Array(0), count: 0 },
      screen,
    );

    expect(silhouetteChange(before, nothing).fraction).toBe(1);
    // And a silhouette that was never there is not evidence of
    // anything either: 0 of 0 lost reads as a total loss, not as
    // "unchanged".
    expect(silhouetteChange(nothing, nothing).fraction).toBe(1);
  });

  it("puts an element where its transform put it", () => {
    const canopy = packed([
      at(new THREE.Vector3(0, 12, 0), new THREE.Vector3(1, 0, 0), 40),
      at(new THREE.Vector3(0, 12, 0), new THREE.Vector3(-1, 0, 0), 40),
    ]);
    // Looking along -z, so screen x runs with world x and the two
    // blades land on opposite sides of the frame.
    const screen = screenFor(element, canopy, SILHOUETTE_VIEWS[2]);
    const both = silhouetteOf(element, canopy, screen);

    const left = silhouetteOf(element, packed([at(new THREE.Vector3(0, 12, 0), new THREE.Vector3(-1, 0, 0), 40)]), screen);
    expect(both.columns).toBeGreaterThan(left.columns);
    expect(silhouetteChange(both, left).fraction).toBeGreaterThan(0.4);
  });
});
