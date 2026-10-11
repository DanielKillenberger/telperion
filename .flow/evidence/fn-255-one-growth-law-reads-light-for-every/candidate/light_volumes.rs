//! fn-255 R3's measures (DESIGN.md section 4): the foliage-volume light
//! candidate against today's lattice, through the one cycle.
//!
//! - `frozen <preset> <ages> <seeds>`: every candidate read beside the
//!   lattice on the same leaves and buds every cycle, the lattice driving
//!   growth: light error per bud, nodes visited, build and read seconds.
//! - `grown <preset> <ages> <seeds> <leaf>`: the tree grown on each model,
//!   and today's build, side by side.
//! - `frame <from> <to> <age> <seed> <t>`: one walk frame (fn-251's
//!   repro) on today's build and through the one cycle.
//! - `cost <age|default> <seed>`: every shipped preset through the one
//!   cycle under the lattice against today's build.
//!
//!   cargo run --profile ci -p telperion-core --example light_volumes -- frozen oregon-white-oak 40 1
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_space::{grow_cycle, grow_marked, Candidate, Measure, Model, Request, Species, Structure};

const PRESETS: [(Preset, &str); 8] = [
    (Preset::EuropeanBeech, "european-beech"),
    (Preset::OregonWhiteOak, "oregon-white-oak"),
    (Preset::NorwaySpruce, "norway-spruce"),
    (Preset::DatePalm, "date-palm"),
    (Preset::SilverBirch, "silver-birch"),
    (Preset::Ordinary, "ordinary"),
    (Preset::Laurelin, "laurelin"),
    (Preset::Telperion, "telperion"),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u64> {
        let text = args.get(i).map(String::as_str).unwrap_or("1");
        text.split(',').map(|v| v.parse().expect("a number")).collect()
    };
    match args.first().map(String::as_str) {
        Some("frozen") => frozen(&args[1], &list(2), &list(3), args.get(4)),
        Some("grown") => grown(&args[1], &list(2), &list(3), args[4].parse().expect("a leaf multiplier")),
        Some("frame") => frame(&args[1], &args[2], list(3)[0], list(4)[0], args[5].parse().expect("a t")),
        Some("cost") => cost(args.get(1).map_or("default", String::as_str), list(2)[0]),
        _ => panic!("frozen, grown or cost"),
    }
}

fn preset(name: &str) -> (Species, telperion_space::Light, f64) {
    let (p, _) = PRESETS.iter().find(|p| p.1 == name).expect("a shipped preset");
    let family = p.parameters();
    let space = family.space;
    (space.species.expect("an engine species"), space.light, family.age)
}

fn request(age: u64, seed: u64, light: telperion_space::Light) -> Request {
    Request {
        age: age as u32,
        seed,
        budget: 20_000_000,
        light,
    }
}

/// Every candidate given as `leaf:near:far`, a leaf of `lattice` reaching
/// the lattice's half-metre cell; by default four.
fn frozen(name: &str, ages: &[u64], seeds: &[u64], given: Option<&String>) {
    let (species, light, _) = preset(name);
    let leaf = species.states.iter().map(|s| s.leaf_area).fold(0.0, f64::max).sqrt();
    let given = given.map_or("1:1.5:3,4:1.5:3,lattice:1.5:3,1:3:6", String::as_str);
    let candidates: Vec<Candidate> = (given.split(','))
        .map(|c| {
            let v: Vec<&str> = c.split(':').collect();
            let leaf = if v[0] == "lattice" { 0.5 / leaf } else { v[0].parse().unwrap() };
            Candidate { leaf, near: v[1].parse().unwrap(), far: v[2].parse().unwrap() }
        })
        .collect();
    for &age in ages {
        for &seed in seeds {
            let cycled = grow_cycle(&species, request(age, seed, light), Model::Lattice, &candidates).unwrap();
            let probe = cycled.probe.expect("a probe");
            println!(
                "\n{name} age {age} seed {seed}: grown {}, built {:.1} s, lattice {:.2} s",
                cycled.grown, cycled.seconds, probe.lattice_seconds
            );
            for m in &probe.measures {
                report(m);
            }
        }
    }
}

/// The q-th quantile of `values`, sorted in place.
fn quantile(values: &mut [f64], q: f64) -> f64 {
    if values.is_empty() {
        return f64::NAN;
    }
    values.sort_by(f64::total_cmp);
    values[((values.len() - 1) as f64 * q).round() as usize]
}

fn report(m: &Measure) {
    let c = m.candidate;
    let n = m.buds as f64;
    let mut abs: Vec<f64> = m.sample.iter().map(|s| f64::from((s[1] - s[0]).abs())).collect();
    let mut visits: Vec<f64> = m.sample.iter().map(|s| f64::from(s[2])).collect();
    println!(
        "  leaf {:.2} near {} far {}: buds {} | error signed {:+.4} abs {:.4} p50 {:.4} p90 {:.4} p99 {:.4} max {:.4} | visited mean {:.1} p50 {:.0} p99 {:.0} max {} | build {:.2} s read {:.2} s",
        c.leaf, c.near, c.far, m.buds, m.sum / n, m.sum_abs / n,
        quantile(&mut abs, 0.5), quantile(&mut abs, 0.9), quantile(&mut abs, 0.99), m.max_abs,
        m.visited as f64 / n, quantile(&mut visits, 0.5), quantile(&mut visits, 0.99), m.visited_max,
        m.build, m.read,
    );
    let mut line = String::from("    by the lattice's light:");
    for (low, high) in [(0.0, 0.25), (0.25, 0.5), (0.5, 0.75), (0.75, 1.01)] {
        let bin: Vec<f64> = (m.sample.iter())
            .filter(|s| f64::from(s[0]) >= low && f64::from(s[0]) < high)
            .map(|s| f64::from(s[1] - s[0]))
            .collect();
        let k = bin.len().max(1) as f64;
        let (signed, size) = (bin.iter().sum::<f64>() / k, bin.iter().map(|d| d.abs()).sum::<f64>() / k);
        line += &format!(" [{low:.2},{high:.2}) n {} {signed:+.4}/{size:.4}", bin.len());
    }
    println!("{line}");
    let k = m.exhaustive.len().max(1) as f64;
    let sum = |f: &dyn Fn(&[f32; 3]) -> f64| m.exhaustive.iter().map(f).sum::<f64>() / k;
    let most = |f: &dyn Fn(&[f32; 3]) -> f64| m.exhaustive.iter().map(f).fold(0.0, f64::max);
    let hierarchy = |e: &[f32; 3]| f64::from((e[1] - e[2]).abs());
    println!(
        "    on {} buds read exhaustively: hierarchy error mean {:.4} max {:.4}; kernels less lattice signed {:+.4} abs {:.4}",
        m.exhaustive.len(), sum(&hierarchy), most(&hierarchy),
        sum(&|e| f64::from(e[2] - e[0])), sum(&|e| f64::from((e[2] - e[0]).abs())),
    );
    let worst = m.cycles.iter().enumerate().max_by(|a, b| a.1[3].total_cmp(&b.1[3]));
    if let Some((cycle, w)) = worst {
        println!(
            "    most visits in cycle {}: {} buds, {} nodes, mean {:.1}, most {}",
            cycle + 1, w[0], w[4], w[2], w[3]
        );
    }
}

/// Kept phytomers, height, crown spans (x, y) and the live crown's base.
fn shape(s: &Structure) -> (usize, f64, f64, f64, f64) {
    let base = s.axes[0].base;
    let (mut low, mut high) = ([f64::MAX; 3], [f64::MIN; 3]);
    let mut crown = f64::MAX;
    for a in &s.axes {
        for p in &a.phytomers {
            for (k, v) in [p.tip.x, p.tip.y, p.tip.z].into_iter().enumerate() {
                (low[k], high[k]) = (low[k].min(v), high[k].max(v));
            }
            if p.cycle == s.age && a.origin.parent().is_some() {
                crown = crown.min(p.tip.z - base.z);
            }
        }
    }
    (s.phytomer_count(), high[2] - base.z, high[0] - low[0], high[1] - low[1], crown)
}

fn line(label: &str, s: &Structure, grown: u64, seconds: f64, light: f64) {
    let (kept, height, x, y, crown) = shape(s);
    let shed = 100.0 * (1.0 - kept as f64 / grown as f64);
    println!(
        "  {label:<22} kept {kept:>9} grown {grown:>9} shed {shed:5.1}% height {height:6.2} crown {x:6.2} x {y:6.2} base {crown:5.2} | {seconds:6.2} s (light {light:.2} s)"
    );
}

fn grown(name: &str, ages: &[u64], seeds: &[u64], leaf: f64) {
    let (species, light, _) = preset(name);
    let volumes = Candidate { leaf, ..Candidate::default() };
    for &age in ages {
        for &seed in seeds {
            let r = request(age, seed, light);
            println!("\n{name} age {age} seed {seed}");
            let started = Instant::now();
            let (today, ..) = grow_marked(&species, r).unwrap();
            let seconds = started.elapsed().as_secs_f64();
            let lattice = grow_cycle(&species, r, Model::Lattice, &[]).unwrap();
            line("today (F4 build)", &today, lattice.grown, seconds, f64::NAN);
            line("cycle, lattice", &lattice.structure, lattice.grown, lattice.seconds, lattice.light_seconds);
            drop(lattice);
            let cand = grow_cycle(&species, r, Model::Volumes(volumes), &[]).unwrap();
            line(&format!("cycle, volumes {leaf}"), &cand.structure, cand.grown, cand.seconds, cand.light_seconds);
        }
    }
}

fn cost(age: &str, seed: u64) {
    for (_, name) in PRESETS {
        let (species, light, default) = preset(name);
        let age = age.parse().unwrap_or(default as u64);
        let r = request(age, seed, light);
        println!("\n{name} age {age} seed {seed}");
        let started = Instant::now();
        let today = grow_marked(&species, r).map(|t| t.0);
        let seconds = started.elapsed().as_secs_f64();
        match grow_cycle(&species, r, Model::Lattice, &[]) {
            Ok(c) => {
                match &today {
                    Ok(t) => line("today (F4 build)", t, c.grown, seconds, f64::NAN),
                    Err(e) => println!("  today (F4 build): {e}"),
                }
                line("cycle, lattice", &c.structure, c.grown, c.seconds, c.light_seconds);
            }
            Err(e) => println!("  cycle, lattice: {e} (today: {:?} in {seconds:.2} s)", today.map(|t| t.phytomer_count())),
        }
    }
}

fn frame(from: &str, to: &str, age: u64, seed: u64, t: f64) {
    let family = |name: &str| {
        let (p, _) = PRESETS.iter().find(|p| p.1 == name).expect("a shipped preset");
        let mut f = p.parameters();
        f.age = age as f64;
        f
    };
    let blended = telperion_core::blend::families(&family(from), &family(to), t).unwrap();
    let species = blended.space.species.expect("an engine species");
    let r = request(age, seed, blended.space.light);
    println!("\n{from} -> {to} age {age} seed {seed} t {t}: extinction {}", r.light.extinction);
    let started = Instant::now();
    match grow_marked(&species, r) {
        Ok((s, ..)) => line("today (F4 build)", &s, 0, started.elapsed().as_secs_f64(), f64::NAN),
        Err(e) => println!("  today (F4 build): {e} after {:.1} s", started.elapsed().as_secs_f64()),
    }
    match grow_cycle(&species, r, Model::Lattice, &[]) {
        Ok(c) => line("cycle, lattice", &c.structure, c.grown, c.seconds, c.light_seconds),
        Err(e) => println!("  cycle, lattice: {e}"),
    }
}
