//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::fixed` word for word (FINE-CONTRACT.md, host
//! decision 1), which every fine-root shader begins with, so a decision
//! the GPU takes is the CPU reference executor's to the bit.
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

/// Every fine-root shader's source by name.
pub(super) fn sources() -> Vec<(&'static str, String)> {
    vec![
        ("fine contract probe", tests::source()),
        ("fine unit", units::source()),
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
mod chains;
#[cfg(test)]
mod device;
#[cfg(test)]
mod dress;
#[cfg(test)]
mod dress_tests;
#[cfg(test)]
mod fixture;
#[cfg(test)]
mod placed_tests;
#[cfg(test)]
mod rings;
#[cfg(test)]
mod stations;
#[cfg(test)]
mod stations_tests;
#[cfg(test)]
mod sweep;
#[cfg(test)]
mod sweep_tests;
#[cfg(test)]
mod group;
#[cfg(test)]
mod handoff;
#[cfg(test)]
mod resident;
#[cfg(test)]
mod resident_tests;
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
mod timing_tests;
#[cfg(test)]
mod units;
