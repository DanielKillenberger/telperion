//! The pipeline's conversion of the engine's structure marks only the trunk
//! as stem (fn-195 round 6, host decision 31): an oak whose leader turns
//! into a limb that aborts and relays has no fork of stems in its crown, so
//! the canopy measures its slender wood against the trunk
//! (`Tree::stem_radius`), not a twig.
use telperion_core::{pipeline::executor, presets::Preset};

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
    let mut family = Preset::OregonWhiteOak.parameters();
    (family.age, family.skeleton.seed) = (80.0, 1);
    let x = executor::grow(&family).unwrap().expansion().unwrap();
    let forks = stem_forks(x.tree());
    assert!(forks.is_empty(), "stems part at {forks:?}");
}
