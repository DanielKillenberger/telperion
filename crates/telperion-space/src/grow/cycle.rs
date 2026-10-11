//! fn-255 R1: one cycle for every species (DESIGN.md section 1). The
//! whole-tree grower runs every phase every cycle for every species and
//! every light: the rough layout and the light read, the balance and the
//! allotment, each zero setting a neutral term in them, never another way
//! of building. Light is the lattice for every species (COUNCIL.md,
//! 2026-10-11); its deposit is today's f32 sum or the quantised integer
//! one COUNCIL.md's early test 2 measures. Allocation is
//! `allocation::sizes` throughout (R2).
//!
//! An experiment behind its own entry point until the council's early
//! tests decide the next step: `grow` never builds one.
mod census;

pub use census::{Census, Cycle};

use super::sketch::Sketch;
use super::{lineage_order, start, Grower, Options, Request, Stage};
use crate::error::Result;
use crate::light::Field;
use crate::species::Species;
use crate::structure::{Structure, Vec3};

/// How the cycle's leaves are deposited on the lattice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Deposit {
    /// Today's: each corner's f32 weighted area summed in the leaves' order.
    #[default]
    F32,
    /// Integer quanta of 2^-20 m², a u32 a node, overflow refused by name.
    Quantised,
}

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

/// Grows `species` through the one cycle, its leaves deposited by
/// `deposit`.
pub fn grow_cycle(species: &Species, request: Request, deposit: Deposit) -> Result<Cycled> {
    run(species, request, deposit, None)
}

/// `grow_cycle`, with its census taken (COUNCIL.md's early tests 1 and 2):
/// each cycle's records, lattice and phase seconds, and, with `beside`,
/// the other deposit read beside the one growth uses.
pub fn grow_census(species: &Species, request: Request, deposit: Deposit, beside: bool) -> Result<Cycled> {
    run(species, request, deposit, Some(beside))
}

fn run(species: &Species, request: Request, deposit: Deposit, census: Option<bool>) -> Result<Cycled> {
    let started = crate::clock::seconds();
    let (mut g, keeps) = start(species, request, true, Options::default())?;
    (g.one, g.deposit) = (true, deposit);
    g.sketch.get_or_insert_with(|| Sketch::new(request.light));
    // Every lateral is grown: while it lives its leaves shade.
    g.rules.lasting = vec![None; species.states.len()].into();
    if let Some(beside) = census {
        g.census = Some(Box::new(Census::new(beside)));
    }
    let (mut advanced, mut phases) = (Vec::new(), Phases::new());
    let mut clock = crate::clock::seconds();
    for cycle in 1..=request.age {
        g.year(cycle, cycle < request.age, &mut advanced, &mut |s| tick(&mut phases, &mut clock, s))?;
        g.count(cycle);
    }
    let (grown, light_seconds) = (g.rules.grown, g.light_seconds);
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
    /// deposited by the grower's deposit; with a census, the lattice
    /// recorded and the other deposit read beside.
    pub(super) fn read_light(&mut self, tips: &[Vec3]) -> Result<Vec<f64>> {
        let started = crate::clock::seconds();
        let field = self.field(self.deposit, tips)?;
        let read = self.sample(&field, tips);
        self.light_seconds += crate::clock::seconds() - started;
        if let Some(mut census) = self.census.take() {
            let other = match self.deposit {
                Deposit::F32 => Deposit::Quantised,
                Deposit::Quantised => Deposit::F32,
            };
            let beside = match census.beside {
                true => self.sample(&self.field(other, tips)?, tips),
                false => read.clone(),
            };
            census.lattice(&field, self.leaves.len(), &read, &beside);
            self.census = Some(census);
        }
        Ok(read)
    }

    fn field(&self, deposit: Deposit, tips: &[Vec3]) -> Result<Field> {
        let light = self.sketch.as_ref().expect("sketching").light;
        match deposit {
            Deposit::F32 => Ok(Field::on(&light, &self.leaves, tips, self.threads)),
            Deposit::Quantised => Field::quantised(&light, &self.leaves, tips, self.threads),
        }
    }

    fn sample(&self, field: &Field, tips: &[Vec3]) -> Vec<f64> {
        let mut read = vec![0.0; tips.len()];
        crate::cores::fill(&mut read, 1 << 10, self.threads, &|k| field.at(tips[k]));
        read
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
