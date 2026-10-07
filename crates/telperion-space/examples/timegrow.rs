//! fn-209 decisions 7 and 9: `grow` and `grow_whole` timed alone, cold
//! (first in the process) and warm (best of three more). Arguments: the
//! species to time (default all four); `RAYON_NUM_THREADS=1` for one core.
use std::time::Instant;
use telperion_space::{
    beech, grow, grow_whole, oak, palm, spruce, Light, Request, Result, Species, Structure,
};

type Grow = fn(&Species, Request) -> Result<Structure>;

fn main() {
    let only: Vec<String> = std::env::args().skip(1).collect();
    let all = [
        ("beech", beech()),
        ("spruce", spruce()),
        ("oak", oak()),
        ("palm", palm()),
    ];
    println!("| Tree | grow, cold / warm ms | grow_whole, cold / warm ms |");
    for (name, s) in all {
        if !only.is_empty() && !only.iter().any(|o| o == name) {
            continue;
        }
        for age in [40, 80] {
            let request = Request {
                age,
                seed: 1,
                budget: 20_000_000,
                light: Light::NEUTRAL,
            };
            let ways: [Grow; 2] = [grow, grow_whole];
            let [(gc, gw), (wc, ww)] = ways.map(|way| time(way, &s, request));
            println!("| {name} {age} 1 | {gc:.0} / {gw:.0} | {wc:.0} / {ww:.0} |");
        }
    }
}

/// The first build's milliseconds and the best of three more.
fn time(way: Grow, s: &Species, request: Request) -> (f64, f64) {
    let mut times = Vec::new();
    for _ in 0..4 {
        let t = Instant::now();
        let tree = way(s, request).expect("grows");
        times.push(t.elapsed().as_secs_f64() * 1e3);
        drop(tree);
    }
    let warm = times[1..].iter().copied().fold(f64::INFINITY, f64::min);
    (times[0], warm)
}
