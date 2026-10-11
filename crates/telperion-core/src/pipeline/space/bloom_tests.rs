//! fn-253 R2: an axis of an age that flowers bears its inflorescence at
//! its own tip, the last node it draws from its own wood, one an axis, at
//! its age's share; no node along it bears one.
use super::*;
use crate::presets::Preset;
use std::collections::HashMap;

#[test]
fn a_flowering_axis_bears_its_bloom_at_its_own_tip() {
    let family = Preset::EuropeanBeech.parameters();
    let mut species = family.space.species.unwrap();
    // The shoots, which continue, and the short shoots, which do not.
    let shares = [(12, 0.4), (14, 0.7)];
    for (age, share) in shares {
        species.states[age].flowering = share;
    }
    let request = Request {
        age: 15,
        seed: 7,
        budget: BUDGET,
        light: family.space.light,
    };
    let structure = telperion_space::grow(&species, request).unwrap();
    let tree = convert(
        &structure,
        (&[], &[]),
        (&family.space.trunk, &pa_rows(&species)),
    );
    let node: HashMap<u64, usize> = (1..tree.nodes.len())
        .map(|i| (tree.nodes[i].identity.birth_order(), i))
        .collect();
    // Each flowering axis's tip and its share.
    let mut tips = HashMap::new();
    for axis in &structure.axes {
        let Some(&(_, share)) = shares.iter().find(|(a, _)| *a == axis.pa) else {
            continue;
        };
        let drawn = (axis.phytomers.iter()).filter_map(|p| node.get(&p.key()));
        if let Some(&tip) = drawn.last() {
            assert!(tips.insert(tip as u32, share).is_none());
        }
    }
    let ages = |pa| structure.axes.iter().filter(|a| a.pa == pa).count();
    assert!(ages(12) > 50 && ages(14) > 50, "both ages grow");
    assert_eq!(tree.blooms.len(), tips.len(), "one bloom a flowering axis");
    for b in &tree.blooms {
        assert_eq!(b.share, tips[&b.node], "node {}", b.node);
    }
    assert!(tree.blooms.windows(2).all(|w| w[0].node < w[1].node));
}
