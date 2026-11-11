//! The fidelity dial on a species (fn-325, fn-326): the fine wood grown from
//! fewer, longer phytomers, with what they bear and how they die answering
//! for the rest, so the one cycle grows less of the same tree. Each value
//! moves geometrically with the fidelity, from nothing at the finest to its
//! row at the coarsest, so the dial changes a tree by degree.
use super::{NodeLaw, Species};
use crate::error::{refuse, Result};

/// The fidelity in metres past which a tree grows no coarser: the manifest's
/// grove, what a website's voxel asks for.
pub const COARSEST: f64 = 1.0;

/// A species' rows at the coarsest fidelity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coarsening {
    /// The youngest age that coarsens, and every older one; younger ages,
    /// the trunk and limbs, grow as at the finest. A fraction, which only a
    /// walk between two species states, coarsens the age it falls in by that
    /// share, so the walk changes the tree by degree.
    pub first: f64,
    /// How much longer an internode is, and how many fewer nodes a unit has.
    pub factor: f64,
    /// How much more leaf area a node bears.
    pub leaf: f64,
    /// How much likelier a lateral bud is, up to its zone's certainty; under
    /// 1, how much of the laterals the coarse wood keeps.
    pub bud: f64,
    /// How much higher the shade and balance hazards are.
    pub hazard: f64,
    /// How much shorter a dead lateral stays before it is shed.
    pub shedding: f64,
}

impl Coarsening {
    /// Rows that change nothing, at any fidelity.
    pub const NONE: Self = Self {
        first: 0.0,
        factor: 1.0,
        leaf: 1.0,
        bud: 1.0,
        hazard: 1.0,
        shedding: 1.0,
    };

    /// Refuses, by name, rows the dial cannot grow from: no row may make a
    /// coarser tree finer, leave a setting at nothing or keep a dead lateral
    /// longer.
    pub fn validate(&self) -> Result<()> {
        if !(self.first >= 0.0 && self.first <= super::MAX_STATES as f64) {
            return refuse("coarse first", "an age of the species' reference axis");
        }
        if !(self.factor >= 1.0 && self.factor.is_finite()) {
            return refuse("coarse factor", "a finite factor of at least 1");
        }
        for (name, v) in [("coarse leaf", self.leaf), ("coarse bud", self.bud)] {
            if !(v > 0.0 && v.is_finite()) {
                return refuse(name, "a finite positive multiplier");
            }
        }
        if !(self.hazard >= 1.0 && self.hazard.is_finite()) {
            return refuse("coarse hazard", "a finite multiplier of at least 1");
        }
        if !(self.shedding > 0.0 && self.shedding <= 1.0) {
            return refuse("coarse shedding", "a multiplier above 0 and at most 1");
        }
        Ok(())
    }
}

impl Species {
    /// The species a tree of `fidelity` metres grows from: itself at 0.
    pub fn coarsened(&self, c: &Coarsening, fidelity: f64) -> Species {
        let t = (fidelity / COARSEST).clamp(0.0, 1.0);
        let mut out = self.clone();
        for (pa, state) in out.states.iter_mut().enumerate() {
            let share = t * (pa as f64 + 1.0 - c.first).clamp(0.0, 1.0);
            if share == 0.0 {
                continue;
            }
            let at = |v: f64| v.powf(share);
            let s = at(c.factor);
            state.internode *= s;
            state.leaf_area *= at(c.leaf);
            state.shade_hazard *= at(c.hazard);
            state.balance_hazard *= at(c.hazard);
            state.shedding *= at(c.shedding);
            for zone in &mut state.zones {
                zone.nodes = match zone.nodes {
                    NodeLaw::Uniform { min, max } => NodeLaw::Uniform {
                        min: min / s,
                        max: max / s,
                    },
                    NodeLaw::Poisson { mean } => NodeLaw::Poisson { mean: mean / s },
                };
                let sum: f64 = zone.lateral.iter().sum();
                if sum > 0.0 {
                    let k = at(c.bud).min((1.0 / sum).max(1.0));
                    zone.lateral.iter_mut().for_each(|p| *p *= k);
                }
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow_cycle, oak, Light, Request};

    const ROWS: Coarsening = Coarsening {
        first: 9.0,
        factor: 2.0,
        leaf: 16.0,
        bud: 1.3,
        hazard: 2.0,
        shedding: 0.25,
    };

    fn grown(species: &Species) -> u64 {
        let request = Request {
            age: 16,
            seed: 7,
            light: Light::default(),
        };
        grow_cycle(species, request).expect("a tree").grown
    }

    #[test]
    fn the_finest_is_the_species_itself_to_the_bit() {
        assert_eq!(oak().coarsened(&ROWS, 0.0), oak());
    }

    #[test]
    fn rows_that_change_nothing_change_nothing_at_any_fidelity() {
        assert_eq!(oak().coarsened(&Coarsening::NONE, 1.0), oak());
    }

    #[test]
    fn the_younger_ages_stay_and_the_older_scale_by_their_rows() {
        let (base, c) = (oak(), oak().coarsened(&ROWS, 1.0));
        assert_eq!(base.states[..9], c.states[..9]);
        let (a, b) = (&base.states[9], &c.states[9]);
        assert_eq!(b.internode, a.internode * 2.0);
        assert_eq!(b.leaf_area, a.leaf_area * 16.0);
        assert_eq!(b.shade_hazard, a.shade_hazard * 2.0);
        assert_eq!(b.shedding, a.shedding * 0.25);
    }

    #[test]
    fn past_the_coarsest_a_tree_grows_no_coarser() {
        assert_eq!(
            oak().coarsened(&ROWS, 7.0),
            oak().coarsened(&ROWS, COARSEST)
        );
    }

    #[test]
    fn a_zones_probabilities_never_pass_certainty() {
        let c = oak().coarsened(&Coarsening { bud: 100.0, ..ROWS }, 1.0);
        for zone in c.states.iter().flat_map(|s| &s.zones) {
            assert!(zone.lateral.iter().sum::<f64>() <= 1.0 + 1e-9);
        }
    }

    #[test]
    fn a_row_under_one_thins_the_laterals_by_degree() {
        let thin = Coarsening { bud: 0.25, ..ROWS };
        let sums = |s: &Species, pa: usize| -> f64 {
            s.states[pa].zones.iter().flat_map(|z| &z.lateral).sum()
        };
        let pa = (9..oak().states.len())
            .find(|&pa| sums(&oak(), pa) > 0.0)
            .expect("a bearing age");
        let (base, half, whole) = (
            sums(&oak(), pa),
            oak().coarsened(&thin, 0.5),
            oak().coarsened(&thin, 1.0),
        );
        assert!((sums(&whole, pa) / base - 0.25).abs() < 1e-12);
        assert!((sums(&half, pa) / base - 0.5).abs() < 1e-12);
        assert_eq!(oak().states[..9], whole.states[..9]);
    }

    #[test]
    fn a_fractional_first_coarsens_its_age_by_that_share() {
        let c = oak().coarsened(&Coarsening { first: 8.5, ..ROWS }, 1.0);
        let (base, half, whole) = (oak(), &c.states[8], &oak().coarsened(&ROWS, 1.0).states[9]);
        assert_eq!(base.states[..8], c.states[..8]);
        assert_eq!(half.internode, base.states[8].internode * 2f64.sqrt());
        assert_eq!(c.states[9].internode, whole.internode);
    }

    #[test]
    fn rows_off_their_domain_are_refused_by_name() {
        let bad = [
            (
                "first",
                Coarsening {
                    first: -1.0,
                    ..ROWS
                },
            ),
            (
                "factor",
                Coarsening {
                    factor: 0.0,
                    ..ROWS
                },
            ),
            (
                "factor",
                Coarsening {
                    factor: f64::NAN,
                    ..ROWS
                },
            ),
            ("leaf", Coarsening { leaf: 0.0, ..ROWS }),
            ("bud", Coarsening { bud: 0.0, ..ROWS }),
            (
                "hazard",
                Coarsening {
                    hazard: 0.5,
                    ..ROWS
                },
            ),
            (
                "shedding",
                Coarsening {
                    shedding: 0.0,
                    ..ROWS
                },
            ),
            (
                "shedding",
                Coarsening {
                    shedding: 2.0,
                    ..ROWS
                },
            ),
        ];
        for (name, rows) in bad {
            let refused = rows.validate().expect_err(name).to_string();
            assert!(refused.contains(name), "{refused}");
        }
        assert!(ROWS.validate().is_ok() && Coarsening::NONE.validate().is_ok());
    }

    #[test]
    fn a_coarser_tree_grows_fewer_phytomers() {
        let at = |f| grown(&oak().coarsened(&ROWS, f));
        let (finest, mid, coarsest) = (at(0.0), at(0.25), at(1.0));
        assert!(coarsest < mid && mid < finest, "{coarsest} {mid} {finest}");
    }
}
