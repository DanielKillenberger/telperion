//! The carbon balance on hand-built trees (fn-349 R4): dead wood is
//! charged nothing, a living tip's charge rises with the square of its
//! path, and a subtree counts only the child laterals that pay.
use super::passes::charge;
use super::Working;
use crate::allocation::{Link, TIP};
use crate::contract::shed::{walk, Bud, Lateral};

/// A working set of `links`, each axis open but those in `closed`, its
/// children by node then index as `Working::extend` sets them.
fn working(links: &[(Option<usize>, usize)], closed: &[usize]) -> Working {
    let n = links.len();
    let mut w = Working::default();
    w.links = links
        .iter()
        .map(|&(parent, node)| Link { parent, node })
        .collect();
    w.by_node = vec![Vec::new(); n];
    for (i, &(parent, node)) in links.iter().enumerate() {
        if let Some(p) = parent {
            let kids = &w.by_node[p];
            let at = kids.partition_point(|&c| links[c as usize].1 <= node);
            w.by_node[p].insert(at, i as u32);
        }
    }
    w.open = (0..n).map(|i| !closed.contains(&i)).collect();
    w.list = (0..n).filter(|&i| w.open[i]).collect();
    (w.light, w.cost, w.living) = (vec![0.0; n], vec![0.0; n], vec![false; n]);
    w
}

/// Each axis's own light and charge, every one a living tip.
fn totals(w: &mut Working, own: &[(f32, f32)]) {
    w.totals(&|i| (own[i].0, own[i].1, true));
}

/// A limb whose twigs have died is charged what the same limb without
/// them is charged (goal fn-347 R1): its dead twigs, closed, add nothing,
/// whatever they were charged while they lived.
#[test]
fn a_limb_is_charged_nothing_for_its_dead_twigs() {
    let own = [(1.0, 0.5), (0.8, 0.3), (0.2, 0.9), (0.1, 0.7)];
    let mut with = working(
        &[(None, 0), (Some(0), 2), (Some(1), 0), (Some(1), 1)],
        &[2, 3],
    );
    totals(&mut with, &own);
    let mut without = working(&[(None, 0), (Some(0), 2)], &[]);
    totals(&mut without, &own[..2]);
    assert_eq!(
        (with.light[1], with.cost[1]),
        (without.light[1], without.cost[1])
    );
    assert_eq!(
        (with.light[0], with.cost[0]),
        (without.light[0], without.cost[0])
    );
}

/// A living tip is charged its size times the square of its path over the
/// potential height: a seed tip at the root collar nothing, a lateral tip
/// at half the height a quarter of its size, one at the full height its
/// whole size, and none where the species has no height.
#[test]
fn a_tip_is_charged_the_square_of_its_path() {
    let inverse = 1.0 / 32.0;
    assert_eq!(charge(0.5, 0.0, inverse), 0.0);
    assert_eq!(charge(0.5, 16.0, inverse), 0.125);
    assert_eq!(charge(0.5, 32.0, inverse), 0.5);
    assert_eq!(charge(0.5, 32.0, 0.0), 0.0);
    // A seed tip and two lateral tips on it, at 8 m and 16 m of path, each
    // lit enough to pay: the seed's subtree is charged their sum.
    let paths = [0.0, 8.0, 16.0];
    let own: Vec<(f32, f32)> = paths
        .iter()
        .map(|&l| (1.0, charge(1.0, l, inverse)))
        .collect();
    let mut w = working(&[(None, 0), (Some(0), 1), (Some(0), 3)], &[]);
    totals(&mut w, &own);
    assert_eq!(own[2].1, 4.0 * own[1].1);
    assert_eq!(w.cost[0], own[1].1 + own[2].1);
}

/// A bough with one paying and one failing child lateral counts only the
/// one that pays, and is kept, while the failing child is judged alone and
/// shed; a continuation that fails is its bearer's own line and is
/// always counted.
#[test]
fn a_subtree_counts_only_the_children_that_pay() {
    // 0 the seed; 1 a bough on it; 2 and 3 its paying and failing child
    // laterals; 4 a failing continuation of the seed.
    let links = [
        (None, 0),
        (Some(0), 1),
        (Some(1), 0),
        (Some(1), 1),
        (Some(0), TIP),
    ];
    let own = [(0.0, 0.0), (0.2, 0.1), (1.0, 0.5), (0.1, 1.0), (0.1, 1.0)];
    let mut w = working(&links, &[]);
    totals(&mut w, &own);
    assert_eq!((w.light[1], w.cost[1]), (0.2 + 1.0, 0.1 + 0.5));
    assert_eq!(
        (w.light[0], w.cost[0]),
        ((0.2 + 1.0) + 0.1, (0.1 + 0.5) + 1.0)
    );
    let bud = |i: usize| Bud {
        q: w.light[i],
        c: w.cost[i],
        memory: 0.0,
        hazard: 100.0,
        tolerance: 0.0,
        draw: 1 << 52,
        window: 1.0,
        keep: 1.0,
    };
    let axes: Vec<Lateral> = (0..links.len())
        .map(|i| Lateral {
            parent: links[i].0.map(|p| p as u32),
            decides: links[i].0.is_some(),
            bud: bud(i),
        })
        .collect();
    let shed: Vec<bool> = walk(&axes, None).iter().map(|w| w.shed).collect();
    assert_eq!(shed, [false, false, false, true, true]);
}
