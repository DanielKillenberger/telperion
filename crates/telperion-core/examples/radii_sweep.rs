//! fn-281 R1: which shipped trees refuse with "unsolved radii". Grows each
//! `<preset> <ages> <seeds>` through the pipeline's executor and holds the
//! grown tree to `validate_solved`, printing one line a tree and, for a
//! refusal, the nodes whose radius is not above zero.
//!
//!   radii_sweep <preset> <age,age|lo-hi> <seed lo-hi>
use std::time::Instant;
use telperion_core::{pipeline::executor, presets::Preset, tree::Tree};

fn list(s: &str) -> Result<Vec<u32>, String> {
    let mut out = Vec::new();
    for part in s.split(',') {
        let num = |t: &str| t.parse::<u32>().map_err(|e| format!("{t}: {e}"));
        match part.split_once('-') {
            Some((a, b)) => out.extend(num(a)?..=num(b)?),
            None => out.push(num(part)?),
        }
    }
    Ok(out)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, ages, seeds] = &args[..] else {
        return Err("usage: radii_sweep <preset> <ages> <seeds>".into());
    };
    let preset = Preset::from_id(id).ok_or(format!("no preset {id}"))?;
    let native = preset.parameters().age;
    let ages = if ages == "own" {
        vec![native as u32]
    } else {
        list(ages)?
    };
    for age in ages {
        for seed in list(seeds)? {
            let mut family = preset.parameters();
            family.age = f64::from(age);
            family.skeleton.seed = seed;
            let started = Instant::now();
            let verdict = executor::grow(&family).and_then(|g| g.expansion());
            let ms = started.elapsed().as_millis();
            match verdict {
                Err(e) => println!("{id} age {age} seed {seed}: REFUSED {e} ({ms} ms)"),
                Ok(x) => report(id, age, seed, x.tree(), ms),
            }
        }
    }
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let peak = status
        .lines()
        .find(|l| l.starts_with("VmHWM"))
        .unwrap_or("VmHWM: ?");
    println!("peak {}", peak.trim_start_matches("VmHWM:").trim());
    Ok(())
}

/// The tree's nodes as bits, so two builds compare to the byte, and how many
/// are drawn above zero but thinner than the conversion's `FINEST` (2e-4 m).
fn digest(tree: &Tree) -> (u64, usize) {
    use std::hash::{Hash, Hasher};
    let mut h = std::collections::hash_map::DefaultHasher::new();
    for n in &tree.nodes {
        let p = n.position;
        for v in [p.x, p.y, p.z, n.radius, n.start_radius, n.base_radius] {
            v.to_bits().hash(&mut h);
        }
        (n.parent, n.branch, n.stem, n.identity).hash(&mut h);
    }
    let fine = (tree.nodes.iter())
        .filter(|n| n.radius > 0.0 && n.radius < 2e-4)
        .count();
    (h.finish(), fine)
}

fn report(id: &str, age: u32, seed: u32, tree: &Tree, ms: u128) {
    let n = tree.nodes.len();
    let (d, fine) = digest(tree);
    match tree.validate_solved() {
        Ok(()) => println!(
            "{id} age {age} seed {seed}: ok {n} nodes, digest {d:016x}, {fine} under FINEST ({ms} ms)"
        ),
        Err(e) => {
            let bad: Vec<usize> = (0..n).filter(|&i| tree.nodes[i].radius <= 0.0).collect();
            println!(
                "{id} age {age} seed {seed}: REFUSED {e} ({ms} ms); {n} nodes, {} unsolved",
                bad.len()
            );
            for &i in bad.iter().take(8) {
                let d = &tree.nodes[i];
                let p = d.parent.map(|p| p as usize);
                let pr = p.map_or(f64::NAN, |p| tree.nodes[p].radius);
                println!(
                    "  node {i}: radius {:e} start {:e} base {:e} parent {p:?} parent radius {pr:e} \
                     branch {} stem {} fate {:?} codominant {:?} kind {:?}",
                    d.radius, d.start_radius, d.base_radius, d.branch, d.stem, d.shoot.bud_fate,
                    d.codominant, d.kind
                );
            }
        }
    }
}
