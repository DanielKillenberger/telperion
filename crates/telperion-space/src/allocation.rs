//! Relative allocation (fn-197, host decision 7): the buds growing on one
//! bearer in a cycle share its growth by their light, as the extended
//! Borchert–Honda model shares a resource between the branches at a node
//! (Pałubicki et al. 2009, 4.2). A bud's size is its light to its PA's
//! `shade_size` (ψ) over the bearer's presence-weighted mean of the same,
//! so the bearer's presence-weighted total is conserved: a bud lit beside
//! shaded siblings outgrows them, and with every ψ at 0 every bud is
//! whole. The mean is smooth in every bud's weight, so a bud made or
//! lost at vanishing presence moves its siblings by nothing.

/// One growing bud: its presence (its weight among its siblings), its
/// light and its PA's ψ.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Bud {
    pub weight: f64,
    pub light: f64,
    pub psi: f64,
}

/// Each bud's size among its siblings: light^ψ over the siblings' weighted
/// mean of light^ψ. The weighted sizes sum to the weights' sum; with no
/// weight, every bud is whole.
pub(crate) fn sizes(buds: &[Bud]) -> impl Iterator<Item = f64> + '_ {
    let drawn = |b: &Bud| b.light.powf(b.psi);
    let total: f64 = buds.iter().map(|b| b.weight).sum();
    let mean = buds.iter().map(|b| b.weight * drawn(b)).sum::<f64>() / total;
    buds.iter().map(move |b| {
        if total > 0.0 && mean > 0.0 {
            drawn(b) / mean
        } else {
            1.0
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bud(weight: f64, light: f64, psi: f64) -> Bud {
        Bud { weight, light, psi }
    }

    /// Pałubicki's Borchert–Honda split at λ = 0.5 shares a resource v
    /// between two branches in proportion to their light, v Q_i / (Q_m +
    /// Q_l). With ψ = 1 the two buds' sizes over their sum are exactly
    /// those shares; as ψ rises the lit bud's share rises towards all of
    /// it, and the sum is the same at every ψ.
    #[test]
    fn two_siblings_one_shaded_share_as_borchert_honda_and_keep_their_sum() {
        let (lit, shaded) = (0.9, 0.3);
        let at = |psi: f64| sizes(&[bud(1.0, lit, psi), bud(1.0, shaded, psi)]).collect::<Vec<_>>();
        let one = at(1.0);
        let share = one[0] / (one[0] + one[1]);
        assert!(
            (share - lit / (lit + shaded)).abs() < 1e-12,
            "share {share}"
        );
        let mut last = 0.5;
        for psi in [0.0, 0.5, 1.0, 2.0, 4.0] {
            let s = at(psi);
            assert!(
                (s[0] + s[1] - 2.0).abs() < 1e-12,
                "psi {psi}: sum {}",
                s[0] + s[1]
            );
            let share = s[0] / (s[0] + s[1]);
            assert!(share >= last, "psi {psi}: the lit share fell to {share}");
            last = share;
        }
        assert!(last > 0.98, "at psi 4 the lit share is only {last}");
    }

    /// With every ψ at 0 every bud is whole, to the bit; a bud of no
    /// weight moves its siblings by nothing; and the weighted total is
    /// conserved for unequal weights.
    #[test]
    fn neutral_whole_vanishing_buds_inert_and_totals_conserved() {
        let neutral = [bud(0.3, 0.2, 0.0), bud(0.7, 0.9, 0.0), bud(0.1, 0.5, 0.0)];
        assert!(sizes(&neutral).all(|s| s == 1.0));
        let two = [bud(0.4, 0.8, 1.5), bud(0.6, 0.3, 1.5)];
        let three = [two[0], two[1], bud(0.0, 0.05, 1.5)];
        let (a, b): (Vec<f64>, Vec<f64>) = (sizes(&two).collect(), sizes(&three).collect());
        assert!((a[0] - b[0]).abs() < 1e-15 && (a[1] - b[1]).abs() < 1e-15);
        let total: f64 = two.iter().zip(&a).map(|(x, s)| x.weight * s).sum();
        assert!((total - 1.0).abs() < 1e-12, "total {total}");
    }
}
