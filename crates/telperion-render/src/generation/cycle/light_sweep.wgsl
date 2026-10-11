// The light's sweep, combine and read (`light.wgsl` holds the bindings),
// and the hooks the programs call between their f64 operations.
//
// `swept`'s node splits by what it reads: its bilinear weights are the
// direction's at its column, the same at every layer (`weights_main`);
// its path term reads densities only (`path_main`); only `d0`, the layer
// above's depths, makes the layers sequential (`sweep_main`); and its
// light, `exp` of its depth, is summed over the directions in their order
// (`combine_main`). Every value is the CPU's, by the CPU's operations in
// its order; only which dispatch takes them moves.
//
// `scratch` holds each direction's path term at each node, overwritten
// by its depth, from 0; then from `C.dirs C.capacity` each direction's
// column records: four weights and the column's lower node.

// The direction, layer and column a program is at.
var<private> at_dir: u32;
var<private> at_k: u32;
var<private> at_ij: u32;
var<private> at_size: vec3<u32>;
var<private> col_low: vec2<i32>;

var<workgroup> shape: vec4<u32>;

fn layer_of() -> u32 {
    return at_size.x * at_size.y;
}

fn nodes_of() -> u32 {
    return layer_of() * at_size.z;
}

fn column_record() -> u32 {
    return C.dirs * C.capacity + 5u * (at_dir * layer_of() + at_ij);
}

// A direction's reals into OX, OY, EP and SHARE.
fn load_direction() {
    for (var r = 0u; r < 2u; r++) {
        let w = C.dir[2u * at_dir + r];
        R[R_OX + 2u * r] = w.xy;
        R[R_OX + 2u * r + 1u] = w.zw;
    }
}

fn place(i: u32, size: vec3<u32>) {
    at_size = size;
    let layer = layer_of();
    at_dir = i / (layer * size.z);
    at_k = (i / layer) % size.z;
    at_ij = i % layer;
}

fn grid_size() -> vec3<u32> {
    return vec3<u32>(grid[3], grid[4], grid[5]);
}

@compute @workgroup_size(64)
fn weights_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    let size = grid_size();
    if !standing() || C.lit == 0u || i >= C.dirs * size.x * size.y {
        return;
    }
    place(i, vec3<u32>(size.xy, 1u));
    at_size = size;
    load_constants();
    load_direction();
    R[R_I] = f64_from_u32(at_ij % size.x);
    R[R_J] = f64_from_u32(at_ij / size.x);
    for (var pc = 0u; pc < WEIGHTS_LEN; pc++) {
        exec(WEIGHTS[pc]);
    }
}

@compute @workgroup_size(64)
fn path_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    let size = grid_size();
    if !standing() || C.lit == 0u || i >= C.dirs * size.x * size.y * size.z {
        return;
    }
    place(i, size);
    load_constants();
    load_direction();
    for (var pc = 0u; pc < PATH_LEN; pc++) {
        exec(PATH[pc]);
    }
}

@compute @workgroup_size(SWEEP_WG)
fn sweep_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid == 0u {
        shape = vec4<u32>(grid_size(), grid[7] & C.lit & u32(C.go != 0u));
    }
    let s = workgroupUniformLoad(&shape);
    if s.w == 0u || wid.x >= C.dirs {
        return;
    }
    at_dir = wid.x;
    at_size = s.xyz;
    // This dispatch's layers: as many as keep a thread's nodes within
    // `SWEEP_WORK`, the depths carried in `scratch` between dispatches.
    let layer = s.x * s.y;
    let per = max(1u, SWEEP_WORK / ((layer + SWEEP_WG - 1u) / SWEEP_WG));
    let first = C.chunk * per;
    for (var step = first; step < min(s.z, first + per); step++) {
        at_k = s.z - 1u - step;
        for (var ij = lid; ij < layer; ij += SWEEP_WG) {
            at_ij = ij;
            for (var pc = 0u; pc < DEPTH_LEN; pc++) {
                exec(DEPTH[pc]);
            }
        }
        storageBarrier();
        workgroupBarrier();
    }
}

// The column's weights and lower node, then the four values of the layer
// above about it, from `scratch` (depths) or `field` (densities); none
// past the box or above the top layer.
fn load_column(depths: bool, first: u32) {
    let rec = column_record();
    for (var t = 0u; t < 4u; t++) {
        R[R_W0 + t] = scratch[rec + t];
    }
    let low = bitcast<vec2<i32>>(scratch[rec + 4u]);
    let layer = layer_of();
    let top = at_k + 1u >= at_size.z;
    for (var t = 0u; t < 4u; t++) {
        let i = low.x + i32(t >> 1u);
        let j = low.y + i32(t & 1u);
        let inside = !top && i >= 0 && j >= 0 && u32(i) < at_size.x && u32(j) < at_size.y;
        let above = (at_k + 1u) * layer + u32(max(j, 0)) * at_size.x + u32(max(i, 0));
        var v = vec2<u32>(0u);
        if inside && depths {
            v = scratch[at_dir * nodes_of() + above];
        } else if inside {
            v = f64_from_f32(field[above]);
        }
        R[first + t] = v;
    }
}

fn node_slot() -> u32 {
    return at_dir * nodes_of() + at_k * layer_of() + at_ij;
}

fn hook(code: u32) {
    switch code {
        case H_COLUMN: {
            R[R_FX] = f64_floor(R[R_X]);
            R[R_FY] = f64_floor(R[R_Y]);
            col_low = vec2<i32>(f64_to_i32(R[R_FX]), f64_to_i32(R[R_FY]));
        }
        case H_STORE_W: {
            let rec = column_record();
            for (var t = 0u; t < 4u; t++) {
                scratch[rec + t] = R[R_W0 + t];
            }
            scratch[rec + 4u] = bitcast<vec2<u32>>(col_low);
        }
        case H_LOAD_PATH: {
            load_column(false, R_V0);
            R[R_RHO] = f64_from_f32(field[at_k * layer_of() + at_ij]);
        }
        case H_STORE_A, H_STORE_DEP: {
            scratch[node_slot()] = R[select(R_A, R_DEP, code == H_STORE_DEP)];
        }
        case H_LOAD_DEPTH: {
            load_column(true, R_D0);
            R[R_A] = scratch[node_slot()];
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
    load_constants();
    R[R_SUM] = vec2<u32>(0u);
    for (var d = 0u; d < C.dirs; d++) {
        R[R_XN] = f64_neg(scratch[d * n + i]);
        R[R_SHARE] = C.dir[2u * d + 1u].zw;
        for (var pc = 0u; pc < SHADE_LEN; pc++) {
            exec(SHADE[pc]);
        }
    }
    // `clamp(0.0, 1.0)`.
    var sum = R[R_SUM];
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
        sum = sum + exact_kept(f64_to_f32(R[R_WC0 + c]) * l);
    }
    if sum < 0.0 {
        sum = 0.0;
    }
    if sum > 1.0 {
        sum = 1.0;
    }
    reads[k] = sum;
}
