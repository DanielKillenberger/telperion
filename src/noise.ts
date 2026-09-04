import * as THREE from "three";

import { createRng } from "@/lib/grower/rng";

/* ------------------------------------------------------------------ *
 * STRUCTURED NOISE
 *
 * The spec is explicit that the wander in a branch is structured noise
 * and never white noise: white reads as mush because neighbouring
 * points are uncorrelated, so a branch jitters instead of flowing. This
 * is gradient noise - neighbouring points agree, and the field has a
 * feature size you can name and art-direct.
 *
 * The field the generator actually consumes is `curl`, the curl of a
 * vector potential built from three decorrelated fBm fields. Curl is
 * divergence-free by construction, which is what makes it read as flow
 * rather than as drift: there are no sources for a branch to pile into
 * and no sinks for it to drain toward, so branches sweep past each
 * other the way limbs on a real tree do.
 *
 * Nothing here knows about trees. It answers "which way does the field
 * point here", and torsion.ts decides what that means.
 * ------------------------------------------------------------------ */

/** Permutation table size. 256 is the classic Perlin figure and the
 *  wrap period of the field; a tree is a few feature-lengths across, so
 *  the repeat is never in frame. */
const TABLE = 256;
const MASK = TABLE - 1;

/** The 12 edge-midpoint gradients of a cube. Twelve rather than a
 *  random direction per cell because they are equidistributed, which is
 *  what keeps the field free of the directional bias a small random set
 *  would bake in. */
const GRADIENTS: readonly (readonly [number, number, number])[] = [
  [1, 1, 0],
  [-1, 1, 0],
  [1, -1, 0],
  [-1, -1, 0],
  [1, 0, 1],
  [-1, 0, 1],
  [1, 0, -1],
  [-1, 0, -1],
  [0, 1, 1],
  [0, -1, 1],
  [0, 1, -1],
  [0, -1, -1],
];

/** Perlin's quintic ease. Its first and second derivatives vanish at
 *  the cell boundaries, so the field is C2 and the curl below - which
 *  is a derivative of it - is continuous rather than creased. */
function fade(t: number): number {
  return t * t * t * (t * (t * 6 - 15) + 10);
}

function lerp(a: number, b: number, t: number): number {
  return a + (b - a) * t;
}

/** How many octaves the vector potential is summed over. Two, not one:
 *  a single octave gives every bend the same size, and the references
 *  show a long sweep with smaller kinks riding on it. More than two
 *  costs evaluations for detail finer than one growth step. */
const OCTAVES = 2;

/** Offsets that decorrelate the three components of the vector
 *  potential. They have to be far enough apart that the same field does
 *  not show up twice - a shared component would leave the curl with a
 *  preferred plane. */
const POTENTIAL_OFFSETS: readonly (readonly [number, number, number])[] = [
  [0, 0, 0],
  [31.416, 17.271, 5.772],
  [-11.331, 43.5, 27.183],
];

/** Central-difference step for the curl, in noise-space units. Small
 *  against the field's unit feature size and huge against double
 *  precision, so the derivative is the field's and not the arithmetic's. */
const CURL_EPS = 1e-3;

export interface Noise {
  /** Gradient noise at a point, in roughly [-1, 1]. Deterministic. */
  at(x: number, y: number, z: number): number;
  /** `OCTAVES` of `at`, each half the amplitude and twice the
   *  frequency, normalized back to roughly [-1, 1]. */
  fbm(x: number, y: number, z: number): number;
  /**
   * The curl of a vector potential built from `fbm`, at `point` metres,
   * with a feature size of `wavelength` metres. Divergence-free.
   *
   * The magnitude carries no meaning worth relying on - it is a
   * derivative, and its scale moves with `wavelength`. Callers want the
   * direction, and torsion.ts normalizes it.
   */
  curl(
    point: THREE.Vector3,
    wavelength: number,
    out?: THREE.Vector3,
  ): THREE.Vector3;
}

/** A field from `seed`. The same seed gives the same field on any
 *  machine: the permutation is shuffled by the library's one source of
 *  chance and nothing else is drawn anywhere. */
export function createNoise(seed: number): Noise {
  const rng = createRng(seed);

  // Fisher-Yates over 0..255, then doubled so the lookups below can
  // index up to 511 without a wrap test in the inner loop.
  const permutation = new Uint8Array(TABLE);
  for (let i = 0; i < TABLE; i += 1) permutation[i] = i;
  for (let i = TABLE - 1; i > 0; i -= 1) {
    const j = Math.floor(rng.next() * (i + 1));
    const swap = permutation[i];
    permutation[i] = permutation[j];
    permutation[j] = swap;
  }
  const table = new Uint8Array(TABLE * 2);
  table.set(permutation, 0);
  table.set(permutation, TABLE);

  const gradient = (hash: number, x: number, y: number, z: number): number => {
    const g = GRADIENTS[hash % GRADIENTS.length];
    return g[0] * x + g[1] * y + g[2] * z;
  };

  const at = (x: number, y: number, z: number): number => {
    const xi = Math.floor(x);
    const yi = Math.floor(y);
    const zi = Math.floor(z);
    const xf = x - xi;
    const yf = y - yi;
    const zf = z - zi;
    const cx = xi & MASK;
    const cy = yi & MASK;
    const cz = zi & MASK;

    const u = fade(xf);
    const v = fade(yf);
    const w = fade(zf);

    const a = table[cx] + cy;
    const aa = table[a] + cz;
    const ab = table[a + 1] + cz;
    const b = table[cx + 1] + cy;
    const ba = table[b] + cz;
    const bb = table[b + 1] + cz;

    return lerp(
      lerp(
        lerp(
          gradient(table[aa], xf, yf, zf),
          gradient(table[ba], xf - 1, yf, zf),
          u,
        ),
        lerp(
          gradient(table[ab], xf, yf - 1, zf),
          gradient(table[bb], xf - 1, yf - 1, zf),
          u,
        ),
        v,
      ),
      lerp(
        lerp(
          gradient(table[aa + 1], xf, yf, zf - 1),
          gradient(table[ba + 1], xf - 1, yf, zf - 1),
          u,
        ),
        lerp(
          gradient(table[ab + 1], xf, yf - 1, zf - 1),
          gradient(table[bb + 1], xf - 1, yf - 1, zf - 1),
          u,
        ),
        v,
      ),
      w,
    );
  };

  const fbm = (x: number, y: number, z: number): number => {
    let sum = 0;
    let amplitude = 1;
    let frequency = 1;
    let total = 0;
    for (let octave = 0; octave < OCTAVES; octave += 1) {
      sum += amplitude * at(x * frequency, y * frequency, z * frequency);
      total += amplitude;
      amplitude *= 0.5;
      frequency *= 2;
    }
    return sum / total;
  };

  const potential = (
    component: number,
    x: number,
    y: number,
    z: number,
  ): number => {
    const offset = POTENTIAL_OFFSETS[component];
    return fbm(x + offset[0], y + offset[1], z + offset[2]);
  };

  const curl = (
    point: THREE.Vector3,
    wavelength: number,
    out = new THREE.Vector3(),
  ): THREE.Vector3 => {
    const scale = wavelength > 0 ? wavelength : 1;
    const x = point.x / scale;
    const y = point.y / scale;
    const z = point.z / scale;
    const e = CURL_EPS;
    const inv = 1 / (2 * e);

    // curl(P) = (dPz/dy - dPy/dz, dPx/dz - dPz/dx, dPy/dx - dPx/dy)
    const dPzdy = (potential(2, x, y + e, z) - potential(2, x, y - e, z)) * inv;
    const dPydz = (potential(1, x, y, z + e) - potential(1, x, y, z - e)) * inv;
    const dPxdz = (potential(0, x, y, z + e) - potential(0, x, y, z - e)) * inv;
    const dPzdx = (potential(2, x + e, y, z) - potential(2, x - e, y, z)) * inv;
    const dPydx = (potential(1, x + e, y, z) - potential(1, x - e, y, z)) * inv;
    const dPxdy = (potential(0, x, y + e, z) - potential(0, x, y - e, z)) * inv;

    return out.set(dPzdy - dPydz, dPxdz - dPzdx, dPydx - dPxdy);
  };

  return { at, fbm, curl };
}
