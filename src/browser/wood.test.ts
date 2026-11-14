import { describe, expect, it } from "vitest";
import { decodeNormals } from "./wood";

/* Unit normals and the codes the core writes for them
 * (`surface::encode::normal`, pinned on the Rust side by
 * `encode::tests::the_browser_decodes_the_codes_the_core_writes`): the six
 * axes, one fold onto the lower half, and four drawn at random. */
const GOLDEN: [[number, number, number], [number, number]][] = [
  [[0, 0, 1], [0, 0]],
  [[0, 0, -1], [32767, 32767]],
  [[1, 0, 0], [32767, 0]],
  [[0, -1, 0], [0, -32767]],
  [[-0.663556801, -0.646286771, 0.376836544], [-12891, -12555]],
  [[-0.712254759, -0.044520411, -0.700507738], [-31766, -16752]],
  [[0.626027246, 0.115370593, 0.771219498], [13561, 2499]],
  [[-0.71698689, 0.566264184, -0.406539879], [-21786, 18864]],
];

describe("decodeNormals", () => {
  it("reads each code within the core's 1e-4 radians of its normal", () => {
    const decoded = decodeNormals(new Int16Array(GOLDEN.flatMap(([, code]) => code)));
    GOLDEN.forEach(([v], k) => {
      const n = decoded.subarray(k * 3, k * 3 + 3);
      // The angle from the cross and dot products: acos near one would read
      // float32 rounding as a hundredth of a degree.
      const dot = n[0] * v[0] + n[1] * v[1] + n[2] * v[2];
      const cross = Math.hypot(n[1] * v[2] - n[2] * v[1], n[2] * v[0] - n[0] * v[2], n[0] * v[1] - n[1] * v[0]);
      expect(Math.atan2(cross, dot)).toBeLessThanOrEqual(1e-4);
    });
  });
});
