import * as THREE from "three";
import { describe, expect, it } from "vitest";

import {
  DEFAULT_ENVELOPE,
  envelopeRadiusAt,
  type Envelope,
} from "@/lib/grower/envelope";
import type { GrowthConfig, Skeleton } from "@/lib/grower/skeleton/colonize";
import { defaultGrowth, growSkeleton } from "@/lib/grower/skeleton/grow";
import { DEFAULT_BIAS, NO_BIAS, type BiasParams } from "@/lib/grower/torsion";

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

  it("keeps the trunk bare all the way to the crown base", () => {
    /* The envelope has no width below its crown base, so a branch down
       there is outside the authored silhouette however plausible it
       looks. Reaching only until some attractor is in range is not
       enough: the influence radius is wide, so the lowest attractors
       are within reach of the trunk long before it has climbed to the
       crown, and the tree starts forking at half its intended trunk
       height.

       Bare is counted as forks, not as distance from the y axis. It
       used to be the latter, which was only ever a proxy - it worked
       while the trunk was a straight vertical extrusion and stopped
       meaning anything the moment the bias field was allowed to bend
       it. A trunk that leans and S-curves up to the crown is off the
       axis at every node and has still branched nowhere. */
    const envelope = DEFAULT_ENVELOPE;
    const step = defaultGrowth(envelope).stepDistance;
    const crownBase = envelope.height * envelope.crownBase;
    for (const seed of [1, 2, 3, 4, 5]) {
      const nodes = growSkeleton({ ...params, seed, envelope }).nodes;
      const children = new Int32Array(nodes.length);
      for (const node of nodes) if (node.parent >= 0) children[node.parent] += 1;
      for (let i = 0; i < nodes.length; i += 1) {
        if (nodes[i].position.y < crownBase - step) {
          expect(children[i]).toBeLessThanOrEqual(1);
        }
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

  it("grows up: downward steps fall well below the unbiased baseline", () => {
    /* The term the spec originally missed. Without a bias field the
       generator has no notion that a tree wants to grow up, and
       branches wander back down through their own crown; both figures
       are computed here rather than pinned, so the claim is a
       comparison and not a number someone typed in.

       The conductor's own sweep of the fn-11.2 skeleton reported 22.2%
       under its metric. This one counts a step as downward when the
       child sits lower than its parent, which is stricter about what
       counts and reports about 16% for the same skeleton - the same
       finding, measured a different way. What matters is the gap. */
    const share = (
      bias: Partial<BiasParams>,
      growth?: Partial<GrowthConfig>,
    ): number => {
      const nodes = growSkeleton({ ...params, attractors: 800, bias, growth })
        .nodes;
      let down = 0;
      let steps = 0;
      for (const node of nodes) {
        if (node.parent < 0) continue;
        steps += 1;
        if (node.position.y < nodes[node.parent].position.y) down += 1;
      }
      return down / steps;
    };

    /* The fn-11.2 skeleton is the generator with no opinion about
       direction at all, and by fn-11.8 there are two rails that carry
       one: the bias field, and the persistence limit that stops a step
       reversing the step before it. Both suppress downward wander, so
       both come off to measure the baseline - with the limit still on,
       the unbiased tree already drops to 9.7% and the comparison would
       be measuring the fix against itself. */
    const baseline = share(NO_BIAS, { maxTurnPerStep: 180 });
    const biased = share(DEFAULT_BIAS);
    expect(baseline).toBeGreaterThan(0.15);
    expect(biased).toBeLessThan(baseline * 0.7);
  });

  it("bends the trunk off a straight line", () => {
    /* The bare trunk used to be a mathematically straight extrusion:
       14 nodes on one line over the lowest 7.2 m of a 24 m tree. The
       climb runs through the bias field now, so it leans and wanders -
       and it is measured as a departure from the straight line through
       its own ends, so a trunk that merely leans does not pass. */
    const envelope = DEFAULT_ENVELOPE;
    const crownBase = envelope.height * envelope.crownBase;

    const trunk = (bias: Partial<BiasParams>): THREE.Vector3[] => {
      const points = growSkeleton({ ...params, envelope, bias }).nodes
        .filter((node) => node.position.y <= crownBase)
        .map((node) => node.position);
      expect(points.length).toBeGreaterThan(8);
      return points;
    };

    /** Worst departure from the straight line through the trunk's own
     *  two ends. A trunk that only leans has a bow of zero. */
    const bow = (bias: Partial<BiasParams>): number => {
      const points = trunk(bias);
      const first = points[0];
      const axis = points[points.length - 1].clone().sub(first).normalize();
      let worst = 0;
      for (const point of points) {
        const offset = point.clone().sub(first);
        worst = Math.max(
          worst,
          offset.clone().addScaledVector(axis, -offset.dot(axis)).length(),
        );
      }
      return worst;
    };

    /** How far the trunk gets from the tree's own root axis. */
    const drift = (bias: Partial<BiasParams>): number =>
      trunk(bias).reduce(
        (worst, point) => Math.max(worst, Math.hypot(point.x, point.z)),
        0,
      );

    expect(bow(NO_BIAS)).toBeLessThan(1e-9);
    expect(bow(DEFAULT_BIAS)).toBeGreaterThan(0.2);
    // The amplitude dial is what governs how far it gets, and it has
    // headroom well past the default.
    expect(drift(NO_BIAS)).toBeLessThan(1e-9);
    expect(drift({ ...DEFAULT_BIAS, writheAmplitude: 0.25 })).toBeGreaterThan(
      drift(DEFAULT_BIAS) * 1.5,
    );
  });

  it("stays a well-formed tree however hard the field is driven", () => {
    /* The dials run past what looks good on purpose, so the invariants
       have to hold at the ceiling and not only at the defaults: nothing
       below the ground, a trunk that only ever gains height so it can
       never turn back through the crown it just left, and a parent
       always ahead of its child in the array. */
    const extremes: Partial<BiasParams>[] = [
      NO_BIAS,
      DEFAULT_BIAS,
      { ...DEFAULT_BIAS, lean: 0.5 },
      { ...DEFAULT_BIAS, writheAmplitude: 0.25, writheWavelength: 0.08 },
      { ...DEFAULT_BIAS, spiralRate: 6 },
      {
        gravitropism: 1.3,
        lean: 0.5,
        writheAmplitude: 0.25,
        writheWavelength: 0.08,
        spiralRate: 6,
      },
    ];
    const envelope = DEFAULT_ENVELOPE;
    const crownBase = envelope.height * envelope.crownBase;
    for (const bias of extremes) {
      for (const seed of [1, 2, 3]) {
        const nodes = growSkeleton({ ...params, seed, envelope, bias }).nodes;
        expect(nodes.length).toBeGreaterThan(100);
        expect(nodes.length).toBeLessThan(defaultGrowth(envelope).maxNodes);
        for (let i = 0; i < nodes.length; i += 1) {
          const node = nodes[i];
          expect(Number.isFinite(node.position.lengthSq())).toBe(true);
          expect(node.position.y).toBeGreaterThanOrEqual(0);
          expect(node.parent).toBeLessThan(i);
          if (
            node.parent >= 0 &&
            node.position.y <= crownBase &&
            nodes[node.parent].position.y <= crownBase
          ) {
            // The trunk only ever climbs, so it can never turn back
            // through the crown it just left.
            expect(node.position.y).toBeGreaterThan(
              nodes[node.parent].position.y,
            );
          }
          /* Only a parent that is itself under the crown base is held
             to that, and it is not a loosening. A limb leaving a node
             just over the line may land just under it - the line is
             not a wall and the step does not know about it - but that
             one step is as far as it gets, because the rule above then
             forbids the next one. So nothing can dive under the crown
             and keep going, which is what the rule is for; one step is
             all a graze can be. Measured across this sweep, exactly
             one such graze exists and it is 16 cm on a 24 m tree. */
        }
      }
    }
  });

  it("stays deterministic through the bias field", () => {
    const bias = { ...DEFAULT_BIAS, writheAmplitude: 0.18, spiralRate: 3 };
    expect(signature(growSkeleton({ ...params, bias }))).toBe(
      signature(growSkeleton({ ...params, bias })),
    );
    expect(signature(growSkeleton({ ...params, bias }))).not.toBe(
      signature(growSkeleton({ ...params, bias, seed: 2 })),
    );
    expect(signature(growSkeleton({ ...params, bias }))).not.toBe(
      signature(growSkeleton({ ...params, bias: DEFAULT_BIAS })),
    );
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
