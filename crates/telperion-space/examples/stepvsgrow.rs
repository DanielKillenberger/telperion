//! fn-211.11 and .12: today's path (`grow_today`) against `grow`, the
//! stepped executor (cut 1,024), each species at each age and seed:
//! digests, and best of three warm builds taken back to back, one of each
//! in turn.
//!
//!   cargo run --profile ci -p telperion-space --example stepvsgrow -- [species] [ages] [seeds]
use std::time::Instant;
use telperion_space::{
    beech, digest, grow, grow_today, oak, palm, spruce, Light, Request, Species,
};

const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
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
    println!("| Tree | today, warm s | grow, warm s | grow / today | same tree |");
    for (name, make, light) in all {
        if !only.split(',').any(|o| o == name) {
            continue;
        }
        let species = make();
        for &age in &ages {
            for &seed in &seeds {
                let request = Request {
                    age: age as u32,
                    seed,
                    budget: 20_000_000,
                    light,
                };
                let (mut best, mut same) = ([f64::INFINITY; 2], true);
                // One cold build of each, then three warm of each in turn.
                for round in 0..4 {
                    let started = Instant::now();
                    let today = grow_today(&species, request).expect("grows");
                    let g = started.elapsed().as_secs_f64();
                    let started = Instant::now();
                    let tree = grow(&species, request).expect("grows");
                    let t = started.elapsed().as_secs_f64();
                    same &= digest(&tree) == digest(&today);
                    if round > 0 {
                        best = [best[0].min(g), best[1].min(t)];
                    }
                }
                println!(
                    "| {name} {age} s{seed} | {:.3} | {:.3} | {:.2} | {same} |",
                    best[0],
                    best[1],
                    best[1] / best[0],
                );
            }
        }
    }
}
