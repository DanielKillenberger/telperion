//! The silver birch through the one pipeline and the headless renderer:
//! bare and whole stills per age and seed, the camera fitted to each tree.
//! The silver-birch preset carries the engine species, its dressing and
//! its material (fn-213).
//!
//!   cargo run --profile ci -p telperion-render --example space_birch -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;

fn main() -> Result<(), String> {
    still::run("birch", "silver-birch")
}
