//! fn-209 probe P2: the oak's committed light field, against the field
//! its realized last-cycle leaves cast.
//!
//!   cargo run --release -p telperion-space --example field -- [ages] [seeds] [levels]
use std::time::Instant;
use telperion_space::{field_probe, grow, oak, Expected, Light, Request};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize, d: &str| -> Vec<u64> {
        let text = args.get(i).map_or(d, String::as_str);
        text.split(',').map(|v| v.parse().expect("a number")).collect()
    };
    let (ages, seeds, levels) = (list(0, "40"), list(1, "1,7"), list(2, "1,2"));
    let species = oak();
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    for &age in &ages {
        let t = Instant::now();
        let expected = Expected::new(&species, age as u32, 0);
        println!("oak age {age}: rows {:.2} s", t.elapsed().as_secs_f64());
        for &seed in &seeds {
            let request = Request {
                age: age as u32,
                seed,
                budget: 20_000_000,
                light,
            };
            let reference = grow(&species, request).expect("grows");
            for &level in &levels {
                for (lit, p) in [false, true].into_iter().flat_map(|lit| field_probe(&species, &reference, light, &expected, level as usize, &[0.5, 1.0, 2.0], lit).into_iter().map(move |p| (lit, p))) {
                    println!(
                        "  seed {seed} levels <= {level}, light axis {lit}, cell {} m: build {:.2} ms, {} nodes, {} own leaves, {} buds spread; light at living apexes |committed - realized| p50 {:.3} p95 {:.3}, means {:.3} against {:.3}; buds' expected last-cycle leaf area {:.1} m2 against realized {:.1} m2",
                        p.cell, p.millis, p.nodes, p.leaves, p.buds, p.diff.0, p.diff.1, p.means.0, p.means.1, p.area.0, p.area.1
                    );
                }
            }
        }
    }
}
