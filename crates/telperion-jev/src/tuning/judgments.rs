use super::engine::Run;
use crate::ledger::LedgerEntry;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

/// The exact value transmitted for one judgment, checkpointed before dispatch
/// so an interrupted attempt still shows what the judgment was asked.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct JudgmentInput {
    pub label: String,
    pub state_sha256: String,
    pub state: Value,
    pub ledger: Option<String>,
}

impl Run {
    pub(super) fn push_judgment_input(&mut self, label: &str, state: Value) {
        let state_sha256 = crate::sha256_hex(&serde_json::to_vec(&state).unwrap());
        self.judgment_inputs.push(JudgmentInput {
            label: label.into(),
            state_sha256,
            state,
            ledger: None,
        });
    }
    pub(super) fn record_ledger(&mut self, ledger: Option<String>) {
        if let Some(input) = self.judgment_inputs.last_mut() {
            input.ledger = ledger;
        }
    }
}
/// What the reviewer said looks wrong in the tree the loop is standing on,
/// from the contact sheet that also showed it. Empty under any other mode.
fn looks_wrong(state: &Run) -> Vec<String> {
    state
        .current
        .and_then(|i| state.trials.get(i))
        .and_then(|t| t.sheet.as_ref())
        .map(|s| s.wrong.clone())
        .unwrap_or_default()
}

/// What the router is told about the table.
///
/// It chooses between tuning a dial and a capability gap, so it needs to know
/// which kinds of dial exist, not every row: with the full 200-row table the
/// old per-dial listing was 22,211 bytes of every routing question, and this
/// is 1,807. The proposal question is still shown every dial, with its own
/// meaning, its range and its current value.
fn dial_groups(state: &Run) -> Value {
    let mut groups: Vec<(String, Vec<&super::actions::Dial>)> = vec![];
    for dial in &state.dials {
        let name = dial.group.clone().unwrap_or_default();
        match groups.iter_mut().find(|(g, _)| g == &name) {
            Some((_, rows)) => rows.push(dial),
            None => groups.push((name, vec![dial])),
        }
    }
    json!({"meaning":"Counts and examples, not the table. Every dial is shown, with its meaning and its range, to the question that proposes a move.",
        "groups":groups.iter().map(|(name,rows)| json!({"group":name,"dials":rows.len(),
            "examples":rows.iter().take(3).map(|d| json!({"id":d.id,"meaning":d.meaning}))
                .collect::<Vec<_>>()})).collect::<Vec<_>>()})
}

pub fn summary(state: &Run) -> Value {
    let recent=state.trials.iter().rev().take(5).map(|t|json!({"label":t.label,"identity":t.identity,"current_revision":t.identity==state.identity,"feasible":t.feasible,
        "resource_feasibility":{"nodes":t.measurement["metrics"]["nodes"],"growth":t.measurement["metrics"]["growth"]},
        "score":t.score,"reason":t.reason,"review":super::look::words(t),"views":t.comparisons.iter().map(|c|json!({"reference":c.reference,
        "target":c.target,"observed":c.observed})).collect::<Vec<_>>()})).collect::<Vec<_>>();
    let nodes = state
        .current
        .and_then(|i| state.trials.get(i))
        .and_then(|t| t.measurement["metrics"]["nodes"]["value"].as_u64());
    let cap = state
        .effective
        .pointer("/skeleton/growth/maxNodes")
        .and_then(Value::as_u64);
    json!({"measured_facts":super::facts::measured(state),
        "current_tree_looks_wrong":looks_wrong(state),
        "owner_priorities":{"approval":state.approved_priorities(),"authority":"Explicit owner ranking outranks model severity. It selects objectives, not implementation or resolved status; all original findings remain below."},"current_identity":state.identity,"resource_limit":{"meaning":"computational feasibility, not botanical character","max_nodes":cap,"current_nodes":nodes,"remaining_nodes":cap.zip(nodes).map(|(c,n)|c.saturating_sub(n))},"owner_notes":state.owner_notes,"visual":state.visual,"recent_attempts":recent,
        "dials":dial_groups(state)})
}
/// What the proposal judgment is shown, and nothing else.
///
/// The live pilot sent a 25 KB run summary and five of six dials answered
/// insufficient_evidence. This carries the owner's tuning priorities, the
/// findings behind them, the dials, what has already been tried from this
/// candidate, and the numbers for the current trial. No budgets, no
/// authorizations, no amendments, no diagnoses, no reuse records, no joint
/// packet, no cells, no evidence ids.
/// What the proposal state may weigh: the digest sheds its phrases first
/// and then its per-dial lines to fit under it. The cap is a size
/// discipline, not Jev's limit: the fn-68 ledger holds proposal batches of
/// 34,000 input tokens that Jev answered, and the one refusal it holds was
/// a state several times this size. It was 24 KiB until fn-109's ten
/// rosette rows found 358 bytes of headroom; a score-visible dial costs
/// about 160 bytes here, so 32 KiB leaves room for about forty more before
/// the menu has to be scoped to the priorities a round is tuning.
pub const PROPOSAL_CAP: usize = 32 * 1024;

pub fn proposal_state(state: &Run) -> Value {
    let mut out = proposal_state_with(state, super::digest::Trim::None);
    for trim in [
        super::digest::Trim::Phrases,
        super::digest::Trim::PhrasesAndDials,
    ] {
        if serde_json::to_vec(&out).map_or(0, |b| b.len()) <= PROPOSAL_CAP {
            return out;
        }
        out["attempts_from_this_candidate"] = super::digest::attempts(state, trim);
    }
    out
}

fn proposal_state_with(state: &Run, trim: super::digest::Trim) -> Value {
    let tuning = super::look::objectives(state)
        .iter()
        .enumerate()
        .map(|(i, gap)| json!({"rank":i + 1,"gap_id":gap.id,"observation":gap.observation}))
        .collect::<Vec<_>>();
    let visual = state.visual.as_ref();
    let wanted = tuning
        .iter()
        .filter_map(|p| p["observation"].as_str().map(str::to_string))
        .collect::<Vec<_>>();
    let findings = visual
        .map(|v| {
            v.findings
                .iter()
                .filter(|f| {
                    wanted.is_empty()
                        || wanted.iter().any(|w| {
                            f.observation.contains(w) || w.contains(&f.observation)
                        })
                        || f.impact != super::joint::Impact::Supported
                })
                .map(|f| json!({"observation":f.observation,"impact":f.impact,"uncertain":f.uncertain}))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    let current = state.current.and_then(|i| state.trials.get(i));
    json!({
        "objectives": if tuning.is_empty() {
            json!(visual.map(|v| v.defects.clone()).unwrap_or_default())
        } else { json!(tuning) },
        "visual": {"defects": visual.map(|v| v.defects.clone()).unwrap_or_default(),
            "findings": findings},
        "dials": state.dials.iter().map(|d| json!({"id":d.id,"meaning":d.meaning,
            "current":state.effective.pointer(&d.path),"min":d.min,"max":d.max,
            "integer":d.integer,"small":d.small,"substantial":d.substantial}))
            .collect::<Vec<_>>(),
        "measured_facts": super::facts::measured(state),
        "current_tree_looks_wrong": looks_wrong(state),
        "attempts_from_this_candidate": super::digest::attempts(state, trim),
        "measured_views": current.map(|t| t.comparisons.iter().map(|c| json!({
            "reference":c.reference,"target":c.target,"observed":c.observed}))
            .collect::<Vec<_>>()).unwrap_or_default(),
        "owner_notes": state.owner_notes})
}

pub fn proposals(state: &Run) -> Result<Value, String> {
    proposal_batch(state, &state.dials)
}

/// One question per dial in this batch, in table order.
pub fn proposal_batch(state: &Run, dials: &[super::actions::Dial]) -> Result<Value, String> {
    let mut questions = serde_json::Map::new();
    for dial in dials {
        let current = state
            .effective
            .pointer(&dial.path)
            .and_then(Value::as_f64)
            .ok_or("missing dial")?;
        questions.insert(dial.id.clone(), dial.question(current)?);
    }
    Ok(Value::Object(questions))
}
/// The owner's first priority, "stretch the crown vertically", was routed
/// insufficient_evidence in every round of the run of 2026-09-21 while
/// `measured_facts` said the crown was 8% too wide for its height. The facts
/// were in the state; nothing told the router they counted.
pub const ROUTE_INSTRUCTIONS: &str = "Route remaining visible defects using authored dial meanings and prior outcomes. Numeric stall alone proves no generator gap. Choose an existing spec only when its stated scope matches the defect. Measured facts in the state are evidence; a priority about size or proportion that a measured fact supports and an authored dial can change routes to tuning.";

/// A choice at or above the frozen threshold, else explicit insufficiency.
pub fn thresholded(entry: &LedgerEntry, question: &str, threshold: f64) -> String {
    if entry
        .confidence(question)
        .is_some_and(|c| c.is_finite() && c >= threshold && c <= 1.)
    {
        entry
            .choice(question)
            .unwrap_or_else(|| "insufficient_evidence".into())
    } else {
        "insufficient_evidence".into()
    }
}
pub fn allowance(state: &Value, questions: &Value) -> u64 {
    (serde_json::to_vec(state).unwrap().len() + serde_json::to_vec(questions).unwrap().len()) as u64
        + 1024
}
