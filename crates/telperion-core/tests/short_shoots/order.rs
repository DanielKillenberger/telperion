//! The same wood draws the same short shoots however it was stored, grown or
//! reached: another storage order, a month-by-month replay, the growth view,
//! and every step of a blend from none.
use super::*;
use telperion_core::blend;

/// The same wood stored in another order: children visited last-first.
fn reordered(tree: &Tree) -> Tree {
    let count = tree.nodes.len();
    let mut children = vec![Vec::new(); count];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        children[n.parent.unwrap() as usize].push(i);
    }
    let (mut order, mut stack) = (Vec::new(), vec![0]);
    while let Some(i) = stack.pop() {
        order.push(i);
        stack.extend(&children[i]);
    }
    let mut index = vec![0; count];
    for (new, &old) in order.iter().enumerate() {
        index[old] = new as u32;
    }
    let nodes = order
        .iter()
        .map(|&old| {
            let mut n = tree.nodes[old].clone();
            n.parent = n.parent.map(|p| index[p as usize]);
            n.branch = index[n.branch as usize];
            n
        })
        .collect();
    Tree {
        nodes,
        ..tree.clone()
    }
}

#[test]
fn the_same_wood_draws_the_same_short_shoots_in_any_storage_order() {
    for seed in SEEDS {
        let f = beech(seed);
        let tree = grown(&f);
        let other = reordered(&tree);
        other.validate_solved().unwrap();
        assert_ne!(
            other.nodes, tree.nodes,
            "seed {seed}: the order did not change"
        );
        let (e, p) = (f.skeleton.envelope, f.canopy);
        let box_of = foliage::Reference::of(&f).unwrap();
        let (mut a, mut b) = (Instances::new(box_of), Instances::new(box_of));
        foliage::place_short_shoots(&tree, e, seed, p, &mut a).unwrap();
        foliage::place_short_shoots(&other, e, seed, p, &mut b).unwrap();
        assert!(!a.is_empty());
        assert_eq!(
            bytes(&a),
            bytes(&b),
            "seed {seed}: storage order moved a leaf"
        );
    }
}

#[test]
fn a_monthly_replay_and_a_growth_view_draw_what_a_fresh_build_draws() {
    let mut f = beech(1);
    f.age = 14.0;
    let fresh = branching::Specimen::build(&f).unwrap().read().unwrap();
    let box_of = foliage::Reference::of(&f).unwrap();
    let draw = |tree: &Tree, envelope| {
        let mut out = Instances::new(box_of);
        foliage::place_short_shoots(tree, envelope, 1, f.canopy, &mut out).unwrap();
        bytes(&out)
    };
    let expected = draw(&fresh.tree, fresh.envelope);
    assert!(!expected.is_empty(), "the test must grow short shoots");
    // Month by month to the same age: the same wood, the same draws.
    let mut monthly = f.clone();
    monthly.age = 0.0;
    let mut replay = branching::Specimen::build(&monthly).unwrap();
    for _ in 0..14 * 12 {
        replay.advance(1.0 / 12.0).unwrap();
    }
    let read = replay.read().unwrap();
    assert_eq!(
        draw(&read.tree, read.envelope),
        expected,
        "a monthly replay"
    );
    // The growth view rebuilds its tree in its own order and hangs the same
    // clusters after the record's leaves.
    let view = telperion_core::specimen::SpecimenView::build(&f).unwrap();
    let element = foliage::build_element(f.element).unwrap();
    let mut placements = Instances {
        leaves: fresh.placements.iter().map(|p| p.leaf).collect(),
        reference: box_of,
        thinned: 0,
    };
    foliage::place_short_shoots(&fresh.tree, fresh.envelope, 1, f.canopy, &mut placements).unwrap();
    let culled = foliage::cull(placements, &element, fresh.envelope, f.shell_depth).unwrap();
    assert_eq!(
        view.mesh().unwrap().foliage.instances,
        culled,
        "the growth view"
    );
}

#[test]
fn a_walk_from_none_thins_in_and_closes_continuously() {
    let to = beech(3);
    let from = Family {
        canopy: none(to.canopy),
        ..to.clone()
    };
    let tree = grown(&to);
    let e = to.skeleton.envelope;
    let full = foliage::short_shoots(&tree, e, 3, to.canopy).unwrap().len() as f64;
    let mut last = 0.0;
    for step in 0..=10 {
        let t = f64::from(step) / 10.0;
        let walked = blend::families(&from, &to, t).unwrap();
        assert_eq!(grown(&walked), tree, "step {step}: the walk moved the wood");
        let spacing = walked.canopy.short_shoot_spacing;
        assert!(
            step == 0 || spacing >= to.canopy.short_shoot_spacing,
            "step {step}"
        );
        let count = foliage::short_shoots(&tree, e, 3, walked.canopy)
            .unwrap()
            .len() as f64;
        // Density walks linearly, so the count does too: no frame where the
        // wood is suddenly crowded, and none where it is suddenly bare.
        assert!(count >= last, "step {step}: the walk thinned");
        assert!(
            (count - t * full).abs() <= 0.03 * full + 50.0,
            "step {step}: {count}"
        );
        last = count;
    }
    assert_eq!(last, full);
}
