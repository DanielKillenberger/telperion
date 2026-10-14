//! Growth that continues on the device (fn-270 R2): the family's tree
//! begun for a horizon, its build resident between calls and paused
//! (`build_pause.rs`), grown on to any age within the horizon, and drawn
//! at any age within its last year as `tree::drawn` draws the tree grown
//! in one call. A request the state cannot meet is refused by name
//! before anything moves; a failure while it grows (a count past a table
//! or limit, a device fault or loss) spends the state, and every later
//! call is refused by name. A draw never moves the state. Within its
//! last year the tree is drawn from the year before's (`present.rs`,
//! R5 and R5a), which the state keeps: its node table as settled.
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
use super::present::present;
use super::settle_device::Table;
use super::settle_run::{settle, Settled, View};
use super::tree::{rows, Kernels};
use super::chains;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::pipeline::executor::Growth;
use telperion_space::{admitted, within_year, Capture, ChainTables, SettleTo};

/// A family's tree growing on the device.
pub(in crate::generation) struct GrowthState {
    tables: Capture,
    t: Tables,
    rows: Vec<f32>,
    /// Each PA's elongation and expansion in days, its internode's spread,
    /// its cluster leaves and its budburst's day (`present.wgsl`'s
    /// `unfold`).
    unfold: Vec<f32>,
    scale: f32,
    /// The tree drawn a year before the age grown: none before the
    /// second year.
    previous: Option<Previous>,
    /// The bytes the device may hold while it grows and draws.
    budget: u64,
    /// The build, paused; none once a failure while it grew spent it.
    growing: Option<Growing>,
    /// Whether a draw failed (a fault, the budget, the device lost): the
    /// state is spent, as a failed growth spends it.
    failed: std::cell::Cell<bool>,
}

/// What a state's tables start from: the hint, capped to the budget
/// (`Sizes::margins`), or stated sizes.
pub(super) enum Start {
    Hint(super::build_arena::Hint),
    #[cfg_attr(not(test), allow(dead_code))]
    Sizes(Sizes),
}

/// A tree as settled and laid: its node table, layout and each depth's
/// first axis.
struct Previous {
    nt: Owned,
    table: Table,
    starts: Vec<u32>,
}

impl Previous {
    fn view(&self) -> View<'_> {
        View { nt: &self.nt, table: &self.table, starts: &self.starts }
    }
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
        Self::sized(gpu, k, growth, (seed, horizon), (Start::Hint(hint), budget)).await
    }

    /// `begin`, its tables started from `start`.
    pub(super) async fn sized(gpu: &Gpu, k: &Kernels, growth: &Growth, (seed, horizon): (u64, u32), (start, budget): (Start, u64)) -> Result<Self> {
        let _capped = gpu.ledger.capped(budget);
        let request = telperion_space::Request { age: horizon, seed, ..growth.request };
        let species = &growth.species;
        let tables = telperion_space::grow_capture_tables(species, request).map_err(named)?;
        let chain = ChainTables::cycle(species, horizon).map_err(named)?;
        let t = Tables::new(gpu, &tables, &chain)?;
        let growing = match start {
            Start::Hint(hint) => Growing::within(gpu, &k.build, &t, &tables, hint).await?,
            Start::Sizes(sizes) => Growing::begin(gpu, &k.build, &t, &tables, sizes).await?,
        };
        let growing = growing.paused(gpu, (&t, &tables)).await?;
        let (rows, scale) = (rows(&growth.rows), growth.scale as f32);
        let unfold = species.states.iter().flat_map(unfold_row).collect();
        let growing = Some(growing);
        Ok(Self { tables, t, rows, unfold, scale, previous: None, budget, growing, failed: Default::default() })
    }

    /// The whole years grown; none once spent.
    pub(in crate::generation) fn committed(&self) -> Option<u32> {
        self.usable().ok().map(|g| g.committed)
    }

    /// The paused build; refused by name once a failure spent it.
    fn usable(&self) -> Result<&Growing> {
        self.growing.as_ref().filter(|_| !self.failed.get()).ok_or_else(spent)
    }

    /// The state spent by a failure after a call on it succeeded: the
    /// placement of a tree it drew, or a device error its scopes caught.
    pub(in crate::generation) fn spend(&self) {
        self.failed.set(true);
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
        let mut g = self.growing.take().expect("a build");
        // The year before `to` drawn on the way, last year's let go first.
        self.previous = None;
        if to - 1 > committed {
            g = self.grown(gpu, k, g, to - 1).await?;
        }
        if to > 1 {
            let s = self.settled_from(gpu, k, &g).await?;
            self.previous = Some(Previous { nt: s.nt, table: s.table, starts: s.kept_starts });
        }
        self.growing = Some(self.grown(gpu, k, g, to).await?);
        Ok(())
    }

    /// `g` grown to `to` and paused.
    async fn grown(&self, gpu: &Gpu, k: &Kernels, g: Growing, to: u32) -> Result<Growing> {
        let (t, tables) = (&self.t, &self.tables);
        let g = g.grow_to(gpu, (&k.build, t, tables), (to, false)).await?;
        g.paused(gpu, (t, tables)).await
    }

    /// The tree at the age grown, settled and laid from the paused build,
    /// which it leaves as it was.
    pub(super) async fn settled(&self, gpu: &Gpu, k: &Kernels) -> Result<Settled> {
        let g = self.usable()?;
        let _capped = gpu.ledger.capped(self.budget);
        self.settled_from(gpu, k, g).await
    }

    /// The tree `g` holds, settled and laid.
    async fn settled_from(&self, gpu: &Gpu, k: &Kernels, g: &Growing) -> Result<Settled> {
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
            let scan = &k.build.hier.scan;
            let mut settled = Some(s);
            let presented = match fraction < 1.0 {
                true => {
                    let s = settled.take().expect("a settled tree");
                    let p = present(gpu, (&k.present, scan), (s.view(), self.previous.as_ref().map(Previous::view)), &self.unfold, fraction).await?;
                    // This year's settled tree let go, and freed, before the
                    // presented one is drawn: never both beside the drawn.
                    drop(s);
                    super::super::io::complete(gpu).await?;
                    Some(p)
                }
                false => None,
            };
            let view = presented.as_ref().map_or_else(|| settled.as_ref().expect("a settled tree").view(), |p| p.view());
            let planned = chains::plan(gpu, (&k.chains, scan), view).await?;
            let leaf = presented.as_ref().map(|p| &p.leaf);
            let at = Rows { rows: &self.rows, age: committed, scale: self.scale, fraction, leaf };
            let dressed = dress(gpu, &k.drawn, (view, &planned), &at)?;
            let measured = trunk(gpu, &k.drawn, &dressed).await?;
            Ok((dressed, measured))
        };
        self.latched(drawn.await)
    }
}

#[cfg(test)]
mod presets;
#[cfg(test)]
mod present_tests;
#[cfg(test)]
pub(super) mod tests;

/// A PA's row as `present.wgsl` reads it: its elongation and expansion,
/// its internode's spread, its cluster's count as a word
/// (`telperion_space::word`, fn-277) and its budburst's day.
fn unfold_row(s: &telperion_space::PaState) -> [f32; 5] {
    let cluster = f32::from_bits(telperion_space::word(s.cluster_leaves));
    let day = telperion_space::unfold::Season::of(s).budburst;
    [s.elongation as f32, s.expansion as f32, s.form.internode_spread as f32, cluster, day as f32]
}
