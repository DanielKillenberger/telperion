//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::fixed` word for word (FINE-CONTRACT.md, host
//! decision 1), which every fine-root shader begins with, so a decision
//! the GPU takes is the CPU reference executor's to the bit.
use super::Prepared;
use crate::Result;
use telperion_core::presets::Family;

const WIDE: &str = include_str!("fine/wide.wgsl");
const KEYS: &str = include_str!("fine/lineage.wgsl");
const CONTRACT: &str = include_str!("fine/contract.wgsl");
const ZONES: &str = include_str!("fine/zones.wgsl");
const FEEDBACK_WIDE: &str = include_str!("fine/feedback_wide.wgsl");
const FEEDBACK: &str = include_str!("fine/feedback.wgsl");
const FEEDBACK_SITES: &str = include_str!("fine/feedback_sites.wgsl");
#[cfg(test)]
const FEEDBACK_TURN: &str = include_str!("fine/feedback_turn.wgsl");

/// The contract's WGSL: its wide words, its lineage keys, then its
/// arithmetic.
pub(super) fn contract() -> String {
    format!("{WIDE}\n{KEYS}\n{CONTRACT}")
}

/// The decision contract and the feedback contract's wide words, powers
/// and ripening (fn-211 task 8, CONTRACT-A.md): what the fine roots'
/// girth takes contract A's fixed point from (host, 2026-10-08).
pub(super) fn fixed_point() -> String {
    format!(
        "{}\n{FEEDBACK_WIDE}\n{FEEDBACK}\n{FEEDBACK_SITES}",
        contract()
    )
}

/// The decision contract and the feedback contract (fn-211 task 8,
/// CONTRACT-A.md): `telperion_space::fixed::feedback` word for word.
#[cfg(test)]
pub(super) fn feedback() -> String {
    format!(
        "{}\n{FEEDBACK_WIDE}\n{FEEDBACK}\n{FEEDBACK_TURN}\n{FEEDBACK_SITES}",
        contract()
    )
}

/// The contract and a growth unit's zones, for a shader that binds the
/// unit tables as `table` and `words`.
pub(super) fn zones() -> String {
    format!("{}\n{ZONES}", contract())
}

/// The device's kernels and tools for a generator's trees.
pub(super) struct Kit {
    kernels: roots::Kernels,
    tools: resident::Tools,
}

impl super::Generator {
    /// The bytes the fine executor keeps on the device between builds: its
    /// kernels' pool and its grower's regions (fn-212 task 10).
    #[cfg(test)]
    pub(super) fn fine_pool_bytes(&self) -> (u64, u64) {
        self.fine.get().map_or((0, 0), |kit| {
            (kit.kernels.pool.bytes(), kit.kernels.grower.bytes())
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
        budget::check(family, gpu.ledger.live(), budget::BROWSER)?;
        #[cfg(test)]
        if let Some(stated) = self.budget.get() {
            budget::check(family, gpu.ledger.live(), stated)?;
        }
        // The kernels are made inside the scopes, so a shader the device
        // refuses is the error reported.
        let scopes = super::io::scope(gpu);
        let kit = self.fine.get_or_init(|| Kit {
            kernels: roots::kernels(gpu),
            tools: resident::Tools::new(gpu),
        });
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
        // What the build's regions keep is what a same-size retune needs.
        let trimmed = kit.kernels.grower.trim(gpu);
        // A device error is the cause of any readback it failed, so it is
        // the one reported.
        super::io::errors(gpu, scopes).await?;
        trimmed?;
        Ok(built?.prepared)
    }
}

/// Every fine-root shader's source by name.
#[cfg(test)]
pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("fine contract probe", tests::source()),
        ("feedback probe", feedback_tests::source()),
        ("feedback slice", feedback_slice::source()),
        ("feedback stepped", feedback_stepped::source()),
        ("feedback lay", feedback_stepped::lay_source()),
        ("fine unit", units_tests::source()),
        ("fine chain grower", chains::source()),
        ("fine pack", chains::pack_source()),
        ("fine geometry", laid::source()),
        ("fine fades", shedding::source()),
        ("fine dress", dress::DRESS.to_string()),
        ("fine fold", dress::FOLD.to_string()),
        ("fine sweep", sweep::source()),
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
mod feedback_cases;
#[cfg(test)]
mod feedback_quotient;
#[cfg(test)]
mod feedback_slice;
#[cfg(test)]
mod feedback_stepped;
#[cfg(test)]
mod feedback_stepped_tests;
#[cfg(test)]
mod feedback_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod floret_stills;
mod florets;
#[cfg(test)]
mod florets_tests;
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
mod pool;
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
mod resident_tests;
mod rings;
mod roots;
#[cfg(test)]
mod route_tests;
#[cfg(test)]
mod seam_stills;
#[cfg(test)]
mod selectors;
mod shedding;
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
mod units;
#[cfg(test)]
mod units_tests;
