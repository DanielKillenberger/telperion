//! fn-211.4, probe (a): today's tree against the same tree with every
//! whole-tree reduction grouped as a partition at a cut takes it, the
//! state compared after every phase of every cycle; varied over cut, the
//! order fine roots' partials are combined in, and their chunking.
//!
//!   cargo run --profile ci -p telperion-space --example partition -- [ages] [seeds] [species]
use telperion_space::{
    beech, digest, grow, grow_partitioned, oak, spruce, Expected, Grouping, Light, PartitionOrder,
    Request, Species, PHASES,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds) = (list(0, "40,80"), list(1, "1,7"));
    let only = args
        .get(2)
        .cloned()
        .unwrap_or_else(|| "oak,beech,spruce".into());
    let all: [(&str, fn() -> Species); 3] = [("oak", oak), ("beech", beech), ("spruce", spruce)];
    let every = |order, chunk| Grouping {
        order,
        chunk,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let one = |sizes, leaves, girth| Grouping {
        order: PartitionOrder::Forward,
        chunk: 1,
        sizes,
        leaves,
        girth,
    };
    for (name, make) in all.into_iter().filter(|s| only.contains(s.0)) {
        let species = make();
        for &age in &ages {
            let expected = Expected::new(&species, age as u32, 0);
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light: LIT,
                };
                let whole =
                    grow_partitioned(&species, request, &expected, 1024.0, None).expect("grows");
                let (wd, wk) = (digest(&whole.structure), whole.structure.phytomer_count());
                let today = digest(&grow(&species, request).expect("grows"));
                println!(
                    "{name} {age} s{seed}: the reference is today's tree to the bit: {}",
                    wd == today
                );
                for cut in [256.0, 1024.0, 4096.0] {
                    let mut runs: Vec<(String, Grouping)> = vec![
                        ("sizes alone".into(), one(true, false, false)),
                        ("leaves alone".into(), one(false, true, false)),
                        ("girth alone".into(), one(false, false, true)),
                    ];
                    for order in [
                        PartitionOrder::Forward,
                        PartitionOrder::Reverse,
                        PartitionOrder::Shuffled(3),
                    ] {
                        for chunk in [1, 7, 64] {
                            runs.push((
                                format!("all, {order:?}, chunk {chunk}"),
                                every(order, chunk),
                            ));
                        }
                    }
                    for (label, g) in runs {
                        let p = grow_partitioned(&species, request, &expected, cut, Some(g))
                            .expect("grows");
                        let first = whole.phases.iter().zip(&p.phases).enumerate().find_map(
                            |(c, (a, b))| (0..4).find(|&k| a[k] != b[k]).map(|k| (c, PHASES[k])),
                        );
                        let same = digest(&p.structure) == wd;
                        let k = p.structure.phytomer_count();
                        let (tip, radius) = moved(&whole.structure, &p.structure);
                        println!(
                            "{name} {age} s{seed} cut {cut} {label}: {} ; tree {} ; kept {k} against {wk} ({:+.3} %); tips moved at most {tip:.2e} m, radii {radius:.2e} relative",
                            first.map_or("every phase identical".into(), |(c, ph)| format!("first difference cycle {c}, {ph}")),
                            if same { "identical" } else { "differs" },
                            100.0 * (k as f64 / wk as f64 - 1.0)
                        );
                    }
                }
            }
        }
    }
}

/// Where two trees share their topology axis for axis: the most a tip
/// moved and a radius changed (relative); infinite where they do not.
fn moved(a: &telperion_space::Structure, b: &telperion_space::Structure) -> (f64, f64) {
    if a.axes.len() != b.axes.len() {
        return (f64::INFINITY, f64::INFINITY);
    }
    let (mut tip, mut radius) = (0.0f64, 0.0f64);
    for (x, y) in a.axes.iter().zip(&b.axes) {
        if x.lineage != y.lineage || x.phytomers.len() != y.phytomers.len() {
            return (f64::INFINITY, f64::INFINITY);
        }
        for (p, q) in x.phytomers.iter().zip(&y.phytomers) {
            tip = tip.max((p.tip - q.tip).length());
            if p.radius > 0.0 {
                radius = radius.max((q.radius / p.radius - 1.0).abs());
            }
        }
    }
    (tip, radius)
}
