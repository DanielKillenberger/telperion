//! The ordinary tree, the tree space's centroid (fn-215), through the one
//! pipeline and the headless renderer: bare and whole stills per age and
//! seed, the camera fitted to each tree.
//!
//!   cargo run --profile ci -p telperion-render --example space_ordinary -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;

fn main() -> Result<(), String> {
    still::run("ordinary", "ordinary")
}
