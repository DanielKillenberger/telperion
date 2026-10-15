//! What fn-209 F1's fine orders left behind them (fn-256 deleted the
//! fine roots grown below a cut): the canonical lineage order every tree
//! is returned in (`canonical.rs`); the device's unit tables
//! (`tables.rs`, `surface.rs`); and the site's coarse tree (`coarse.rs`),
//! the structure above a cut grown whole and settled through realized
//! mechanics' four phases (`expansion.rs`, `lay.rs`, `exchange.rs`) on its
//! fine roots' expectations, never grown, which fn-257 replaces.
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

pub(crate) use canonical::lineage_permutation;
pub use canonical::{canonical, digest, identities, lineage_order};
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
