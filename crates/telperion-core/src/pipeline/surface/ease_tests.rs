//! fn-360: one rule for every drop along a run (host). Each fall of the
//! wood lays a hold that fades over twice the radius it fell from, and the
//! holds add, so two forks close together draw no ledge, a stem that steps
//! every growth unit draws no ring, and a stem that thins steadily is drawn
//! wider than its radius by about half its thinning over that reach. Each
//! girth reads only the run's radii and distances up to it.
use super::{paths::paths, samples::sample_path, Sample, SurfaceParams};
use crate::{
    math::Vec3,
    tree::{Node, Tree},
};

/// Node spacing along the stem.
const STEP: f64 = 0.05;

/// An upright stem of one node a `STEP`, each node's radius read off its
/// height, and a codominant sibling of `sibling` radius leaning out of each
/// node of `forks`.
fn stem(length: f64, radius: impl Fn(f64) -> f64, forks: &[(usize, f64)]) -> Tree {
    let mut nodes = vec![Node {
        radius: radius(0.0),
        start_radius: radius(0.0),
        ..Node::root()
    }];
    let push = |nodes: &mut Vec<Node>, parent: usize, at: Vec3, r: f64| {
        let i = nodes.len();
        nodes.push(Node {
            position: at,
            parent: Some(parent as u32),
            radius: r,
            start_radius: r,
            base_radius: r,
            branch: i as u32,
            stem: true,
            ..Node::root()
        });
        i
    };
    let count = (length / STEP).round() as usize;
    for k in 1..=count {
        let y = STEP * k as f64;
        push(&mut nodes, k - 1, Vec3::Y * y, radius(y));
    }
    let lean = Vec3::new(25_f64.to_radians().sin(), 25_f64.to_radians().cos(), 0.0);
    for &(at, r) in forks {
        let mut side = at;
        for k in 0..10 {
            let to = nodes[at].position + lean * (STEP * (k + 1) as f64);
            side = push(&mut nodes, side, to, r);
            if k == 0 {
                nodes[side].codominant = Some(1.0);
            }
        }
    }
    Tree {
        crossover: nodes.len(),
        nodes,
        ..Tree::default()
    }
}

/// The trunk run's rings, one a node from the root, on a surface with no
/// flare, so the radii read are the girths themselves.
fn trunk(tree: &Tree) -> Vec<Sample> {
    let paths = paths(tree).unwrap();
    let mut distance = vec![0.0; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let p = n.parent.unwrap() as usize;
        distance[i] = distance[p] + tree.nodes[p].position.distance(n.position);
    }
    let params = SurfaceParams {
        flare_radius: 1.0,
        ..SurfaceParams::default()
    };
    let run = paths.runs.iter().find(|r| r.trunk).unwrap();
    let mut samples = Vec::new();
    sample_path(tree, 18.0, &params, &paths, run, &distance, &mut samples);
    samples.split_off(samples.len() - (run.end - run.start))
}

/// Two forks 0.3 m apart on a 0.64 m stem: the trunk eases into the stem
/// above both with no ring-to-ring slope near the 1-in-1 floor, and its
/// girth only ever falls over the first fork's ease.
#[test]
fn two_forks_close_together_draw_no_ledge() {
    let radius = |y: f64| match y {
        y if y <= 1.0 + 1e-9 => 0.64,
        y if y <= 1.3 + 1e-9 => 0.47,
        _ => 0.34,
    };
    let tree = stem(3.5, radius, &[(20, 0.38), (26, 0.33)]);
    let rings = trunk(&tree);
    let fork = rings.iter().position(|s| (s.p.y - 1.0).abs() < 1e-9).unwrap();
    let mut steepest = 0.0_f64;
    for pair in rings.windows(2) {
        steepest = steepest.max((pair[0].r - pair[1].r) / (pair[1].d - pair[0].d));
    }
    assert!(steepest < 0.9, "the steepest slope is {steepest}");
    let ease = 2.0 * 0.64;
    for pair in rings[fork..].windows(2) {
        if pair[1].d - rings[fork].d <= ease {
            let (a, b) = (pair[0].r, pair[1].r);
            assert!(b <= a, "it rises at {}: {a} to {b}", pair[1].d);
        }
    }
    assert_eq!(rings.last().unwrap().r, 0.34);
}

/// A stem that steps down 5% every 0.8 m, as a growth unit does, draws no
/// ring falling more than 1% from the one before.
#[test]
fn a_stem_stepping_every_growth_unit_draws_no_rings() {
    let tree = stem(6.0, |y| 0.4 * 0.95_f64.powi((y / 0.8 + 1e-9) as i32), &[]);
    let rings = trunk(&tree);
    for pair in rings.windows(2) {
        let fall = 1.0 - pair[1].r / pair[0].r;
        assert!(fall <= 0.01, "{fall} at {}", pair[1].d);
    }
}

/// A run whose radius never drops is drawn at its radii, to the bit.
#[test]
fn a_run_that_never_thins_is_drawn_at_its_radii() {
    let radius = |y: f64| 0.3 + 0.02 * (y / 0.5 + 1e-9).floor();
    let tree = stem(3.0, radius, &[]);
    let rings = trunk(&tree);
    for (s, n) in rings.iter().zip(&tree.nodes) {
        assert_eq!(s.r, n.radius, "at {}", s.d);
    }
}

/// A run thinning 0.05% a ring over 200 rings is drawn over its radii by
/// at most its thinning over the reach upstream of each ring, since each
/// hold is at most its fall, and where a full reach of wood lies upstream
/// by at most half that thinning plus one ring's drop.
#[test]
fn a_stem_thinning_steadily_is_drawn_by_half_its_thinning() {
    let tree = stem(10.0, |y| 0.3 * 0.9995_f64.powi((y / STEP).round() as i32), &[]);
    let rings = trunk(&tree);
    let r: Vec<f64> = tree.nodes.iter().map(|n| n.radius).collect();
    assert!(rings.len() > 200);
    for k in 1..rings.len() {
        let first = (0..=k)
            .find(|&i| rings[k].d - rings[i].d < 2.0 * r[i])
            .unwrap();
        let thinning = r[first] - r[k];
        let bound = if first > 0 {
            0.5 * thinning + (r[k - 1] - r[k])
        } else {
            thinning
        } + 1e-6;
        let over = rings[k].r - r[k];
        assert!((0.0..=bound).contains(&over), "{over} over, bound {bound}, at {}", rings[k].d);
    }
}

/// The girth at a ring is the same drawn with the whole run or with the
/// run cut off at that ring: nothing downstream or carried shapes it.
#[test]
fn a_rings_girth_reads_only_the_wood_up_to_it() {
    let radius = |y: f64| match y {
        y if y <= 1.0 + 1e-9 => 0.64,
        y if y <= 1.3 + 1e-9 => 0.47,
        _ => 0.34 * 0.97_f64.powi((y / 0.4) as i32),
    };
    let whole = stem(3.5, radius, &[]);
    let rings = trunk(&whole);
    for k in 1..whole.nodes.len() {
        let cut = Tree {
            crossover: k + 1,
            nodes: whole.nodes[..=k].to_vec(),
            ..Tree::default()
        };
        assert_eq!(trunk(&cut).last().unwrap().r, rings[k].r, "at ring {k}");
    }
}

/// A run of 0.2 m that carries on into a 3 mm shoot 0.2 m long ends at the
/// shoot's radius and narrows all along it: a hold fades over what is left
/// of the run when that is shorter than twice the radius it fell from.
#[test]
fn a_run_ends_at_its_own_radius() {
    let tree = stem(1.2, |y| if y <= 1.0 + 1e-9 { 0.2 } else { 0.003 }, &[]);
    let rings = trunk(&tree);
    assert_eq!(rings.last().unwrap().r, 0.003);
    let shoot = rings.iter().position(|s| s.d > 1.0 + 1e-9).unwrap();
    for pair in rings[shoot - 1..].windows(2) {
        assert!(pair[1].r <= pair[0].r, "it rises at {}", pair[1].d);
    }
}
