// The per-axis phases with no hierarchy: each axis's own light and
// upkeep for the balance, each surviving bud's lit presence for the
// allocation, and the normalisation (`allocation::sizes`): each PA's
// Σp and Σp (v/p)^ψ as 64-bit integer sums of 2^-32 units, then each
// bud's size against its PA's.
//
// P.a[0]: n, groups.

@group(0) @binding(1) var<storage, read_write> pa: array<u32>;
@group(0) @binding(2) var<storage, read_write> tab: array<f32>;
@group(0) @binding(3) var<storage, read_write> base_scale: array<f32>;
@group(0) @binding(4) var<storage, read_write> running: array<f32>;
@group(0) @binding(5) var<storage, read_write> light: array<f32>;
@group(0) @binding(6) var<storage, read_write> wood: array<f32>;
@group(0) @binding(7) var<storage, read_write> live: array<u32>;
@group(0) @binding(8) var<storage, read_write> own_a: array<f32>;
@group(0) @binding(9) var<storage, read_write> own_b: array<f32>;
@group(0) @binding(10) var<storage, read_write> shed: array<u32>;
@group(0) @binding(11) var<storage, read_write> kept: array<f32>;
@group(0) @binding(12) var<storage, read_write> lit: array<f32>;
@group(0) @binding(13) var<storage, read_write> pres: array<f32>;
@group(0) @binding(14) var<storage, read_write> vigour: array<f32>;
@group(0) @binding(15) var<storage, read_write> gacc: array<atomic<u32>>;
@group(0) @binding(16) var<storage, read_write> sizes: array<f32>;
@group(0) @binding(17) var<storage, read_write> sums: array<f32>;

@compute @workgroup_size(256)
fn own(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    var l = 0.0;
    if live[i] != 0u {
        l = base_scale[i] * running[i] * light[i];
    }
    own_a[i] = l;
    own_b[i] = tab[8u * pa[i]] * wood[i];
}

@compute @workgroup_size(256)
fn lit_of(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    var p = 0.0;
    if live[i] != 0u && shed[i] == 0u {
        p = base_scale[i] * running[i] * kept[i];
    }
    pres[i] = p;
    lit[i] = p * light[i];
}

fn add_at(at: u32, v: vec2<u32>) {
    if v.x != 0u {
        let old = atomicAdd(&gacc[at], v.x);
        if old + v.x < old {
            atomicAdd(&gacc[at + 1u], v.y + 1u);
            return;
        }
    }
    if v.y != 0u {
        atomicAdd(&gacc[at + 1u], v.y);
    }
}

// `allocation::drawn`: (v/p)^ψ.
fn drawn(v: f32, p: f32, psi: f32) -> f32 {
    if p <= 0.0 || psi == 0.0 {
        return 1.0;
    }
    if v <= 0.0 {
        return select(INF, 0.0, psi > 0.0);
    }
    return exp2c(psi * (log2c(v) - log2c(p)));
}

@compute @workgroup_size(256)
fn group_sums(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    let p = pres[i];
    if p <= 0.0 {
        return;
    }
    let k = pa[i];
    add_at(4u * k, fixed(p));
    add_at(4u * k + 2u, fixed(drawn(vigour[i], p, tab[8u * k + 4u]) * p));
}

@compute @workgroup_size(256)
fn size_of(@builtin(global_invocation_id) g: vec3<u32>) {
    let i = g.x;
    if i >= P.a[0].x {
        return;
    }
    if i < P.a[0].y {
        sums[2u * i] = unfixed(atomicLoad(&gacc[4u * i]), atomicLoad(&gacc[4u * i + 1u]));
        sums[2u * i + 1u] = unfixed(atomicLoad(&gacc[4u * i + 2u]), atomicLoad(&gacc[4u * i + 3u]));
    }
    let k = pa[i];
    let total = unfixed(atomicLoad(&gacc[4u * k]), atomicLoad(&gacc[4u * k + 1u]));
    let weighted = unfixed(atomicLoad(&gacc[4u * k + 2u]), atomicLoad(&gacc[4u * k + 3u]));
    let p = pres[i];
    if p <= 0.0 || weighted <= 0.0 {
        sizes[i] = 1.0;
        return;
    }
    sizes[i] = drawn(vigour[i], p, tab[8u * k + 4u]) * total / weighted;
}
