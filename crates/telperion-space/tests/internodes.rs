//! fn-222: internode lengths that vary node to node. Each internode's
//! length is scaled by a factor its node's key draws, 1 + spread (2u − 1),
//! so its mean is one: the tree's topology and scales are the same to the
//! bit at any spread, its laid lengths keep their mean, and at none every
//! tree is the one it was.
use telperion_space::{beech, grow, oak, Light, Request, Species, Structure, Vec3};

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 5_000_000,
        light: Light::NEUTRAL,
    }
}

fn spread(species: &Species, spread: f64) -> Species {
    let mut s = species.clone();
    for state in &mut s.states {
        state.form.internode_spread = spread;
    }
    s
}

/// Each phytomer's laid length over its PA's internode times its scale,
/// weighted by the latter, for the wood clear of the ground.
fn factors(species: &Species, tree: &Structure) -> Vec<(f64, f64)> {
    let mut out = Vec::new();
    for axis in &tree.axes {
        let internode = species.states[axis.pa].internode;
        let mut base: Vec3 = axis.base;
        for p in &axis.phytomers {
            let d = p.tip - base;
            let expected = internode * p.scale;
            if base.z > 1e-6 && p.tip.z > 1e-6 && expected > 1e-9 {
                out.push((d.length() / expected, expected));
            }
            base = p.tip;
        }
    }
    out
}

/// The weighted mean and standard deviation of `(value, weight)`.
fn moments(of: &[(f64, f64)]) -> (f64, f64) {
    let w: f64 = of.iter().map(|&(_, w)| w).sum();
    let mean = of.iter().map(|&(v, w)| v * w).sum::<f64>() / w;
    let var = of.iter().map(|&(v, w)| w * (v - mean).powi(2)).sum::<f64>() / w;
    (mean, var.sqrt())
}

#[test]
fn the_spread_moves_no_node_and_keeps_the_mean_length() {
    for (name, species) in [("beech", beech()), ("oak", oak())] {
        let plain = grow(&species, request(20, 1)).expect("grows");
        let varied_species = spread(&species, 0.6);
        let varied = grow(&varied_species, request(20, 1)).expect("grows");
        // The same nodes to the bit, the same scales by lineage (the f32
        // cycle's sums part at their last places where the geometry
        // moves; fn-256).
        assert_eq!(plain.counts(), varied.counts(), "{name}: counts");
        assert_eq!(plain.axes.len(), varied.axes.len(), "{name}: axes");
        for (a, b) in plain.axes.iter().zip(&varied.axes) {
            assert_eq!(a.phytomers.len(), b.phytomers.len(), "{name}");
            for (p, q) in a.phytomers.iter().zip(&b.phytomers) {
                let apart = (p.scale - q.scale).abs() / p.scale.abs().max(1e-12);
                assert!(apart <= telperion_space::agree::PATHS.scale, "{name}: scale {apart}");
                assert_eq!(p.key(), q.key(), "{name}");
            }
        }
        // Without spread each internode is its PA's length times its scale,
        // read back from its ends: f32 positions (fn-254) hold the shortest
        // internode far from the base to a few parts in ten thousand
        // (measured mean 1 - 2.1e-7, sd 2.1e-5).
        let (mean, sd) = moments(&factors(&species, &plain));
        assert!(
            (mean - 1.0).abs() < 1e-5 && sd < 1e-4,
            "{name}: {mean} {sd}"
        );
        // With it the lengths keep their mean and spread as the uniform
        // factor does: 0.6 / sqrt(3).
        let laid = factors(&varied_species, &varied);
        let (mean, sd) = moments(&laid);
        println!(
            "{name}: {} internodes, mean {mean:.4}, sd {sd:.4}",
            laid.len()
        );
        assert!(laid.len() > 5_000, "{name}: {}", laid.len());
        assert!((mean - 1.0).abs() < 0.01, "{name}: mean {mean}");
        assert!((sd - 0.6 / 3f64.sqrt()).abs() < 0.01, "{name}: sd {sd}");
        // Read back from f32 ends (fn-254), a length is its laid one to a
        // few units in the last place of the tree's reach: so far each
        // factor may stand past the spread's bounds.
        let reach = varied
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .fold(0.0f64, |r, p| r.max(p.tip.length()));
        let ulps = 4.0 * f64::from(f32::EPSILON) * reach;
        let outside = laid
            .iter()
            .filter(|&&(v, e)| v < 0.4 - ulps / e || v > 1.6 + ulps / e)
            .count();
        assert_eq!(outside, 0, "{name}: factors past the spread");
    }
}

#[test]
fn no_spread_is_the_tree_as_it_was() {
    // An explicit zero is the neutral value: the same tree to the bit.
    let species = beech();
    let a = grow(&species, request(15, 7)).expect("grows");
    let b = grow(&spread(&species, 0.0), request(15, 7)).expect("grows");
    assert_eq!(a, b);
}

#[test]
fn a_spread_beyond_one_is_refused() {
    let species = spread(&beech(), 1.01);
    assert!(grow(&species, request(3, 1)).is_err());
}
