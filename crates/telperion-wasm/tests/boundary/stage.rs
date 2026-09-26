// A stage called from the binding around the pipeline.
use telperion_core::{branching, presets::Preset};

fn main() {
    let family = Preset::Ordinary.parameters();
    let _ = branching::generate(&family.skeleton, family.radii);
}
