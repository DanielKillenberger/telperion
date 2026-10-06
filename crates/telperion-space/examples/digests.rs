//! fn-209 decision 7's proof: every tree's lineage digest (`digest`, canonical order).
use std::time::Instant;
use telperion_space::{beech, digest, grow, oak, palm, spruce, Light, Request};
fn main() {
    let lights = [
        Light::NEUTRAL,
        Light {
            extinction: 0.5,
            sky: 0.5,
        },
    ];
    for (name, s) in [
        ("beech", beech()),
        ("spruce", spruce()),
        ("oak", oak()),
        ("palm", palm()),
    ] {
        for age in [40, 80] {
            for seed in [1, 7] {
                for light in lights {
                    let t = Instant::now();
                    let tree = grow(
                        &s,
                        Request {
                            age,
                            seed,
                            budget: 20_000_000,
                            light,
                        },
                    )
                    .expect("grows");
                    println!(
                        "{name} {age} {seed} {}: {:016x} {} {:.2}s",
                        light.extinction,
                        digest(&tree),
                        tree.phytomer_count(),
                        t.elapsed().as_secs_f64()
                    );
                }
            }
        }
    }
}
