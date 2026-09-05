import { describe, expect, it } from "vitest";

import { buildSurface } from "../mesh/surface";
import { solveRadii } from "../radius";
import { buildCanopy } from "../canopy/place";
import { DEFAULT_ELEMENT } from "../canopy/element";
import {
  growReport,
  growSkeleton,
  type SkeletonParams,
} from "../skeleton/grow";

import { getPreset, PRESETS } from "./index";
import type { TreePreset } from "./preset";
import { LAURELIN, TELPERION } from "./two-trees";

/* ------------------------------------------------------------------ *
 * Whether Telperion and Laurelin read as two deliberate trees is the
 * owner's judgement and nothing here claims to make it. What a box
 * with no browser CAN hold to account is the half of that claim which
 * is measurable: that the two are one algorithm with different
 * numbers, and that the numbers actually come out as the difference
 * they were authored to be.
 *
 * So the measurements below are taken on the generated skeleton, not
 * on the parameters. Asserting that LAURELIN.spread exceeds
 * TELPERION.spread only restates the file; asserting that the tree
 * that comes out is three times as wide relative to its height is the
 * claim that could fail - a parameter can be authored and then eaten
 * by a clamp, a rail or a sampling floor somewhere downstream, which
 * is exactly what happened to the writhe wavelength in fn-11.3.
 * ------------------------------------------------------------------ */

/** What a preset's skeleton actually measures, in units of its own
 *  envelope height so the two trees are comparable at different
 *  sizes. */
function measure(preset: TreePreset) {
  const skeleton = growSkeleton(preset.skeleton, preset.radii);
  const height = preset.skeleton.envelope.height;

  let reach = 0;
  let top = 0;
  for (const node of skeleton.nodes) {
    reach = Math.max(reach, Math.hypot(node.position.x, node.position.z));
    top = Math.max(top, node.position.y);
  }

  // The lowest node that more than one node grows out of: where the
  // tree stops being a trunk. Children are counted rather than stored,
  // which is the skeleton's own representation.
  const childCount = new Array<number>(skeleton.nodes.length).fill(0);
  for (const node of skeleton.nodes) {
    if (node.parent >= 0) childCount[node.parent] += 1;
  }
  let firstFork = Number.POSITIVE_INFINITY;
  skeleton.nodes.forEach((node, index) => {
    if (childCount[index] > 1) {
      firstFork = Math.min(firstFork, node.position.y);
    }
  });

  return {
    nodes: skeleton.nodes.length,
    /** Full crown width over height. Above 1 the tree is wider than it
     *  is tall. */
    aspect: (2 * reach) / height,
    /** How high the crown reaches, as a fraction of its envelope. */
    reachedTop: top / height,
    /** Where the trunk ends, as a fraction of height. */
    forkAt: firstFork / height,
  };
}

/** The median branch tip's radius as a fraction of the trunk's: how
 *  far this tree runs from its own trunk to its own twigs. A ratio
 *  rather than a length, so two trees of different heights and
 *  different girths are still comparable. */
function tipShare(preset: TreePreset): number {
  const skeleton = growSkeleton(preset.skeleton, preset.radii);
  const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);

  const childCount = new Array<number>(skeleton.nodes.length).fill(0);
  for (const node of skeleton.nodes) {
    if (node.parent >= 0) childCount[node.parent] += 1;
  }
  const tips = skeleton.nodes
    .map((_, index) => index)
    .filter((index) => childCount[index] === 0)
    .map((index) => field.radius[index])
    .sort((a, b) => a - b);

  return tips[Math.floor(tips.length / 2)] / field.radius[0];
}

describe("the two trees are two of the same generator", () => {
  it("both run the identical three calls, differing only in arguments", () => {
    /* The acceptance criterion, as literally as a test can put it: one
       sequence of library calls, run twice, with the preset as the only
       thing that changes between the runs. If either tree ever needed a
       branch of its own, this loop is where it would not fit. */
    for (const preset of PRESETS) {
      const skeleton = growSkeleton(preset.skeleton, preset.radii);
      const field = solveRadii(
        skeleton,
        preset.skeleton.envelope,
        preset.radii,
      );
      const surface = buildSurface(
        skeleton,
        field,
        preset.skeleton.envelope,
        preset.surface,
      );

      expect(skeleton.nodes.length).toBeGreaterThan(100);
      expect(surface.triangles).toBeGreaterThan(1000);
      expect(surface.positions.every(Number.isFinite)).toBe(true);
    }
  }, 60_000);

  it("is deterministic: the same preset builds the same skeleton twice", () => {
    for (const preset of PRESETS) {
      const first = growSkeleton(preset.skeleton, preset.radii);
      const second = growSkeleton(preset.skeleton, preset.radii);
      expect(first.nodes.length).toBe(second.nodes.length);
      expect(first.nodes[first.nodes.length - 1].position.toArray()).toEqual(
        second.nodes[second.nodes.length - 1].position.toArray(),
      );
    }
  });

  it("neither preset stalls against the generator's node ceiling", () => {
    /* A preset that reaches the ceiling has been cut off rather than
       finished - the tree on screen would then be the ceiling's shape
       and not the envelope's. Read both cap flags from the library's
       report: the law must finish before either safety stop. */
    for (const preset of PRESETS) {
      const report = growReport(preset.skeleton, preset.radii);
      expect(report.capped).toBe(false);
      expect(report.levelCapped).toBe(false);
    }
  });

  it("bears its leaves on twigs, not on timber: the leaf is a multiple of the wood", () => {
    /* R3, on both presets at the depth they ship at. Before the second
       pass a 12 cm leaf sat on 79 cm wood, a ratio of 0.15 to 1; the
       botanical relationship is the other way, a leaf several times the
       diameter of the twig that bears it. Measured at the median marked twig so
       one stray fine twig cannot pass the crown. */
    for (const preset of PRESETS) {
      const skeleton = growSkeleton(preset.skeleton, preset.radii);
      const field = solveRadii(skeleton, preset.skeleton.envelope, preset.radii);
      const tips = Array.from(skeleton.twig)
        .flatMap((mark, index) => mark === 1 ? [2 * field.radius[index + skeleton.crossover]] : [])
        .sort((a, b) => a - b);
      expect(tips.length).toBeGreaterThan(0);
      for (const diameter of tips) expect(diameter).toBeCloseTo(preset.skeleton.twigs.twig.diameter, 12);
      const medianTwig = tips[Math.floor(tips.length / 2)];
      const leaf = DEFAULT_ELEMENT.length * preset.canopy.size;
      expect(leaf / medianTwig).toBeGreaterThan(10);
      const canopy = buildCanopy(skeleton, field, preset.skeleton.envelope,
        preset.skeleton.seed, preset.canopy, preset.skeleton.twigs.twig);
      expect(canopy.count).toBe(tips.length * preset.skeleton.twigs.twig.stationsPerInternode);
      let leafIndex = 0;
      let worstOffsetError = 0;
      for (let i = skeleton.crossover; i < skeleton.nodes.length; i++) {
        if (skeleton.twig[i - skeleton.crossover] !== 1) continue;
        const foot = skeleton.nodes[skeleton.nodes[i].parent].position;
        for (let station = 0; station < preset.skeleton.twigs.twig.stationsPerInternode; station++) {
          const offset = leafIndex++ * 16 + 12;
          const distance = Math.hypot(canopy.matrices[offset] - foot.x,
            canopy.matrices[offset + 1] - foot.y, canopy.matrices[offset + 2] - foot.z);
          worstOffsetError = Math.max(worstOffsetError, Math.abs(distance - field.startRadius[i]));
        }
      }
      // Packed float32 positions at these 150 m coordinates lose micrometres.
      expect(worstOffsetError).toBeLessThan(2e-5);
    }
  }, 60_000);
});

describe("Laurelin is broad, domed and spreading", () => {
  const laurelin = measure(LAURELIN);
  const telperion = measure(TELPERION);

  it("comes out wider than it is tall", () => {
    // No ordinary tree does this, and it is the single most legible
    // thing about Laurelin in every reference.
    expect(laurelin.aspect).toBeGreaterThan(1);
  });

  it("is at least twice as broad as Telperion for its height", () => {
    // Measured, not authored: the envelope's spread is only a request,
    // and what matters is that the growth actually fills it out.
    expect(laurelin.aspect).toBeGreaterThan(2 * telperion.aspect);
  });

  it("forks low, into the bottom third of its own height", () => {
    expect(laurelin.forkAt).toBeLessThan(0.34);
  });
});

describe("Telperion is narrower, finer and more upright", () => {
  const laurelin = measure(LAURELIN);
  const telperion = measure(TELPERION);

  it("stands on a bare trunk that reaches well above Laurelin's", () => {
    expect(telperion.forkAt).toBeGreaterThan(laurelin.forkAt + 0.1);
  });

  it("is the finer tree where fineness shows: at the tips", () => {
    /* Not in branch count - a crown a quarter the size holds fewer
       nodes however many attractors it is offered, and Laurelin comes
       out with three times as many. Fineness is the twig, and the twig
       is `forkExponent`'s: Telperion sheds nearer its area at every
       fork and runs further from trunk to tip. Measured as a fraction
       of each tree's own trunk, so it is a proportion and not a
       consequence of the two being different sizes. */
    expect(tipShare(TELPERION)).toBeLessThan(tipShare(LAURELIN));
  });

  it("carries its crown to the top of its envelope", () => {
    // Upright means the growth goes up. A gravitropism this strong
    // that failed to reach the tip would be a clamp eating it.
    expect(telperion.reachedTop).toBeGreaterThan(0.9);
  });
});

describe("the preset registry", () => {
  it("offers both trees, the elder first", () => {
    expect(PRESETS.map((preset) => preset.id)).toEqual([
      "telperion",
      "laurelin",
    ]);
  });

  it("finds a preset by id", () => {
    expect(getPreset("laurelin")).toBe(LAURELIN);
  });

  it("returns undefined for an id nobody ships", () => {
    // An id can arrive from a panel, a query string or a saved
    // session, and none of those is a guarantee.
    expect(getPreset("silpion")).toBeUndefined();
  });

  it("states every generator argument, inheriting none", () => {
    /* A preset that spread a default and overrode four fields would be
       a tree whose look is partly authored and partly a constant
       elsewhere, which is the "no magic constants" principle broken in
       the one file that exists to uphold it. Checked structurally, so
       a term added to any of the three argument types fails here until
       both trees say what they want it to be. */
    for (const preset of PRESETS) {
      const skeleton: Required<SkeletonParams> = preset.skeleton;
      expect(Object.keys(skeleton).sort()).toEqual([
        "attractors",
        "bias",
        "envelope",
        "growth",
        "seed",
        "step",
        "twigs",
      ]);
      expect(Object.keys(skeleton.envelope).sort()).toEqual([
        "crownBase",
        "fullness",
        "height",
        "shoulder",
        "spread",
      ]);
      expect(Object.keys(skeleton.bias).sort()).toEqual([
        "gravitropism",
        "lean",
        "spiralRate",
        "writheAmplitude",
        "writheWavelength",
      ]);
      expect(Object.keys(skeleton.twigs).sort()).toEqual([
        "angle",
        "divergence",
        "internodes",
        "laterals",
        "lengthRatio",
        "limbRadius",
        "ratioPower",
        "twig",
      ]);
      expect(Object.keys(skeleton.twigs.twig).sort()).toEqual(["diameter", "internodeLength", "stationsPerInternode"]);
      expect(Object.keys(preset.radii).sort()).toEqual([
        "forkExponent",
        "lengthTaper",
        "trunkRadius",
      ]);
      expect(Object.keys(preset.surface).sort()).toEqual([
        "flareDepth",
        "flareFalloff",
        "flareRadius",
        "forkSocket",
        "forkSwell",
        "lobeDepth",
        "lobes",
        "radialSegments",
        "twistRate",
      ]);
    }
  });

  it("sizes both trunks for the tree they carry, not for a 24 m one", () => {
    /* Elastic similarity: a self-supporting trunk's diameter goes as
       height^1.5, so as a FRACTION of height it goes as height^0.5.
       The library is scale-free and will happily draw a 148 m tree as
       slender as a 24 m one - saying otherwise is the preset's job,
       and this is that job done or not done. */
    for (const preset of PRESETS) {
      const height = preset.skeleton.envelope.height;
      const similar = 0.02 * Math.sqrt(height / 24);
      expect(preset.radii.trunkRadius).toBeGreaterThanOrEqual(similar * 0.95);
    }
  });
});
