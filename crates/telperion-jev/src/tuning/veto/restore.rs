//! Where the run stood before it adopted, and what two assessments say went
//! backwards between them.
use crate::tuning::{
    engine::Run,
    state::{Cell, CellStatus, Visual},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Why an adoption was taken back, and the receipt of the question that said
/// so where one was asked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Veto {
    pub reasons: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ledger: Option<String>,
}

/// The tree the loop stood on, the wire that drew it and the look it held.
pub struct Restore {
    pub current: Option<usize>,
    pub effective: Value,
    pub overrides: Value,
    pub visual: Option<Visual>,
    pub machine_ready: bool,
    pub reviewer_passed_unqualified: bool,
}

pub fn restore_point(state: &Run) -> Restore {
    Restore {
        current: state.current,
        effective: state.effective.clone(),
        overrides: state.overrides.clone(),
        visual: state.visual.clone(),
        machine_ready: state.machine_ready,
        reviewer_passed_unqualified: state.reviewer_passed_unqualified,
    }
}

impl Restore {
    /// Puts the run back exactly where it stood. The trials stay: the attempt
    /// was made, it counts as tried, and what it cost stays spent.
    pub fn apply(self, state: &mut Run) {
        state.current = self.current;
        state.effective = self.effective;
        state.overrides = self.overrides;
        state.visual = self.visual;
        state.machine_ready = self.machine_ready;
        state.reviewer_passed_unqualified = self.reviewer_passed_unqualified;
    }
}

fn status(visual: &Visual, cell: &Cell) -> Option<CellStatus> {
    visual
        .cells
        .iter()
        .find(|(c, _)| c == cell)
        .map(|(_, status)| *status)
}

/// A disposition that went backwards: a pass that is no longer one, or an
/// unknown that has become a failure. An unknown that was a failure is not a
/// regression, and neither is a cell that was never passing.
fn backwards(before: CellStatus, after: CellStatus) -> bool {
    matches!(
        (before, after),
        (CellStatus::Pass, CellStatus::Fail)
            | (CellStatus::Pass, CellStatus::Unknown)
            | (CellStatus::Unknown, CellStatus::Fail)
    )
}

/// What the two assessments say went backwards, in words a router can read.
pub fn worsened(before: &Visual, after: &Visual, required: &[Cell]) -> Vec<String> {
    let mut reasons = vec![];
    for cell in required {
        let (Some(was), Some(now)) = (status(before, cell), status(after, cell)) else {
            continue;
        };
        if was == CellStatus::Pass && now == CellStatus::Fail {
            reasons.push(format!(
                "required cell {} on the {} view at seed {} went from pass to fail",
                cell.item, cell.view, cell.seed
            ));
        }
    }
    for was in &before.coverage {
        let Some(now) = after.coverage.iter().find(|t| t.trait_id == was.trait_id) else {
            continue;
        };
        if backwards(was.status, now.status) {
            reasons.push(format!(
                "trait {} went from {:?} to {:?}",
                was.trait_id, was.status, now.status
            ));
        }
    }
    reasons
}
