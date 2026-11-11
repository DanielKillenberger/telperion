//! fn-352: one tree of the scoreboard's panel, drawn and measured, or one
//! cost read. Every tree passes through the one pipeline (`executor::grow`,
//! its expansion and mesh), as the space examples grow theirs.
//!
//!   probe tree <preset> <age|preset> <seed> <out prefix> [shot,...]
//!       stills `<out prefix>-<shot>.png` and one `TREE {json}` line; no
//!       shots (or `none`) measures without a GPU.
//!   probe cost <preset> <age|preset> <seed>
//!       grows and dresses the tree, prints `COST {json}` (seconds).
//!   probe many <preset> <age> <first seed> <count>
//!       that many distinct trees across the cores, `MANY {json}`.
mod drawn;
mod engine;
mod stills;

use std::time::Instant;
use telperion_core::{pipeline::executor, presets::Preset, Family};

fn family(preset: &str, age: &str, seed: u32) -> Result<Family, String> {
    let mut f = Preset::from_id(preset)
        .ok_or(format!("no preset {preset}"))?
        .parameters();
    if age != "preset" {
        f.age = age.parse::<f64>().map_err(|e| format!("age {age}: {e}"))?;
    }
    f.skeleton.seed = seed;
    Ok(f)
}

/// The tree the pipeline draws: grown, expanded and dressed.
fn dressed(f: &Family) -> Result<(telperion_core::mesh::TreeMesh, usize, f64, drawn::Wood), String> {
    let started = Instant::now();
    let grown = executor::grow(f).map_err(|e| e.to_string())?;
    let x = grown.expansion().map_err(|e| e.to_string())?;
    let nodes = x.tree().nodes.len();
    let wood = drawn::wood(x.tree());
    let mesh = x.mesh().map_err(|e| e.to_string())?;
    Ok((mesh, nodes, started.elapsed().as_secs_f64(), wood))
}

fn tree(args: &[String]) -> Result<(), String> {
    let [preset, age, seed, out, rest @ ..] = args else {
        return Err("tree <preset> <age|preset> <seed> <out prefix> [shot,...]".into());
    };
    let seed: u32 = seed.parse().map_err(|e| format!("seed {seed}: {e}"))?;
    let f = family(preset, age, seed)?;
    let shots: Vec<&str> = rest
        .first()
        .map_or(vec![], |s| s.split(',').filter(|s| *s != "none").collect());
    let head = format!(
        "\"preset\":\"{preset}\",\"age\":{},\"seed\":{seed}",
        f.age
    );
    let (mesh, nodes, secs, wood) = match dressed(&f) {
        Ok(d) => d,
        Err(e) => {
            println!("TREE {{{head},\"refused\":{:?}}}", e);
            return Ok(());
        }
    };
    if !shots.is_empty() {
        stills::shoot(&mesh, &f, out, &shots, wood.limbs.joint)?;
    }
    let leaves = drawn::leaves(&mesh);
    let b = mesh.bounds;
    let (height, sx, sz) = (b.max.y - b.min.y, b.max.x - b.min.x, b.max.z - b.min.z);
    drop(mesh);
    let e = engine::measures(&f)?;
    println!(
        "TREE {{{head},\"height\":{height:.3},\"spread_x\":{sx:.3},\"spread_z\":{sz:.3},\
         \"dbh\":{},\"nodes\":{nodes},\"leaves\":{},\"leaf_area\":{:.2},\"crown_area\":{:.2},\
         \"lai\":{:.3},\"seconds\":{secs:.2},\"limbs\":{:?},\"limb_chord\":{},{}}}",
        json(wood.dbh),
        leaves.count,
        leaves.area,
        leaves.crown,
        leaves.area / leaves.crown.max(1e-9),
        wood.limbs.bases.iter().map(|d| (d * 1e4).round() / 1e4).collect::<Vec<_>>(),
        json(wood.limbs.chord),
        e.json()
    );
    Ok(())
}

/// A number, or null where the tree has none (a stem below 1.3 m).
pub fn json(v: Option<f64>) -> String {
    v.map_or("null".into(), |v| format!("{v:.4}"))
}

fn cost(args: &[String]) -> Result<(), String> {
    let [preset, age, seed] = args else {
        return Err("cost <preset> <age|preset> <seed>".into());
    };
    let seed: u32 = seed.parse().map_err(|e| format!("seed {seed}: {e}"))?;
    let f = family(preset, age, seed)?;
    match dressed(&f) {
        Ok((mesh, nodes, secs, _)) => println!(
            "COST {{\"preset\":\"{preset}\",\"age\":{},\"seed\":{seed},\"seconds\":{secs:.3},\"nodes\":{nodes},\"leaves\":{}}}",
            f.age,
            mesh.foliage.instances.len()
        ),
        Err(e) => println!("COST {{\"preset\":\"{preset}\",\"age\":{},\"seed\":{seed},\"refused\":{e:?}}}", f.age),
    }
    Ok(())
}

fn many(args: &[String]) -> Result<(), String> {
    use rayon::prelude::*;
    let [preset, age, first, count] = args else {
        return Err("many <preset> <age> <first seed> <count>".into());
    };
    let first: u32 = first.parse().map_err(|e| format!("{e}"))?;
    let count: u32 = count.parse().map_err(|e| format!("{e}"))?;
    let started = Instant::now();
    let results: Vec<Result<usize, String>> = (first..first + count)
        .into_par_iter()
        .map(|seed| {
            let f = family(preset, age, seed)?;
            dressed(&f).map(|(mesh, ..)| mesh.foliage.instances.len())
        })
        .collect();
    let refused = results.iter().filter(|r| r.is_err()).count();
    println!(
        "MANY {{\"preset\":\"{preset}\",\"age\":{age},\"first\":{first},\"count\":{count},\"seconds\":{:.2},\"refused\":{refused},\"threads\":{}}}",
        started.elapsed().as_secs_f64(),
        rayon::current_num_threads()
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("tree") => tree(&args[1..]),
        Some("cost") => cost(&args[1..]),
        Some("many") => many(&args[1..]),
        _ => Err("probe tree|cost|many ...".into()),
    }
}
