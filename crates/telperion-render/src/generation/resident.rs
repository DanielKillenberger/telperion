//! A tree built resident on the device, whole (fn-256 task 3, DESIGN.md
//! 12): the one cycle grows and draws it (`cycle/tree.rs`), and these
//! stages fold, sweep, station, ring and place it (`build.rs`), so a
//! harness build reads only summaries back and holds no node of the tree
//! on the CPU.
use super::cycle::tree::Kernels;
use super::Prepared;
use crate::Result;
use telperion_core::presets::Family;

/// The device's kernels and tools for a generator's trees: a field's
/// tools, and a build's drawing where a build is made (fn-319 R1).
pub(super) struct Kit {
    kernels: Kernels,
    tools: build::Tools,
    drawing: std::cell::OnceCell<build::Drawing>,
}

pub use budget::Admitted;
#[cfg(target_arch = "wasm32")]
pub(crate) use budget::BROWSER;

impl Kit {
    /// A field's: the kernels and a field's tools made side by side
    /// (`io::pair`), no drawing.
    fn new(gpu: &crate::Gpu) -> Self {
        let (kernels, tools) =
            super::io::pair(gpu, || Kernels::new(gpu), || build::Tools::new(gpu));
        let drawing = Default::default();
        Self {
            kernels,
            tools,
            drawing,
        }
    }

    /// A build's: a field's kit and the drawing made side by side.
    fn whole(gpu: &crate::Gpu) -> Self {
        let (kit, drawing) = super::io::pair(gpu, || Self::new(gpu), || build::Drawing::new(gpu));
        let _ = kit.drawing.set(drawing);
        kit
    }

    /// A build's tools, the drawing made where it was not.
    fn full(&self, gpu: &crate::Gpu) -> build::Full<'_> {
        let drawing = self.drawing.get_or_init(|| build::Drawing::new(gpu));
        (&self.tools, drawing)
    }

    /// The kernels alone.
    fn kernels(&self) -> &Kernels {
        &self.kernels
    }

    /// The kernels and a field's tools: never a build's drawing.
    fn field(&self) -> (&Kernels, &build::Tools) {
        (&self.kernels, &self.tools)
    }

    /// The kernels and a build's tools.
    fn building<'a>(&'a self, gpu: &crate::Gpu) -> (&'a Kernels, build::Full<'a>) {
        (&self.kernels, self.full(gpu))
    }
}

impl super::Generator {
    /// Whether `family` fits the page's budget beside the tree `renderer`
    /// draws now, only once it is let go, or not at all (by name).
    /// A build keeps nothing on the device once it is done (fn-258), so
    /// what the device holds now is what a build must fit beside.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn admit(family: &Family, renderer: &crate::Renderer, budget: u64) -> Result<Admitted> {
        let held = renderer.gpu().ledger.live();
        budget::admit(family, held, Self::tree_buffer_bytes(renderer), budget)
    }

    /// Every program a build dispatches, compiled before one is asked for:
    /// the page calls this as soon as its device exists, so the browser
    /// compiles in the background and no build compiles again (fn-243 R3).
    pub async fn prepare_programs(&self) -> Result<()> {
        let scopes = super::io::scope(&self.gpu);
        let gpu = &self.gpu;
        // The wood's passes made beside the kit.
        let kit = self.resident.get_or_init(|| {
            super::io::pair(gpu, || Kit::whole(gpu), || drop(crate::curve::passes(gpu))).0
        });
        // A kit a field's preparation made first gains the drawing and the
        // wood's passes (each made once a device).
        kit.full(gpu);
        drop(crate::curve::passes(gpu));
        super::io::programs_ready(&self.gpu, scopes).await
    }

    /// Every program a field dispatches, compiled before one is asked
    /// for: the kernels and a field's tools, never a build's sweep, rings
    /// or the wood's passes (fn-319 R1). A refused program, or a device
    /// lost meanwhile, is refused by name.
    pub async fn prepare_field_programs(&self) -> Result<()> {
        let scopes = super::io::scope(&self.gpu);
        self.kit();
        super::io::programs_ready(&self.gpu, scopes).await
    }

    /// The device bytes a build may hold: a test's stated budget, else
    /// the browser's, or natively the configured one (`budget::native`).
    fn device_budget(&self) -> u64 {
        #[cfg(test)]
        if let Some(stated) = self.budget.get() {
            return stated;
        }
        #[cfg(target_arch = "wasm32")]
        return budget::BROWSER;
        #[cfg(not(target_arch = "wasm32"))]
        budget::native(&self.gpu, self.gpu.ledger.live())
    }

    /// `family`'s field read out on the device to `n` cells a side over its
    /// own box (fn-257 R8, `telperion_core::field::Frame`), its limb systems
    /// opened by laterals of `limb_order` or less (the family's own where
    /// none is given). A flowering tree's florets are boxed and counted
    /// on the device too, within `flowers_agree`'s bounds of the CPU's
    /// (fn-319 R5). Only the grid is read back; nothing stays on the
    /// device. On a lost device it is refused by name.
    pub async fn grow_field(
        &self,
        family: &Family,
        n: u32,
        limb_order: Option<u32>,
    ) -> Result<telperion_core::field::Grid> {
        self.field(family, limb_order, field::Ask::Over(n)).await
    }

    /// `family`'s field read out to a grid named outright: the CPU's, so a
    /// test compares the executors cell by cell.
    #[cfg(test)]
    pub(super) async fn grow_field_in(
        &self,
        family: &Family,
        frame: telperion_core::field::Frame,
        limb_order: Option<u32>,
    ) -> Result<telperion_core::field::Grid> {
        self.field(family, limb_order, field::Ask::In(frame)).await
    }

    async fn field(
        &self,
        family: &Family,
        limb_order: Option<u32>,
        ask: field::Ask,
    ) -> Result<telperion_core::field::Grid> {
        telperion_core::field::Frame::side(ask.n())?;
        let gpu = &self.gpu;
        if let Some(lost) = gpu.lost() {
            return Err(lost);
        }
        // A field's tools alone: the work never sees a build's drawing.
        let read = self
            .on_device(Kit::field, async |(kernels, tools)| {
                field::grow(gpu, (self, kernels, tools), family, limb_order, ask).await
            })
            .await;
        read.map_err(|e| gpu.lost().unwrap_or(e))
    }

    /// The device's loss, where it is lost: the reason no field grows or
    /// answers on it (fn-257).
    pub fn lost(&self) -> Option<crate::RenderError> {
        self.gpu.lost()
    }

    /// The device's kernels and tools, made on first use.
    fn kit(&self) -> &Kit {
        self.resident.get_or_init(|| Kit::new(&self.gpu))
    }

    /// `family` prepared resident, grown, drawn and placed on the device
    /// whole (fn-256 task 3), nothing read back per node.
    pub(super) async fn prepare_resident(&self, family: &Family) -> Result<Prepared> {
        // What was let go before this build is waited for, so the budget
        // counts what is held (`on_device`).
        super::io::complete(&self.gpu).await?;
        // A browser build over its device budget is refused before it starts.
        #[cfg(target_arch = "wasm32")]
        budget::check(family, self.gpu.ledger.live(), budget::BROWSER)?;
        #[cfg(test)]
        if let Some(stated) = self.budget.get() {
            budget::check(family, self.gpu.ledger.live(), stated)?;
        }
        let take = |kit| Kit::building(kit, &self.gpu);
        let built = self
            .on_device(take, async |(kernels, tools)| {
                #[cfg(test)]
                if self.fail_next.take() {
                    // A storage buffer the device may also map: a validation error.
                    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ;
                    let refused = crate::buffer::Owned::create(
                        &self.gpu,
                        &wgpu::BufferDescriptor {
                            label: Some("refused"),
                            size: 16,
                            usage,
                            mapped_at_creation: false,
                        },
                    );
                    drop(refused);
                }
                build::build(&self.gpu, self, kernels, tools, family).await
            })
            .await?;
        Ok(built.prepared)
    }

    /// `work` on what `take` takes of the device's kit, inside its error
    /// scopes: a device error is the cause of any readback it failed, so
    /// it is the one reported, and a program the device refused is
    /// reported before any work starts.
    async fn on_device<'a, K, T>(
        &'a self,
        take: impl FnOnce(&'a Kit) -> K,
        work: impl AsyncFnOnce(K) -> Result<T>,
    ) -> Result<T> {
        let gpu = &self.gpu;
        // What was let go before (a tree, a still's buffers) is charged
        // until the device is done with it: waited for, so the budget
        // counts what is held, not what is on its way out (fn-256 task 5,
        // `Ledger::destroy`).
        super::io::complete(gpu).await?;
        // The kernels are made inside the scopes, so a shader the device
        // refuses is the error reported.
        let scopes = super::io::scope(gpu);
        let kit = take(self.kit());
        if let Err(refused) = crate::programs::ready(gpu).await {
            // The scopes popped; a lost device, then the refusal, as
            // `programs::first_cause` orders them.
            let _ = super::io::errors(gpu, scopes).await;
            return Err(gpu.lost().unwrap_or(refused));
        }
        let done = work(kit).await;
        // The work's scratch, let go as it returned, is charged until the
        // device is done with it.
        let freed = super::io::complete(gpu).await;
        super::io::errors(gpu, scopes).await?;
        freed?;
        done
    }
}

/// Every resident stage's shader source by name.
#[cfg(test)]
pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("resident bases", boots::source()),
        ("resident fold", fold::FOLD.to_string()),
        ("resident sweep", sweep::source()),
        ("resident sort", sort::SORT.to_string()),
        ("resident stations", stations::source()),
        ("resident scan", stations::SCAN.to_string()),
        ("resident rings", rings::source()),
    ]
    .into_iter()
    .chain(field::sources())
    .collect()
}

// The browser's expected peak is checked in a browser alone; natively it
// is tested, and the native budget caps every build.
#[cfg(test)]
pub(super) mod agree_tests;
mod boots;
#[cfg(test)]
mod boots_tests;
pub(super) mod budget;
mod build;
#[cfg(test)]
pub(super) mod digest_tests;
#[cfg(test)]
mod drawn_memory_tests;
#[cfg(test)]
mod field_probe_tests;
#[cfg(test)]
mod full_card_tests;
#[cfg(test)]
mod grow_in_peak;
mod growing;
#[cfg(test)]
mod growing_cost;
#[cfg(test)]
mod growing_tests;
#[cfg(test)]
mod organ_tests;
pub use growing::GrowingTree;
mod field;
#[cfg(test)]
mod field_flowers_tests;
#[cfg(test)]
mod field_params_tests;
#[cfg(test)]
mod field_tests;
#[cfg(test)]
mod field_time_tests;
mod florets;
pub(super) mod fold;
#[cfg(test)]
mod hang_tests;
#[cfg(test)]
mod pack_tests;
#[cfg(test)]
mod rachis_tests;
#[cfg(test)]
mod repeat_tests;
mod rings;
#[cfg(test)]
mod room_tests;
#[cfg(test)]
mod route_tests;
mod sort;
#[cfg(test)]
mod stalk_tests;
mod stations;
mod sweep;
#[cfg(test)]
mod thin_tests;
