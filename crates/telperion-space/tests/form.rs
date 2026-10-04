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
    // A limb near level has no gap for the bend to close beside its base's
    // straightening.
    for limb in limbs(&tree).filter(|l| l.heading.z.abs() > 0.2) {
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

/// Roll turns each lateral about its parent by its own keyed draw, and
/// grows no other tree.
#[test]
fn roll_turns_each_lateral_about_its_parent() {
    let mut species = walk::species();
    species.states[1].form.roll = 1.0;
    let rolled = grown(&species, 1);
    let plain = grown(&walk::species(), 1);
    assert!(rolled == grown(&species, 1), "the same draws");
    let limbs = |t: &Structure| {
        t.axes
            .iter()
            .filter(|a| a.pa == 1 && matches!(a.origin, Origin::Lateral { .. }))
            .map(|a| (a.lineage, a.phytomers.len(), a.heading))
            .collect::<Vec<_>>()
    };
    let (rolled, plain) = (limbs(&rolled), limbs(&plain));
    assert_eq!(rolled.len(), plain.len(), "the same limbs");
    let mut turned = 0;
    for (r, p) in rolled.iter().zip(&plain) {
        assert_eq!((r.0, r.1), (p.0, p.1), "and grows no other tree");
        turned += usize::from(r.2.dot(p.2) < 1.0 - 1e-9);
    }
    assert!(
        turned * 2 > rolled.len(),
        "most limbs turn about their parent"
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
            Origin::Lateral { parent, node, .. } | Origin::Relay { parent, node }
                if node < carried[parent].len() =>
            {
                carried[parent][node] += base
            }
            Origin::Lateral { parent, .. }
            | Origin::Relay { parent, .. }
            | Origin::Continuation { parent } => tip[parent] += base,
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

/// A larger pipe exponent on the trunk keeps its limbs thicker beside
/// it, and grows no other tree.
#[test]
fn a_larger_pipe_exponent_keeps_limbs_thicker_beside_their_bearer() {
    let ratio = |exponent: f64| {
        let mut species = walk::species();
        species.states[0].form.exponent = exponent;
        let tree = grown(&species, 1);
        let trunk = tree.axes[0].phytomers[0].radius;
        let limb = tree
            .axes
            .iter()
            .filter(|a| a.pa == 1 && matches!(a.origin, Origin::Lateral { .. }))
            .filter_map(|a| a.phytomers.first())
            .map(|p| p.radius)
            .fold(0.0, f64::max);
        (limb / trunk, tree.axes.len())
    };
    let (area, axes) = ratio(2.0);
    let (wider, same) = ratio(3.0);
    assert_eq!(axes, same, "the same tree");
    assert!(wider > area, "limbs {wider} against {area} of the trunk");
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

/// Troll's modules: a relay carries on its axis's growth units, so a
/// trunk that relays every unit still reaches its next PA when an
/// unbroken one would.
#[test]
fn a_relay_carries_on_its_axis_growth_units() {
    let mut species = walk::species();
    let trunk = &mut species.states[0];
    trunk.lifespan = 4;
    trunk.next = Some(1);
    trunk.abortion = 1.0 - 1e-12;
    trunk.relay = 1.0;
    let tree = grown(&species, 1);
    let continued = tree
        .axes
        .iter()
        .find(|a| matches!(a.origin, Origin::Continuation { .. }))
        .expect("the stem reaches its next PA");
    assert_eq!(
        continued.birth, 4,
        "after four growth units across its relays"
    );
}

/// A relay stands at its PA's `relay_at` along the stopped axis's last
/// growth unit, and epitony turns it to the parent's upper side.
#[test]
fn a_relay_stands_in_the_curvature_zone_on_the_upper_side() {
    let relays = |at: f64, epitony: f64| {
        let mut species = walk::species();
        let limb = &mut species.states[1];
        limb.relay = 1.0;
        limb.relay_at = at;
        limb.epitony = epitony;
        limb.insertion = 0.6;
        let tree = grown(&species, 1);
        tree.axes
            .iter()
            .filter_map(|a| match a.origin {
                Origin::Relay { parent, node } if tree.axes[parent].phytomers.len() > 3 => {
                    let p = &tree.axes[parent].phytomers;
                    let last = p[p.len() - 1].cycle;
                    let first = p.iter().position(|q| q.cycle == last).unwrap();
                    Some((first, p.len(), node, a.heading.z))
                }
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    let half = relays(0.5, 0.0);
    assert!(!half.is_empty());
    for &(first, n, node, _) in &half {
        let middle = first + (n - first).div_ceil(2) - 1;
        assert_eq!(node, middle, "the middle of nodes {first} to {n}");
    }
    let rise =
        |v: &[(usize, usize, usize, f64)]| v.iter().map(|r| r.3).sum::<f64>() / v.len() as f64;
    assert!(
        rise(&relays(0.5, 1.0)) > rise(&half) + 0.05,
        "epitony turns relays up"
    );
}

/// Troll's secondary erection: an old module's base stands nearer the
/// vertical the longer it has grown, the seed's included.
#[test]
fn erection_lifts_an_axis_base_with_its_years() {
    let rise = |erection: f64| {
        let mut species = walk::species();
        species.states[1].erection = erection;
        let tree = grown(&species, 1);
        let mut sum = 0.0;
        for limb in tree
            .axes
            .iter()
            .filter(|a| a.pa == 1 && !a.phytomers.is_empty())
        {
            sum += (limb.phytomers[0].tip - limb.base).z
                / (limb.phytomers[0].tip - limb.base).length();
        }
        sum
    };
    assert!(rise(0.3) > rise(0.0) + 0.5, "bases rise");
}

/// A rising hazard ends long modules more often than short ones.
#[test]
fn the_abortion_hazard_rises_with_an_axis_units() {
    let mut state = walk::species().states[1].clone();
    state.abortion = 0.2;
    state.abortion_rise = 1.0;
    assert!((state.abortion_at(1) - 0.2).abs() < 1e-12);
    assert!(state.abortion_at(3) > state.abortion_at(2));
    state.abortion_rise = 0.0;
    assert_eq!(state.abortion_at(5), 0.2);
}
