//! The tree once grown: its living apexes marked, its presences assigned,
//! shed, sized, thickened and laid.
use super::{Grower, Grown, Pass, Request, Stage};
use crate::contract::presence::{q, real, times};
use crate::error::{Error, Result};
use crate::geometry::{place, scale};
use crate::girth::{shares, thicken};
use crate::presence::assign;
use crate::sag;
use crate::shed::{shed, shed_past};
use crate::species::Species;
use crate::structure::{Axis, Structure};
use std::sync::Arc;

impl Grower {
    /// Marks every apex living at the tree's age with how much it lives
    /// on, and sizes every bud that woke by what its bearer kept of it.
    pub(super) fn mark(&mut self) {
        let age = self.rules.age;
        for apex in &self.live {
            let living = self.living(*apex);
            let draws = &mut self.draws[apex.axis];
            (draws.alive, draws.living) = (true, living);
        }
        let retired = self.working.as_ref().map(|w| w.retirement());
        for woken in &self.woken {
            // A bud inside a retired subtree was taken with it.
            if retired.is_some_and(|r| r.inside(woken.axis)) {
                continue;
            }
            let kept = woken.kept(&self.axes, &self.draws, &self.successor, age);
            let made = &mut self.draws[woken.axis].birth[1];
            *made = real(times(q(*made), q(kept)));
        }
    }

    /// Settles and, with `lay`, lays the grown tree.
    pub(super) fn settle(
        self,
        request: Request,
        lay: bool,
        stage: &mut dyn FnMut(Stage),
    ) -> Result<Grown> {
        let shed = self.settle_shed(request, stage)?;
        shed.settle_lay(lay, stage)
    }

    /// The settle up to its order (fn-256 task 2's first step): each
    /// living apex marked, every presence assigned, and shed.
    pub(super) fn settle_shed(
        mut self,
        request: Request,
        stage: &mut dyn FnMut(Stage),
    ) -> Result<Settling> {
        let (owned, threads) = (Arc::clone(&self.species), self.threads);
        let species = &*owned;
        self.mark();
        stage(Stage::Pass(Pass::Mark));
        // Each phytomer widened into the tree's, to the bit (fn-358).
        let held = std::mem::take(&mut self.axes);
        let mut axes: Vec<Axis> = held.into_iter().map(Axis::settled).collect();
        assign(&mut axes, &self.draws, species, threads);
        stage(Stage::Pass(Pass::Assign));
        let (count, marked) = (self.rules.grown, self.marked);
        let shed = match self.working.take() {
            Some(w) => {
                let r = w.retirement();
                let (given, gone) = (|i| r.root(i), |i| r.inside(i));
                let stands = r.standing();
                let fates = (&given as _, &gone as _, &stands[..]);
                shed_past(axes, species, request.age, fates, None).0
            }
            None => shed(axes, species, request.age),
        };
        stage(Stage::Pass(Pass::Shed));
        let shares = shares(&shed.axes, &shed.index, &shed.fade);
        let structure = Structure {
            age: request.age,
            pas: species.states.len(),
            axes: shed.axes,
        };
        if structure.phytomer_count() == 0 {
            return Err(Error::Collapsed);
        }
        Ok(Settling {
            structure,
            shares,
            grown: count,
            marked,
            species: owned,
            threads,
        })
    }
}

/// The tree settled as far as its order (`Grower::settle_shed`): shed,
/// its presences assigned, before its scale, girth and lay.
pub(super) struct Settling {
    pub structure: Structure,
    /// Each kept axis's share as drawn (`girth::shares`).
    pub shares: Vec<f64>,
    pub grown: u64,
    pub marked: Vec<u64>,
    pub species: Arc<Species>,
    pub threads: usize,
}

impl Settling {
    /// The rest of the settle: the shed tree scaled, thickened and, with
    /// `lay`, laid and sagged.
    pub(super) fn settle_lay(self, lay: bool, stage: &mut dyn FnMut(Stage)) -> Result<Grown> {
        let Self {
            mut structure,
            shares,
            grown: count,
            marked,
            species,
            threads,
        } = self;
        let species = &*species;
        scale(&mut structure, species, threads);
        stage(Stage::Pass(Pass::Scale));
        // Girth needs no geometry; placing reads it where wood meets the
        // ground.
        thicken(&mut structure, species, &shares);
        stage(Stage::Pass(Pass::Thicken));
        stage(Stage::Settled);
        let done = move |structure| Grown {
            structure,
            grown: count,
            marked,
        };
        if !lay {
            return Ok(done(structure));
        }
        place(&mut structure, species, None, threads)?;
        stage(Stage::Pass(Pass::Place));
        // Sag bends the tree as it stands under the load it carries, and
        // leaves its girth as it was.
        if sag::any(species) {
            let levers = sag::levers(&structure, species, threads);
            stage(Stage::Pass(Pass::Levers));
            place(&mut structure, species, Some(&levers), threads)?;
            stage(Stage::Pass(Pass::SagPlace));
        }
        // Wood that stands exactly at its draw has no size.
        if structure
            .axes
            .iter()
            .flat_map(|a| &a.phytomers)
            .all(|p| p.scale == 0.0)
        {
            return Err(Error::Collapsed);
        }
        stage(Stage::Laid);
        Ok(done(structure))
    }
}
