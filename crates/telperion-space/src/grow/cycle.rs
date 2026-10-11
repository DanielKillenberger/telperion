//! fn-255 R1: one cycle for every species (DESIGN.md section 1). The
//! whole-tree grower runs every phase every cycle for every species and
//! every light: the rough layout and the light read, the balance and the
//! allotment, each zero setting a neutral term in them, never another way
//! of building. Light is the lattice for every species (COUNCIL.md,
//! 2026-10-11), its leaves deposited as integer quanta (`Field::quantised`,
//! the host's adoption of early test 2). Allocation is `allocation::sizes`
//! throughout (R2). `grow` still takes F4's stepped build until fn-256's
//! executor moves with it (R3B.md).
mod capture;
mod capture_grown;
mod capture_hierarchy;
mod capture_layout;
mod capture_light;
mod capture_units;
mod census;
mod passes;
mod retire;
mod working;

pub use capture::{grow_capture, Capture};
pub use capture_grown::grow_capture_grown;
pub use capture_hierarchy::grow_capture_hierarchy_at;
pub use capture_layout::{grow_capture_layout, grow_capture_layout_at};
pub use capture_light::grow_capture_light_at;
pub use capture_units::{grow_capture_units, grow_capture_units_at};
pub use census::{Census, Cycle};
pub(super) use working::Working;

use super::sketch::Sketch;
use super::{lineage_order, start, Grower, Options, Request, Stage};
use crate::error::Result;
use crate::light::Field;
use crate::species::Species;
use crate::structure::{Structure, Vec3};

/// A tree grown through the one cycle: the tree, the phytomers grown for
/// it (the shed ones included), the seconds the build and its light took,
/// and the census, where one was taken.
pub struct Cycled {
    pub structure: Structure,
    pub grown: u64,
    pub seconds: f64,
    pub light_seconds: f64,
    pub census: Option<Census>,
}

/// Grows `species` through the one cycle.
pub fn grow_cycle(species: &Species, request: Request) -> Result<Cycled> {
    run(species, request, false, Working::default())
}

/// `grow_cycle`, with its census taken (COUNCIL.md's early test 1): each
/// cycle's records, lattice and phase seconds.
pub fn grow_census(species: &Species, request: Request) -> Result<Cycled> {
    run(species, request, true, Working::default())
}

/// The one cycle, its working set as `working` begins it (the tests'
/// references: every axis open, or none retired).
fn run(
    species: &Species,
    request: Request,
    census: bool,
    working: Working,
) -> Result<Cycled> {
    let started = crate::clock::seconds();
    let (mut g, keeps) = start(species, request, true, Options::default())?;
    (g.one, g.rules.one) = (true, true);
    g.working = Some(Box::new(working));
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    // Every lateral is grown: while it lives its leaves shade.
    g.rules.lasting = vec![None; species.states.len()].into();
    if census {
        g.census = Some(Box::default());
    }
    let (mut advanced, mut phases) = (Vec::new(), Phases::new());
    let mut clock = crate::clock::seconds();
    for cycle in 1..=request.age {
        g.year(cycle, cycle < request.age, &mut advanced, &mut |s| tick(&mut phases, &mut clock, s))?;
        g.count(cycle);
    }
    let (grown, light_seconds) = (g.rules.grown, g.light_seconds);
    let retired = g.working.as_ref().map_or(0, |w| w.retirement().roots());
    let mut taken = g.census.take().map(|c| *c);
    if let Some(c) = taken.as_mut() {
        c.ended(&g.axes);
    }
    clock = crate::clock::seconds();
    let settled = g.settle((request, Options::default()), (true, keeps), &mut |s| {
        tick(&mut phases, &mut clock, s)
    })?;
    if let Some(c) = taken.as_mut() {
        c.phases = phases;
        c.retired = retired;
    }
    Ok(Cycled {
        structure: lineage_order(settled.structure),
        grown,
        seconds: crate::clock::seconds() - started,
        light_seconds,
        census: taken,
    })
}

/// Seconds by the stage that ended them.
type Phases = std::collections::BTreeMap<String, f64>;

/// A stage's seconds, since the last one ended.
fn tick(phases: &mut Phases, clock: &mut f64, stage: Stage) {
    let now = crate::clock::seconds();
    *phases.entry(format!("{stage:?}")).or_default() += now - *clock;
    *clock = now;
}

impl Grower<'_> {
    /// The light at each of `tips` from the cycle's leaves on the lattice,
    /// deposited as integer quanta; with a census, the lattice recorded.
    pub(super) fn read_light(&mut self, tips: &[Vec3]) -> Result<Vec<f64>> {
        let started = crate::clock::seconds();
        let light = self.sketch.as_ref().expect("sketching").light;
        let field = Field::quantised(&light, &self.leaves, tips, self.threads)?;
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, self.threads, &|k| field.at(tips[k]));
        self.light_seconds += crate::clock::seconds() - started;
        if let Some(census) = self.census.as_mut() {
            census.lattice(&field, self.leaves.len());
        }
        Ok(read)
    }

    /// The census's records at the end of `cycle`.
    fn count(&mut self, cycle: u32) {
        if let Some(mut c) = self.census.take() {
            c.cycle(cycle, &self.axes, &self.live);
            self.census = Some(c);
        }
    }
}

#[cfg(test)]
mod tests;
