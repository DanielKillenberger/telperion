//! fn-248 R1: the fine wood a tree's leaves sit on. For one preset at one
//! age and seed, with the supernatural group off (scale 1) and on, prints
//! the wood by branch order and age, the leaves by the drawn radius and the
//! age of the wood they are planned on, and the leaf against its twig.
//!
//!   cargo run --profile ci -p telperion-core --example twig_layer -- <preset> <age> [seed]
use std::collections::HashMap;

use telperion_core::{
    pipeline::{self, Request},
    presets::{Family, Preset},
    tree::Tree,
};

/// Drawn radius classes, in millimetres: the upper bound of each.
const RADII: [f64; 8] = [0.5, 1.0, 2.0, 4.0, 8.0, 16.0, 32.0, f64::INFINITY];
/// Wood age classes, in years: current-year, 1 to 3, older.
const AGES: [&str; 3] = ["0", "1-3", "4+"];

fn age_class(age: u32) -> usize {
    match age {
        0 => 0,
        1..=3 => 1,
        _ => 2,
    }
}

fn radius_class(mm: f64) -> usize {
    RADII.iter().position(|&r| mm <= r).unwrap()
}

/// Each node's branch order: 0 on the trunk's run and every run the root
/// starts, one more than its bearer's on every other run.
fn orders(tree: &Tree) -> Vec<u32> {
    let mut order = vec![0u32; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        order[i] = if n.stem || n.branch == tree.nodes[p].branch {
            order[p]
        } else if p == 0 {
            0
        } else {
            order[p] + 1
        };
    }
    order
}

/// Each node's wood age in years, by the engine phytomer that drew it,
/// regrown at the family's natural size (the scale draws, never grows).
fn ages(family: &Family, tree: &Tree) -> Result<Vec<u32>, String> {
    let applied = family.applied();
    let species = applied.space.species.as_ref().ok_or("no engine species")?;
    let request = telperion_space::Request {
        age: family.age as u32,
        seed: u64::from(family.skeleton.seed),
        budget: 20_000_000,
        light: applied.space.light,
    };
    let (s, _, _) = telperion_space::grow_marked(species, request).map_err(|e| e.to_string())?;
    let cycle: HashMap<u64, u32> = (s.axes.iter())
        .flat_map(|a| a.phytomers.iter().map(|p| (p.key(), p.cycle)))
        .collect();
    Ok((tree.nodes.iter())
        .map(|n| {
            let c = cycle.get(&n.identity.birth_order()).copied();
            c.map_or(u32::MAX, |c| s.age.saturating_sub(c))
        })
        .collect())
}

fn median(v: &mut [f64]) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

fn quantile(v: &mut [f64], q: f64) -> f64 {
    if v.is_empty() {
        return f64::NAN;
    }
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * q).round() as usize]
}

fn measure(family: &Family, label: &str) -> Result<(), String> {
    let request = Request {
        leaves: true,
        field: Some(None),
        ..Request::default()
    };
    let built = pipeline::build(family, request).map_err(|e| e.to_string())?;
    let tree = &built.skeleton.tree;
    let plan = built.outputs.plan.as_ref().ok_or("no leaf plan")?;
    let (order, age) = (orders(tree), ages(family, tree)?);
    let unknown = age.iter().skip(1).filter(|&&a| a == u32::MAX).count();
    let scale = if family.supernatural.enabled {
        family.supernatural.scale
    } else {
        1.0
    };
    println!(
        "\n== {label}: scale {scale}, {} nodes ({unknown} without an age), planned leaves {}",
        tree.nodes.len(),
        plan.total
    );

    // The wood by order and age: nodes, metres grown, median drawn diameter.
    let top = *order.iter().max().unwrap() as usize;
    let mut wood: Vec<[(usize, f64, Vec<f64>); 3]> =
        (0..=top).map(|_| Default::default()).collect();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if age[i] == u32::MAX {
            continue;
        }
        let cell = &mut wood[order[i] as usize][age_class(age[i])];
        cell.0 += 1;
        cell.1 += tree.grown_length(i);
        cell.2.push(2e3 * n.radius);
    }
    println!("wood: order | age | nodes | metres | median diameter mm");
    for (o, row) in wood.iter_mut().enumerate() {
        for (a, cell) in row.iter_mut().enumerate() {
            if cell.0 > 0 {
                let d = median(&mut cell.2);
                println!("  {o} | {} | {} | {:.0} | {d:.2}", AGES[a], cell.0, cell.1);
            }
        }
    }

    // Current-year shoots: each run's wood of age 0, summed per run.
    let mut shoots: HashMap<u32, f64> = HashMap::new();
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        if age[i] == 0 {
            *shoots.entry(n.branch).or_default() += tree.grown_length(i);
        }
    }
    let mut lengths: Vec<f64> = shoots.values().map(|l| l * 1e3).collect();
    println!(
        "current-year shoots: {} runs, length mm p10 {:.0} median {:.0} p90 {:.0}",
        lengths.len(),
        quantile(&mut lengths, 0.1),
        quantile(&mut lengths, 0.5),
        quantile(&mut lengths, 0.9)
    );

    // Leaves by the drawn radius and age of the wood they are planned on.
    let at: HashMap<[u64; 3], usize> = (tree.nodes.iter().enumerate())
        .map(|(i, n)| {
            let p = n.position;
            ([p.x.to_bits(), p.y.to_bits(), p.z.to_bits()], i)
        })
        .collect();
    let mut by = [[0u64; 3]; RADII.len()];
    let (mut seated, mut missing) = (Vec::new(), 0u64);
    let mut bearing = 0.0;
    for d in &plan.descriptors {
        let p = d.endpoints[1];
        let Some(&i) = at.get(&[p.x.to_bits(), p.y.to_bits(), p.z.to_bits()]) else {
            missing += u64::from(d.count);
            continue;
        };
        let mm = 1e3 * 0.5 * (d.radii[0] + d.radii[1]);
        let a = if age[i] == u32::MAX {
            2
        } else {
            age_class(age[i])
        };
        by[radius_class(mm)][a] += u64::from(d.count);
        bearing += d.endpoints[0].distance(d.endpoints[1]);
        seated.extend(std::iter::repeat(2.0 * mm).take(d.count as usize));
    }
    println!("leaves: radius mm <= | age 0 | 1-3 | 4+  ({missing} not matched to a node)");
    for (r, row) in by.iter().enumerate() {
        if row.iter().sum::<u64>() > 0 {
            println!("  {} | {} | {} | {}", RADII[r], row[0], row[1], row[2]);
        }
    }

    // The leaf against its wood.
    let leaf = family.element.length * family.canopy.size;
    let petiole = family.element.connector_length * family.canopy.size;
    let total: f64 = seated.len() as f64;
    let d50 = median(&mut seated);
    println!(
        "leaf blade {:.0} mm, petiole {:.1} mm; seat diameter p10 {:.2} median {d50:.2} p90 {:.2} mm; blade / seat diameter {:.0}; petiole / seat diameter {:.1}",
        leaf * 1e3,
        petiole * 1e3,
        quantile(&mut seated, 0.1),
        quantile(&mut seated, 0.9),
        leaf * 1e3 / d50,
        petiole * 1e3 / d50
    );
    println!(
        "bearing wood {:.0} m (chord), {:.1} leaves per metre, one every {:.1} mm",
        bearing,
        total / bearing,
        1e3 * bearing / total
    );
    Ok(())
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, age, rest @ ..] = &args[..] else {
        return Err("usage: twig_layer <preset> <age> [seed]".into());
    };
    let preset = Preset::from_id(id).ok_or_else(|| format!("unknown tree {id}"))?;
    let age: f64 = age.parse().map_err(|e| format!("age {age}: {e}"))?;
    let seed: u32 = rest
        .first()
        .map_or(Ok(1), |s| s.parse())
        .map_err(|e| format!("seed: {e}"))?;
    for on in [false, true] {
        let mut family = preset.parameters();
        (family.age, family.skeleton.seed) = (age, seed);
        if on && !family.supernatural.enabled {
            continue;
        }
        family.supernatural.enabled = on;
        measure(
            &family,
            &format!(
                "{id} {age} seed {seed} group {}",
                if on { "on" } else { "off" }
            ),
        )?;
    }
    Ok(())
}
