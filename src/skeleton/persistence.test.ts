import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE, type Envelope } from "../envelope";
import {
  colonize,
  DEFAULT_MAX_TURN_PER_STEP,
  type GrowthConfig,
} from "./colonize";
import { defaultGrowth, growSkeleton } from "./grow";
import { DEFAULT_BIAS, NO_BIAS, type BiasParams } from "../torsion";

/* ------------------------------------------------------------------ *
 * DIRECTIONAL PERSISTENCE, MEASURED
 *
 * The defect was reported as a measurement and it is closed as one.
 * Turn angle here means the angle between the step into a node and the
 * step into its single continuing child - forks excluded, because a
 * fork is supposed to change direction and a continuation is not.
 *
 * The owner's own sweep of the fn-11.3 generator, seed 1, 800
 * attractors, before any of this existed:
 *
 *   config              median   p90    p99    max     reversals >90
 *   NO_BIAS              14.4    53.0    91.9  101.3    8  (1.53%)
 *   DEFAULT_BIAS         12.0    39.4   109.7  176.8   15  (1.79%)
 *
 * A 176-degree turn is a branch going forward and then almost exactly
 * backward. Those figures are the specification and they are not
 * reproducible from here - the generator they were taken on no longer
 * exists - so what the tests below reproduce is the nearest thing that
 * is still reachable: this generator with the turn limit off, which
 * measures 172.5 max and 14 reversals at DEFAULT_BIAS. Slightly better
 * than the recorded baseline and far worse than the fix, which is the
 * shape of the claim being made.
 * ------------------------------------------------------------------ */

interface Turns {
  nodes: number;
  continuations: number;
  median: number;
  p99: number;
  max: number;
  reversals: number;
}

function measure(
  bias: Partial<BiasParams>,
  growth?: Partial<GrowthConfig>,
  seed = 1,
  envelope: Envelope = DEFAULT_ENVELOPE,
  attractors = 800,
): Turns {
  const nodes = growSkeleton({
    seed,
    envelope,
    attractors,
    bias,
    growth,
  }).nodes;

  const arrival = nodes.map((node) =>
    node.parent < 0
      ? null
      : node.position.clone().sub(nodes[node.parent].position).normalize(),
  );
  const children = new Map<number, number[]>();
  nodes.forEach((node, index) => {
    if (node.parent < 0) return;
    const list = children.get(node.parent);
    if (list === undefined) children.set(node.parent, [index]);
    else list.push(index);
  });

  const angles: number[] = [];
  for (const [parent, kids] of children) {
    if (kids.length !== 1) continue; // continuations only, never forks
    const into = arrival[parent];
    const on = arrival[kids[0]];
    if (into === null || on === null) continue;
    angles.push(
      THREE.MathUtils.radToDeg(
        Math.acos(THREE.MathUtils.clamp(into.dot(on), -1, 1)),
      ),
    );
  }
  angles.sort((a, b) => a - b);
  const at = (fraction: number) =>
    angles[Math.floor(fraction * (angles.length - 1))];

  return {
    nodes: nodes.length,
    continuations: angles.length,
    median: at(0.5),
    p99: at(0.99),
    max: angles[angles.length - 1],
    reversals: angles.filter((angle) => angle > 90).length,
  };
}

/** The turn limit off - and only the turn limit: 180 degrees is every
 *  direction, so nothing is refused for turning too far. The other
 *  half of fn-11.8, ending a branch that closes on nothing, is not a
 *  parameter and is still in force, so this is not the fn-11.2
 *  generator restored. It is this generator with persistence taken
 *  out, which is what makes it the honest measurement of what
 *  persistence alone is worth. The recorded fn-11.3 baseline, taken
 *  before either mechanism existed, is 176.8 degrees maximum and 15
 *  reversals - worse than what this reproduces. */
const UNLIMITED: Partial<GrowthConfig> = { maxTurnPerStep: 180 };

/** Every dial at each end of the range the panel offers, one at a
 *  time and then all at once. The tail is what is visible, and the
 *  tail lives at the extremes: at defaults the old generator already
 *  had a median of 12 degrees and still drew sawtooth. */
const EXTREMES: { label: string; bias: Partial<BiasParams> }[] = [
  /* The panel's `torsion` master multiplies lean, writhe and spiral by
     up to 2 before the library ever sees them, so the reachable
     ceiling on each of those three is twice its own dial - and that,
     not the dial, is what the sweep has to cover. */
  {
    label: "every dial at its ceiling, torsion doubling all three",
    bias: {
      gravitropism: 1.3,
      lean: 1,
      writheAmplitude: 0.5,
      writheWavelength: 0.08,
      spiralRate: 12,
    },
  },
  /* And torsion's other end. Not the same tree as no bias at all:
     torsion at 0 zeroes the three departures from vertical and leaves
     gravitropism exactly where the owner set it, so this is a tree
     that is dead straight and still pulling hard at the sky. */
  {
    label: "torsion at zero, gravitropism untouched",
    bias: { ...DEFAULT_BIAS, lean: 0, writheAmplitude: 0, spiralRate: 0 },
  },
  { label: "no bias at all", bias: NO_BIAS },
  { label: "defaults", bias: DEFAULT_BIAS },
  { label: "no gravitropism", bias: { ...DEFAULT_BIAS, gravitropism: 0 } },
  { label: "max gravitropism", bias: { ...DEFAULT_BIAS, gravitropism: 1.3 } },
  { label: "no lean", bias: { ...DEFAULT_BIAS, lean: 0 } },
  { label: "max lean", bias: { ...DEFAULT_BIAS, lean: 0.5 } },
  { label: "no writhe", bias: { ...DEFAULT_BIAS, writheAmplitude: 0 } },
  { label: "max writhe", bias: { ...DEFAULT_BIAS, writheAmplitude: 0.25 } },
  { label: "shortest bend", bias: { ...DEFAULT_BIAS, writheWavelength: 0.08 } },
  { label: "longest bend", bias: { ...DEFAULT_BIAS, writheWavelength: 1.2 } },
  { label: "no spiral", bias: { ...DEFAULT_BIAS, spiralRate: 0 } },
  { label: "max spiral", bias: { ...DEFAULT_BIAS, spiralRate: 6 } },
  {
    label: "every dial at its ceiling",
    bias: {
      gravitropism: 1.3,
      lean: 0.5,
      writheAmplitude: 0.25,
      writheWavelength: 0.08,
      spiralRate: 6,
    },
  },
];

describe("directional persistence", () => {
  /* Five hundred and ten trees, and the whole point of a sweep is its
     tail, so this is a slow test on purpose - it was 3.7 s at three
     envelopes on a loaded box, against vitest's 5 s default, and it
     failed about one run in three. That is not a slow test, it is a
     flake, and it was flaking in the test that guards the defect this
     file exists for. A timeout is a hang detector and not a
     performance budget, so this one is set well clear of the two
     envelopes since added (3.3 s at five, idle) rather than trimmed to
     the measurement. */
  it("holds every continuation inside the limit, at every extreme", { timeout: 30_000 }, () => {
    /* The panel's envelope dials are in the sweep too: the limit is
       per step and the step is a fraction of height, so the same dial
       setting is a different curvature on the ground at every size,
       and all of them have to hold. The heights are the height dial's
       own range end to end - 4 m sapling to the 400 m of a tree of the
       Two Trees' order - at the spread each of them would plausibly be
       given. */
    const envelopes: Envelope[] = [
      DEFAULT_ENVELOPE,
      { ...DEFAULT_ENVELOPE, height: 4, spread: 0.12 },
      { ...DEFAULT_ENVELOPE, height: 60, spread: 0.65 },
      { ...DEFAULT_ENVELOPE, height: 150, spread: 0.4 },
      { ...DEFAULT_ENVELOPE, height: 400, spread: 0.5 },
    ];
    // Collected rather than asserted one at a time, so a failure names
    // the dial and the seed that broke it instead of the first one.
    // Both ends of the density dial as well: the attractor count is
    // what decides how crowded a node's pull is, and a crowded node is
    // where a reversal came from.
    const over: string[] = [];
    for (const envelope of envelopes) {
      for (const { label, bias } of EXTREMES) {
        for (const seed of [1, 2, 3]) {
          for (const attractors of [250, 1600]) {
            const turns = measure(bias, undefined, seed, envelope, attractors);
            const where = `${label}, ${envelope.height} m, seed ${seed}, ${attractors} attractors`;
            if (turns.max > DEFAULT_MAX_TURN_PER_STEP + 1e-9) {
              over.push(`${where}: turned ${turns.max.toFixed(1)} deg`);
            }
            if (turns.reversals > 0) {
              over.push(`${where}: ${turns.reversals} reversals`);
            }
            if (turns.continuations < 20) {
              over.push(`${where}: only ${turns.continuations} continuations`);
            }
          }
        }
      }
    }
    expect(over).toEqual([]);
  });

  it("holds them inside whatever limit it was given", () => {
    // A stiff tree and a whippy one, and the number means the same
    // thing in both: degrees of turn per growth step.
    for (const maxTurnPerStep of [5, 12, 35, 60, 90]) {
      const turns = measure(DEFAULT_BIAS, { maxTurnPerStep });
      expect(turns.max).toBeLessThanOrEqual(maxTurnPerStep + 1e-9);
      expect(turns.continuations).toBeGreaterThan(100);
    }
  });

  it("beats the measurement the defect was reported with", () => {
    const before = measure(DEFAULT_BIAS, UNLIMITED);
    const after = measure(DEFAULT_BIAS);

    // The reported failure, still reproducible with the limit off:
    // a branch that goes forward and then almost exactly backward.
    expect(before.max).toBeGreaterThan(170);
    expect(before.reversals).toBeGreaterThan(10);

    expect(after.max).toBeLessThanOrEqual(DEFAULT_MAX_TURN_PER_STEP + 1e-9);
    expect(after.p99).toBeLessThan(before.p99);
    expect(after.reversals).toBe(0);

    // And it is not paid for in nodes. Every node an accordion spent
    // oscillating in place was a node the rest of the crown did not
    // get, so the fixed tree is the bigger one on the same budget.
    expect(after.nodes).toBeGreaterThanOrEqual(before.nodes);
  });

  it("is the same skeleton, byte for byte, from the same seed", () => {
    const signature = (limit?: number) =>
      JSON.stringify(
        growSkeleton({
          seed: 4,
          envelope: DEFAULT_ENVELOPE,
          attractors: 700,
          growth: limit === undefined ? undefined : { maxTurnPerStep: limit },
        }).nodes.map((node) => [
          node.position.x,
          node.position.y,
          node.position.z,
          node.parent,
        ]),
      );
    expect(signature()).toBe(signature());
    expect(signature(15)).toBe(signature(15));
    // And the limit is a real argument, not a decoration.
    expect(signature(15)).not.toBe(signature());
  });
});

describe("a branch with nowhere to grow", () => {
  /* The trap the screenshot caught: two attractors pulling from
     opposite sides, with their midpoint further from either one than
     `killDistance`. Neither is ever reached, so neither is ever
     retired, and the branch stepped back and forth between them - a
     comb of dozens of segments in a few centimetres - until the node
     budget or the influence radius saved it.

     Persistence alone does not fix this. A branch that cannot reverse
     but also cannot reach anything is still trapped; it just draws a
     smoother trap. What ends it is that a step which closes on none of
     the attractors still pulling the node is not taken at all. */
  const config: GrowthConfig = {
    stepDistance: 0.5,
    killDistance: 1,
    influenceRadius: 8,
    trunkHeight: 4,
    maxNodes: 4000,
  };
  const trap = [
    new THREE.Vector3(-3, 6, 0),
    new THREE.Vector3(3, 6, 0),
    new THREE.Vector3(-3, 6.4, 0.2),
    new THREE.Vector3(3, 6.4, -0.2),
  ];

  it("stops instead of oscillating in place", () => {
    const skeleton = colonize(trap, new THREE.Vector3(0, 0, 0), config);
    // Climb plus a short run out to each pair, and nothing like the
    // hundreds of nodes an accordion burns.
    expect(skeleton.nodes.length).toBeLessThan(60);
    expect(skeleton.nodes.length).toBeGreaterThan(12);

    // No two nodes on top of each other: a comb is dozens of segments
    // in a few centimetres, so this is the shape of it, not a proxy.
    const step = config.stepDistance;
    for (let i = 0; i < skeleton.nodes.length; i += 1) {
      for (let j = i + 1; j < skeleton.nodes.length; j += 1) {
        expect(
          skeleton.nodes[i].position.distanceTo(skeleton.nodes[j].position),
        ).toBeGreaterThan(step * 0.05);
      }
    }
  });

  it("costs the crown nothing when there is somewhere to grow", () => {
    // The guard has to end trapped branches and no others: a tree with
    // a real crown to fill still fills it.
    const growth = defaultGrowth(DEFAULT_ENVELOPE);
    const grown = growSkeleton({
      seed: 1,
      envelope: DEFAULT_ENVELOPE,
      attractors: 800,
    });
    expect(grown.nodes.length).toBeGreaterThan(1000);
    expect(grown.nodes.length).toBeLessThan(growth.maxNodes);
  });
});
