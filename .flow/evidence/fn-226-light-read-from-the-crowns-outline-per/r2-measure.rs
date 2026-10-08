//! fn-226 R2 probe, not product code: the lit oak's measures and build
//! time, before and after the re-lay's deletion. Copied into
//! `crates/telperion-space/examples/r2.rs` for a run, removed after.
//!
//!   cargo run --profile ci -p telperion-space --example r2 -- <tag> <ages> <seeds> <repeats>
//!
//! One JSON line per tree: build time (best of `repeats` of `grow`),
//! phytomers grown (the stepped executor's counts, cut 1,024), kept,
//! height, crown x / y, and the crown base live and major as R1.md
//! defines them.
use telperion_space::{grow, grow_stepped, oak, Expected, Grouping, Light, Origin};
use telperion_space::{PartitionOrder, Request, Structure};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u64> { args[i].split(',').map(|v| v.parse().unwrap()).collect() };
    let tag = &args[0];
    let (ages, seeds) = (list(1), list(2));
    let repeats: usize = args.get(3).map_or(1, |r| r.parse().unwrap());
    for &age in &ages {
        for &seed in &seeds {
            run(tag, age as u32, seed, repeats);
        }
    }
}

fn run(tag: &str, age: u32, seed: u64, repeats: usize) {
    let species = oak();
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: LIT,
    };
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let load = load.split(' ').next().unwrap_or("").to_string();
    let mut best: Option<(f64, Structure)> = None;
    for _ in 0..repeats {
        let t = std::time::Instant::now();
        let built = grow(&species, request).expect("grows");
        let took = t.elapsed().as_secs_f64();
        if best.as_ref().map_or(true, |b| took < b.0) {
            best = Some((took, built));
        }
    }
    let (took, tree) = best.unwrap();
    let g = Grouping {
        order: PartitionOrder::Forward,
        chunk: 16,
        sizes: true,
        leaves: true,
    };
    let expected = Expected::new(&species, age, 0);
    let s = grow_stepped(&species, request, &expected, 1024.0, g).expect("grows");
    let grown = s.fine_grown + s.coarse_grown;
    let same = telperion_space::digest(&s.structure) == telperion_space::digest(&tree);
    let m = measures(&tree);
    println!(
        "{{\"tag\":\"{tag}\",\"age\":{age},\"seed\":{seed},\"build\":{took:.4},\"load\":\"{load}\",\"grown\":{grown},\"kept\":{},\"shed\":{:.2},\"height\":{:.2},\"crown_x\":{:.2},\"crown_y\":{:.2},\"base_live\":{:.2},\"base_major\":{:.2},\"stepped_same\":{same},\"digest\":\"{:x}\"}}",
        m.kept,
        100.0 * (grown as f64 - m.kept as f64) / grown as f64,
        m.height,
        m.crown[0],
        m.crown[1],
        m.base_live,
        m.base_major,
        telperion_space::digest(&tree)
    );
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
