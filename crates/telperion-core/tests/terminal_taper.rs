use telperion_core::{
    branching::{self, BranchHabit},
    presets::Preset,
    radius,
    tree::NodeKind,
};

#[test]
fn only_childless_species_structure_gets_terminal_taper() {
    for preset in [
        Preset::NorwaySpruce,
        Preset::OregonWhiteOak,
        Preset::Ordinary,
        Preset::Telperion,
        Preset::Laurelin,
    ] {
        let mut family = preset.parameters();
        if preset == Preset::NorwaySpruce {
            family.skeleton.seed = 1;
        }
        let report = branching::generate(&family.skeleton, family.radii).unwrap();
        let tree = &report.tree;
        tree.validate_solved().unwrap();
        assert!(tree.diagnostics.complete());
        let mut children = vec![0; tree.nodes.len()];
        for n in tree.nodes.iter().skip(1) {
            children[n.parent.unwrap() as usize] += 1;
        }
        let mut solved = tree.clone();
        radius::solve(&mut solved, family.skeleton.envelope, family.radii).unwrap();
        let tip_radius = family.skeleton.twigs.resolved().unwrap().twig.diameter / 2.0 * 0.25;
        let species = !matches!(family.skeleton.habit, BranchHabit::Colonizing);
        let mut tapered = 0;
        for (i, node) in tree.nodes.iter().enumerate() {
            let mut expected = solved.nodes[i].clone();
            if species && i > 0 && node.kind == NodeKind::Structural && children[i] == 0 {
                expected.radius = expected.radius.min(tip_radius);
                tapered += 1;
            }
            assert_eq!(
                *node, expected,
                "{preset:?} node {i}: only a true terminal may change"
            );
        }
        if preset == Preset::NorwaySpruce {
            let tip = &tree.nodes[5978];
            assert_eq!(tip.kind, NodeKind::Structural);
            assert_eq!(tip.parent, Some(5977));
            assert_eq!(children[5978], 0);
            assert!((tip.position.distance(tree.nodes[5977].position) - 0.041899).abs() < 1e-6);
            assert!((tip.start_radius - 0.002048824).abs() < 1e-9);
            assert!(tip.radius < tip.start_radius * 0.25);
            assert!(tapered > 100, "cover the sibling structural endpoints");
            println!(
                "spruce seed1: {tapered} childless structural ends; node5978 radius={}m",
                tip.radius
            );
        }
    }
}
