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

#[cfg(test)]
mod chain_tests;
#[cfg(test)]
mod chains;
#[cfg(test)]
mod roots;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod units;
