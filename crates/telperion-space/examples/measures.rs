//! The engine's measures for the passed species: each tree's hash over
//! every public field, to the bit, and its grow time.
//!
//!   cargo run --release -p telperion-space --example measures -- [age] [seeds]
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;
use telperion_space::{beech, grow, oak, spruce, Light, Request, Species, Structure, Vec3};

const BUDGET: u32 = 20_000_000;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let age: u32 = args.first().map_or(80, |a| a.parse().expect("age"));
    let seeds: Vec<u64> = args.get(1).map_or(vec![1, 7], |s| {
        s.split(',').map(|v| v.parse().expect("seed")).collect()
    });
    let species: [(&str, fn() -> Species); 3] =
        [("beech", beech), ("spruce", spruce), ("oak", oak)];
    for (name, make) in species {
        for &seed in &seeds {
            let started = Instant::now();
            let tree = grow(
                &make(),
                Request {
                    age,
                    seed,
                    budget: BUDGET,
                    light: Light::NEUTRAL,
                },
            )
            .expect("grows");
            let seconds = started.elapsed().as_secs_f64();
            println!(
                "{name} age {age} seed {seed}: hash {:016x}, {} phytomers, grown in {seconds:.2} s",
                hash(&tree),
                tree.phytomer_count()
            );
        }
    }
}

/// Every public field of the tree, to the bit.
fn hash(tree: &Structure) -> u64 {
    let mut h = DefaultHasher::new();
    let v = |h: &mut DefaultHasher, v: Vec3| [v.x, v.y, v.z].map(|c| c.to_bits()).hash(h);
    (tree.age, tree.pas).hash(&mut h);
    for axis in &tree.axes {
        (axis.lineage, axis.pa, axis.birth, axis.apex_end).hash(&mut h);
        format!("{:?}", axis.origin).hash(&mut h);
        axis.vigour.to_bits().hash(&mut h);
        v(&mut h, axis.base);
        v(&mut h, axis.heading);
        v(&mut h, axis.side);
        for p in &axis.phytomers {
            (p.cycle, p.radius.to_bits(), p.scale.to_bits()).hash(&mut h);
            v(&mut h, p.tip);
            v(&mut h, p.heading);
            v(&mut h, p.side);
        }
    }
    h.finish()
}
