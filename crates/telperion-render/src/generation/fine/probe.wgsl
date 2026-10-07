// The contract's probe: each case's eight u64 arguments in, two u64
// results out, by the function `cfg.y` selects (`fine/tests.rs`).

@group(0) @binding(0) var<uniform> cfg: vec4<u32>;
@group(0) @binding(1) var<storage, read> cases: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> presences: array<vec2<u32>>;
@group(0) @binding(3) var<storage, read_write> results: array<vec2<u32>>;

fn flag(b: bool) -> vec2<u32> {
    return w(select(0u, 1u, b), 0u);
}

// D18's integer form over `len` presences from `at`: `relay_node_q` with
// the unit's first node at 0.
fn relay(at: u32, len: u32, share: vec2<u32>) -> u32 {
    var total = vec4<u32>(0u);
    for (var j = 0u; j < len; j++) {
        total = add128(total, presences[at + j]);
    }
    if total.x != 0u || total.y != 0u || total.z != 0u {
        // share total, against (before + p) 2^32.
        let goal = mul128(share, total.xy);
        var before = vec2<u32>(0u);
        for (var j = 0u; j < len; j++) {
            let p = presences[at + j];
            let reach = add64(before, p);
            // (before + p) << 32 as 128 bits: (0, reach.x, reach.y, 0).
            let r = vec4<u32>(0u, reach.x, reach.y, 0u);
            let ge = r.w > goal.w || (r.w == goal.w && (r.z > goal.z
                || (r.z == goal.z && (r.y > goal.y || (r.y == goal.y && r.x >= goal.x)))));
            if !is_zero(p) && ge {
                return j;
            }
            before = reach;
        }
        return len - 1u;
    }
    let f = mul64(share, w(len, 0u));
    let k = f.y + select(1u, 0u, f.x == 0u);
    return clamp(k, 1u, len) - 1u;
}

@compute @workgroup_size(64)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x + id.y * cfg.z;
    if i >= cfg.x {
        return;
    }
    var a: array<vec2<u32>, 8>;
    for (var k = 0u; k < 8u; k++) {
        a[k] = cases[i * 8u + k];
    }
    var r0 = vec2<u32>(0u);
    var r1 = vec2<u32>(0u);
    switch cfg.y {
        case 0u: {
            r0 = child(a[0], a[1]);
            r1 = draw(a[0]);
        }
        case 1u: {
            r0 = log2q(a[0]);
            r1 = exp2_neg(a[1]);
        }
        case 2u: {
            r0 = exp2q(a[0]);
            r1 = neg_log2(a[1]);
        }
        case 3u: {
            r0 = ln_ratio(a[0], a[1]);
            r1 = expq(a[2]);
        }
        case 4u: {
            r0 = mulq(a[0], a[1]);
            r1 = presence(a[2], a[3]);
        }
        case 5u: {
            r0 = survival_q(a[0], a[1]);
            r1 = scaled(a[0], a[1]);
        }
        case 6u: {
            r0 = abortion_q(a[0], a[1].x != 0u, a[2], a[3], a[4]);
            r1 = lived_q(a[5], a[6], a[7]);
        }
        case 7u: {
            r0 = moved_q(a[0], a[1], a[2], a[3], a[4], a[5], a[6], a[7]);
        }
        case 8u: {
            r0 = past_q(a[0], a[1]);
            r1 = above_q(a[0], a[1]);
        }
        case 9u: {
            r0 = wake_q(a[0], a[1], a[2]);
            r1 = flag(kept_q(a[3], a[4].x != 0u, a[5], a[6]));
        }
        case 10u: {
            r0 = w(relay(a[0].x, a[1].x, a[2]), 0u);
        }
        case 12u: {
            r0 = under_q(a[0], a[1]);
            r1 = over_q(a[0], a[1]);
        }
        case 13u: {
            r0 = times_q(a[0], a[1]);
            r1 = window_q(wood_at_q(a[2], a[3], a[4]), a[5], a[6], a[7]);
        }
        case 14u: {
            r0 = carry_q(a[0], a[1], a[2].x, a[3]);
            r1 = fade_q(a[4], a[5].x, a[6], a[7], a[2].y);
        }
        case 15u: {
            r0 = lead_q(a[0], a[1], a[2], vec4<u32>(a[4], a[5]), a[3].x);
        }
        default: {
            r0 = and_q(a[0], a[1]);
        }
    }
    results[i * 2u] = r0;
    results[i * 2u + 1u] = r1;
}
