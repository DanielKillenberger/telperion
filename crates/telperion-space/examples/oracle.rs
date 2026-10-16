//! fn-211.4's light oracle (host, 2026-10-06): today's oak grown under
//! today's exact light, except that every bud below the cut reads its fine
//! root's mean light that cycle (option C's best case) or that mean plus
//! its fitted spatial slopes (option B's best case), against today's oak:
//! kept phytomers per order, per-PA growth drift, the trunk's base and the
//! order-1 junctions, the crown's extent and the bud light read.
//!
//!   cargo run --profile ci -p telperion-space --example oracle -- [ages] [seeds] [cut]
#[path = "light/fidelity.rs"]
mod fidelity;
#[path = "junctions/stats.rs"]
mod stats;

use fidelity::describe;
use std::time::Instant;
use telperion_space::{digest, grow, grow_oracle, oak, Expected, Light, Read, Request};

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
    let cut: f64 = args.get(2).map_or(1024.0, |c| c.parse().expect("a cut"));
    let species = oak();
    for &age in &ages {
        let expected = Expected::new(&species, age as u32, 0);
        for &seed in &seeds {
            let request = Request {
                age: age as u32,
                seed,
                budget: 20_000_000,
                light: LIT,
            };
            let started = Instant::now();
            let today = grow_oracle(&species, request, &expected, cut, Read::Exact).expect("grows");
            let s = started.elapsed().as_secs_f64();
            let same = digest(&today.structure) == digest(&grow(&species, request).expect("grows"));
            println!(
                "oak {age} s{seed} cut {cut}: today's ({s:.1} s; the oracle's exact read is today's tree to the bit: {same})"
            );
            describe(
                "today",
                (&today.structure, &today.growth),
                (&today.structure, &today.growth),
            );
            for (name, read) in [
                ("C, a root's mean", Read::Mean),
                ("B, mean and slopes", Read::Slopes),
            ] {
                let ours = grow_oracle(&species, request, &expected, cut, read).expect("grows");
                describe(
                    name,
                    (&ours.structure, &ours.growth),
                    (&today.structure, &today.growth),
                );
            }
        }
    }
}
