//! Where the run stood before it adopted, and what two assessments say went
//! backwards between them.
use crate::tuning::{
    engine::Run,
    state::{Cell, CellStatus, Visual},
    unexpressed::Unexpressed,
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

/// A disposition that went backwards: a pass the reviewer now calls a failure,
/// or an unknown it has since judged one. An unknown that was a failure is not
/// a regression, and neither is a cell that was never passing.
///
/// A pass that became unknown is not one either: unknown is the reviewer
/// declining to judge the trait, not a finding that the tree got worse, and a
/// required cell has never been vetoed for it. Readiness is unaffected, because
/// `ready()` still wants every cell passing and the core-coverage gate still
/// refuses a core trait that is not. Live, this transition on the variation
/// trait `variation-crown-density` rolled back the only two candidates a whole
/// run judged better, because changing a crown is what makes its density hard
/// to call.
fn backwards(before: CellStatus, after: CellStatus) -> bool {
    matches!(
        (before, after),
        (CellStatus::Pass, CellStatus::Fail) | (CellStatus::Unknown, CellStatus::Fail)
    )
}

/// What went backwards and rolls the adoption back, and what went backwards
/// on a trait the generator cannot draw yet and is only recorded.
#[derive(Debug, Default, PartialEq)]
pub struct Worsened {
    pub reasons: Vec<String>,
    pub notes: Vec<String>,
}

/// What the two assessments say went backwards, in words a router can read.
/// A required cell always counts; a coverage trait listed as unexpressed never
/// does, because no dial can draw it until its spec lands.
pub fn worsened(
    before: &Visual,
    after: &Visual,
    required: &[Cell],
    unexpressed: &[Unexpressed],
) -> Worsened {
    let mut out = Worsened::default();
    for cell in required {
        let (Some(was), Some(now)) = (status(before, cell), status(after, cell)) else {
            continue;
        };
        if was == CellStatus::Pass && now == CellStatus::Fail {
            out.reasons.push(format!(
                "required cell {} on the {} view at seed {} went from pass to fail",
                cell.item, cell.view, cell.seed
            ));
        }
    }
    for was in &before.coverage {
        let Some(now) = after.coverage.iter().find(|t| t.trait_id == was.trait_id) else {
            continue;
        };
        if !backwards(was.status, now.status) {
            continue;
        }
        let change = format!(
            "trait {} went from {:?} to {:?}",
            was.trait_id, was.status, now.status
        );
        match unexpressed.iter().find(|u| u.trait_id == was.trait_id) {
            Some(u) => out.notes.push(format!(
                "{change}; not a veto: the generator cannot draw it until {} lands",
                u.spec
            )),
            None => out.reasons.push(change),
        }
    }
    out
}
