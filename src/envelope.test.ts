import { describe, expect, it } from "vitest";

import {
  DEFAULT_ENVELOPE,
  envelopeContains,
  envelopeMaxRadius,
  envelopeRadiusAt,
  sampleEnvelope,
  type Envelope,
} from "./envelope";
import { createRng } from "./rng";

/* The envelope is the authored half of the generator, so these are the
   tests that say what "authored" means: the silhouette is exactly what
   the profile describes, and every attractor the branching will chase
   is inside it. */

const narrow: Envelope = { ...DEFAULT_ENVELOPE, spread: 0.2, shoulder: 1.2 };
const broad: Envelope = { ...DEFAULT_ENVELOPE, spread: 1.2, shoulder: 4 };

describe("envelopeRadiusAt", () => {
  it("is zero outside the crown", () => {
    const { height, crownBase } = DEFAULT_ENVELOPE;
    expect(envelopeRadiusAt(DEFAULT_ENVELOPE, -1)).toBe(0);
    expect(envelopeRadiusAt(DEFAULT_ENVELOPE, height * crownBase)).toBe(0);
    expect(envelopeRadiusAt(DEFAULT_ENVELOPE, height)).toBe(0);
    expect(envelopeRadiusAt(DEFAULT_ENVELOPE, height + 1)).toBe(0);
  });

  it("reaches the full half-width at the widest point and nowhere else", () => {
    const { height, crownBase, fullness } = DEFAULT_ENVELOPE;
    const base = height * crownBase;
    const widest = base + (height - base) * fullness;
    expect(envelopeRadiusAt(DEFAULT_ENVELOPE, widest)).toBeCloseTo(
      envelopeMaxRadius(DEFAULT_ENVELOPE),
      10,
    );
    for (const y of [widest - 2, widest + 2]) {
      expect(envelopeRadiusAt(DEFAULT_ENVELOPE, y)).toBeLessThan(
        envelopeMaxRadius(DEFAULT_ENVELOPE),
      );
    }
  });

  it("has no seam where the two profile halves meet", () => {
    const { height, crownBase, fullness } = DEFAULT_ENVELOPE;
    const base = height * crownBase;
    const widest = base + (height - base) * fullness;
    const below = envelopeRadiusAt(DEFAULT_ENVELOPE, widest - 1e-6);
    const above = envelopeRadiusAt(DEFAULT_ENVELOPE, widest + 1e-6);
    expect(Math.abs(above - below)).toBeLessThan(1e-4);
  });

  it("squares the crown off as the shoulder rises", () => {
    // The one dial that separates a pointed, upright Telperion from a
    // broad, domed Laurelin; both envelopes here are the same width.
    const pointed = { ...DEFAULT_ENVELOPE, shoulder: 1 };
    const domed = { ...DEFAULT_ENVELOPE, shoulder: 4 };
    const y = DEFAULT_ENVELOPE.height * 0.85;
    expect(envelopeRadiusAt(domed, y)).toBeGreaterThan(
      envelopeRadiusAt(pointed, y) * 1.5,
    );
  });

  it("has no width at all when there is no crown to fill", () => {
    const flat = { ...DEFAULT_ENVELOPE, crownBase: 1 };
    expect(envelopeRadiusAt(flat, DEFAULT_ENVELOPE.height * 0.9)).toBe(0);
    const flattened = { ...DEFAULT_ENVELOPE, spread: 0 };
    expect(envelopeRadiusAt(flattened, DEFAULT_ENVELOPE.height * 0.6)).toBe(0);
  });
});

describe("sampleEnvelope", () => {
  it("scatters exactly the requested number of points", () => {
    const points = sampleEnvelope(DEFAULT_ENVELOPE, 500, createRng(1));
    expect(points).toHaveLength(500);
  });

  it("puts every point inside the envelope", () => {
    for (const envelope of [DEFAULT_ENVELOPE, narrow, broad]) {
      const points = sampleEnvelope(envelope, 800, createRng(3));
      for (const point of points) {
        expect(envelopeContains(envelope, point)).toBe(true);
        expect(point.y).toBeGreaterThan(envelope.height * envelope.crownBase);
        expect(point.y).toBeLessThan(envelope.height);
      }
    }
  });

  it("is deterministic in the seed", () => {
    const a = sampleEnvelope(DEFAULT_ENVELOPE, 200, createRng(42));
    const b = sampleEnvelope(DEFAULT_ENVELOPE, 200, createRng(42));
    expect(a.map((p) => p.toArray())).toEqual(b.map((p) => p.toArray()));
  });

  it("scatters differently for a different seed", () => {
    const a = sampleEnvelope(DEFAULT_ENVELOPE, 200, createRng(42));
    const b = sampleEnvelope(DEFAULT_ENVELOPE, 200, createRng(43));
    expect(a.map((p) => p.toArray())).not.toEqual(b.map((p) => p.toArray()));
  });

  it("fills a wider envelope more widely", () => {
    const spread = (envelope: Envelope): number =>
      sampleEnvelope(envelope, 800, createRng(5)).reduce(
        (widest, p) => Math.max(widest, Math.hypot(p.x, p.z)),
        0,
      );
    expect(spread(broad)).toBeGreaterThan(spread(narrow) * 3);
  });

  it("returns nothing rather than spinning on an envelope with no volume", () => {
    expect(sampleEnvelope({ ...DEFAULT_ENVELOPE, spread: 0 }, 100, createRng(1)))
      .toHaveLength(0);
    expect(sampleEnvelope({ ...DEFAULT_ENVELOPE, crownBase: 1 }, 100, createRng(1)))
      .toHaveLength(0);
    expect(sampleEnvelope(DEFAULT_ENVELOPE, 0, createRng(1))).toHaveLength(0);
    expect(sampleEnvelope(DEFAULT_ENVELOPE, -5, createRng(1))).toHaveLength(0);
  });
});
