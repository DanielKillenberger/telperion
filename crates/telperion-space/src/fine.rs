//! Fine orders grown in the expansion stage (fn-209 F1; DESIGN-FINE.md,
//! accepted with Astra's changes, ASTRA-REVIEW-FINE.md), under realized
//! mechanics (task 6; fn-211's DESIGN-INTERLEAVE.md, "After Astra's
//! review", host decision 3).
//!
//! The CPU grows the structure above a cut by the engine's own grower; a
//! bud below the cut is made where it stands and handed on, its stand-in
//! keeping its place among its bearer's children. The cut is chosen
//! before growth, by the closed form's expected kept phytomers of the bud
//! (`closed_form::predictor`, light-neutral), so which buds are fine never
//! depends on what they grow into. The expansion grows every fine root
//! whole from its bud alone (`realized.rs`), and the two settle together
//! in four phases: each root reports its realized totals up (its
//! subtree's living, its base radius, its load), the structure above the
//! cut settles on them exactly as the whole tree does, and hands each
//! root its bearer's scale and laid frames down.
//!
//! The cut decides where the work runs, never what is computed (Astra's
//! change 2): every pass is the whole tree's own, and every sum a root's
//! subtree enters is taken in the whole tree's order, so the tree is
//! today's whole-tree engine's to the bit, at every cut, in every order
//! of its items and in any chunking of them (`tests/realized.rs`).
//! Where no bud reads light only; a species that reads light follows
//! fn-211's stepped executor.
mod canonical;
mod coarse;
mod exchange;
mod expansion;
mod lay;
mod realized;
mod step;
mod surface;
mod tables;
#[cfg(test)]
mod tests;

pub use canonical::{canonical, digest, identities, lineage_order};
pub(crate) use canonical::lineage_permutation;
pub use coarse::{grow_coarse, Coarse, Stand};
pub(crate) use realized::Made;
pub use surface::{ChainTables, PaRow};
pub use tables::{UnitZone, Units};

/// Where the coarse tree is cut, in expected kept phytomers (a bud at or
/// below it stands as its expectation), and the cores (0: every core).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Plan {
    pub cut: f64,
    pub threads: usize,
}

/// The seconds each phase of the expansion took.
#[derive(Default)]
pub(crate) struct Clock {
    pub phases: Vec<f64>,
}

impl Clock {

    /// An executor's step, awaited and timed as a phase.
    pub async fn timed<T>(&mut self, step: impl std::future::Future<Output = T>) -> T {
        let started = crate::clock::seconds();
        let out = step.await;
        self.phases.push(crate::clock::seconds() - started);
        out
    }
}
