// The cycle's light on the device (DESIGN.md section 2, step 8), the
// CPU's `Field::quantised` and `Field::at` word for word:
// - `span_main`: the box over the leaves then the tips, each coordinate's
//   node the floor of twice it, by order-free atomics;
// - `grid_main`: the box's size, refused past the contract's nodes or the
//   buffers' capacity;
// - `deposit_main`: each leaf's area times each corner's f32 weight as
//   integer quanta of 2^-20 m², a u32 a node, overflow refused by name;
// - `densities_main`: 8 q 2^-20, to f32 once;
// - `weights_main`, `path_main`, `sweep_main`: `swept` in software f64
//   (`light_vm.wgsl`), its parts that read no depth over every node at
//   once, its depth recurrence one workgroup a sky direction, a range of
//   layers a dispatch (`light_sweep.wgsl`);
// - `combine_main`: each direction's light, libm's `exp`, summed in order,
//   clamped, to f32 once;
// - `read_main`: each tip's eight corners in f32, as the CPU sums them.
// Errors set bits in `state[6]`; a refused box stops every later entry.

struct Cfg {
    leaves: u32,
    tips: u32,
    dirs: u32,
    capacity: u32,
    lit: u32,
    row: u32,
    // The depth recurrence's range of layers.
    chunk: u32,
    // A word the driver cannot see is zero (`exact.wgsl`).
    zero: u32,
    // The first item of this dispatch's range of its list.
    base: u32,
    pad1: u32,
    pad2: u32,
    pad3: u32,
    // Per direction: x/z, y/z, extinction CELL/z and its share, as f64s,
    // two to a row.
    dir: array<vec4<u32>, 18>,
}

@group(0) @binding(0) var<uniform> C: Cfg;
// The leaves (x, y, z, area), then the tips (x, y, z).
@group(0) @binding(1) var<storage, read> points: array<vec4<f32>>;
// Low node (3), high node (3), errors.
@group(0) @binding(2) var<storage, read_write> state: array<atomic<i32>>;
// Low node (3), size (3), nodes, whether the box stands.
@group(0) @binding(3) var<storage, read_write> grid: array<u32>;
@group(0) @binding(4) var<storage, read_write> quanta: array<atomic<u32>>;
// The densities, then from `C.capacity` the lattice.
@group(0) @binding(5) var<storage, read_write> field: array<f32>;
// Each direction's share of the light at every node, then from
// `C.dirs C.capacity` two layers' depths a direction, alternating.
@group(0) @binding(6) var<storage, read_write> scratch: array<vec2<u32>>;
@group(0) @binding(7) var<storage, read_write> reads: array<f32>;

const NONE: u32 = 0xffffffffu;
const E_REACH: u32 = 1u;
const E_NODES: u32 = 2u;
const E_CAPACITY: u32 = 4u;
const E_AREA: u32 = 8u;
const E_DEPOSIT: u32 = 16u;
const E_OUTSIDE: u32 = 32u;
const ONE64: vec2<u32> = vec2<u32>(0u, 0x3ff00000u);

fn exact_zero() -> u32 {
    return C.zero;
}

fn fail(bit: u32) {
    atomicOr(&state[6], i32(bit));
}

// Twice an f64 that is an f32 widened: exact.
fn twice(x: vec2<u32>) -> vec2<u32> {
    if is_zero64(x) {
        return x;
    }
    return vec2<u32>(x.x, x.y + (1u << 20u));
}

fn invocation(id: vec3<u32>) -> u32 {
    return C.base + id.x + id.y * C.row;
}

@compute @workgroup_size(64)
fn span_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= C.leaves + C.tips {
        return;
    }
    let p = points[k].xyz;
    for (var a = 0u; a < 3u; a++) {
        if (bitcast<u32>(p[a]) & 0x7fffffffu) >= 0x45800000u {
            fail(E_REACH);
            continue;
        }
        let n = f64_to_i32(f64_floor(twice(f64_from_f32(p[a]))));
        atomicMin(&state[a], n);
        atomicMax(&state[a + 3u], n + 1);
    }
}

@compute @workgroup_size(1)
fn grid_main() {
    var low = vec3<i32>(atomicLoad(&state[0]), atomicLoad(&state[1]), atomicLoad(&state[2]));
    var high = vec3<i32>(atomicLoad(&state[3]), atomicLoad(&state[4]), atomicLoad(&state[5]));
    if low.x > high.x {
        low = vec3<i32>(0);
        high = vec3<i32>(0);
    }
    let size = vec3<u32>(high - low + vec3<i32>(1));
    let nodes = mul32(size.x * size.y, size.z);
    var ok = atomicLoad(&state[6]) == 0;
    if nodes.y != 0u || nodes.x > (1u << 28u) {
        fail(E_NODES);
        ok = false;
    } else if nodes.x > C.capacity {
        fail(E_CAPACITY);
        ok = false;
    }
    for (var a = 0u; a < 3u; a++) {
        grid[a] = u32(low[a]);
        grid[a + 3u] = size[a];
    }
    grid[6] = nodes.x;
    grid[7] = select(0u, 1u, ok);
}

fn standing() -> bool {
    return grid[7] == 1u;
}

// A node's index, x fastest; none outside the box.
fn index(n: vec3<i32>) -> u32 {
    let k = n - vec3<i32>(i32(grid[0]), i32(grid[1]), i32(grid[2]));
    let size = vec3<i32>(i32(grid[3]), i32(grid[4]), i32(grid[5]));
    if any(k < vec3<i32>(0)) || any(k >= size) {
        return NONE;
    }
    return u32((k.z * size.y + k.y) * size.x + k.x);
}

// The node below each coordinate of the point the corners are about.
var<private> corner_node: vec3<i32>;

// The eight corners' f32 weights about `p` into WC0..WC7 (`corners`).
fn corners_of(p: vec3<f32>) {
    load_constants();
    for (var a = 0u; a < 3u; a++) {
        R[R_S_0 + a] = twice(f64_from_f32(p[a]));
    }
    for (var pc = 0u; pc < CORNERS_LEN; pc++) {
        exec(CORNERS[pc]);
    }
}

fn corner_index(c: u32) -> u32 {
    let up = vec3<i32>(i32(c & 1u), i32((c >> 1u) & 1u), i32((c >> 2u) & 1u));
    return index(corner_node + up);
}

// Round(area w 2^20), half away from zero, of two non-negative f32s,
// exactly: their 24-bit significands' product shifted. `y` set past a
// u32, or for an infinite area.
fn quanta_of(area: u32, w: u32) -> vec2<u32> {
    if (area >> 23u) == 0xffu {
        return vec2<u32>(0u, 1u);
    }
    if (area & 0x7fffffffu) == 0u || (w & 0x7fffffffu) == 0u {
        return vec2<u32>(0u);
    }
    let ea = max(i32(area >> 23u), 1) - 127;
    let ew = max(i32(w >> 23u), 1) - 127;
    let ma = (area & 0x7fffffu) | select(0u, 0x800000u, (area >> 23u) != 0u);
    let mw = (w & 0x7fffffu) | select(0u, 0x800000u, (w >> 23u) != 0u);
    let prod = mul32(ma, mw);
    let sh = ea + ew - 26;
    if sh >= 0 {
        if i32(64u - u64_clz(prod)) + sh > 32 {
            return vec2<u32>(0u, 1u);
        }
        return u64_shl(prod, u32(sh));
    }
    let k = u32(-sh);
    let half = u64_shr(prod, k - 1u).x & 1u;
    let q = u64_add(u64_shr(prod, k), vec2<u32>(half, 0u));
    return vec2<u32>(q.x, select(0u, 1u, q.y != 0u));
}

@compute @workgroup_size(64)
fn deposit_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= C.leaves || !standing() {
        return;
    }
    let leaf = points[k];
    let area = bitcast<u32>(leaf.w);
    if (area & 0x7fffffffu) > 0x7f800000u || (area > SIGN) {
        fail(E_AREA);
        return;
    }
    corners_of(leaf.xyz);
    for (var c = 0u; c < 8u; c++) {
        let at = corner_index(c);
        let q = quanta_of(area, bitcast<u32>(f64_to_f32(R[R_WC0 + c])));
        if at == NONE {
            fail(E_OUTSIDE);
            continue;
        }
        if q.y != 0u {
            fail(E_DEPOSIT);
            continue;
        }
        if q.x != 0u {
            let old = atomicAdd(&quanta[at], q.x);
            if old + q.x < old {
                fail(E_DEPOSIT);
            }
        }
    }
}

@compute @workgroup_size(64)
fn densities_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    if !standing() || i >= grid[6] {
        return;
    }
    let q = atomicLoad(&quanta[i]);
    var x = f64_from_u32(q);
    if q != 0u {
        x.y -= 17u << 20u;
    }
    field[i] = f64_to_f32(x);
}
