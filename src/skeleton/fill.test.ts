import { describe, expect, it } from "vitest";
import { Vector3 } from "three";
import type { Envelope } from "../envelope";
import { LAURELIN, TELPERION } from "../presets/two-trees";
import type { Skeleton } from "./colonize";
import { growReport } from "./grow";
import { shellOccupancy, tipClustering } from "./fill";

const crown: Envelope = { height: 40, crownBase: 0, spread: 0.5, fullness: 0.5, shoulder: 1 };
const skeleton = (positions: number[][], parents = positions.map((_, i) => i - 1)): Skeleton => ({
  nodes: positions.map(([x, y, z], i) => ({ position: new Vector3(x, y, z), parent: parents[i] })),
});

describe("crown fill measurements", () => {
  it("counts distinct occupied shell voxels in metres, clipped to the crown", () => {
    // Eight voxel centres lie inside this double cone at cellSize = H/4.
    // (5,15,5) has profile depth 5.61 m: inside the 9 m shell, beyond 0.45 m.
    const tree = skeleton([[5, 15, 5], [6, 15, 5], [0, 20, 0], [25, 15, 5], [0, -1, 0]]);
    expect(shellOccupancy(tree, new Uint8Array(5).fill(1), crown, 0.25))
      .toEqual({ tested: true, fraction: 1 / 8, numerator: 1, denominator: 8 });
    const scaled = skeleton(tree.nodes.map(({ position: p }) => [p.x / 10, p.y / 10, p.z / 10]));
    expect(shellOccupancy(scaled, new Uint8Array(5).fill(1), { ...crown, height: 4 }, 0.25))
      .toEqual(shellOccupancy(tree, new Uint8Array(5).fill(1), crown, 0.25));
  });

  it("classifies colonization tips using only the prefix, and includes the distance boundary", () => {
    const tree = skeleton([[0, 0, 0], [1, 0, 0], [2, 0, 0], [3, 0, 0], [9, 0, 0]], [-1, 0, 1, 2, 2]);
    expect(tipClustering(tree, 3, new Uint8Array([0, 0, 0, 1, 1]), 1))
      .toEqual({ tested: true, fraction: 0.5, numerator: 1, denominator: 2 });
    expect(tipClustering(tree, 3, new Uint8Array([0, 0, 0, 1, 1]), 0))
      .toEqual({ tested: true, fraction: 0, numerator: 0, denominator: 2 });
  });

  it("reports an empty classification or unmeasurable domain as untested", () => {
    const tree = skeleton([[0, 0, 0]]);
    expect(shellOccupancy(tree, new Uint8Array(1), crown, 0.02))
      .toEqual({ tested: false, reason: "no-terminals" });
    expect(tipClustering(tree, 1, new Uint8Array(1), 1))
      .toEqual({ tested: false, reason: "no-terminals" });
    expect(shellOccupancy(tree, new Uint8Array([1]), crown, 0))
      .toEqual({ tested: false, reason: "invalid-cell-size" });
    expect(shellOccupancy(tree, new Uint8Array([1]), { ...crown, spread: 0 }, 0.02))
      .toEqual({ tested: false, reason: "invalid-envelope" });
    expect(shellOccupancy(tree, new Uint8Array([1]), crown, 1e-9))
      .toEqual({ tested: false, reason: "grid-too-large" });
    expect(shellOccupancy(tree, new Uint8Array(0), crown, 0.02))
      .toEqual({ tested: false, reason: "terminal-size-mismatch" });
    expect(tipClustering(tree, 1, new Uint8Array([1]), 1))
      .toEqual({ tested: false, reason: "no-colonization-tips" });
    expect(tipClustering(tree, 2, new Uint8Array([1]), 1))
      .toEqual({ tested: false, reason: "invalid-crossover" });
    expect(tipClustering(tree, 1, new Uint8Array([1]), NaN))
      .toEqual({ tested: false, reason: "invalid-distance" });
    expect(shellOccupancy(tree, new Uint8Array([1]), crown, 10))
      .toEqual({ tested: false, reason: "no-shell-cells" });
    const invalidTerminal = skeleton([[NaN, 0, 0]]);
    expect(shellOccupancy(invalidTerminal, new Uint8Array([1]), crown))
      .toEqual({ tested: false, reason: "non-finite-terminal" });
    const invalidParent = skeleton([[0, 0, 0], [1, 0, 0]], [-1, 1]);
    expect(tipClustering(invalidParent, 2, new Uint8Array([0, 1]), 1))
      .toEqual({ tested: false, reason: "invalid-colonization-topology" });
    const invalidTip = skeleton([[0, 0, 0], [NaN, 0, 0], [1, 0, 0]]);
    expect(tipClustering(invalidTip, 2, new Uint8Array([0, 0, 1]), 1))
      .toEqual({ tested: false, reason: "non-finite-colonization-tip" });
  });

  it.each([TELPERION, LAURELIN])("records the unchanged $name tuft and zero-order baseline", (preset) => {
    for (const levels of [0, 8]) {
      const report = growReport({ ...preset.skeleton, twigs: { ...preset.skeleton.twigs, levels } });
      const tree = report.skeleton;
      const terminal = new Uint8Array(tree.nodes.length);
      terminal.fill(1, tree.crossover);
      for (let i = 1; i < tree.nodes.length; i += 1) terminal[tree.nodes[i].parent] = 0;
      const occupancy = shellOccupancy(tree, terminal, preset.skeleton.envelope, 0.02);
      const clustering = tipClustering(tree, tree.crossover, terminal, preset.skeleton.envelope.height * 0.05);
      expect(report.capped).toBe(false);
      expect(occupancy.tested).toBe(levels > 0);
      expect(clustering.tested).toBe(levels > 0);
      process.stdout.write(JSON.stringify({ preset: preset.id, levels, nodes: tree.nodes.length,
        crossover: tree.crossover, terminals: terminal.reduce((sum, mark) => sum + mark, 0),
        cellSizeHeightFraction: 0.02, distanceHeightFraction: 0.05, occupancy, clustering }) + "\n");
    }
  }, 60_000);
});
