// The light lattice (`light/field.rs`): the box spanning the leaves and
// the buds, each leaf's weighted area as integer quanta of 2^-20 m² at
// its eight corners (a u32 a node, overflow flagged), the densities, the
// sweep down each sky direction (`light.rs::swept`: a node's depth is the
// layer above's, read bilinearly where its ray meets that layer, plus the
// trapezoid of the density between), the directions summed in order,
// and each bud's trilinear read.
//
// The sweep two ways: `sweep_wg`, one workgroup a direction looping the
// layers with a barrier between; `sweep_layer`, one dispatch a layer
// over every direction's nodes.
//
// P.a[0]: leaves, tips, directions, layer k. P.f[0].x: extinction.

@group(0) @binding(1) var<storage, read_write> leaves: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> tips: array<f32>;
@group(0) @binding(3) var<storage, read_write> box: array<atomic<i32>>;
@group(0) @binding(4) var<storage, read_write> quanta: array<atomic<u32>>;
@group(0) @binding(5) var<storage, read_write> density: array<f32>;
@group(0) @binding(6) var<storage, read_write> dirs: array<vec4<f32>>;
@group(0) @binding(7) var<storage, read_write> dlight: array<f32>;
@group(0) @binding(8) var<storage, read_write> depth: array<f32>;
@group(0) @binding(9) var<storage, read_write> lattice: array<f32>;
@group(0) @binding(10) var<storage, read_write> reads: array<f32>;
@group(0) @binding(11) var<storage, read_write> flags: array<atomic<u32>>;
@group(0) @binding(12) var<storage, read_write> lattice_in: array<f32>;

struct Grid {
    low: vec3<i32>,
    size: vec3<u32>,
}

fn grid() -> Grid {
    let low = vec3<i32>(atomicLoad(&box[0]), atomicLoad(&box[1]), atomicLoad(&box[2]));
    let high = vec3<i32>(atomicLoad(&box[3]), atomicLoad(&box[4]), atomicLoad(&box[5]));
    return Grid(low, vec3<u32>(high - low + vec3<i32>(1)));
}

fn split_at(x: f32) -> vec2<f32> {
    let s = x * 2.0;
    let n = floor(s);
    return vec2<f32>(n, s - n);
}

fn point(k: u32) -> vec3<f32> {
    let leaves_n = P.a[0].x;
    if k < leaves_n {
        return leaves[k].xyz;
    }
    let t = 3u * (k - leaves_n);
    return vec3<f32>(tips[t], tips[t + 1u], tips[t + 2u]);
}

@compute @workgroup_size(256)
fn span(@builtin(global_invocation_id) g: vec3<u32>) {
    if g.x >= P.a[0].x + P.a[0].y {
        return;
    }
    let p = point(g.x);
    for (var a = 0u; a < 3u; a++) {
        let n = i32(split_at(p[a]).x);
        atomicMin(&box[a], n);
        atomicMax(&box[a + 3u], n + 1);
    }
}

// Corner `c`'s node index (NONE outside) and weight about `at`.
fn corner(gr: Grid, at: vec3<f32>, c: u32) -> vec2<u32> {
    var w = 1.0;
    var idx = vec3<i32>(0);
    var inside = true;
    for (var a = 0u; a < 3u; a++) {
        let s = split_at(at[a]);
        let upper = ((c >> a) & 1u) == 1u;
        let n = i32(s.x) + select(0, 1, upper);
        w = w * select(1.0 - s.y, s.y, upper);
        let k = n - gr.low[a];
        if k < 0 || u32(k) >= gr.size[a] {
            inside = false;
        }
        idx[a] = k;
    }
    var at_index = NONE;
    if inside {
        at_index = (u32(idx.z) * gr.size.y + u32(idx.y)) * gr.size.x + u32(idx.x);
    }
    return vec2<u32>(at_index, bitcast<u32>(w));
}

// The exact quanta of area × weight at 2^-20 m², half away from zero;
// `hi` set past a u32.
fn quanta_of(area: f32, w: f32) -> vec2<u32> {
    let ba = bitcast<u32>(area);
    let bw = bitcast<u32>(w);
    if (ba & 0x7fffffffu) == 0u || (bw & 0x7fffffffu) == 0u {
        return vec2<u32>(0u);
    }
    var ma = ba & 0x7fffffu;
    var ea = -126;
    if ((ba >> 23u) & 0xffu) != 0u {
        ma |= 0x800000u;
        ea = i32((ba >> 23u) & 0xffu) - 127;
    }
    var mw = bw & 0x7fffffu;
    var ew = -126;
    if ((bw >> 23u) & 0xffu) != 0u {
        mw |= 0x800000u;
        ew = i32((bw >> 23u) & 0xffu) - 127;
    }
    let prod = mul32(ma, mw);
    let sh = ea + ew - 26;
    if sh >= 0 {
        if sh >= 16 {
            return vec2<u32>(0u, 1u);
        }
        return shl64(prod, u32(sh));
    }
    let k = u32(-sh);
    let q = shr64(prod, k);
    let half = shr64(prod, k - 1u).x & 1u;
    return add64(q, vec2<u32>(half, 0u));
}

@compute @workgroup_size(256)
fn deposit(@builtin(global_invocation_id) g: vec3<u32>) {
    if g.x >= P.a[0].x {
        return;
    }
    let gr = grid();
    let leaf = leaves[g.x];
    for (var c = 0u; c < 8u; c++) {
        let cw = corner(gr, leaf.xyz, c);
        let q = quanta_of(leaf.w, bitcast<f32>(cw.y));
        if q.y != 0u || cw.x == NONE {
            atomicOr(&flags[0], 1u);
            continue;
        }
        if q.x == 0u {
            continue;
        }
        let old = atomicAdd(&quanta[cw.x], q.x);
        if old + q.x < old {
            atomicOr(&flags[0], 1u);
        }
    }
}

@compute @workgroup_size(256)
fn densities(@builtin(global_invocation_id) g: vec3<u32>) {
    let gr = grid();
    let n = gr.size.x * gr.size.y * gr.size.z;
    if g.x >= n {
        return;
    }
    density[g.x] = f32(atomicLoad(&quanta[g.x])) * 7.62939453125e-6; // 2^-17
}

// The value at fractional node (x, y) of the layer at `base` of `src`
// (0: densities, 1: depths), bilinear; none beyond it.
fn bilinear(src: u32, base: u32, gr: Grid, x: f32, y: f32) -> f32 {
    let fx = floor(x);
    let fy = floor(y);
    let tx = x - fx;
    let ty = y - fy;
    var sum = 0.0;
    for (var dx = 0; dx < 2; dx++) {
        for (var dy = 0; dy < 2; dy++) {
            let i = i32(fx) + dx;
            let j = i32(fy) + dy;
            if i >= 0 && j >= 0 && u32(i) < gr.size.x && u32(j) < gr.size.y {
                let wx = select(1.0 - tx, tx, dx == 1);
                let wy = select(1.0 - ty, ty, dy == 1);
                let at = base + u32(j) * gr.size.x + u32(i);
                var v = 0.0;
                if src == 0u {
                    v = density[at];
                } else {
                    v = depth[at];
                }
                sum = sum + wx * wy * v;
            }
        }
    }
    return sum;
}

// One node of direction `d` in layer `k`: its depth (written at
// `depth_out`) and its share of the light. The layer above's depths
// start at `depth_above`.
fn sweep_node(gr: Grid, d: u32, k: u32, ij: u32, depth_above: u32, depth_out: u32) {
    let layer = gr.size.x * gr.size.y;
    let n = layer * gr.size.z;
    let dir = dirs[d];
    let i = ij % gr.size.x;
    let j = ij / gr.size.x;
    let x = f32(i) + dir.x;
    let y = f32(j) + dir.y;
    var d0 = 0.0;
    var rho0 = 0.0;
    if k + 1u < gr.size.z {
        d0 = bilinear(1u, depth_above, gr, x, y);
        rho0 = bilinear(0u, (k + 1u) * layer, gr, x, y);
    }
    let rho = density[k * layer + ij];
    let dep = d0 + P.f[0].x * dir.z * (rho + rho0) / 2.0;
    depth[depth_out + ij] = dep;
    dlight[d * n + k * layer + ij] = dir.w * expc(-dep);
}

@compute @workgroup_size(256)
fn sweep_wg(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    let gr = grid();
    let layer = gr.size.x * gr.size.y;
    let d = wid.x;
    let nz = gr.size.z;
    for (var step = 0u; step < nz; step++) {
        let k = nz - 1u - step;
        let here = (2u * d + (step & 1u)) * layer;
        let above = (2u * d + ((step + 1u) & 1u)) * layer;
        for (var ij = lid; ij < layer; ij += WG) {
            sweep_node(gr, d, k, ij, above, here);
        }
        storageBarrier();
        workgroupBarrier();
    }
}

@compute @workgroup_size(256)
fn sweep_layer(@builtin(global_invocation_id) g: vec3<u32>) {
    let gr = grid();
    let layer = gr.size.x * gr.size.y;
    let n = layer * gr.size.z;
    let k = P.a[0].w;
    if k >= gr.size.z || g.x >= P.a[0].z * layer {
        return;
    }
    let d = g.x / layer;
    let ij = g.x % layer;
    sweep_node(gr, d, k, ij, d * n + (k + 1u) * layer, d * n + k * layer);
}

@compute @workgroup_size(256)
fn combine(@builtin(global_invocation_id) g: vec3<u32>) {
    let gr = grid();
    let n = gr.size.x * gr.size.y * gr.size.z;
    if g.x >= n {
        return;
    }
    if P.f[0].x <= 0.0 {
        lattice[g.x] = 1.0;
        return;
    }
    var sum = 0.0;
    for (var d = 0u; d < P.a[0].z; d++) {
        sum = sum + dlight[d * n + g.x];
    }
    lattice[g.x] = clamp(sum, 0.0, 1.0);
}

@compute @workgroup_size(256)
fn read(@builtin(global_invocation_id) g: vec3<u32>) {
    if g.x >= P.a[0].y {
        return;
    }
    let gr = grid();
    let at = point(P.a[0].x + g.x);
    var sum = 0.0;
    for (var c = 0u; c < 8u; c++) {
        let cw = corner(gr, at, c);
        var l = 1.0;
        if cw.x != NONE {
            l = lattice_in[cw.x];
        }
        // P.a[1].x set: the product passed through its bits first, so
        // the driver cannot fuse it into the add (the FMA probe).
        var prod = bitcast<f32>(cw.y) * l;
        if P.a[1].x != 0u {
            prod = bitcast<f32>(bitcast<u32>(prod) ^ (P.a[1].y));
        }
        sum = sum + prod;
    }
    reads[g.x] = clamp(sum, 0.0, 1.0);
}
