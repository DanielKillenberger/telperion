/** The wood's normals as the core stores them, decoded.
 *
 * Each vertex's unit normal is folded onto the octahedron and kept as two
 * snorm16, read exactly as the core wrote it. The Rust encoder of record is
 * `telperion-core`'s `surface::encode`, which bounds the decoded normal
 * within 1e-4 radians of the one it encodes. */

/** The fold's sign, as Rust's `signum`: zero counts as positive. */
const sign = (v: number) => (v < 0 ? -1 : 1);

/** A chunk's oct-encoded normals as three floats a vertex. */
export function decodeNormals(normals: Int16Array): Float32Array {
  const out = new Float32Array(normals.length / 2 * 3);
  for (let i = 0, j = 0; i < normals.length; i += 2, j += 3) {
    let x = normals[i] / 32767, y = normals[i + 1] / 32767;
    const z = 1 - Math.abs(x) - Math.abs(y);
    if (z < 0) [x, y] = [(1 - Math.abs(y)) * sign(x), (1 - Math.abs(x)) * sign(y)];
    const length = Math.hypot(x, y, z);
    out[j] = x / length; out[j + 1] = y / length; out[j + 2] = z / length;
  }
  return out;
}
