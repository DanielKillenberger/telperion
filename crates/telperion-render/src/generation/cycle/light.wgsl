// The cycle's light on the device (DESIGN.md sections 2, step 8, and 19),
// the CPU's `Field::quantised` and `Field::at` in f32:
// - `span_main`: the box over the leaves then the tips, each coordinate's
//   node the floor of twice it, by order-free atomics; and the highest
//   node a leaf with area reaches, above which nothing shades;
// - `grid_main`: the box's size, refused past the contract's nodes or the
//   buffers' capacity, and the layers the sweep takes;
// - `deposit_main`: each leaf's area times each corner's f32 weight as
//   integer quanta of 2^-20 m², a u32 a node, overflow refused by name;
// - `densities_main`: 8 q 2^-20, to f32 once;
// - `sweep_main` and `read_main` (`light_sweep.wgsl`): each direction's
//   optical depths down the occupied layers, then each tip's eight
//   corners, each corner's light summed over the directions as it is read.
// The box, corners, quanta and densities are the CPU's word for word: each
// f32 operation is the CPU's, and none is a product added. Errors set bits
// in `state[6]`; a refused box stops every later entry.

struct Cfg {
    leaves: u32,
    tips: u32,
    dirs: u32,
    capacity: u32,
    lit: u32,
    row: u32,
    // The sweep's range of layers.
    chunk: u32,
    // The most whole cells a direction's ray moves across a layer, on
    // either axis: the tiles' reach.
    reach: u32,
    // The first item of this dispatch's range of its list.
    base: u32,
    // None where the cycle halted (`vm.wgsl`): nothing stands to light.
    go: u32,
    // The light where no leaf shades: the shares summed and held to one,
    // rounded once, as the CPU takes it.
    open: f32,
    // Whether the lattice sleeps (`build_light.rs`): nothing shades, its
    // dispatches are not recorded, and every node's light is one.
    dormant: u32,
    // Per direction, two rows: its whole cells' offset across a layer (x,
    // y, as i32s) and its bilinear weights in `bilinear`'s order; then the
    // extinction times its path, and its share; as f32 words.
    dir: array<vec4<u32>, 18>,
}

@group(0) @binding(0) var<uniform> C: Cfg;
// The leaves (x, y, z, area), then the tips (x, y, z).
@group(0) @binding(1) var<storage, read> points: array<vec4<f32>>;
// Low node (3), high node (3), errors, the highest node a leaf with area
// reaches.
@group(0) @binding(2) var<storage, read_write> state: array<atomic<i32>>;
// Low node (3), size (3), nodes, whether the box stands, the layers the
// sweep takes (from the bottom).
@group(0) @binding(3) var<storage, read_write> grid: array<u32>;
@group(0) @binding(4) var<storage, read_write> quanta: array<atomic<u32>>;
// The densities.
@group(0) @binding(5) var<storage, read_write> field: array<f32>;
// Each direction's optical depth at every node the sweep takes.
@group(0) @binding(6) var<storage, read_write> depths: array<f32>;
@group(0) @binding(7) var<storage, read_write> reads: array<f32>;

const NONE: u32 = 0xffffffffu;
const E_REACH: u32 = 1u;
const E_NODES: u32 = 2u;
const E_CAPACITY: u32 = 4u;
const E_AREA: u32 = 8u;
const E_DEPOSIT: u32 = 16u;
const E_OUTSIDE: u32 = 32u;

fn fail(bit: u32) {
    atomicOr(&state[6], i32(bit));
}

fn invocation(id: vec3<u32>) -> u32 {
    return C.base + id.x + id.y * C.row;
}

// A coordinate's node: the floor of twice it, as the CPU's `node`.
fn node_of(x: f32) -> f32 {
    return floor(x / 0.5);
}

@compute @workgroup_size(64)
fn span_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = invocation(id);
    if k >= C.leaves + C.tips {
        return;
    }
    let p = points[k];
    for (var a = 0u; a < 3u; a++) {
        if (bitcast<u32>(p[a]) & 0x7fffffffu) >= 0x45800000u {
            fail(E_REACH);
            continue;
        }
        let n = i32(node_of(p[a]));
        atomicMin(&state[a], n);
        atomicMax(&state[a + 3u], n + 1);
        if a == 2u && k < C.leaves && (bitcast<u32>(p.w) & 0x7fffffffu) != 0u {
            atomicMax(&state[7], n + 1);
        }
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
    // The layers up to the highest a leaf with area reaches; above it
    // every depth is nothing.
    let top = atomicLoad(&state[7]);
    grid[8] = select(0u, min(u32(top - low.z) + 1u, size.z), top >= low.z);
}

fn standing() -> bool {
    return grid[7] == 1u && C.go != 0u;
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

// The node below each coordinate of the point the corners are about, and
// the eight corners' weights, corner `c` upper on axis `a` where bit `a`
// of `c` is set: the CPU's `corners`, each weight 1 times the axes'
// fractions or their complements in axis order.
var<private> corner_node: vec3<i32>;
var<private> corner_w: array<f32, 8>;

fn corners_of(p: vec3<f32>) {
    var f: vec3<f32>;
    var g: vec3<f32>;
    for (var a = 0u; a < 3u; a++) {
        let s = p[a] / 0.5;
        let n = floor(s);
        corner_node[a] = i32(n);
        f[a] = s - n;
        g[a] = 1.0 - f[a];
    }
    for (var c = 0u; c < 8u; c++) {
        var w = 1.0;
        for (var a = 0u; a < 3u; a++) {
            w = w * select(g[a], f[a], ((c >> a) & 1u) == 1u);
        }
        corner_w[c] = w;
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
        let bits = select(32u - countLeadingZeros(prod.x), 64u - countLeadingZeros(prod.y), prod.y != 0u);
        if i32(bits) + sh > 32 {
            return vec2<u32>(0u, 1u);
        }
        return shl64(prod, u32(sh));
    }
    let k = u32(-sh);
    let half = shr64(prod, k - 1u).x & 1u;
    let q = add64(shr64(prod, k), vec2<u32>(half, 0u));
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
    if (area & 0x7fffffffu) > 0x7f800000u || (area > 0x80000000u) {
        fail(E_AREA);
        return;
    }
    corners_of(leaf.xyz);
    for (var c = 0u; c < 8u; c++) {
        let at = corner_index(c);
        let q = quanta_of(area, bitcast<u32>(corner_w[c]));
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

// A u32 to f32 to nearest, ties to even, by its kept bits: WGSL leaves
// the conversion's rounding to the driver.
fn rounded(q: u32) -> f32 {
    let drop = 8u - min(countLeadingZeros(q), 8u);
    if drop == 0u {
        return f32(q);
    }
    let kept = q >> drop;
    let rest = q & ((1u << drop) - 1u);
    let half = 1u << (drop - 1u);
    let up = rest > half || (rest == half && (kept & 1u) == 1u);
    return f32(kept + select(0u, 1u, up)) * f32(1u << drop);
}

@compute @workgroup_size(64)
fn densities_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    if !standing() || i >= grid[6] {
        return;
    }
    // 8 q 2^-20, a power of two times q rounded once.
    field[i] = rounded(atomicLoad(&quanta[i])) * (1.0 / 131072.0);
}
