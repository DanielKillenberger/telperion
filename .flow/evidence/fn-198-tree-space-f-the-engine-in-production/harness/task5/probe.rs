//! fn-198 task 5 scratch: `grow` on the cores given, best of N warm, seed 1.
//! Arguments: species age [runs]. Prints the best, the median and the
//! process's threads after growth.
use telperion_space::{grow, Light, Request};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let (species, light) = match a[0].as_str() {
        "beech" => (telperion_space::beech(), Light::default()),
        "oak" => (telperion_space::oak(), Light { extinction: 0.5, sky: 0.5 }),
        "spruce" => (telperion_space::spruce(), Light::default()),
        "palm" => (telperion_space::palm(), Light::default()),
        _ => panic!(),
    };
    let age: u32 = a[1].parse().unwrap();
    let runs: usize = a.get(2).map_or(3, |r| r.parse().unwrap());
    let mut times = Vec::new();
    let mut count = 0;
    for _ in 0..runs {
        let t = std::time::Instant::now();
        let s = grow(&species, Request { age, seed: 1, budget: 40_000_000, light }).unwrap();
        times.push(t.elapsed().as_secs_f64() * 1e3);
        count = s.phytomer_count();
        drop(s);
    }
    let threads = std::fs::read_to_string("/proc/self/status")
        .unwrap()
        .lines()
        .find(|l| l.starts_with("Threads"))
        .unwrap()
        .to_string();
    times.sort_by(f64::total_cmp);
    println!(
        "{} {age} best {:.0} median {:.0} ms kept {count} {threads}",
        a[0],
        times[0],
        times[times.len() / 2]
    );
}
