//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::contract` by the same f32 operations
//! (fn-254), which every fine-root shader begins with, so a decision the
//! GPU takes is the CPU reference executor's but where two reals stand
//! within their last places of its bound.
use super::Prepared;
use crate::Result;
use telperion_core::presets::Family;

const WIDE: &str = include_str!("fine/wide.wgsl");
const KEYS: &str = include_str!("fine/lineage.wgsl");
const CONTRACT: &str = include_str!("fine/contract.wgsl");
const ZONES: &str = include_str!("fine/zones.wgsl");
const ARITH: &str = include_str!("fine/arith.wgsl");

/// The contract's WGSL: its wide words, its lineage keys, then its
/// arithmetic.
pub(super) fn contract() -> String {
    contract_with(ARITH)
}

/// The contract with its marked arithmetic `arith`: the plain operations
/// (`fine/arith.wgsl`), or the device cycle's choice (`cycle::Arith`).
pub(super) fn contract_with(arith: &str) -> String {
    format!("{WIDE}\n{KEYS}\n{CONTRACT}\n{arith}")
}

/// The contract and a growth unit's zones, for a shader that binds the
/// unit tables as `table` and `words`.
pub(super) fn zones() -> String {
    zones_with(ARITH)
}

/// `zones` with the contract's marked arithmetic `arith`.
pub(super) fn zones_with(arith: &str) -> String {
    format!("{}\n{ZONES}", contract_with(arith))
}

/// The device's kernels and tools for a generator's trees.
pub(super) struct Kit {
    kernels: roots::Kernels,
    tools: resident::Tools,
}

pub use budget::Admitted;
#[cfg(target_arch = "wasm32")]
pub(crate) use budget::BROWSER;

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
        self.kit();
        crate::curve::passes(&self.gpu);
        super::io::programs_ready(&self.gpu, scopes).await
    }

    /// The fine executor's kernels and tools, made on first use.
    fn kit(&self) -> &Kit {
        self.fine.get_or_init(|| Kit {
            kernels: roots::kernels(&self.gpu),
            tools: resident::Tools::new(&self.gpu),
        })
    }

    /// `family` prepared resident, its fine roots settled, drawn and
    /// placed on the device (fn-212 task 10, host decision 2): the
    /// structure above the cut and the roots laid on the CPU through the
    /// pipeline, the rest on the device, nothing read back per node.
    pub(super) async fn prepare_fine(&self, family: &Family) -> Result<Prepared> {
        let gpu = &self.gpu;
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
        let built = resident::build(gpu, self, &kit.kernels, &kit.tools, family).await;
        // The regions are growth scratch: no frame reads them, so the
        // build lets them go (fn-258 R2).
        kit.kernels.grower.release();
        // A device error is the cause of any readback it failed, so it is
        // the one reported.
        super::io::errors(gpu, scopes).await?;
        Ok(built?.prepared)
    }
}

/// Every fine-root shader's source by name.
#[cfg(test)]
pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("fine contract probe", tests::source()),
        ("fine unit", units_tests::source()),
        ("fine chain grower", chains::source()),
        ("fine pack", chains::pack_source()),
        ("fine geometry", laid::source()),
        ("fine fades", shedding::source()),
        ("fine dress", dress::DRESS.to_string()),
        ("fine fold", dress::FOLD.to_string()),
        ("fine sweep", sweep::source()),
        ("fine sort", sort::SORT.to_string()),
        ("fine stations", stations::source()),
        ("fine rings", rings::source()),
        ("fine handoff", handoff::source()),
    ]
    .into_iter()
    .chain(steps::sources())
    .collect()
}

#[cfg(test)]
mod chain_tests;
#[cfg(test)]
mod cluster_tests;
// The browser's budget is checked in a browser alone; natively it is tested.
#[cfg(test)]
mod blossom_measure;
#[cfg(test)]
mod blossom_parity;
#[cfg(test)]
mod blossom_trees;
#[cfg(test)]
mod birch_tests;
#[cfg_attr(not(any(test, target_arch = "wasm32")), allow(dead_code))]
mod budget;
mod chains;
mod device;
#[cfg(test)]
mod drawn_tests;
mod dress;
#[cfg(test)]
mod dress_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod floret_stills;
mod florets;
#[cfg(test)]
mod florets_tests;
#[cfg(test)]
mod preset_agree_tests;
#[cfg(test)]
mod ground_tests;
mod group;
mod handoff;
#[cfg(test)]
mod hang_tests;
#[cfg(test)]
mod hostrun;
mod laid;
#[cfg(test)]
mod mirror_tests;
#[cfg(test)]
mod option3_tests;
#[cfg(test)]
mod placed_tests;
#[cfg(test)]
mod profile_tests;
#[cfg(test)]
mod readback;
mod realized;
#[cfg(test)]
mod realized_tests;
#[cfg(test)]
mod realized_timing;
mod resident;
#[cfg(test)]
mod repeat_tests;
#[cfg(test)]
mod resident_tests;
mod rings;
#[cfg(test)]
mod room_tests;
mod roots;
#[cfg(test)]
mod route_tests;
#[cfg(test)]
mod seam_stills;
#[cfg(test)]
mod selectors;
mod shedding;
mod sort;
#[cfg(test)]
mod speed_tests;
mod stations;
#[cfg(test)]
mod stations_tests;
#[cfg(test)]
mod step_probe;
mod steps;
mod stubs;
mod sweep;
#[cfg(test)]
mod sweep_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod timing_tests;
pub(super) mod units;
#[cfg(test)]
mod units_tests;
