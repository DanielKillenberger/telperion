//! The tree-space conversion marks only the trunk as stem (fn-195 round 6,
//! host decision 31): an oak whose leader turns into a limb that aborts and
//! relays has no fork of stems in its crown, so the canopy measures its
//! slender wood against the trunk (`Tree::stem_radius`), not a twig.
#[path = "../examples/space/tree.rs"]
#[allow(dead_code)]
mod tree;
use telperion_space::{grow, oak, Light, Request};

/// The non-root nodes two or more stems part from: where `stem_radius`
/// would find a fork.
fn stem_forks(t: &telperion_core::tree::Tree) -> Vec<usize> {
    let mut parts = vec![0u32; t.nodes.len()];
    for n in &t.nodes[1..] {
        let p = n.parent.unwrap() as usize;
        if p == 0 || n.stem {
            parts[p] += 1;
        }
    }
    (1..t.nodes.len()).filter(|&i| parts[i] > 1).collect()
}

#[test]
fn a_leader_that_turns_limb_and_relays_leaves_no_fork_of_stems() {
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let request = Request {
        age: 80,
        seed: 1,
        budget: 20_000_000,
        light,
    };
    let structure = grow(&oak(), request).unwrap();
    let t = tree::convert(&structure, &tree::OAK_TRUNK);
    let forks = stem_forks(&t);
    assert!(forks.is_empty(), "stems part at {forks:?}");
}
