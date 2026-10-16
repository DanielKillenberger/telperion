//! fn-198 task 5 scratch: `grow_staged` on the cores given, the time
//! between reports summed by report, best run of N (by total).
use std::collections::BTreeMap;
use std::time::Instant;
use telperion_space::{grow_staged, Light, Request, Stage};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (species, light) = match a[0].as_str() {
        "beech" => (telperion_space::beech(), Light::default()),
        "oak" => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        "spruce" => (telperion_space::spruce(), Light::default()),
        _ => panic!(),
    };
    let age: u32 = a[1].parse().unwrap();
    let runs: usize = a.get(2).map_or(3, |r| r.parse().unwrap());
    let mut best: Option<(f64, BTreeMap<String, f64>)> = None;
    for _ in 0..runs {
        let mut sums: BTreeMap<String, f64> = BTreeMap::new();
        let start = Instant::now();
        let mut last = start;
        let s = grow_staged(
            &species,
            Request { age, seed: 1, budget: 40_000_000, light },
            &mut |st: Stage| {
                let now = Instant::now();
                let key = match st {
                    Stage::Pass(p) => format!("{p:?}"),
                    other => format!("[{other:?}]"),
                };
                *sums.entry(key).or_default() += (now - last).as_secs_f64() * 1e3;
                last = now;
            },
        )
        .unwrap();
        let end = Instant::now();
        *sums.entry("~order".into()).or_default() += (end - last).as_secs_f64() * 1e3;
        let total = (end - start).as_secs_f64() * 1e3;
        drop(s);
        if best.as_ref().is_none_or(|b| total < b.0) {
            best = Some((total, sums));
        }
    }
    let (total, sums) = best.unwrap();
    println!("{} {age} total {total:.1}", a[0]);
    for (k, v) in sums {
        if v >= 0.05 {
            println!("  {k:<14} {v:9.1}");
        }
    }
}
