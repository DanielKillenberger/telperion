//! fn-198 task 4 scratch: `grow` timed, best of three warm, seed 1.
use telperion_space::{grow, Light, Request};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (species, light) = match a[0].as_str() {
        "beech" => (telperion_space::beech(), Light::default()),
        "oak" => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        "spruce" => (telperion_space::spruce(), Light::default()),
        _ => panic!(),
    };
    let age: u32 = a[1].parse().unwrap();
    let mut best = f64::MAX;
    for _ in 0..3 {
        let t = std::time::Instant::now();
        let s = grow(&species, Request { age, seed: 1, budget: 40_000_000, light }).unwrap();
        best = best.min(t.elapsed().as_secs_f64() * 1e3);
        drop(s);
    }
    println!("{} {age} best {best:.0} ms", a[0]);
}
