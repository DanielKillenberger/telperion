//! The history of attempts from one tree, folded into a fixed shape.
//!
//! The proposal state used to carry every bundle attempt in full, so five
//! rounds of 33 to 52 dials each grew it until Jev refused the batch with
//! `max_tokens_exceeded` (`bundle-search-run-2-2026-09-21.md`). What the
//! proposer needs is not every receipt: it is the last attempt, and then, per
//! dial family, how often it moved, which way, and how it went. Code builds
//! this; no model is asked, and nothing here selects anything.
use super::{bundle, engine::Run, evaluation::Trial, sheet};
use serde_json::{json, Value};

pub const MEANING: &str =
    "The last attempt in full, then every attempt from this tree counted per dial family. Counts, never a score.";
/// A reviewer phrase longer than this is cut; two per family is the most any
/// of them carries.
const PHRASE: usize = 160;
const PHRASES: usize = 2;

/// How much of the digest survives, when the state would not otherwise fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trim {
    /// Everything: the families, their phrases and the per-dial lines.
    None,
    /// The reviewer's phrases go first; the counts say more per byte.
    Phrases,
    /// Then the per-dial lines, which the family counts already imply.
    PhrasesAndDials,
}

/// What one attempt did, in one word.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Adopted,
    Slight,
    None,
    Worse,
    Breaks,
    Infeasible,
    RolledBack,
    /// The render came back byte-identical: the move changed nothing visible.
    Inert,
}

impl Outcome {
    fn name(self) -> &'static str {
        match self {
            Self::Adopted => "adopted",
            Self::Slight => "slight",
            Self::None => "none",
            Self::Worse => "worse",
            Self::Breaks => "breaks",
            Self::Infeasible => "infeasible",
            Self::RolledBack => "rolled_back",
            Self::Inert => "inert",
        }
    }
}

/// One attempt's outcome, read off the trial and the row of the sheet it got.
fn outcome(trial: &Trial, current: Option<&str>) -> Outcome {
    if !trial.feasible {
        return Outcome::Infeasible;
    }
    if trial.vetoed.is_some() {
        return Outcome::RolledBack;
    }
    if !trial.adopted_over.is_empty() || current == Some(trial.key.as_str()) {
        return Outcome::Adopted;
    }
    if trial.sheet.as_ref().is_some_and(|s| s.inert)
        || trial.progress.as_ref().is_some_and(|p| p.inert)
    {
        return Outcome::Inert;
    }
    if let Some(review) = &trial.progress {
        return if review.worse() > 0 {
            Outcome::Worse
        } else if review.breaks_something() {
            Outcome::Breaks
        } else if review.better() > 0 {
            Outcome::Slight
        } else {
            Outcome::None
        };
    }
    let Some(sheet) = &trial.sheet else {
        return Outcome::None;
    };
    // Worse outranks breaking: a render the reviewer put below the current
    // tree is not the better-but-breaks case a split is for.
    if sheet.below_current
        || sheet
            .per_priority
            .values()
            .any(|m| *m == sheet::Movement::Worse)
    {
        return Outcome::Worse;
    }
    if !sheet.breaks.is_empty() {
        return Outcome::Breaks;
    }
    if sheet
        .per_priority
        .values()
        .any(|m| matches!(m, sheet::Movement::Clear | sheet::Movement::Slight))
    {
        return Outcome::Slight;
    }
    Outcome::None
}

/// The gate a refused attempt named, so a family's infeasibility says which.
fn gate(trial: &Trial) -> Option<String> {
    trial
        .reason
        .as_ref()
        .map(|r| r.chars().take(PHRASE).collect())
}

fn phrases(trial: &Trial) -> Vec<String> {
    let Some(sheet) = &trial.sheet else {
        return vec![];
    };
    sheet
        .wrong
        .iter()
        .chain(sheet.breaks.iter())
        .chain(std::iter::once(&sheet.missing))
        .filter(|text| !text.trim().is_empty())
        .map(|text| text.chars().take(PHRASE).collect())
        .collect()
}

/// What one attempt moved, and which way: a bundle's own rows, or the single
/// dial a candidate is named after.
fn moved(trial: &Trial) -> Vec<(String, String)> {
    if let Some(bundle) = &trial.bundle {
        return bundle
            .moves
            .iter()
            .map(|m| (m.dial.clone(), m.direction.clone()))
            .collect();
    }
    let direction = match trial.action.and_then(bundle::direction) {
        Some(1) => "up",
        Some(-1) => "down",
        _ => "hold",
    };
    vec![(trial.label.clone(), direction.into())]
}

#[derive(Default)]
struct Family {
    moves: usize,
    directions: Vec<String>,
    outcomes: Vec<(&'static str, usize)>,
    gates: Vec<String>,
    phrases: Vec<String>,
}

impl Family {
    fn count(&mut self, outcome: Outcome) {
        let name = outcome.name();
        match self.outcomes.iter_mut().find(|(n, _)| *n == name) {
            Some((_, n)) => *n += 1,
            None => self.outcomes.push((name, 1)),
        }
    }
}

fn push_unique(list: &mut Vec<String>, value: &str, cap: usize) {
    if list.len() < cap && !list.iter().any(|known| known == value) {
        list.push(value.to_string());
    }
}

/// One dial that several bundles were judged worse with and none was adopted
/// with. A dial the reviewer keeps refusing is worth saying once, plainly.
fn dial_lines(rows: &[(String, String, usize, bool)]) -> Vec<String> {
    let mut out = rows
        .iter()
        .filter(|(_, _, against, adopted)| *against >= 2 && !adopted)
        .map(|(dial, direction, against, _)| {
            format!("{dial} {direction}: in {against} bundles judged worse")
        })
        .collect::<Vec<_>>();
    out.sort();
    out
}

/// Every attempt made from the tree the loop is standing on, folded down.
pub fn attempts(state: &Run, trim: Trim) -> Value {
    let history = state.attempts_here_trials();
    let current = state
        .current
        .and_then(|i| state.trials.get(i))
        .map(|t| t.key.as_str());
    let mut families: Vec<(String, Family)> = vec![];
    // dial -> (direction, bundles judged against it, ever in an adopted one)
    let mut dials: Vec<(String, String, usize, bool)> = vec![];
    for trial in &history {
        let outcome = outcome(trial, current);
        let said = phrases(trial);
        let against = matches!(outcome, Outcome::Worse | Outcome::RolledBack);
        let mut seen: Vec<String> = vec![];
        for (dial, direction) in moved(trial) {
            let name = state
                .dials
                .iter()
                .find(|d| d.id == dial)
                .map_or_else(String::new, |d| bundle::family(&d.path));
            if !families.iter().any(|(n, _)| n == &name) {
                families.push((name.clone(), Family::default()));
            }
            let family = &mut families.iter_mut().find(|(n, _)| n == &name).unwrap().1;
            family.moves += 1;
            push_unique(&mut family.directions, &direction, 4);
            if !seen.iter().any(|known| known == &name) {
                seen.push(name.clone());
                family.count(outcome);
                if outcome == Outcome::Infeasible {
                    if let Some(gate) = gate(trial) {
                        push_unique(&mut family.gates, &gate, PHRASES);
                    }
                }
                for text in &said {
                    push_unique(&mut family.phrases, text, PHRASES);
                }
            }
            match dials.iter_mut().find(|(d, _, _, _)| d == &dial) {
                Some(row) => {
                    row.2 += usize::from(against);
                    row.3 |= outcome == Outcome::Adopted;
                }
                None => dials.push((
                    dial,
                    direction,
                    usize::from(against),
                    outcome == Outcome::Adopted,
                )),
            }
        }
    }
    families.sort_by(|a, b| a.0.cmp(&b.0));
    let rows = families
        .iter()
        .map(|(name, f)| {
            let mut row = json!({"family":name,"moves":f.moves,"directions":f.directions,
                "outcomes":f.outcomes.iter().map(|(n,c)| ((*n).to_string(),json!(c)))
                    .collect::<serde_json::Map<_,_>>()});
            if !f.gates.is_empty() {
                row["refused_by_gate"] = json!(f.gates);
            }
            if trim == Trim::None && !f.phrases.is_empty() {
                row["reviewer_said"] = json!(f.phrases);
            }
            row
        })
        .collect::<Vec<_>>();
    let last = history.last().and_then(|t| super::progress::words(t));
    let mut out = json!({"meaning":MEANING,"attempts":history.len(),
        "last_attempt":last,"by_dial_family":rows});
    if trim != Trim::PhrasesAndDials {
        let lines = dial_lines(&dials);
        if !lines.is_empty() {
            out["dials_the_reviewer_kept_refusing"] = json!(lines);
        }
    }
    out
}
