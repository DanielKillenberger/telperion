import * as THREE from "three";
import { describe, expect, it } from "vitest";

import {
  DEFAULT_ENVELOPE,
  envelopeRadiusAt,
  type Envelope,
} from "../envelope";
import type { GrowthConfig, Skeleton } from "./colonize";
import {
  DEFAULT_STEP,
  defaultGrowth,
  growSkeleton,
  influenceRadiusFor,
  type SkeletonParams,
  growReport,
  resolveGrowth,
} from "./grow";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import { DEFAULT_BIAS, NO_BIAS, type BiasParams } from "../torsion";

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

/** Node count and terminal-run count: the two numbers a starved tree
 *  cannot fake. A collapse is a few dozen nodes on a handful of tips
 *  whatever else is measured about it. */
function census(skeleton: Skeleton): { nodes: number; tips: number } {
  const nodes = skeleton.nodes;
  const children = new Int32Array(nodes.length);
  for (const node of nodes) if (node.parent >= 0) children[node.parent] += 1;
  let tips = 0;
  for (let i = 1; i < nodes.length; i += 1) if (children[i] === 0) tips += 1;
  return { nodes: nodes.length, tips };
}

/** Growth distances for a step other than the default, derived the way
 *  a depth dial will derive them: step and kill distance scaled
 *  together, the search radius recomputed from the step and the
 *  attractor count rather than scaled with them. */
function growthAtStep(
  envelope: Envelope,
  step: number,
  attractors: number,
): Partial<GrowthConfig> {
  return {
    stepDistance: step,
    killDistance: step * 2,
    influenceRadius: influenceRadiusFor(envelope, step, attractors),
    // Lifted so that a fine step is measured, not truncated.
    maxNodes: 60000,
  };
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

/** A preset's colonization alone, its second pass stated at zero
 *  orders. Both presets ship with orders now, and the tests below
 *  reason about the base tree the twigs are appended to, so that base
 *  has to be asked for rather than assumed. */
const bare = (tree: SkeletonParams): SkeletonParams => ({
  ...tree,
  twigs: { ...tree.twigs, levels: 0 },
});

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

  it("branches the same way at every height on the dial", () => {
    /* Every growth distance is a fraction of height, so the tree at
       the bottom of the height dial and the tree at the top of it are
       one tree at two sizes - the small one is not a bare fork and the
       large one is not a solid mat. The band is the dial's, 4 m to
       400 m, and not the 60 m the dial used to stop at. */
    const nodes = (height: number): number =>
      growSkeleton({ ...params, envelope: { ...DEFAULT_ENVELOPE, height } })
        .nodes.length;
    const sapling = nodes(4);
    for (const height of [DEFAULT_ENVELOPE.height, 60, 150, 400]) {
      expect(nodes(height), `${height} m`).toBe(sapling);
    }
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
          if (node.parent >= 0 && node.position.y <= crownBase) {
            // The trunk only ever climbs, and nothing descends into the
            // bare-trunk region from the crown above it either: the
            // envelope has no width down there, so a step that would
            // land under the line is refused as no progress at all.
            expect(node.position.y).toBeGreaterThan(
              nodes[node.parent].position.y,
            );
          }
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

describe("the search radius and the attractor spacing", () => {
  it("resolves to nine steps at today's step, on both presets and every fixture above the floor", () => {
    /* R7, as narrowed when the floor was raised to 2.0 spacings: both
       presets as shipped, and every fixture whose nine-step radius is
       wider than the floor, are the same tree byte for byte. Not
       "close" - or a later change in any of them is no longer
       attributable to a dial. The two fixtures the floor does move are
       asserted the other way round in the next test, so the narrowing
       is written down rather than silent. */
    const trees: SkeletonParams[] = [
      TELPERION.skeleton,
      LAURELIN.skeleton,
      params,
      { ...params, attractors: 200 },
      { ...params, attractors: 800 },
      { ...params, attractors: 1600 },
      { ...params, envelope: { ...DEFAULT_ENVELOPE, spread: 0.2 } },
      { ...params, envelope: { ...DEFAULT_ENVELOPE, spread: 0.2, shoulder: 1.2 } },
    ];
    for (const tree of trees) {
      const nineSteps = defaultGrowth(tree.envelope).stepDistance * 9;
      expect(
        signature(growSkeleton(tree)),
        `${tree.envelope.height} m, ${tree.attractors} attractors`,
      ).toBe(
        signature(
          growSkeleton({
            ...tree,
            growth: { ...tree.growth, influenceRadius: nineSteps },
          }),
        ),
      );
    }
  });

  it("lets the floor bind on the two fixtures R7 no longer covers, and only there", () => {
    /* The other half of the narrowing. On the suite's two sparsest
       scatters nine steps is 1.229 and 1.463 spacings, under the 2.0
       floor, so the derived radius is wider than nine steps and the
       tree they grow is a different tree. That is the cost of R4's
       margin and it is stated here so it can never be mistaken for
       drift. Both presets stay above the floor. */
    const moved: SkeletonParams[] = [
      { ...params, envelope: { ...DEFAULT_ENVELOPE, spread: 1.2 } },
      { ...params, envelope: { ...DEFAULT_ENVELOPE, spread: 1.4, shoulder: 4, height: 50 } },
    ];
    for (const tree of moved) {
      const growth = defaultGrowth(tree.envelope, tree.attractors);
      expect(growth.influenceRadius).toBeGreaterThan(growth.stepDistance * 9);
    }
    for (const tree of [TELPERION.skeleton, LAURELIN.skeleton]) {
      const growth = defaultGrowth(tree.envelope, tree.attractors);
      expect(growth.influenceRadius).toBe(growth.stepDistance * 9);
    }
  });

  it("grows a whole tree where nine steps grew a stump", () => {
    /* R4's regression case, measured before the floor existed: Telperion
       at a 0.44 m step with 1,600 attractors. Nine steps is 3.96 m
       against a 4.96 m spacing, and the growth dies at the crown base
       with the crown untouched. Both halves are asserted - the collapse
       under the old radius, so the case cannot quietly stop being one,
       and the tree under the derived radius, grown rather than
       reported. */
    const tree = bare(TELPERION.skeleton);
    const step = 0.44;
    const growth = growthAtStep(tree.envelope, step, tree.attractors);

    const stump = census(
      growSkeleton({
        ...tree,
        growth: { ...growth, influenceRadius: step * 9 },
      }),
    );
    expect(stump.nodes).toBeLessThan(250);
    expect(stump.tips).toBeLessThanOrEqual(4);

    const whole = growSkeleton({ ...tree, growth });
    const grown = census(whole);
    expect(grown.nodes).toBeGreaterThan(5000);
    expect(grown.tips).toBeGreaterThan(500);
    // And it is the crown that grew, not a mast: the tree reaches most
    // of the height and width the envelope authored for it.
    const envelope = tree.envelope;
    let top = 0;
    let widest = 0;
    for (const node of whole.nodes) {
      top = Math.max(top, node.position.y);
      widest = Math.max(widest, Math.hypot(node.position.x, node.position.z));
    }
    expect(top).toBeGreaterThan(envelope.height * 0.9);
    expect(widest).toBeGreaterThan(envelope.height * envelope.spread * 0.6);
  });

  it("does not starve anywhere on the panel as the step shrinks", () => {
    /* The panel's density dial runs 250 to 1,600 attractors, and the
       depth rail runs from today's step down to about a seventh of it.
       Across that range, on both presets and the default envelope, a
       finer step must give more nodes and more terminal runs than
       today's step, never fewer - a step that halves is a limb
       subdivided twice as finely before it is anything else, so fewer
       nodes is not a sparser tree, it is the collapse.

       What this does and does not prove. It holds the deterministic
       collapse - the one that struck every seed the moment the radius
       fell below the spacing - off the whole panel. It does not make
       starvation impossible: at 1.2 spacings an exhausted tip early in
       growth sees nothing ahead a few per cent of the time, and in a
       survey of twelve seeds over nine of these cells three draws still
       stalled at the crown base. The floor is pinned there by R7 - see
       `influenceRadiusFor` - so the seeds here are the presets' own and
       the survey's result is recorded rather than asserted. */
    const trees: SkeletonParams[] = [
      bare(TELPERION.skeleton),
      bare(LAURELIN.skeleton),
      params,
    ];
    for (const tree of trees) {
      for (const attractors of [250, 1600]) {
        const today = census(growSkeleton({ ...tree, attractors }));
        expect(today.tips).toBeGreaterThan(50);
        for (const fraction of [0.011, 0.0055, 0.003]) {
          const step = tree.envelope.height * fraction;
          const finer = census(
            growSkeleton({
              ...tree,
              attractors,
              growth: growthAtStep(tree.envelope, step, attractors),
            }),
          );
          const label = `${tree.envelope.height} m, ${attractors} attractors, step ${step.toFixed(2)} m`;
          expect(finer.nodes, label).toBeGreaterThan(today.nodes);
          expect(finer.tips, label).toBeGreaterThan(today.tips);
        }
      }
    }
  });

  it("holds the radius at nine steps until the spacing overtakes it", () => {
    const envelope = TELPERION.skeleton.envelope;
    const count = TELPERION.skeleton.attractors;
    const step = defaultGrowth(envelope).stepDistance;
    expect(influenceRadiusFor(envelope, step, count)).toBe(step * 9);
    expect(influenceRadiusFor(envelope, step / 2, count)).toBe((step / 2) * 9);
    // Shrinking the step past the floor leaves the radius where it is.
    const floor = influenceRadiusFor(envelope, step / 16, count);
    expect(floor).toBeGreaterThan((step / 16) * 9);
    expect(influenceRadiusFor(envelope, step / 32, count)).toBe(floor);
    // Fewer attractors are further apart, and the floor rises with them.
    expect(influenceRadiusFor(envelope, step / 16, count / 8)).toBeCloseTo(
      floor * 2,
      10,
    );
    // No count is no floor: a caller scattering its own attractors gets
    // the nine-step radius it always did.
    expect(influenceRadiusFor(envelope, step / 32, 0)).toBe((step / 32) * 9);
  });
});

describe("the growth step", () => {
  it.each([
    ["Telperion", TELPERION.skeleton],
    ["Laurelin", LAURELIN.skeleton],
    ["the default envelope", params],
  ] as const)("at its default, %s is the tree it was before the dial existed", (_name, tree) => {
    /* R7, and R1's error case in the same breath. A step left out, a
       step stated at the default, and a step that is not a number at
       all are one tree, byte for byte - the last two through the
       `held` rail - so that any tree that differs from today's does so
       because someone moved the dial. */
    const { step: _stated, ...unstated } = tree;
    const today = signature(growSkeleton(unstated));
    expect(signature(growSkeleton({ ...unstated, step: DEFAULT_STEP }))).toBe(today);
    expect(signature(growSkeleton({ ...unstated, step: Number.NaN }))).toBe(today);
    expect(signature(growSkeleton({ ...unstated, step: Number.POSITIVE_INFINITY }))).toBe(today);
  });

  it("moves step and kill distance together, and derives the rest", () => {
    /* One dial, not three: the kill distance keeps the ratio the
       default encodes, the search radius is recomputed from the new
       step rather than scaled with it, and the node ceiling grows as
       the step shrinks so the fine end of the rail is measured rather
       than truncated. */
    const envelope = TELPERION.skeleton.envelope;
    const count = TELPERION.skeleton.attractors;
    const today = defaultGrowth(envelope, count);
    const finer = defaultGrowth(envelope, count, 0.011);
    expect(finer.stepDistance).toBe(envelope.height * 0.011);
    expect(finer.killDistance / finer.stepDistance).toBe(
      today.killDistance / today.stepDistance,
    );
    expect(finer.influenceRadius).toBe(
      influenceRadiusFor(envelope, finer.stepDistance, count),
    );
    expect(finer.trunkHeight).toBe(today.trunkHeight);
    expect(today.maxNodes).toBe(8000);
    expect(finer.maxNodes).toBe(16000);
  });

  it("fits the whole rail under the node ceiling", () => {
    /* The bottom of the panel's rail on the widest crown the panel can
       ask for, at full density: measured at 42,414 nodes, which the old
       fixed ceiling of 8,000 cut off before the middle of the rail. The
       scaled ceiling has to hold it with room, or the dial's fine end
       is the ceiling's shape and not the envelope's. */
    const widest: SkeletonParams = {
      seed: 1,
      envelope: { height: 148, spread: 0.65, crownBase: 0, fullness: 0.05, shoulder: 4 },
      attractors: 1600,
      step: 0.003,
    };
    const ceiling = defaultGrowth(widest.envelope, widest.attractors, widest.step).maxNodes;
    const grown = census(growSkeleton(widest));
    expect(grown.nodes).toBeGreaterThan(30000);
    expect(grown.nodes).toBeLessThan(ceiling);
    // And the dial reaches the tree: finer is deeper, not smaller.
    const today = census(growSkeleton(params));
    const finer = census(growSkeleton({ ...params, step: 0.011 }));
    expect(finer.nodes).toBeGreaterThan(today.nodes);
    expect(finer.tips).toBeGreaterThan(today.tips);
  });
});

describe("defaultGrowth", () => {
  it("scales every distance with the envelope", () => {
    // The spacing floor included: for a fixed attractor count the crown
    // volume goes as the cube of height, so the spacing is a fraction
    // of height like every other distance here. The count is small
    // enough that the floor, not the nine-step multiple, is the radius.
    const small = defaultGrowth({ ...DEFAULT_ENVELOPE, height: 10 }, 20);
    const large = defaultGrowth({ ...DEFAULT_ENVELOPE, height: 40 }, 20);
    expect(large.influenceRadius).toBeGreaterThan(large.stepDistance * 9);
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

describe("the node ceiling under twigs", () => {
  it("is exactly the step's ceiling at zero orders and grows with the orders and children", () => {
    const params = bare(TELPERION.skeleton);
    const rest = resolveGrowth(params).maxNodes;
    expect(rest).toBe(8000);

    const orders = (levels: number, children = 2): number =>
      resolveGrowth({
        ...params,
        twigs: { ...params.twigs, levels, children },
      }).maxNodes;
    // 1 + 0.25 x (2 + 4 + 8 + 16): a stop that scales with what the
    // tips could add, never the count they do add.
    expect(orders(4)).toBe(8000 * (1 + 0.25 * 30));
    expect(orders(8)).toBeGreaterThan(orders(4));
    expect(orders(4, 3)).toBeGreaterThan(orders(4));
    // And never past the ceiling's own ceiling.
    expect(orders(12, 8)).toBe(250000);
    expect(orders(12, 8)).toBe(orders(12, 7));
  });

  it("reports reaching the ceiling, after the shell rule has made the tree smaller than it", () => {
    const params = {
      ...TELPERION.skeleton,
      twigs: { ...TELPERION.skeleton.twigs, levels: 6 },
    };
    const finished = growReport(params);
    expect(finished.capped).toBe(false);
    expect(finished.shed).toBeGreaterThan(0);

    const cutOff = growReport({ ...params, growth: { maxNodes: 3000 } });
    expect(cutOff.capped).toBe(true);
    // Smaller than the ceiling it hit: the count alone would have said
    // it finished.
    expect(cutOff.skeleton.nodes.length).toBeLessThan(3000);
    expect(cutOff.skeleton.nodes.length + cutOff.shed).toBe(3000);
  });
});
