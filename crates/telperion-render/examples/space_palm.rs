//! The date palm through the one pipeline and the headless renderer: bare
//! and whole stills per age and seed, the camera fitted to each tree. The
//! date-palm preset carries the engine species, its dressing and its
//! material (fn-198).
//!
//!   cargo run --profile ci -p telperion-render --example space_palm -- <out dir> <age>... [--seeds 1,7]
#[path = "space/still.rs"]
mod still;

fn main() -> Result<(), String> {
    still::run("palm", "date-palm")
}
