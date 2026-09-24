//! The vision receipt's tidiness rules (fn-80, 2026-09-24).
//!
//! A tidiness violation (a list over its cap, an item with no, repeated or
//! too many evidence ids, empty or over-long text, a repeated or unusable
//! coverage row) does not make an answer untrustworthy. The reviewer is asked
//! once to repair it (`repair.rs`); whatever is still left is trimmed or
//! dropped here, first items kept, and every trim or drop is recorded in the
//! assessment's observations. Trust violations are not here: a stale answer,
//! an evidence id the request never supplied, a missing or mis-ordered
//! required cell still refuse the whole answer.
use super::joint::{Finding, Packet};
use super::reference_first::{ComparisonRequest, Coverage};
use std::collections::HashSet;

pub const MAX_FINDINGS: usize = 16;
pub const MAX_COVERAGE: usize = 16;
pub const MAX_EVIDENCE: usize = 12;
pub const MAX_FINDING_TEXT: usize = 2048;
pub const MAX_COVERAGE_TEXT: usize = 4096;

/// One broken tidiness rule: `violation` names it exactly for the reviewer's
/// repair, `note` records what code did about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Untidy {
    pub violation: String,
    pub note: String,
}

/// Refuses a finding that cites an input the request never supplied.
pub fn invented_findings(packet: &Packet, findings: &[Finding]) -> Result<(), String> {
    let known = |id: &String| packet.inputs.iter().any(|i| &i.id == id);
    if findings.iter().flat_map(|f| &f.evidence_ids).all(known) {
        return Ok(());
    }
    Err("invalid joint finding evidence".into())
}

/// Refuses a coverage row that cites an input the request never supplied.
pub fn invented_coverage(packet: &Packet, rows: &[Coverage]) -> Result<(), String> {
    let known = |id: &String| packet.inputs.iter().any(|i| &i.id == id);
    if rows.iter().flat_map(|c| &c.evidence_ids).all(known) {
        return Ok(());
    }
    Err("invalid trait coverage".into())
}

fn evidence_rule(ids: &[String]) -> Option<String> {
    if ids.is_empty() {
        return Some("no evidence cited".into());
    }
    if ids.len() > MAX_EVIDENCE {
        return Some(format!(
            "cites {} evidence ids, at most {MAX_EVIDENCE} allowed",
            ids.len()
        ));
    }
    let mut seen = HashSet::new();
    ids.iter()
        .find(|id| !seen.insert(*id))
        .map(|id| format!("cites evidence id {id} more than once"))
}

fn text_rule(label: &str, text: &str, max: usize) -> Option<String> {
    if text.trim().is_empty() {
        Some(format!("{label} is empty"))
    } else if text.len() > max {
        Some(format!(
            "{label} is {} characters, at most {max} allowed",
            text.len()
        ))
    } else {
        None
    }
}

/// The first 240 characters, so a note never carries an over-long text whole.
fn clip(text: &str) -> String {
    match text.char_indices().nth(240) {
        Some((at, _)) => format!("{}\u{2026}", &text[..at]),
        None => text.into(),
    }
}

fn finding_rule(f: &Finding) -> Option<String> {
    evidence_rule(&f.evidence_ids)
        .or_else(|| text_rule("observation", &f.observation, MAX_FINDING_TEXT))
        .or_else(|| {
            let h = f.causal_hypothesis.as_deref()?;
            (h.len() > MAX_FINDING_TEXT).then(|| {
                format!(
                    "causal hypothesis is {} characters, at most {MAX_FINDING_TEXT} allowed",
                    h.len()
                )
            })
        })
}

/// Drops each finding breaking a rule, then keeps the first `cap`; `why`
/// finishes the cap's rule when the cap is not the plain one.
pub fn findings(list: &mut Vec<Finding>, cap: usize, why: &str) -> Vec<Untidy> {
    let n = list.len();
    let mut out = vec![];
    let mut kept = vec![];
    for (i, f) in std::mem::take(list).into_iter().enumerate() {
        let at = format!("finding {} of {n}", i + 1);
        match finding_rule(&f) {
            None => kept.push((at, f)),
            Some(rule) => out.push(Untidy {
                note: if f.evidence_ids.is_empty() {
                    format!("dropped finding with no evidence: {}", clip(&f.observation))
                } else {
                    format!("dropped {at} ({rule}): {}", clip(&f.observation))
                },
                violation: format!("{at}: {rule}"),
            }),
        }
    }
    let over = kept.split_off(cap.min(kept.len()));
    for (at, f) in over {
        let rule = format!("at most {cap} findings allowed{why}");
        out.push(Untidy {
            violation: format!("{at}: {rule}"),
            note: format!("dropped {at} ({rule}): {}", clip(&f.observation)),
        });
    }
    *list = kept.into_iter().map(|(_, f)| f).collect();
    out
}

/// Drops each coverage row breaking a rule, in the order the rules are
/// listed, then keeps the first 16. The first usable row per trait stands.
pub fn coverage(request: &ComparisonRequest, rows: &mut Vec<Coverage>) -> Vec<Untidy> {
    let inputs = request
        .comparison
        .joint
        .as_ref()
        .map(|p| p.inputs.as_slice())
        .unwrap_or_default();
    let has_role = |c: &Coverage, role: &str| {
        c.evidence_ids
            .iter()
            .any(|id| inputs.iter().any(|i| &i.id == id && i.role == role))
    };
    let n = rows.len();
    let mut out = vec![];
    let mut kept: Vec<(usize, Coverage)> = vec![];
    for (i, c) in std::mem::take(rows).into_iter().enumerate() {
        let id = &c.trait_id;
        let first = kept
            .iter()
            .find(|(_, k)| &k.trait_id == id)
            .map(|(j, _)| j + 1);
        let broken = if !request.inventory.traits.iter().any(|t| &t.id == id) {
            Some((
                "for unknown trait",
                format!("{id} is not an inventory trait"),
            ))
        } else if let Some(rule) = text_rule("explanation", &c.explanation, MAX_COVERAGE_TEXT)
            .or_else(|| evidence_rule(&c.evidence_ids))
        {
            Some(("with unusable text or evidence", rule))
        } else if !has_role(&c, "render") || !has_role(&c, "reference") {
            let rule = "cites no render or no reference".to_string();
            Some(("without render and reference evidence", rule))
        } else {
            first.map(|j| {
                (
                    "repeating its trait",
                    format!("repeats {id}, given by row {j}"),
                )
            })
        };
        match broken {
            None => kept.push((i, c)),
            Some((why, rule)) => out.push(row(i, n, &c, why, rule)),
        }
    }
    let over = kept.split_off(MAX_COVERAGE.min(kept.len()));
    for (i, c) in over {
        let rule = format!("at most {MAX_COVERAGE} coverage rows allowed");
        out.push(row(i, n, &c, "over the cap", rule));
    }
    *rows = kept.into_iter().map(|(_, c)| c).collect();
    out
}

fn row(i: usize, n: usize, c: &Coverage, why: &str, rule: String) -> Untidy {
    Untidy {
        violation: format!("coverage row {} of {n} ({}): {rule}", i + 1, c.trait_id),
        note: format!(
            "dropped coverage row {why} {}: {:?} \u{2014} {}",
            c.trait_id,
            c.status,
            clip(&c.explanation)
        ),
    }
}

/// Records each note once on the result and on its assessment, so a rebind
/// of the bound result keeps them.
pub fn record<'a>(visual: &mut super::vision::Result, notes: impl IntoIterator<Item = &'a String>) {
    for note in notes {
        if !visual.observations.contains(note) {
            visual.observations.push(note.clone());
        }
        if !visual.assessment.observations.contains(note) {
            visual.assessment.observations.push(note.clone());
        }
    }
}
