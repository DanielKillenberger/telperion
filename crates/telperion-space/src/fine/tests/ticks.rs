//! Time in whole ticks (FINE-CONTRACT.md, host decision 1): a lifespan is
//! read on the tick grid, a sleeping bud wakes on it, and a lifespan too
//! short for the grid grows its age's unit cut to none (fn-231).
use super::request;
use crate::fixed::CYCLE;
use crate::{grow, spruce};

#[test]
fn a_lifespan_is_read_on_the_tick_grid() {
    let species = spruce();
    let pa = (0..species.states.len())
        .find(|&k| species.states[k].lifespan > 1.0)
        .expect("a PA that lives");
    let mut off = species.clone();
    // Less than half a tick from its grid point: the same tree.
    off.states[pa].lifespan += 0.4 / CYCLE;
    let request = request(25, 1);
    assert!(grow(&off, request).expect("grows") == grow(&species, request).expect("grows"));
    // Past it, the next tick: a lifespan's fraction grows by degree.
    off.states[pa].lifespan += 1.37;
    grow(&off, request).expect("a fractional lifespan grows");
}

#[test]
fn a_sleeping_bud_wakes_on_the_tick_grid() {
    let tree = grow(&spruce(), request(40, 7)).expect("grows");
    let woken: Vec<f64> = tree
        .axes
        .iter()
        .map(|a| a.sleep)
        .filter(|&s| s > 0.0)
        .collect();
    assert!(!woken.is_empty(), "the spruce wakes sleeping buds");
    for sleep in woken {
        let ticks = sleep * CYCLE;
        assert_eq!(ticks, ticks.trunc(), "slept {sleep} of a cycle");
    }
}

/// A lifespan under half a tick grows (fn-231): its age's one unit is cut
/// to none and lays none of its nodes; a negative one is refused by name.
#[test]
fn a_lifespan_under_half_a_tick_grows_and_a_negative_one_is_refused() {
    let mut species = spruce();
    species.states[1].lifespan = 0.2 / CYCLE;
    grow(&species, request(10, 1)).expect("a lifespan above none grows");
    species.states[1].lifespan = -0.2 / CYCLE;
    let refused = grow(&species, request(10, 1)).expect_err("refused");
    assert!(format!("{refused}").contains("lifespan"), "{refused}");
}
