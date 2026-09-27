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
    // A codominant sibling parting at station 5: an axis of its own.
    let mut part = at(Vec3::new(-0.5, 5.8, 0.0), 5);
    part.codominant = Some(0.8);
    nodes.push(part);
    for k in 2..=4 {
        nodes.push(at(
            Vec3::new(-0.5 * k as f64, 5.0 + 0.8 * k as f64, 0.0),
            nodes.len() - 1,
        ));
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

/// The default length taper, so an axis's first node ends thinner than it
/// starts.
fn params(girth_hold: f64, girth_fall: f64) -> RadiusParams {
    RadiusParams {
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
    let base = pipe.nodes[1].start_radius;
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

/// Walking either row in small steps moves every radius, start and distal,
/// by a small step, from the pipe model at zero hold onwards: on a tapering
/// axis, at its laterals and at a codominant sibling.
#[test]
fn walking_the_rows_moves_the_profile_continuously() {
    let largest = |a: &Tree, b: &Tree| {
        a.nodes
            .iter()
            .zip(&b.nodes)
            .flat_map(|(a, b)| [a.radius / b.radius, a.start_radius / b.start_radius])
            .map(|ratio| ratio.ln().abs())
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
    walk(&|s| (1e-6 * s, 2.0), 10);
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

/// A fork at the root, a primary and a codominant sibling: the root is a
/// point, so both parts start axes of their own, and a hold just above zero
/// leaves every radius, start and distal, the pipe model's.
#[test]
fn a_fork_at_the_root_rises_from_zero_with_no_jump() {
    let mut nodes = vec![Node::root()];
    for side in [1.0, -1.0] {
        for k in 1..=4 {
            let parent = if k == 1 { 0 } else { nodes.len() - 1 };
            nodes.push(Node {
                position: Vec3::new(side * 0.3 * k as f64, k as f64, 0.0),
                parent: Some(parent as u32),
                codominant: (side < 0.0 && k == 1).then_some(1.0),
                ..Node::root()
            });
        }
    }
    for (i, n) in nodes.iter_mut().enumerate() {
        n.branch = i as u32;
    }
    let crossover = nodes.len();
    let mut pipe = Tree {
        nodes,
        crossover,
        ..Tree::default()
    };
    solve(&mut pipe, Envelope::default(), params(0.0, 2.0)).unwrap();
    for hold_share in [1e-9, 1e-3] {
        let mut t = pipe.clone();
        hold(&mut t, params(hold_share, 2.0));
        for (a, b) in t.nodes.iter().zip(&pipe.nodes) {
            assert_eq!([a.start_radius, a.radius], [b.start_radius, b.radius]);
        }
    }
    let mut t = pipe.clone();
    hold(&mut t, params(0.5, 2.0));
    assert_eq!(t.nodes[2].start_radius, pipe.nodes[1].start_radius);
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

/// fn-177's boundary: a hold adds or loses no twig or leaf. Wood bears leaves
/// by the pipe model's radii, here where the canopy clothes slender
/// structural wood and bears short shoots along it. A leaf still sits on the
/// wood as drawn, so the envelope's interior cull, which reads where a leaf
/// is, may keep a handful fewer or more.
#[test]
fn a_hold_adds_or_loses_no_leaf() {
    let placed = |girth_hold: f64| {
        let mut f = crate::presets::Preset::Ordinary.parameters();
        f.skeleton.seed = 1;
        f.canopy.shoot_radius = 0.2;
        f.canopy.short_shoot_spacing = 0.5;
        f.radii.girth_hold = girth_hold;
        f.radii.girth_fall = 4.0;
        let built = crate::pipeline::build(&f, crate::pipeline::Request::mesh()).unwrap();
        let leaves = built.outputs.leaves.unwrap();
        (built.skeleton.tree.nodes.len(), leaves.placed)
    };
    assert_eq!(placed(0.75), placed(0.0));
}
