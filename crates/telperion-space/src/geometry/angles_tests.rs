//! fn-224: leaving angles and turning rates that vary axis to axis. Each
//! lateral's insertion angle is its PA's offset by spread (2u − 1), and
//! each axis's tropism its PA's times 1 + spread (2u − 1), u the axis's
//! own lineage draw: the tree's topology is the same to the bit at any
//! spread and its scales by lineage (the f32 cycle's sums part at their
//! last places where the geometry moves; fn-256), each angle is the one its draw keys, and at none
//! every tree is the one it was.
use crate::contract::{insertion_draw, tropism_draw};
use crate::{beech, grow, oak, Light, Origin, Request, Species, Structure};

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 5_000_000,
        light: Light::NEUTRAL,
    }
}

fn spread(species: &Species, (insertion, tropism): (f64, f64)) -> Species {
    let mut s = species.clone();
    for state in &mut s.states {
        state.form.insertion_spread = insertion;
        state.form.tropism_spread = tropism;
    }
    s
}

/// A 53-bit draw as the real in [0, 1) its geometry reads: its top 24
/// bits (`lineage::unit`).
fn unit(bits: u64) -> f64 {
    (bits >> 29) as f64 / (1u64 << 24) as f64
}

/// The same nodes and keys to the bit, and scales by lineage
/// (`agree::PATHS`).
fn same_topology(name: &str, a: &Structure, b: &Structure) {
    assert_eq!(a.counts(), b.counts(), "{name}: counts");
    assert_eq!(a.axes.len(), b.axes.len(), "{name}: axes");
    for (x, y) in a.axes.iter().zip(&b.axes) {
        assert_eq!((x.pa, x.lineage), (y.pa, y.lineage), "{name}");
        assert_eq!(x.phytomers.len(), y.phytomers.len(), "{name}");
        for (p, q) in x.phytomers.iter().zip(&y.phytomers) {
            let apart = (p.scale - q.scale).abs() / p.scale.abs().max(1e-12);
            assert!(apart <= crate::agree::PATHS.scale, "{name}: scale {apart}");
            assert_eq!(p.key(), q.key(), "{name}");
        }
    }
}

/// Each lateral's angle to its bearer's phytomer at its node, against
/// the angle its draw keys: the largest difference, the offsets, and the
/// laterals measured.
fn insertions(species: &Species, tree: &Structure) -> (f64, Vec<f64>) {
    let (mut most, mut offsets) = (0.0f64, Vec::new());
    for axis in &tree.axes {
        let Origin::Lateral { parent, node, .. } = axis.origin else {
            continue;
        };
        let state = &species.states[axis.pa];
        let at = tree.axes[parent].phytomers[node].heading;
        let angle = axis.heading.dot(at).clamp(-1.0, 1.0).acos();
        let s = state.form.insertion_spread;
        let offset = s * (2.0 * unit(insertion_draw(axis.lineage)) - 1.0);
        most = most.max((angle - (state.insertion + offset).abs()).abs());
        offsets.push(offset / s.max(f64::MIN_POSITIVE));
    }
    (most, offsets)
}

fn mean(values: &[f64]) -> f64 {
    values.iter().sum::<f64>() / values.len() as f64
}

#[test]
fn the_spreads_move_no_node_and_each_angle_is_its_draws() {
    for (name, species) in [("beech", beech()), ("oak", oak())] {
        let plain = grow(&species, request(20, 1)).expect("grows");
        let varied_species = spread(&species, (0.25, 0.8));
        let varied = grow(&varied_species, request(20, 1)).expect("grows");
        same_topology(name, &plain, &varied);
        assert_ne!(plain, varied, "{name}: the spreads moved nothing");
        // Each lateral leaves its bearer at its PA's angle without spread,
        // and at the angle its own draw keys with it.
        let (most, _) = insertions(&species, &plain);
        assert!(most < 1e-6, "{name}: {most}");
        let (most, offsets) = insertions(&varied_species, &varied);
        // The offsets over the spread are uniform on [−1, 1]: mean 0 and
        // standard deviation 1 / sqrt(3).
        let (m, sd) = (
            mean(&offsets),
            mean(&offsets.iter().map(|o| o * o).collect::<Vec<_>>()),
        );
        let sd = (sd - m * m).sqrt();
        println!(
            "{name}: {} laterals, angles within {most:.1e}, offsets mean {m:.4} sd {sd:.4}",
            offsets.len()
        );
        assert!(most < 1e-6, "{name}: {most}");
        assert!(offsets.len() > 1_000, "{name}: {}", offsets.len());
        assert!(m.abs() < 0.05 && (sd - 3f64.sqrt().recip()).abs() < 0.03);
        // Each axis's tropism factor, 1 + spread (2u − 1), has mean one.
        let factors: Vec<f64> = varied
            .axes
            .iter()
            .map(|a| 1.0 + 0.8 * (2.0 * unit(tropism_draw(a.lineage)) - 1.0))
            .collect();
        let f = mean(&factors);
        assert!((f - 1.0).abs() < 0.03, "{name}: tropism factor mean {f}");
    }
}

#[test]
fn a_tropism_spread_alone_turns_the_axes() {
    let species = beech();
    let plain = grow(&species, request(15, 3)).expect("grows");
    let varied = grow(&spread(&species, (0.0, 0.5)), request(15, 3)).expect("grows");
    same_topology("beech", &plain, &varied);
    // Every lateral still leaves at its PA's angle; only the laid axes turn.
    assert!(insertions(&species, &varied).0 < 1e-6);
    assert_ne!(plain, varied);
}

#[test]
fn no_spread_is_the_tree_as_it_was() {
    let species = beech();
    let a = grow(&species, request(15, 7)).expect("grows");
    let b = grow(&spread(&species, (0.0, 0.0)), request(15, 7)).expect("grows");
    assert_eq!(a, b);
}

#[test]
fn a_spread_beyond_its_bound_is_refused() {
    let wide = std::f64::consts::FRAC_PI_2 + 0.01;
    assert!(grow(&spread(&beech(), (wide, 0.0)), request(3, 1)).is_err());
    assert!(grow(&spread(&beech(), (0.0, 1.01)), request(3, 1)).is_err());
}
