//! fn-209 decision 7's proof: every tree's lineage digest (`digest`,
//! canonical order); then the lit oak through the stepped executor (cut
//! 1,024), fn-211's, which must equal `grow`'s. Each line also prints
//! the tree's topology by lineage (`topology`), which arithmetic that moves
//! fields leaves alone.
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;
use telperion_space::{
    beech, digest, grow, grow_stepped, oak, palm, spruce, Expected, Grouping, Light,
    PartitionOrder, Request, Structure,
};

/// The sorted lineage identities, each with its axis's phytomer count.
fn topology(tree: &Structure) -> u64 {
    let mut axes: Vec<_> = (tree.axes.iter())
        .map(|a| (a.lineage, a.phytomers.len()))
        .collect();
    axes.sort_unstable();
    let mut h = DefaultHasher::new();
    axes.hash(&mut h);
    h.finish()
}
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
                        "{name} {age} {seed} {}: {:016x} {:016x} {} {:.2}s",
                        light.extinction,
                        digest(&tree),
                        topology(&tree),
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
                "oak-stepped {age} {seed} 0.5: {:016x} {:016x} {} {:.2}s",
                digest(&tree),
                topology(&tree),
                tree.phytomer_count(),
                t.elapsed().as_secs_f64()
            );
        }
    }
}
