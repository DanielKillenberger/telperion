import * as THREE from "three";
import { describe, expect, it } from "vitest";

import {
  DEFAULT_ENVELOPE,
  envelopeRadiusAt,
  type Envelope,
} from "../envelope";
import type { Skeleton } from "./colonize";
import { defaultGrowth, growSkeleton } from "./grow";

/* The spec's word is "consistently", and this is where that is held to
   account end to end: seed and envelope in, the same skeleton out, and
   the skeleton stays inside the silhouette that was authored for it. */

const PROFILE_SAMPLES = 600;

/** How far `point` lies outside the envelope solid, in metres. The
 *  solid is a body of revolution, so this is a distance in the (radius,
 *  height) half-plane, sampled along the profile. Zero inside. */
function distanceOutside(envelope: Envelope, point: THREE.Vector3): number {
  const radius = Math.hypot(point.x, point.z);
  let nearest = Number.POSITIVE_INFINITY;
  for (let i = 0; i <= PROFILE_SAMPLES; i += 1) {
    const y = (envelope.height * i) / PROFILE_SAMPLES;
    const inside = Math.min(radius, envelopeRadiusAt(envelope, y));
    nearest = Math.min(nearest, Math.hypot(radius - inside, point.y - y));
    if (nearest === 0) break;
  }
  return nearest;
}

function signature(skeleton: Skeleton): string {
  return JSON.stringify(
    skeleton.nodes.map((node) => [
      node.position.x,
      node.position.y,
      node.position.z,
      node.parent,
    ]),
  );
}

const params = { seed: 1, envelope: DEFAULT_ENVELOPE, attractors: 900 };

describe("growSkeleton", () => {
  it("grows the same skeleton, byte for byte, from the same seed", () => {
    expect(signature(growSkeleton(params))).toBe(
      signature(growSkeleton(params)),
    );
  });

  it("grows a different skeleton from a different seed", () => {
    expect(signature(growSkeleton(params))).not.toBe(
      signature(growSkeleton({ ...params, seed: 2 })),
    );
  });

  it("grows a different skeleton from a different envelope", () => {
    expect(signature(growSkeleton(params))).not.toBe(
      signature(
        growSkeleton({
          ...params,
          envelope: { ...DEFAULT_ENVELOPE, spread: 0.4 },
        }),
      ),
    );
  });

  it("stays inside the envelope it was given", () => {
    /* Not exactly inside: a growth step is taken toward attractors that
       are inside, so a node can cut a corner by part of a step. Four
       steps is under half the influence radius, which is what makes
       this a claim about the silhouette holding rather than a tolerance
       wide enough to pass anything. */
    for (const envelope of [
      DEFAULT_ENVELOPE,
      { ...DEFAULT_ENVELOPE, spread: 0.2, shoulder: 1.2 },
      { ...DEFAULT_ENVELOPE, spread: 1.4, shoulder: 4, height: 50 },
    ]) {
      const allowed = defaultGrowth(envelope).stepDistance * 4;
      for (const node of growSkeleton({ ...params, envelope }).nodes) {
        expect(distanceOutside(envelope, node.position)).toBeLessThanOrEqual(
          allowed,
        );
      }
    }
  });

  it("changes the silhouette when the envelope changes", () => {
    const widest = (envelope: Envelope): number =>
      growSkeleton({ ...params, envelope }).nodes.reduce(
        (widest, node) => Math.max(widest, Math.hypot(node.position.x, node.position.z)),
        0,
      );
    const upright = widest({ ...DEFAULT_ENVELOPE, spread: 0.2 });
    const spreading = widest({ ...DEFAULT_ENVELOPE, spread: 1.2 });
    expect(spreading).toBeGreaterThan(upright * 3);
  });

  it("branches the same way at any scale", () => {
    // Every growth distance is a fraction of height, so a 4 m tree and
    // a 60 m tree are the same tree at different sizes - the small one
    // is not a bare fork and the large one is not a solid mat.
    const nodes = (height: number): number =>
      growSkeleton({ ...params, envelope: { ...DEFAULT_ENVELOPE, height } })
        .nodes.length;
    expect(nodes(4)).toBe(nodes(60));
    expect(nodes(4)).toBe(nodes(DEFAULT_ENVELOPE.height));
  });

  it("grows a denser tree from more attractors", () => {
    const sparse = growSkeleton({ ...params, attractors: 200 });
    const dense = growSkeleton({ ...params, attractors: 1600 });
    expect(dense.nodes.length).toBeGreaterThan(sparse.nodes.length * 2);
  });

  it("takes the growth distances as arguments", () => {
    // Kill distance and influence radius are the two dials art
    // direction reaches for, so overriding either has to reach the
    // algorithm rather than being shadowed by the derived defaults.
    const derived = defaultGrowth(DEFAULT_ENVELOPE);
    expect(
      signature(growSkeleton({ ...params, growth: { influenceRadius: derived.influenceRadius * 2 } })),
    ).not.toBe(signature(growSkeleton(params)));
    expect(
      signature(growSkeleton({ ...params, growth: { killDistance: derived.killDistance * 3 } })),
    ).not.toBe(signature(growSkeleton(params)));
  });

  it("survives an envelope with nothing to fill", () => {
    const empty = growSkeleton({
      ...params,
      envelope: { ...DEFAULT_ENVELOPE, spread: 0 },
    });
    expect(empty.nodes).toHaveLength(1);
  });
});

describe("defaultGrowth", () => {
  it("scales every distance with the envelope", () => {
    const small = defaultGrowth({ ...DEFAULT_ENVELOPE, height: 10 });
    const large = defaultGrowth({ ...DEFAULT_ENVELOPE, height: 40 });
    expect(large.stepDistance).toBeCloseTo(small.stepDistance * 4, 10);
    expect(large.influenceRadius).toBeCloseTo(small.influenceRadius * 4, 10);
    expect(large.killDistance).toBeCloseTo(small.killDistance * 4, 10);
  });

  it("keeps kill distance at or above one step", () => {
    // Below one step a branch orbits an attractor it has arrived at.
    const growth = defaultGrowth(DEFAULT_ENVELOPE);
    expect(growth.killDistance).toBeGreaterThanOrEqual(growth.stepDistance);
    expect(growth.influenceRadius).toBeGreaterThan(growth.killDistance);
  });
});
