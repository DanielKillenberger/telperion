/** One leaf in twelve bytes, read exactly as the core packed it.
 *
 * The three words are the storage on both sides of the wire, so this is the
 * only place in the browser source that knows the bit layout: a second copy
 * would be a reader free to drift from the writer. The Rust encoder of record
 * is `telperion-core`'s `foliage::packed`, and the shader that draws these
 * words is `leaf.wgsl`; all three decode one leaf one way.
 *
 * Word 0 is the rotation as a smallest-three quaternion - the index of the
 * dropped largest component in the top two bits, the other three below it, ten
 * bits each, over plus or minus one over root two. Word 1 is x and y as
 * unsigned normals over the reference box, word 2 is z in its low half and the
 * uniform scale as a half float in its high half. A scale is never negative,
 * so the half float's sign bit marks a withered leaf instead. */

/** Words one stored leaf occupies. */
export const LEAF_WORDS = 3;
/** The largest a dropped quaternion component leaves the other three. */
const RANGE = Math.SQRT1_2;
const CODES = 1023;

export type Vector = [number, number, number];
/** The three words of one leaf, in storage order: rotation, xy, z and scale. */
export type LeafWords = readonly [number, number, number];
/** The box leaf positions are quantised against. It comes from the family's
 * parameters, not from the tree that grew, so one box decodes a species at
 * every age; the build metadata and a specimen read both carry it. */
export interface LeafReference { min: Vector; extent: Vector }

/** The words of one leaf in a packed buffer, by instance index. */
export function leafWords(leaves: Uint32Array, index: number): LeafWords {
  const at = index * LEAF_WORDS;
  if (at < 0 || at + LEAF_WORDS > leaves.length) throw Error(`Leaf ${index} is outside the packed buffer`);
  return [leaves[at], leaves[at + 1], leaves[at + 2]];
}

/** Where a leaf stands. The crown's bounds and any spatial pass read nothing
 * else, so neither pays for a rotation it never uses. */
export function leafPosition(words: LeafWords, reference: LeafReference): Vector {
  const unorm = (word: number, shift: number) => ((word >>> shift) & 0xffff) / 65535;
  return [
    reference.min[0] + reference.extent[0] * unorm(words[1], 0),
    reference.min[1] + reference.extent[1] * unorm(words[1], 16),
    reference.min[2] + reference.extent[2] * unorm(words[2], 0),
  ];
}

/** The uniform scale a leaf carries, out of the high half of word 2, whether
 * or not the leaf is withered. */
export function leafScale(words: LeafWords): number {
  return half((words[2] >>> 16) & 0x7fff);
}

/** The rotation word 0 carries, as the three columns of its matrix: side, the
 * leaf's own axis, face. */
export function leafRotation(words: LeafWords): [Vector, Vector, Vector] {
  const word = words[0];
  const code = (slot: number) => ((word >>> (slot * 10)) & CODES) / CODES * (2 * RANGE) - RANGE;
  const [a, b, c] = [code(0), code(1), code(2)];
  // The dropped component is never negative: a quaternion and its negation are
  // the same rotation, so the writer takes it positive and stores no sign.
  const dropped = Math.sqrt(Math.max(0, 1 - a * a - b * b - c * c));
  const largest = word >>> 30;
  const q = largest === 0 ? [dropped, a, b, c]
    : largest === 1 ? [a, dropped, b, c]
    : largest === 2 ? [a, b, dropped, c]
    : [a, b, c, dropped];
  const [w, x, y, z] = q;
  return [
    [1 - 2 * (y * y + z * z), 2 * (x * y + z * w), 2 * (x * z - y * w)],
    [2 * (x * y - z * w), 1 - 2 * (x * x + z * z), 2 * (y * z + x * w)],
    [2 * (x * z + y * w), 2 * (y * z - x * w), 1 - 2 * (x * x + y * y)],
  ];
}

/** The whole transform three words stand for, column-major, as the sixteen
 * floats the core used to write: the rotation's columns at the stored scale,
 * and the stored position in column three. */
export function leafTransform(words: LeafWords, reference: LeafReference): Float32Array {
  const columns = leafRotation(words), scale = leafScale(words), at = leafPosition(words, reference);
  const m = new Float32Array(16);
  for (let c = 0; c < 3; c++) for (let axis = 0; axis < 3; axis++) m[c * 4 + axis] = columns[c][axis] * scale;
  m[12] = at[0]; m[13] = at[1]; m[14] = at[2]; m[15] = 1;
  return m;
}

/** The half float sixteen bits stand for. Written out rather than taken from a
 * DataView so the decoder needs no scratch buffer and no recent runtime. */
function half(bits: number): number {
  const sign = bits & 0x8000 ? -1 : 1;
  const exponent = (bits >>> 10) & 0x1f;
  const mantissa = bits & 0x3ff;
  // A subnormal half is the mantissa over the smallest normal's step; the top
  // exponent is the format's infinities and its not-a-numbers.
  if (exponent === 0) return sign * mantissa * 2 ** -24;
  if (exponent === 0x1f) return mantissa ? NaN : sign * Infinity;
  return sign * (1 + mantissa / 1024) * 2 ** (exponent - 15);
}
