// The light's sweep, combine and read (`light.wgsl` holds the bindings),
// and the hooks the programs call between their f64 operations.

// The node the sweep's program is at, and its layer's place.
var<private> sw_size: vec3<u32>;
var<private> sw_k: u32;
var<private> sw_ij: u32;
var<private> sw_dir: u32;
var<private> sw_here: u32;
var<private> sw_above: u32;

var<workgroup> shape: vec4<u32>;

@compute @workgroup_size(256)
fn sweep_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid == 0u {
        shape = vec4<u32>(grid[3], grid[4], grid[5], grid[7] & C.lit);
    }
    let s = workgroupUniformLoad(&shape);
    if s.w == 0u || wid.x >= C.dirs {
        return;
    }
    load_constants();
    sw_dir = wid.x;
    sw_size = s.xyz;
    for (var r = 0u; r < 2u; r++) {
        let w = C.dir[2u * sw_dir + r];
        R[R_OX + 2u * r] = w.xy;
        R[R_OX + 2u * r + 1u] = w.zw;
    }
    let layer = s.x * s.y;
    for (var step = 0u; step < s.z; step++) {
        sw_k = s.z - 1u - step;
        let depths = C.dirs * C.capacity;
        sw_here = depths + (2u * sw_dir + (step & 1u)) * layer;
        sw_above = depths + (2u * sw_dir + ((step + 1u) & 1u)) * layer;
        for (var ij = lid; ij < layer; ij += SWEEP_WG) {
            sw_ij = ij;
            R[R_I] = f64_from_u32(ij % s.x);
            R[R_J] = f64_from_u32(ij / s.x);
            R[R_RHO] = f64_from_f32(field[sw_k * layer + ij]);
            for (var pc = 0u; pc < SWEEP_LEN; pc++) {
                exec(SWEEP[pc]);
            }
        }
        storageBarrier();
        workgroupBarrier();
    }
}

// `bilinear`'s four nodes of the layer above about (X, Y), their depths
// and densities; none past the box or below the top layer.
fn neighbours() {
    R[R_FX] = f64_floor(R[R_X]);
    R[R_FY] = f64_floor(R[R_Y]);
    let i0 = f64_to_i32(R[R_FX]);
    let j0 = f64_to_i32(R[R_FY]);
    let layer = sw_size.x * sw_size.y;
    let top = sw_k + 1u >= sw_size.z;
    for (var t = 0u; t < 4u; t++) {
        let i = i0 + i32(t >> 1u);
        let j = j0 + i32(t & 1u);
        let inside = !top && i >= 0 && j >= 0 && u32(i) < sw_size.x && u32(j) < sw_size.y;
        let at = u32(max(j, 0)) * sw_size.x + u32(max(i, 0));
        var d = vec2<u32>(0u);
        var rho = vec2<u32>(0u);
        if inside {
            d = scratch[sw_above + at];
            rho = f64_from_f32(field[(sw_k + 1u) * layer + at]);
        }
        R[R_D0 + t] = d;
        R[R_V0 + t] = rho;
    }
}

fn hook(code: u32) {
    switch code {
        case H_NEIGHBOURS: {
            neighbours();
        }
        case H_DEPTH: {
            scratch[sw_here + sw_ij] = R[R_DEP];
            R[R_XN] = f64_neg(R[R_DEP]);
        }
        case H_LIGHT: {
            let layer = sw_size.x * sw_size.y;
            scratch[(sw_dir * sw_size.z + sw_k) * layer + sw_ij] = R[R_L];
        }
        case H_FLOOR: {
            for (var a = 0u; a < 3u; a++) {
                R[R_N_0 + a] = f64_floor(R[R_S_0 + a]);
                corner_node[a] = f64_to_i32(R[R_N_0 + a]);
            }
        }
        default: {
            exp_hook(code);
        }
    }
}

@compute @workgroup_size(64)
fn combine_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    let n = grid[6];
    if !standing() || i >= n {
        return;
    }
    if C.lit == 0u {
        field[C.capacity + i] = 1.0;
        return;
    }
    var sum = vec2<u32>(0u);
    for (var d = 0u; d < C.dirs; d++) {
        sum = f64_add(sum, scratch[d * n + i]);
    }
    // `clamp(0.0, 1.0)`.
    if f64_lt(sum, vec2<u32>(0u)) {
        sum = vec2<u32>(0u);
    }
    if f64_lt(ONE64, sum) {
        sum = ONE64;
    }
    field[C.capacity + i] = f64_to_f32(sum);
}

@compute @workgroup_size(64)
fn read_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= C.tips || !standing() {
        return;
    }
    corners_of(points[C.leaves + k].xyz);
    var sum = 0.0;
    for (var c = 0u; c < 8u; c++) {
        let at = corner_index(c);
        var l = 1.0;
        if at != NONE {
            l = field[C.capacity + at];
        }
        sum = sum + f64_to_f32(R[R_WC0 + c]) * l;
    }
    if sum < 0.0 {
        sum = 0.0;
    }
    if sum > 1.0 {
        sum = 1.0;
    }
    reads[k] = sum;
}
