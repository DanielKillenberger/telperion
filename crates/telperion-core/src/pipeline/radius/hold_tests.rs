//! fn-177 R3: a synthetic limb holds its girth over `girthHold` of its reach
//! and falls to the pipe model's radius by its tip.
use super::*;
use crate::{math::Vec3, tree::Node};

const STATIONS: usize = 24;

/// A straight axis of `STATIONS` nodes a metre apart, each station between
/// the root and the tip bearing a short lateral of two nodes, so the pipe
/// thins up the axis and a station's share of the reach is its height's;
/// with `whorl` the tip bears a long lateral too, reaching 12 m on, as a
/// leader ending in laterals does.
fn limb(whorl: bool) -> Tree {
    let at = |position: Vec3, parent: usize| Node {
        position,
        parent: Some(parent as u32),
        ..Node::root()
    };
    let mut nodes = vec![Node::root()];
    let mut axis = vec![0];
    for k in 1..STATIONS {
        nodes.push(at(Vec3::new(0.0, k as f64, 0.0), *axis.last().unwrap()));
        axis.push(nodes.len() - 1);
    }
    for &station in &axis[1..STATIONS - 1] {
        let y = nodes[station].position.y;
        let mut first = at(Vec3::new(0.4, y + 0.1, 0.0), station);
        first.shoot.bud_fate = BudFate::Lateral;
        nodes.push(first);
        nodes.push(at(Vec3::new(0.8, y + 0.2, 0.0), nodes.len() - 1));
    }
    if whorl {
        let tip = nodes[STATIONS - 1].position;
        for k in 1..=4 {
            let parent = if k == 1 {
                STATIONS - 1
            } else {
                nodes.len() - 1
            };
            let mut next = at(tip + Vec3::new(3.0 * k as f64, 0.0, 0.0), parent);
            next.shoot.bud_fate = [BudFate::Terminal, BudFate::Lateral][usize::from(k == 1)];
            nodes.push(next);
        }
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

fn params(girth_hold: f64, girth_fall: f64) -> RadiusParams {
    RadiusParams {
        length_taper: 0.0,
        girth_hold,
        girth_fall,
        ..RadiusParams::default()
    }
}

fn pipe() -> Tree {
    let mut tree = limb(false);
    solve(&mut tree, Envelope::default(), params(0.0, 2.0)).unwrap();
    tree
}

fn held(girth_hold: f64, girth_fall: f64) -> Tree {
    let mut tree = pipe();
    hold(&mut tree, params(girth_hold, girth_fall));
    tree
}

/// A solved tree's invariants: no node starts thinner than it ends, and no
/// part starts thicker than its parent at the junction.
fn junctions_hold(t: &Tree) {
    t.validate_solved().unwrap();
    for n in &t.nodes[1..] {
        assert!(n.start_radius >= n.radius);
        assert!(n.start_radius <= t.nodes[n.parent.unwrap() as usize].radius);
    }
}

#[test]
fn zero_hold_is_the_pipe_model_to_the_bit() {
    let bits = |t: &Tree| -> Vec<[u64; 2]> {
        let bits = |n: &Node| [n.radius.to_bits(), n.start_radius.to_bits()];
        t.nodes.iter().map(bits).collect()
    };
    for fall in [0.5, 2.0, 8.0] {
        assert_eq!(bits(&held(0.0, fall)), bits(&pipe()));
    }
}

/// Over the hold the axis keeps its base girth to within 1e-9 of it; past
/// the fall, and at the tip, it is the pipe model's radius.
#[test]
fn a_limb_holds_its_girth_then_falls_to_the_pipe_at_its_tip() {
    let (pipe, hold, fall) = (pipe(), 0.6, 2.0);
    let t = held(hold, fall);
    let base = pipe.nodes[0].start_radius;
    let tip = pipe.nodes[STATIONS - 1].radius;
    assert!(
        tip < 0.3 * base,
        "the synthetic limb thins: {tip} of {base}"
    );
    let end = hold * (1.0 + 1.0 / fall);
    for k in 0..STATIONS {
        let share = k as f64 / (STATIONS - 1) as f64;
        let r = t.nodes[k].radius;
        if share <= hold {
            assert!(
                (r / base - 1.0).abs() < 1e-9,
                "station {k}: {r} against {base}"
            );
        } else if share >= end {
            assert_eq!(r, pipe.nodes[k].radius, "station {k}");
        } else {
            assert!(
                r < t.nodes[k - 1].radius && r > pipe.nodes[k].radius,
                "station {k}"
            );
        }
    }
    assert_eq!(t.nodes[STATIONS - 1].radius, tip);
    junctions_hold(&t);
}

/// Walking either row in small steps moves every radius by a small step,
/// from the pipe model at zero hold onwards.
#[test]
fn walking_the_rows_moves_the_profile_continuously() {
    let largest = |a: &Tree, b: &Tree| {
        a.nodes
            .iter()
            .zip(&b.nodes)
            .map(|(a, b)| (a.radius / b.radius).ln().abs())
            .fold(0.0, f64::max)
    };
    let walk = |rows: &dyn Fn(f64) -> (f64, f64), steps: usize| {
        let mut before = held(rows(0.0).0, rows(0.0).1);
        for k in 1..=steps {
            let (h, f) = rows(k as f64 / steps as f64);
            let now = held(h, f);
            let step = largest(&now, &before);
            assert!(step < 0.01, "{h} {f}: a step of {step}");
            junctions_hold(&now);
            before = now;
        }
    };
    walk(&|s| (0.9 * s, 2.0), 9000);
    walk(&|s| (0.9 * s, 8.0), 9000);
    walk(&|s| (0.6, 0.5 + 7.5 * s), 7500);
}

/// An axis's reach runs along its own continuation: a leader ending in a
/// whorl of laterals falls to the pipe radius at its tip rather than holding
/// its girth there because its laterals reach on.
#[test]
fn a_leader_ending_in_laterals_is_not_left_blunt() {
    let mut pipe = limb(true);
    solve(&mut pipe, Envelope::default(), params(0.0, 2.0)).unwrap();
    let mut t = pipe.clone();
    hold(&mut t, params(0.6, 2.0));
    let tip = STATIONS - 1;
    assert_eq!(t.nodes[tip].radius, pipe.nodes[tip].radius);
    assert!(t.nodes[tip].radius < 0.5 * t.nodes[0].radius);
    junctions_hold(&t);
}

#[test]
fn the_rows_are_refused_off_their_rails() {
    for (field, p) in [
        ("girthHold", params(-0.1, 2.0)),
        ("girthHold", params(0.95, 2.0)),
        ("girthHold", params(f64::NAN, 2.0)),
        ("girthFall", params(0.5, 0.4)),
        ("girthFall", params(0.5, 9.0)),
        ("girthFall", params(0.5, f64::NAN)),
    ] {
        match p.resolved() {
            Err(Error::InvalidValue { field: named, .. }) => assert_eq!(named, field),
            other => panic!("{field} gave {other:?}"),
        }
    }
}
