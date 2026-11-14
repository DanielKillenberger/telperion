//! fn-198 task 3 probe (not kept in the crate): the phytomers each engine
//! species grows by an age, the shed ones included, found as the least
//! budget its growth passes.
use telperion_space::{grow, Error, Light, Request};

fn passes(species: &telperion_space::Species, age: u32, light: Light, budget: u32) -> bool {
    let request = Request { age, seed: 42, budget, light };
    match grow(species, request) {
        Ok(_) => true,
        Err(Error::Budget { .. }) => false,
        Err(e) => panic!("{e}"),
    }
}

fn main() {
    let shaded = Light { extinction: 0.5, sky: 0.5 };
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
        let request = Request { age, seed: 42, budget: 40_000_000, light };
        let tree = grow(&species, request).unwrap();
        let kept: usize = tree.axes.iter().map(|a| a.phytomers.len()).sum();
        println!("{name} {age} kept {kept}");
        if std::env::var("GROWN").is_err() { continue; }
        let (mut lo, mut hi) = (0u32, 40_000_000u32);
        while hi - lo > 10_000 {
            let mid = lo + (hi - lo) / 2;
            if passes(&species, age, light, mid) { hi = mid } else { lo = mid }
        }
        println!("{name} {age} grown <= {hi}");
    }
}
