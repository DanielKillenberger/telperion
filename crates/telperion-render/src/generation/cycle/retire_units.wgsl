// fn-256 task 2 (DESIGN.md section 6, compaction at a move): each unit
// record's first phytomer remapped to its place in the compacted arena,
// or, its axis retired, its count none and its axis's phytomers none: a
// retired root stands for its subtree by its frozen girth (fn-270 R4).

struct Cfg {
    // The units' and the axes' strides, `ret`'s stride, the census's
    // units word, a dispatch row.
    units_cap: u32,
    axes_cap: u32,
    rstride: u32,
    units_word: u32,
    row: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> census: array<u32>;
@group(0) @binding(2) var<storage, read> ret: array<u32>;
@group(0) @binding(3) var<storage, read> scan: array<u32>;
@group(0) @binding(4) var<storage, read_write> units: array<u32>;
@group(0) @binding(5) var<storage, read_write> axes: array<u32>;

const U_AXIS: u32 = 0u;
const U_FIRST: u32 = 11u;
const U_COUNT: u32 = 12u;
const A_PHYTS: u32 = 17u;
const R_FATE: u32 = 1u;
const RETIRED: u32 = 2u;

@compute @workgroup_size(64)
fn units_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let u = id.x + id.y * cfg.row;
    if u >= census[cfg.units_word] {
        return;
    }
    let axis = units[U_AXIS * cfg.units_cap + u];
    if ret[R_FATE * cfg.rstride + axis] < RETIRED {
        units[U_FIRST * cfg.units_cap + u] = scan[units[U_FIRST * cfg.units_cap + u]];
        return;
    }
    units[U_COUNT * cfg.units_cap + u] = 0u;
    axes[A_PHYTS * cfg.axes_cap + axis] = 0u;
}
