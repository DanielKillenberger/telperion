// The positive control: the binding builds through the pipeline.
use telperion_core::{pipeline, presets::Preset, Result};

fn skeleton(family: &telperion_core::Family) -> Result<pipeline::Skeleton> {
    Ok(pipeline::build(family, pipeline::Request::default())?.skeleton)
}

fn main() {
    let _ = (skeleton, Preset::Ordinary);
}
