//! fn-211.11: today's `grow` against the stepped executor (cut 1,024),
//! each species at each age and seed: digests, and best of three warm
//! builds taken back to back, one of each in turn.
//!
//!   cargo run --profile ci -p telperion-space --example stepvsgrow -- [species] [ages] [seeds]
use std::time::Instant;
use telperion_space::{
    beech, digest, grow, grow_stepped, oak, palm, spruce, Expected, Grouping, Light,
    PartitionOrder, Request, Species,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};

const GROUPING: Grouping = Grouping {
    order: PartitionOrder::Forward,
    chunk: 1,
    sizes: true,
    leaves: true,
    girth: true,
};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let arg = |i: usize, d: &str| args.get(i).cloned().unwrap_or_else(|| d.into());
    let list = |i: usize, d: &str| -> Vec<u64> {
        arg(i, d)
            .split(',')
            .map(|v| v.parse().expect("a number"))
            .collect()
    };
    let only = arg(0, "beech,spruce,oak,litoak,palm");
    let (ages, seeds) = (list(1, "40,80"), list(2, "1,7"));
    let all: [(&str, fn() -> Species, Light); 5] = [
        ("beech", beech, Light::NEUTRAL),
        ("spruce", spruce, Light::NEUTRAL),
        ("oak", oak, Light::NEUTRAL),
        ("litoak", oak, LIT),
        ("palm", palm, Light::NEUTRAL),
    ];
    println!("| Tree | grow, warm s | stepped, warm s | stepped / grow | stepped years / settle s | roots | same tree |");
    for (name, make, light) in all {
        if !only.split(',').any(|o| o == name) {
            continue;
        }
        let species = make();
        for &age in &ages {
            let expected = Expected::new(&species, age as u32, 0);
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light,
                };
                let (mut best, mut roots, mut same) = ([f64::INFINITY; 2], 0, true);
                let mut split = (0.0, 0.0);
                // One cold build of each, then three warm of each in turn.
                for round in 0..4 {
                    let started = Instant::now();
                    let want = digest(&grow(&species, request).expect("grows"));
                    let g = started.elapsed().as_secs_f64();
                    let started = Instant::now();
                    let s = grow_stepped(&species, request, &expected, 1024.0, GROUPING)
                        .expect("grows");
                    let t = started.elapsed().as_secs_f64();
                    same &= digest(&s.structure) == want;
                    roots = s.roots;
                    if round > 0 && t < best[1] {
                        split = (s.years.iter().sum::<f64>(), s.settle);
                    }
                    if round > 0 {
                        best = [best[0].min(g), best[1].min(t)];
                    }
                }
                println!(
                    "| {name} {age} s{seed} | {:.3} | {:.3} | {:.2} | {:.3} / {:.3} | {roots} | {same} |",
                    best[0],
                    best[1],
                    best[1] / best[0],
                    split.0,
                    split.1,
                );
            }
        }
    }
}
