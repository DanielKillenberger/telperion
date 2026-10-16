// The field's query (`field.wgsl`): any batch of closed cubes answered
// from the grid of buckets.

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> prims: array<vec4<u32>>;
// The leaves' limb systems, the first leaf primitive's first.
@group(0) @binding(2) var<storage, read> owner: array<u32>;
@group(0) @binding(3) var<storage, read> index: array<u32>;
@group(0) @binding(4) var<storage, read> cells: array<vec4<f32>>;
@group(0) @binding(5) var<storage, read_write> answers: array<vec4<u32>>;

// Each cell answered (`Field::query`): whether wood reaches it and the
// thickest that does, the leaves the segments that meet it carry inside
// it, and the limb system with the most of them, the lower on a tie.
// Each primitive is met once, in the first bucket of its box and the
// cell's both hold; every reduction is a maximum, a flag or a sum of
// words, so the answer does not depend on the order a bucket lists its
// primitives in. A cell met by more systems than its tally holds is
// flagged, and asked again a window of systems at a time (`cfg.flip` the
// window's first, `NONE` for none): each window's answer is its best
// system and that system's leaves.
@compute @workgroup_size(64)
fn query(@builtin(global_invocation_id) id: vec3<u32>) {
    let c = index_of(id);
    if c >= cfg.n {
        return;
    }
    let cell = cells[c];
    let center = cell.xyz;
    let half = cell.w;
    let inflation = half * sqrt(3.0);
    let window = cfg.flip;
    let lo = bucket(center - inflation);
    let hi = bucket(center + inflation);
    var wood = false;
    var radius = 0.0;
    var foliage = false;
    var total = vec2<u32>(0u);
    var systems: array<u32, TALLY>;
    var shares: array<vec2<u32>, TALLY>;
    var held = 0u;
    var over = false;
    let list = 2u * cfg.buckets + 1u;
    for (var z = lo.z; z <= hi.z; z++) {
        for (var y = lo.y; y <= hi.y; y++) {
            for (var x = lo.x; x <= hi.x; x++) {
                let here = vec3<u32>(x, y, z);
                let b = flat(here);
                let first = list + index[b];
                let n = index[cfg.buckets + 1u + b];
                for (var i = 0u; i < n; i++) {
                    let k = index[first + i];
                    let e = endpoints(k);
                    let a = e[0];
                    let b = e[1];
                    // Its box, which the cell's reach must meet before
                    // anything finer is asked, and which names the one
                    // bucket the primitive is met in.
                    let r = reach(k);
                    let low = min(a - r.x, b - r.y);
                    let high = max(a + r.x, b + r.y);
                    if any(low > center + inflation) || any(high < center - inflation) {
                        continue;
                    }
                    if any(max(bucket(low), lo) != here) {
                        continue;
                    }
                    if k < cfg.nodes {
                        if contains(a, b, r.x, r.y, center, inflation) {
                            wood = true;
                            radius = max(radius, max(r.x, r.y));
                        }
                        continue;
                    }
                    // A frond's leaflet is a box carrying one leaf; any
                    // other leaf-bearing segment a sweep carrying its own.
                    let leaflet = k >= cfg.nodes + cfg.segs;
                    let high_bits = attributes(k) >> 15u;
                    var count = f32(high_bits);
                    if leaflet {
                        count = 1.0;
                        if !meets_box(a, b, r.x, high_bits, center, half) {
                            continue;
                        }
                    } else if !contains(a, b, r.x, r.x, center, inflation) {
                        continue;
                    }
                    foliage = true;
                    let leaves = fixed(count * inside(a, b, center, half));
                    total = add(total, leaves);
                    let system = owner[k - cfg.nodes];
                    if window != NONE && (system < window || system - window >= TALLY) {
                        continue;
                    }
                    var j = 0u;
                    // cap: the systems held, at most `TALLY`.
                    while j < held && systems[j] != system {
                        j += 1u;
                    }
                    if j == held {
                        if held == TALLY {
                            over = true;
                            continue;
                        }
                        systems[j] = system;
                        shares[j] = vec2<u32>(0u);
                        held += 1u;
                    }
                    shares[j] = add(shares[j], leaves);
                }
            }
        }
    }
    var best = NONE;
    var most = vec2<u32>(0u);
    for (var j = 0u; j < held; j++) {
        let more = shares[j].y > most.y || (shares[j].y == most.y && shares[j].x > most.x);
        if best == NONE || more || (all(shares[j] == most) && systems[j] < best) {
            best = systems[j];
            most = shares[j];
        }
    }
    let flags = select(0u, WOOD, wood) | select(0u, FOLIAGE, foliage) | select(0u, OVER, over);
    if window != NONE {
        total = most;
    }
    answers[c] = vec4<u32>(flags, bitcast<u32>(radius), bitcast<u32>(leaves_of(total)), best);
}

// A share of leaves in fixed point, a 4096th of a leaf: summed as words,
// a cell's leaves are the same whatever order its buckets list them in.
const UNIT: f32 = 4096.0;

fn fixed(leaves: f32) -> u32 {
    return u32(round(leaves * UNIT));
}

// Two words' sum (low, high) and a word.
fn add(a: vec2<u32>, b: u32) -> vec2<u32> {
    let low = a.x + b;
    return vec2<u32>(low, a.y + u32(low < b));
}

fn leaves_of(a: vec2<u32>) -> f32 {
    return (f32(a.y) * 4294967296.0 + f32(a.x)) / UNIT;
}
