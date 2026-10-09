//! fn-211.4, probe (a) through the stepped executor: every fine root at a
//! cut grown by its own grower beside the coarse one, compared with the
//! whole tree after every phase of every cycle, varied over cut, the
//! order items are visited and merged in, and their chunking.
//!
//!   cargo run --profile ci -p telperion-space --example stepped -- [ages] [seeds] [species] [cuts] [full]
//!
//! `full` runs every order with every chunking; otherwise each order
//! once, each with another chunking.
use telperion_space::{
    beech, digest, grow_partitioned, grow_stepped, oak, spruce, Expected, Grouping, Light,
    PartitionOrder, Request, Species, PHASES,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |i: usize, d: &str| args.get(i).cloned().unwrap_or_else(|| d.into());
    let list = |i: usize, d: &str| -> Vec<f64> {
        arg(i, d)
            .split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let (ages, seeds, cuts) = (list(0, "40,80"), list(1, "1,7"), list(3, "256,1024,4096"));
    let only = arg(2, "oak,unlitoak,beech,spruce");
    let full = arg(4, "") == "full";
    let orders = [
        PartitionOrder::Forward,
        PartitionOrder::Reverse,
        PartitionOrder::Shuffled(3),
    ];
    let mut groupings = Vec::new();
    for (k, order) in orders.into_iter().enumerate() {
        for (c, chunk) in [1, 7, 64].into_iter().enumerate() {
            if full || k == c {
                groupings.push(Grouping {
                    order,
                    chunk,
                    sizes: true,
                    leaves: true,
                });
            }
        }
    }
    // The unlit oak's roots run ahead between synchronisation points;
    // the lit oak's step every cycle (fn-211 task 12).
    let all: [(&str, fn() -> Species, Light); 4] = [
        ("oak", oak, LIT),
        ("unlit oak", oak, Light::NEUTRAL),
        ("beech", beech, LIT),
        ("spruce", spruce, LIT),
    ];
    let wanted = |name: &str| only.split(',').any(|o| o == name.replace(' ', ""));
    for (name, make, light) in all.into_iter().filter(|s| wanted(s.0)) {
        let species = make();
        for &age in &ages {
            let expected = Expected::new(&species, age as u32, 0);
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed: seed as u64,
                    budget: 20_000_000,
                    light,
                };
                let started = std::time::Instant::now();
                let whole = grow_partitioned(&species, request, &expected, &[]).expect("grows");
                let took = started.elapsed().as_secs_f64();
                let want = digest(&whole.structure);
                println!("{name} {age} s{seed}: the whole tree in {took:.1} s");
                for &cut in &cuts {
                    for g in &groupings {
                        let started = std::time::Instant::now();
                        let s = grow_stepped(&species, request, &expected, cut, *g).expect("grows");
                        let took = started.elapsed().as_secs_f64();
                        assert_eq!(s.phases.len(), whole.phases.len(), "every cycle hashed");
                        let first = whole.phases.iter().zip(&s.phases).enumerate().find_map(
                            |(c, (a, b))| (0..4).find(|&k| a[k] != b[k]).map(|k| (c, PHASES[k])),
                        );
                        let all = s.fine_grown + s.coarse_grown;
                        println!(
                            "{name} {age} s{seed} cut {cut} {:?} chunk {}: {}; {} cycles compared; tree {}; {} fine roots growing {} of {all} phytomers ({:.1} %); {took:.1} s",
                            g.order,
                            g.chunk,
                            first.map_or("every phase identical".into(), |(c, ph)| {
                                format!("first difference cycle {c}, {ph}")
                            }),
                            s.phases.len() - 1,
                            if digest(&s.structure) == want { "identical" } else { "differs" },
                            s.roots,
                            s.fine_grown,
                            100.0 * s.fine_grown as f64 / all as f64,
                        );
                    }
                }
            }
        }
    }
}
