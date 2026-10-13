//! Growth that continues on the device (fn-270 R2): the family's tree
//! begun for a horizon, its build resident between calls and paused
//! (`build_pause.rs`), grown on to any age within the horizon, and drawn
//! at any age within its last year as `tree::drawn` draws the tree grown
//! in one call. A request the state cannot meet is refused by name
//! before anything moves; a failure while it grows (a count past a table
//! or limit, a device fault or loss) spends the state, and every later
//! call is refused by name. A draw never moves the state.
//!
//! Admission (R2c): every call grows and draws under the state's device
//! budget, which the ledger holds against every buffer live on the device
//! together (the state, the tree drawn now, the settle's scratch, a move's
//! new tables beside its old). Nothing here lets the drawn tree go to
//! make room: a growth whose overlap cannot fit is refused by name, the
//! state spent (R2b), and the tree drawn now stays as it is.
use super::super::resident::fold::Dressed;
use super::build::Tables;
use super::build_grow::{Growing, Sizes};
use super::drawn::{dress, trunk, Rows, Trunk};
use super::settle_run::{settle, Settled};
use super::tree::{rows, Kernels};
use super::chains;
use crate::{Gpu, Result};
use telperion_core::pipeline::executor::Growth;
use telperion_space::{admitted, within_year, Capture, ChainTables, SettleTo};

/// A family's tree growing on the device.
pub(in crate::generation) struct GrowthState {
    tables: Capture,
    t: Tables,
    rows: Vec<f32>,
    scale: f32,
    /// The bytes the device may hold while it grows and draws.
    budget: u64,
    /// The build, paused; none once a failure while it grew spent it.
    growing: Option<Growing>,
    /// Whether a draw failed (a fault, the budget, the device lost): the
    /// state is spent, as a failed growth spends it.
    failed: std::cell::Cell<bool>,
}

/// The engine's refusal as the device's, by its name.
fn named(e: telperion_space::Error) -> crate::RenderError {
    super::super::io::error(e.to_string())
}

/// The refusal of a state a failure spent.
fn spent() -> crate::RenderError {
    super::super::io::error("state.failed: a growth that failed cannot go on or be drawn".into())
}

impl GrowthState {
    /// The tree before its first year, to grow at most `horizon` years
    /// from `seed` under `budget` bytes, its tables sized from the
    /// species' hint.
    pub(in crate::generation) async fn begin(gpu: &Gpu, k: &Kernels, growth: &Growth, (seed, horizon): (u64, u32), budget: u64) -> Result<Self> {
        let hint = super::build_arena::hint(&growth.species, horizon)?;
        Self::sized(gpu, k, growth, (seed, horizon), (Sizes::hinted(hint)?, budget)).await
    }

    /// `begin`, its tables sized from `sizes`.
    pub(super) async fn sized(gpu: &Gpu, k: &Kernels, growth: &Growth, (seed, horizon): (u64, u32), (sizes, budget): (Sizes, u64)) -> Result<Self> {
        let _capped = gpu.ledger.capped(budget);
        let request = telperion_space::Request { age: horizon, seed, ..growth.request };
        let species = &growth.species;
        let tables = telperion_space::grow_capture_tables(species, request).map_err(named)?;
        let chain = ChainTables::cycle(species, horizon).map_err(named)?;
        let t = Tables::new(gpu, &tables, &chain)?;
        let growing = Growing::begin(gpu, &k.build, &t, &tables, sizes).await?;
        let growing = growing.paused(gpu, (&t, &tables)).await?;
        let (rows, scale) = (rows(&growth.rows), growth.scale as f32);
        Ok(Self { tables, t, rows, scale, budget, growing: Some(growing), failed: Default::default() })
    }

    /// The whole years grown; none once spent.
    pub(in crate::generation) fn committed(&self) -> Option<u32> {
        self.usable().ok().map(|g| g.committed)
    }

    /// The paused build; refused by name once a failure spent it.
    fn usable(&self) -> Result<&Growing> {
        self.growing.as_ref().filter(|_| !self.failed.get()).ok_or_else(spent)
    }

    /// `result`, its failure spending the state.
    fn latched<T>(&self, result: Result<T>) -> Result<T> {
        self.failed.set(self.failed.get() || result.is_err());
        result
    }

    /// The tree grown to `age` years: every whole cycle to its ceiling,
    /// none where `age` falls within the last year grown, then paused.
    pub(in crate::generation) async fn grow_to(&mut self, gpu: &Gpu, k: &Kernels, age: f64) -> Result<()> {
        let committed = self.usable()?.committed;
        let to = admitted(committed, self.t.age, age).map_err(named)?;
        if to == committed {
            return Ok(());
        }
        let _capped = gpu.ledger.capped(self.budget);
        let g = self.growing.take().expect("a build");
        let (t, tables) = (&self.t, &self.tables);
        let g = g.grow_to(gpu, (&k.build, t, tables), (to, false)).await?;
        self.growing = Some(g.paused(gpu, (t, tables)).await?);
        Ok(())
    }

    /// The tree at the age grown, settled and laid from the paused build,
    /// which it leaves as it was.
    pub(super) async fn settled(&self, gpu: &Gpu, k: &Kernels) -> Result<Settled> {
        let g = self.usable()?;
        let _capped = gpu.ledger.capped(self.budget);
        let s = settle(gpu, (&k.build, &k.settle), (&g.b, &self.t), (&self.t.settle_pa, SettleTo::Laid)).await?;
        if s.error[0] != 0 {
            return Err(super::tree::unlaid(s.error));
        }
        Ok(s)
    }

    /// The tree drawn at `at`, within the last year grown: its drawn
    /// records and its trunk's measures, a snapshot of its own. An `at`
    /// it cannot draw is refused by name; a draw that fails spends the
    /// state, and every snapshot drawn before stays whole.
    pub(in crate::generation) async fn settle(&self, gpu: &Gpu, k: &Kernels, at: f64) -> Result<(Dressed, Trunk)> {
        let committed = self.usable()?.committed;
        let fraction = within_year(committed, at).map_err(named)? as f32;
        let drawn = async {
            let s = self.settled(gpu, k).await?;
            let _capped = gpu.ledger.capped(self.budget);
            let planned = chains::plan(gpu, (&k.chains, &k.build.hier.scan), &s).await?;
            let at = Rows { rows: &self.rows, age: committed, scale: self.scale, fraction };
            let dressed = dress(gpu, &k.drawn, (&s, &planned), &at)?;
            let measured = trunk(gpu, &k.drawn, &dressed).await?;
            Ok((dressed, measured))
        };
        self.latched(drawn.await)
    }
}

#[cfg(test)]
mod presets;
#[cfg(test)]
mod tests;
