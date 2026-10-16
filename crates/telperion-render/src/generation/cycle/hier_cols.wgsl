// fn-256 task 2 (DESIGN.md sections 1.6, 3 and 4): the hierarchy's
// columns, shared by its build and its sweeps. Every table is columns of
// one word a row, `stride` rows apart (a multiple of 64, so a column
// binds at a 256-byte offset for the scan).

const NONE: u32 = 0xffffffffu;
const WG: u32 = 64u;

// The tree, per laid axis: its bearer and node (`NONE` at the tip), PA,
// lineage, wood and light (f64 bits), and its pencil's f32s.
const T_PARENT: u32 = 0u;
const T_NODE: u32 = 1u;
const T_PA: u32 = 2u;
const T_LINEAGE: u32 = 3u;
const T_WOOD: u32 = 5u;
const T_LIGHT: u32 = 7u;
const T_BASE: u32 = 9u;
const T_RUNNING: u32 = 10u;
const T_KEPT: u32 = 11u;
const T_MEMORY: u32 = 12u;
const T_SIZE: u32 = 13u;
// A retired root's frozen upkeep (`compact.rs`), none where unset.
const T_FROZEN: u32 = 14u;

// The shape, per axis: its depth, block and generation in its cycle; per
// block, its open spill (`NONE` until it opens one), its level, and the
// full block it spills from (`NONE` for the root's).
const H_DEPTH: u32 = 0u;
const H_BLOCK: u32 = 1u;
const H_SPILL: u32 = 2u;
const H_GEN: u32 = 3u;
const H_LEVEL: u32 = 4u;
const H_FROM: u32 = 5u;

// The ordered lists (`cur`, `next`), `stride + 64` rows a column: each
// axis's children (a start per axis, then the children), each block's
// members by depth, each level's blocks.
const R_KID_START: u32 = 0u;
const R_KIDS: u32 = 1u;
const R_MEM_START: u32 = 2u;
const R_MEMBERS: u32 = 3u;
const R_LVL_START: u32 = 4u;
const R_LVL_BLOCKS: u32 = 5u;

// A block's run (its members' first and end, its least and greatest
// depth) within a hierarchy of `stride` rows, none where a halted cycle
// left it past them: no sweep loops on what it left (fn-256 task 5).
fn within(run: vec4<u32>, stride: u32) -> vec4<u32> {
    let ok = run.x < run.y && run.y <= stride && run.z <= run.w && run.w - run.z < stride;
    return select(vec4<u32>(0u), run, ok);
}

// An axis's children's range within the lists' rows, none past them.
fn kids_within(kids: vec2<u32>, stride: u32) -> vec2<u32> {
    let ok = kids.x <= kids.y && kids.y <= stride + 64u;
    return select(vec2<u32>(0u), kids, ok);
}
