// The contract's probe: each case's eight 64-bit arguments in, two
// 64-bit results out, by the functions `cfg.y` selects (`contract_tests.rs`).
// An f32 argument or result sits in its word's low half.

// The selectors, mirroring `selectors.rs`'s `ALL`, which is the
// source; `mirror_tests.rs` checks the copy.
const P_CHILD_DRAW: u32 = 0u;
const P_INTERNODE: u32 = 1u;
const P_ANGLES: u32 = 2u;
const P_CONVERT: u32 = 3u;
const P_THRESHOLD: u32 = 4u;
const P_LOGS: u32 = 5u;
const P_LOGIT: u32 = 6u;
const P_SURVIVAL: u32 = 7u;
const P_ABORTION: u32 = 8u;
const P_LIVED: u32 = 9u;
const P_MOVED: u32 = 10u;
const P_PAST: u32 = 11u;
const P_WAKE: u32 = 12u;
const P_LEAD: u32 = 13u;
const P_WINDOW: u32 = 14u;
const P_CARRY: u32 = 15u;
const P_RELAY_NODE: u32 = 16u;

@group(0) @binding(0) var<uniform> cfg: vec4<u32>;
@group(0) @binding(1) var<storage, read> cases: array<vec2<u32>>;
@group(0) @binding(2) var<storage, read> presences: array<f32>;
@group(0) @binding(3) var<storage, read_write> results: array<vec2<u32>>;

fn fl(x: f32) -> vec2<u32> {
    return vec2<u32>(bitcast<u32>(x), 0u);
}

fn arg(a: vec2<u32>) -> f32 {
    return bitcast<f32>(a.x);
}

// D18 over `len` presences from `at`, the unit's first node at 0
// (`decide::relay_node_f`).
fn relay(at: u32, len: u32, share: f32) -> u32 {
    var total = 0.0;
    for (var j = 0u; j < len; j++) {
        total += presences[at + j];
    }
    if total > 0.0 {
        let goal = share * total;
        var before = 0.0;
        for (var j = 0u; j < len; j++) {
            let p = presences[at + j];
            if p > 0.0 && before + p >= goal {
                return j;
            }
            before += p;
        }
        return len - 1u;
    }
    let k = u32(ceil(share * f32(len)));
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
        case P_CHILD_DRAW: {
            r0 = child(a[0], a[1]);
            r1 = draw(a[0]);
        }
        case P_INTERNODE: {
            r0 = internode_draw(a[0]);
        }
        case P_ANGLES: {
            r0 = insertion_draw(a[0]);
            r1 = tropism_draw(a[0]);
        }
        case P_CONVERT: {
            r0 = fl(to_f32(a[0]));
            r1 = floor64(arg(a[1]));
        }
        case P_THRESHOLD: {
            r0 = threshold(arg(a[0]));
            r1 = scaled53(arg(a[0]), false);
        }
        case P_LOGS: {
            r0 = fl(neg_log2(arg(a[0])));
            r1 = fl(prob(arg(a[1])));
        }
        case P_LOGIT: {
            r0 = fl(logit(arg(a[0])));
            r1 = fl(logit53(a[1]));
        }
        case P_SURVIVAL: {
            r0 = survival_fine(arg(a[0]), a[1]);
            r1 = fl(scaled(arg(a[0]), a[1]));
        }
        case P_ABORTION: {
            r0 = abortion_fine(arg(a[0]), a[1].x != 0u, arg(a[2]), a[3], a[4]);
            r1 = lived_ticks(arg(a[5]), a[6], a[7]);
        }
        case P_LIVED: {
            r0 = lived_fine(arg(a[5]), a[6], a[7]);
            r1 = fl(draw_log2(a[7]));
        }
        case P_MOVED: {
            r0 = fl(moved(arg(a[0]), a[1], arg(a[2]), arg(a[3]), a[4], arg(a[5]), arg(a[6]), arg(a[7])));
        }
        case P_PAST: {
            r0 = fl(past(a[0], arg(a[1])));
            r1 = fl(above(a[0], arg(a[1])));
        }
        case P_WAKE: {
            r0 = wake_ticks(a[0], arg(a[1]), a[2]);
        }
        case P_LEAD: {
            r0 = fl(lead(a[0], a[1], a[2]));
        }
        case P_WINDOW: {
            let wood = wood_at(arg(a[2]), arg(a[3]), a[4]);
            r0 = fl(window(wood, arg(a[5]), arg(a[6]), arg(a[7])));
            r1 = fl(presence(arg(a[0]), arg(a[1])));
        }
        case P_CARRY: {
            r0 = fl(carry(arg(a[0]), arg(a[1]), a[2].x, arg(a[3])));
            r1 = fl(fade(arg(a[4]), a[5].x, arg(a[6]), arg(a[7]), a[2].y));
        }
        case P_RELAY_NODE, default: {
            r0 = vec2<u32>(relay(a[0].x, a[1].x, arg(a[2])), 0u);
        }
    }
    results[i * 2u] = r0;
    results[i * 2u + 1u] = r1;
}
