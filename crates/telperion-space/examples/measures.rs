//! The engine's measures for the passed species (fn-197).
//!
//!   cargo run --release -p telperion-space --example measures -- <what> [age] [seeds] [species] [light]
//!
//! - `hash`: each tree's hash over every public field, to the bit, without
//!   light, or under `LIT` (`lit`) or both (`both`): the proof that a
//!   change left every species as it was.
use std::hash::{DefaultHasher, Hash, Hasher};
use std::time::Instant;
use telperion_space::{beech, grow, oak, palm, spruce, Light, Request, Species, Structure, Vec3};

const BUDGET: u32 = 20_000_000;
/// The light the measures are taken under: leaves at every angle, under
/// the standard overcast sky.
const LIT: Light = Light {
    extinction: 0.5,
    sky: 0.5,
};
const SPECIES: [(&str, fn() -> Species); 4] = [
    ("beech", beech),
    ("spruce", spruce),
    ("oak", oak),
    ("palm", palm),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let what = args.first().map_or("hash", String::as_str);
    let age: u32 = args.get(1).map_or(80, |a| a.parse().expect("age"));
    let seeds: Vec<u64> = args.get(2).map_or(vec![1, 7], |s| {
        s.split(',').map(|v| v.parse().expect("seed")).collect()
    });
    let request = |seed, light| Request {
        age,
        seed,
        budget: BUDGET,
        light,
    };
    let only = args.get(3).filter(|o| o.as_str() != "all");
    // `hash` defaults to neutral light.
    let lights: Vec<Light> = match (args.get(4).map(String::as_str), what) {
        (Some("neutral"), _) | (None, "hash") => vec![Light::NEUTRAL],
        (Some("lit"), _) => vec![LIT],
        _ => vec![Light::NEUTRAL, LIT],
    };
    for (name, make) in SPECIES {
        if only.is_some_and(|o| o != name) {
            continue;
        }
        let species = make();
        for &seed in &seeds {
            match what {
                "hash" => {
                    for &light in &lights {
                        let started = Instant::now();
                        let tree = grow(&species, request(seed, light)).expect("grows");
                        println!(
                            "{name} age {age} seed {seed} extinction {}: hash {:016x}, {} phytomers, grown in {:.2} s",
                            light.extinction,
                            hash(&tree),
                            tree.phytomer_count(),
                            started.elapsed().as_secs_f64()
                        );
                    }
                }
                _ => panic!("measures hash [age] [seeds]"),
            }
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
