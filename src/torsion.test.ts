import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { DEFAULT_ENVELOPE } from "./envelope";
import { defaultGrowth } from "./skeleton/grow";
import {
  createGrowthBias,
  DEFAULT_BIAS,
  MIN_STEPS_PER_BEND,
  NO_BIAS,
  type BiasParams,
} from "./torsion";

/* Two things are being held to account here. That the five terms are
   really five - each one reaches the result on its own, which is the
   whole reason they are not one "torsion" number - and that the field
   keeps the promise colonize.ts relies on: a unit vector that never
   reverses the step it was given, whatever the dials are set to. */

/* The field is sampled by a growth step, and it will not hand out a
   bend that step is too coarse to draw - so every probe below is taken
   at the step the generator actually uses for that envelope, the same
   way colonize.ts takes it. */
const STEP = defaultGrowth(DEFAULT_ENVELOPE).stepDistance;

const UP = new THREE.Vector3(0, 1, 0);
const DOWN = new THREE.Vector3(0, -1, 0);

function field(overrides: Partial<BiasParams> = {}, seed = 1) {
  return createGrowthBias(DEFAULT_ENVELOPE, seed, {
    ...DEFAULT_BIAS,
    ...overrides,
  });
}

/** Points spread through the tree, on the axis and off it, low and
 *  high - the field's behaviour differs at all of those. */
const PROBES: readonly THREE.Vector3[] = [
  new THREE.Vector3(0, 0, 0),
  new THREE.Vector3(0, 3, 0),
  new THREE.Vector3(0.4, 7, -0.2),
  new THREE.Vector3(3, 12, 2),
  new THREE.Vector3(-5, 18, 1),
  new THREE.Vector3(0.1, 23.5, -0.1),
];

const DIRECTIONS: readonly THREE.Vector3[] = [
  UP,
  DOWN,
  new THREE.Vector3(1, 0, 0),
  new THREE.Vector3(0.3, -0.9, 0.3).normalize(),
  new THREE.Vector3(-0.6, 0.5, 0.6).normalize(),
];

describe("createGrowthBias", () => {
  it("returns a unit direction", () => {
    const bias = field();
    for (const position of PROBES) {
      for (const direction of DIRECTIONS) {
        expect(bias(position, direction, STEP).length()).toBeCloseTo(1, 10);
      }
    }
  });

  it("leaves the direction alone when every term is off", () => {
    // NO_BIAS has to stay a reachable configuration: it is what the
    // downward-step baseline is measured against.
    const bias = createGrowthBias(DEFAULT_ENVELOPE, 1, NO_BIAS);
    for (const position of PROBES) {
      for (const direction of DIRECTIONS) {
        const out = bias(position, direction, STEP);
        expect(out.distanceTo(direction)).toBeLessThan(1e-12);
      }
    }
  });

  it("turns a downward step upward", () => {
    // The term the spec originally missed, and the whole reason 22% of
    // the fn-11.2 skeleton's steps went down through its own crown.
    const bias = field({ ...NO_BIAS, gravitropism: 1 });
    for (const position of PROBES) {
      const shallow = new THREE.Vector3(1, -0.35, 0).normalize();
      expect(bias(position, shallow, STEP).y).toBeGreaterThan(shallow.y);
    }
  });

  it("pulls hardest low and least at the tips", () => {
    const bias = field({ ...NO_BIAS, gravitropism: 1 });
    const flat = new THREE.Vector3(1, 0, 0);
    const low = bias(new THREE.Vector3(0, 1, 0), flat, STEP).y;
    const high = bias(new THREE.Vector3(0, 23, 0), flat, STEP).y;
    expect(low).toBeGreaterThan(high);
    // Never all the way to nothing: a tip with no upward pull is where
    // the diving twigs come from.
    expect(high).toBeGreaterThan(0.1);
  });

  it("never reverses the step it was given", () => {
    /* colonize.ts's trunk climb depends on this and would otherwise
       stall against maxNodes or turn back at the ground: whatever the
       dials say, a step asked to go up still goes up. Swept past the
       panel's own ceilings on every term at once. */
    for (const seed of [1, 2, 3, 4, 5]) {
      const bias = createGrowthBias(DEFAULT_ENVELOPE, seed, {
        gravitropism: 3,
        lean: 2,
        writheAmplitude: 3,
        writheWavelength: 0.02,
        spiralRate: 40,
      });
      for (const position of PROBES) {
        expect(bias(position, UP, STEP).y).toBeGreaterThan(0);
      }
    }
  });

  it("survives a degenerate envelope and degenerate dials", () => {
    const bias = createGrowthBias(
      { ...DEFAULT_ENVELOPE, height: 0, crownBase: 0, spread: 0 },
      1,
      {
        gravitropism: -5,
        lean: -5,
        writheAmplitude: -5,
        writheWavelength: 0,
        spiralRate: -5,
      },
    );
    const out = bias(new THREE.Vector3(0, 0, 0), UP, STEP);
    expect(Number.isFinite(out.x + out.y + out.z)).toBe(true);
    expect(out.length()).toBeCloseTo(1, 10);
  });

  it("is deterministic in the seed, and the seed changes the field", () => {
    const probe = new THREE.Vector3(0, 5, 0);
    expect(field({}, 7)(probe, UP, STEP).toArray()).toEqual(
      field({}, 7)(probe, UP, STEP).toArray(),
    );
    expect(field({}, 7)(probe, UP, STEP).toArray()).not.toEqual(
      field({}, 8)(probe, UP, STEP).toArray(),
    );
  });

  it("gives each of the five terms its own reach", () => {
    /* The point of five parameters rather than one. Each is moved on
       its own from a field with everything else off, and each has to
       change the answer - otherwise it is decoration on the panel. */
    const probe = new THREE.Vector3(0.5, 6, 0.25);
    // Not straight up: gravitropism only ever adds height, so a step
    // already pointing at the sky is the one direction it cannot move.
    const heading = new THREE.Vector3(0.8, -0.2, 0.4).normalize();
    const base = createGrowthBias(DEFAULT_ENVELOPE, 1, NO_BIAS)(probe, heading, STEP);
    const moved: Partial<BiasParams>[] = [
      { gravitropism: 0.8 },
      { lean: 0.3 },
      { writheAmplitude: 0.1, writheWavelength: 0.4 },
      { writheAmplitude: 0.1, writheWavelength: 0.1 },
      { writheAmplitude: 0.1, spiralRate: 3 },
    ];
    const results = moved.map((overrides) =>
      createGrowthBias(DEFAULT_ENVELOPE, 1, { ...NO_BIAS, ...overrides })(
        probe,
        heading,
        STEP,
      ),
    );
    for (const result of results) {
      expect(result.distanceTo(base)).toBeGreaterThan(1e-3);
    }
    // Amplitude and wavelength are not the same dial wearing two
    // labels: same stray budget, different bend length, different tree.
    expect(results[2].distanceTo(results[3])).toBeGreaterThan(1e-3);
  });

  it("will not hand out a bend the growth step cannot sample", () => {
    /* The panel's shortest bend length was 1.92 m against a 0.528 m
       step - 3.6 samples per period, which is a sawtooth however
       smooth the function behind it. The floor is the library's, not
       the panel's, so asking for anything below it is not a way to get
       an aliased field; it is the same field as asking for the floor. */
    const floor = (MIN_STEPS_PER_BEND * STEP) / DEFAULT_ENVELOPE.height;
    const aliased = field({ writheWavelength: 0.001 });
    const panelFloor = field({ writheWavelength: 0.08 });
    const clamped = field({ writheWavelength: floor });
    for (const position of PROBES) {
      expect(aliased(position, UP, STEP).distanceTo(clamped(position, UP, STEP)))
        .toBeLessThan(1e-12);
      expect(panelFloor(position, UP, STEP).distanceTo(clamped(position, UP, STEP)))
        .toBeLessThan(1e-12);
    }
    // And a bend the step can sample is left exactly as it was asked for.
    const longer = field({ writheWavelength: floor * 2 });
    expect(
      longer(PROBES[3], UP, STEP).distanceTo(clamped(PROBES[3], UP, STEP)),
    ).toBeGreaterThan(1e-6);
  });

  it("holds the spiral to the same sampling floor", () => {
    // One turn of spiral is a wave in height like the writhe is, and
    // the same growth step samples it.
    const turns = DEFAULT_ENVELOPE.height / (MIN_STEPS_PER_BEND * STEP);
    const asked = field({ writheAmplitude: 0.1, spiralRate: 40 });
    const clamped = field({ writheAmplitude: 0.1, spiralRate: turns });
    for (const position of PROBES) {
      expect(
        asked(position, UP, STEP).distanceTo(clamped(position, UP, STEP)),
      ).toBeLessThan(1e-12);
    }
    const slower = field({ writheAmplitude: 0.1, spiralRate: turns / 2 });
    expect(
      slower(PROBES[3], UP, STEP).distanceTo(clamped(PROBES[3], UP, STEP)),
    ).toBeGreaterThan(1e-6);
  });

  it("bends a tree the same way at every height on the dial", () => {
    /* Every term is a fraction of height or a count of turns over it,
       for the same reason grow.ts derives its distances that way - so
       the same point of the way up gets the same bend whatever size
       the tree is. The heights are the dial's own range end to end:
       the 4 m sapling at the bottom of it and the 400 m at the top,
       which is the range the library is claimed to be scale-free
       over. */
    const bendAt = (height: number, t: number): THREE.Vector3 => {
      const envelope = { ...DEFAULT_ENVELOPE, height };
      return createGrowthBias(envelope, 1, DEFAULT_BIAS)(
        new THREE.Vector3(0.05 * height, t * height, 0),
        UP,
        defaultGrowth(envelope).stepDistance,
      );
    };
    for (const t of [0.1, 0.35, 0.6, 0.9]) {
      const sapling = bendAt(4, t);
      for (const height of [24, 60, 150, 400]) {
        expect(
          bendAt(height, t).distanceTo(sapling),
          `${height} m at ${t} of the way up`,
        ).toBeLessThan(1e-9);
      }
    }
  });
});
