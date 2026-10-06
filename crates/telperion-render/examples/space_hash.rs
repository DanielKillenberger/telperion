//! fn-198 task 2's proof: each engine preset's grown tree through the one
//! pipeline (`executor::grow`, then the expansion's clothed tree), hashed as
//! `space/hash.rs` hashes it, for the four species at ages 40 and 80 and
//! seeds 1 and 7, against the handed-in path's hashes recorded before it
//! closed (`.flow/evidence/fn-198-*/HASHES.md`). Run with a species' short
//! name to hash that species alone.
#[path = "space/hash.rs"]
mod hash;
use telperion_core::{pipeline::executor, presets::Preset};

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    for (name, preset) in [
        ("beech", Preset::EuropeanBeech),
        ("spruce", Preset::NorwaySpruce),
        ("oak", Preset::OregonWhiteOak),
        ("palm", Preset::DatePalm),
    ] {
        if !args.is_empty() && !args.iter().any(|a| a == name) {
            continue;
        }
        for age in [40.0, 80.0] {
            for seed in [1, 7] {
                let mut family = preset.parameters();
                (family.age, family.skeleton.seed) = (age, seed);
                let t = std::time::Instant::now();
                let x = executor::grow(&family)
                    .and_then(|g| g.expansion())
                    .map_err(|e| e.to_string())?;
                let clothed = hash::tree(x.tree());
                println!(
                    "{name} {age} {seed} nodes {} clothed {clothed:016x} ms {:.0}",
                    x.tree().nodes.len(),
                    t.elapsed().as_secs_f64() * 1e3
                );
            }
        }
    }
    Ok(())
}
