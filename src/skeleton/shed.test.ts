import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_CULL } from "../canopy/cull";
import { DEFAULT_ENVELOPE, envelopeRadiusAt } from "../envelope";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import { colonize, type Skeleton } from "./colonize";
import { sampleEnvelope } from "../envelope";
import { createRng } from "../rng";
import { solveRadii } from "../radius";
import { resolveGrowth } from "./grow";
import { DEFAULT_SHED, shedTwigs } from "./shed";
import { branchTwigs, resolveTwigs } from "./twigs";

/* ------------------------------------------------------------------ *
 * The shell rule over twigs, held to the same two-sided guard the
 * leaf culler is held to: a pass that sheds nothing fails the
 * count-falls assertion, a pass that sheds everything fails the
 * count-remains one, and both are asserted on the presets as authored
 * with their twigs grown to the depth where an interior exists.
 * ------------------------------------------------------------------ */

/** The tip pass before the shell rule, under the preset's own radii. */
function grown(preset: typeof TELPERION) {
  const params = preset.skeleton;
  const config = resolveGrowth(params);
  const base = colonize(sampleEnvelope(params.envelope, params.attractors, createRng(params.seed)), new THREE.Vector3(), config);
  const field = solveRadii(base, params.envelope, preset.radii);
  const twigged = branchTwigs(base, field, config, resolveTwigs(params.twigs));
  return { twigged, from: base.nodes.length };
}

const at = (x: number, y: number, z: number) => new THREE.Vector3(x, y, z);

describe("shedTwigs", () => {
  it.each([
    ["Telperion", TELPERION],
    ["Laurelin", LAURELIN],
  ])("sheds a part of %s's twigs and leaves the rest", (_name, preset) => {
    const { twigged, from } = grown(preset);
    const shed = shedTwigs(twigged, from, preset.skeleton.envelope);
    const twigs = twigged.nodes.length - from;
    const kept = shed.nodes.length - from;

    // The fixture's own premise: enough twigs that there is an
    // interior to speak of.
    expect(twigs).toBeGreaterThan(10000);
    // A rule that sheds nothing dies here.
    expect(kept).toBeLessThan(twigs * 0.95);
    // A rule that sheds everything dies here.
    expect(kept).toBeGreaterThan(twigs / 2);
  });

  it("keeps every colonization node as it was and re-indexes every parent below itself", () => {
    const { twigged, from } = grown(TELPERION);
    const shed = shedTwigs(twigged, from, TELPERION.skeleton.envelope);

    for (let i = 0; i < from; i += 1) expect(shed.nodes[i]).toBe(twigged.nodes[i]);
    for (let i = 1; i < shed.nodes.length; i += 1) {
      expect(shed.nodes[i].parent).toBeGreaterThanOrEqual(0);
      expect(shed.nodes[i].parent).toBeLessThan(i);
    }
    // Twigs come out in the order they went in, and no kept twig has
    // lost the node it grew from.
    // Identity by position object, looked up once: the same assertion as
    // a find-and-indexOf per node, without the quadratic walk that took
    // five minutes once the pass appended two hundred thousand nodes.
    const indexByPosition = new Map<object, number>();
    twigged.nodes.forEach((node, index) => indexByPosition.set(node.position, index));
    let last = -1;
    for (let i = from; i < shed.nodes.length; i += 1) {
      const index = indexByPosition.get(shed.nodes[i].position);
      expect(index).toBeDefined();
      expect(index!).toBeGreaterThan(last);
      last = index!;
    }
  }, 60_000);

  it("keeps an interior twig whose line reaches the shell, and sheds one that never does", () => {
    /* A crown-centre limb node, two twigs off it: one that heads out
       to the envelope's surface, one that stays buried. The buried
       base of the outbound twig is exactly the boundary case the
       every-node rule exists for. */
    const envelope = DEFAULT_ENVELOPE;
    const y = envelope.height * 0.7;
    const surface = envelopeRadiusAt(envelope, y);
    const skeleton: Skeleton = {
      nodes: [
        { position: at(0, 0, 0), parent: -1 },
        { position: at(0, y, 0), parent: 0 },
        // Outbound: buried base, then a node on the silhouette.
        { position: at(surface * 0.2, y, 0), parent: 1 },
        { position: at(surface * 0.98, y, 0), parent: 2 },
        // Buried: two nodes that never leave the centre.
        { position: at(0, y, surface * 0.1), parent: 1 },
        { position: at(0, y, surface * 0.2), parent: 4 },
      ],
    };
    const shed = shedTwigs(skeleton, 2, envelope, { shellDepth: 0.2 });
    expect(shed.nodes.map((node) => node.position)).toEqual([
      skeleton.nodes[0].position,
      skeleton.nodes[1].position,
      skeleton.nodes[2].position,
      skeleton.nodes[3].position,
    ]);
    expect(shed.nodes.map((node) => node.parent)).toEqual([-1, 0, 1, 2]);
  });

  it("compacts records and remaps a surviving branch and its twig after an earlier subtree is shed", () => {
    const y = DEFAULT_ENVELOPE.height * 0.7;
    const edge = envelopeRadiusAt(DEFAULT_ENVELOPE, y);
    const tree = {
      nodes: [
        { position: at(0, 0, 0), parent: -1 },
        { position: at(0, y, 0), parent: 0 },
        { position: at(0, y, edge * 0.1), parent: 1 },
        { position: at(0, y, edge * 0.2), parent: 2 },
        { position: at(edge * 0.2, y, 0), parent: 1 },
        { position: at(edge * 0.98, y, 0), parent: 4 },
        { position: at(edge, y, 0), parent: 5 },
      ],
      crossover: 2,
      branchId: new Int32Array([2, 3, 4, 4, 6]),
      baseRadius: new Float64Array([0.1, 0.0025, 0.2, 0.2, 0.0025]),
      twig: new Uint8Array([0, 1, 0, 0, 1]),
      levelCapped: true, nodeCapped: false,
    };
    const shed = shedTwigs(tree, tree.crossover, DEFAULT_ENVELOPE, { shellDepth: 0.2 });
    expect(shed.nodes.map(n => n.position)).toEqual([0, 1, 4, 5, 6].map(i => tree.nodes[i].position));
    expect(shed.nodes.map(n => n.parent)).toEqual([-1, 0, 1, 2, 3]);
    expect(shed.branchId).toEqual(new Int32Array([2, 2, 4]));
    expect(shed.baseRadius).toEqual(new Float64Array([0.2, 0.2, 0.0025]));
    expect(shed.twig).toEqual(new Uint8Array([0, 0, 1]));
    expect(shed.crossover).toBe(2);
    expect(shed.levelCapped).toBe(true);
    expect(tree.branchId).toEqual(new Int32Array([2, 3, 4, 4, 6]));
  });

  it("sheds nothing under a shell as thick as the crown, and under a shell it cannot read", () => {
    const { twigged, from } = grown(TELPERION);
    const envelope = TELPERION.skeleton.envelope;
    const whole = shedTwigs(twigged, from, envelope, { shellDepth: 1 });
    expect(whole.nodes.length).toBe(twigged.nodes.length);

    // NaN is the default shell, not no shell and not every twig.
    const unreadable = shedTwigs(twigged, from, envelope, { shellDepth: NaN });
    expect(unreadable.nodes.length).toBe(
      shedTwigs(twigged, from, envelope, DEFAULT_SHED).nodes.length,
    );
    expect(DEFAULT_SHED).toBe(DEFAULT_CULL);
  });

  it("keeps a twig it cannot place, and a skeleton with no twigs", () => {
    const envelope = DEFAULT_ENVELOPE;
    const y = envelope.height * 0.7;
    const skeleton: Skeleton = {
      nodes: [
        { position: at(0, 0, 0), parent: -1 },
        { position: at(0, y, 0), parent: 0 },
        { position: at(NaN, y, 0), parent: 1 },
      ],
    };
    expect(shedTwigs(skeleton, 2, envelope, { shellDepth: 0 }).nodes.length).toBe(3);
    expect(shedTwigs(skeleton, 3, envelope).nodes).toEqual(skeleton.nodes);
    expect(shedTwigs(skeleton, NaN, envelope).nodes).toEqual(skeleton.nodes);
    expect(shedTwigs({ nodes: [] }, 0, envelope).nodes).toEqual([]);
  });
});
