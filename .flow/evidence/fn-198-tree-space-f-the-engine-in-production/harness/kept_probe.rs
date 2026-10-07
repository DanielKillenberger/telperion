//! fn-198 task 4 scratch probe: expected kept against kept.
use telperion_space::{expected_kept, grow, Light, Request};
fn main() {
    let shaded = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let args: Vec<String> = std::env::args().collect();
    let name = args[1].as_str();
    let (species, light) = match name {
        "beech" => (telperion_space::beech(), Light::default()),
        "oak" => (telperion_space::oak(), shaded),
        "spruce" => (telperion_space::spruce(), Light::default()),
        "palm" => (telperion_space::palm(), Light::default()),
        _ => panic!("species"),
    };
    for age in args[2..].iter().map(|a| a.parse::<u32>().unwrap()) {
        let t = std::time::Instant::now();
        let e = expected_kept(&species, age).unwrap().unwrap_or(f64::NAN);
        let ms = t.elapsed().as_secs_f64() * 1e3;
        for seed in [42u64, 1, 7] {
            let tree = grow(
                &species,
                Request {
                    age,
                    seed,
                    budget: 40_000_000,
                    light,
                },
            )
            .unwrap();
            let kept: usize = tree.axes.iter().map(|a| a.phytomers.len()).sum();
            println!(
                "{name} {age} seed {seed} expected {e:.0} ({ms:.1} ms) kept {kept} ratio {:.3}",
                kept as f64 / e
            );
        }
    }
}
