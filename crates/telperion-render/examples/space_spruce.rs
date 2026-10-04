//! The tree space's Norway spruce through the pipeline's own expansion and
//! the headless renderer: bare and whole stills per age and seed, the
//! camera fitted to each tree. The structure comes from `telperion-space`;
//! the wood, needles and material from the norway-spruce preset's rows.
//!
//!   cargo run --release -p telperion-render --example space_spruce -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;
#[path = "space/tree.rs"]
mod tree;

/// The preset's rows restated for a tree that grows its own shoots: no
/// clusters seated along slender wood, needles along the grown shoots.
const ROWS: &str = r#"{"canopy": {"shortShootSpacing": 0, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.0025}}}}"#;

fn main() -> Result<(), String> {
    still::run("spruce", telperion_space::spruce, "norway-spruce", ROWS)
}
