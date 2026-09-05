import { readFileSync } from "node:fs";
import { Vector3 } from "three";
import { describe, expect, it } from "vitest";
import { DEFAULT_ENVELOPE, sampleEnvelope } from "../envelope";
import { TELPERION, LAURELIN } from "../presets/two-trees";
import { solveRadii, DEFAULT_RADII, type RadiusField } from "../radius";
import { createRng } from "../rng";
import { createGrowthBias, NO_BIAS } from "../torsion";
import { colonize, type Skeleton, type GrowthConfig } from "./colonize";
import { growReport, resolveGrowth } from "./grow";
import { branchLength, childRadius, DEFAULT_TWIG_ANATOMY } from "./law";
import { branchTwigs, resolveTwigs, DEFAULT_TWIGS, MAX_TWIG_LEVELS } from "./twigs";

const base: Skeleton = { nodes: [
  { position: new Vector3(0, 0, 0), parent: -1 },
  { position: new Vector3(0, 10, 0), parent: 0 },
] };
const config: GrowthConfig = { stepDistance: 1, killDistance: 2, influenceRadius: 9,
  trunkHeight: 0, maxNodes: 250000, maxTurnPerStep: 90 };
const field = (radius: number, count = 2): RadiusField => ({
  radius: new Float64Array(count).fill(radius), startRadius: new Float64Array(count).fill(radius),
});
const arrival = (tree: Skeleton, i: number) => tree.nodes[i].position.clone()
  .sub(tree.nodes[tree.nodes[i].parent].position).normalize();
const signature = (tree: Skeleton) => JSON.stringify(tree.nodes.map(n => [n.parent, ...n.position.toArray()]));

describe("branch generations", () => {
  it("grows internode runs with lateral radii from the law and terminal twigs as the sole radius exception", () => {
    const params = resolveTwigs({ lengthRatio: 0.4 });
    const tree = branchTwigs(base, field(0.04), config, params);
    const counts = new Int32Array(tree.nodes.length);
    const runs = new Map<number, number[]>();
    for (let i = 1; i < tree.nodes.length; i++) counts[tree.nodes[i].parent]++;
    expect(tree.nodes[0].parent).toBe(-1);
    for (let i = tree.crossover; i < tree.nodes.length; i++) {
      const k = i - tree.crossover;
      const node = tree.nodes[i];
      expect(node.parent).toBeGreaterThanOrEqual(0);
      expect(node.parent).toBeLessThan(i);
      const id = tree.branchId[k];
      expect(id).toBeLessThanOrEqual(i);
      const run = runs.get(id) ?? [];
      run.push(i); runs.set(id, run);
      if (tree.twig[k]) {
        expect(tree.baseRadius[k]).toBe(params.twig.diameter / 2);
        expect(node.position.distanceTo(tree.nodes[node.parent].position)).toBeCloseTo(params.twig.internodeLength, 12);
      } else {
        const origin = tree.nodes[id].parent;
        const expected = origin < tree.crossover ? 0.04 : childRadius(
          tree.baseRadius[origin - tree.crossover], params.lengthRatio, params.ratioPower);
        expect(tree.baseRadius[k]).toBe(expected);
      }
      if (counts[i] === 0) expect(tree.twig[k]).toBe(1);
    }
    let branches = 0;
    for (const [id, run] of runs) {
      if (tree.twig[id - tree.crossover]) { expect(run).toHaveLength(1); continue; }
      branches++;
      expect(run).toHaveLength(params.internodes);
      expect(run.slice(0, -1).map(i => counts[i])).toEqual([2, 2]);
      const origin = tree.nodes[id].parent;
      const length = run.reduce((sum, i) => sum + tree.nodes[i].position.distanceTo(tree.nodes[tree.nodes[i].parent].position), 0);
      if (origin < tree.crossover) expect(length).toBeCloseTo(branchLength(0.04), 12);
      else {
        const parentRun = runs.get(tree.branchId[origin - tree.crossover])!;
        const parentLength = parentRun.reduce((sum, i) => sum + tree.nodes[i].position.distanceTo(tree.nodes[tree.nodes[i].parent].position), 0);
        expect(length).toBeCloseTo(parentLength * params.lengthRatio, 12);
      }
    }
    expect(branches).toBeGreaterThan(1);
    expect(tree.levelCapped).toBe(false);
  });

  it.each([0, DEFAULT_TWIG_ANATOMY.diameter / 2, 0.1, 1])("keeps fixed twig dimensions at handoff radius %s", radius => {
    const tree = branchTwigs(base, field(radius), config, resolveTwigs());
    expect(tree.twig.some(mark => mark === 1)).toBe(true);
    for (let k = 0; k < tree.twig.length; k++) if (tree.twig[k]) {
      const node = tree.nodes[k + tree.crossover];
      expect(tree.baseRadius[k]).toBe(DEFAULT_TWIG_ANATOMY.diameter / 2);
      expect(node.position.distanceTo(tree.nodes[node.parent].position)).toBeCloseTo(DEFAULT_TWIG_ANATOMY.internodeLength, 12);
    }
    if (radius <= DEFAULT_TWIG_ANATOMY.diameter / 2) expect(tree.twig).toEqual(new Uint8Array([1]));
  });

  it("collapses a sub-internode branch to one twig", () => {
    const twig = { ...DEFAULT_TWIG_ANATOMY, internodeLength: 10 };
    const tree = branchTwigs(base, field(0.01), config, resolveTwigs({ twig }));
    expect(tree.twig).toEqual(new Uint8Array([1]));
    expect(tree.baseRadius[0]).toBe(twig.diameter / 2);
    expect(tree.nodes[2].position.distanceTo(base.nodes[1].position)).toBe(10);
  });

  it("reports a nonconverging level cap and the node ceiling without calling either a twig", () => {
    const params = resolveTwigs({ internodes: 2, lengthRatio: 1 });
    const tree = branchTwigs(base, field(0.1), config, params);
    expect(tree.levelCapped).toBe(true);
    const depth = new Int32Array(tree.nodes.length);
    for (let i = 2; i < tree.nodes.length; i++) {
      const k = i - 2, parent = tree.nodes[i].parent;
      depth[i] = depth[parent] + Number(!tree.twig[k] && tree.branchId[k] === i && parent >= 2);
    }
    expect(Math.max(...depth)).toBe(MAX_TWIG_LEVELS - 1);
    const boundary = branchTwigs(base, field(0.0025 * 2 ** MAX_TWIG_LEVELS), config,
      resolveTwigs({ internodes: 2, lengthRatio: 0.5, ratioPower: 1 }));
    expect(boundary.levelCapped).toBe(false);
    const capped = branchTwigs(base, field(1), { ...config, maxNodes: 7 }, resolveTwigs());
    expect(capped.nodes).toHaveLength(7);
    expect(capped.nodeCapped).toBe(true);
  });

  it.each([ ["self", 1], ["forward", 2], ["out-of-range", 20], ["negative", -1] ])("refuses a %s parent unchanged", (_name, parent) => {
    const invalid = { nodes: [base.nodes[0], { ...base.nodes[1], parent: parent as number }] };
    const tree = branchTwigs(invalid, field(1), config, resolveTwigs());
    expect(tree.nodes).toEqual(invalid.nodes);
    expect(tree.crossover).toBe(2);
    expect(tree.refused).toBe("invalid-parent");
    expect(tree.branchId).toHaveLength(0);
  });
  it("refuses a wrong-length radius field unchanged", () => {
    const tree = branchTwigs(base, field(1, 1), config, resolveTwigs());
    expect(tree.nodes).toEqual(base.nodes);
    expect(tree.crossover).toBe(2);
    expect(tree.refused).toBe("radius-length-mismatch");
  });
  it("returns empty and root-only skeletons without growing", () => {
    for (const nodes of [[], base.nodes.slice(0, 1)]) {
      const tree = branchTwigs({ nodes }, field(1, nodes.length), config, resolveTwigs());
      expect(tree.nodes).toEqual(nodes);
      expect(tree.twig).toHaveLength(0);
    }
  });

  it("collides laterals against arrival even when the accepted leader bends onto the lateral", () => {
    const params = resolveTwigs({ internodes: 2, divergence: 0, angle: 45 });
    const bent = new Vector3(0, 1, -1).normalize();
    const bias = (p: Vector3, wanted: Vector3) => p.y === 10 ? wanted.clone().normalize() : bent.clone();
    const tree = branchTwigs(base, field(0.01), { ...config, bias }, params);
    const children = tree.nodes.map((node, i) => node.parent === 2 ? i : -1).filter(i => i >= 0);
    // Both depart along bent, away from their parent's vertical arrival.
    expect(children).toHaveLength(2);
    expect(arrival(tree, children[0]).distanceTo(arrival(tree, children[1]))).toBeLessThan(1e-10);
    const folded = branchTwigs(base, field(0.01), { ...config, bias: () => new Vector3(0, 1, 0) }, params);
    expect(folded.nodes.filter(node => node.parent === 2)).toHaveLength(1);
    const source = readFileSync(new URL("twigs.ts", import.meta.url), "utf8");
    expect(source).not.toMatch(/new (Map|Set|WeakMap|WeakSet)\b/);
  });

  it("consults the bias on every internode, limits every turn and guards the candidate's trunk height", () => {
    let calls = 0;
    const lean = createGrowthBias(DEFAULT_ENVELOPE, 1, { ...NO_BIAS, lean: 0.5 });
    const biased = { ...config, maxTurnPerStep: 26, bias: (p: Vector3, wanted: Vector3, step: number) => {
      calls++; expect(step).toBe(config.stepDistance); return lean(p, wanted, step);
    } };
    const tree = branchTwigs(base, field(0.04), biased, resolveTwigs());
    expect(calls).toBeGreaterThanOrEqual(tree.nodes.length - 2);
    for (let i = 2; i < tree.nodes.length; i++) expect(arrival(tree, i).dot(arrival(tree, tree.nodes[i].parent)))
      .toBeGreaterThanOrEqual(Math.cos(26 * Math.PI / 180) - 1e-10);
    const down = { nodes: [{ ...base.nodes[1], parent: -1 }, { position: new Vector3(0, 0.01, 0), parent: 0 }] };
    const guarded = branchTwigs(down, field(0.1), config, resolveTwigs());
    expect(guarded.nodes).toEqual(down.nodes);
    const zero = createGrowthBias(DEFAULT_ENVELOPE, 1, NO_BIAS);
    expect(branchTwigs(base, field(0.04), { ...config, bias: zero }, resolveTwigs()))
      .toEqual(branchTwigs(base, field(0.04), config, resolveTwigs()));
    expect(signature(tree)).not.toBe(signature(branchTwigs(base, field(0.04), config, resolveTwigs())));
  });

  it.each([TELPERION, LAURELIN])("keeps $name colonization byte-identical and only changes appended wood with the field", preset => {
    const p = preset.skeleton;
    const cfg = resolveGrowth(p);
    const colonized = colonize(sampleEnvelope(p.envelope, p.attractors, createRng(p.seed)), new Vector3(), cfg);
    const radii = solveRadii(colonized, p.envelope, preset.radii);
    const before = signature(colonized);
    const a = branchTwigs(colonized, radii, cfg, resolveTwigs(p.twigs));
    const b = branchTwigs(colonized, radii, cfg, resolveTwigs(p.twigs));
    expect(a).toEqual(b);
    const changed = branchTwigs(colonized, field(0.0025, colonized.nodes.length), cfg, resolveTwigs(p.twigs));
    expect(signature(changed)).not.toBe(signature(a));
    expect(signature({ nodes: a.nodes.slice(0, a.crossover) })).toBe(before);
    expect(signature({ nodes: changed.nodes.slice(0, changed.crossover) })).toBe(before);
    expect(signature(colonized)).toBe(before);
    for (const height of [p.envelope.height, p.envelope.height / 10]) {
      const report = growReport({ ...p, envelope: { ...p.envelope, height } }, preset.radii);
      expect(report.capped).toBe(false);
      const tree = report.skeleton;
      expect(tree.twig.some(mark => mark === 1)).toBe(true);
      for (let k = 0; k < tree.twig.length; k++) if (tree.twig[k]) {
        expect(tree.baseRadius[k]).toBe(DEFAULT_TWIG_ANATOMY.diameter / 2);
        const n = tree.nodes[k + tree.crossover];
        expect(n.position.distanceTo(tree.nodes[n.parent].position)).toBeCloseTo(DEFAULT_TWIG_ANATOMY.internodeLength, 10);
      }
    }
  }, 60000);

  it("resolves finite rails and non-finite defaults without a levels key", () => {
    expect(resolveTwigs()).toEqual(DEFAULT_TWIGS);
    expect(resolveTwigs({ angle: NaN, divergence: Infinity, internodes: NaN, laterals: Infinity,
      lengthRatio: NaN, ratioPower: Infinity })).toEqual(DEFAULT_TWIGS);
    expect(resolveTwigs({ internodes: 0, laterals: -1, lengthRatio: 0, ratioPower: 99, angle: 99 }))
      .toMatchObject({ internodes: 1, laterals: 0, lengthRatio: 0.05, ratioPower: 8, angle: 90 });
    expect(DEFAULT_TWIGS).not.toHaveProperty("levels");
  });
});
