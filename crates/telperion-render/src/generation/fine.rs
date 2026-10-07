//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::fixed` word for word (FINE-CONTRACT.md, host
//! decision 1), which every fine-root shader begins with, so a decision
//! the GPU takes is the CPU reference executor's to the bit.
use super::Prepared;
use crate::Result;
use telperion_core::presets::Family;

const WIDE: &str = include_str!("fine/wide.wgsl");
const CONTRACT: &str = include_str!("fine/contract.wgsl");
const ZONES: &str = include_str!("fine/zones.wgsl");

/// The contract's WGSL: its wide words, then its arithmetic.
pub(super) fn contract() -> String {
    format!("{WIDE}\n{CONTRACT}")
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
    /// `family` prepared resident, its fine roots settled, drawn and
    /// placed on the device (fn-212 task 10, host decision 2): the
    /// structure above the cut and the roots laid on the CPU through the
    /// pipeline, the rest on the device, nothing read back per node.
    pub(super) fn prepare_fine(&self, family: &Family) -> Result<Prepared> {
        let gpu = &self.gpu;
        let kit = self.fine.get_or_init(|| Kit {
            kernels: roots::kernels(gpu),
            tools: resident::Tools::new(gpu),
        });
        let scopes = super::io::scope(gpu);
        let built = resident::build(gpu, self, &kit.kernels, &kit.tools, family);
        let errors = pollster::block_on(super::io::errors(gpu, scopes));
        let built = built?;
        errors?;
        Ok(built.prepared)
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
mod chains;
mod device;
#[cfg(test)]
mod drawn_tests;
mod dress;
#[cfg(test)]
mod dress_tests;
#[cfg(test)]
mod fixture;
mod group;
mod handoff;
#[cfg(test)]
mod hostrun;
mod laid;
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
