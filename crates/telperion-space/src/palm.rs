//! The date palm (*Phoenix dactylifera*) as a point in the tree space:
//! values only, on Corner's model (MODEL-PALM.md, rules C1 to C9): one
//! orthotropic, monopodial, indeterminate stem from one vegetative
//! meristem, never branched, growing continuously, one frond to a
//! phytomer. Branching readiness is zero, so its zones' lateral rows have
//! nothing to act on. The values and their sources are in
//! `.flow/evidence/fn-196-tree-space-d-the-date-palm-as-a-point/SOURCES.md`;
//! a value no source gives is marked there as estimated.
use crate::chain::{self, on_chain};
use crate::species::{Form, NodeLaw, PaState, Species, Zone};
use std::f64::consts::FRAC_PI_2;

/// The golden angle: the fronds' and the bases' one spiral (unsourced for
/// *Phoenix*; the pipeline's rosette divergence).
const GOLDEN: f64 = 2.399_963;

/// One zone of `fronds` nodes a year, one bud each, bearing nothing.
fn unit(fronds: u32) -> Vec<Zone> {
    vec![Zone {
        nodes: NodeLaw::Uniform {
            min: f64::from(fronds),
            max: f64::from(fronds),
        },
        buds: 1.0,
        lateral: vec![0.0; chain::AGES],
        dormant: vec![0.0; chain::AGES],
        delay: 0.0,
        rate: 0.0,
    }]
}

/// The palm, one growth cycle a year: a single PA that never moves on.
pub fn palm() -> Species {
    // A year's height of 0.3 to 0.45 m (A1) over a dozen fronds a year
    // (estimated): an internode of 0.03 m, raised to 0.0339 m because the
    // stem waits on its crown's establishment (fn-275 R2a): 0.407 m a year,
    // inside A1's range, keeps the palm at 40 at 21.59 m. Erect (C3),
    // straightened hard towards the vertical, a little wander for the
    // stem's sway, no sag.
    // No secondary growth (C8): the stem keeps the width its apex laid it
    // down at, today's palm's trunk radius, 0.297 m; a mature trunk is
    // 0.52 to 1.08 m across its retained bases (P7, Table 1).
    let stem = PaState {
        // For as long as a date palm lives, over a century (P4), then
        // it stops.
        lifespan: 150.0,
        continuation: 0.0,
        viability: 1.0,
        zones: unit(12),
        shedding: f64::INFINITY,
        internode: 0.0339,
        insertion: 0.0,
        divergence: GOLDEN,
        abortion: 0.0,
        abortion_rise: 0.0,
        relay: 0.0,
        relay_ended: 0.0,
        relay_failed: 0.0,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 0.0,
        rhythm: 0.0,
        // One frond a node of 2.81 m², the mean of four fully grown leaves
        // on each of nine 15-year-old palms, drip-irrigated 8 m apart
        // near Riyadh (Abdel-Sattar et al. 2024, Folia Hortic., Table 2;
        // length x width x 0.8, derived; fn-264 R2, host 2026-10-12; Jev
        // fn264-pick:18dddb39ffe7316b-3e89b7-0). Its responses stay
        // neutral: it has no laterals to shed and no published shade
        // tolerance, and its one axis makes them dormant.
        leaf_area: 2.81,
        flowering: 0.0,
        cluster_leaves: 0.0,
        cluster_spread: 0.0,
        leaf_years: f64::INFINITY,
        // fn-270 R5: the days its shoot elongates and its leaf expands
        // within the year, sourced in its preset's value file.
        elongation: 226.0,
        expansion: 139.0,
        leaf_life: f64::INFINITY,
        budburst: 0.0,
        senescence: 0.0,
        colouring: 0.0,
        leaf_fall: 0.0,
        shade_hazard: 0.0,
        ground_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
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
    // The stem stands at the chain's trunk.
    on_chain(vec![(chain::TRUNK, stem)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{grow, Light, Request};

    /// Corner: one axis and no laterals (C1), and a stem of one width from
    /// its foot to its crown (C8).
    #[test]
    fn one_unbranched_stem_of_one_width() {
        for seed in [1, 7] {
            let request = Request {
                age: 50,
                seed,
                budget: 1_000_000,
                light: Light::NEUTRAL,
            };
            let s = grow(&palm(), request).unwrap();
            assert_eq!(s.axes.len(), 1, "seed {seed}");
            let p = &s.axes[0].phytomers;
            let (base, top) = (p[0].radius, p[p.len() - 1].radius);
            assert!((base - top).abs() < 1e-12, "{base} against {top}");
            // Inside A1's 0.30 to 0.45 m a year over 50 years: the engine's
            // stem rises 0.407 m a year (20.3 m), since fn-275 R2a lowers
            // what it draws by the rise it waited on its crown.
            let height = p[p.len() - 1].tip.z;
            assert!((15.0..22.5).contains(&height), "{height} m");
        }
    }
}
