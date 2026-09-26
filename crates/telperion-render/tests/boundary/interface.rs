// The positive control: the renderer builds through the executor interface.
use telperion_core::{pipeline::executor, presets::Preset, surface::SurfaceMesh, Result};

fn wood(family: &telperion_core::Family) -> Result<SurfaceMesh> {
    executor::grow(family)?.expansion()?.wood()
}

fn main() {
    let _ = (wood, Preset::Ordinary);
}
