//! The stepped executor's coarse run (fn-211.4, fn-214 task 2): the
//! engine's own grower holding the structure above a cut, stepped
//! through every cycle's phases with each fine root standing as its
//! aggregate (`stands.rs`), never grown, with only per-root totals and
//! the light lattice crossing between them. The site's coarse field is
//! its one reader (`fine::grow_coarse`); fn-257 replaces it. The fine
//! roots once grown each by a grower of its own went with fn-256.
//!
//! A fine root is a lateral whose bearer stands above the cut and whose
//! expected kept phytomers (the closed form's `count`) are at most the
//! cut. Each cycle:
//!
//! 1. **(i) balance**: each stand's totals at its placeholder; the coarse
//!    grower decides the root's shedding there.
//! 2. **(ii) demand**: each stand's lit presence; the coarse grower splits
//!    its vigour with it at the placeholder.
//! 3. **(iii) normalisers**: the coarse grower's per-PA sums with the
//!    stands', and the buds sized by them.
//! 4. **(iv)** the coarse grower grows, wakes and lays; new roots stand;
//!    its leaves and the stands' are deposited into one lattice, and each
//!    reads its own.
mod feedback;
mod handover;
mod phases;
mod standing;
mod stands;

use super::Grower;
use super::Request;
use crate::error::{Error, Result};
pub(crate) use standing::stand;
pub(crate) use stands::Stood;
use std::sync::Arc;

/// The closed form's expected kept phytomers of a bud of a PA born in a
/// cycle: what the cut reads.
pub(super) type Counts = Arc<dyn Fn(usize, u32) -> f64 + Send + Sync>;

pub(super) struct Executor<'a> {
    pub coarse: Grower<'a>,
    pub counts: Counts,
    pub cut: f64,
    pub request: Request,
    /// Where the roots stand as their aggregates, never grown (fn-214).
    pub stands: stands::Stands,
}

impl<'a> Executor<'a> {
    /// An executor about to grow `coarse`'s first cycle, its roots at
    /// `cut` by `counts`, standing on `stands`.
    pub(super) fn new(coarse: Grower<'a>, (counts, cut): (Counts, f64), request: Request, stands: stands::Stands) -> Self {
        Self { coarse, counts, cut, request, stands }
    }

    fn cycle(&mut self, cycle: u32) -> Result<()> {
        let first_new = self.coarse.axes.len();
        let advanced: Vec<usize> = self.coarse.live.iter().map(|a| a.axis).collect();
        self.balance(cycle);
        let factors = self.allot();
        self.stand_advance(cycle, &factors);
        // Phase (iv): the coarse grower grows, wakes and lays its own.
        let sketching = self.coarse.sketch.is_some() && cycle < self.request.age;
        let g = &mut self.coarse;
        let live = std::mem::take(&mut g.live);
        g.grow_live(live, cycle, &mut |_| {})?;
        g.wake(cycle)?;
        if sketching {
            g.sketch(cycle, &advanced)?;
        }
        if self.coarse.rules.grown > u64::from(self.request.budget) {
            return Err(Error::Budget {
                limit: self.request.budget,
            });
        }
        if sketching || self.coarse.sketch.is_none() {
            self.stand_over(first_new);
        }
        if sketching {
            self.shade(cycle);
        }
        Ok(())
    }

    /// The last cycle a root made in `cycle` runs to before anything
    /// crosses between it and the structure above: its light and layout
    /// at a cycle's end, and the balance or allotment reading its wood at
    /// a cycle's start.
    pub(super) fn reach(&self, cycle: u32) -> u32 {
        let c = &self.coarse;
        let before = c.balances() || c.allots();
        let after = |k: u32| c.sketch.is_some() && k < self.request.age;
        let mut last = cycle;
        while last < self.request.age && !after(last) && !before {
            last += 1;
        }
        last
    }
}
