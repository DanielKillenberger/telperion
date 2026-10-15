//! The tree once grown: its living apexes marked, its presences assigned,
//! shed, sized, thickened and laid.
use super::{Grower, Grown, Options, Pass, Record, Request, Stage};
use crate::contract::presence::{q, real, times};
use crate::error::{Error, Result};
use crate::geometry::{place, scale};
use crate::girth::{attachments, attachments_past, disused, thicken, Girth};
use crate::presence::assign;
use crate::sag;
use crate::shed::{shed, shed_past};
use crate::species::Species;
use crate::structure::{Origin, Structure};

impl<'a> Grower<'a> {
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

    /// Settles and, with `lay`, lays the grown tree; `keeps` where any PA
    /// reads the tree as grown (its shed pipes or its leaves' girth).
    pub(super) fn settle(
        self,
        (request, options): (Request, Options),
        (lay, keeps): (bool, bool),
        stage: &mut dyn FnMut(Stage),
    ) -> Result<Grown> {
        let shed = self.settle_shed((request, options), keeps, stage)?;
        shed.settle_lay(options, lay, stage)
    }

    /// The settle up to its order (fn-256 task 2's first step): each
    /// living apex marked, every presence assigned, the tree as grown
    /// thickened where any PA keeps it, shed, and its disused pipes kept.
    pub(super) fn settle_shed(
        mut self,
        (request, options): (Request, Options),
        keeps: bool,
        stage: &mut dyn FnMut(Stage),
    ) -> Result<Settling<'a>> {
        let (species, threads) = (self.species, self.threads);
        self.mark();
        stage(Stage::Pass(Pass::Mark));
        assign(&mut self.axes, &self.draws, species, threads);
        stage(Stage::Pass(Pass::Assign));
        let made = if options.settle || options.records {
            made(&self.axes)
        } else {
            Vec::new()
        };
        let (count, marked) = (self.rules.grown, self.marked);
        // The tree as grown, where any PA keeps shed pipes or thickens by
        // its leaves: what each shed branch had laid down, and the one
        // population the leaf term's mean is taken over at any retained
        // share.
        let grown = keeps.then(|| match self.working.as_ref() {
            Some(w) => {
                let r = w.retirement();
                attachments_past(&self.axes, species, (request.age, threads), &|i| {
                    r.frozen_root(i)
                })
            }
            None => attachments(&self.axes, species, request.age, threads),
        });
        stage(Stage::Pass(Pass::Attachments));
        let shed = match self.working.take() {
            Some(w) => {
                let r = w.retirement();
                let (given, gone) = (|i| r.root(i), |i| r.inside(i));
                shed_past(self.axes, species, request.age, (&given, &gone), None).0
            }
            None => shed(self.axes, species, request.age),
        };
        stage(Stage::Pass(Pass::Shed));
        let records = kept_records(made, &shed.index);
        let girth = grown.map_or_else(Girth::default, |g| disused(&g, &shed, species));
        stage(Stage::Pass(Pass::Disused));
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
            girth,
            index: shed.index,
            records,
            grown: count,
            marked,
            species,
            threads,
        })
    }
}

/// The tree settled as far as its order (`Grower::settle_shed`): shed,
/// its presences assigned, before its scale, girth and lay.
pub(super) struct Settling<'a> {
    pub structure: Structure,
    pub girth: Girth,
    pub index: Vec<usize>,
    pub records: Vec<Record>,
    pub grown: u64,
    pub marked: Vec<u64>,
    pub species: &'a Species,
    pub threads: usize,
}

impl Settling<'_> {
    /// The rest of the settle: the shed tree scaled, thickened and, with
    /// `lay`, laid and sagged.
    pub(super) fn settle_lay(
        self,
        options: Options,
        lay: bool,
        stage: &mut dyn FnMut(Stage),
    ) -> Result<Grown> {
        let Self {
            mut structure,
            girth,
            index,
            records,
            grown: count,
            marked,
            species,
            threads,
        } = self;
        scale(&mut structure, species, threads);
        stage(Stage::Pass(Pass::Scale));
        // Girth needs no geometry; placing reads it where wood meets the
        // ground.
        thicken(&mut structure, species, &girth, threads);
        stage(Stage::Pass(Pass::Thicken));
        let leaf_mean = girth
            .leaf_mean
            .or_else(|| crate::girth::leaf_mean(&structure, species));
        stage(Stage::Pass(Pass::LeafMean));
        stage(Stage::Settled);
        let done = move |structure| Grown {
            structure,
            grown: count,
            marked,
            records,
            leaf_mean,
            index,
        };
        if !lay || options.settle {
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

/// Every lateral bud made, as its bearer made it.
fn made(axes: &[crate::structure::Axis]) -> Vec<Record> {
    axes.iter()
        .enumerate()
        .filter_map(|(i, a)| {
            matches!(a.origin, Origin::Lateral { .. }).then_some(Record {
                origin: a.origin,
                pa: a.pa,
                birth: a.birth,
                lineage: a.lineage,
                vigour: a.vigour,
                axis: Some(i),
            })
        })
        .collect()
}

/// The records whose bearer is kept, renamed to the kept tree's indices.
fn kept_records(made: Vec<Record>, index: &[usize]) -> Vec<Record> {
    made.into_iter()
        .filter_map(|mut r| {
            let Origin::Lateral { parent, .. } = &mut r.origin else {
                return None;
            };
            *parent = index[*parent];
            if *parent == usize::MAX {
                return None;
            }
            r.axis = r.axis.map(|i| index[i]).filter(|&i| i != usize::MAX);
            Some(r)
        })
        .collect()
}
