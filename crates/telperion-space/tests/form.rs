//! fn-193: the shape an axis takes beyond its angles. Tropism bends an axis
//! towards its PA's elevation, wander turns each node by a keyed draw, a
//! parent's plane turns the plane its laterals branch in, laterals leave
//! from their parent's direction at their node, and girth follows the pipe
//! model.
mod walk;
use telperion_space::{beech, grow, Axis, Origin, Request, Species, Structure, Vec3};

fn grown(species: &Species, seed: u64) -> Structure {
    walk::tree(species, seed).unwrap()
}

fn limbs(tree: &Structure) -> impl Iterator<Item = &Axis> {
    tree.axes
        .iter()
        .filter(|a| a.pa == 1 && a.phytomers.len() > 3)
}

fn rise(a: Vec3, b: Vec3) -> f64 {
    let run = b - a;
    (run.z / run.length()).asin()
}

#[test]
fn tropism_bends_an_axis_towards_its_elevation() {
    let mut species = walk::species();
    species.states[1].form.tropism = 3.0;
    species.states[1].form.elevation = 0.0;
    let tree = grown(&species, 1);
    let mut bent = 0;
    for limb in limbs(&tree) {
        let n = limb.phytomers.len();
        let tip = rise(limb.phytomers[n - 2].tip, limb.phytomers[n - 1].tip);
        let gap = limb.heading.z.asin();
        assert!(
            tip.abs() < gap.abs() + 1e-9,
            "the tip is nearer level: {tip} vs {gap}"
        );
        bent += usize::from(tip.abs() < 0.5 * gap.abs());
    }
    assert!(bent > 0, "some limb bends at least halfway");
}

#[test]
fn wander_turns_each_node_by_its_own_draw() {
    let mut species = walk::species();
    species.states[1].form.wander = 0.5;
    let once = grown(&species, 1);
    assert!(once == grown(&species, 1), "the same draws");
    assert!(once != grown(&walk::species(), 1), "wander turns the limbs");
    let topology = |t: &Structure| {
        t.axes
            .iter()
            .map(|a| (a.lineage, a.phytomers.len()))
            .collect::<Vec<_>>()
    };
    assert_eq!(
        topology(&once),
        topology(&grown(&walk::species(), 1)),
        "and grows no other tree"
    );
}

/// A parent's plane turns its laterals about their headings; with a
/// quarter turn a distichous lateral's own laterals leave square to where
/// they left without it.
#[test]
fn a_parents_plane_turns_its_laterals_plane() {
    let side_of = |plane: f64| {
        let mut species = walk::species();
        species.states[0].form.plane = plane;
        let tree = grown(&species, 1);
        let limb = tree
            .axes
            .iter()
            .find(|a| a.pa == 1 && matches!(a.origin, Origin::Lateral { .. }))
            .unwrap();
        (limb.lineage, limb.heading, limb.side)
    };
    let (lineage, heading, side) = side_of(0.0);
    let (turned_lineage, turned_heading, turned_side) = side_of(std::f64::consts::FRAC_PI_2);
    assert_eq!(lineage, turned_lineage);
    assert!(
        (heading - turned_heading).length() < 1e-12,
        "the heading holds"
    );
    assert!(
        side.dot(turned_side).abs() < 1e-12,
        "the side turns a quarter"
    );
}

/// A lateral leaves from its parent's direction at its node, so laterals
/// on a bent limb keep their insertion angle to the wood they leave.
#[test]
fn a_lateral_leaves_from_its_parents_direction_at_its_node() {
    let mut species = walk::species();
    species.states[1].form.tropism = 3.0;
    species.states[1].form.elevation = 0.0;
    let tree = grown(&species, 1);
    let mut checked = 0;
    for axis in tree.axes.iter().filter(|a| a.pa == 2) {
        let Origin::Lateral { parent, node, .. } = axis.origin else {
            continue;
        };
        let along = tree.axes[parent].phytomers[node].heading;
        let angle = along.dot(axis.heading).clamp(-1.0, 1.0).acos();
        assert!(
            (angle - species.states[2].insertion).abs() < 1e-9,
            "{angle}"
        );
        checked += 1;
    }
    assert!(checked > 0);
}

/// Pipe model: a section is the sum of the sections it bears.
#[test]
fn girth_is_the_sum_of_the_sections_carried() {
    let species = walk::species();
    let tree = grown(&species, 1);
    let mut carried = vec![vec![0.0; 0]; tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate() {
        carried[i] = vec![0.0; axis.phytomers.len()];
    }
    let mut tip = vec![0.0; tree.axes.len()];
    for (i, axis) in tree.axes.iter().enumerate().rev() {
        let base = axis
            .phytomers
            .first()
            .map_or(tip[i], |p| p.radius * p.radius);
        match axis.origin {
            Origin::Lateral { parent, node, .. } => carried[parent][node] += base,
            Origin::Continuation { parent } | Origin::Relay { parent } => tip[parent] += base,
            Origin::Seed => {}
        }
    }
    for (i, axis) in tree.axes.iter().enumerate() {
        let pipe = species.states[axis.pa].form.pipe;
        for (k, p) in axis.phytomers.iter().enumerate() {
            let above = axis
                .phytomers
                .get(k + 1)
                .map_or(tip[i], |q| q.radius * q.radius);
            let own = (pipe * p.scale).powi(2);
            let want = above + own + carried[i][k];
            assert!(
                (p.radius * p.radius - want).abs() < 1e-12 * want.max(1e-12),
                "axis {i} node {k}"
            );
        }
    }
}

/// The beech grows at every age its sheet draws, at both seeds, standing
/// and above the ground.
#[test]
fn the_beech_grows_at_every_age_of_its_sheet() {
    for age in [10, 20, 40, 80] {
        for seed in [1, 7] {
            let request = Request {
                age,
                seed,
                budget: 10_000_000,
            };
            let tree = grow(&beech(), request).unwrap_or_else(|e| panic!("{age}, {seed}: {e:?}"));
            assert!(tree.axes[0].phytomers[0].radius > 0.0);
        }
    }
}
