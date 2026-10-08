//! fn-226 task 3 probe, not product code: today's model (`grow`) against
//! R3+R4 (`grow_outline`) on the lit oak. Copied into
//! `crates/telperion-space/examples/r3.rs` for a run, removed after.
//!
//!   cargo run --profile ci -p telperion-space --example r3 -- <model> <ages> <seeds> <repeats>
//!
//! `model` is `today` or `outline`. One JSON line per tree: build time
//! (best of `repeats`), kept, shed share (outline: grown from its own
//! count; today: the stepped executor's counts), height, crown x / y,
//! crown base live and major (R1.md), and for the outline its phases'
//! seconds, roots, and host decision 23's drift: each PA's
//! presence-weighted mean size per cycle, its largest departure from 1
//! and its presence-weighted mean over the run.
use telperion_space::{grow, grow_outline, grow_stepped, oak, Expected, Grouping, Light, Origin};
use telperion_space::{PartitionOrder, Request, Structure};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u64> { args[i].split(',').map(|v| v.parse().unwrap()).collect() };
    let model = &args[0];
    let (ages, seeds) = (list(1), list(2));
    let repeats: usize = args.get(3).map_or(1, |r| r.parse().unwrap());
    for &age in &ages {
        for &seed in &seeds {
            run(model, age as u32, seed, repeats);
        }
    }
}

fn run(model: &str, age: u32, seed: u64, repeats: usize) {
    let species = oak();
    let request = Request {
        age,
        seed,
        budget: 40_000_000,
        light: LIT,
    };
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load = load.split(' ').next().unwrap_or("").to_string();
    let mut best = f64::INFINITY;
    let mut tree = None;
    let mut extra = String::new();
    let mut grown = 0u64;
    for _ in 0..repeats {
        let t = std::time::Instant::now();
        if model == "today" {
            let built = grow(&species, request).expect("grows");
            let took = t.elapsed().as_secs_f64();
            if took < best {
                (best, tree) = (took, Some(built));
            }
        } else {
            let o = grow_outline(&species, request).expect("grows");
            let took = t.elapsed().as_secs_f64();
            if took < best {
                best = took;
                grown = o.grown;
                extra = format!(
                    ",\"roots\":{},\"seconds\":{:?},\"drift\":{}",
                    o.roots,
                    o.seconds.map(|s| (s * 1000.0).round() / 1000.0),
                    drift(&o.growth)
                );
                tree = Some(o.structure);
            }
        }
    }
    let tree = tree.unwrap();
    if model == "today" {
        let g = Grouping {
            order: PartitionOrder::Forward,
            chunk: 16,
            sizes: true,
            leaves: true,
        };
        let expected = Expected::new(&species, age, 0);
        let s = grow_stepped(&species, request, &expected, 1024.0, g).expect("grows");
        grown = s.fine_grown + s.coarse_grown;
    }
    let m = measures(&tree);
    println!(
        "{{\"model\":\"{model}\",\"age\":{age},\"seed\":{seed},\"build\":{best:.4},\"load\":\"{load}\",\"grown\":{grown},\"kept\":{},\"shed\":{:.2},\"height\":{:.2},\"crown_x\":{:.2},\"crown_y\":{:.2},\"base_live\":{:.2},\"base_major\":{:.2}{extra}}}",
        m.kept,
        100.0 * (grown as f64 - m.kept as f64) / grown as f64,
        m.height,
        m.crown[0],
        m.crown[1],
        m.base_live,
        m.base_major,
    );
}

/// Per PA: the largest departure of a cycle's presence-weighted mean
/// size from 1 (cycles where the PA's presence is at least 1), and the
/// presence-weighted mean size over every cycle.
fn drift(growth: &[Vec<[f64; 2]>]) -> String {
    let pas = growth.iter().map(Vec::len).max().unwrap_or(0);
    let each: Vec<String> = (0..pas)
        .map(|pa| {
            let (mut worst, mut p, mut ps) = (0.0f64, 0.0, 0.0);
            for row in growth {
                if let Some(&[a, b]) = row.get(pa) {
                    p += a;
                    ps += b;
                    if a >= 1.0 {
                        worst = worst.max((b / a - 1.0).abs());
                    }
                }
            }
            let mean = if p > 0.0 { ps / p } else { 1.0 };
            format!("[{worst:.4},{mean:.4}]")
        })
        .collect();
    format!("[{}]", each.join(","))
}

struct Measures {
    kept: usize,
    height: f64,
    crown: [f64; 2],
    base_live: f64,
    base_major: f64,
}

/// R1.md's (OAK-LIGHT.md's) definitions: the kept tips' extent; the
/// lowest lateral off the stem bearing a living apex, and among those
/// carrying 1 % of the kept phytomers.
fn measures(tree: &Structure) -> Measures {
    let axes = &tree.axes;
    let (mut lo, mut hi) = ([f64::INFINITY; 3], [f64::NEG_INFINITY; 3]);
    let mut kept = 0;
    for a in axes {
        for p in &a.phytomers {
            kept += 1;
            for (k, v) in [p.tip.x, p.tip.y, p.tip.z].into_iter().enumerate() {
                lo[k] = lo[k].min(v);
                hi[k] = hi[k].max(v);
            }
        }
    }
    let parent = |a: &telperion_space::Axis| match a.origin {
        Origin::Seed => None,
        Origin::Lateral { parent, .. } => Some(parent),
        Origin::Continuation { parent } | Origin::Relay { parent, .. } => Some(parent),
    };
    let mut stem = vec![false; axes.len()];
    let (mut living, mut count) = (vec![false; axes.len()], vec![0usize; axes.len()]);
    for (i, a) in axes.iter().enumerate() {
        living[i] = a.apex_end.is_none();
        count[i] = a.phytomers.len();
        stem[i] = match a.origin {
            Origin::Seed => true,
            Origin::Continuation { parent } | Origin::Relay { parent, .. } => {
                stem[parent] && (2..=4).contains(&a.pa)
            }
            Origin::Lateral { .. } => false,
        };
        if let Some(p) = parent(a) {
            assert!(p < i, "parents first");
        }
    }
    for i in (1..axes.len()).rev() {
        if let Some(p) = parent(&axes[i]) {
            living[p] |= living[i];
            count[p] += count[i];
        }
    }
    let (mut live, mut major) = (f64::INFINITY, f64::INFINITY);
    for (i, a) in axes.iter().enumerate() {
        if let Origin::Lateral { parent, .. } = a.origin {
            if stem[parent] && living[i] {
                live = live.min(a.base.z);
                if count[i] as f64 >= 0.01 * kept as f64 {
                    major = major.min(a.base.z);
                }
            }
        }
    }
    Measures {
        kept,
        height: hi[2],
        crown: [hi[0] - lo[0], hi[1] - lo[1]],
        base_live: live,
        base_major: major,
    }
}
