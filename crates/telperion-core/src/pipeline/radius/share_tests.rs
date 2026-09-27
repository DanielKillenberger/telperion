//! fn-170 R5 (fn-103 R1 and R2): wood divides at a fork by role.
use super::*;
use crate::{math::Vec3, tree::Node};

const E: f64 = 2.0;

/// A trunk to a fork, and two children leaving it: `a` and `b` at their
/// positions, `b` carrying on to a tip of its own when `long`.
fn fork(b: Vec3, long: bool, mark: impl Fn(&mut Node)) -> Tree {
    let at = |position: Vec3, parent: u32| Node {
        position,
        parent: Some(parent),
        ..Node::root()
    };
    let mut nodes = vec![
        Node::root(),
        at(Vec3::new(0.0, 4.0, 0.0), 0),
        at(Vec3::new(0.3, 6.0, 0.0), 1),
        at(b, 1),
    ];
    mark(&mut nodes[3]);
    if long {
        nodes.push(at(b + Vec3::new(-0.3, 3.0, 0.0), 3));
        nodes.push(at(b + Vec3::new(-2.0, 1.5, 0.0), 3));
    }
    for (i, n) in nodes.iter_mut().enumerate() {
        n.branch = i as u32;
    }
    let crossover = nodes.len();
    Tree {
        nodes,
        crossover,
        ..Tree::default()
    }
}

fn lateral(n: &mut Node) {
    n.shoot.bud_fate = BudFate::Lateral;
}

fn solved(mut tree: Tree, lateral_share: f64, fork_balance: f64) -> Tree {
    let p = RadiusParams {
        fork_exponent: E,
        lateral_share,
        fork_balance,
        ..RadiusParams::default()
    };
    solve(&mut tree, Envelope::default(), p).unwrap();
    tree
}

/// The pipe model at the fork: the parent's distal radius against the sum
/// of its children's proximal ones.
fn pipes_hold(t: &Tree) {
    let carried = t.nodes[2].start_radius.powf(E) + t.nodes[3].start_radius.powf(E);
    assert!((carried.powf(1.0 / E) - t.nodes[1].radius).abs() < 1e-12);
}

#[test]
fn neutral_shares_are_the_pipe_model_to_the_bit() {
    let b = Vec3::new(-1.5, 5.0, 0.0);
    let plain = solved(fork(b, false, |_| {}), 1.0, 1.0);
    let radii = |t: &Tree| -> Vec<[u64; 2]> {
        let bits = |n: &Node| [n.radius.to_bits(), n.start_radius.to_bits()];
        t.nodes.iter().map(bits).collect()
    };
    for mark in [lateral as fn(&mut Node), |n: &mut Node| n.codominant = Some(1.0)] {
        assert_eq!(radii(&solved(fork(b, false, mark), 1.0, 1.0)), radii(&plain));
    }
    pipes_hold(&plain);
}

#[test]
fn a_lateral_leaves_thinner_by_the_root_of_its_share() {
    let b = Vec3::new(-1.5, 5.0, 0.0);
    let whole = solved(fork(b, false, lateral), 1.0, 1.0);
    let thin = solved(fork(b, false, lateral), 0.3, 1.0);
    let ratio = |t: &Tree| t.nodes[3].start_radius / t.nodes[2].start_radius;
    assert!((ratio(&thin) - ratio(&whole) * 0.3_f64.sqrt()).abs() < 1e-12);
    pipes_hold(&thin);
    // The lateral's whole subtree thins with it: its taper is the one it
    // had. And the trunk stays the authored trunk.
    let taper = |t: &Tree| t.nodes[3].radius / t.nodes[3].start_radius;
    assert!((taper(&thin) - taper(&whole)).abs() < 1e-12);
    assert!((thin.nodes[0].radius - whole.nodes[0].radius).abs() < 1e-12);
}

#[test]
fn below_a_whole_share_a_lateral_leaves_thinner_than_a_continuation_as_large() {
    // Mirror images: the lateral carries as many tips as the axis it leaves.
    let b = Vec3::new(-0.3, 6.0, 0.0);
    for share in [0.2, 0.6, 0.99] {
        let t = solved(fork(b, false, lateral), share, 1.0);
        assert!(t.nodes[3].start_radius < t.nodes[2].start_radius, "{share}");
        pipes_hold(&t);
    }
}

#[test]
fn a_codominant_sibling_leaves_within_the_balance() {
    // Two parts as alike as mirror images: the sibling's wood is the balance
    // of the primary's, times the weight it has grown in by, and none of it
    // as the weight goes to nothing.
    let b = Vec3::new(-0.3, 6.0, 0.0);
    for balance in [1.0, 0.7, 0.4] {
        for weight in [1.0, 0.5, 1e-9] {
            let t = solved(fork(b, false, |n| n.codominant = Some(weight)), 1.0, balance);
            let areas = (t.nodes[3].start_radius / t.nodes[2].start_radius).powf(E);
            assert!((areas - balance * weight).abs() < 1e-12, "{balance} {weight}: {areas}");
            pipes_hold(&t);
            // No child leaves thicker than the wood it leaves.
            assert!(t.nodes[3].start_radius <= t.nodes[1].radius);
        }
    }
}

#[test]
fn a_share_off_its_rail_is_refused_by_name() {
    for (field, p) in [
        ("lateralShare", RadiusParams { lateral_share: 0.0, ..RadiusParams::default() }),
        ("lateralShare", RadiusParams { lateral_share: 0.009, ..RadiusParams::default() }),
        ("lateralShare", RadiusParams { lateral_share: 1.01, ..RadiusParams::default() }),
        ("forkBalance", RadiusParams { fork_balance: f64::NAN, ..RadiusParams::default() }),
    ] {
        match p.resolved() {
            Err(Error::InvalidValue { field: named, .. }) => assert_eq!(named, field),
            other => panic!("{field} gave {other:?}"),
        }
    }
}
