//! The lineage order's one identity check (fn-198 task 6, host decision 13).
use super::{distinct, identities};
use crate::fine::tests::request;
use crate::grow::grow;
use crate::structure::{Origin, Structure};

/// A young beech with one lateral grown twice: two axes of one lineage.
fn twinned() -> Structure {
    let mut tree = grow(&crate::beech(), request(6, 1)).expect("grows");
    let lateral = (tree.axes.iter())
        .position(|a| matches!(a.origin, Origin::Lateral { .. }))
        .expect("a lateral");
    let twin = tree.axes[lateral].clone();
    tree.axes.push(twin);
    tree
}

#[test]
fn a_grown_tree_holds_each_lineage_once_and_a_twin_is_seen() {
    let tree = grow(&crate::beech(), request(6, 1)).expect("grows");
    let ids = identities(&tree);
    let keys: Vec<(u64, u64)> = tree
        .axes
        .iter()
        .zip(&ids)
        .map(|(a, &i)| (a.lineage, i))
        .collect();
    let mut all: Vec<usize> = (0..keys.len()).collect();
    all.sort_by_key(|&i| keys[i]);
    assert!(distinct(&all, &keys), "a grown tree repeats a lineage");
    let twinned = twinned();
    let ids = identities(&twinned);
    let keys: Vec<(u64, u64)> = (twinned.axes.iter().zip(&ids))
        .map(|(a, &i)| (a.lineage, i))
        .collect();
    let mut all: Vec<usize> = (0..keys.len()).collect();
    all.sort_by_key(|&i| keys[i]);
    assert!(!distinct(&all, &keys), "the twin was not seen");
}

#[test]
#[cfg(debug_assertions)]
#[should_panic(expected = "two axes share a lineage")]
fn the_lineage_order_asserts_no_two_axes_share_a_lineage() {
    lineage_order(twinned());
}
