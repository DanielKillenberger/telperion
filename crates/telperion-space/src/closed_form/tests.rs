//! The closed form against itself: the expected length weighs the
//! expected counts.
use super::{expected_counts, expected_log_lengths};
use crate::species::{Form, NodeLaw, PaState, Species, Zone};

/// The expected length is the expected counts weighted by internode,
/// with every setting away from neutral but two that count what weighs
/// less than whole: bud places are whole (a fractional one counts whole
/// at its share of the length), and an age that ends relays none (a move
/// all but made grows its relay, counted whole and faded).
#[test]
fn the_expected_length_weighs_the_expected_counts() {
    let state = |continuation, internode, lateral: [f64; 2]| PaState {
        lifespan: 3.0,
        continuation,
        viability: 0.9,
        zones: vec![
            Zone {
                nodes: NodeLaw::Poisson { mean: 1.5 },
                buds: 2.0,
                dormant: vec![0.0; lateral.len()],
                delay: 0.0,
                rate: 0.0,
                lateral: lateral.to_vec(),
            },
            Zone {
                nodes: NodeLaw::Uniform { min: 1.0, max: 2.4 },
                buds: 1.0,
                dormant: vec![0.0; 2],
                delay: 0.0,
                rate: 0.0,
                lateral: vec![0.0, 0.2],
            },
        ],
        shedding: f64::INFINITY,
        internode,
        insertion: 0.5,
        divergence: 2.4,
        abortion: 0.2,
        abortion_rise: 0.0,
        relay: 0.4,
        // A move all but made grows its relay beside its continuation,
        // counted whole and of no expected length (host decision 13).
        relay_ended: 0.0,
        relay_failed: 0.4,
        relay_at: 1.0,
        epitony: 0.0,
        erection: 0.0,
        readiness: 0.8,
        rhythm: 0.6,
        straightening: 0.0,
        leaf_area: 0.0,
        shade_hazard: 0.0,
        shade_size: 0.0,
        apical_control: 0.5,
        upkeep: 0.0,
        balance_hazard: 0.0,
        tolerance: 0.0,
        retained: 0.0,
        leaf_girth: 0.0,
        form: Form::default(),
    };
    let species = Species {
        states: vec![state(0.6, 0.7, [0.1, 0.5]), state(0.0, 0.3, [0.0, 0.3])],
    };
    let counts = expected_counts(&species, 7).unwrap();
    let weighed = counts.total(0) * 0.7 + counts.total(1) * 0.3;
    let length = expected_log_lengths(&species, 7)[7][0].exp();
    assert!(
        (length - weighed).abs() < 1e-9 * weighed,
        "{length} vs {weighed}"
    );
}
