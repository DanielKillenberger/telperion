//! The date palm (*Phoenix dactylifera*) as a point in the tree space:
//! values only, on Corner's model (MODEL-PALM.md, rules C1 to C9): one
//! orthotropic, monopodial, indeterminate stem from one vegetative
//! meristem, never branched, growing continuously, one frond to a
//! phytomer. Branching readiness is zero, so its zones' lateral rows have
//! nothing to act on. The values and their sources are in
//! `.flow/evidence/fn-196-tree-space-d-the-date-palm-as-a-point/SOURCES.md`;
//! a value no source gives is marked there as estimated.
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::FRAC_PI_2;

/// The golden angle: the fronds' and the bases' one spiral (unsourced for
/// *Phoenix*; the pipeline's rosette divergence).
const GOLDEN: f64 = 2.399_963;

/// One zone of `fronds` nodes a year, one bud each, bearing nothing.
fn unit(fronds: u32) -> Vec<Zone> {
    vec![Zone {
        nodes: NodeLaw::Uniform {
            min: fronds,
            max: fronds,
        },
        buds: 1,
        lateral: vec![0.0],
        dormant: vec![0.0],
        delay: 0.0,
        rate: 0.0,
    }]
}

/// The palm, one growth cycle a year: a single PA that never moves on.
pub fn palm() -> Species {
    // A year's height of 0.3 to 0.45 m (A1) over a dozen fronds a year
    // (estimated): an internode of 0.03 m. Erect (C3), straightened hard
    // towards the vertical, a little wander for the stem's sway, no sag.
    // No secondary growth (C8): the stem keeps the width its apex laid it
    // down at, today's palm's trunk radius, 0.297 m; a mature trunk is
    // 0.52 to 1.08 m across its retained bases (P7, Table 1).
    let stem = PaState {
        lifespan: 1_000,
        next: None,
        viability: 1.0,
        zones: unit(12),
        shedding: None,
        internode: 0.031,
        insertion: 0.0,
        divergence: GOLDEN,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 0.0,
        rhythm: 0.0,
        straightening: 0.0,
        form: Form {
            tropism: 0.5,
            elevation: FRAC_PI_2,
            wander: 0.05,
            pipe: 0.297,
            secondary: 0.0,
            ..Form::default()
        },
    };
    Species { states: vec![stem] }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow, Request};

    /// Corner: one axis and no laterals (C1), and a stem of one width from
    /// its foot to its crown (C8).
    #[test]
    fn one_unbranched_stem_of_one_width() {
        for seed in [1, 7] {
            let request = Request {
                age: 50,
                seed,
                budget: 1_000_000,
            };
            let s = grow(&palm(), request).unwrap();
            assert_eq!(s.axes.len(), 1, "seed {seed}");
            let p = &s.axes[0].phytomers;
            let (base, top) = (p[0].radius, p[p.len() - 1].radius);
            assert!((base - top).abs() < 1e-12, "{base} against {top}");
            let height = p[p.len() - 1].tip.z;
            assert!((17.0..20.0).contains(&height), "{height} m");
        }
    }
}
