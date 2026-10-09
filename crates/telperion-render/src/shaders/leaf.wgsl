// One leaf in twelve bytes, read exactly as the core packed it. Every shader
// that binds the placement buffer prepends this file, so the three that do -
// the crown, the selection pass and the sun's own pass - decode one leaf one
// way and no reader can drift from the writer.
//
// Word 0 is the rotation as a smallest-three quaternion: the index of the
// dropped largest component in the top two bits, the other three below it, ten
// bits each, over plus or minus one over root two. Word 1 is x and y over the
// reference box. Word 2 is z in its low half and the scale as a half float in
// its high half, whose sign bit marks a withered leaf.

const LEAF_WORDS: u32 = 3u;
/// The top bit of word 2, the scale's sign: set on a withered leaf.
const LEAF_WITHERED: u32 = 0x80000000u;
/// The largest a dropped component leaves the other three.
const LEAF_RANGE: f32 = 0.7071067811865476;

/// The rotation word 0 carries, as the three columns of its matrix: side, the
/// leaf's own axis, face.
fn leaf_rotation(word: u32) -> mat3x3<f32> {
    let codes = vec3<f32>(
        f32(word & 1023u),
        f32((word >> 10u) & 1023u),
        f32((word >> 20u) & 1023u),
    );
    let v = codes / 1023.0 * (2.0 * LEAF_RANGE) - vec3<f32>(LEAF_RANGE);
    var q = vec4<f32>(v, sqrt(max(0.0, 1.0 - dot(v, v))));
    let largest = word >> 30u;
    if (largest == 0u) {
        q = q.wxyz;
    } else if (largest == 1u) {
        q = q.xwyz;
    } else if (largest == 2u) {
        q = q.xywz;
    }
    let w = q.x;
    let x = q.y;
    let y = q.z;
    let z = q.w;
    return mat3x3<f32>(
        vec3<f32>(1.0 - 2.0 * (y * y + z * z), 2.0 * (x * y + z * w), 2.0 * (x * z - y * w)),
        vec3<f32>(2.0 * (x * y - z * w), 1.0 - 2.0 * (x * x + z * z), 2.0 * (y * z + x * w)),
        vec3<f32>(2.0 * (x * z + y * w), 2.0 * (y * z - x * w), 1.0 - 2.0 * (x * x + y * y)),
    );
}

/// The uniform scale a leaf carries, out of the high half of word 2. A scale
/// is never negative, so the half float's sign bit is free, and it carries
/// whether the leaf is withered instead.
fn leaf_scale(words: vec3<u32>) -> f32 {
    return abs(unpack2x16float(words.z).y);
}

/// Whether a leaf is one of the dead a rosette keeps: word 2's top bit.
fn leaf_withered(words: vec3<u32>) -> bool {
    return (words.z & LEAF_WITHERED) != 0u;
}

/// Where a leaf stands: its three unsigned normals over the reference box.
fn leaf_position(words: vec3<u32>, box_min: vec3<f32>, box_extent: vec3<f32>) -> vec3<f32> {
    let xy = unpack2x16unorm(words.y);
    let z = unpack2x16unorm(words.z).x;
    return box_min + box_extent * vec3<f32>(xy.x, xy.y, z);
}

/// The size on screen, in pixels across its bounding sphere, below which a
/// leaf or a flower is thinned (fn-246): a pixel, so a leaf is thinned only
/// where its whole sphere fits inside one.
const THIN_PIXELS: f32 = 1.0;

/// The share of the organs `across` pixels wide that are drawn: all of them
/// at a pixel and above, and below it the share whose kept organs, enlarged
/// by `1 / sqrt(share)`, cover the area all of them would. Continuous in
/// size, so nothing pops as the camera moves.
fn leaf_kept(across: f32) -> f32 {
    let r = clamp(across / THIN_PIXELS, 0.0, 1.0);
    return r * r;
}

/// A number in [0, 1) that is the organ's own: a hash of its stored words,
/// so the same organs stay drawn from frame to frame and a smaller share is
/// always a subset of a larger one.
fn leaf_draw_rank(words: vec3<u32>) -> f32 {
    var h = words.x * 747796405u + 2891336453u;
    h = (h ^ words.y) * 747796405u + 2891336453u;
    h = (h ^ words.z) * 277803737u;
    h = h ^ (h >> 16u);
    h = h * 2246822519u;
    h = h ^ (h >> 13u);
    return f32(h >> 8u) / 16777216.0;
}

/// The whole transform three words stand for, column-major, as the sixteen
/// floats the core used to write were.
fn leaf_transform(words: vec3<u32>, box_min: vec3<f32>, box_extent: vec3<f32>) -> mat4x4<f32> {
    let rotation = leaf_rotation(words.x);
    let scale = leaf_scale(words);
    let at = leaf_position(words, box_min, box_extent);
    return mat4x4<f32>(
        vec4<f32>(rotation[0] * scale, 0.0),
        vec4<f32>(rotation[1] * scale, 0.0),
        vec4<f32>(rotation[2] * scale, 0.0),
        vec4<f32>(at, 1.0),
    );
}
