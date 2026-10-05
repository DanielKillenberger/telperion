//! The tree space's beech through the pipeline's own expansion and the
//! headless renderer: bare and whole stills per age and seed, the camera
//! fitted to each tree. The structure comes from `telperion-space`; the
//! wood, leaves and material from the european-beech preset's rows.
//!
//!   cargo run --release -p telperion-render --example space_beech -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;
#[path = "space/tree.rs"]
mod tree;

/// The preset's rows restated for a tree that grows its own short shoots:
/// the clusters the preset seats along slender wood stood in for them.
const ROWS: &str = r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#;

fn main() -> Result<(), String> {
    still::run(
        "beech",
        telperion_space::beech,
        &tree::BEECH_TRUNK,
        "european-beech",
        ROWS,
    )
}
