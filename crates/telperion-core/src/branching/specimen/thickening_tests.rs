use super::*;
use crate::{presets::Preset, tree::BudFate};

fn fixture() -> Specimen {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 0.0;
    f.skeleton.habit.shedding_threshold = 0.5;
    f.growth.shedding_tolerance = 3.0;
    let mut s = Specimen::build(&f).unwrap();
    s.tree.nodes.push(Node {
        position: Vec3::new(0.0, 12.0, 0.0),
        parent: Some(0),
        branch: 1,
        radius: 0.1,
        start_radius: 0.1,
        ..Node::root()
    });
    s.tree.crossover = 2;
    for x in [0.0, 12.0] {
        let i = s.tree.nodes.len() as u32;
        let mut n = Node {
            position: Vec3::new(x, 12.0, 0.0),
            parent: Some(1),
            branch: i,
            kind: NodeKind::Branch,
            radius: 0.01,
            start_radius: 0.02,
            base_radius: 0.02,
            ..Node::root()
        };
        n.shoot.bud_fate = BudFate::Lateral;
        s.tree.nodes.push(n);
    }
    s.identify();
    s.timeline.as_mut().unwrap().envelope = s.params.envelope;
    s
}

#[test]
fn shedding_waits_for_tolerance_and_retires_only_the_dark_subtree() {
    let mut s = fixture();
    let dark = s.tree.nodes[2].identity;
    let live = s.tree.nodes[3].identity;
    for slice in 1..=3 {
        let roots = s.environment(slice);
        if slice < 3 {
            assert!(roots.is_empty(), "shed before tolerance");
        }
        s.retire(&roots);
    }
    assert!(
        s.node(dark).is_err(),
        "dark shoot must shed on third active slice"
    );
    assert!(s.node(live).is_ok());
    s.tree.validate_solved().unwrap();
}

#[test]
fn local_wood_thickens_with_its_parent_and_never_shrinks() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 15.0;
    let mut s = Specimen::build(&f).unwrap();
    let before: Vec<_> = s.tree().nodes[s.tree().crossover..]
        .iter()
        .map(|n| (n.identity, n.radius, n.start_radius, n.base_radius))
        .collect();
    assert!(!before.is_empty());
    s.advance(5.0).unwrap();
    let mut thicker = 0;
    for (id, radius, start, base) in before {
        let n = s.node(id).unwrap();
        assert!(n.radius >= radius && n.start_radius >= start && n.base_radius >= base);
        thicker += usize::from(n.base_radius > base);
    }
    assert!(
        thicker > 0,
        "surviving local allocations remain frozen at birth"
    );
}

#[test]
fn threshold_equality_recovers_and_restarts_the_tolerance_clock() {
    let mut s = fixture();
    s.environment(1);
    let equality = s.tree.nodes[2].shoot.vigour();
    assert_eq!(s.tree.nodes[2].shoot.low_slices(), 1);
    s.params.habit.shedding_threshold = equality;
    assert!(s.environment(1).is_empty());
    assert_eq!(
        s.tree.nodes[2].shoot.low_slices(),
        0,
        "equality must recover"
    );
    s.params.habit.shedding_threshold = 0.5;
    assert!(s.environment(2).is_empty());
    assert!(s.environment(3).is_empty());
    assert_eq!(s.environment(4), vec![s.tree.nodes[2].identity]);
}

#[test]
fn snapshot_cuts_are_capped_and_remove_new_descendants_without_reusing_ids() {
    let mut s = fixture();
    s.timeline.as_mut().unwrap().traits.shedding_tolerance = 0.0;
    for _ in 0..40 {
        let mut n = s.tree.nodes[2].clone();
        n.identity = NodeIdentity::default();
        n.branch = s.tree.nodes.len() as u32;
        s.tree.nodes.push(n);
    }
    s.identify();
    let roots = s.environment(1);
    assert_eq!(roots.len(), 32);
    assert!(roots.windows(2).all(|p| p[0] < p[1]));
    // Buds appended after the snapshot cannot rescue a condemned subtree.
    let i = s.tree.nodes.len() as u32;
    s.tree.nodes.push(Node {
        parent: Some(2),
        branch: i,
        kind: NodeKind::Twig,
        radius: 0.002,
        start_radius: 0.002,
        base_radius: 0.002,
        position: Vec3::new(20.0, 12.0, 0.0),
        ..Node::root()
    });
    s.identify();
    let newborn = s.tree.nodes[i as usize].identity;
    let next_birth = s.next_identity;
    s.retire(&roots);
    assert!(s.node(newborn).is_err());
    assert!(roots.iter().all(|&id| s.node(id).is_err()));
    let i = s.tree.nodes.len() as u32;
    s.tree.nodes.push(Node {
        parent: Some(1),
        branch: i,
        ..Node::root()
    });
    s.identify();
    assert_eq!(s.tree.nodes[i as usize].identity.birth_order(), next_birth);
    assert!(roots.iter().all(|&id| s.node(id).is_err()));
}

#[test]
fn structural_forks_and_cuts_preserve_surviving_widths_and_pipe_cache() {
    let mut s = fixture();
    // Make the dark shoot structural, retaining the local sibling after it.
    s.tree.nodes[2].kind = NodeKind::Structural;
    s.tree.nodes[3].kind = NodeKind::Structural;
    s.tree.crossover = 4;
    let mut pipes = radius::Pipes::default();
    pipes.update(&mut s.tree, 12.0, 24.0, s.radii).unwrap();
    s.timeline.as_mut().unwrap().pipes = pipes;
    let survivor = s.tree.nodes[1].identity;
    let before = s.node(survivor).unwrap().radius;
    s.retire(&[s.tree.nodes[2].identity]);
    s.timeline
        .as_mut()
        .unwrap()
        .pipes
        .update(&mut s.tree, 13.0, 24.0, s.radii)
        .unwrap();
    assert!(s.node(survivor).unwrap().radius >= before);
    assert_eq!(s.tree().crossover, 3);
    s.tree.validate_solved().unwrap();
    let mut fresh = s.tree.clone();
    radius::Pipes::default()
        .update(&mut fresh, 100.0, 24.0, s.radii)
        .unwrap();
    s.timeline
        .as_mut()
        .unwrap()
        .pipes
        .update(&mut s.tree, 100.0, 24.0, s.radii)
        .unwrap();
    assert_eq!(
        fresh, s.tree,
        "compaction must retain correct fork reductions"
    );
}

#[test]
fn live_shedding_replays_shoot_state_and_surviving_radii_exactly() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 0.0;
    f.skeleton.habit.shedding_threshold = 0.7;
    f.growth.shedding_tolerance = 0.5;
    let mut a = Specimen::build(&f).unwrap();
    let mut retired = Vec::new();
    for _ in 0..120 {
        let previous = a.tree.clone();
        a.advance(0.1).unwrap();
        for n in previous.nodes {
            if let Ok(live) = a.node(n.identity) {
                assert!(live.radius >= n.radius && live.start_radius >= n.start_radius);
                assert_eq!(live.shoot.birth_year, n.shoot.birth_year);
            } else {
                retired.push(n.identity);
            }
        }
    }
    assert!(!retired.is_empty(), "replay must exercise actual shedding");
    f.age = 12.0;
    let b = Specimen::build(&f).unwrap();
    assert!(a.tree == b.tree, "replayed wood and shoot state differ");
    assert_eq!(a.shed, b.shed);
    assert_eq!(a.next_identity, b.next_identity);
    assert!(retired.iter().all(|&id| a.node(id).is_err()));
    assert!(a
        .tree
        .nodes
        .iter()
        .any(|n| n.shoot.bud_fate == BudFate::Lateral));
    assert!(a.tree.nodes.iter().any(|n| n.shoot.birth_year > 0.0));
}

#[test]
fn tolerance_and_apical_loss_are_validated_blended_numeric_traits() {
    let a = Preset::Ordinary.parameters();
    let mut b = a.clone();
    b.growth.shedding_tolerance = 4.0;
    b.growth.apical_control_loss = 0.2;
    let mid = crate::blend::families(&a, &b, 0.5).unwrap();
    assert_eq!(mid.growth.shedding_tolerance, 3.0);
    assert_eq!(mid.growth.apical_control_loss, 0.1);
    for invalid in [-0.1, f64::NAN, f64::INFINITY] {
        b.growth.shedding_tolerance = invalid;
        assert!(b
            .growth
            .validate()
            .unwrap_err()
            .to_string()
            .contains("sheddingTolerance"));
        b.growth.shedding_tolerance = 2.0;
        b.growth.apical_control_loss = invalid;
        assert!(b
            .growth
            .validate()
            .unwrap_err()
            .to_string()
            .contains("apicalControlLoss"));
        b.growth.apical_control_loss = 0.2;
    }
    let mut f = Preset::NorwaySpruce.parameters();
    f.age = 8.0;
    let young_control = Specimen::build(&f).unwrap();
    f.growth.apical_control_loss = 0.2;
    let lost_control = Specimen::build(&f).unwrap();
    assert!(
        young_control.tree != lost_control.tree,
        "loss must affect growth"
    );
}
