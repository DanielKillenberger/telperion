//! The one cycle paused between years (fn-270): the grower begun for a
//! horizon, grown on to any age within it, and settled at the age grown
//! from a copy of what the settle reads, the grower left as it was.
//! `grow_cycle` is this loop run to the horizon in one call. A request
//! the grower cannot meet is refused by name before anything moves; a
//! failure while it grows leaves it spent, and every later call is
//! refused by name (spec R2b).
use super::super::sketch::Sketch;
use super::super::{start, Grower, Options, Request, Stage};
use super::{tick, Census, Cycled, Phases, Working};
use crate::error::{refuse, Result};
use crate::fine::lineage_order;
use crate::species::Species;
use crate::structure::Structure;
use crate::unfold::{present, Presented};
use std::hash::Hasher;

/// How a paused grower retires its shed subtrees.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Retiring {
    /// The rule (fn-270 R3): by the fade at the age grown, a fading
    /// subtree waiting.
    AtAge,
    /// None retired, every axis open: the tests' reference.
    Never,
}

/// The one cycle between years.
pub struct Paused<'a> {
    g: Grower<'a>,
    keeps: bool,
    request: Request,
    committed: u32,
    advanced: Vec<usize>,
    phases: Phases,
    clock: f64,
    /// Whether a cycle failed while it grew: the grower is spent.
    failed: bool,
    /// Whether it keeps the tree drawn a year before the age grown, which
    /// a draw within the year moves from (fn-270 R5a): the thinnest wood
    /// the drawing draws, where it does; and that tree.
    presenting: bool,
    previous: Option<Structure>,
}

/// The whole cycle a request for `age` commits a grower at `committed`
/// under `horizon` to (fn-270 R2a', R2b): its ceiling; `committed` where
/// the request falls within the last year grown. A non-finite or negative
/// age, one a year or more back, and one past the horizon are refused by
/// name.
pub fn admitted(committed: u32, horizon: u32, age: f64) -> Result<u32> {
    if !age.is_finite() || age < 0.0 {
        return refuse("age", "an age is finite and not negative");
    }
    if age > f64::from(horizon) {
        return refuse("age.horizon", "a tree grows no further than its horizon");
    }
    if age <= f64::from(committed) - 1.0 {
        return refuse("age.backward", "a tree grows only forward");
    }
    Ok(committed.max(age.ceil() as u32))
}

/// How far into the last year grown, `committed`, `at` stands: in (0, 1];
/// an `at` outside that year, or before any year is grown, refused by
/// name.
pub fn within_year(committed: u32, at: f64) -> Result<f64> {
    if committed == 0 {
        return refuse("at", "a tree is drawn once it has grown a year");
    }
    let fraction = at - (f64::from(committed) - 1.0);
    if !(fraction > 0.0 && fraction <= 1.0) {
        return refuse("at", "a tree is drawn within the last year grown");
    }
    Ok(fraction)
}

impl<'a> Paused<'a> {
    /// The grower before its first cycle, for a tree of at most
    /// `horizon.age` cycles, with a census where `census`.
    pub fn begin(
        species: &'a Species,
        horizon: Request,
        retiring: Retiring,
        census: bool,
    ) -> Result<Self> {
        let working = match retiring {
            Retiring::AtAge => Working::default(),
            Retiring::Never => Working::reference(),
        };
        Self::with(species, horizon, census, working)
    }

    pub(super) fn with(
        species: &'a Species,
        request: Request,
        census: bool,
        working: Working,
    ) -> Result<Self> {
        let (mut g, keeps) = start(species, request, Options::default())?;
        (g.one, g.rules.one) = (true, true);
        g.working = Some(Box::new(working));
        g.sketch.get_or_insert_with(|| Sketch::new(request.light));
        // Every lateral is grown: while it lives its leaves shade.
        g.rules.lasting = vec![None; species.states.len()].into();
        if census {
            g.census = Some(Box::default());
        }
        let clock = crate::clock::seconds();
        let (advanced, phases) = (Vec::new(), Phases::new());
        let (presenting, previous) = (false, None);
        Ok(Self {
            g,
            keeps,
            request,
            committed: 0,
            advanced,
            phases,
            clock,
            failed: false,
            presenting,
            previous,
        })
    }

    /// The grower keeping the tree drawn a year before the age grown, so
    /// it can be drawn within its last year (`settle_at`).
    pub fn presenting(mut self) -> Self {
        self.presenting = true;
        self
    }

    /// The tree drawn a year before the age grown, where it keeps one.
    pub fn previous(&self) -> Option<&Structure> {
        self.previous.as_ref()
    }

    /// The cycles grown.
    pub fn committed(&self) -> u32 {
        self.committed
    }

    /// The most cycles it grows.
    pub fn horizon(&self) -> u32 {
        self.request.age
    }

    /// Refused by name where a failure left the grower spent.
    pub fn usable(&self) -> Result<()> {
        match self.failed {
            true => refuse(
                "state.failed",
                "a growth that failed cannot go on or be drawn",
            ),
            false => Ok(()),
        }
    }

    /// The census so far, where one is taken.
    pub fn census(&self) -> Option<&Census> {
        self.g.census.as_deref()
    }

    /// Every cycle up to `age` grown; none where it has grown them.
    /// Past the horizon it is refused by name, as is an age before the one
    /// grown, and a spent grower; a cycle that fails spends it.
    pub fn grow_to(&mut self, age: u32) -> Result<()> {
        self.usable()?;
        if age > self.request.age {
            return refuse("age.horizon", "a tree grows no further than its horizon");
        }
        if age < self.committed {
            return refuse("age.backward", "a tree grows only forward");
        }
        let grown = self.grown_to(age);
        self.failed = grown.is_err();
        grown
    }

    /// `grow_to`, its request admitted: where it keeps the year before,
    /// the tree drawn a year before `age` is settled on the way.
    fn grown_to(&mut self, age: u32) -> Result<()> {
        if self.presenting && age > self.committed {
            self.cycles(age - 1)?;
            self.previous = match self.committed {
                0 => None,
                _ => Some(self.settle()?),
            };
        }
        self.cycles(age)
    }

    /// Every cycle up to `age` grown.
    fn cycles(&mut self, age: u32) -> Result<()> {
        let (phases, clock) = (&mut self.phases, &mut self.clock);
        for cycle in self.committed + 1..=age {
            // The last cycle's layout and light are read by no bud.
            let sketching = cycle < self.request.age;
            self.g
                .year(cycle, sketching, &mut self.advanced, &mut |s| {
                    tick(phases, clock, s)
                })?;
            self.g.count(cycle);
            self.committed = cycle;
        }
        Ok(())
    }

    /// The tree at the age grown, settled and laid from a copy: the grower
    /// is left as it was. A spent grower is refused by name.
    pub fn settle(&self) -> Result<Structure> {
        self.usable()?;
        let at = self.committed;
        let request = Request {
            age: at,
            ..self.request
        };
        let copy = self.g.settled_copy(at);
        let grown = copy.settle(
            (request, Options::default()),
            (true, self.keeps),
            &mut |_| {},
        )?;
        Ok(lineage_order(grown.structure))
    }

    /// The tree at the age grown drawn at `at` within its last year
    /// (`unfold::present`), `at` checked before anything is settled. Within
    /// the year only a grower that keeps the year before draws it.
    pub fn settle_at(&self, at: f64) -> Result<Presented> {
        self.usable()?;
        let f = within_year(self.committed, at)?;
        if f >= 1.0 {
            return Ok(Presented::whole(self.settle()?));
        }
        if !self.presenting {
            return refuse(
                "at",
                "a tree is drawn within its year where it keeps the year before",
            );
        }
        let tree = self.settle()?;
        let empty = Structure {
            age: 0,
            pas: tree.pas,
            axes: Vec::new(),
        };
        let before = self.previous.as_ref().unwrap_or(&empty);
        Ok(present(tree, before, self.g.species, f))
    }

    /// The tree at the horizon, settled as `grow_cycle` settles it, with
    /// its census.
    pub(super) fn finish(self, started: f64) -> Result<Cycled> {
        let Self {
            g,
            keeps,
            request,
            mut phases,
            ..
        } = self;
        let mut g = g;
        let (grown, light_seconds) = (g.rules.grown, g.light_seconds);
        let retired = g.working.as_ref().map_or(0, |w| w.retirement().roots());
        let mut taken = g.census.take().map(|c| *c);
        if let Some(c) = taken.as_mut() {
            c.ended(&g.axes);
        }
        let mut clock = crate::clock::seconds();
        let settled = g.settle(
            (request, Options::default()),
            (true, keeps),
            &mut |s: Stage| tick(&mut phases, &mut clock, s),
        )?;
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

    /// A digest of everything the next cycle and the settle read, to the
    /// bit: the grower's axes, draws, living apexes, sleeping and woken
    /// buds, rough layout, leaves, working set and retirement, and its
    /// counts. Two growers with one digest grow on alike.
    pub fn digest(&self) -> u64 {
        let g = &self.g;
        let mut h = Digest(std::collections::hash_map::DefaultHasher::new());
        let pencils = g.sketch.as_ref().map(|s| &s.pencils);
        let counts = (g.rules.grown, g.rules.age, self.committed);
        write(
            &mut h,
            &(
                counts,
                &g.horizon,
                &g.marked,
                &g.units,
                &g.successor,
                &g.roots,
            ),
        );
        write(&mut h, &(&g.axes, &g.draws, &g.live, &g.asleep, &g.woken));
        write(&mut h, &(pencils, &g.leaves, &g.working));
        h.0.finish()
    }
}

impl<'a> Grower<'a> {
    /// What the settle reads of this grower, copied, as a tree of age
    /// `at` (fn-270 task 1): the axes, draws, living apexes, woken buds
    /// and retirement; none of the growth's scratch, which it never reads.
    fn settled_copy(&self, at: u32) -> Grower<'a> {
        let mut rules = self.rules.clone();
        rules.age = at;
        Grower {
            rules,
            species: self.species,
            threads: self.threads,
            share: self.share,
            horizon: self.horizon.clone(),
            marked: self.marked.clone(),
            axes: self.axes.clone(),
            draws: self.draws.clone(),
            live: self.live.clone(),
            units: self.units.clone(),
            successor: self.successor.clone(),
            roots: self.roots.clone(),
            asleep: Vec::new(),
            woken: self.woken.clone(),
            sketch: None,
            leaves: Vec::new(),
            spare: Default::default(),
            one: self.one,
            light_seconds: self.light_seconds,
            jobs: [0; 2],
            census: None,
            working: self.working.as_ref().map(|w| Box::new(w.settled_copy())),
        }
    }
}

/// A hasher written to as text.
struct Digest(std::collections::hash_map::DefaultHasher);

impl std::fmt::Write for Digest {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.write(s.as_bytes());
        Ok(())
    }
}

/// `value`'s Debug text into `h`: every float to its last bit, -0 apart.
fn write(h: &mut Digest, value: &dyn std::fmt::Debug) {
    std::fmt::Write::write_fmt(h, format_args!("{value:?}")).expect("a digest");
}

/// A digest of `tree` to the bit, as `Paused::digest` takes one.
pub fn tree_digest(tree: &Structure) -> u64 {
    let mut h = Digest(std::collections::hash_map::DefaultHasher::new());
    write(&mut h, tree);
    h.0.finish()
}

#[cfg(test)]
mod tests;
