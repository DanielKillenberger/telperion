//! The tree space's date palm through the pipeline's own expansion and the
//! headless renderer: bare and whole stills per age and seed, the camera
//! fitted to each tree. The stem comes from `telperion-space`; the fronds,
//! the retained leaf bases, the skirt and the material from the date-palm
//! preset's rows, unchanged.
//!
//!   cargo run --profile ci -p telperion-render --example space_palm -- <out dir> <age>... [--seeds 1,7] [--today]
#[path = "space/still.rs"]
mod still;
#[path = "space/tree.rs"]
mod tree;

fn main() -> Result<(), String> {
    still::run(
        "palm",
        telperion_space::palm,
        &tree::PALM_TRUNK,
        "date-palm",
        "{}",
    )
}
