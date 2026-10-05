//! Borchert–Honda allocation against Pałubicki et al. (2009, 4.2): the
//! split at one branching point, conservation over a tree, a shaded limb
//! receiving less, and λ = 0.5 sharing in proportion to light.
use super::*;

fn link(parent: Option<usize>, node: usize) -> Link {
    Link { parent, node }
}

/// An axis with its own bud and one lateral at its first node: the lateral
/// receives v (1 - λ) Q_l / (λ Q_m + (1 - λ) Q_l), the continuing axis the
/// rest, Q_m being all above the node: here the axis's own bud.
#[test]
fn one_branching_point_splits_as_palubicki_states() {
    let links = [link(None, 0), link(Some(0), 0)];
    for (lambda, qm, ql) in [
        (0.5, 1.0, 1.0),
        (0.7, 2.0, 1.0),
        (0.3, 0.5, 1.5),
        (0.9, 1.0, 3.0),
    ] {
        let v = vigours(&links, &[qm, ql], &[lambda, lambda]);
        let total = qm + ql;
        let lateral = total * (1.0 - lambda) * ql / (lambda * qm + (1.0 - lambda) * ql);
        let main = total * lambda * qm / (lambda * qm + (1.0 - lambda) * ql);
        assert!(
            (v[1] - lateral).abs() < 1e-12,
            "λ {lambda}: lateral {} against {lateral}",
            v[1]
        );
        assert!(
            (v[0] - main).abs() < 1e-12,
            "λ {lambda}: main {} against {main}",
            v[0]
        );
    }
}

/// A trunk with a whorl, laterals with laterals and a continuation: every
/// bud's vigour sums to the tree's light at every λ.
fn tree() -> (Vec<Link>, Vec<f64>) {
    let links = vec![
        link(None, 0),
        link(Some(0), 1),
        link(Some(0), 1),
        link(Some(0), 3),
        link(Some(0), TIP),
        link(Some(1), 0),
        link(Some(1), 2),
        link(Some(3), TIP),
        link(Some(4), 1),
    ];
    let own = vec![0.0, 0.4, 0.7, 0.0, 1.1, 0.3, 0.2, 0.9, 0.5];
    (links, own)
}

#[test]
fn the_trees_vigour_is_conserved_at_every_lambda() {
    let (links, own) = tree();
    let total: f64 = own.iter().sum();
    for lambda in [0.1, 0.3, 0.5, 0.62, 0.8, 0.95] {
        let v = vigours(&links, &own, &vec![lambda; links.len()]);
        let sum: f64 = v.iter().sum();
        assert!(
            (sum - total).abs() < 1e-12,
            "λ {lambda}: {sum} against {total}"
        );
    }
}

/// λ = 0.5 is the unbiased split: every bud's vigour is its own light, so
/// a bud's size against a uniformly lit tree is its light alone.
#[test]
fn an_unbiased_split_gives_every_bud_its_own_light() {
    let (links, own) = tree();
    let v = vigours(&links, &own, &vec![0.5; links.len()]);
    for (i, (&vi, &qi)) in v.iter().zip(&own).enumerate() {
        assert!((vi - qi).abs() < 1e-12, "bud {i}: {vi} against {qi}");
    }
}

/// Two limbs alike but for light: the shaded one's bud receives less
/// vigour, and less against a uniformly lit tree, at every λ.
#[test]
fn a_shaded_limb_receives_less() {
    let links = [
        link(None, 0),
        link(Some(0), 0),
        link(Some(0), 1),
        link(Some(0), TIP),
    ];
    let weights = [0.0, 1.0, 1.0, 1.0];
    let lit = [0.0, 0.9, 0.3, 0.8];
    for lambda in [0.3, 0.5, 0.7] {
        let l = [lambda; 4];
        let real = vigours(&links, &lit, &l);
        let uniform = vigours(&links, &weights, &l);
        assert!(
            real[2] < real[1],
            "λ {lambda}: shaded {} against lit {}",
            real[2],
            real[1]
        );
        let (r1, r2) = (real[1] / uniform[1], real[2] / uniform[2]);
        assert!(r2 < r1, "λ {lambda}: shaded ratio {r2} against lit {r1}");
    }
}

/// Vigour moves by degree as one bud's light moves through nothing.
#[test]
fn vigour_moves_by_degree_as_light_does() {
    let (links, mut own) = tree();
    let lambda = vec![0.7; links.len()];
    let mut last = vigours(&links, &own, &lambda);
    for step in 1..=1000 {
        own[6] = 0.2 * (1.0 - f64::from(step) / 1000.0);
        let v = vigours(&links, &own, &lambda);
        let worst = v
            .iter()
            .zip(&last)
            .map(|(a, b)| (a - b).abs())
            .fold(0.0, f64::max);
        assert!(worst < 0.01, "step {step}: {worst}");
        last = v;
    }
}

/// Host decision 14: the presence-weighted mean size is exactly 1 at every
/// ψ, every bud is whole at ψ 0, and a lit bud outgrows a shaded one.
#[test]
fn sizes_keep_the_mean_whole_and_favour_light() {
    let (links, own) = tree();
    let presence = [0.0, 0.5, 0.9, 0.0, 1.2, 0.4, 0.3, 1.0, 0.6];
    let lit: Vec<f64> = own.iter().zip(&presence).map(|(q, w)| q * w).collect();
    let v = vigours(&links, &lit, &vec![0.45; links.len()]);
    for psi in [0.0, 0.5, 1.0, 2.0, 4.0] {
        let s = sizes(&v, &presence, &vec![psi; links.len()]);
        let mean =
            s.iter().zip(&presence).map(|(a, w)| a * w).sum::<f64>() / presence.iter().sum::<f64>();
        assert!((mean - 1.0).abs() < 1e-12, "ψ {psi}: mean {mean}");
        if psi == 0.0 {
            assert!(s.iter().all(|&x| x == 1.0));
        }
    }
    // Buds 4 (light 1.1) and 6 (0.2) at λ 0.5: the lit one outgrows.
    let even = vigours(&links, &lit, &vec![0.5; links.len()]);
    let s = sizes(&even, &presence, &vec![1.0; links.len()]);
    assert!(s[4] > 1.0 && s[6] < 1.0 && s[4] > s[6], "{s:?}");
}

/// The sizes move by degree as one bud's presence falls to nothing.
#[test]
fn sizes_move_by_degree_as_a_bud_vanishes() {
    let (links, own) = tree();
    let mut presence = vec![0.0, 0.5, 0.9, 0.0, 1.2, 0.4, 0.3, 1.0, 0.6];
    let psi = vec![1.0; links.len()];
    let at = |presence: &[f64]| {
        let lit: Vec<f64> = own.iter().zip(presence).map(|(q, w)| q * w).collect();
        sizes(
            &vigours(&links, &lit, &vec![0.45; links.len()]),
            presence,
            &psi,
        )
    };
    let mut last = at(&presence);
    for step in 1..=1000 {
        presence[2] = 0.9 * (1.0 - f64::from(step) / 1000.0);
        let s = at(&presence);
        let worst = s
            .iter()
            .zip(&last)
            .enumerate()
            .filter(|(i, _)| *i != 2)
            .map(|(_, (a, b))| (a - b).abs())
            .fold(0.0, f64::max);
        assert!(worst < 0.01, "step {step}: {worst}");
        last = s;
    }
}
