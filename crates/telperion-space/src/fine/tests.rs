//! The fine roots under realized mechanics against today's whole-tree
//! engine (`realized.rs`), the decision contract's draws (`draws.rs`) and
//! ticks (`ticks.rs`), and the landing rule (`landing.rs`).
use super::{digest, grow_fine, Plan};
use crate::grow::{grow_whole, Request};
use crate::light::Light;
use crate::species::Species;

mod coarse;
mod draws;
mod landing;
mod realized;
mod ticks;

pub(super) fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The tree at each of `cuts` is today's whole-tree engine's.
pub(super) fn same_at_cuts(species: &Species, request: Request, cuts: &[f64]) {
    let whole = digest(&grow_whole(species, request).expect("grows whole"));
    for &cut in cuts {
        let plan = Plan {
            cut,
            ..Plan::default()
        };
        let fine = grow_fine(species, request, plan).expect("grows fine");
        assert_eq!(digest(&fine.structure), whole, "at cut {cut}");
    }
}

/// A species any bud of which reads light is refused by name: its fine
/// roots need fn-211's stepped executor.
#[test]
fn a_species_that_reads_light_is_refused() {
    let species = crate::oak();
    let refused = grow_fine(&species, request(5, 1), Plan::default());
    assert!(
        matches!(refused, Err(crate::Error::Refused { .. })),
        "{:?}",
        refused.err()
    );
}
