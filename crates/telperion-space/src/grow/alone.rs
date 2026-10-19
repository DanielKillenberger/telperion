//! The rules a tree's units draw by, its light unchecked (fn-209 probe
//! P2): what the device's unit tables are read from (`ChainTables::cycle`).
use super::shoot::Rules;
use crate::error::{refuse, Result};
use crate::presence::Windows;
use crate::shed::lasting;
use crate::species::{PaState, Species};

/// What every chain of a tree reads: grown once per tree.
pub(crate) fn rules<'a>(species: &'a Species, age: u32, budget: u32) -> Result<Rules<'a>> {
    species.validate()?;
    if age == 0 {
        return refuse("age", "a tree grows at least one cycle");
    }
    Ok(drawn(species, age, budget))
}

/// The rules a tree's units draw by, its light unchecked: its laterals,
/// lasting and windows.
pub(crate) fn drawn(species: &Species, age: u32, budget: u32) -> Rules<'_> {
    let laterals: Vec<_> = species.states.iter().map(PaState::laterals).collect();
    let lasting = lasting(species, &laterals);
    Rules {
        species,
        laterals: laterals.into(),
        lasting: lasting.into(),
        certain: super::Certain::Skip,
        windows: Windows::new(species, age),
        age,
        budget,
        grown: 0,
        one: false,
    }
}
