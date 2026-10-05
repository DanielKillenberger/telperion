//! The tree space's pedunculate oak through the pipeline's own expansion
//! and the headless renderer: bare and whole stills per age and seed, the
//! camera fitted to each tree. The structure comes from `telperion-space`;
//! the wood, leaves and material from the oregon-white-oak preset's rows.
//!
//!   cargo run --release -p telperion-render --example space_oak -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;
#[path = "space/tree.rs"]
mod tree;

/// The preset's rows restated for a tree that grows its own short shoots:
/// no clusters seated along slender wood, leaves along the grown shoots.
const ROWS: &str = r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.06, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.008}}}}"#;

fn main() -> Result<(), String> {
    still::run(
        "oak",
        telperion_space::oak,
        &tree::OAK_TRUNK,
        "oregon-white-oak",
        ROWS,
    )
}
