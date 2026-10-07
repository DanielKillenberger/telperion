//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::fixed` word for word (FINE-CONTRACT.md, host
//! decision 1), which every fine-root shader begins with, so a decision
//! the GPU takes is the CPU reference executor's to the bit.
const WIDE: &str = include_str!("fine/wide.wgsl");
const CONTRACT: &str = include_str!("fine/contract.wgsl");
const ZONES: &str = include_str!("fine/zones.wgsl");
const FEEDBACK_WIDE: &str = include_str!("fine/feedback_wide.wgsl");
const FEEDBACK: &str = include_str!("fine/feedback.wgsl");
const FEEDBACK_SITES: &str = include_str!("fine/feedback_sites.wgsl");

/// The contract's WGSL: its wide words, then its arithmetic.
pub(super) fn contract() -> String {
    format!("{WIDE}\n{CONTRACT}")
}

/// The decision contract and the feedback contract (fn-211 task 8,
/// CONTRACT-A.md): `telperion_space::fixed::feedback` word for word.
pub(super) fn feedback() -> String {
    format!(
        "{}\n{FEEDBACK_WIDE}\n{FEEDBACK}\n{FEEDBACK_SITES}",
        contract()
    )
}

/// The contract and a growth unit's zones, for a shader that binds the
/// unit tables as `table` and `words`.
pub(super) fn zones() -> String {
    format!("{}\n{ZONES}", contract())
}

/// Every fine-root shader's source by name.
pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("fine contract probe", tests::source()),
        ("feedback probe", feedback_tests::source()),
        ("feedback slice", feedback_slice::source()),
        ("fine unit", units::source()),
        ("fine chain grower", chains::source()),
        ("fine pack", chains::pack_source()),
        ("fine geometry", laid::source()),
        ("fine fades", shedding::source()),
    ]
    .into_iter()
    .chain(steps::sources())
    .collect()
}

#[cfg(test)]
mod chain_tests;
#[cfg(test)]
mod chains;
#[cfg(test)]
mod device;
#[cfg(test)]
mod feedback_cases;
#[cfg(test)]
mod feedback_slice;
#[cfg(test)]
mod feedback_tests;
#[cfg(test)]
mod hostrun;
#[cfg(test)]
mod laid;
#[cfg(test)]
mod pool;
#[cfg(test)]
mod profile_tests;
#[cfg(test)]
mod readback;
#[cfg(test)]
mod realized;
#[cfg(test)]
mod realized_tests;
#[cfg(test)]
mod realized_timing;
#[cfg(test)]
mod roots;
#[cfg(test)]
mod shedding;
#[cfg(test)]
mod speed_tests;
#[cfg(test)]
mod step_probe;
#[cfg(test)]
mod steps;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod units;
