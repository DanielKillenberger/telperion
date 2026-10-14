//! fn-252 R2: every shipped preset's engine tree, grown under its own
//! light, hashed over every public field (as `telperion-space`'s
//! `measures hash` hashes it), so a base and a branch can be compared.
//! Measure only; copied into `crates/telperion-core/examples/` to run.
//!
//!   cargo run --profile ci -p telperion-core --example space_hash_all -- <ages,..> <seeds,..>
use std::hash::{DefaultHasher, Hash, Hasher};
use telperion_core::presets::Preset;
use telperion_space::{grow, Request, Structure, Vec3};

fn hash(tree: &Structure) -> u64 {
    let mut h = DefaultHasher::new();
    let v = |h: &mut DefaultHasher, v: Vec3| [v.x, v.y, v.z].map(|c| c.to_bits()).hash(h);
    (tree.age, tree.pas).hash(&mut h);
    for axis in &tree.axes {
        (axis.lineage, axis.pa, axis.birth, axis.apex_end).hash(&mut h);
        format!("{:?}", axis.origin).hash(&mut h);
        axis.vigour.to_bits().hash(&mut h);
        v(&mut h, axis.base);
        v(&mut h, axis.heading);
        v(&mut h, axis.side);
        for p in &axis.phytomers {
            (p.cycle, p.radius.to_bits(), p.scale.to_bits()).hash(&mut h);
            v(&mut h, p.tip);
            v(&mut h, p.heading);
            v(&mut h, p.side);
        }
    }
    h.finish()
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |t: &str| -> Vec<u32> { t.split(',').map(|v| v.parse().expect("number")).collect() };
    let (ages, seeds) = (list(&args[0]), list(&args[1]));
    for id in ["ordinary", "oregon-white-oak", "norway-spruce", "european-beech", "date-palm", "silver-birch", "laurelin", "telperion"] {
        let family = Preset::from_id(id).ok_or("no preset")?.parameters();
        let species = family.space.species.clone().ok_or("no engine species")?;
        for &age in &ages {
            for &seed in &seeds {
                let r = Request { age, seed: u64::from(seed), budget: 20_000_000, light: family.space.light };
                let tree = grow(&species, r).map_err(|e| e.to_string())?;
                println!("{id} age {age} seed {seed}: {:016x} {} phytomers", hash(&tree), tree.phytomer_count());
            }
        }
    }
    Ok(())
}
