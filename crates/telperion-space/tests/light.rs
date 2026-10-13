//! fn-197 step 2: the rough layout grown with the tree, and the light it
//! casts, leave every passed species as it was while no bud reads them;
//! and where the final lay knows nothing the tree's later years add (no
//! sag, erection, straightening or shedding), the rough layout is the
//! final one (`grow/tests.rs`, against the engine that lays it).
use telperion_space::{beech, grow, oak, spruce, Light, Request, Species};

fn request(age: u32, seed: u64, light: Light) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light,
    }
}

const SHADING: [Light; 3] = [
    Light {
        extinction: 0.5,
        sky: 0.0,
    },
    Light {
        extinction: 0.5,
        sky: 0.5,
    },
    Light {
        extinction: 2.0,
        sky: 1.0,
    },
];

#[test]
fn light_no_bud_reads_leaves_every_passed_species_to_the_bit() {
    let all: [(&str, fn() -> Species); 3] = [("beech", beech), ("spruce", spruce), ("oak", oak)];
    for (name, make) in all {
        // The species as no bud reading light would grow it: the oak reads
        // light by its own values since its round 5.
        let mut species = make();
        for state in &mut species.states {
            state.shade_hazard = 0.0;
            state.shade_size = 0.0;
            state.balance_hazard = 0.0;
            state.leaf_girth = 0.0;
        }
        for seed in [1, 7] {
            let today = grow(&species, request(30, seed, Light::NEUTRAL)).unwrap();
            let sky = Light {
                extinction: 0.0,
                sky: 0.0,
            };
            assert!(
                grow(&species, request(30, seed, sky)).unwrap() == today,
                "{name} {seed}: sky"
            );
            for light in SHADING {
                let lit = grow(&species, request(30, seed, light)).unwrap();
                assert!(
                    lit == today,
                    "{name} seed {seed}: {light:?} changed the tree"
                );
            }
        }
    }
}
