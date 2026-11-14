//! The stage a build is in, so a request the machine refuses where the
//! code cannot return the refusal is still named by its stage (fn-369): the
//! Wasm binding's allocator records it beside the bytes asked. A tag is set
//! at a boundary the code already has; it adds no stage. Each is one static
//! store.
use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

/// Before growth: the request read and the family parsed.
pub const BEFORE: u32 = 0;
// Growth tags each year by its own number, 1 to 999.
/// The grown tree settled and laid (`Paused::finish`, `Paused::settle`).
pub const SETTLE: u32 = 1001;
/// The settled tree put in lineage order.
pub const ORDER: u32 = 1002;
/// The structure established on its trunk rows (`Establishing::apply`).
pub const ESTABLISH: u32 = 1003;
/// The structure converted into the pipeline's tree.
pub const CONVERT: u32 = 1004;
/// The tree clothed and its buffers fitted.
pub const CLOTHE: u32 = 1005;
/// The wood's rings, and its curve where it is traced.
pub const RINGS: u32 = 1006;
/// The wood's mesh around its rings.
pub const FACES: u32 = 1007;
/// The leaf plan and element prepared.
pub const PREPARE: u32 = 1008;
/// The leaves placed and culled, and the field.
pub const FOLIAGE: u32 = 1009;
/// The flowers.
pub const ORGAN: u32 = 1010;
/// The structure transfer.
pub const STRUCTURE: u32 = 1011;
/// A curve traced apart from the wood.
pub const CURVE: u32 = 1012;
/// The binding's buffers handed out.
pub const HANDOUT: u32 = 1013;

static STAGE: AtomicU32 = AtomicU32::new(BEFORE);

/// The build is in `stage` from here on.
pub fn at(stage: u32) {
    STAGE.store(stage, Relaxed);
}

/// The stage the build is in.
pub fn now() -> u32 {
    STAGE.load(Relaxed)
}

/// A stage's name, as a refusal states it; every growth year is `growth`.
pub fn name(stage: u32) -> &'static str {
    match stage {
        BEFORE => "request",
        1..=999 => "growth",
        SETTLE => "settle",
        ORDER => "lineage order",
        ESTABLISH => "establishing",
        CONVERT => "conversion",
        CLOTHE => "clothing",
        RINGS => "wood's rings",
        FACES => "wood's mesh",
        PREPARE => "leaf plan",
        FOLIAGE => "foliage",
        ORGAN => "flowers",
        STRUCTURE => "structure transfer",
        CURVE => "curve",
        HANDOUT => "handout",
        _ => "build",
    }
}
