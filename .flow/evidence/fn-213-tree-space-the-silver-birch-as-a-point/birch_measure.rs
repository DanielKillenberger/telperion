//! fn-213 COMPARE.md's measures, by code: one preset's tree through the
//! pipeline at an age and seed, and the hanging strands, the twigs along
//! the limbs, the fine wood and where the leaves sit. Compiles against
//! master (the old generator) and the fn-213 branch (the engine) alike;
//! copied into `crates/telperion-core/examples/` to run, never committed
//! there.
//!
//!   cargo run --profile ci -p telperion-core --example birch_measure -- <preset> <age> <seed>
use std::collections::HashMap;
use telperion_core::{pipeline, presets::Preset};

/// Wood thinner than this, in metres of radius, is fine wood.
const FINE: f64 = 0.005;
/// A run hangs when its chord falls this far below the horizontal (radians).
const HANG: f64 = 0.52;
/// A run whose base radius is at least this is a limb, in metres.
const LIMB: f64 = 0.02;
/// A run shorter than this, in metres, is a short shoot.
const SHORT: f64 = 0.05;

struct Run {
    nodes: Vec<usize>,
    length: f64,
    droop: f64,
    base_radius: f64,
    children: usize,
}

fn quantiles(mut v: Vec<f64>) -> String {
    if v.is_empty() {
        return "none".into();
    }
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    format!(
        "n {} p10 {:.3} p50 {:.3} p90 {:.3} max {:.3}",
        v.len(),
        q(0.1),
        q(0.5),
        q(0.9),
        v[v.len() - 1]
    )
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, age, seed] = &args[..] else {
        return Err("usage: birch_measure <preset> <age> <seed>".into());
    };
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.age = age.parse().map_err(|e| format!("{e}"))?;
    family.skeleton.seed = seed.parse().map_err(|e| format!("{e}"))?;
    let built = pipeline::build(&family, pipeline::Request::mesh()).map_err(|e| e.to_string())?;
    let nodes = &built.skeleton.tree.nodes;
    let seg = |i: usize| -> Option<(f64, f64, f64, f64)> {
        let p = nodes[i].parent? as usize;
        let d = nodes[i].position - nodes[p].position;
        Some((d.x, d.y, d.z, d.length()))
    };
    // Runs: nodes sharing a branch id, in storage order.
    let mut by: HashMap<u32, Vec<usize>> = HashMap::new();
    for i in 1..nodes.len() {
        by.entry(nodes[i].branch).or_default().push(i);
    }
    let mut runs: Vec<Run> = by
        .values()
        .map(|ids| {
            let (mut len, mut dx, mut dy, mut dz) = (0.0, 0.0, 0.0, 0.0);
            for &i in ids {
                if let Some((x, y, z, l)) = seg(i) {
                    (dx, dy, dz, len) = (dx + x, dy + y, dz + z, len + l);
                }
            }
            let first = ids[0];
            let base = nodes[first].radius;
            Run {
                nodes: ids.clone(),
                length: len,
                droop: (-dy).atan2(dx.hypot(dz)),
                base_radius: base,
                children: 0,
            }
        })
        .collect();
    // Children per run: runs whose first node's parent sits in it.
    let mut run_of = vec![usize::MAX; nodes.len()];
    for (r, run) in runs.iter().enumerate() {
        for &i in &run.nodes {
            run_of[i] = r;
        }
    }
    for r in 0..runs.len() {
        if let Some(p) = nodes[runs[r].nodes[0]].parent {
            let owner = run_of[p as usize];
            if owner != usize::MAX && owner != r {
                runs[owner].children += 1;
            }
        }
    }
    let fine_run = |r: &Run| r.base_radius < FINE;
    let strands: Vec<&Run> = runs
        .iter()
        .filter(|r| fine_run(r) && r.droop > HANG && r.length >= SHORT)
        .collect();
    let fine_runs = runs.iter().filter(|r| fine_run(r)).count();
    println!(
        "preset {id} age {age} seed {seed}: {} nodes, {} runs",
        nodes.len(),
        runs.len()
    );
    println!(
        "  hanging strands (fine runs >= {SHORT} m whose chord falls > 30 deg): {} of {fine_runs} fine runs",
        strands.len()
    );
    println!(
        "    length m: {}",
        quantiles(strands.iter().map(|r| r.length).collect())
    );
    println!(
        "    droop deg: {}",
        quantiles(strands.iter().map(|r| r.droop.to_degrees()).collect())
    );
    println!(
        "  all fine runs >= {SHORT} m, droop deg: {}",
        quantiles(
            runs.iter()
                .filter(|r| fine_run(r) && r.length >= SHORT)
                .map(|r| r.droop.to_degrees())
                .collect()
        )
    );
    // Each strand's leaving angle (its first segment's elevation) and drop
    // rate (the elevation it loses over its first 0.3 m, per metre).
    let elevation = |i: usize| seg(i).map(|(x, y, z, _)| y.atan2(x.hypot(z)).to_degrees());
    let (mut leaving, mut drop) = (Vec::new(), Vec::new());
    for r in &strands {
        let Some(e0) = elevation(r.nodes[0]) else {
            continue;
        };
        leaving.push(e0);
        let mut walked = 0.0;
        for &i in &r.nodes {
            walked += seg(i).map_or(0.0, |s| s.3);
            if walked >= 0.3 {
                if let Some(e1) = elevation(i) {
                    drop.push((e0 - e1) / walked);
                }
                break;
            }
        }
    }
    println!("    leaving elevation deg: {}", quantiles(leaving));
    println!("    drop over first 0.3 m, deg per m: {}", quantiles(drop));
    let limbs: Vec<&Run> = runs
        .iter()
        .filter(|r| r.base_radius >= LIMB && r.length > 1.0)
        .collect();
    println!(
        "  limbs (base radius >= {LIMB} m, > 1 m): {}; laterals per metre: {}",
        limbs.len(),
        quantiles(limbs.iter().map(|r| r.children as f64 / r.length).collect())
    );
    // Fine runs leaving limb wood (radius >= 1 cm), per metre of it.
    let mut limb_m = 0.0;
    for i in 1..nodes.len() {
        if nodes[i].radius >= 0.01 {
            limb_m += seg(i).map_or(0.0, |s| s.3);
        }
    }
    let off_limbs = runs
        .iter()
        .filter(|r| {
            fine_run(r)
                && nodes[r.nodes[0]]
                    .parent
                    .is_some_and(|p| nodes[p as usize].radius >= 0.01)
        })
        .count();
    println!(
        "  wood >= 1 cm radius: {limb_m:.1} m, bearing {off_limbs} fine runs: {:.2} a metre",
        off_limbs as f64 / limb_m.max(1e-9)
    );
    println!(
        "  segment length of fine wood m: {}",
        quantiles(
            (1..nodes.len())
                .filter(|&i| nodes[i].radius < FINE)
                .filter_map(|i| seg(i).map(|s| s.3))
                .collect()
        )
    );
    // Fine wood by radius band: count of segments and metres.
    let bands = [0.0005, 0.001, 0.002, 0.005, 0.01, 0.03, f64::INFINITY];
    let mut count = [0usize; 7];
    let mut metres = [0.0f64; 7];
    for i in 1..nodes.len() {
        if let Some((.., l)) = seg(i) {
            let b = bands.iter().position(|&e| nodes[i].radius < e).unwrap();
            count[b] += 1;
            metres[b] += l;
        }
    }
    let names = [
        "<0.5mm", "0.5-1mm", "1-2mm", "2-5mm", "5-10mm", "10-30mm", ">30mm",
    ];
    for b in 0..7 {
        println!(
            "    radius {:>8}: {:>9} segments, {:>9.1} m",
            names[b], count[b], metres[b]
        );
    }
    // Leaves: the nearest node's run, by a coarse grid.
    let leaves = built.outputs.leaves.ok_or("no leaves")?;
    let n = leaves.instances.leaves.len();
    let cell = 0.25;
    let key = |x: f64, y: f64, z: f64| {
        (
            (x / cell).floor() as i64,
            (y / cell).floor() as i64,
            (z / cell).floor() as i64,
        )
    };
    let mut grid: HashMap<(i64, i64, i64), Vec<usize>> = HashMap::new();
    for i in 1..nodes.len() {
        if nodes[i].radius < 0.03 {
            let p = nodes[i].position;
            grid.entry(key(p.x, p.y, p.z)).or_default().push(i);
        }
    }
    let (mut on_strand, mut on_short, mut on_other, mut unmatched) = (0, 0, 0, 0);
    let strand_set: std::collections::HashSet<usize> = strands.iter().map(|r| r.nodes[0]).collect();
    for k in 0..n {
        let m = leaves.instances.matrix(k);
        let (x, y, z) = (m[12] as f64, m[13] as f64, m[14] as f64);
        let (cx, cy, cz) = key(x, y, z);
        let mut best = (f64::INFINITY, usize::MAX);
        for a in -1..=1 {
            for b in -1..=1 {
                for c in -1..=1 {
                    for &i in grid.get(&(cx + a, cy + b, cz + c)).into_iter().flatten() {
                        let p = nodes[i].position;
                        let d = (p.x - x).powi(2) + (p.y - y).powi(2) + (p.z - z).powi(2);
                        if d < best.0 {
                            best = (d, i);
                        }
                    }
                }
            }
        }
        if best.1 == usize::MAX {
            unmatched += 1;
            continue;
        }
        let run = &runs[run_of[best.1]];
        if strand_set.contains(&run.nodes[0]) {
            on_strand += 1;
        } else if run.length < SHORT {
            on_short += 1;
        } else {
            on_other += 1;
        }
    }
    println!(
        "  leaves {n}: nearest wood on a hanging strand {on_strand}, on a short run (< {SHORT} m) {on_short}, elsewhere {on_other}, none within 0.5 m {unmatched}"
    );
    // Leaves per metre of hanging strand, and per short run.
    let strand_m: f64 = strands.iter().map(|r| r.length).sum();
    let short_runs = runs
        .iter()
        .filter(|r| fine_run(r) && r.length < SHORT)
        .count();
    println!(
        "  leaves per metre of hanging strand: {:.1} ({on_strand} on {strand_m:.0} m); per short run: {:.2} ({short_runs} runs)",
        on_strand as f64 / strand_m.max(1e-9),
        on_short as f64 / (short_runs.max(1) as f64)
    );
    // Leaf size: the blade's extent along and across its axis, and the
    // stalk below it, times each instance's scale (the column length).
    if let Some(element) = built.outputs.element.as_ref() {
        let blade = element
            .anatomy
            .as_ref()
            .map_or(0..element.positions.len(), |a| a.vertices.clone());
        let (mut y0, mut y1, mut x0, mut x1) = (f64::MAX, f64::MIN, f64::MAX, f64::MIN);
        for p in &element.positions[blade] {
            (y0, y1, x0, x1) = (y0.min(p.y), y1.max(p.y), x0.min(p.x), x1.max(p.x));
        }
        let scales: Vec<f64> = (0..n)
            .step_by((n / 20_000).max(1))
            .map(|k| {
                let m = leaves.instances.matrix(k);
                f64::from(m[4])
                    .hypot(f64::from(m[5]))
                    .hypot(f64::from(m[6]))
            })
            .collect();
        let mm = |s: f64| s * 1e3;
        let q = |v: &[f64], f: f64| v[((v.len() - 1) as f64 * f).round() as usize];
        let mut s = scales.clone();
        s.sort_by(f64::total_cmp);
        println!(
            "  leaf blade length mm p10/p50/p90: {:.0} / {:.0} / {:.0}; width p50 {:.0}; stalk p50 {:.0}",
            mm((y1 - y0) * q(&s, 0.1)),
            mm((y1 - y0) * q(&s, 0.5)),
            mm((y1 - y0) * q(&s, 0.9)),
            mm((x1 - x0) * q(&s, 0.5)),
            mm(y0 * q(&s, 0.5))
        );
    }
    let m = &family.material;
    println!(
        "  leaf colour rows (linear): front {:.3}/{:.3}/{:.3}, back {:.3}/{:.3}/{:.3}, transmission {:.3}/{:.3}/{:.3} at {:.2}",
        m.leaf_front_red, m.leaf_front_green, m.leaf_front_blue,
        m.leaf_back_red, m.leaf_back_green, m.leaf_back_blue,
        m.transmission_red, m.transmission_green, m.transmission_blue, m.transmission_strength
    );
    let b = built
        .skeleton
        .tree
        .nodes
        .iter()
        .fold((f64::MAX, f64::MIN), |(lo, hi), n| {
            (lo.min(n.position.y), hi.max(n.position.y))
        });
    println!("  height {:.1} m", b.1 - b.0);
    Ok(())
}
