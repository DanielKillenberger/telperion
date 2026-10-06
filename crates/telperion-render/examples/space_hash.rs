//! fn-198 task 2's baseline: the examples' handed-in path's grown trees,
//! hashed, for the four species at the named ages and seeds. Scratch.
#[path = "space/hash.rs"]
mod hash;
#[path = "space/tree.rs"]
mod tree;
use telperion_core::{params, pipeline::executor, presets::Preset};
use telperion_space::{beech, grow, oak, palm, spruce, Light, Request, Species};

const ROWS_BEECH: &str = r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#;

fn main() -> Result<(), String> {
    let oak_rows = include_str!("space_oak.rs");
    let spruce_rows = include_str!("space_spruce.rs");
    let rows = |src: &str| -> String {
        let start = src.find("r#\"").unwrap() + 3;
        let end = src[start..].find("\"#").unwrap() + start;
        src[start..end].to_string()
    };
    let cases: [(&str, fn() -> Species, &[usize], &str, String, Light); 4] = [
        ("beech", beech, &tree::BEECH_TRUNK, "european-beech", ROWS_BEECH.into(), Light::NEUTRAL),
        ("spruce", spruce, &tree::SPRUCE_TRUNK, "norway-spruce", rows(spruce_rows), Light::NEUTRAL),
        ("oak", oak, &tree::OAK_TRUNK, "oregon-white-oak", rows(oak_rows), Light { extinction: 0.5, sky: 0.5 }),
        ("palm", palm, &tree::PALM_TRUNK, "date-palm", "{}".into(), Light::NEUTRAL),
    ];
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (name, species, trunk, preset, rows, light) in cases {
        if !args.is_empty() && !args.iter().any(|a| a == name) {
            continue;
        }
        let preset = Preset::from_id(preset).unwrap().parameters();
        let rows: serde_json::Value = serde_json::from_str(&rows).map_err(|e| e.to_string())?;
        let family = params::overlay(&preset, &rows).map_err(|e| format!("{e:?}"))?;
        let species = species();
        for age in [40u32, 80] {
            for seed in [1u64, 7] {
                let request = Request { age, seed, budget: 20_000_000, light };
                let t = std::time::Instant::now();
                let structure = grow(&species, request).map_err(|e| format!("{e:?}"))?;
                let converted = tree::convert(&structure, trunk);
                drop(structure);
                let grown = hash::tree(&converted);
                let mut family = family.clone();
                family.skeleton.seed = (seed ^ (seed >> 32)) as u32;
                let x = executor::expand(converted, &family).map_err(|e| e.to_string())?;
                let clothed = hash::tree(x.tree());
                println!(
                    "{name} {age} {seed} nodes {} grown {grown:016x} clothed {clothed:016x} ms {:.0}",
                    x.tree().nodes.len(),
                    t.elapsed().as_secs_f64() * 1e3
                );
            }
        }
    }
    Ok(())
}
