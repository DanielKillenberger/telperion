// The renderer's packed curve point (`CurvePoint::pack`, `pack.rs`): the
// centre, radius and distance along in float32, then the frame's normal
// and binormal octahedral at 24 bits a component, four components in the
// last three words, low first. The sweep writes its points so (fn-293);
// the rings read their frames back as the renderer does
// (`curve_walk.wgsl`'s `point`).

const POINT_WORDS: u32 = 8u;
const OCT_BITS: u32 = 24u;
const OCT_MASK: u32 = 0xffffffu;

// `f64::signum` as the CPU's packing takes it: a zero is one, signed as
// its sign bit says (WGSL's `sign` makes it zero; fn-212 task 11).
fn signum(x: f32) -> f32 {
    return select(1.0, -1.0, (bitcast<u32>(x) & 0x80000000u) != 0u);
}

// A unit vector folded onto the octahedron and quantised to two snorm24
// (`pack.rs`, `octahedral`).
fn octahedral(v: vec3<f32>) -> vec2<u32> {
    let l1 = abs(v.x) + abs(v.y) + abs(v.z);
    var x = v.x / l1;
    var y = v.y / l1;
    if v.z < 0.0 {
        let ox = x;
        let oy = y;
        x = (1.0 - abs(oy)) * signum(ox);
        y = (1.0 - abs(ox)) * signum(oy);
    }
    let top = f32((1u << (OCT_BITS - 1u)) - 1u);
    let qx = u32(i32(round(clamp(x, -1.0, 1.0) * top))) & OCT_MASK;
    let qy = u32(i32(round(clamp(y, -1.0, 1.0) * top))) & OCT_MASK;
    return vec2<u32>(qx, qy);
}

// A frame's last three words of a packed point.
fn frame_words(normal: vec3<f32>, binormal: vec3<f32>) -> vec3<u32> {
    let n = octahedral(normal);
    let b = octahedral(binormal);
    return vec3<u32>(n.x | (n.y << 24u), (n.y >> 8u) | (b.x << 16u), (b.x >> 16u) | (b.y << 8u));
}

fn snorm24(w: u32) -> f32 {
    return f32(bitcast<i32>(w << 8u) >> 8u) / 8388607.0;
}

// Two snorm24 unfolded off the octahedron, as `curve_walk.wgsl` reads them.
fn unoctahedral(a: u32, b: u32) -> vec3<f32> {
    var x = snorm24(a);
    var y = snorm24(b);
    let z = 1.0 - abs(x) - abs(y);
    if z < 0.0 {
        let ox = x;
        x = (1.0 - abs(y)) * select(-1.0, 1.0, ox >= 0.0);
        y = (1.0 - abs(ox)) * select(-1.0, 1.0, y >= 0.0);
    }
    return normalize(vec3(x, y, z));
}

// A packed point's normal and binormal from its frame's three words.
fn frame_normal(w: vec3<u32>) -> vec3<f32> {
    return unoctahedral(w.x & OCT_MASK, (w.x >> 24u) | ((w.y & 0xFFFFu) << 8u));
}

fn frame_binormal(w: vec3<u32>) -> vec3<f32> {
    return unoctahedral((w.y >> 16u) | ((w.z & 0xFFu) << 16u), w.z >> 8u);
}
