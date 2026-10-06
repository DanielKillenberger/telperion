//! The fine-root executor's GPU side (fn-209 F1): the decision contract
//! in WGSL, `telperion_space::fixed` word for word (FINE-CONTRACT.md, host
//! decision 1), which every fine-root shader begins with, so a decision
//! the GPU takes is the CPU reference executor's to the bit.
const WIDE: &str = include_str!("fine/wide.wgsl");
const CONTRACT: &str = include_str!("fine/contract.wgsl");

/// The contract's WGSL: its wide words, then its arithmetic.
pub(super) fn contract() -> String {
    format!("{WIDE}\n{CONTRACT}")
}

#[cfg(test)]
mod tests;
