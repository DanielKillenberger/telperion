//! A relay joins its parent at the node it grows from, where its wood
//! enters (fn-244): the stem never thickens again on its way up past a
//! join. The stub's own wood is `telperion-space`'s girth test.
use super::*;
use crate::presets::Preset;

/// The engine's tree of `preset` at `age` and `seed`, converted.
fn converted_at(preset: Preset, age: u32, seed: u32) -> Tree {
    let family = preset.parameters();
    let species = family.space.species.unwrap();
    let request = Request {
        age,
        seed: u64::from(seed),
        budget: BUDGET,
        light: family.space.light,
    };
    let (structure, fine, bases) = telperion_space::grow_marked(&species, request).unwrap();
    let rows = pa_rows(&species);
    convert(&structure, (&fine, &bases), (&family.space.trunk, &rows))
}

/// The trees whose trunks dented at their relays' joins (DENTS.md).
const DENTED: [(Preset, u32, u32); 3] = [
    (Preset::EuropeanBeech, 80, 42),
    (Preset::Ordinary, 80, 42),
    (Preset::Ordinary, 40, 42),
];

#[test]
fn a_stem_never_thickens_on_its_way_up() {
    for (preset, age, seed) in DENTED {
        let tree = converted_at(preset, age, seed);
        for (i, n) in tree.nodes.iter().enumerate().skip(2) {
            let p = &tree.nodes[n.parent.unwrap() as usize];
            if n.stem && p.stem {
                // The girth is f32 (fn-254): a node may stand an f32 place
                // over the one below (1.0e-7 relative, one node of these
                // three trees), never a dent's worth.
                assert!(
                    n.radius <= p.radius * (1.0 + 2.5e-7),
                    "{preset:?} {age}/{seed}: stem node {i} at {:.2} m is {:.4} m over {:.4} m below",
                    n.position.y,
                    n.radius,
                    p.radius
                );
            }
        }
    }
}
