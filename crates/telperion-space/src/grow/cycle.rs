//! fn-255 R1: one cycle for every species (DESIGN.md section 1). The
//! whole-tree grower runs every phase every cycle for every species and
//! every light: the rough layout and the light read, the balance and the
//! allotment, each zero setting a neutral term in them, never another way
//! of building. Light is the lattice for every species (COUNCIL.md,
//! 2026-10-11), its leaves deposited as integer quanta (`Field::quantised`,
//! the host's adoption of early test 2). Allocation is `allocation::sizes`
//! throughout (R2). `grow` is this cycle, the CPU's reference for the
//! device's (fn-256).
mod capture;
mod capture_grown;
mod capture_hierarchy;
mod capture_layout;
mod capture_light;
mod capture_settled;
mod capture_units;
mod census;
mod passes;
mod paused;
mod retire;
mod working;

pub use capture::{grow_capture, Capture};
pub use capture_grown::{grow_capture_grown, grow_capture_grown_at, grow_capture_tables};
pub use capture_hierarchy::grow_capture_hierarchy_at;
pub use capture_layout::{grow_capture_layout, grow_capture_layout_at};
pub use capture_light::grow_capture_light_at;
pub use capture_settled::{grow_capture_settle_tables, grow_capture_settled, grow_capture_retired, grow_capture_settled_at, grow_cycle_is_the_reference, settle_sag_turn, SettleTo};
pub use capture_units::{grow_capture_units, grow_capture_units_at};
pub use census::{Census, Cycle};
pub use paused::{admitted, tree_digest, within_year, Paused, Retiring};
pub(super) use working::Working;

use super::{Grower, Request, Stage};
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
/// references: every axis open, or none retired), grown to its age in one
/// call.
fn run(
    species: &Species,
    request: Request,
    census: bool,
    working: Working,
) -> Result<Cycled> {
    let started = crate::clock::seconds();
    let mut paused = Paused::with(species, request, census, working)?;
    paused.grow_to(request.age)?;
    paused.finish(started)
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
            c.cycle((cycle, self.rules.grown), &self.axes, &self.live);
            c.cycles.last_mut().expect("a cycle").jobs = std::mem::take(&mut self.jobs);
            self.census = Some(c);
        }
    }
}

#[cfg(test)]
mod tests;

/// The girth two trees of one shape stand apart by (fn-270 R4: retirement
/// freezes a closed subtree's girth, so the tree retiring nothing is the
/// same tree to the bit but for its radii and the pose they bend): every
/// phytomer's radius's relative shift, its median and its most; none where
/// anything else differs.
pub(crate) fn girth_apart(a: &Structure, b: &Structure) -> Option<(f64, f64)> {
    // Everything but the girth and the pose a girth bends (its sag) the
    // same to the bit: topology, PAs, vigour, presences, sizes and lights.
    let bare = |t: &Structure| {
        let mut t = t.clone();
        for axis in &mut t.axes {
            (axis.base, axis.heading, axis.side, axis.joined) = (Default::default(), Default::default(), Default::default(), None);
            for p in &mut axis.phytomers {
                (p.radius, p.tip, p.heading, p.side) = (0.0, Default::default(), Default::default(), Default::default());
            }
        }
        t
    };
    if bare(a) != bare(b) {
        return None;
    }
    let mut shift: Vec<f64> = (a.axes.iter().zip(&b.axes))
        .flat_map(|(x, y)| x.phytomers.iter().zip(&y.phytomers))
        .map(|(p, q)| if p.radius == q.radius { 0.0 } else { ((p.radius - q.radius) / q.radius.abs().max(1e-12)).abs() })
        .collect();
    shift.sort_by(f64::total_cmp);
    let most = shift.last().copied().unwrap_or(0.0);
    Some((shift.get(shift.len() / 2).copied().unwrap_or(0.0), most))
}
