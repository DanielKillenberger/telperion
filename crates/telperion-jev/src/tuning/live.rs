//! Live services for the engine: the renderer, the reviewer and Jev. The
//! thresholds a judgment is acted on at come from the labelled sets
//! (`data/thresholds.json`), never from a runtime calibration gate.
use super::{
    actions::{Action, Dial},
    engine::{Answer, Proposal, Run, Services},
    evaluation::{self, Image, Trial},
    look,
    matched::Matched,
    state::{Cell, Visual},
    vision,
};
use crate::{
    caller::{evaluate, EvaluateRequest, Transport},
    ledger::LedgerEntry,
    pipeline::render::SpeciesExample,
    sha256_hex,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

fn is_false(value: &bool) -> bool {
    !value
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub preset: String,
    pub seed: u32,
    /// The adapter the contact sheet, a round's one comparison, is asked
    /// through.
    pub sheet: look::Adapter,
    /// The strengths one bundle is drawn at, as multiples of each dial's own
    /// small step. Ascending, one to four of them, each above zero.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle_strengths: Option<Vec<f64>>,
    /// How many sheet reviews one round may spend isolating what breaks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_split_reviews: Option<u64>,
    /// The tracks a bundle round runs, in order. Empty is one track over
    /// every dial, which is every run before 2026-09-21.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tracks: Vec<super::bundle::Track>,
    /// Owner priorities that carry the size of their gap. A round on one of
    /// them asks nobody how far to move.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub magnitudes: std::collections::BTreeMap<String, super::stride::Class>,
    pub initial_overrides: Value,
    pub dials: Vec<Dial>,
    pub owner_notes: String,
    pub measure_binary: PathBuf,
    pub profiles: PathBuf,
    pub profile_id: String,
    pub matched: Matched,
    pub vision: vision::Adapter,
    pub references: Vec<Image>,
    pub required: Vec<Cell>,
    pub checklist: String,
    pub quality_anchors: Vec<vision::QualityAnchor>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_first: Option<super::reference_first::RuntimeConfig>,
    /// Rounds in a row that keep nothing before the run pauses as a runaway.
    /// `None` is the engine's own count of five.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub runaway_rounds: Option<std::num::NonZeroU64>,
    /// The reviewer has never been shown to pass an owner-accepted tree, so a
    /// replay with no positive is admitted and no run can claim readiness.
    /// Dials per proposal call. `None` asks them all in one call, as before.
    /// A large table answered in one question set is a large question set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_questions_per_call: Option<u64>,
    #[serde(default, skip_serializing_if = "is_false")]
    pub visual_bootstrap: bool,
    pub judgment_model: String,
    /// Inventory traits the generator cannot draw until an open spec lands.
    /// One going backwards is recorded, never a reason to roll back.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unexpressed: Vec<super::unexpressed::Unexpressed>,
    pub ledger: PathBuf,
    pub budget: super::state::Budget,
}

impl Config {
    pub fn priority_scope(&self, state: &Run) -> String {
        sha256_hex(&serde_json::to_vec(&json!({"base":state.priority_scope(&self.references),"checklist":self.checklist,"finish_anchors":self.quality_anchors.iter().map(|a|json!({"sha256":a.image.sha256,"scope":a.scope})).collect::<Vec<_>>()})).unwrap())
    }
    pub fn identity(&self) -> Result<String, String> {
        let mut bytes = serde_json::to_vec(self).unwrap();
        for path in [
            &self.measure_binary,
            &self.profiles,
            &self.matched.headless,
            &self.matched.compare_script,
            &self.matched.references,
        ] {
            bytes.extend(sha256_hex(&fs::read(path).map_err(|e| e.to_string())?).as_bytes());
        }
        if let Some(prepared) = &self.reference_first {
            bytes.extend(prepared.inventory.bytes()?);
            bytes.extend(prepared.preparation.bytes()?);
        }
        bytes.extend(fs::read(&self.sheet.protocol).map_err(|e| e.to_string())?);
        Ok(sha256_hex(&bytes))
    }
    pub fn preparation(&self) -> Result<Option<super::reference_first::PreparationCharge>, String> {
        self.reference_first
            .as_ref()
            .map(|prepared| {
                let (inventory, charge) = prepared.load(&self.vision.model, &self.vision.effort)?;
                if inventory.request.target_species != self.preset
                    || inventory.request.references.len() != self.references.len()
                    || !inventory
                        .request
                        .references
                        .iter()
                        .zip(&self.references)
                        .all(|(a, b)| a.image.sha256 == b.sha256 && a.image.view == b.view)
                {
                    return Err("prepared inventory differs from runtime species/references".into());
                }
                for image in &self.references {
                    image.verify()?;
                }
                Ok(charge)
            })
            .transpose()
    }
    /// The strengths a bundle round draws, defaulted and checked.
    pub fn strengths(&self) -> Result<Vec<f64>, String> {
        let strengths = self
            .bundle_strengths
            .clone()
            .unwrap_or_else(|| vec![0.5, 1.0, 2.0, 4.0]);
        if strengths.is_empty()
            || strengths.len() > 4
            || strengths.iter().any(|s| !s.is_finite() || *s <= 0.0)
            || strengths.windows(2).any(|w| w[0] >= w[1])
        {
            return Err("bundle strengths must be one to four ascending values above zero".into());
        }
        Ok(strengths)
    }
    pub fn split_reviews(&self) -> u64 {
        self.max_split_reviews.unwrap_or(6)
    }
    pub fn verify(&self) -> Result<(), String> {
        super::unexpressed::verify(&self.unexpressed, self.reference_first.as_ref())?;
        fs::read(&self.sheet.protocol)
            .map_err(|e| format!("contact-sheet protocol unreadable: {e}"))?;
        self.strengths()?;
        super::bundle::verify_tracks(&self.tracks, &self.required)?;
        if self.owner_notes.is_empty()
            || self.dials.is_empty()
            || self.required.is_empty()
            || !self.required.iter().any(|c| c.seed == self.seed)
            || !self.required.iter().any(|c| c.seed != self.seed)
        {
            return Err("missing owner notes, dials or fixed/fresh seed checklist".into());
        }
        for dial in &self.dials {
            dial.validate()?;
        }
        self.preparation()?;
        Ok(())
    }
    pub fn measurer(&self) -> SpeciesExample {
        SpeciesExample {
            measure_binary: self.measure_binary.clone(),
            headless_binary: Some(self.matched.headless.clone()),
            profiles: self.profiles.clone(),
            profile_id: self.profile_id.clone(),
            work_dir: self.matched.scratch.join("measure"),
        }
    }
}

pub struct Live<'a> {
    pub config: &'a Config,
    pub transport: &'a dyn Transport,
    pub key: &'a str,
}
impl Live<'_> {
    /// Dials per proposal call.
    fn batch_size(&self, state: &Run) -> usize {
        self.config
            .max_questions_per_call
            .unwrap_or(state.dials.len().max(1) as u64) as usize
    }
    fn visual_cells(&self, trial: &Trial) -> Vec<Cell> {
        if trial.round != 0 {
            return self.config.required.clone();
        }
        let first = self.config.required.iter().find(|c| c.seed == trial.seed);
        self.config
            .required
            .iter()
            .filter(|c| first.is_some_and(|f| c.seed == f.seed && c.view == f.view))
            .cloned()
            .collect()
    }
    fn assess_visual(
        &mut self,
        trial: &Trial,
        required: Vec<Cell>,
    ) -> Result<Answer<Visual>, String> {
        let mut images = trial
            .comparisons
            .iter()
            .filter_map(|c| c.images.first().cloned())
            .filter(|i| {
                required
                    .iter()
                    .any(|c| c.view == i.view && c.seed == i.seed)
            })
            .collect::<Vec<_>>();
        for cell in &required {
            if images
                .iter()
                .any(|i| i.view == cell.view && i.seed == cell.seed)
            {
                continue;
            }
            let mut matched = self.config.matched.clone();
            matched.numeric_references = vec![cell.view.clone()];
            let captures = matched.capture(
                &self.config.preset,
                cell.seed,
                &trial.overrides,
                &trial.key,
                false,
            )?;
            images.extend(
                captures
                    .into_iter()
                    .filter_map(|c| c.images.into_iter().next()),
            );
        }
        let mut request = vision::Request {
            schema: "tuning-vision-v3".into(),
            target_species: self.config.preset.clone(),
            identity: trial.key.clone(),
            required: required.clone(),
            images,
            references: self
                .config
                .references
                .iter()
                .filter(|i| {
                    self.config.reference_first.is_some()
                        || required.iter().any(|c| c.view == i.view)
                })
                .cloned()
                .collect(),
            checklist: self.config.checklist.clone(),
            quality_anchors: self.config.quality_anchors.clone(),
            joint: None,
        };
        request.joint = Some(
            super::joint::Packet::from_request(&request)
                .with_shots(&self.config.matched.references)?,
        );
        let prepared = self
            .config
            .reference_first
            .as_ref()
            .ok_or("the reviewer needs a reference-first inventory")?;
        let mut result = {
            self.config.preparation()?;
            let (inventory, _) =
                prepared.load(&self.config.vision.model, &self.config.vision.effort)?;
            request.checklist = format!(
                "{}\nAuthoritative owner requirements: {}",
                request.checklist, self.config.owner_notes
            );
            let mut comparison =
                super::reference_first::ComparisonRequest::production(&request, inventory);
            comparison.known_gaps = self.config.unexpressed.clone();
            super::reference_first::assess(&self.config.vision, &comparison)?.visual
        };
        for cell in &self.config.required {
            if !required.contains(cell) {
                result
                    .assessment
                    .cells
                    .push((cell.clone(), super::state::CellStatus::Unknown));
            }
        }
        Ok(Answer {
            ledger: Some(result.assessment.ledger.clone()),
            value: result.assessment,
            tokens: result
                .usage
                .and_then(|u| u.input_tokens.checked_add(u.output_tokens)),
        })
    }
    fn ask(&self, state: &Value, questions: &Value) -> Result<LedgerEntry, String> {
        self.config.preparation()?;
        let entry = evaluate(
            self.transport,
            self.key,
            EvaluateRequest {
                tool: "tuning",
                source: None,
                state,
                questions,
                ledger_dir: &self.config.ledger,
            },
        )
        .map_err(|e| match &e {
            crate::caller::CallerError::Http { status, body } => {
                refused(*status, body, request_bytes(state, questions))
            }
            other => other.to_string(),
        })?;
        if entry.model != self.config.judgment_model {
            return Err("runtime judgment model differs from the config's".into());
        }
        Ok(entry)
    }
    fn answer<T>(entry: &LedgerEntry, value: T) -> Answer<T> {
        Answer {
            ledger: Some(entry.reference()),
            value,
            tokens: entry
                .usage
                .as_ref()
                .and_then(|u| u.input_tokens.checked_add(u.output_tokens)),
        }
    }
}
impl Services for Live<'_> {
    fn priority_scope(&self, state: &Run) -> String {
        self.config.priority_scope(state)
    }
    fn visual_tokens_for(
        &self,
        trial: &Trial,
        priorities: Option<&super::priority::Approval>,
    ) -> u64 {
        if let Some(approval) = priorities {
            let required = super::priority::requirements(&self.config.required, Some(approval));
            40000
                + serde_json::to_vec(&approval.ordered).unwrap().len() as u64
                + serde_json::to_vec(&required).unwrap().len() as u64
                + 256
        } else {
            self.visual_tokens(trial)
        }
    }
    fn visual_images_for(
        &self,
        trial: &Trial,
        required: &[Cell],
        priorities: Option<&super::priority::Approval>,
    ) -> u64 {
        if priorities.is_none() {
            return self.visual_images(trial);
        }
        let existing = trial
            .comparisons
            .iter()
            .map(|c| (c.reference.as_str(), trial.seed))
            .collect::<std::collections::HashSet<_>>();
        required
            .iter()
            .map(|c| (c.view.as_str(), c.seed))
            .collect::<std::collections::HashSet<_>>()
            .iter()
            .filter(|pair| !existing.contains(pair))
            .count() as u64
            * 2
    }
    fn priority_references(&self) -> Vec<Image> {
        self.config.references.clone()
    }
    fn priority_evidence(
        &self,
        trial: &Trial,
        visual: &Visual,
    ) -> Result<Vec<super::priority::Evidence>, String> {
        let mut renders = trial
            .comparisons
            .iter()
            .filter_map(|c| c.images.first().cloned())
            .collect::<Vec<_>>();
        if let Ok(bytes) = fs::read(&visual.ledger) {
            if let Ok(record) = serde_json::from_slice::<Value>(&bytes) {
                let request = record
                    .pointer("/request/comparison")
                    .or_else(|| record.get("request"));
                if let Some(images) = request.and_then(|r| r.get("images")) {
                    let images: Vec<Image> =
                        serde_json::from_value(images.clone()).map_err(|e| e.to_string())?;
                    renders.extend(images);
                }
            }
        }
        super::priority::evidence(
            visual,
            &renders,
            &self.config.references,
            &self
                .config
                .quality_anchors
                .iter()
                .map(|a| a.image.clone())
                .collect::<Vec<_>>(),
        )
    }
    fn visual_for(
        &mut self,
        trial: &Trial,
        required: &[Cell],
        priorities: Option<&super::priority::Approval>,
    ) -> Result<Answer<Visual>, String> {
        let mut config = self.config.clone();
        config.required = required.to_vec();
        if let Some(approval) = priorities {
            config.checklist=format!("Owner-approved priorities, in order (authoritative over model severity; not evidence of resolution): {}\n{}",serde_json::to_string(&approval.ordered).unwrap(),config.checklist);
        }
        let mut live = Live {
            config: &config,
            transport: self.transport,
            key: self.key,
        };
        if priorities.is_some() {
            live.assess_visual(trial, required.to_vec())
        } else {
            live.visual(trial)
        }
    }
    fn preparation(&self) -> Result<Option<super::reference_first::PreparationCharge>, String> {
        self.config.preparation()
    }
    fn bundle_strengths(&self) -> Vec<f64> {
        self.config.strengths().unwrap_or_default()
    }
    fn max_split_reviews(&self) -> u64 {
        self.config.split_reviews()
    }
    fn tracks(&self) -> Vec<super::bundle::Track> {
        self.config.tracks.clone()
    }
    fn unexpressed(&self) -> Vec<super::unexpressed::Unexpressed> {
        self.config.unexpressed.clone()
    }
    fn inventory(&self) -> Result<Option<super::reference_first::Inventory>, String> {
        let Some(prepared) = &self.config.reference_first else {
            return Ok(None);
        };
        serde_json::from_slice(&prepared.inventory.bytes()?)
            .map(Some)
            .map_err(|e| e.to_string())
    }
    fn owner_magnitude(&self, priority: &str) -> Option<super::stride::Class> {
        self.config.magnitudes.get(priority).copied()
    }
    fn offers_gap_magnitude(&self) -> bool {
        true
    }
    fn gap_magnitude_tokens(&self, state: &Value) -> u64 {
        super::judgments::allowance(state, &super::stride::questions())
    }
    fn gap_magnitude(&mut self, state: &Value) -> Result<Answer<super::stride::Judged>, String> {
        let entry = self.ask(state, &super::stride::questions())?;
        let judged = super::stride::Judged {
            choice: entry.choice(super::stride::QUESTION).unwrap_or_default(),
            confidence: entry.confidence(super::stride::QUESTION),
            threshold: super::stride::labelled().min_confidence,
        };
        Ok(Self::answer(&entry, judged))
    }
    fn capture_views(
        &mut self,
        trial: &Trial,
        views: &[String],
    ) -> Result<Vec<evaluation::Comparison>, String> {
        let mut out = vec![];
        for view in views {
            let mut matched = self.config.matched.clone();
            matched.numeric_references = vec![view.clone()];
            out.extend(matched.capture(
                &self.config.preset,
                trial.seed,
                &trial.overrides,
                &trial.key,
                false,
            )?);
        }
        Ok(out)
    }
    fn sheet_request(
        &self,
        state: &Run,
        current: usize,
        variants: &[usize],
        priorities: &[super::priority::Gap],
        view: Option<&str>,
    ) -> Result<super::sheet::Look, String> {
        super::sheet::look(
            state,
            &self.config.preset,
            &self.config.references,
            current,
            variants,
            priorities,
            view,
        )
    }
    fn sheet_tokens(&self, request: &super::sheet::Request) -> u64 {
        30_000 + serde_json::to_vec(request).unwrap().len() as u64
    }
    fn sheet(
        &mut self,
        plan: &super::sheet::Plan,
    ) -> Result<Answer<super::sheet::Verdict>, String> {
        super::sheet::dispatch(&self.config.sheet, plan)
    }
    fn proposal_tokens(&self, state: &Run) -> u64 {
        super::judgments::allowance(
            &super::judgments::proposal_state(state),
            &super::judgments::proposals(state).unwrap_or(Value::Null),
        )
    }
    fn runaway_rounds(&self) -> u64 {
        self.config
            .runaway_rounds
            .map_or(super::runaway::ROUNDS, |n| n.get())
    }
    fn evaluation_images(&self) -> u64 {
        (self.config.matched.numeric_references.len() * 2) as u64
    }
    fn visual_images(&self, trial: &Trial) -> u64 {
        let existing = trial
            .comparisons
            .iter()
            .map(|c| (c.reference.as_str(), trial.seed))
            .collect::<std::collections::HashSet<_>>();
        self.visual_cells(trial)
            .iter()
            .map(|c| (c.view.as_str(), c.seed))
            .collect::<std::collections::HashSet<_>>()
            .iter()
            .filter(|c| !existing.contains(c))
            .count() as u64
            * 2
    }
    fn visual_tokens(&self, trial: &Trial) -> u64 {
        if trial.round == 0 && self.config.reference_first.is_none() {
            25000
        } else {
            40000
        }
    }
    fn evaluate(
        &mut self,
        overrides: Value,
        round: u64,
        label: &str,
        ledger: Option<String>,
    ) -> Trial {
        evaluation::evaluate(
            &self.config.measurer(),
            &self.config.matched,
            &self.config.preset,
            &self.config.identity().unwrap_or_default(),
            self.config.seed,
            round,
            label,
            overrides,
            ledger,
        )
    }
    fn visual(&mut self, trial: &Trial) -> Result<Answer<Visual>, String> {
        self.assess_visual(trial, self.visual_cells(trial))
    }
    fn side_effect_tokens(&self, state: &Value) -> u64 {
        super::judgments::allowance(state, &super::veto::questions())
    }
    fn side_effects(&mut self, state: &Value) -> Result<Answer<super::veto::Judged>, String> {
        let entry = self.ask(state, &super::veto::questions())?;
        let threshold = crate::thresholds().tuning_min_confidence;
        let judged = super::veto::Judged {
            choice: super::judgments::thresholded(&entry, super::veto::QUESTION, threshold),
            raw_choice: entry.choice(super::veto::QUESTION),
            confidence: entry.confidence(super::veto::QUESTION),
            threshold,
            ledger: Some(entry.reference()),
        };
        Ok(Self::answer(&entry, judged))
    }
    fn proposal_batches(&self, state: &Run) -> usize {
        let size = self.batch_size(state);
        state.dials.len().div_ceil(size.max(1)).max(1)
    }
    fn propose(&mut self, state: &Run, batch: usize) -> Result<Answer<Vec<Proposal>>, String> {
        let floor = crate::thresholds().tuning_min_confidence;
        let focused = self.proposal_state(state);
        let size = self.batch_size(state);
        let mut accepted = vec![];
        let mut tokens = Some(0u64);
        let mut ledger = None;
        for batch in state.dials.chunks(size.max(1)).skip(batch).take(1) {
            let questions = super::judgments::proposal_batch(state, batch)?;
            let entry = self.ask(&focused, &questions)?;
            ledger = Some(entry.reference());
            tokens = tokens
                .zip(Self::answer(&entry, ()).tokens)
                .map(|(a, b)| a + b);
            for dial in batch {
                let rank = state
                    .dials
                    .iter()
                    .position(|d| d.id == dial.id)
                    .unwrap_or(usize::MAX);
                let current = state
                    .effective
                    .pointer(&dial.path)
                    .and_then(Value::as_f64)
                    .ok_or("missing dial")?;
                let available = [
                    Action::SmallDecrease,
                    Action::SubstantialDecrease,
                    Action::SmallIncrease,
                    Action::SubstantialIncrease,
                ]
                .into_iter()
                .filter(|a| dial.value(current, *a).is_ok())
                .collect::<Vec<_>>();
                let Some(choice) = entry
                    .probabilities(&dial.id)
                    .and_then(|p| super::direction::accept(p, &available, floor))
                else {
                    continue;
                };
                accepted.push((rank, choice, dial.id.clone(), entry.reference()));
            }
        }
        // Strongest direction first, ties in the dial table's own order.
        accepted.sort_by(|a, b| {
            b.1.direction_mass
                .total_cmp(&a.1.direction_mass)
                .then(a.0.cmp(&b.0))
        });
        // The engine truncates, after refusing repeats, so a move already
        // tried cannot consume the round's only slot.
        let proposals = accepted
            .into_iter()
            .map(|(_, choice, dial, ledger)| Proposal {
                dial,
                action: choice.action,
                ledger,
                direction_mass: Some(choice.direction_mass),
                rule: Some(super::direction::RULE.into()),
            })
            .collect::<Vec<_>>();
        Ok(Answer {
            value: proposals,
            tokens,
            ledger,
        })
    }
}

/// The bytes the caller puts on the wire for one judgment, so a size refusal
/// says how big the request that earned it was.
fn request_bytes(state: &Value, questions: &Value) -> usize {
    serde_json::to_vec(&json!({"model":crate::caller::MODEL,"state":state,"questions":questions}))
        .map_or(0, |bytes| bytes.len())
}

/// The kind of refusal the service named, wherever it put it.
fn error_type(body: &str) -> String {
    let Ok(parsed) = serde_json::from_str::<Value>(body) else {
        return "unparsed".into();
    };
    for path in ["/error/type", "/error/code", "/error_type", "/type"] {
        if let Some(kind) = parsed.pointer(path).and_then(Value::as_str) {
            return kind.to_string();
        }
    }
    "unknown".into()
}

/// A refused judgment, said plainly. The live run of 2026-09-21 stopped on
/// `HTTP 400 max_tokens_exceeded` and the pause carried only the raw text, so
/// nobody could tell a refused request from a broken one.
pub const REFUSED: &str = "judgment refused by the service";

fn refused(status: u16, body: &str, bytes: usize) -> String {
    format!(
        "{REFUSED}: {status} {}; request bytes {bytes}",
        error_type(body)
    )
}
