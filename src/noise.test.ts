import * as THREE from "three";
import { describe, expect, it } from "vitest";

import { createNoise } from "@/lib/grower/noise";

/* The spec's line is "structured noise only, never white noise", and
   that is a testable claim rather than a stylistic one: white noise has
   no correlation between neighbouring samples and this does. The other
   claim worth holding to account is that the curl field is a curl -
   divergence-free - because that is what makes it read as flow. */

const noise = createNoise(1);

function samples(count: number, spacing: number): number[] {
  const values: number[] = [];
  for (let i = 0; i < count; i += 1) {
    values.push(noise.at(i * spacing, 7.5, -3.25));
  }
  return values;
}

/** Mean absolute difference between consecutive samples. */
function roughness(values: number[]): number {
  let total = 0;
  for (let i = 1; i < values.length; i += 1) {
    total += Math.abs(values[i] - values[i - 1]);
  }
  return total / (values.length - 1);
}

describe("createNoise", () => {
  it("gives the same field for the same seed", () => {
    const a = createNoise(42);
    const b = createNoise(42);
    for (let i = 0; i < 32; i += 1) {
      expect(a.at(i * 0.37, i * 0.11, i * 0.73)).toBe(
        b.at(i * 0.37, i * 0.11, i * 0.73),
      );
    }
  });

  it("gives a different field for a different seed", () => {
    const a = createNoise(42);
    const b = createNoise(43);
    const differences = Array.from({ length: 32 }, (_, i) =>
      Math.abs(a.at(i * 0.37, 1.5, 2.5) - b.at(i * 0.37, 1.5, 2.5)),
    );
    expect(Math.max(...differences)).toBeGreaterThan(0.1);
  });

  it("stays inside the range it claims", () => {
    for (let i = 0; i < 2000; i += 1) {
      const value = noise.fbm(i * 0.137, i * 0.311, i * 0.079);
      expect(value).toBeGreaterThanOrEqual(-1);
      expect(value).toBeLessThanOrEqual(1);
    }
  });

  it("is structured, not white", () => {
    /* The whole difference. Sampled a hundredth of a feature apart the
       field barely moves; sampled a feature apart it moves freely.
       White noise has the same roughness at both, which is why it reads
       as mush when a branch follows it. */
    const near = roughness(samples(400, 0.01));
    const far = roughness(samples(400, 1));
    expect(near).toBeLessThan(far / 10);
  });

  it("has a continuous field across cell boundaries", () => {
    // The quintic fade makes it C2, so a step across an integer lattice
    // line must not show up as a crease.
    const before = noise.at(3 - 1e-6, 0.5, 0.5);
    const after = noise.at(3 + 1e-6, 0.5, 0.5);
    expect(Math.abs(after - before)).toBeLessThan(1e-4);
  });

  it("returns a divergence-free curl", () => {
    /* Curl of anything is divergence-free, and that is the property
       being bought here: no sources for branches to pile into and no
       sinks for them to drain toward. Measured by central differences
       on the field itself, against its own local magnitude - an
       absolute threshold would only be testing how big the field
       happens to be. */
    const wavelength = 4;
    const e = 1e-3;
    const at = (x: number, y: number, z: number): THREE.Vector3 =>
      noise.curl(new THREE.Vector3(x, y, z), wavelength);

    for (const [x, y, z] of [
      [0.5, 1.5, 2.5],
      [-6.25, 11.75, 3.125],
      [17.5, -2.5, 8.75],
    ]) {
      const dx = (at(x + e, y, z).x - at(x - e, y, z).x) / (2 * e);
      const dy = (at(x, y + e, z).y - at(x, y - e, z).y) / (2 * e);
      const dz = (at(x, y, z + e).z - at(x, y, z - e).z) / (2 * e);
      const scale = at(x, y, z).length() / wavelength;
      expect(Math.abs(dx + dy + dz)).toBeLessThan(scale * 0.05);
    }
  });

  it("scales its features with the wavelength it is given", () => {
    // Two points a metre apart are neighbours in a 40 m field and
    // strangers in a 0.5 m one.
    const a = new THREE.Vector3(0, 0, 0);
    const b = new THREE.Vector3(1, 0, 0);
    const coarse = noise
      .curl(a, 40)
      .clone()
      .normalize()
      .dot(noise.curl(b, 40).clone().normalize());
    const fine = noise
      .curl(a, 0.5)
      .clone()
      .normalize()
      .dot(noise.curl(b, 0.5).clone().normalize());
    expect(coarse).toBeGreaterThan(0.9);
    expect(coarse).toBeGreaterThan(fine);
  });

  it("writes into the vector it was handed", () => {
    const out = new THREE.Vector3();
    expect(noise.curl(new THREE.Vector3(1, 2, 3), 5, out)).toBe(out);
  });
});
