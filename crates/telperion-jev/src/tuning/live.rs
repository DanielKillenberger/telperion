//! Live services for the bounded engine. Construction verifies frozen judgment evidence.
use super::{
    actions::{Action, Dial, DIRECTION_VERSION, QUESTION_VERSION},
    calibration,
    continuation::{self, Basis},
    engine::{Answer, Proposal, Run, Services},
    evaluation::{self, Image, Trial},
    matched::Matched,
    progress::{self, Selection},
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

/// An owner's recorded verdict that a replay case's expected label was wrong.
/// It never rewrites the manifest or the result; it only records that the
/// rejection was correct.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OwnerRelabel {
    pub case_id: String,
    pub by: String,
    /// The owner's words, verbatim, which must appear in the evidence file.
    pub verdict: String,
    pub evidence: PathBuf,
    pub sha256: String,
}

impl OwnerRelabel {
    pub fn verify(&self) -> Result<(), String> {
        if self.case_id.trim().is_empty()
            || self.by.trim().is_empty()
            || self.verdict.trim().is_empty()
        {
            return Err("owner relabel lacks a case, an author or a verdict".into());
        }
        let bytes = fs::read(&self.evidence).map_err(|e| format!("owner relabel: {e}"))?;
        if sha256_hex(&bytes) != self.sha256 {
            return Err("owner relabel evidence changed".into());
        }
        if !String::from_utf8_lossy(&bytes).contains(&self.verdict) {
            return Err("owner relabel verdict is not in its evidence".into());
        }
        Ok(())
    }
}

/// The engine refuses more than this many candidates in one round.
pub const CANDIDATE_LIMIT: u64 = 4;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validation {
    pub manifest: PathBuf,
    pub result: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub preset: String,
    pub seed: u32,
    /// What decides between the current tree and a candidate. `score` is every
    /// run before 2026-09-21; `visual` puts the reviewer's comparative verdict
    /// in its place and leaves the numbers as telemetry.
    #[serde(default, skip_serializing_if = "Selection::is_score")]
    pub selection: Selection,
    /// The adapter the progress review is asked through. Required by `visual`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<progress::Adapter>,
    /// The adapter the contact sheet is asked through. Required by `bundle`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<progress::Adapter>,
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
    /// The frozen gap-magnitude calibration. Until it qualifies, a class above
    /// near that Jev chose needs scoped experimental authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gap_magnitude: Option<Validation>,
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
    pub adjustments: Validation,
    pub direction: Validation,
    pub continuation: Validation,
    pub visual_validation: Validation,
    pub vision_protocol: PathBuf,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_first: Option<super::reference_first::RuntimeConfig>,
    pub convergence_run: Option<PathBuf>,
    /// Upper bound on candidates evaluated in one round. `None` keeps the
    /// engine's own limit of four; a lower bound buys a cheaper round.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_candidates: Option<u64>,
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
    /// Owner verdicts that a falsely-rejected replay case was labelled wrong.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub owner_relabels: Vec<OwnerRelabel>,
    pub judgment_model: String,
    #[serde(default)]
    pub gap_specs: std::collections::BTreeMap<String, String>,
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
    fn verify_reference_protocol(&self) -> Result<(), String> {
        if self.reference_first.is_some() {
            let replay: super::reference_first::Replay = serde_json::from_slice(
                &fs::read(&self.visual_validation.manifest).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
            if replay.schema != "reference-first-replay-v1"
                || replay.protocol_sha256
                    != sha256_hex(&fs::read(&self.vision_protocol).map_err(|e| e.to_string())?)
            {
                return Err("reference-first adapter protocol changed or unqualified".into());
            }
        }
        Ok(())
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
            bytes.extend(fs::read(&self.vision_protocol).map_err(|e| e.to_string())?);
        }
        for review in [&self.progress, &self.sheet].into_iter().flatten() {
            bytes.extend(fs::read(&review.protocol).map_err(|e| e.to_string())?);
        }
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
    /// True when every falsely-rejected case carries an owner verdict naming
    /// it. Only reachable under bootstrap; it never touches the manifest.
    fn relabelled_rejections(&self, manifest: &[u8], result: &Value) -> Result<bool, String> {
        if !self.visual_bootstrap || self.owner_relabels.is_empty() {
            return Ok(false);
        }
        let replay: super::reference_first::Replay =
            serde_json::from_slice(manifest).map_err(|e| e.to_string())?;
        let observed: super::reference_first::ReplayResult =
            serde_json::from_value(result.clone()).map_err(|e| e.to_string())?;
        let mut rejected = vec![];
        for (case, got) in replay.cases.iter().zip(&observed.results) {
            let mut bound = got.clone();
            bound.bind(&case.request)?;
            let r = &case.request.comparison;
            let ready = super::state::ready(&r.required, &r.identity, &bound.visual.assessment);
            if case.expected_ready && !ready {
                rejected.push(case.id.clone());
            }
        }
        for relabel in &self.owner_relabels {
            relabel.verify()?;
            if !replay.cases.iter().any(|c| c.id == relabel.case_id) {
                return Err("owner relabel names a case the replay does not have".into());
            }
            if !rejected.contains(&relabel.case_id) {
                return Err("owner relabel names a case that was not falsely rejected".into());
            }
        }
        Ok(rejected
            .iter()
            .all(|id| self.owner_relabels.iter().any(|r| &r.case_id == id)))
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
        if self.selection == Selection::Bundle {
            let sheet = self
                .sheet
                .as_ref()
                .ok_or("bundle selection requires a contact-sheet adapter")?;
            fs::read(&sheet.protocol)
                .map_err(|e| format!("contact-sheet protocol unreadable: {e}"))?;
            self.strengths()?;
            super::bundle::verify_tracks(&self.tracks, &self.required)?;
            if !self.visual_bootstrap {
                return Err(
                    "bundle selection is uncalibrated; bootstrap authority required".into(),
                );
            }
        }
        if !self.selection.is_score() && self.selection != Selection::Bundle {
            let progress = self
                .progress
                .as_ref()
                .ok_or("visual selection requires a progress adapter")?;
            fs::read(&progress.protocol)
                .map_err(|e| format!("progress protocol unreadable: {e}"))?;
            if !self.visual_bootstrap {
                return Err(
                    "visual selection is uncalibrated; bootstrap authority required".into(),
                );
            }
        }
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
        if self
            .max_candidates
            .is_some_and(|n| !(1..=CANDIDATE_LIMIT).contains(&n))
        {
            return Err("max_candidates must be between 1 and 4".into());
        }
        let table = sha256_hex(&serde_json::to_vec(&self.dials).unwrap());
        for (v, kind, version) in [
            (&self.adjustments, "magnitude", QUESTION_VERSION),
            (&self.direction, "direction", DIRECTION_VERSION),
            (&self.continuation, "continuation", continuation::VERSION),
        ] {
            let manifest = fs::read(&v.manifest).map_err(|e| e.to_string())?;
            let declared: calibration::Manifest =
                serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
            if declared.model != self.judgment_model {
                return Err("calibrated judgment model mismatch".into());
            }
            let raw: Value =
                serde_json::from_slice(&fs::read(&v.result).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            let result = serde_json::from_value(raw.get("result").unwrap_or(&raw).clone())
                .map_err(|e| e.to_string())?;
            calibration::qualified(&manifest, &result, kind, version, &table)?;
        }
        let manifest = fs::read(&self.visual_validation.manifest).map_err(|e| e.to_string())?;
        let raw: Value = serde_json::from_slice(
            &fs::read(&self.visual_validation.result).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let result = raw.get("result").unwrap_or(&raw).clone();
        let raw_result = result.clone();
        let (model, effort, protocol, score) = if self.reference_first.is_some() {
            self.preparation()?;
            let result = serde_json::from_value(result).map_err(|e| e.to_string())?;
            let (replay, score) = super::reference_first::replay_score(&manifest, &result)?;
            (replay.model, replay.effort, replay.protocol_sha256, score)
        } else {
            let replay: vision::Replay =
                serde_json::from_slice(&manifest).map_err(|e| e.to_string())?;
            if replay
                .cases
                .iter()
                .any(|c| c.request.schema != "tuning-vision-v3" || c.request.joint.is_none())
            {
                return Err("joint visual protocol requires fresh calibration".into());
            }
            let result = serde_json::from_value(result).map_err(|e| e.to_string())?;
            let score = vision::replay_score(&manifest, &result)?;
            (replay.model, replay.effort, replay.protocol_sha256, score)
        };
        // Bootstrap admits a replay with no positive, because no owner-accepted
        // render exists yet for one. Every other guard stands.
        if model != self.vision.model
            || effort != self.vision.effort
            || score.negatives == 0
            || (score.positives == 0 && !self.visual_bootstrap)
            || score.false_ready > 0
        {
            return Err(
                "visual role lacks qualifying replay; human policy/convergence assessment required"
                    .into(),
            );
        }
        if protocol != sha256_hex(&fs::read(&self.vision_protocol).map_err(|e| e.to_string())?) {
            return Err("vision protocol differs from replay".into());
        }
        if score.false_rejections > 0 && self.relabelled_rejections(&manifest, &raw_result)? {
            // Every false rejection carries an owner verdict that the expected
            // label was wrong, so there is no reviewer error to converge on.
        } else if score.false_rejections > 0 {
            let path = self
                .convergence_run
                .as_ref()
                .ok_or("stricter role: bounded convergence remains unproven")?;
            let proof: Run = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
            let trial = proof
                .current
                .and_then(|i| proof.trials.get(i))
                .ok_or("missing convergence finalist")?;
            let visual = proof
                .visual
                .as_ref()
                .ok_or("missing convergence assessment")?;
            if let Some(prepared) = &self.reference_first {
                if proof.identity != self.identity()? {
                    return Err(
                        "reference-first convergence belongs to a different configuration".into(),
                    );
                }
                let expected = self.preparation()?;
                super::reference_first::verify_preparation(
                    expected.as_ref(),
                    proof.preparation_charge.as_ref(),
                )?;
                super::reference_first::verify_convergence(&self.vision, prepared, visual)?;
            }
            if !proof.machine_ready
                || proof.pause.is_some()
                || !trial.feasible
                || visual.model != self.vision.model
                || proof.required != self.required
                || !super::state::ready(&proof.required_cells(), &trial.key, visual)
                || !proof.usage_known
                || super::state::over(proof.budget.tokens, proof.budget.max_tokens)
                || super::state::over(proof.budget.images, proof.budget.max_images)
                || super::state::over(proof.budget.evaluations, proof.budget.max_evaluations)
            {
                return Err("bounded convergence remains unproven".into());
            }
            for c in &trial.comparisons {
                for image in &c.images {
                    image.verify()?;
                }
            }
        }
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
        let mut result = if let Some(prepared) = &self.config.reference_first {
            self.config.verify_reference_protocol()?;
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
        } else {
            self.config.vision.assess(&request)?
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
            return Err("runtime judgment model differs from calibrated role".into());
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
    fn selection(&self) -> Selection {
        self.config.selection
    }
    fn progress_request(
        &self,
        state: &Run,
        current: usize,
        candidate: usize,
        priorities: &[super::priority::Gap],
    ) -> Result<progress::Look, String> {
        progress::request(
            state,
            &self.config.preset,
            &self.config.references,
            state.trials.get(current).ok_or("no current trial")?,
            state.trials.get(candidate).ok_or("no candidate trial")?,
            priorities,
        )
    }
    fn progress_tokens(&self, request: &progress::Request) -> u64 {
        30_000 + serde_json::to_vec(request).unwrap().len() as u64
    }
    fn progress(
        &mut self,
        request: &progress::Request,
        side: &str,
    ) -> Result<Answer<progress::Verdict>, String> {
        let adapter = self
            .config
            .progress
            .as_ref()
            .ok_or("progress review has no adapter")?;
        progress::dispatch(adapter, request, side)
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
        let table = sha256_hex(&serde_json::to_vec(&self.config.dials).unwrap());
        let (threshold, calibrated) = super::stride::calibration(
            self.config.gap_magnitude.as_ref(),
            &self.config.judgment_model,
            &table,
        );
        let entry = self.ask(state, &super::stride::questions())?;
        let judged = super::stride::Judged {
            choice: entry.choice(super::stride::QUESTION).unwrap_or_default(),
            confidence: entry.confidence(super::stride::QUESTION),
            threshold,
            calibrated,
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
        let adapter = self
            .config
            .sheet
            .as_ref()
            .ok_or("contact-sheet review has no adapter")?;
        super::sheet::dispatch(adapter, plan)
    }
    fn proposal_tokens(&self, state: &Run) -> u64 {
        super::judgments::allowance(
            &super::judgments::proposal_state(state),
            &super::judgments::proposals(state).unwrap_or(Value::Null),
        )
    }
    fn continuation_tokens(&self, basis: &Basis) -> u64 {
        super::judgments::allowance(
            &serde_json::to_value(basis).unwrap(),
            &continuation::questions(),
        )
    }
    fn route_tokens(&self, state: &Run) -> u64 {
        super::judgments::allowance(
            &super::judgments::summary(state),
            &self.route_questions(state),
        )
    }
    fn max_candidates(&self) -> u64 {
        self.config.max_candidates.unwrap_or(CANDIDATE_LIMIT)
    }
    fn runaway_rounds(&self) -> u64 {
        self.config
            .runaway_rounds
            .map_or(super::runaway::ROUNDS, |n| n.get())
    }
    fn route_questions(&self, state: &Run) -> Value {
        super::judgments::routes(&self.config.gap_specs, state.approved_priorities())
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
    fn risk(&mut self, basis: &Basis) -> Result<Answer<String>, String> {
        let entry = self.ask(
            &serde_json::to_value(basis).unwrap(),
            &continuation::risk_only(),
        )?;
        let risk = supported(
            &entry,
            "risk",
            super::judgments::threshold(&self.config.continuation)?,
        );
        Ok(Self::answer(&entry, risk))
    }
    fn evidence(&mut self, state: &Value) -> Result<Answer<String>, String> {
        let entry = self.ask(state, &super::round::questions())?;
        let answer = supported(
            &entry,
            super::round::EVIDENCE_QUESTION,
            super::judgments::threshold(&self.config.continuation)?,
        );
        Ok(Self::answer(&entry, answer))
    }
    fn side_effect_tokens(&self, state: &Value) -> u64 {
        super::judgments::allowance(state, &super::veto::questions())
    }
    fn side_effects(&mut self, state: &Value) -> Result<Answer<super::veto::Judged>, String> {
        let entry = self.ask(state, &super::veto::questions())?;
        let threshold = super::judgments::threshold(&self.config.continuation)?;
        let judged = super::veto::Judged {
            choice: super::judgments::thresholded(&entry, super::veto::QUESTION, threshold),
            raw_choice: entry.choice(super::veto::QUESTION),
            confidence: entry.confidence(super::veto::QUESTION),
            threshold,
            ledger: Some(entry.reference()),
        };
        Ok(Self::answer(&entry, judged))
    }
    fn evidence_tokens(&self, state: &Value) -> u64 {
        super::judgments::allowance(state, &super::round::questions())
    }
    fn proposal_batches(&self, state: &Run) -> usize {
        let size = self.batch_size(state);
        state.dials.len().div_ceil(size.max(1)).max(1)
    }
    fn propose(&mut self, state: &Run, batch: usize) -> Result<Answer<Vec<Proposal>>, String> {
        let bytes = fs::read(&self.config.adjustments.manifest).map_err(|e| e.to_string())?;
        let manifest: calibration::Manifest =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
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
                    .and_then(|p| super::direction::accept(p, &available, manifest.min_confidence))
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
    fn route(&mut self, state: &Run) -> Result<Answer<Vec<super::handoff::PriorityRoute>>, String> {
        let questions = self.route_questions(state);
        let entry = self.ask(&self.route_state(state), &questions)?;
        let routes = super::judgments::priority_routes(
            &entry,
            state.approved_priorities(),
            super::judgments::threshold(&self.config.continuation)?,
        );
        Ok(Self::answer(&entry, routes))
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

fn supported(entry: &LedgerEntry, question: &str, threshold: f64) -> String {
    super::judgments::thresholded(entry, question, threshold)
}
