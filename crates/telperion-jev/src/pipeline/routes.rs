//! The two routes that ship a value code proposed and a render measured.
//!
//! The described route scores a trait over levels a person wrote, proposes the
//! candidate values the manifest's value table lists, renders and measures each
//! one, and ships the candidate whose measured trait lands inside the level's
//! range nearest its midpoint, ties to the first in table order. When none
//! lands inside, the trait is a decision and never a nearest miss.
//!
//! The reference route takes a dial the literature cannot fill: Jev names the
//! nearest tuned template and scores the relation to it, code computes the
//! candidate from that template's shipped value, and the candidate enters the
//! same render-and-measure step.
//!
//! Jev selects and never supplies a number, so nothing here reads a probability
//! and no output carries one. The multipliers come from the manifest, the
//! arithmetic is below, and the measurement is a render's.

use serde::{Deserialize, Serialize};

use super::render::{family_for, metric, Measurer, RenderError};

/// The value table the manifest names for one described trait.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValueTable {
    pub trait_name: String,
    /// The metric the render is read on, such as `crown_width_height_ratio`.
    pub measured_metric: String,
    /// The dotted wire path the candidate values are laid on.
    pub dial: String,
    pub levels: Vec<Level>,
}

/// One described level: what it says, what a render of it must measure, and the
/// dial values code proposes for it, in table order.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Level {
    pub key: String,
    pub summary: String,
    pub target_range: [f64; 2],
    pub candidates: Vec<f64>,
}

/// One described trait ready to run: the table, the level Jev scored highest,
/// the sentence behind it and the ledger reference that entry sits under.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribedInput {
    pub table: ValueTable,
    pub level_key: String,
    pub sentence: String,
    pub ledger: String,
    pub preset: String,
    pub seed: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DescribedOutcome {
    Shipped(DescribedEntry),
    LevelMiss(LevelMiss),
    Unavailable { reason: String },
}

/// The sidecar entry of a shipped described trait: level, sentence,
/// candidates, measurements, ranking and ledger reference, and no probability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DescribedEntry {
    /// Always `described`.
    pub route: String,
    pub trait_name: String,
    pub dial: String,
    pub level: String,
    pub sentence: String,
    pub candidates: Vec<Candidate>,
    pub shipped: f64,
    /// Candidate indices by `|measured - midpoint|`, in-range first, ties in
    /// table order.
    pub ranking: Vec<usize>,
    pub ledger: Vec<String>,
}

/// One proposed dial value and what a render of it measured.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Candidate {
    pub value: f64,
    pub measured: Option<f64>,
    pub in_range: bool,
    /// The measurement receipt's path, or `unmeasured: <reason>` when the
    /// render or the measurement of this candidate failed.
    pub receipt: String,
}

/// A described trait no candidate measured inside its level's range. The stage
/// files this as a decision of kind level-miss.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LevelMiss {
    pub trait_name: String,
    pub level: String,
    pub target_range: [f64; 2],
    pub candidates: Vec<Candidate>,
}

/// Runs the described route for one trait. Every candidate the level lists is
/// rendered and measured once, in table order. A `RenderError::Measure` on one
/// candidate is recorded as `measured: None`, never in range, and the run goes
/// on over the rest; any other error is the run's and is returned. A level the
/// table does not hold, or one that proposes no candidate, is recorded as
/// unavailable and never mapped by the model.
pub fn described(
    input: &DescribedInput,
    measurer: &dyn Measurer,
) -> Result<DescribedOutcome, RenderError> {
    let table = &input.table;
    let Some(level) = table.levels.iter().find(|l| l.key == input.level_key) else {
        return Ok(unavailable(format!(
            "level {} is not in the value table for {}",
            input.level_key, table.trait_name
        )));
    };
    if level.candidates.is_empty() {
        return Ok(unavailable(format!(
            "level {} of {} proposes no candidate value",
            level.key, table.trait_name
        )));
    }
    let range = level.target_range;
    if range[0] > range[1] {
        return Ok(unavailable(format!(
            "level {} of {} states a reversed target range",
            level.key, table.trait_name
        )));
    }
    let mut candidates = Vec::with_capacity(level.candidates.len());
    for &value in &level.candidates {
        let family = family_for(&table.dial, value);
        let (measured, receipt) = match measurer.measure(&input.preset, input.seed, &family) {
            Ok(done) => (
                metric(&done.metrics, &table.measured_metric),
                done.receipt_path.display().to_string(),
            ),
            Err(RenderError::Measure(reason)) => (None, format!("unmeasured: {reason}")),
            Err(other) => return Err(other),
        };
        candidates.push(Candidate {
            value,
            measured,
            in_range: measured.is_some_and(|m| m >= range[0] && m <= range[1]),
            receipt,
        });
    }
    let ranking = ranking(&candidates, range);
    let Some(chosen) = select_in_range(&candidates, range) else {
        return Ok(DescribedOutcome::LevelMiss(LevelMiss {
            trait_name: table.trait_name.clone(),
            level: level.key.clone(),
            target_range: range,
            candidates,
        }));
    };
    Ok(DescribedOutcome::Shipped(DescribedEntry {
        route: "described".into(),
        trait_name: table.trait_name.clone(),
        dial: table.dial.clone(),
        level: level.key.clone(),
        sentence: input.sentence.clone(),
        shipped: candidates[chosen].value,
        candidates,
        ranking,
        ledger: vec![input.ledger.clone()],
    }))
}

fn unavailable(reason: String) -> DescribedOutcome {
    DescribedOutcome::Unavailable { reason }
}

/// The fixed rule: among the candidates whose measurement landed inside the
/// closed range, the one nearest its midpoint, ties to the lowest index, which
/// is table order. `None` when none landed inside.
pub fn select_in_range(candidates: &[Candidate], range: [f64; 2]) -> Option<usize> {
    let midpoint = midpoint(range);
    candidates
        .iter()
        .enumerate()
        .filter(|(_, c)| c.in_range)
        .min_by(|(left, a), (right, b)| {
            distance(a, midpoint)
                .total_cmp(&distance(b, midpoint))
                .then(left.cmp(right))
        })
        .map(|(index, _)| index)
}

/// Every candidate index by the same rule, in-range first and then the rest by
/// their distance from the midpoint, ties in table order. An unmeasured
/// candidate sorts last.
pub fn ranking(candidates: &[Candidate], range: [f64; 2]) -> Vec<usize> {
    let midpoint = midpoint(range);
    let mut order: Vec<usize> = (0..candidates.len()).collect();
    order.sort_by(|&left, &right| {
        let (a, b) = (&candidates[left], &candidates[right]);
        b.in_range
            .cmp(&a.in_range)
            .then(distance(a, midpoint).total_cmp(&distance(b, midpoint)))
            .then(left.cmp(&right))
    });
    order
}

fn midpoint(range: [f64; 2]) -> f64 {
    (range[0] + range[1]) / 2.0
}

fn distance(candidate: &Candidate, midpoint: f64) -> f64 {
    candidate
        .measured
        .map_or(f64::INFINITY, |m| (m - midpoint).abs())
}

/// A template already onboarded, as the reference route reads it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reference {
    pub template: String,
    pub shipped_value: f64,
    pub growth_form: String,
}

/// One level of the relation Jev scored, as the manifest writes it: a botanical
/// situation and the factor code multiplies the reference's value by. The
/// factors are the manifest's, never this module's.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelationLevel {
    pub key: String,
    pub summary: String,
    pub multiplier: f64,
}

/// One dial ready for transfer: the tuned templates, the nearest one Jev chose
/// or `none`, the relation it scored, and what the manifest allows.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferInput {
    pub dial: String,
    pub references: Vec<Reference>,
    /// Jev's choice of nearest reference, or `none`.
    pub nearest: String,
    /// A second bracketing reference when Jev's distribution named one.
    pub second: Option<String>,
    pub relation: String,
    pub levels: Vec<RelationLevel>,
    /// The level's measured range, when the manifest states one.
    pub target_range: Option<[f64; 2]>,
    pub measured_metric: Option<String>,
    pub forbid_other_growth_form: bool,
    /// The new species' growth form.
    pub growth_form: String,
    pub ledger: Vec<String>,
    pub preset: String,
    pub seed: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransferOutcome {
    Shipped(TransferEntry),
    NoReference { dial: String, reason: String },
    Unavailable { reason: String },
}

/// The sidecar entry of a transferred dial: reference, level, candidate,
/// measurement and ledger reference, and no probability.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TransferEntry {
    /// Always `reference`.
    pub route: String,
    pub dial: String,
    pub reference: String,
    /// The second reference, when it bracketed the candidate and code
    /// interpolated between the two.
    pub second: Option<String>,
    pub relation: String,
    pub candidate: f64,
    pub measured: Option<f64>,
    pub receipt: String,
    pub ledger: Vec<String>,
}

/// Runs the reference route for one dial.
///
/// Code computes the candidate from the nearest reference's shipped value and
/// the relation's factor. When a second reference brackets that candidate, the
/// two bound the new species and the candidate becomes their midpoint. The
/// candidate is then rendered and measured once, under the described route's
/// rule: with a range stated, a measurement outside it is no ship.
pub fn transfer(
    input: &TransferInput,
    measurer: &dyn Measurer,
) -> Result<TransferOutcome, RenderError> {
    if input.nearest == "none" {
        return Ok(no_reference(input, "the nearest-reference answer is none"));
    }
    if input.references.is_empty() {
        return Ok(no_reference(input, "no tuned template is a reference yet"));
    }
    let allowed =
        |r: &Reference| !input.forbid_other_growth_form || r.growth_form == input.growth_form;
    if !input.references.iter().any(allowed) {
        return Ok(no_reference(
            input,
            "every reference is of another growth form",
        ));
    }
    let Some(reference) = input
        .references
        .iter()
        .find(|r| r.template == input.nearest)
    else {
        return Ok(TransferOutcome::Unavailable {
            reason: format!(
                "the nearest reference {} is not a tuned template",
                input.nearest
            ),
        });
    };
    if !allowed(reference) {
        return Ok(no_reference(
            input,
            "the nearest reference is of another growth form",
        ));
    }
    let Some(level) = input.levels.iter().find(|l| l.key == input.relation) else {
        return Ok(TransferOutcome::Unavailable {
            reason: format!(
                "relation {} is not a level the manifest states",
                input.relation
            ),
        });
    };
    let scaled = reference.shipped_value * level.multiplier;
    // A second reference on the other side of the scaled value brackets the new
    // species, and the candidate is the midpoint of the two bounds.
    let bracket = input
        .second
        .as_ref()
        .and_then(|name| input.references.iter().find(|r| &r.template == name))
        .filter(|r| allowed(r))
        .filter(|r| (scaled - reference.shipped_value) * (r.shipped_value - scaled) > 0.0);
    let candidate = match bracket {
        Some(second) => (scaled + second.shipped_value) / 2.0,
        None => scaled,
    };
    let done = measurer.measure(
        &input.preset,
        input.seed,
        &family_for(&input.dial, candidate),
    )?;
    let measured = input
        .measured_metric
        .as_deref()
        .and_then(|name| metric(&done.metrics, name));
    if let Some(reason) = missed_range(input, measured) {
        return Ok(TransferOutcome::Unavailable { reason });
    }
    Ok(TransferOutcome::Shipped(TransferEntry {
        route: "reference".into(),
        dial: input.dial.clone(),
        reference: reference.template.clone(),
        second: bracket.map(|r| r.template.clone()),
        relation: level.key.clone(),
        candidate,
        measured,
        receipt: done.receipt_path.display().to_string(),
        ledger: input.ledger.clone(),
    }))
}

/// Why a measured candidate does not answer the level's stated range, when the
/// manifest states one. `None` when it does, or when no range was stated.
fn missed_range(input: &TransferInput, measured: Option<f64>) -> Option<String> {
    let range = input.target_range?;
    let Some(name) = input.measured_metric.as_deref() else {
        return Some("the level states a range but the manifest names no metric".into());
    };
    match measured {
        None => Some(format!(
            "the candidate's measured metric {name} is unavailable"
        )),
        Some(value) if value < range[0] || value > range[1] => {
            Some("candidate measured outside the level's range".into())
        }
        Some(_) => None,
    }
}

fn no_reference(input: &TransferInput, reason: &str) -> TransferOutcome {
    TransferOutcome::NoReference {
        dial: input.dial.clone(),
        reason: reason.into(),
    }
}
