// The field of a tree grown on the device (fn-257 R1'): the field's
// primitives made from the drawn nodes (`sweep.wgsl`'s `NW` words) and
// the station segments (`stations.wgsl`'s `GW` words), kept on the
// device in a grid of buckets, and any batch of closed cubes answered
// there, only the answers read back. The CPU's field
// (`pipeline/field.rs`) is the reference: the same wood sweeps, the same
// leaf-bearing segments with their leaves and limb systems
// (`clumping::systems`), the same tests a cube meets them by.
// Shared by the build (`field_build.wgsl`) and the query
// (`field_query.wgsl`), each with its own bindings.

struct Cfg {
    // What this dispatch covers (nodes, segments, primitives, buckets or
    // cells) and the invocations a dispatch row holds.
    n: u32,
    row: u32,
    // The drawn nodes, the root's id and the deepest lateral order that
    // opens a limb system; the station segments; which half of the
    // doubled jump words a jump reads.
    nodes: u32,
    root: u32,
    most: u32,
    segs: u32,
    flip: u32,
    buckets: u32,
    // The bucket grid's sides, and the primitives (wood first).
    dims: vec4<u32>,
    // The grid's corner and its bucket's side, in metres.
    lo: vec4<f32>,
    // A leaf's reach off its wood (`Plan::reach`): the seat, the
    // rachis and the blade; and a frond's length and reach over its share.
    reach: vec4<f32>,
    // The radius structural wood is at least (`space.rs`'s kinds, the
    // expansion's `bearing_bounds`); then a leaf's draws as placement
    // takes them (`data::config`): its size's variation, its scatter in
    // radians and the leaves' seed (its bits).
    bound: vec4<f32>,
    // A frond leaflet's box in its own frame at its size (`fronds.rs`'s
    // `Shape`): its centre and the leaf's size, its half extents.
    leaf_centre: vec4<f32>,
    leaf_half: vec4<f32>,
}

const NONE: u32 = 0xffffffffu;

const NW: u32 = 39u;

const GW: u32 = 40u;

// A retained leaf base (`boots.wgsl`): wood, but no structural node.
const BOOT: u32 = 0x40000000u;

// A record the dress drew a node in (`dress.wgsl`): the rest are none.
const MADE: u32 = 2u;

// The work words, `cfg.nodes` each: the widest start radius of a node's
// carrying children and the least id of them so wide, whether the node
// opens a system, and the jumps' pointers and values, two halves each.
const WIDE: u32 = 0u;

const CARRIER: u32 = 1u;

const OPENS: u32 = 2u;

const PTR: u32 = 3u;

const VAL: u32 = 5u;

const WORDS: u32 = 7u;

// After them, the primitives' box: three least and three most, ordered.
// Words of an answer's flags: wood, foliage, and a cell past its tally.
const WOOD: u32 = 1u;

const FOLIAGE: u32 = 2u;

const OVER: u32 = 4u;

const TALLY: u32 = 128u;

fn index_of(id: vec3<u32>) -> u32 {
    return id.x + id.y * cfg.row;
}

fn bucket(p: vec3<f32>) -> vec3<u32> {
    let c = floor((p - cfg.lo.xyz) / cfg.lo.w);
    let most = vec3<f32>(cfg.dims.xyz) - 1.0;
    return vec3<u32>(clamp(c, vec3<f32>(0.0), most));
}

// A primitive's box of buckets.
fn span(k: u32) -> array<vec3<u32>, 2> {
    let a = prims[2u * k];
    let b = prims[2u * k + 1u];
    var r = vec2<f32>(a.w, b.w);
    if k >= cfg.nodes {
        r = vec2<f32>(a.w);
    }
    let lo = min(a.xyz - r.x, b.xyz - r.y);
    let hi = max(a.xyz + r.x, b.xyz + r.y);
    return array<vec3<u32>, 2>(bucket(lo), bucket(hi));
}

fn flat(c: vec3<u32>) -> u32 {
    return c.x + cfg.dims.x * (c.y + cfg.dims.y * c.z);
}

// Whether the sphere of `inflation` about `p` meets the tapered sweep
// (`Segment::contains`).
fn contains(a: vec3<f32>, b: vec3<f32>, start: f32, end: f32, p: vec3<f32>, inflation: f32) -> bool {
    let d = b - a;
    let q = p - a;
    let r = start + inflation;
    let dr = end - start;
    let aa = dot(d, d) - dr * dr;
    let bb = dot(q, d) + r * dr;
    var t = 0.0;
    if aa > 0.0 {
        t = clamp(bb / aa, 0.0, 1.0);
    } else if bb > aa * 0.5 {
        t = 1.0;
    }
    let distance = q - d * t;
    let rr = r + dr * t;
    return dot(distance, distance) <= rr * rr;
}

// The share of the segment's length inside the closed cube
// (`Segment::share_inside`); a point segment counts whole inside.
fn inside(a: vec3<f32>, b: vec3<f32>, c: vec3<f32>, half: f32) -> f32 {
    var t0 = 0.0;
    var t1 = 1.0;
    for (var k = 0u; k < 3u; k++) {
        let d = b[k] - a[k];
        if d == 0.0 {
            if abs(a[k] - c[k]) > half {
                return 0.0;
            }
            continue;
        }
        let ta = (c[k] - half - a[k]) / d;
        let tb = (c[k] + half - a[k]) / d;
        t0 = max(t0, min(ta, tb));
        t1 = min(t1, max(ta, tb));
    }
    return max(t1 - t0, 0.0);
}

fn octahedral(v: vec3<f32>) -> vec2<f32> {
    let n = v / (abs(v.x) + abs(v.y) + abs(v.z));
    if n.z >= 0.0 {
        return n.xy;
    }
    return (1.0 - abs(n.yx)) * select(vec2<f32>(-1.0), vec2<f32>(1.0), n.xy >= vec2<f32>(0.0));
}

fn unoctahedral(o: vec2<f32>) -> vec3<f32> {
    var v = vec3<f32>(o, 1.0 - abs(o.x) - abs(o.y));
    if v.z < 0.0 {
        let xy = (1.0 - abs(v.yx)) * select(vec2<f32>(-1.0), vec2<f32>(1.0), v.xy >= vec2<f32>(0.0));
        v = vec3<f32>(xy, v.z);
    }
    return normalize(v);
}

fn square_to(v: vec3<f32>) -> vec3<f32> {
    let a = abs(v);
    var reference = vec3<f32>(0.0, 0.0, 1.0);
    if a.x <= a.y && a.x <= a.z {
        reference = vec3<f32>(1.0, 0.0, 0.0);
    } else if a.y <= a.z {
        reference = vec3<f32>(0.0, 1.0, 0.0);
    }
    return normalize(cross(reference, v));
}

// Whether the closed cube of `half` about `p` meets leaflet box `k`
// (`Cuboid::meets`): the world's axes and the box's tried as separating
// axes.
fn meets_box(pa: vec4<f32>, pb: vec4<f32>, p: vec3<f32>, half: f32) -> bool {
    let w = bitcast<u32>(pb.w);
    let o = vec2<f32>(f32(w & 2047u), f32((w >> 11u) & 2047u)) / 2047.0 * 2.0 - 1.0;
    let across = unoctahedral(o);
    let share = f32(w >> 22u) / 1023.0;
    let thickness = pa.w * share;
    let breadth = pa.w - thickness;
    let a = pa.xyz;
    let b = pb.xyz;
    let d = b - a;
    var along = d - across * dot(d, across);
    if dot(along, along) > 0.0 {
        along = normalize(along);
    } else {
        along = square_to(across);
    }
    let normal = cross(along, across);
    let centre = (a + b) * 0.5;
    let axes = array<vec3<f32>, 3>(along, across, normal);
    let extent = vec3<f32>(dot(d, along) * 0.5, breadth, thickness);
    // Its bounds on the world's axes.
    for (var k = 0u; k < 3u; k++) {
        var u = vec3<f32>(0.0);
        u[k] = 1.0;
        let r = abs(dot(along, u)) * extent.x + abs(dot(across, u)) * extent.y + abs(dot(normal, u)) * extent.z;
        if p[k] + half < centre[k] - r || p[k] - half > centre[k] + r {
            return false;
        }
    }
    for (var k = 0u; k < 3u; k++) {
        let u = axes[k];
        let c = dot(p, u);
        let r = half * (abs(u.x) + abs(u.y) + abs(u.z));
        let at = dot(centre, u);
        if c + r < at - extent[k] || c - r > at + extent[k] {
            return false;
        }
    }
    return true;
}
