//! The carbon balance on hand-built trees (fn-349 R4): dead wood is
//! charged nothing, a living tip's charge rises with the square of its
//! height (fn-363), and a part earns what its paying child laterals have
//! left over.
use super::super::bud;
use super::passes::{charge, charged};
use super::Working;
use crate::allocation::{Link, TIP};
use crate::contract::shed::{walk, Bud, Lateral};
use crate::lineage::Key;
use crate::structure::{Axis, Growing, Origin, Vec3};

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

/// A charge is a tip's size times the square of its height over the
/// potential height: a seed tip at the root collar nothing, a lateral tip
/// at half the height a quarter of its size, one at the full height its
/// whole size, and none where the species has no height.
#[test]
fn a_charge_is_the_square_of_the_height() {
    let inverse = 1.0 / 32.0;
    assert_eq!(charge(0.5, 0.0, inverse), 0.0);
    assert_eq!(charge(0.5, 16.0, inverse), 0.125);
    assert_eq!(charge(0.5, 32.0, inverse), 0.5);
    assert_eq!(charge(0.5, 32.0, 0.0), 0.0);
    // A seed tip and two lateral tips on it, 8 m and 16 m up, each lit
    // enough to pay: the seed, charged nothing itself, earns its own light
    // and what each has left over.
    let heights = [0.0, 8.0, 16.0];
    let own: Vec<(f32, f32)> = heights
        .iter()
        .map(|&l| (1.0, charge(1.0, l, inverse)))
        .collect();
    let mut w = working(&[(None, 0), (Some(0), 1), (Some(0), 3)], &[]);
    totals(&mut w, &own);
    assert_eq!(own[2].1, 4.0 * own[1].1);
    assert_eq!(w.cost[0], 0.0);
    assert_eq!(w.light[0], (1.0 + (1.0 - own[1].1)) + (1.0 - own[2].1));
}

/// A seed whose collar stands at `collar` and a lateral on it, born at
/// the seed's first node, its nodes at `tips`.
pub(super) fn lateral(collar: f32, tips: &[[f32; 3]]) -> Vec<Axis<Growing>> {
    let laid = |at: &[f32; 3]| Growing {
        tip: *at,
        ..Growing::new(1, 0)
    };
    let mut seed = bud(Key(1), 0, 0, Origin::Seed);
    seed.base = Vec3::new(0.0, 0.0, f64::from(collar));
    seed.phytomers.push(laid(&[0.0, 0.0, collar + 4.0]));
    let born = Origin::Lateral {
        parent: 0,
        node: 0,
        slot: 0,
        whorl: 0.0,
        woken: false,
    };
    let mut side = bud(Key(2), 1, 1, born);
    side.base = Vec3::new(0.0, 0.0, f64::from(collar) + 4.0);
    side.phytomers.extend(tips.iter().map(laid));
    vec![seed, side]
}

/// fn-363 R2: a living tip is charged its size times the square of its
/// height above the root collar's base over the potential height, the
/// same whatever length of wood reaches it: a lateral 8 m above the
/// collar, straight or wandering out and back, is charged a sixteenth of
/// its size at a potential height of 32 m.
#[test]
fn a_tip_is_charged_the_square_of_its_height_whatever_its_path() {
    let inverse = 1.0 / 32.0;
    let straight = lateral(0.25, &[[2.0, 0.0, 8.25]]);
    let wandering = lateral(
        0.25,
        &[
            [6.0, 0.0, 6.25],
            [6.0, 5.0, 10.25],
            [-3.0, 4.0, 9.25],
            [1.0, 3.0, 8.25],
        ],
    );
    assert_eq!(charged(0.5, &straight, 1, inverse), 0.5 / 16.0);
    assert_eq!(charged(0.5, &wandering, 1, inverse), 0.5 / 16.0);
    // A tip that has laid no node stands at its base, 4 m up.
    assert_eq!(charged(0.5, &lateral(0.25, &[]), 1, inverse), 0.5 / 64.0);
}

/// fn-363 R2: a tip below the root collar's base is charged nothing, and
/// so is every tip of a species with no potential height.
#[test]
fn a_tip_below_the_collar_or_without_a_height_is_charged_nothing() {
    let below = lateral(0.25, &[[3.0, 0.0, 2.0], [5.0, 0.0, -0.5]]);
    assert_eq!(charged(0.5, &below, 1, 1.0 / 32.0), 0.0);
    let high = lateral(0.25, &[[2.0, 0.0, 8.25]]);
    assert_eq!(charged(0.5, &high, 1, 0.0), 0.0);
}

/// A bough with one paying and one failing child lateral earns what the
/// paying one has left over and nothing from the failing one, and is kept,
/// while the failing child is judged alone and shed; a continuation that
/// fails is its bearer's own line, counted whole, and tested as a lateral
/// is.
#[test]
fn a_part_earns_what_its_paying_children_leave_over() {
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
    assert_eq!((w.light[1], w.cost[1]), ((0.2 + 0.5) + 0.0, 0.1));
    let bough = w.light[1] - w.cost[1];
    assert_eq!((w.light[0], w.cost[0]), ((0.0 + bough) + 0.1, 1.0));
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

/// A child lateral crossing break-even moves its bearer by degree: just
/// below, at and just above, the bearer earns within a hair of the same.
#[test]
fn a_child_at_break_even_moves_its_bearer_by_degree() {
    let links = [(None, 0), (Some(0), 1)];
    let earned = |light: f32| {
        let mut w = working(&links, &[]);
        totals(&mut w, &[(0.5, 0.25), (light, 1.0)]);
        w.light[0]
    };
    let at = earned(1.0);
    assert_eq!(at, 0.5);
    for light in [1.0 - 1e-6, 1.0 + 1e-6] {
        assert!((earned(light) - at).abs() <= 2e-6, "{light}");
    }
}
