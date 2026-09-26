//! Stage 2 alone: grows one preset's skeleton `GROWTH_SAMPLES` times (default
//! 6, the first cold) and prints each build's milliseconds with a hash of
//! every node's full debug record, so two builds compare byte for byte.
use telperion_core::{pipeline, presets::Preset};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().collect();
    let preset = args.get(1).ok_or("preset required")?;
    let seed: u32 = args.get(2).ok_or("seed required")?.parse()?;
    let mut f = Preset::from_id(preset)
        .ok_or("unknown preset")?
        .parameters();
    f.skeleton.seed = seed;
    let samples: usize = std::env::var("GROWTH_SAMPLES")
        .unwrap_or_else(|_| "6".into())
        .parse()?;
    for sample in 0..samples {
        // A request for no output runs the skeleton stage alone.
        let built = pipeline::build(&f, pipeline::Request::default())?;
        let (skeleton, ms) = (built.skeleton, built.outputs.stages.skeleton_ms);
        let mut hash = 14695981039346656037_u64;
        for node in &skeleton.tree.nodes {
            for byte in format!("{node:?}").bytes() {
                hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
            }
        }
        println!(
            "{{\"preset\":\"{preset}\",\"seed\":{seed},\"sample\":{sample},\"ms\":{ms:.3},\"nodes\":{},\"shed\":{},\"tree_fnv1a64\":\"{hash:016x}\"}}",
            skeleton.tree.nodes.len(),
            skeleton.shed
        );
    }
    Ok(())
}
