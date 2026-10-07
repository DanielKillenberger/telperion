//! fn-209 decision 7's proof: every tree's lineage digest (`digest`,
//! canonical order); then the lit oak through the stepped executor (cut
//! 1,024), fn-211's, which must equal `grow`'s.
use std::time::Instant;
use telperion_space::{
    beech, digest, grow, grow_stepped, oak, palm, spruce, Expected, Grouping, Light,
    PartitionOrder, Request,
};
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
    let grouping = Grouping {
        order: PartitionOrder::Forward,
        chunk: 1,
        sizes: true,
        leaves: true,
        girth: true,
    };
    let s = oak();
    for age in [40, 80] {
        let expected = Expected::new(&s, age, 0);
        for seed in [1, 7] {
            let t = Instant::now();
            let request = Request {
                age,
                seed,
                budget: 20_000_000,
                light: lights[1],
            };
            let tree = grow_stepped(&s, request, &expected, 1024.0, grouping).expect("grows");
            let tree = tree.structure;
            println!(
                "oak-stepped {age} {seed} 0.5: {:016x} {} {:.2}s",
                digest(&tree),
                tree.phytomer_count(),
                t.elapsed().as_secs_f64()
            );
        }
    }
}
