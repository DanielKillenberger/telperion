//! A tree built resident on the device, whole (fn-256 task 3, DESIGN.md
//! 12): the one cycle grows and draws it (`cycle/tree.rs`), and these
//! stages fold, sweep, station, ring and place it (`build.rs`), so a
//! harness build reads only summaries back and holds no node of the tree
//! on the CPU.
use super::Prepared;
use crate::Result;
use telperion_core::presets::Family;

/// The device's kernels and tools for a generator's trees.
pub(super) struct Kit {
    kernels: super::cycle::tree::Kernels,
    tools: build::Tools,
}

pub use budget::Admitted;
#[cfg(target_arch = "wasm32")]
pub(crate) use budget::BROWSER;

impl Kit {
    /// The kernels and the tools made side by side (`io::pair`).
    fn new(gpu: &crate::Gpu) -> Self {
        let (kernels, tools) = super::io::pair(gpu, || super::cycle::tree::Kernels::new(gpu), || build::Tools::new(gpu));
        Self { kernels, tools }
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
        self.resident.get_or_init(|| super::io::pair(gpu, || Kit::new(gpu), || drop(crate::curve::passes(gpu))).0);
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

    /// The device's kernels and tools, made on first use.
    fn kit(&self) -> &Kit {
        self.resident.get_or_init(|| Kit::new(&self.gpu))
    }

    /// `family` prepared resident, grown, drawn and placed on the device
    /// whole (fn-256 task 3), nothing read back per node.
    pub(super) async fn prepare_resident(&self, family: &Family) -> Result<Prepared> {
        let gpu = &self.gpu;
        // What was let go before this build (a tree, a still's buffers) is
        // charged until the device is done with it: waited for, so the
        // budget counts what is held, not what is on its way out (fn-256
        // task 5, `Ledger::destroy`).
        super::io::complete(gpu).await?;
        // A browser build over its device budget is refused before it starts.
        #[cfg(target_arch = "wasm32")]
        budget::check(family, self.gpu.ledger.live(), budget::BROWSER)?;
        #[cfg(test)]
        if let Some(stated) = self.budget.get() {
            budget::check(family, self.gpu.ledger.live(), stated)?;
        }
        // The kernels are made inside the scopes, so a shader the device
        // refuses is the error reported.
        let scopes = super::io::scope(gpu);
        let kit = self.kit();
        if let Err(refused) = crate::programs::ready(gpu).await {
            // The scopes popped; a lost device, then the refusal, as
            // `programs::first_cause` orders them.
            let _ = super::io::errors(gpu, scopes).await;
            return Err(gpu.lost().unwrap_or(refused));
        }
        #[cfg(test)]
        if self.fail_next.take() {
            // A storage buffer the device may also map: a validation error.
            let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::MAP_READ;
            let refused = crate::buffer::Owned::create(
                gpu,
                &wgpu::BufferDescriptor {
                    label: Some("refused"),
                    size: 16,
                    usage,
                    mapped_at_creation: false,
                },
            );
            drop(refused);
        }
        let built = build::build(gpu, self, &kit.kernels, &kit.tools, family).await;
        // The build's scratch, let go as it returned, is charged until the
        // device is done with it: waited for, so what the device holds
        // after a build is the tree it hands over (`Ledger::destroy`).
        let freed = super::io::complete(gpu).await;
        // A device error is the cause of any readback it failed, so it is
        // the one reported.
        super::io::errors(gpu, scopes).await?;
        freed?;
        Ok(built?.prepared)
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
        ("resident handoff", handoff::source()),
    ]
}

// The browser's expected peak is checked in a browser alone; natively it
// is tested, and the native budget caps every build.
#[cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]
mod budget;
#[cfg(test)]
mod agree_tests;
mod boots;
#[cfg(test)]
mod boots_tests;
mod build;
#[cfg(test)]
mod hang_tests;
#[cfg(test)]
mod repeat_tests;
#[cfg(test)]
mod room_tests;
#[cfg(test)]
mod route_tests;
#[cfg(test)]
mod stalk_tests;
pub(super) mod fold;
mod florets;
mod handoff;
mod rings;
mod sort;
mod stations;
mod sweep;
