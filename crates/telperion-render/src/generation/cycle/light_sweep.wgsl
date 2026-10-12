// The light's sweep and read (`light.wgsl` holds the bindings): the
// CPU's `swept` and `Field::at` in f32 (DESIGN.md section 19).
//
// A direction's optical depth at a node is the depth the layer above
// holds where its ray meets that layer, read bilinearly, plus the path
// term. Its ray moves the same whole cells `o` and fraction across every
// layer, so in coordinates skewed by `o` a layer down (`p = i + o m`, `m`
// the layers down from the one a dispatch starts from), a cell reads only
// the cell above it and its three neighbours on the far side. A
// workgroup takes a tile of TILE x TILE cells of one direction down
// LAYERS layers: it loads the layer above over the tile and LAYERS cells
// more on each axis, then each layer down one cell fewer, in its memory,
// and writes its own tile of each. A dispatch takes LAYERS layers of every
// direction from the occupied top down; the next takes the layer the last
// wrote as its layer above.
//
// The read sums each corner's light over the directions as it reads it:
// share times exp of minus the depth, held to one; above the swept layers
// every depth is nothing, and the light is the open sky's.

const SPAN: u32 = TILE + LAYERS;
const CELLS: u32 = SPAN * SPAN;

// Two layers' depths of a tile: the one above, and the one being taken.
var<workgroup> plane: array<f32, 2 * CELLS>;
// The box's columns on x and y, the swept layers, and whether it is swept.
var<workgroup> shape: vec4<u32>;

fn dir_word(d: u32, w: u32) -> u32 {
    return C.dir[2u * d + w / 4u][w % 4u];
}

fn dir_real(d: u32, w: u32) -> f32 {
    return bitcast<f32>(dir_word(d, w));
}

fn column_inside(ij: vec2<i32>, size: vec2<u32>) -> bool {
    return all(ij >= vec2<i32>(0)) && all(vec2<u32>(ij) < size);
}

fn at_node(ij: vec2<i32>, k: u32, size: vec2<u32>) -> u32 {
    return (k * size.y + u32(ij.y)) * size.x + u32(ij.x);
}

// A layer's density at a column, nothing outside the box.
fn density(ij: vec2<i32>, k: u32, size: vec2<u32>) -> f32 {
    if !column_inside(ij, size) {
        return 0.0;
    }
    return field[at_node(ij, k, size)];
}

@compute @workgroup_size(SWEEP_WG)
fn sweep_main(@builtin(workgroup_id) wid: vec3<u32>, @builtin(local_invocation_index) lid: u32) {
    if lid == 0u {
        shape = vec4<u32>(grid[3], grid[4], grid[8], grid[7] & C.lit & u32(C.go != 0u));
    }
    let s = workgroupUniformLoad(&shape);
    let size = s.xy;
    let more = C.reach * (LAYERS - 1u) + TILE - 1u;
    let across = (s.x + more) / TILE;
    let tiles = across * ((s.y + more) / TILE);
    let g = wid.x + wid.y * (C.row / 64u);
    let d = g / tiles;
    let done = C.chunk * LAYERS;
    if s.w == 0u || d >= C.dirs || done >= s.z {
        return;
    }
    let first = s.z - 1u - done;
    let count = min(LAYERS, first + 1u);
    let t = g % tiles;
    let o = vec2<i32>(bitcast<i32>(dir_word(d, 0u)), bitcast<i32>(dir_word(d, 1u)));
    let w = vec4<f32>(dir_real(d, 2u), dir_real(d, 3u), dir_real(d, 4u), dir_real(d, 5u));
    let ep = dir_real(d, 6u);
    let base = d * C.capacity;
    // The tile's least skewed column: the least the box's columns reach
    // over the dispatch's layers, `o m` for m in 1 to LAYERS.
    let origin = min(o, o * i32(LAYERS)) + vec2<i32>(vec2<u32>(t % across, t / across) * TILE);
    // The layer above, nothing above the swept layers.
    for (var c = lid; c < CELLS; c += SWEEP_WG) {
        let ij = origin + vec2<i32>(vec2<u32>(c % SPAN, c / SPAN));
        var v = 0.0;
        if first + 1u < s.z && column_inside(ij, size) {
            v = depths[base + at_node(ij, first + 1u, size)];
        }
        plane[c] = v;
    }
    workgroupBarrier();
    for (var m = 1u; m <= count; m++) {
        let k = first + 1u - m;
        let above = ((m + 1u) & 1u) * CELLS;
        let here = (m & 1u) * CELLS;
        let live = SPAN - m;
        for (var c = lid; c < CELLS; c += SWEEP_WG) {
            let uv = vec2<u32>(c % SPAN, c / SPAN);
            let ij = origin + vec2<i32>(uv) - o * i32(m);
            if all(uv < vec2<u32>(live)) {
                var depth = 0.0;
                if column_inside(ij, size) {
                    // `bilinear`'s corners in its order: (0, 0), (0, 1),
                    // (1, 0), (1, 1).
                    let at = above + c;
                    var d0 = w.x * plane[at];
                    d0 = d0 + w.y * plane[at + SPAN];
                    d0 = d0 + w.z * plane[at + 1u];
                    d0 = d0 + w.w * plane[at + SPAN + 1u];
                    var rho0 = 0.0;
                    if k + 1u < s.z {
                        let up = ij + o;
                        rho0 = w.x * density(up, k + 1u, size);
                        rho0 = rho0 + w.y * density(up + vec2<i32>(0, 1), k + 1u, size);
                        rho0 = rho0 + w.z * density(up + vec2<i32>(1, 0), k + 1u, size);
                        rho0 = rho0 + w.w * density(up + vec2<i32>(1, 1), k + 1u, size);
                    }
                    let rho = field[at_node(ij, k, size)];
                    depth = d0 + ep * (rho + rho0) * 0.5;
                    if all(uv < vec2<u32>(TILE)) {
                        depths[base + at_node(ij, k, size)] = depth;
                    }
                }
                plane[here + c] = depth;
            }
        }
        workgroupBarrier();
    }
}

// A node's light: each direction's share of exp of minus its depth,
// summed in the directions' order and held to one; the open sky's above
// the swept layers.
fn light_at(at: u32) -> f32 {
    if at / (grid[3] * grid[4]) >= grid[8] {
        return C.open;
    }
    var sum = 0.0;
    for (var d = 0u; d < C.dirs; d++) {
        sum = sum + dir_real(d, 7u) * exp(-depths[d * C.capacity + at]);
    }
    return clamp(sum, 0.0, 1.0);
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
        if at != NONE && C.dormant == 0u && C.lit != 0u {
            l = light_at(at);
        }
        sum = sum + corner_w[c] * l;
    }
    reads[k] = clamp(sum, 0.0, 1.0);
}
