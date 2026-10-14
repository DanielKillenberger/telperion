//! A family's tree grown on the device year by year and within the year
//! (fn-270 R6, R7): `begin` for the family's age as its horizon, `grow_to`
//! any age within it, and `settle` the tree at any age in its last year
//! as a resident tree the renderer submits, prepared as a tree built at
//! once is (`build::placed`). The tree drawn now stays drawn while the
//! next stretch grows: nothing here lets it go (R2c), and a growth or a
//! draw the budget cannot hold beside it is refused by name.
use super::super::cycle::tree::GrowthState;
use super::super::{Generator, Prepared};
use super::build::{placeable, placed, stage, Timed};
use crate::Result;
use std::sync::Arc;
use telperion_core::pipeline::executor::{self, Growth};
use telperion_core::presets::Family;
use telperion_core::tree::Year;

/// A family's tree growing on one generator's device.
pub struct GrowingTree {
    identity: Arc<()>,
    family: Family,
    growth: Growth,
    state: GrowthState,
}

impl GrowingTree {
    /// The whole years grown; none once a failure spent the tree.
    pub fn committed(&self) -> Option<u32> {
        self.state.committed()
    }

    /// The most years it grows: the family's age.
    pub fn horizon(&self) -> u32 {
        self.growth.request.age
    }

    /// The family it grows.
    pub fn family(&self) -> &Family {
        &self.family
    }
}

/// `result`, a failure after the state's own call succeeded (`done`)
/// spending it (R2b): a refusal before anything moved leaves it as it
/// was, and a failure inside the call has spent it already.
fn spent_after<T>(state: &GrowthState, done: bool, result: Result<T>) -> Result<T> {
    if done && result.is_err() {
        state.spend();
    }
    result
}

/// The refusal of a tree another generator began.
fn foreign() -> crate::RenderError {
    telperion_core::Error::InvalidInput("generation renderer mismatch").into()
}

impl Generator {
    /// `family`'s tree before its first year, to grow at most the family's
    /// age from its seed. A family the device cannot grow or place is
    /// refused by name, and one whose least tables the budget cannot hold.
    pub async fn begin_growth(&self, family: &Family) -> Result<GrowingTree> {
        placeable(family)?;
        let growth = executor::growth(family)?;
        let at = (growth.request.seed, growth.request.age);
        let budget = self.device_budget();
        let state = self.on_device(async |kit| GrowthState::begin(&self.gpu, &kit.kernels, &growth, at, budget).await).await?;
        Ok(GrowingTree { identity: self.identity.clone(), family: family.clone(), growth, state })
    }

    /// `tree` grown to `age` years, as `GrowthState::grow_to` grows it: a
    /// request past the horizon, a year or more back, or not finite is
    /// refused by name before anything moves; a failure spends the tree.
    pub async fn grow_to(&self, tree: &mut GrowingTree, age: f64) -> Result<()> {
        if !Arc::ptr_eq(&self.identity, &tree.identity) {
            return Err(foreign());
        }
        let (state, mut moved) = (&mut tree.state, false);
        let grown = self
            .on_device(async |kit| {
                state.grow_to(&self.gpu, &kit.kernels, age).await?;
                moved = true;
                Ok(())
            })
            .await;
        spent_after(&tree.state, moved, grown)
    }

    /// `tree` drawn at `at`, within the last year grown, and prepared for
    /// the renderer as a resident tree, with how far its falling leaves
    /// have coloured (`Prepared::colouring`). The tree is left as it was.
    pub async fn settle(&self, tree: &GrowingTree, at: f64) -> Result<Prepared> {
        if !Arc::ptr_eq(&self.identity, &tree.identity) {
            return Err(foreign());
        }
        let mut drew = false;
        let built = self
            .on_device(async |kit| {
                let gpu = &self.gpu;
                let _capped = gpu.ledger.capped(self.device_budget());
                let mut t = Timed::default();
                // The draw refuses an `at` outside the last year, or a spent
                // tree, by name before anything is drawn.
                let drawn = stage(gpu, &mut t.settle, async || tree.state.settle(gpu, &kit.kernels, at).await).await?;
                drew = true;
                let fraction = at - (f64::from(tree.committed().unwrap_or(0)) - 1.0);
                let year = tree.growth.year(fraction).map(|y| Year { age: at, ..y });
                let built = placed(gpu, (self, &kit.tools), (&tree.family, &tree.growth), (drawn, year), t).await?;
                Ok((built, fraction))
            })
            .await;
        let (built, fraction) = spent_after(&tree.state, drew, built)?;
        let colouring = telperion_space::unfold::colouring(&tree.growth.species, fraction);
        Ok(Prepared { colouring, ..built.prepared })
    }
}
