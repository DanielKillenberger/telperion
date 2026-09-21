//! One bounded species run. Capability repair is returned to fn-89, never dispatched here.
use super::{
    actions::{candidate, Action, Dial},
    continuation::{self, Basis, Pause},
    evaluation::Trial,
    state::{ready, Budget, Cell, Visual},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub dial: String,
    pub action: Action,
    pub ledger: String,
    /// Why this move was tried: the probability mass behind its direction and
    /// the rule that accepted it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_mass: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

pub struct Answer<T> {
    pub value: T,
    pub tokens: Option<u64>,
    /// The receipt of the call that produced this, independent of the value.
    /// A call that returns nothing still spent, so it still has one.
    pub ledger: Option<String>,
}

pub trait Services {
    fn priority_scope(&self, state: &Run) -> String {
        state.priority_scope(&self.priority_references())
    }
    fn visual_tokens_for(
        &self,
        trial: &Trial,
        _priorities: Option<&super::priority::Approval>,
    ) -> u64 {
        self.visual_tokens(trial)
    }
    fn visual_images_for(
        &self,
        trial: &Trial,
        _required: &[Cell],
        _priorities: Option<&super::priority::Approval>,
    ) -> u64 {
        self.visual_images(trial)
    }
    fn priority_references(&self) -> Vec<super::evaluation::Image> {
        vec![]
    }
    fn priority_evidence(
        &self,
        trial: &Trial,
        visual: &Visual,
    ) -> Result<Vec<super::priority::Evidence>, String> {
        let renders = trial
            .comparisons
            .iter()
            .filter_map(|c| c.images.first().cloned())
            .collect::<Vec<_>>();
        super::priority::evidence(visual, &renders, &self.priority_references(), &[])
    }
    fn visual_for(
        &mut self,
        trial: &Trial,
        _required: &[Cell],
        _priorities: Option<&super::priority::Approval>,
    ) -> Result<Answer<Visual>, String> {
        self.visual(trial)
    }
    fn preparation(&self) -> Result<Option<super::reference_first::PreparationCharge>, String> {
        Ok(None)
    }
    fn proposal_tokens(&self, _state: &Run) -> u64 {
        4000
    }
    fn continuation_tokens(&self, _basis: &Basis) -> u64 {
        2000
    }
    fn evidence_tokens(&self, _state: &Value) -> u64 {
        2000
    }
    fn route_tokens(&self, _state: &Run) -> u64 {
        2000
    }
    /// The exact state value this service will transmit for each judgment.
    fn route_state(&self, state: &Run) -> Value {
        super::judgments::summary(state)
    }
    fn proposal_state(&self, state: &Run) -> Value {
        super::judgments::proposal_state(state)
    }
    /// The question set the router was shown, used to quote the chosen criterion.
    fn route_questions(&self, state: &Run) -> Value {
        super::judgments::routes(&Default::default(), state.approved_priorities())
    }
    fn evaluation_images(&self) -> u64;
    fn visual_images(&self, trial: &Trial) -> u64;
    fn visual_tokens(&self, _trial: &Trial) -> u64 {
        25000
    }
    fn evaluate(
        &mut self,
        overrides: Value,
        round: u64,
        label: &str,
        ledger: Option<String>,
    ) -> Trial;
    fn visual(&mut self, trial: &Trial) -> Result<Answer<Visual>, String>;
    /// The pre-dispatch judgment, now the risk question alone.
    fn risk(&mut self, basis: &Basis) -> Result<Answer<String>, String>;
    /// The uncalibrated evidence-difference question, asked only after a stall.
    fn evidence(&mut self, state: &Value) -> Result<Answer<String>, String>;
    fn propose(&mut self, state: &Run) -> Result<Answer<Vec<Proposal>>, String>;
    fn route(&mut self, state: &Run) -> Result<Answer<Vec<super::handoff::PriorityRoute>>, String>;
}

pub use super::judgments::JudgmentInput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Run {
    pub identity: String,
    pub preset: String,
    pub seed: u32,
    pub effective: Value,
    pub overrides: Value,
    pub dials: Vec<Dial>,
    pub owner_notes: String,
    pub required: Vec<Cell>,
    pub budget: Budget,
    pub usage_known: bool,
    pub trials: Vec<Trial>,
    pub current: Option<usize>,
    pub visual: Option<Visual>,
    pub pause: Option<Pause>,
    pub machine_ready: bool,
    pub pending: Option<String>,
    pub routes: Vec<String>,
    #[serde(default)]
    pub authorizations: Vec<continuation::HumanDecision>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preparation_charge: Option<super::reference_first::PreparationCharge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_checkpoints: Vec<super::priority::Checkpoint>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub handoffs: Vec<super::handoff::Handoff>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub judgment_inputs: Vec<JudgmentInput>,
    /// The reviewer has not been shown to pass an owner-accepted tree, so this
    /// run can never be machine ready.
    #[serde(default)]
    pub visual_bootstrap: bool,
    /// The reviewer passed every required cell, which under bootstrap is a
    /// prompt for an owner look, not readiness.
    #[serde(default)]
    pub reviewer_passed_unqualified: bool,
}

fn merge(target: &mut Value, patch: &Value) {
    if let (Some(target), Some(patch)) = (target.as_object_mut(), patch.as_object()) {
        for (key, value) in patch {
            match target.get_mut(key) {
                Some(old) if old.is_object() && value.is_object() => merge(old, value),
                _ => {
                    target.insert(key.clone(), value.clone());
                }
            }
        }
    }
}

impl Run {
    pub fn priority_scope(&self, references: &[super::evaluation::Image]) -> String {
        super::priority::scope(&self.preset, &self.owner_notes, &self.required, references)
    }
    pub fn approved_priorities(&self) -> Option<&super::priority::Approval> {
        let checkpoint = self.priority_checkpoints.last()?;
        self.authorizations
            .iter()
            .rev()
            .filter_map(|d| d.priority_approval.as_ref())
            .find(|a| a.checkpoint_sha256 == checkpoint.hash())
    }
    pub fn required_cells(&self) -> Vec<Cell> {
        super::priority::requirements(&self.required, self.approved_priorities())
    }
    pub fn accept_priorities(
        &mut self,
        decision: &continuation::HumanDecision,
        scope: &str,
    ) -> Result<(), String> {
        if let Some(approval) = &decision.priority_approval {
            if decision.by.trim().is_empty() || decision.rationale.trim().is_empty() {
                return Err("missing owner priority attribution".into());
            }
            let checkpoint = self
                .priority_checkpoints
                .last()
                .ok_or("no proposed priority checkpoint")?;
            approval.verify(checkpoint, scope)?;
            if approval.ordered.iter().any(|g| {
                g.views
                    .iter()
                    .any(|v| !self.required.iter().any(|c| &c.view == v))
            }) {
                return Err("priority view outside configured scope".into());
            }
            self.machine_ready = false;
        }
        Ok(())
    }
    pub(super) fn priority_gate(
        &mut self,
        services: &dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<bool, String> {
        let references = services.priority_references();
        for image in &references {
            image.verify()?;
        }
        let scope = services.priority_scope(self);
        if self
            .priority_checkpoints
            .last()
            .is_none_or(|p| p.scope_sha256 != scope)
        {
            let trial = self
                .current
                .and_then(|i| self.trials.get(i))
                .ok_or("no current priority evidence")?;
            let visual = self.visual.clone().ok_or("missing initial visual review")?;
            let evidence = services.priority_evidence(trial, &visual)?;
            self.priority_checkpoints
                .push(super::priority::Checkpoint::new(
                    &self.identity,
                    &scope,
                    visual,
                    evidence,
                )?);
        }
        if let Some(approval) = self.approved_priorities() {
            approval.verify(self.priority_checkpoints.last().unwrap(), &scope)?;
            return Ok(true);
        }
        self.stop(
            "Owner gap-priority review required; model readiness is not owner approval".into(),
            "approve gap priorities",
        );
        self.pause.as_mut().unwrap().decision_requested="Review priority-review.json, confirm/reorder/add gaps in priority_approval, and submit a scoped --resume JSON. This chooses objectives, not mechanics or final acceptance.".into();
        save(self)?;
        Ok(false)
    }
    pub fn pilot_authority(&self) -> Result<(), String> {
        let authority = self
            .authorizations
            .last()
            .and_then(|d| d.experimental_pilot.as_ref())
            .ok_or("magnitude live efficacy unvalidated")?;
        if authority.visual_bootstrap != self.visual_bootstrap {
            return Err("bootstrap run requires authority naming visual_bootstrap".into());
        }
        authority.verify(&self.identity, &self.budget)
    }

    /// Under bootstrap a reviewer pass stops the loop for an owner look
    /// instead of finishing it.
    fn bootstrap_finalist(
        &mut self,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<bool, String> {
        if !self.visual_bootstrap || !self.reviewer_passed_unqualified {
            return Ok(false);
        }
        self.stop(
            "reviewer passed all required cells; reviewer unqualified for positives (bootstrap); owner look required".into(),
            "owner look at bootstrap finalist",
        );
        save(self)?;
        Ok(true)
    }
    pub(super) fn stop(&mut self, reason: String, action: &str) {
        self.machine_ready = false;
        self.pause = Some(Pause {
            id: crate::ledger::new_entry_id(),
            identity: self.identity.clone(),
            reason,
            basis: self.basis(action),
            decision_requested: format!(
                "Provide a scoped decision for {action}; changed evidence requires reassessment."
            ),
        });
    }
    pub(super) fn basis(&self, action: &str) -> Basis {
        let projection = super::judgments::summary(self);
        let mut evidence = self
            .visual
            .as_ref()
            .map(|v| v.defects.clone())
            .unwrap_or_default();
        evidence.push(
            serde_json::json!({
                "current_identity":projection["current_identity"],
                "owner_priorities":projection["owner_priorities"],
                "visual_evidence":projection["visual"],
                "resource_limit":projection["resource_limit"],
            "resource_amendments":projection["resource_amendments"],
            "agent_diagnoses":projection["agent_diagnoses"],
            "verified_evidence_reuse":projection["verified_evidence_reuse"]
            })
            .to_string(),
        );
        Basis {
            identity: self.identity.clone(),
            proposed_action: action.into(),
            evidence,
            recent_outcomes: projection["recent_attempts"]
                .as_array()
                .unwrap()
                .iter()
                .map(|t| t.to_string())
                .collect(),
            next_tokens: None,
            estimate_basis: String::new(),
            usage_known: self.usage_known,
        }
    }
    pub fn round_basis(&self, services: &dyn Services) -> Result<Basis, String> {
        let mut basis = self.basis("targeted tuning round");
        basis.proposed_action = format!("One bounded round, max four single-dial candidates. Existing authored dials only: {}. Code enforces bounds/integer type, measures numeric gates/node cap BEFORE render, chooses only a lower feasible five-metric score, then separately verifies all required visual cells at fixed/fresh seeds. No generator/renderer source changes or shipped preset edits; candidate overlays only. No shipping or sweep. Stop or hand off if unsupported; at most {} remaining rounds.",
            serde_json::to_string(&self.dials.iter().map(|d| serde_json::json!({"id":d.id,"meaning":d.meaning,"current":self.effective.pointer(&d.path),"min":d.min,"max":d.max,"integer":d.integer,"small":d.small,"substantial":d.substantial})).collect::<Vec<_>>()).unwrap(),
            self.budget.max_rounds.saturating_sub(self.budget.rounds));
        let mut finalist = self.trials[self.current.ok_or("no current trial")?].clone();
        finalist.round = self.budget.rounds + 1;
        let next = services
            .proposal_tokens(self)
            .checked_add(services.visual_tokens_for(&finalist, self.approved_priorities()))
            .ok_or("reservation overflow")?;
        basis.next_tokens = Some(next);
        basis.estimate_basis = format!("proposal serialized-request bound {} + all-cell visual reservation {}; actual usage may exceed estimate and then pauses", services.proposal_tokens(self), services.visual_tokens_for(&finalist,self.approved_priorities()));
        Ok(basis)
    }
    pub(super) fn reserve(
        &mut self,
        evaluations: u64,
        images: u64,
        tokens: u64,
        rounds: u64,
        label: &str,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.verify_diagnoses()?;
        self.budget.reserve(evaluations, images, tokens, rounds)?;
        self.pending = Some(label.into());
        save(self)?;
        self.verify_diagnoses()
    }
    pub fn verify_diagnoses(&self) -> Result<(), String> {
        let mut count = 0;
        for d in self
            .authorizations
            .iter()
            .filter_map(|a| a.diagnosis.as_ref())
            .filter(|d| d.target_identity == self.identity)
        {
            count += d.findings.len();
            if count > 8 {
                return Err("active diagnosis finding bound exceeded".into());
            }
            d.verify(&self.identity)?;
        }
        Ok(())
    }
    pub(super) fn settle<T>(&mut self, answer: Answer<T>, reserved: u64) -> Result<T, String> {
        let Some(actual) = answer.tokens else {
            self.usage_known = false;
            return Err("unknown judgment usage".into());
        };
        self.budget.tokens = self
            .budget
            .tokens
            .checked_sub(reserved)
            .and_then(|v| v.checked_add(actual))
            .ok_or("usage overflow")?;
        self.pending = None;
        self.record_ledger(answer.ledger);
        if actual > reserved || self.budget.tokens > self.budget.max_tokens {
            return Err("judgment exceeded reservation".into());
        }
        Ok(answer.value)
    }
    fn assess(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.verify_diagnoses()?;
        let trial = self.trials[self.current.ok_or("no feasible current candidate")?].clone();
        let scope = services.priority_scope(self);
        let approval = self
            .approved_priorities()
            .filter(|a| a.scope_sha256 == scope)
            .cloned();
        if let Some(approval) = &approval {
            approval.verify(self.priority_checkpoints.last().unwrap(), &scope)?;
        }
        let required = super::priority::requirements(&self.required, approval.as_ref());
        let allowance = services.visual_tokens_for(&trial, approval.as_ref());
        let mut budget = self.budget.clone();
        budget.reserve_visual()?;
        budget.reserve(
            0,
            services.visual_images_for(&trial, &required, approval.as_ref()),
            allowance,
            0,
        )?;
        self.budget = budget;
        self.pending = Some("visual assessment".into());
        save(self)?;
        self.verify_diagnoses()?;
        let answer = services.visual_for(&trial, &required, approval.as_ref())?;
        let visual = self.settle(answer, allowance)?;
        let passed = approval.is_some() && ready(&required, &trial.key, &visual);
        // Bootstrap never awards readiness: a pass here is an owner prompt.
        self.reviewer_passed_unqualified = passed && self.visual_bootstrap;
        self.machine_ready = passed && !self.visual_bootstrap;
        self.visual = Some(visual);
        save(self)
    }
    /// Caller verifies calibration linkage before this method. `save` must durably checkpoint.
    pub fn execute(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        if self.pause.is_some() {
            return Err("run paused; scoped resume decision required".into());
        }
        if self.pending.is_some() {
            self.stop(
                "interrupted attempt; reservation retained".into(),
                "reconcile interrupted attempt",
            );
            return save(self);
        }
        let result = self.execute_inner(services, save);
        if let Err(reason) = result {
            self.stop(reason, "reassess or diagnose");
            save(self)?;
        }
        Ok(())
    }
    fn execute_inner(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        self.verify_diagnoses()?;
        let expected = services.preparation()?;
        super::reference_first::verify_preparation(
            expected.as_ref(),
            self.preparation_charge.as_ref(),
        )?;
        if self.current.is_none() {
            self.reserve(1, services.evaluation_images(), 0, 0, "baseline", save)?;
            let trial = services.evaluate(self.overrides.clone(), 0, "baseline", None);
            self.pending = None;
            let feasible = trial.feasible;
            self.trials.push(trial);
            save(self)?;
            if !feasible {
                return Err("baseline infeasible".into());
            }
            self.current = Some(self.trials.len() - 1);
            self.assess(services, save)?;
        }
        if self.visual.is_none() {
            self.assess(services, save)?;
        }
        if !self.priority_gate(services, save)? {
            return Ok(());
        }
        let required = self.required_cells();
        if self.visual.as_ref().is_none_or(|v| {
            required
                .iter()
                .any(|c| !v.cells.iter().any(|(got, _)| got == c))
        }) {
            self.assess(services, save)?;
        } else {
            let passed = self
                .current
                .zip(self.visual.as_ref())
                .is_some_and(|(i, v)| ready(&required, &self.trials[i].key, v));
            self.reviewer_passed_unqualified = passed && self.visual_bootstrap;
            self.machine_ready = passed && !self.visual_bootstrap;
        }
        if self.bootstrap_finalist(save)? {
            return Ok(());
        }
        while !self.machine_ready {
            if !self.priority_gate(services, save)? {
                return Ok(());
            }
            if self
                .budget
                .visual_passes
                .zip(self.budget.max_visual_passes)
                .is_none_or(|(used, cap)| used >= cap)
            {
                return Err("visual pass limit exhausted before routing".into());
            }
            if self.budget.rounds >= self.budget.max_rounds {
                return Err("hard round limit exhausted before routing".into());
            }
            let basis = self.round_basis(services)?;
            let per_priority = self.approved_priorities().map_or(0, |a| a.ordered.len()) as u64;
            let planned = basis
                .next_tokens
                .unwrap()
                .checked_add(services.continuation_tokens(&basis))
                .and_then(|n| n.checked_add(services.route_tokens(self)))
                .and_then(|n| {
                    n.checked_add(
                        services
                            .continuation_tokens(&basis)
                            .checked_mul(per_priority)?,
                    )
                })
                .ok_or("reservation overflow")?;
            if self
                .budget
                .tokens
                .checked_add(planned)
                .is_none_or(|n| n > self.budget.max_tokens)
            {
                return Err(format!(
                    "round preflight cannot fit: need {planned} tokens before any dispatch"
                ));
            }
            self.route_remaining(services, save)?;
            // Code owns the round boundary; only a repeat after a stall asks.
            self.settle_round_boundary(services, save)?;
            let allowance = services.proposal_tokens(self);
            self.push_judgment_input("targeted proposals", services.proposal_state(self));
            self.reserve(0, 0, allowance, 1, "targeted proposals", save)?;
            let answer = services.propose(self)?;
            let proposals = self.settle(answer, allowance)?;
            let proposals = self.filter_repeats(proposals);
            if proposals.is_empty() {
                return Err("no supported proposal; bounded diagnosis required".into());
            }
            if proposals.len() > 4 {
                return Err("more than four proposals refused".into());
            }
            let old = self.current.unwrap();
            let mut best = old;
            let mut best_effective = self.effective.clone();
            for proposal in proposals {
                let dial = self
                    .dials
                    .iter()
                    .find(|d| d.id == proposal.dial)
                    .ok_or("unsupported dial")?;
                let patch = match candidate(&self.preset, &self.effective, dial, proposal.action) {
                    Ok(p) => p,
                    Err(reason) => {
                        self.routes
                            .push(format!("invalid proposal {}: {reason}", proposal.dial));
                        continue;
                    }
                };
                let mut overrides = self.overrides.clone();
                merge(&mut overrides, &patch);
                self.reserve(
                    1,
                    services.evaluation_images(),
                    0,
                    0,
                    "candidate evaluation",
                    save,
                )?;
                let mut trial = services.evaluate(
                    overrides,
                    self.budget.rounds,
                    &proposal.dial,
                    Some(proposal.ledger),
                );
                // What this attempt moved, from where, and on what evidence.
                trial.base = Some(self.trials[old].key.clone());
                trial.action = Some(proposal.action);
                trial.direction_mass = proposal.direction_mass;
                trial.rule = proposal.rule.clone();
                trial.evidence = self.visual.as_ref().map(|v| v.ledger.clone());
                self.pending = None;
                if trial.feasible
                    && trial
                        .score
                        .zip(self.trials[best].score)
                        .is_some_and(|(s, b)| s < b)
                {
                    best = self.trials.len();
                    best_effective = self.effective.clone();
                    merge(&mut best_effective, &patch);
                }
                self.trials.push(trial);
                save(self)?;
            }
            if best == old {
                self.routes.push(
                    "numeric stall; reassess remaining defect and recent failed attempts".into(),
                );
                save(self)?;
                continue;
            }
            self.current = Some(best);
            self.effective = best_effective;
            self.overrides = self.trials[best].overrides.clone();
            self.assess(services, save)?;
            if self.bootstrap_finalist(save)? {
                return Ok(());
            }
            if self.machine_ready {
                break;
            }
        }
        save(self)
    }
    pub fn finalists(&self) -> Vec<&Trial> {
        let mut trials = self
            .trials
            .iter()
            .filter(|t| t.identity == self.identity && t.feasible && t.score.is_some())
            .collect::<Vec<_>>();
        trials.sort_by(|a, b| a.score.unwrap().total_cmp(&b.score.unwrap()));
        trials.truncate(3);
        trials
    }
}
