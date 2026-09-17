use telperion_core::{
    branching::{generate, in_curtain_band},
    presets::Preset,
};
#[test]
fn complete_presets_are_deterministic_and_solved() {
    for preset in [Preset::Ordinary, Preset::Telperion, Preset::Laurelin] {
        let p = preset.parameters();
        let a = generate(&p.skeleton, p.radii).unwrap();
        a.tree.validate_solved().unwrap();
        assert_eq!(a, generate(&p.skeleton, p.radii).unwrap());
        assert!(a.tree.nodes.len() > a.tree.crossover);
        assert!(a.tree.diagnostics.complete());
        let (envelope, seed) = (p.skeleton.envelope, p.skeleton.seed);
        for n in a.tree.nodes.iter().skip(a.tree.crossover) {
            assert!(
                envelope.contains(n.position, 1e-8, seed)
                    || in_curtain_band(&envelope, &p.skeleton.twigs, seed, n.position, 1e-8),
                "seed {seed}: {:?} is outside the shell and the curtain's band",
                n.position
            );
            assert!(
                n.position
                    .distance(a.tree.nodes[n.parent.unwrap() as usize].position)
                    > 0.0
            );
        }
    }
}

use telperion_core::{
    branching::{append, shed, HabitParams, SkeletonParams},
    colonization::GrowthConfig,
    envelope::Envelope,
    math::Vec3,
    radius::{solve, RadiusParams},
    tree::{Node, NodeKind, Tree},
    twigs::TwigParams,
};
fn crown() -> Tree {
    let mut root = Node::root();
    root.radius = 0.1;
    root.start_radius = 0.1;
    let tip = Node {
        position: Vec3::new(0.0, 1.0, 0.0),
        parent: Some(0),
        radius: 0.1,
        start_radius: 0.1,
        branch: 1,
        ..Node::root()
    };
    Tree {
        nodes: vec![root, tip],
        crossover: 2,
        ..Default::default()
    }
}

#[path = "growth/anatomy.rs"]
mod anatomy;
#[path = "growth/habit.rs"]
mod habit;
#[path = "growth/limits.rs"]
mod limits;
