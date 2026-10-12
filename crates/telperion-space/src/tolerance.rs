//! How a species answers light, from its shade tolerance (fn-264 R2, host
//! decisions 4, 2026-10-12): no source gives the engine's response rows for
//! any species, so one monotone mapping from Niinemets and Valladares's
//! shade-tolerance score T (0 to 5; Ecol. Monogr. 76: 521, 2006, Appendix A)
//! sets them, anchored on the oak, whose rows are its own (fn-195 round 5).
//!
//! - The hazards (shade and balance), the oak's one value of each, and the
//!   size response, its mean over the ages the oak lives in, times
//!   (5 - T) / (5 - T of the oak): an intolerant species sheds and shrinks
//!   more in shade.
//! - Upkeep, tolerance and leaf girth are shared physiology: the oak's,
//!   unscaled, so the balance hazard bites.
//! - Apical control is architecture, the species' own.
use crate::species::PaState;

/// The oak preset's T: *Quercus garryana*, 2.4 ± 0.3.
const ANCHOR: f64 = 2.4;
/// The score of maximal tolerance.
const MOST: f64 = 5.0;

/// Sets every one of `ages`' response rows from shade tolerance `t`.
pub(crate) fn respond(ages: &mut [(usize, PaState)], t: f64) {
    let oak = crate::oak::oak();
    let living: Vec<&PaState> = oak.states.iter().filter(|s| s.lifespan > 0.0).collect();
    let mean =
        |row: fn(&PaState) -> f64| living.iter().map(|s| row(s)).sum::<f64>() / living.len() as f64;
    // The oak's one value of a row it states alike at every age.
    let one = |row: fn(&PaState) -> f64| {
        let v = row(living[0]);
        assert!(
            living.iter().all(|s| row(s) == v),
            "the oak states the row alike"
        );
        v
    };
    let k = (MOST - t) / (MOST - ANCHOR);
    let (hazard, balance, size) = (
        k * one(|s| s.shade_hazard),
        k * one(|s| s.balance_hazard),
        k * mean(|s| s.shade_size),
    );
    let (upkeep, tolerance, girth) = (
        one(|s| s.upkeep),
        one(|s| s.tolerance),
        one(|s| s.leaf_girth),
    );
    for (_, s) in ages {
        (s.shade_hazard, s.balance_hazard, s.shade_size) = (hazard, balance, size);
        (s.upkeep, s.tolerance, s.leaf_girth) = (upkeep, tolerance, girth);
    }
}

/// `species` with no response to light, as every species but the oak grew
/// before fn-264: the premise of the tests of the paths a lit species never
/// takes (the closed form, the coarse tree, the shed-certain shortcut).
#[cfg(test)]
pub(crate) fn unlit(mut species: crate::Species) -> crate::Species {
    for s in &mut species.states {
        (s.shade_hazard, s.shade_size, s.balance_hazard, s.leaf_girth) = (0.0, 0.0, 0.0, 0.0);
    }
    species
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The mapping read back on a species at the oak's own T gives the
    /// oak's means; tolerance moves the hazards and size one way only.
    #[test]
    fn the_mapping_is_the_oaks_at_its_t_and_monotone() {
        let one = |t| {
            let mut ages = vec![(0, crate::chain::passed())];
            respond(&mut ages, t);
            ages.remove(0).1
        };
        let at = one(ANCHOR);
        assert_eq!((at.shade_hazard, at.balance_hazard), (1.0, 2.0));
        assert!((at.shade_size - 1.1).abs() < 1e-12, "{}", at.shade_size);
        assert_eq!((at.upkeep, at.tolerance, at.leaf_girth), (0.35, 0.0, 1.0));
        let (shy, bold) = (one(4.56), one(2.03));
        assert!(shy.shade_hazard < at.shade_hazard && at.shade_hazard < bold.shade_hazard);
        assert!(shy.shade_size < bold.shade_size && shy.balance_hazard < bold.balance_hazard);
        assert_eq!((shy.upkeep, bold.leaf_girth), (0.35, 1.0));
        assert_eq!(one(MOST).shade_hazard, 0.0);
    }
}
