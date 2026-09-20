//! One bounded species run. Capability repair is returned to fn-89, never dispatched here.
use super::{
    actions::{candidate, Action, Dial},
    continuation::{self, Assessment, Basis, Pause},
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
}

pub struct Answer<T> {
    pub value: T,
    pub tokens: Option<u64>,
}

pub trait Services {
    fn proposal_tokens(&self, _state: &Run) -> u64 {
        4000
    }
    fn continuation_tokens(&self, _basis: &Basis) -> u64 {
        2000
    }
    fn route_tokens(&self, _state: &Run) -> u64 {
        2000
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
    fn continuation(&mut self, basis: &Basis) -> Result<Answer<Assessment>, String>;
    fn propose(&mut self, state: &Run) -> Result<Answer<Vec<Proposal>>, String>;
    fn route(&mut self, state: &Run) -> Result<Answer<String>, String>;
}

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
    pub fn pilot_authority(&self) -> Result<(), String> {
        self.authorizations
            .last()
            .and_then(|d| d.experimental_pilot.as_ref())
            .ok_or("magnitude live efficacy unvalidated")?
            .verify(&self.identity, &self.budget)
    }
    fn stop(&mut self, reason: String, action: &str) {
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
    fn basis(&self, action: &str) -> Basis {
        let projection = super::judgments::summary(self);
        let mut evidence = self
            .visual
            .as_ref()
            .map(|v| v.defects.clone())
            .unwrap_or_default();
        evidence.push(
            serde_json::json!({
                "current_identity":projection["current_identity"],
                "resource_limit":projection["resource_limit"],
            "resource_amendments":projection["resource_amendments"],
            "agent_diagnoses":projection["agent_diagnoses"]
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
            .checked_add(services.visual_tokens(&finalist))
            .ok_or("reservation overflow")?;
        basis.next_tokens = Some(next);
        basis.estimate_basis = format!("proposal serialized-request bound {} + all-cell visual reservation {}; actual usage may exceed estimate and then pauses", services.proposal_tokens(self), services.visual_tokens(&finalist));
        Ok(basis)
    }
    fn reserve(
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
    fn settle<T>(&mut self, answer: Answer<T>, reserved: u64) -> Result<T, String> {
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
        let allowance = services.visual_tokens(&trial);
        let mut budget = self.budget.clone();
        budget.reserve_visual()?;
        budget.reserve(0, services.visual_images(&trial), allowance, 0)?;
        self.budget = budget;
        self.pending = Some("visual assessment".into());
        save(self)?;
        self.verify_diagnoses()?;
        let answer = services.visual(&trial)?;
        let visual = self.settle(answer, allowance)?;
        self.machine_ready = ready(&self.required, &trial.key, &visual);
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
        while !self.machine_ready {
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
            let planned = basis
                .next_tokens
                .unwrap()
                .checked_add(services.continuation_tokens(&basis))
                .and_then(|n| n.checked_add(services.route_tokens(self)))
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
            let allowance = services.continuation_tokens(&basis);
            self.reserve(0, 0, allowance, 0, "continuation judgment", save)?;
            let answer = services.continuation(&basis)?;
            let assessment = self.settle(answer, allowance)?;
            continuation::assess(&basis, &self.budget, Some(&assessment), true)?;
            let allowance = services.proposal_tokens(self);
            self.reserve(0, 0, allowance, 1, "targeted proposals", save)?;
            let answer = services.propose(self)?;
            let proposals = self.settle(answer, allowance)?;
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
                let trial = services.evaluate(
                    overrides,
                    self.budget.rounds,
                    &proposal.dial,
                    Some(proposal.ledger),
                );
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
            if self.machine_ready {
                break;
            }
        }
        save(self)
    }
    fn route_remaining(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        let allowance = services.route_tokens(self);
        self.reserve(0, 0, allowance, 0, "defect routing", save)?;
        let answer = services.route(self)?;
        let route = self.settle(answer, allowance)?;
        self.routes.push(route.clone());
        save(self)?;
        if route != "tuning" {
            let mut basis = self.basis(&route);
            basis.next_tokens = Some(services.continuation_tokens(&basis));
            basis.estimate_basis =
                "bounded pre-dispatch assessment only; host owns repair estimate".into();
            let allowance = services.continuation_tokens(&basis);
            self.reserve(0, 0, allowance, 0, "pre-dispatch continuation", save)?;
            let answer = services.continuation(&basis)?;
            let assessment = self.settle(answer, allowance)?;
            continuation::assess(&basis, &self.budget, Some(&assessment), true)?;
            return Err(format!(
                "fn-89 handoff: {route}; await verified repair then reassess"
            ));
        }
        Ok(())
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
