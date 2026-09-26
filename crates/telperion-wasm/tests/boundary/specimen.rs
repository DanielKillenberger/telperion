// The scaffold's own grower, called from the binding around the pipeline.
use telperion_core::{branching::Specimen, presets::Preset};

fn main() {
    let family = Preset::Ordinary.parameters();
    let _ = Specimen::grow(&family.skeleton, family.radii);
}
