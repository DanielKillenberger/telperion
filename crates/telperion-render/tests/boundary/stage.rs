// A stage called from the renderer around the executor interface.
use telperion_core::{presets::Preset, surface};

fn main() {
    let family = Preset::Ordinary.parameters();
    let tree = telperion_core::tree::Tree::default();
    let _ = surface::build(&tree, family.skeleton.envelope.height, &family.surface);
}
