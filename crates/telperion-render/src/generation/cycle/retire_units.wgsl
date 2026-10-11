// fn-256 task 2 (DESIGN.md section 6, compaction at a move): each unit
// record's first phytomer remapped to its place in the compacted arena,
// or, its axis retired, into the records (flagged by the high bit); where
// no PA keeps the retired rows, its count none and its axis's phytomers
// none. A unit already remapped into the records stays.

struct Cfg {
    // The units' and the axes' strides, `ret`'s stride, the scans'
    // stride, the records already held before this compaction, whether
    // rows are recorded, the census's units word, a dispatch row.
    units_cap: u32,
    axes_cap: u32,
    rstride: u32,
    sstride: u32,
    rec_base: u32,
    keeps: u32,
    units_word: u32,
    row: u32,
}

@group(0) @binding(0) var<uniform> cfg: Cfg;
@group(0) @binding(1) var<storage, read> census: array<u32>;
@group(0) @binding(2) var<storage, read> ret: array<u32>;
@group(0) @binding(3) var<storage, read> scan: array<u32>;
@group(0) @binding(4) var<storage, read_write> units: array<u32>;
@group(0) @binding(5) var<storage, read_write> axes: array<u32>;

const RECORDED: u32 = 0x80000000u;
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
    let first = units[U_FIRST * cfg.units_cap + u];
    if (first & RECORDED) != 0u {
        return;
    }
    let axis = units[U_AXIS * cfg.units_cap + u];
    if ret[R_FATE * cfg.rstride + axis] < RETIRED {
        units[U_FIRST * cfg.units_cap + u] = scan[first];
        return;
    }
    if cfg.keeps != 0u {
        units[U_FIRST * cfg.units_cap + u] = RECORDED | (cfg.rec_base + scan[cfg.sstride + first]);
    } else {
        units[U_COUNT * cfg.units_cap + u] = 0u;
        axes[A_PHYTS * cfg.axes_cap + axis] = 0u;
    }
}
