import { describe, expect, it } from "vitest";

import { createRng } from "./rng";

/* The library's whole determinism claim rests on this file: if the
   stream is reproducible the tree is, and if it is not, nothing
   downstream can be. */

function draw(seed: number, count: number): number[] {
  const rng = createRng(seed);
  return Array.from({ length: count }, () => rng.next());
}

describe("createRng", () => {
  it("gives the same stream for the same seed", () => {
    expect(draw(12_345, 32)).toEqual(draw(12_345, 32));
  });

  it("gives a different stream for a neighbouring seed", () => {
    // Neighbouring, not distant, because the panel's reroll button
    // means the owner walks consecutive seeds.
    expect(draw(12_345, 32)).not.toEqual(draw(12_346, 32));
  });

  it("stays in [0, 1)", () => {
    for (const seed of [0, 1, 7, 0xff_ff_ff_ff]) {
      for (const value of draw(seed, 2000)) {
        expect(value).toBeGreaterThanOrEqual(0);
        expect(value).toBeLessThan(1);
      }
    }
  });

  it("treats zero as an ordinary seed", () => {
    // xorshift, which this replaced, has an all-zero fixed point.
    const values = draw(0, 16);
    expect(new Set(values).size).toBe(16);
  });

  it("spreads roughly evenly across the unit interval", () => {
    const buckets = new Array<number>(10).fill(0);
    const rng = createRng(9);
    for (let i = 0; i < 100_000; i += 1) {
      buckets[Math.floor(rng.next() * 10)] += 1;
    }
    for (const count of buckets) {
      expect(count).toBeGreaterThan(9000);
      expect(count).toBeLessThan(11_000);
    }
  });

  it("maps range onto [min, max)", () => {
    const rng = createRng(4);
    for (let i = 0; i < 1000; i += 1) {
      const value = rng.range(-3, 7);
      expect(value).toBeGreaterThanOrEqual(-3);
      expect(value).toBeLessThan(7);
    }
  });
});
