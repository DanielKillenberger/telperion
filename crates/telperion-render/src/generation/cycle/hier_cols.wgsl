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

// The shape, per axis: its depth, block, depth in its block and
// generation in its cycle; per block, its level.
const H_DEPTH: u32 = 0u;
const H_BLOCK: u32 = 1u;
const H_LOCAL: u32 = 2u;
const H_GEN: u32 = 3u;
const H_LEVEL: u32 = 4u;

// The ordered lists (`cur`, `next`), `stride + 64` rows a column: each
// axis's children (a start per axis, then the children), each block's
// members by local depth, each level's blocks.
const R_KID_START: u32 = 0u;
const R_KIDS: u32 = 1u;
const R_MEM_START: u32 = 2u;
const R_MEMBERS: u32 = 3u;
const R_LVL_START: u32 = 4u;
const R_LVL_BLOCKS: u32 = 5u;
