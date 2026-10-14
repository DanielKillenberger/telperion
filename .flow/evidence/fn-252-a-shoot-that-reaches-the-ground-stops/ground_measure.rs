//! fn-252 R1: a preset's engine tree at its ages and seeds, read for its
//! crown base, its lowest living branches (their PA, age, light and how
//! low their wood hangs), the wood that rests on the ground, and, with
//! `--leaves`, where the leaves stand against the hanging strands' wood.
//! Measure only; copied into `crates/telperion-core/examples/` to run,
//! never committed there.
//!
//!   cargo run --profile ci -p telperion-core --example ground_measure -- \
//!     <preset> <ages,..> <seeds,..> [--leaves <strand pa>,<short pa>] [--no-light]
use std::time::Instant;
use telperion_core::{pipeline, presets::Preset};
use telperion_space::{bud_light, grow, Origin, Request, Structure};

/// Wood whose node stands this low rests on the ground (m): the lay
/// clamps a resting step to exactly 0.
const ON: f64 = 1e-3;
/// Wood this low is at the ground in a still (m).
const NEAR: f64 = 0.05;
/// Height bands for the leaves (m).
const BANDS: [f64; 9] = [0.0, 0.25, 0.5, 1.0, 2.0, 4.0, 8.0, 16.0, f64::INFINITY];

struct Read {
    stem: Vec<bool>,
    order: Vec<u32>,
    /// The first-order branch an axis belongs to (itself if first-order).
    first: Vec<Option<usize>>,
    /// Whether any apex in the axis's subtree lives.
    alive: Vec<bool>,
    /// The lowest node in the axis's subtree (m).
    low: Vec<f64>,
}

fn read(s: &Structure, trunk: &[usize]) -> Read {
    let n = s.axes.len();
    let (mut stem, mut order, mut first) = (vec![false; n], vec![0u32; n], vec![None; n]);
    for (i, a) in s.axes.iter().enumerate() {
        match a.origin {
            Origin::Seed => stem[i] = true,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                stem[i] = stem[parent] && trunk.contains(&a.pa);
                order[i] = order[parent];
                first[i] = first[parent];
                if !stem[i] && stem[parent] {
                    // A stem age carried on into a branch age: a first-order branch.
                    (order[i], first[i]) = (1, Some(i));
                }
            }
            Origin::Lateral { parent, .. } => {
                order[i] = order[parent] + 1;
                first[i] = if stem[parent] { Some(i) } else { first[parent] };
            }
        }
    }
    let mut alive: Vec<bool> = s.axes.iter().map(|a| a.apex_end.is_none()).collect();
    let mut low: Vec<f64> = s
        .axes
        .iter()
        .map(|a| a.phytomers.iter().map(|p| p.tip.z).fold(a.base.z, f64::min))
        .collect();
    for i in (0..n).rev() {
        if let Some(p) = s.axes[i].origin.parent() {
            alive[p] |= alive[i];
            low[p] = low[p].min(low[i]);
        }
    }
    Read { stem, order, first, alive, low }
}

fn quart(mut v: Vec<f64>) -> String {
    if v.is_empty() {
        return "-".into();
    }
    v.sort_by(f64::total_cmp);
    let q = |p: f64| v[((v.len() - 1) as f64 * p).round() as usize];
    format!("{:.2}/{:.2}/{:.2}", q(0.1), q(0.5), q(0.9))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, ages, seeds, rest @ ..] = &args[..] else {
        return Err("usage: ground_measure <preset> <ages> <seeds> [--leaves s,t] [--no-light]".into());
    };
    let list = |t: &str| -> Vec<u32> { t.split(',').map(|v| v.parse().expect("number")).collect() };
    let leaves_pas: Option<(usize, usize)> = rest.iter().position(|w| w == "--leaves").map(|k| {
        let v = list(&rest[k + 1]);
        (v[0] as usize, v[1] as usize)
    });
    let lit = !rest.iter().any(|w| w == "--no-light");
    for age in list(ages) {
        for seed in list(seeds) {
            let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
            family.age = f64::from(age);
            family.skeleton.seed = seed;
            let species = family.space.species.clone().ok_or("no engine species")?;
            let request = Request {
                age,
                seed: u64::from(seed),
                budget: 20_000_000,
                light: family.space.light,
            };
            let t = Instant::now();
            let s = grow(&species, request).map_err(|e| e.to_string())?;
            let grown = t.elapsed().as_secs_f64();
            let r = read(&s, &family.space.trunk);
            let phyto: usize = s.axes.iter().map(|a| a.phytomers.len()).sum();
            let height = s.axes.iter().flat_map(|a| a.phytomers.iter()).map(|p| p.tip.z).fold(0.0, f64::max);
            println!("== {id} age {age} seed {seed}: {} axes, {phyto} phytomers, height {height:.2} m, grown {grown:.2} s", s.axes.len());
            let light: Vec<Option<f64>> = if lit {
                let mut v = vec![None; s.axes.len()];
                for (i, l) in bud_light(&s, &species, &family.space.light) {
                    v[i] = Some(l);
                }
                v
            } else {
                vec![None; s.axes.len()]
            };
            // A PA that bears nothing (a short shoot) is not a branch.
            let bare: Vec<bool> = species
                .states
                .iter()
                .map(|st| st.zones.iter().all(|z| z.lateral.iter().chain(&z.dormant).all(|&p| p == 0.0)))
                .collect();
            crown(&s, &r, &light, age, &bare);
            ground(&s, &r, age);
            if let Some(pas) = leaves_pas {
                leaves(&family, &s, pas)?;
            }
        }
    }
    Ok(())
}

/// The crown base and the lowest living first-order branches.
fn crown(s: &Structure, r: &Read, light: &[Option<f64>], age: u32, bare: &[bool]) {
    let all: Vec<f64> = light.iter().flatten().copied().collect();
    let mut firsts: Vec<usize> = (0..s.axes.len())
        .filter(|&i| r.first[i] == Some(i) && !bare[s.axes[i].pa])
        .collect();
    firsts.sort_by(|&a, &b| s.axes[a].base.z.total_cmp(&s.axes[b].base.z));
    let living: Vec<usize> = firsts.iter().copied().filter(|&i| r.alive[i]).collect();
    let dead = firsts.len() - living.len();
    // Light per first-order branch: its subtree's living apices.
    let mut sum = vec![(0.0, 0u32); s.axes.len()];
    for (i, l) in light.iter().enumerate() {
        if let (Some(l), Some(f)) = (l, r.first[i]) {
            sum[f].0 += l;
            sum[f].1 += 1;
        }
    }
    let base = living.first().map_or(f64::NAN, |&i| s.axes[i].base.z);
    let tenth = living.get(living.len() / 10).map_or(f64::NAN, |&i| s.axes[i].base.z);
    println!(
        "  crown base (lowest living 1st-order branch, short shoots aside) {base:.2} m; p10 of living bases {tenth:.2} m; living 1st-order {} , dead unshed {dead}; apex light tree p10/p50/p90 {}",
        living.len(),
        quart(all)
    );
    for &i in living.iter().take(6) {
        let a = &s.axes[i];
        let (l, k) = sum[i];
        println!(
            "    low branch: base {:.2} m, pa {}, age {} y, lowest wood {:.3} m, living apices {k}, their light {}",
            a.base.z,
            a.pa,
            age.saturating_sub(a.birth),
            r.low[i],
            if k > 0 { format!("{:.3}", l / f64::from(k)) } else { "-".into() }
        );
    }
}

/// The wood that rests on (and nears) the ground.
fn ground(s: &Structure, r: &Read, age: u32) {
    let (mut axes, mut near_axes, mut on_len, mut alive_tips) = (0, 0, 0.0, 0);
    let mut by_pa: std::collections::BTreeMap<usize, (u32, f64)> = Default::default();
    let mut from: Vec<f64> = Vec::new();
    let (mut orders, mut reach, mut runs) = (Vec::new(), 0.0f64, Vec::new());
    let mut first_hits: std::collections::BTreeSet<usize> = Default::default();
    for (i, a) in s.axes.iter().enumerate() {
        if r.stem[i] {
            continue;
        }
        let mut prev = a.base;
        let (mut run, mut touches, mut near) = (0.0, false, false);
        for p in &a.phytomers {
            let d = ((p.tip.x - prev.x).powi(2) + (p.tip.y - prev.y).powi(2) + (p.tip.z - prev.z).powi(2)).sqrt();
            if p.tip.z < ON {
                touches = true;
                reach = reach.max(p.tip.x.hypot(p.tip.y));
                if prev.z < ON {
                    run += d;
                }
            }
            near |= p.tip.z < NEAR;
            prev = p.tip;
        }
        near_axes += u32::from(near);
        if touches {
            axes += 1;
            on_len += run;
            runs.push(run);
            alive_tips += u32::from(a.apex_end.is_none() && a.phytomers.last().is_some_and(|p| p.tip.z < ON));
            let e = by_pa.entry(a.pa).or_default();
            (e.0, e.1) = (e.0 + 1, e.1 + run);
            orders.push(f64::from(r.order[i]));
            if let Some(f) = r.first[i] {
                if first_hits.insert(f) {
                    from.push(s.axes[f].base.z);
                }
            }
        }
    }
    let _ = age;
    println!(
        "  ground: {axes} axes rest on it ({near_axes} within {NEAR} m), {on_len:.2} m of wood lying on it, {alive_tips} living apices on it, farthest {reach:.2} m out"
    );
    if axes > 0 {
        let pas: Vec<String> = by_pa.iter().map(|(pa, (n, m))| format!("pa{pa}:{n} ax/{m:.2} m")).collect();
        println!(
            "    by pa {}; order p10/p50/p90 {}; run per axis p10/p50/p90 {} m; from {} first-order branches based at p10/p50/p90 {} m",
            pas.join(" "),
            quart(orders),
            quart(runs),
            from.len(),
            quart(from)
        );
    }
}

/// Leaves and hanging-strand wood by height band.
fn leaves(family: &telperion_core::Family, s: &Structure, (strand, short): (usize, usize)) -> Result<(), String> {
    let request = pipeline::Request {
        leaves: true,
        ..Default::default()
    };
    let built = pipeline::build(family, request).map_err(|e| e.to_string())?;
    let tree = &built.skeleton.tree;
    let root = tree.nodes.first().map_or(0.0, |n| n.radius);
    let leaves = built.outputs.leaves.as_ref().ok_or("no leaves")?;
    let n = leaves.instances.leaves.len();
    // The core tree is y-up.
    let band = |z: f64| BANDS.windows(2).position(|w| z >= w[0] && z < w[1]).unwrap_or(0);
    let k = BANDS.len() - 1;
    let mut count = vec![0u32; k];
    for i in 0..n {
        count[band(leaves.instances.position(i).y.max(0.0))] += 1;
    }
    let (mut wood, mut thick, mut ages) = (vec![0.0; k], vec![0.0; k], vec![Vec::new(); k]);
    let (mut short_alive, mut short_dead) = (vec![0u32; k], vec![0u32; k]);
    for a in &s.axes {
        if a.pa == strand {
            let mut prev = a.base;
            for p in &a.phytomers {
                let d = ((p.tip.x - prev.x).powi(2) + (p.tip.y - prev.y).powi(2) + (p.tip.z - prev.z).powi(2)).sqrt();
                let b = band(p.tip.z.max(0.0));
                wood[b] += d;
                if p.radius > 0.03 * root {
                    thick[b] += d;
                }
                ages[b].push(f64::from(s.age.saturating_sub(p.cycle)));
                prev = p.tip;
            }
        } else if a.pa == short {
            let b = band(a.base.z.max(0.0));
            if a.apex_end.is_none() {
                short_alive[b] += 1;
            } else {
                short_dead[b] += 1;
            }
        }
    }
    println!("  leaves {n}, root radius {root:.4} m; by band (m): leaves | strand wood m | leaves per strand m | strand wood > 3% root radius (the along-shoot leaf bound, slender = stem radius x shootRadius 0.03) | strand wood age p10/p50/p90 | short shoots living/stopped per strand m");
    for b in 0..k {
        if wood[b] == 0.0 && count[b] == 0 {
            continue;
        }
        let per = |x: f64| if wood[b] > 0.0 { x / wood[b] } else { f64::NAN };
        println!(
            "    {:>5.2}-{:<5.2} {:>8} | {:>8.1} | {:>6.1} | {:>4.0}% | {} | {:.1}/{:.1}",
            BANDS[b],
            BANDS[b + 1],
            count[b],
            wood[b],
            per(f64::from(count[b])),
            100.0 * per(thick[b]),
            quart(ages[b].clone()),
            per(f64::from(short_alive[b])),
            per(f64::from(short_dead[b]))
        );
    }
    Ok(())
}
