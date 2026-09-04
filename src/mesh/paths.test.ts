import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE } from "@/lib/grower/envelope";
import { branchPaths } from "@/lib/grower/mesh/paths";
import { DEFAULT_RADII, solveRadii } from "@/lib/grower/radius";
import type { Skeleton } from "@/lib/grower/skeleton/colonize";
import { growSkeleton } from "@/lib/grower/skeleton/grow";

/* The decomposition is what makes a continuous surface possible at
   all: a run is swept once, with one ring per node, so there are no
   two ends to fail to meet. These check that the runs cover the
   skeleton exactly - every edge once, no edge twice - because a
   missing edge is a limb that is not drawn and a repeated one is two
   surfaces fighting for the same space. */

function paths(skeleton: Skeleton) {
  return branchPaths(
    skeleton,
    solveRadii(skeleton, DEFAULT_ENVELOPE, DEFAULT_RADII),
  );
}

const at = (x: number, y: number, z: number, parent: number) => ({
  position: new THREE.Vector3(x, y, z),
  parent,
});

/** A trunk that forks: root, two trunk nodes, then a thick limb and a
 *  thin one. */
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

describe("branchPaths", () => {
  it("cuts the skeleton into runs that cover every edge exactly once", () => {
    const runs = paths(forked);
    const seen = new Set<string>();
    for (const run of runs) {
      for (let i = 1; i < run.nodes.length; i += 1) {
        const edge = `${run.nodes[i - 1]}->${run.nodes[i]}`;
        expect(seen.has(edge)).toBe(false);
        seen.add(edge);
      }
    }
    expect(seen.size).toBe(forked.nodes.length - 1);
  });

  it("starts a side run at the fork it leaves, not at its own node", () => {
    /* This is the fork half of the continuity fix. A run that began at
       its own first node would leave a gap between it and the parent's
       surface; beginning at the fork gives the sweep somewhere inside
       the parent to start from. */
    const runs = paths(forked);
    const trunk = runs.filter((run) => run.trunk);
    expect(trunk).toHaveLength(1);
    expect(trunk[0].nodes[0]).toBe(0);
    for (const run of runs.filter((r) => !r.trunk)) {
      const fork = run.nodes[0];
      expect(forked.nodes[run.nodes[1]].parent).toBe(fork);
      // The fork node belongs to some other run as well: that is what
      // "the child leaves the parent's surface" means structurally.
      expect(
        runs.some((other) => other !== run && other.nodes.includes(fork)),
      ).toBe(true);
    }
  });

  it("carries the limb on through the thickest child", () => {
    // Thickness is the radius solve's answer to which limb is the
    // leader; the surface is not entitled to a second opinion.
    const [trunk] = paths(forked);
    expect(trunk.nodes).toEqual([0, 1, 2, 3, 5]);
  });

  it("folds a zero-length step onto its parent, keeping the subtree", () => {
    // A repeated node is a ring of degenerate triangles. Dropping the
    // node outright would drop everything grown from it with it.
    const doubled: Skeleton = {
      nodes: [at(0, 0, 0, -1), at(0, 0, 0, 0), at(0, 1, 0, 1)],
    };
    expect(paths(doubled)).toEqual([{ nodes: [0, 2], trunk: true }]);
  });

  it("has no run in a skeleton that never grew", () => {
    expect(paths({ nodes: [at(0, 0, 0, -1)] })).toEqual([]);
  });

  it("covers a grown tree edge for edge", () => {
    const skeleton = growSkeleton({
      seed: 3,
      envelope: DEFAULT_ENVELOPE,
      attractors: 400,
    });
    const runs = paths(skeleton);
    const drawn = new Set<number>();
    for (const run of runs) {
      for (let i = 1; i < run.nodes.length; i += 1) drawn.add(run.nodes[i]);
    }
    // Every node but the root is the far end of exactly one swept
    // segment, so nothing in the tree goes unskinned.
    expect(drawn.size).toBe(skeleton.nodes.length - 1);
  });
});
