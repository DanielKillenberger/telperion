//! Birth, death and append-only vigour observations for a retained shoot.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum BudFate {
    #[default]
    Terminal,
    Lateral,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ShootState {
    pub birth_year: f64,
    /// Absent while alive; a shed shoot keeps its original slot forever.
    pub death_year: Option<u64>,
    pub bud_fate: BudFate,
    pub(crate) vigour_events: Vec<VigourEvent>,
    pub(crate) width: Option<LocalWidth>,
}
impl Default for ShootState {
    fn default() -> Self {
        Self {
            birth_year: 0.0,
            death_year: None,
            bud_fate: BudFate::Terminal,
            vigour_events: Vec::new(),
            width: None,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct VigourEvent {
    pub year: u64,
    pub vigour: f64,
    pub low_slices: u64,
}
impl ShootState {
    pub fn vigour(&self) -> f64 {
        self.vigour_events.last().map_or(1.0, |event| event.vigour)
    }
    pub(crate) fn low_slices(&self) -> u64 {
        self.vigour_events
            .last()
            .map_or(0, |event| event.low_slices)
    }
    pub(crate) fn record_vigour(&mut self, year: u64, vigour: f64, low_slices: u64) {
        if vigour != self.vigour() || low_slices != self.low_slices() {
            self.vigour_events.push(VigourEvent {
                year,
                vigour,
                low_slices,
            });
        }
    }
}
/// Birth allocation and taper; queries derive widths from the living parent record.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct LocalWidth {
    pub birth: [f64; 3],
    pub ratio: f64,
    pub power: f64,
    pub distal: f64,
    pub proximal: f64,
}
