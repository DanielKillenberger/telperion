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
import { branchLength, childRadius, internodeLength, DEFAULT_TWIG_ANATOMY } from "./law";
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
/** The anatomy with the last-order rule switched off: no wood bears
 *  twigs at its stations, so a fixture can describe the branch law and
 *  its laterals alone. The rule itself has its own test below. */
const branchesOnly = { ...DEFAULT_TWIG_ANATOMY, bearingDiameter: DEFAULT_TWIG_ANATOMY.diameter };

describe("branch generations", () => {
  it("grows a fixed-length twig shoot with one branch id and no lateral offspring", () => {
    const params = resolveTwigs({ twig: { ...DEFAULT_TWIG_ANATOMY, length: 0.25 } });
    const tree = branchTwigs(base, field(0.0025), config, params);
    expect(tree.nodes.length - tree.crossover).toBe(1);
    expect([...tree.twig]).toEqual(Array(1).fill(1));
    expect([...tree.branchId]).toEqual(Array(1).fill(tree.crossover));
    expect([...tree.baseRadius]).toEqual(Array(1).fill(0.0025));
    expect(tree.nodes.at(-1)!.position.y - base.nodes[1].position.y).toBeCloseTo(0.25, 12);
    const solved = solveRadii(tree, DEFAULT_ENVELOPE, DEFAULT_RADII);
    for (let i = tree.crossover; i < tree.nodes.length; i++) {
      expect(solved.startRadius[i]).toBe(0.0025);
      expect(solved.radius[i]).toBe(0.0025);
    }
  });

  it("charges finer branch resolution only for internodes, with the same lateral tree", () => {
    const rows = [4, 2.5, 1.5].map(internodeFactor => {
      const params = resolveTwigs({ internodeFactor, laterals: 3, limbRadius: 0, twig: branchesOnly });
      const tree = branchTwigs(base, field(0.04), config, params);
      expect(tree.nodeCapped || tree.levelCapped).toBe(false);
      const runs = new Map<number, number[]>();
      for (let i = tree.crossover; i < tree.nodes.length; i++) {
        const id = tree.branchId[i - tree.crossover];
        const run = runs.get(id) ?? []; run.push(i); runs.set(id, run);
      }
      let branches = 0, twigs = 0, woodNodes = 0;
      for (const [id, run] of runs) {
        if (tree.twig[id - tree.crossover]) { twigs++; continue; }
        branches++; woodNodes += run.length;
        const laterals = [...runs.keys()].filter(child =>
          tree.nodes[child].parent >= tree.crossover &&
          tree.branchId[tree.nodes[child].parent - tree.crossover] === id &&
          tree.baseRadius[child - tree.crossover] < tree.baseRadius[id - tree.crossover]);
        // A terminal twig is also smaller: identify it by its axial departure.
        const sideBranches = laterals.filter(child => arrival(tree, child).dot(arrival(tree, tree.nodes[child].parent)) < 0.999);
        expect(sideBranches).toHaveLength(3);
        const stations = sideBranches.map(child => run.indexOf(tree.nodes[child].parent) + 1).sort((a, b) => a - b);
        expect(stations).toEqual([1, 2, 3].map(j => Math.max(1, Math.round(j * run.length / 4))));
      }
      return { branches, twigs, woodNodes, nodes: tree.nodes.length };
    });
    for (const row of rows) {
      expect(row.branches).toBe(rows[0].branches);
      expect(row.twigs).toBe(rows[0].twigs);
      expect(row.nodes - rows[0].nodes).toBe(row.woodNodes - rows[0].woodNodes);
    }
    expect(rows[2].woodNodes).toBeGreaterThan(rows[0].woodNodes);
  });

  it("separates a bud's departure angle from its subsequent curvature limit", () => {
    const params = resolveTwigs({ angle: 45, angleVariation: 0, vigourVariation: 0, laterals: 2 });
    const tree = branchTwigs(base, field(0.04), { ...config, maxTurnPerStep: 5 }, params);
    let lateralCount = 0;
    for (let i = tree.crossover; i < tree.nodes.length; i++) {
      const parent = tree.nodes[i].parent;
      const lateral = tree.branchId[i - tree.crossover] === i &&
        tree.baseRadius[i - tree.crossover] < (parent < tree.crossover ? 0.04 : tree.baseRadius[parent - tree.crossover]) &&
        arrival(tree, i).dot(arrival(tree, parent)) < 0.999;
      const angle = arrival(tree, i).angleTo(arrival(tree, parent)) * 180 / Math.PI;
      expect(angle).toBeCloseTo(lateral ? 45 : 0, 5);
      lateralCount += Number(lateral);
    }
    expect(lateralCount).toBeGreaterThan(2);
  });

  it("keys bud variation by seed and lineage, with an exact zero-variation path", () => {
    const params = resolveTwigs({ angleVariation: 10, vigourVariation: 0.15, twig: branchesOnly });
    const a = branchTwigs(base, field(0.04), config, params, 17);
    expect(branchTwigs(base, field(0.04), config, params, 17)).toEqual(a);
    expect(signature(branchTwigs(base, field(0.04), config, params, 18))).not.toBe(signature(a));
    const zero = resolveTwigs({ angleVariation: 0, vigourVariation: 0, twig: branchesOnly });
    expect(branchTwigs(base, field(0.04), config, zero, 17))
      .toEqual(branchTwigs(base, field(0.04), config, zero, 18));
    const ratios = new Set<number>();
    for (let i = a.crossover; i < a.nodes.length; i++) {
      if (a.twig[i - a.crossover] || a.branchId[i - a.crossover] !== i) continue;
      const parent = a.nodes[i].parent;
      if (parent < a.crossover) continue;
      const ratio = (a.baseRadius[i - a.crossover] / a.baseRadius[parent - a.crossover]) ** (1 / params.ratioPower);
      expect(ratio).toBeGreaterThanOrEqual(params.lengthRatio * (1 - params.vigourVariation) - 1e-12);
      expect(ratio).toBeLessThanOrEqual(params.lengthRatio * (1 + params.vigourVariation) + 1e-12);
      const departure = arrival(a, i).angleTo(arrival(a, parent)) * 180 / Math.PI;
      expect(departure).toBeGreaterThanOrEqual(35 - 1e-10);
      expect(departure).toBeLessThanOrEqual(55 + 1e-10);
      ratios.add(ratio);
    }
    expect(ratios.size).toBeGreaterThan(3);
  });

  it("bears a twig at every station of the last order, spaced no closer than a twig's length, and no lateral branch there", () => {
    const params = resolveTwigs({ angleVariation: 0, vigourVariation: 0 });
    const tree = branchTwigs(base, field(0.1), config, params);
    const bearing = params.twig.bearingDiameter / 2;
    const runs = new Map<number, number[]>();
    for (let i = tree.crossover; i < tree.nodes.length; i++) {
      const id = tree.branchId[i - tree.crossover];
      const run = runs.get(id) ?? []; run.push(i); runs.set(id, run);
    }
    let bearingBranches = 0, twigsOnBearing = 0;
    for (const [id, run] of runs) {
      if (tree.twig[id - tree.crossover]) continue;
      const radius = tree.baseRadius[id - tree.crossover];
      const children = (node: number) => tree.nodes.map((n, i) => n.parent === node && i >= tree.crossover ? i : -1).filter(i => i >= 0);
      if (radius > bearing) {
        // Thicker wood bears no twig at its interior stations.
        for (const node of run.slice(0, -1)) for (const child of children(node)) {
          if (tree.branchId[child - tree.crossover] === id) continue;
          expect(tree.twig[child - tree.crossover]).toBe(0);
        }
        continue;
      }
      bearingBranches++;
      for (const [k, node] of run.entries()) {
        const step = tree.nodes[node].position.distanceTo(tree.nodes[tree.nodes[node].parent].position);
        expect(step).toBeGreaterThanOrEqual(params.twig.length - 1e-9);
        const side = children(node).filter(child => tree.branchId[child - tree.crossover] !== id);
        if (k < run.length - 1) {
          // One twig per interior station, never a lateral branch.
          expect(side).toHaveLength(1);
          expect(tree.twig[side[0] - tree.crossover]).toBe(1);
          twigsOnBearing++;
        }
      }
    }
    expect(bearingBranches).toBeGreaterThanOrEqual(2);
    expect(twigsOnBearing).toBeGreaterThan(bearingBranches);
  });

  it("grows internode runs with lateral radii from the law and terminal twigs as the sole radius exception", () => {
    const params = resolveTwigs({ lengthRatio: 0.4, angleVariation: 0, vigourVariation: 0, twig: branchesOnly });
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
        expect(node.position.distanceTo(tree.nodes[node.parent].position)).toBeCloseTo(params.twig.length, 12);
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
      const origin = tree.nodes[id].parent;
      const length = run.reduce((sum, i) => sum + tree.nodes[i].position.distanceTo(tree.nodes[tree.nodes[i].parent].position), 0);
      expect(run).toHaveLength(Math.max(1, Math.round(length / internodeLength(
        tree.baseRadius[id - tree.crossover], length, params.internodeFactor, params.twig.internodeLength))));
      const extraChildren = run.reduce((sum, i) => sum + counts[i] - 1, 0);
      expect(extraChildren).toBe(params.laterals);
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
      expect(node.position.distanceTo(tree.nodes[node.parent].position)).toBeCloseTo(DEFAULT_TWIG_ANATOMY.length, 12);
    }
    if (radius <= DEFAULT_TWIG_ANATOMY.diameter / 2) expect(tree.twig).toEqual(new Uint8Array([1]));
  });

  it("collapses a sub-internode branch to one twig", () => {
    const twig = { ...DEFAULT_TWIG_ANATOMY, internodeLength: 10 };
    const tree = branchTwigs(base, field(0.01), config, resolveTwigs({ twig }));
    expect(tree.twig).toEqual(new Uint8Array([1]));
    expect(tree.baseRadius[0]).toBe(twig.diameter / 2);
    expect(tree.nodes[2].position.distanceTo(base.nodes[1].position)).toBe(twig.length);
  });

  it("reports a nonconverging level cap and the node ceiling without calling either a twig", () => {
    const params = resolveTwigs({ internodeFactor: 32, laterals: 1, angleVariation: 0, vigourVariation: 0, lengthRatio: 1 });
    const tree = branchTwigs(base, field(0.1), config, params);
    expect(tree.levelCapped).toBe(true);
    const depth = new Int32Array(tree.nodes.length);
    for (let i = 2; i < tree.nodes.length; i++) {
      const k = i - 2, parent = tree.nodes[i].parent;
      depth[i] = depth[parent] + Number(!tree.twig[k] && tree.branchId[k] === i && parent >= 2);
    }
    expect(Math.max(...depth)).toBe(MAX_TWIG_LEVELS - 1);
    const boundary = branchTwigs(base, field(0.0025 * 2 ** MAX_TWIG_LEVELS), config,
      resolveTwigs({ internodeFactor: 32, laterals: 1, angleVariation: 0, vigourVariation: 0, lengthRatio: 0.5, ratioPower: 1 }));
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
    const params = resolveTwigs({ internodeFactor: 32, laterals: 1, angleVariation: 0, vigourVariation: 0, divergence: 0, angle: 45, limbRadius: 1 });
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
    const wants: { position: Vector3; direction: Vector3 }[] = [];
    const lean = createGrowthBias(DEFAULT_ENVELOPE, 1, { ...NO_BIAS, lean: 0.5 });
    const biased = { ...config, maxTurnPerStep: 26, bias: (p: Vector3, wanted: Vector3, step: number) => {
      calls++; expect(step).toBe(config.stepDistance);
      wants.push({ position: p.clone(), direction: wanted.clone() });
      return lean(p, wanted, step);
    } };
    const tree = branchTwigs(base, field(0.04), biased, resolveTwigs());
    expect(calls).toBeGreaterThanOrEqual(tree.nodes.length - 2);
    for (let i = 2; i < tree.nodes.length; i++) {
      const origin = tree.nodes[tree.nodes[i].parent].position;
      const candidates = wants.filter(w => w.position.equals(origin));
      expect(candidates.some(w => arrival(tree, i).dot(w.direction) >= Math.cos(26 * Math.PI / 180) - 1e-10)).toBe(true);
    }
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
        expect(n.position.distanceTo(tree.nodes[n.parent].position)).toBeCloseTo(p.twigs.twig.length, 10);
      }
    }
  }, 60000);

  it("seeds eligible limbs and tips with the same arrival frame, law and node phase", () => {
    const wood = { nodes: [...base.nodes,
      { position: new Vector3(0, 20, 0), parent: 1 },
      { position: new Vector3(0, 30, 0), parent: 2 },
    ] };
    const radii = field(0.05, 4);
    radii.radius[0] = 1;
    radii.radius[3] = 0.1;
    const params = resolveTwigs({ limbRadius: 0.1, laterals: 2, angleVariation: 0, vigourVariation: 0 });
    const tree = branchTwigs(wood, radii, config, params);
    const lateralRadius = childRadius(0.05, params.lengthRatio, params.ratioPower);
    for (const origin of [1, 2]) {
      const children = tree.nodes.map((n, i) => n.parent === origin && i >= tree.crossover ? i : -1).filter(i => i >= 0);
      expect(children).toHaveLength(2);
      for (const [station, child] of children.entries()) {
        expect(tree.baseRadius[child - tree.crossover]).toBe(lateralRadius);
        const phase = ((origin * params.divergence * Math.PI / 180) % (2 * Math.PI))
          + params.divergence * Math.PI / 180 + station * Math.PI;
        const expected = new Vector3(Math.sin(phase) * Math.SQRT1_2, Math.SQRT1_2, Math.cos(phase) * Math.SQRT1_2);
        expect(arrival(tree, child).distanceTo(expected)).toBeLessThan(1e-12);
        expect(tree.nodes[child].position.distanceTo(wood.nodes[origin].position))
          .toBeCloseTo(branchLength(0.05) * params.lengthRatio / Math.max(1, Math.round(
            branchLength(0.05) * params.lengthRatio / internodeLength(lateralRadius,
              branchLength(0.05) * params.lengthRatio, params.internodeFactor,
              lateralRadius <= params.twig.bearingDiameter / 2 ? params.twig.length : params.twig.internodeLength))), 12);
      }
    }
    // Equality is outside the strict radius threshold; the tip still has its leader.
    expect(tree.nodes.filter((n, i) => i >= tree.crossover && n.parent === 3)).toHaveLength(1);
    expect(tree.nodes.filter((n, i) => i >= tree.crossover && n.parent === 0)).toHaveLength(0);
    expect(branchTwigs(wood, radii, config, resolveTwigs({ limbRadius: 0 })).nodes
      .filter((n, i) => i >= wood.nodes.length && n.parent === 1)).toHaveLength(0);
  });

  it.each(["origin", "candidate"])("guards the %s of an interior lateral at the bare-trunk line", endpoint => {
    const wood = { nodes: [
      { position: new Vector3(0, endpoint === "candidate" ? 20 : 0, 0), parent: -1 },
      { position: new Vector3(0, endpoint === "candidate" ? 10.001 : 9.999, 0), parent: 0 },
      { position: new Vector3(0, 11, 0), parent: 1 },
    ] };
    const radii = field(0.05, 3); radii.radius[0] = 1;
    const tree = branchTwigs(wood, radii, { ...config, trunkHeight: 10 }, resolveTwigs({ limbRadius: 0.1 }));
    expect(tree.nodes.filter((n, i) => i >= tree.crossover && n.parent === 1)).toHaveLength(0);
    for (const node of tree.nodes.slice(tree.crossover)) {
      expect(tree.nodes[node.parent].position.y).toBeGreaterThanOrEqual(10);
      expect(node.position.y).toBeGreaterThanOrEqual(10);
    }
  });

  it("R8 preserves whole lateral subtrees on a round-boundary prefix under the mature field", () => {
    const p = TELPERION.skeleton;
    const cfg = resolveGrowth(p);
    const attractors = sampleEnvelope(p.envelope, p.attractors, createRng(p.seed));
    const mature = colonize(attractors, new Vector3(), cfg);
    const radii = solveRadii(mature, p.envelope, TELPERION.radii);
    // The bias is evaluated for every candidate in a round before any node
    // is appended. An extra callback when the cap advances by one proves
    // that the previous cap ended a complete round, rather than part of it.
    const capped = (maxNodes: number) => {
      let calls = 0;
      const tree = colonize(attractors, new Vector3(), { ...cfg, maxNodes,
        bias: (position, wanted, step) => { calls++; return cfg.bias!(position, wanted, step); },
      });
      return { tree, calls };
    };
    let boundary = Math.floor(mature.nodes.length / 5);
    let prefix = capped(boundary);
    for (; boundary < mature.nodes.length - 1; boundary++) {
      const next = capped(boundary + 1);
      if (next.calls > prefix.calls) break;
      prefix = next;
    }
    expect(boundary).toBeLessThan(mature.nodes.length - 1);
    expect(prefix.tree.nodes).toEqual(mature.nodes.slice(0, boundary));
    const params = resolveTwigs({ ...p.twigs, limbRadius: 0.2, laterals: 1 });
    const young = branchTwigs(prefix.tree, { radius: radii.radius.slice(0, boundary),
      startRadius: radii.startRadius.slice(0, boundary) }, cfg, params, p.seed);
    const full = branchTwigs(mature, radii, cfg, params, p.seed);
    expect(young.nodeCapped || full.nodeCapped).toBe(false);
    const subtrees = (tree: ReturnType<typeof branchTwigs>) => {
      const children: number[][] = tree.nodes.map(() => []);
      tree.nodes.forEach((n, i) => { if (n.parent >= 0) children[n.parent].push(i); });
      const encode = (i: number): unknown => [tree.nodes[i].position.toArray(),
        tree.baseRadius[i - tree.crossover], tree.twig[i - tree.crossover],
        tree.branchId[i - tree.crossover] === i, children[i].map(encode)];
      return Array.from({ length: boundary }, (_, origin) => children[origin]
        .filter(i => i >= tree.crossover && tree.baseRadius[i - tree.crossover] < radii.radius[origin])
        .map(encode));
    };
    const a = subtrees(young), b = subtrees(full);
    expect(a.filter(laterals => laterals.length > 0).length).toBeGreaterThan(10);
    expect(JSON.stringify(a)).toBe(JSON.stringify(b));
    const youngParents = new Set(prefix.tree.nodes.map(n => n.parent));
    const matureParents = new Set(mature.nodes.map(n => n.parent));
    const formerTips = [...matureParents].filter(i => i > 0 && i < boundary && !youngParents.has(i));
    expect(formerTips.filter(i => a[i].length > 0).length).toBeGreaterThan(0);
    for (const origin of formerTips) {
      expect(young.nodes.filter((n, i) => i >= young.crossover && n.parent === origin
        && young.baseRadius[i - young.crossover] === radii.radius[origin])).toHaveLength(1);
      expect(full.nodes.filter((n, i) => i >= full.crossover && n.parent === origin
        && full.baseRadius[i - full.crossover] === radii.radius[origin])).toHaveLength(0);
    }
    expect(branchTwigs(prefix.tree, radii, cfg, params).refused).toBe("radius-length-mismatch");
  });

  it("resolves finite rails and non-finite defaults without a levels key", () => {
    expect(resolveTwigs()).toEqual(DEFAULT_TWIGS);
    expect(resolveTwigs({ angle: NaN, divergence: Infinity, internodeFactor: NaN, angleVariation: NaN, vigourVariation: Infinity, laterals: Infinity,
      lengthRatio: NaN, ratioPower: Infinity, limbRadius: NaN })).toEqual(DEFAULT_TWIGS);
    expect(resolveTwigs({ internodeFactor: 0, angleVariation: -1, vigourVariation: 2, laterals: -1, lengthRatio: 0, ratioPower: 99, angle: 99, limbRadius: -1 }))
      .toMatchObject({ internodeFactor: 0.05, angleVariation: 0, vigourVariation: 0.95, laterals: 0, lengthRatio: 0.05, ratioPower: 8, angle: 90, limbRadius: 0 });
    expect(resolveTwigs({ limbRadius: 2 }).limbRadius).toBe(1);
    expect(DEFAULT_TWIGS).not.toHaveProperty("levels");
  });
});
