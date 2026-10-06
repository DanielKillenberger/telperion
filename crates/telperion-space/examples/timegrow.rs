//! fn-209 decision 7: `grow` timed alone, cold (first in the process) and warm (best of three more).
use std::time::Instant;
use telperion_space::{beech, grow, oak, palm, spruce, Light, Request};
fn main() {
    for (name, s) in [
        ("beech", beech()),
        ("spruce", spruce()),
        ("oak", oak()),
        ("palm", palm()),
    ] {
        for age in [40, 80] {
            let request = Request {
                age,
                seed: 1,
                budget: 20_000_000,
                light: Light::NEUTRAL,
            };
            let mut times = Vec::new();
            for _ in 0..4 {
                let t = Instant::now();
                let tree = grow(&s, request).expect("grows");
                times.push(t.elapsed().as_secs_f64() * 1e3);
                drop(tree);
            }
            let warm = times[1..].iter().cloned().fold(f64::INFINITY, f64::min);
            println!("| {name} {age} 1 | {:.0} | {:.0} |", times[0], warm);
        }
    }
}
