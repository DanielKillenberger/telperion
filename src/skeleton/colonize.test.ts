import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE, envelopeRadiusAt } from "../envelope";
import { colonize, type GrowthConfig, type Skeleton } from "./colonize";

/* Colonization is the deterministic half: no seed reaches it, so the
   same attractors in the same order must always give the same nodes in
   the same order. The rest of these hold it to being a well-formed
   tree that terminates. */

const config: GrowthConfig = {
  stepDistance: 0.5,
  killDistance: 1,
  influenceRadius: 4.5,
  trunkHeight: 6,
  maxNodes: 4000,
};

const origin = new THREE.Vector3(0, 0, 0);

/** A reproducible cloud that does not depend on the library's own RNG,
 *  so a change to the RNG can never quietly rewrite these fixtures.
 *
 *  Broad and shallow on purpose: a wide crown is what puts a node
 *  between attractors pulling it in near-opposite directions, which is
 *  the case the termination guard below exists for. A tall narrow
 *  cloud never reaches it and would make that test prove nothing. */
function cloud(count: number): THREE.Vector3[] {
  const points: THREE.Vector3[] = [];
  for (let i = 0; i < count; i += 1) {
    const t = i / count;
    const angle = i * 2.399_963; // golden angle, in radians
    const radius = 9 * Math.sqrt(t);
    points.push(
      new THREE.Vector3(
        Math.cos(angle) * radius,
        6 + t * 4,
        Math.sin(angle) * radius,
      ),
    );
  }
  return points;
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

describe("colonize", () => {
  it.each([new THREE.Vector3(3, 8, 0), new THREE.Vector3(0, 20, 0)])("rejects crown candidates outside the shell while allowing the bare trunk to climb", attractor => {
    const shell = { ...DEFAULT_ENVELOPE, height: 12, crownBase: 0.5, spread: 0.1 };
    const outside = (p: THREE.Vector3) => p.y > shell.height || Math.hypot(p.x, p.z) > envelopeRadiusAt(shell, p.y);
    const crownConfig = { ...config, influenceRadius: 20,
      bias: attractor.x === 0 ? undefined : (position: THREE.Vector3, wanted: THREE.Vector3) =>
        position.y < config.trunkHeight ? wanted : wanted.clone().add(new THREE.Vector3(0.5, 0, 0)).normalize(),
    };
    const grown = colonize([attractor], origin, { ...crownConfig, shell });
    expect(grown.nodes.length).toBeGreaterThan(1);
    expect(grown.nodes.some(n => n.position.y >= config.trunkHeight)).toBe(true);
    for (const node of grown.nodes) if (node.position.y >= config.trunkHeight) expect(outside(node.position)).toBe(false);
    expect(colonize([attractor], origin, crownConfig).nodes.some(n => n.position.y >= config.trunkHeight && outside(n.position))).toBe(true);
  });

  it("lets the outside-starting trunk approach enter the shell before constraining crown departures", () => {
    const shell = { ...DEFAULT_ENVELOPE, height: 12, crownBase: 0.5, spread: 0.1 };
    const outside = (p: THREE.Vector3) => p.y > shell.height || Math.hypot(p.x, p.z) > envelopeRadiusAt(shell, p.y);
    const root = new THREE.Vector3(4, 8, 0);
    const grown = colonize([new THREE.Vector3(0, 8, 0)], root,
      { ...config, shell, influenceRadius: 20, killDistance: 0.1 });
    expect(outside(root)).toBe(true);
    expect(grown.nodes.length).toBeGreaterThan(2);
    expect(grown.nodes.some(n => !outside(n.position))).toBe(true);
    for (const node of grown.nodes.slice(1)) {
      if (!outside(grown.nodes[node.parent].position)) expect(outside(node.position)).toBe(false);
    }
  });

  it("is deterministic given the same attractors", () => {
    const attractors = cloud(300);
    expect(signature(colonize(attractors, origin, config))).toBe(
      signature(colonize(attractors, origin, config)),
    );
  });

  it("returns a lone root when there is nothing to grow toward", () => {
    const skeleton = colonize([], origin, config);
    expect(skeleton.nodes).toHaveLength(1);
    expect(skeleton.nodes[0].parent).toBe(-1);
  });

  it("emits a well-formed tree", () => {
    const skeleton = colonize(cloud(300), origin, config);
    expect(skeleton.nodes.length).toBeGreaterThan(50);
    expect(skeleton.nodes[0].parent).toBe(-1);
    for (let i = 1; i < skeleton.nodes.length; i += 1) {
      // Strictly lower, so one forward pass always meets a parent
      // before its children - what fn-11.4's radius solve will walk.
      expect(skeleton.nodes[i].parent).toBeGreaterThanOrEqual(0);
      expect(skeleton.nodes[i].parent).toBeLessThan(i);
    }
  });

  it("climbs to the crown before it branches", () => {
    const skeleton = colonize(cloud(300), origin, config);
    // Nothing is in reach at the origin, so the first nodes are a bare
    // trunk on the axis.
    expect(skeleton.nodes[1].position.x).toBe(0);
    expect(skeleton.nodes[1].position.z).toBe(0);
    expect(skeleton.nodes[1].position.y).toBeCloseTo(config.stepDistance, 10);
  });

  it("holds the trunk bare to the height it was given", () => {
    /* The influence radius is nine times the step, so the cloud is
       within reach of the trunk long before the trunk has climbed to
       it. Without an explicit trunk height the tree forks out in the
       open, below where its crown is supposed to start. */
    for (const node of colonize(cloud(300), origin, config).nodes) {
      if (Math.hypot(node.position.x, node.position.z) > 1e-9) {
        expect(node.position.y).toBeGreaterThan(
          config.trunkHeight - config.stepDistance,
        );
      }
    }
  });

  it("forks", () => {
    const children = new Map<number, number>();
    for (const node of colonize(cloud(300), origin, config).nodes) {
      if (node.parent >= 0) {
        children.set(node.parent, (children.get(node.parent) ?? 0) + 1);
      }
    }
    const forks = [...children.values()].filter((count) => count > 1);
    expect(forks.length).toBeGreaterThan(5);
  });

  it("never grows the same node twice in one direction", () => {
    /* The termination guard, and it is load-bearing: without it an
       attractor outvoted by the rest of its node's pull leaves that
       node unchanged, so the identical child is emitted every round
       until the node budget is gone. Before the guard this fixture
       produced 7809 duplicate nodes out of 8000. */
    const skeleton = colonize(cloud(300), origin, config);
    const seen = new Set<string>();
    for (const node of skeleton.nodes) {
      const key = `${node.parent}|${node.position.x}|${node.position.y}|${node.position.z}`;
      expect(seen.has(key)).toBe(false);
      seen.add(key);
    }
    // And it stops on its own rather than by running out of budget.
    expect(skeleton.nodes.length).toBeLessThan(config.maxNodes);
  });

  it("reaches what it grew toward", () => {
    const attractors = cloud(300);
    const skeleton = colonize(attractors, origin, config);
    const killSq = config.killDistance * config.killDistance;
    const reached = attractors.filter((attractor) =>
      skeleton.nodes.some(
        (node) => node.position.distanceToSquared(attractor) <= killSq,
      ),
    );
    expect(reached.length / attractors.length).toBeGreaterThan(0.9);
  });

  it("stops at maxNodes", () => {
    const capped = colonize(cloud(300), origin, { ...config, maxNodes: 60 });
    expect(capped.nodes).toHaveLength(60);
  });

  it("refuses to grow on a config that forbids it", () => {
    expect(
      colonize(cloud(300), origin, { ...config, stepDistance: 0 }).nodes,
    ).toHaveLength(1);
    expect(
      colonize(cloud(300), origin, { ...config, maxNodes: 1 }).nodes,
    ).toHaveLength(1);
  });

  it("does not grow a flagpole past an unreachable crown", () => {
    // Attractors far off the axis and out of reach of the trunk: the
    // climb has to stop at the top of the cloud, not at maxNodes.
    const unreachable = [new THREE.Vector3(400, 12, 0)];
    const skeleton = colonize(unreachable, origin, config);
    const top = skeleton.nodes[skeleton.nodes.length - 1].position.y;
    expect(top).toBeLessThanOrEqual(12 + config.stepDistance);
    expect(skeleton.nodes.length).toBeLessThan(config.maxNodes);
  });
});
