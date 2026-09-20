//! Live services for the bounded engine. Construction verifies frozen judgment evidence.
use super::{
    actions::{Action, Dial, DIRECTION_VERSION, QUESTION_VERSION},
    calibration,
    continuation::{self, Assessment, Basis},
    engine::{Answer, Proposal, Run, Services},
    evaluation::{self, Image, Trial},
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
    pub judgment_model: String,
    #[serde(default)]
    pub gap_specs: std::collections::BTreeMap<String, String>,
    pub ledger: PathBuf,
    pub budget: super::state::Budget,
}

impl Config {
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
    pub fn verify(&self) -> Result<(), String> {
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
        if model != self.vision.model
            || effort != self.vision.effort
            || score.negatives == 0
            || score.positives == 0
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
        if score.false_rejections > 0 {
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
                || !super::state::ready(&self.required, &trial.key, visual)
                || !proof.usage_known
                || proof.budget.tokens > proof.budget.max_tokens
                || proof.budget.images > proof.budget.max_images
                || proof.budget.evaluations > proof.budget.max_evaluations
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
        .map_err(|e| e.to_string())?;
        if entry.model != self.config.judgment_model {
            return Err("runtime judgment model differs from calibrated role".into());
        }
        Ok(entry)
    }
    fn answer<T>(entry: &LedgerEntry, value: T) -> Answer<T> {
        Answer {
            value,
            tokens: entry
                .usage
                .as_ref()
                .and_then(|u| u.input_tokens.checked_add(u.output_tokens)),
        }
    }
}
impl Services for Live<'_> {
    fn preparation(&self) -> Result<Option<super::reference_first::PreparationCharge>, String> {
        self.config.preparation()
    }
    fn proposal_tokens(&self, state: &Run) -> u64 {
        super::judgments::allowance(
            &super::judgments::summary(state),
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
            &super::judgments::routes(&self.config.gap_specs),
        )
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
        let required = self.visual_cells(trial);
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
            let comparison =
                super::reference_first::ComparisonRequest::production(&request, inventory);
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
            value: result.assessment,
            tokens: result
                .usage
                .and_then(|u| u.input_tokens.checked_add(u.output_tokens)),
        })
    }
    fn continuation(&mut self, basis: &Basis) -> Result<Answer<Assessment>, String> {
        let entry = self.ask(
            &serde_json::to_value(basis).unwrap(),
            &continuation::questions(),
        )?;
        let value = Assessment {
            identity: basis.identity.clone(),
            ledger: entry.reference(),
            tractability: supported(
                &entry,
                "tractability",
                super::judgments::threshold(&self.config.continuation)?,
            ),
            progress: supported(
                &entry,
                "progress",
                super::judgments::threshold(&self.config.continuation)?,
            ),
            risk: supported(
                &entry,
                "risk",
                super::judgments::threshold(&self.config.continuation)?,
            ),
        };
        Ok(Self::answer(&entry, value))
    }
    fn propose(&mut self, state: &Run) -> Result<Answer<Vec<Proposal>>, String> {
        let questions = super::judgments::proposals(state)?;
        let observations = super::judgments::summary(state);
        let entry = self.ask(&observations, &questions)?;
        let bytes = fs::read(&self.config.adjustments.manifest).map_err(|e| e.to_string())?;
        let manifest: calibration::Manifest =
            serde_json::from_slice(&bytes).map_err(|e| e.to_string())?;
        let mut proposals = vec![];
        for dial in &state.dials {
            let confidence = entry
                .confidence(&dial.id)
                .ok_or("missing proposal confidence")?;
            if !confidence.is_finite() || confidence < manifest.min_confidence {
                continue;
            }
            let action: Action =
                serde_json::from_value(json!(entry.choice(&dial.id).ok_or("missing proposal")?))
                    .map_err(|_| "unsupported adjustment")?;
            if matches!(action, Action::Hold | Action::InsufficientEvidence) {
                continue;
            }
            proposals.push(Proposal {
                dial: dial.id.clone(),
                action,
                ledger: entry.reference(),
            });
            if proposals.len() == 4 {
                break;
            }
        }
        Ok(Self::answer(&entry, proposals))
    }
    fn route(&mut self, state: &Run) -> Result<Answer<String>, String> {
        let questions = super::judgments::routes(&self.config.gap_specs);
        let entry = self.ask(&super::judgments::summary(state), &questions)?;
        let route = supported(
            &entry,
            "route",
            super::judgments::threshold(&self.config.continuation)?,
        );
        Ok(Self::answer(&entry, route))
    }
}

fn supported(entry: &LedgerEntry, question: &str, threshold: f64) -> String {
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
