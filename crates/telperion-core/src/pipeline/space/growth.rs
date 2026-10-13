//! Growth that continues (fn-270 R2): a family's tree begun for a horizon,
//! grown on to any age within it, and drawn at any age within its last
//! year, through the one cycle and the one conversion `grow` takes. The
//! state holds the engine's grower between calls (`telperion_space::Paused`);
//! a draw is a snapshot of its own and never moves the state. Ages are
//! years: growth commits whole cycles, and a draw within the year is the
//! stand-in presentation until R5's curves (`telperion_space::presented`).
use super::{convert, engine, fits, foreseen, pa_rows, scaled, PaRow, BUDGET, KEPT};
use crate::presets::Family;
use crate::tree::Tree;
use crate::{Error, Result};
use telperion_space::{admitted, Paused, Request, Retiring, Species};

/// What a family's tree grows and is drawn by, read from the family once:
/// its engine species, site light, trunk PAs, each PA's conversion row,
/// and the scale it is drawn at.
pub struct Params {
    species: Species,
    light: telperion_space::Light,
    trunk: Vec<usize>,
    rows: Vec<PaRow>,
    scale: f64,
}

impl Params {
    /// The family's growth, its supernatural rows applied and judged. A
    /// family with no engine species is refused by name.
    pub fn of(family: &Family) -> Result<Self> {
        let input = super::GrowInput::of(family);
        input.supernatural.validate()?;
        let space = input.space.as_ref();
        let species = space.species.clone().ok_or(Error::InvalidInput("no engine species"))?;
        let rows = pa_rows(&species);
        let (light, trunk, scale) = (space.light, space.trunk.clone(), input.supernatural.scale());
        Ok(Self { species, light, trunk, rows, scale })
    }
}

/// A tree growing: the engine's grower paused between calls, at the
/// cycles it has committed.
pub struct GrowthState<'p> {
    params: &'p Params,
    paused: Paused<'p>,
}

impl GrowthState<'_> {
    /// The whole years grown.
    pub fn committed(&self) -> u32 {
        self.paused.committed()
    }

    /// The most years it grows.
    pub fn horizon(&self) -> u32 {
        self.paused.horizon()
    }
}

/// The tree before its first year, to grow at most `horizon` whole years
/// from `seed`. A horizon that is no whole number of years is refused by
/// name, and one whose tree is expected past the ceiling before it grows.
pub fn begin(params: &Params, seed: u32, horizon: f64) -> Result<GrowthState<'_>> {
    if horizon.fract() != 0.0 || !(0.0..=f64::from(u32::MAX)).contains(&horizon) {
        return Err(Error::InvalidValue { field: "horizon", value: format!("{horizon}") });
    }
    let age = horizon as u32;
    foreseen(&params.species, age, KEPT)?;
    let request = Request { age, seed: u64::from(seed), budget: BUDGET, light: params.light };
    let paused = Paused::begin(&params.species, request, Retiring::AtAge, false).map_err(engine)?;
    Ok(GrowthState { params, paused })
}

/// The tree grown to `age` years: every whole cycle to its ceiling,
/// none where `age` falls within the last year grown. An age that is not
/// finite, a year or more back, or past the horizon is refused by name
/// before anything moves; a failure while it grows spends the state.
pub fn grow_to(state: &mut GrowthState, age: f64) -> Result<()> {
    let p = &mut state.paused;
    p.usable().map_err(engine)?;
    let to = admitted(p.committed(), p.horizon(), age).map_err(engine)?;
    p.grow_to(to).map_err(engine)
}

/// The tree drawn at `at`, within the last year grown, as the pipeline's
/// tree: a snapshot of its own, the state left as it was.
pub fn settle(state: &GrowthState, at: f64) -> Result<Tree> {
    let structure = state.paused.settle_at(at).map_err(engine)?;
    fits(&structure, KEPT)?;
    let p = state.params;
    let mut tree = convert(&structure, (&p.trunk, &p.rows));
    scaled(&mut tree, p.scale);
    Ok(tree)
}

#[cfg(test)]
mod tests;
