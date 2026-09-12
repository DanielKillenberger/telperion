//! State retained by a shoot across monthly growth and storage compaction.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BudFate {
    #[default]
    Terminal,
    Lateral,
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShootState {
    pub birth_year: f64,
    pub bud_fate: BudFate,
    pub vigour: f64,
    pub(crate) low_months: u64,
    pub(crate) width: Option<LocalWidth>,
}
impl Default for ShootState {
    fn default() -> Self {
        Self {
            birth_year: 0.0,
            bud_fate: BudFate::Terminal,
            vigour: 1.0,
            low_months: 0,
            width: None,
        }
    }
}
/// Birth allocation and taper, evaluated against the living parent each slice.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LocalWidth {
    pub ratio: f64,
    pub power: f64,
    pub distal: f64,
    pub proximal: f64,
}
