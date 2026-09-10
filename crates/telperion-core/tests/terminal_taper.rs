use telperion_core::{
    branching::{self},
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
        let taper = family.skeleton.habit.twig_tip_taper;
        let tip_radius = family.skeleton.twigs.resolved().unwrap().twig.diameter / 2.0 * taper;
        let mut tapered = 0;
        for (i, node) in tree.nodes.iter().enumerate() {
            let mut expected = solved.nodes[i].clone();
            if i > 0 && node.kind == NodeKind::Structural && children[i] == 0 {
                expected.radius = expected.radius.min(tip_radius);
                tapered += 1;
            }
            assert_eq!(
                *node, expected,
                "{preset:?} node {i}: only a true terminal may change"
            );
        }
        if preset == Preset::NorwaySpruce {
            // The narrowest structural ends are the ones the local layer never
            // reached; the taper trait is what decides how far they narrow.
            let narrowed: Vec<_> = tree
                .nodes
                .iter()
                .enumerate()
                .skip(1)
                .filter(|(i, n)| {
                    n.kind == NodeKind::Structural && children[*i] == 0 && n.radius <= tip_radius
                })
                .collect();
            assert!(tapered > 100, "cover the sibling structural endpoints");
            assert_eq!(narrowed.len(), tapered);
            for (_, n) in &narrowed {
                assert!(n.radius < n.start_radius * 0.5);
            }
            println!(
                "spruce seed1: {tapered} childless structural ends at taper {taper}, tip radius {tip_radius}m"
            );
        }
    }
}
