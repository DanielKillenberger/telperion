//! A tree at its node budget (fn-180). It gives up fine twig detail evenly and
//! finishes every axis to its tips, rather than stopping where the count ran
//! out, and a larger budget walks it back toward the tree no budget binds.
//! No device is needed; this is the core's own arithmetic.
use telperion_core::{
    branching,
    presets::{Family, Preset},
    tree::{NodeKind, Tree},
};

/// The oak grows about 125 000 nodes on seed 1, so each of these binds.
const BUDGETS: [usize; 4] = [20_000, 50_000, 80_000, 110_000];

fn oak(budget: Option<usize>) -> Family {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.skeleton.seed = 1;
    f.skeleton.growth.max_nodes = budget;
    f
}

fn grow(f: &Family) -> Tree {
    branching::generate(&f.skeleton, f.radii).unwrap().tree
}

/// Local axes that end on their own wood, with no twig beyond it.
fn stubs(tree: &Tree) -> usize {
    let mut children = vec![0; tree.nodes.len()];
    for n in &tree.nodes[1..] {
        children[n.parent.unwrap() as usize] += 1;
    }
    (0..tree.nodes.len())
        .filter(|&i| tree.nodes[i].kind == NodeKind::Branch && children[i] == 0)
        .count()
}

#[test]
fn a_tree_at_its_budget_finishes_every_axis() {
    // The tree no budget binds has a few axes that end short on their own,
    // where a twig was refused; a budget must add none.
    let whole = stubs(&grow(&oak(None)));
    for budget in BUDGETS {
        let f = oak(Some(budget));
        let tree = grow(&f);
        let d = tree.diagnostics;
        assert!(
            tree.nodes.len() <= budget,
            "{budget}: {} nodes",
            tree.nodes.len()
        );
        assert!(
            d.reduced() && d.complete() && d.incomplete().is_none(),
            "{budget}: {d:?}"
        );
        assert!(
            stubs(&tree) <= whole,
            "{budget}: {} axes stop short, {whole} without a budget",
            stubs(&tree)
        );
        assert_eq!(tree, grow(&f), "{budget}: a second build reduced otherwise");
    }
}

#[test]
fn a_larger_budget_walks_toward_the_unbudgeted_tree() {
    let mut last = (0, 0);
    for budget in BUDGETS {
        let tree = grow(&oak(Some(budget)));
        let next = (tree.diagnostics.twig_detail.unwrap(), tree.nodes.len());
        assert!(
            next.0 >= last.0 && next.1 > last.1,
            "{budget}: {next:?} after {last:?}"
        );
        last = next;
    }
    let whole = grow(&oak(None));
    assert_eq!(whole.diagnostics, Default::default());
    assert!(whole.nodes.len() > last.1);
}

#[test]
fn a_budget_the_tree_cannot_meet_leaves_it_incomplete_and_says_why() {
    // Just above the oak's framework (2 750 nodes), even a tree with no
    // lateral twig is over the budget; below it, the framework itself is.
    for (budget, why) in [
        (3_000, "no twig detail fits the node budget"),
        (1_000, "the framework alone exceeds the node budget"),
    ] {
        let d = grow(&oak(Some(budget))).diagnostics;
        assert!(d.node_capped && !d.reduced() && !d.complete(), "{d:?}");
        assert_eq!(d.incomplete(), Some(why), "{budget}");
    }
}
