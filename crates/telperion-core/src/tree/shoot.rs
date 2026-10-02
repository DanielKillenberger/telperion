//! Whether a shoot grew from its parent's terminal bud or a lateral one.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub enum BudFate {
    #[default]
    Terminal,
    Lateral,
}
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct ShootState {
    pub bud_fate: BudFate,
}
