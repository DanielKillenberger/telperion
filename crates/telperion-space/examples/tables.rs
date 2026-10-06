//! How long the closed form's tables take to read for one species point on
//! one core (fn-209, probe P1): every row a bud can need at each age.
//!
//! `cargo run --profile ci -p telperion-space --example tables -- [species|all] [ages] [repeats]`
//!
//! Prints, per species and age, the rows held and the fastest and median
//! of `repeats` reads on one core.
use std::time::Instant;
use telperion_space::{beech, oak, palm, spruce, Expected, Species};

type Make = fn() -> Species;

const SPECIES: [(&str, Make); 4] = [
    ("beech", beech),
    ("spruce", spruce),
    ("oak", oak),
    ("palm", palm),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let only = args.first().filter(|o| o.as_str() != "all");
    let ages: Vec<u32> = args
        .get(1)
        .map_or("10,20,40,80", String::as_str)
        .split(',')
        .map(|v| v.parse().expect("an age"))
        .collect();
    let repeats: usize = args.get(2).map_or(5, |r| r.parse().expect("a count"));
    for (name, make) in SPECIES {
        if only.is_some_and(|o| o != name) {
            continue;
        }
        let species = make();
        for &age in &ages {
            let mut ms: Vec<f64> = Vec::with_capacity(repeats);
            let mut held = 0;
            for _ in 0..repeats {
                let started = Instant::now();
                let expected = Expected::new(&species, age, 1);
                ms.push(started.elapsed().as_secs_f64() * 1e3);
                held = expected.cost().1;
            }
            ms.sort_by(f64::total_cmp);
            println!(
                "{name} age {age}: {held} rows, one core min {:.2} ms, median {:.2} ms",
                ms[0],
                ms[ms.len() / 2]
            );
        }
    }
}
