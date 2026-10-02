//! Stage 2 alone: grows one preset's skeleton `GROWTH_SAMPLES` times (default
//! 6, the first cold) and prints each build's milliseconds with a hash of
//! every node's full debug record, so two builds compare byte for byte.
//! Built with `--features query-count` it also prints the crown radius
//! queries each build made by purpose, and the axes the twig layer planned;
//! time a build without it, since counting changes the cost it counts.
use telperion_core::envelope::queries::{self, Purpose};
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
        let _ = queries::take();
        // A request for no output runs the skeleton stage alone.
        let leaves = std::env::var_os("LADDER_STATS").is_some();
        let built = pipeline::build(
            &f,
            pipeline::Request {
                leaves,
                ..pipeline::Request::default()
            },
        )?;
        if let Some(l) = &built.outputs.leaves {
            eprintln!(
                "LADDER_LEAVES {{\"placed\":{},\"retained\":{}}}",
                l.placed, l.retained
            );
        }
        let (skeleton, ms) = (built.skeleton, built.outputs.stages.skeleton_ms);
        let mut hash = 14695981039346656037_u64;
        for node in &skeleton.tree.nodes {
            for byte in format!("{node:?}").bytes() {
                hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
            }
        }
        println!(
            "{{\"preset\":\"{preset}\",\"seed\":{seed},\"sample\":{sample},\"ms\":{ms:.3},\"nodes\":{},\"shed\":{},\"tree_fnv1a64\":\"{hash:016x}\"{}}}",
            skeleton.tree.nodes.len(),
            skeleton.shed,
            queries::take().map_or(String::new(), counted)
        );
    }
    Ok(())
}

/// The counts as JSON fields after the build's own.
fn counted(c: queries::Counts) -> String {
    let by: Vec<_> = Purpose::ALL
        .iter()
        .map(|p| format!("\"{}\":{}", p.name(), c.queries[*p as usize]))
        .collect();
    format!(
        ",\"radius_queries\":{},\"by_purpose\":{{{}}},\"planned_axes\":{}",
        c.queries.iter().sum::<u64>(),
        by.join(","),
        c.planned_axes
    )
}
