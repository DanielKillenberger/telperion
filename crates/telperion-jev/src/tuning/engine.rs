//! One tuning revision: rounds over the live dials until a round keeps
//! nothing. Nothing here asks permission; a revision ends, and the runner's
//! Gaps and Accept stages read what it left.
use super::{
    actions::{Action, Dial},
    evaluation::Trial,
    priority::Approval,
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
    /// The strengths one bundle is drawn at, as multiples of each dial's own
    /// authored small step.
    fn bundle_strengths(&self) -> Vec<f64> {
        vec![0.5, 1.0, 2.0, 4.0]
    }
    /// How many sheet reviews one round may spend isolating what breaks.
    fn max_split_reviews(&self) -> u64 {
        6
    }
    /// The tracks a round runs, in order. Empty is one track over every dial.
    fn tracks(&self) -> Vec<super::bundle::Track> {
        vec![]
    }
    /// The reference-first inventory, when the run carries one.
    fn inventory(&self) -> Result<Option<super::reference_first::Inventory>, String> {
        Ok(None)
    }
    /// Traits the generator cannot draw yet; one going backwards never vetoes.
    fn unexpressed(&self) -> Vec<super::unexpressed::Unexpressed> {
        vec![]
    }
    /// The gap class an owner priority carries in the config. Asks nobody.
    fn owner_magnitude(&self, _priority: &str) -> Option<super::stride::Class> {
        None
    }
    /// Whether the gap-magnitude question can be asked at all. Without it
    /// every round draws the ladder as configured.
    fn offers_gap_magnitude(&self) -> bool {
        false
    }
    fn gap_magnitude_tokens(&self, _state: &Value) -> u64 {
        2000
    }
    fn gap_magnitude(&mut self, _state: &Value) -> Result<Answer<super::stride::Judged>, String> {
        Err("gap-magnitude question unavailable".into())
    }
    /// Two stills per view, as any matched capture costs.
    fn capture_images(&self, views: &[String]) -> u64 {
        2 * views.len() as u64
    }
    /// The views a track is judged at that the evaluation does not render.
    fn capture_views(
        &mut self,
        _trial: &Trial,
        _views: &[String],
    ) -> Result<Vec<super::evaluation::Comparison>, String> {
        Err("extra-view capture unavailable".into())
    }
    /// Which variants a sheet is worth showing, and the sheet it asks for.
    /// A track with a fixed view is judged there and nowhere else.
    fn sheet_request(
        &self,
        _state: &Run,
        _current: usize,
        _variants: &[usize],
        _priorities: &[super::priority::Gap],
        _view: Option<&str>,
    ) -> Result<super::sheet::Look, String> {
        Err("contact-sheet review unavailable".into())
    }
    fn sheet_tokens(&self, _request: &super::sheet::Request) -> u64 {
        0
    }
    fn sheet(
        &mut self,
        _plan: &super::sheet::Plan,
    ) -> Result<Answer<super::sheet::Verdict>, String> {
        Err("contact-sheet review unavailable".into())
    }
    fn side_effect_tokens(&self, _state: &Value) -> u64 {
        2000
    }
    /// The one uncalibrated question that can take an adoption back.
    fn side_effects(&mut self, _state: &Value) -> Result<Answer<super::veto::Judged>, String> {
        Err("side-effect question unavailable".into())
    }
    fn proposal_state(&self, state: &Run) -> Value {
        super::judgments::proposal_state(state)
    }
    /// The question set the router was shown, used to quote the chosen criterion.
    /// Consecutive rounds that keep nothing before the run pauses as a runaway.
    fn runaway_rounds(&self) -> u64 {
        super::runaway::ROUNDS
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
    /// How many calls the dial table is asked in. One unless the config caps
    /// the questions per call.
    fn proposal_batches(&self, _state: &Run) -> usize {
        1
    }
    fn propose(&mut self, state: &Run, batch: usize) -> Result<Answer<Vec<Proposal>>, String>;
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
    /// Why the revision stopped before a round kept nothing, if it did.
    pub stopped: Option<String>,
    pub machine_ready: bool,
    pub pending: Option<String>,
    pub routes: Vec<String>,
    /// The objectives the revision tunes toward: the latest checkpoint's
    /// proposal, taken as it stands.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval: Option<Approval>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub preparation_charge: Option<super::reference_first::PreparationCharge>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub priority_checkpoints: Vec<super::priority::Checkpoint>,
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
    /// Per track, the gap class its stride may not exceed and the class its
    /// last adopted bundle was drawn at.
    #[serde(default, skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    pub strides: std::collections::BTreeMap<String, super::stride::Standing>,
    /// The trailing rounds that kept nothing, and the spend they opened on.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unkept: Option<super::runaway::Streak>,
}

impl Run {
    pub fn priority_scope(&self, references: &[super::evaluation::Image]) -> String {
        super::priority::scope(&self.preset, &self.owner_notes, &self.required, references)
    }
    pub fn approved_priorities(&self) -> Option<&Approval> {
        let checkpoint = self.priority_checkpoints.last()?;
        self.approval
            .as_ref()
            .filter(|a| a.checkpoint_sha256 == checkpoint.hash())
    }
    /// Evidence this revision measured, as opposed to a record it was handed.
    pub fn measured_here(&self, identity: &str) -> bool {
        identity == self.identity
    }
    pub fn required_cells(&self) -> Vec<Cell> {
        super::priority::requirements(&self.required, self.approved_priorities())
    }
    /// The objectives: a new checkpoint whenever the scope changed, and its
    /// proposal taken as the revision's objectives.
    pub(super) fn priority_gate(&mut self, services: &dyn Services) -> Result<(), String> {
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
            let mut checkpoint =
                super::priority::Checkpoint::new(&self.identity, &scope, visual, evidence)?;
            super::objectives::offer(self, services, &mut checkpoint)?;
            self.priority_checkpoints.push(checkpoint);
        }
        let checkpoint = self.priority_checkpoints.last().unwrap();
        if self.approved_priorities().is_none() {
            self.approval = Some(Approval {
                checkpoint_sha256: checkpoint.hash(),
                scope_sha256: scope.clone(),
                ordered: checkpoint.proposed.clone(),
            });
        }
        let approval = self.approved_priorities().unwrap();
        approval.verify(checkpoint, &scope)?;
        super::objectives::verify_tracks(&approval.ordered, &services.tracks())
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
        );
        save(self)?;
        Ok(true)
    }
    pub(super) fn stop(&mut self, reason: String) {
        self.machine_ready = false;
        self.stopped = Some(reason);
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
        self.budget.reserve(evaluations, images, tokens, rounds);
        self.pending = Some(label.into());
        save(self)
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
        if actual > reserved {
            return Err("judgment exceeded reservation".into());
        }
        Ok(answer.value)
    }
    pub(super) fn assess(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
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
        self.budget.reserve_visual();
        let images = services.visual_images_for(&trial, &required, approval.as_ref());
        self.budget.reserve(0, images, allowance, 0);
        self.pending = Some("visual assessment".into());
        save(self)?;
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
        if let Err(reason) = self.execute_inner(services, save) {
            self.stop(reason);
            save(self)?;
        }
        Ok(())
    }
    fn execute_inner(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
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
        self.priority_gate(services)?;
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
            self.priority_gate(services)?;
            if super::look::objectives(self).is_empty() {
                return Err("no objective to tune toward".into());
            }
            self.runaway(services.runaway_rounds())?;
            let opening = super::runaway::Spend::of(&self.budget);
            // Each batch of dials is its own reserved, settled and persisted
            // judgment; the round is charged once.
            let mut proposals = vec![];
            for batch in 0..services.proposal_batches(self) {
                let allowance = services.proposal_tokens(self);
                let label = format!("targeted proposals {}", batch + 1);
                self.push_judgment_input(&label, services.proposal_state(self));
                let rounds = u64::from(batch == 0);
                self.reserve(0, 0, allowance, rounds, &label, save)?;
                let answer = services.propose(self, batch)?;
                proposals.extend(self.settle(answer, allowance)?);
            }
            proposals.sort_by(|a: &Proposal, b: &Proposal| {
                b.direction_mass
                    .unwrap_or(0.)
                    .total_cmp(&a.direction_mass.unwrap_or(0.))
            });
            // One bundle of every dial Jev supported, judged on one sheet.
            let kept = super::bundle::round(self, proposals, services, save)?;
            self.close_round(kept, opening, save)?;
            if kept && self.bootstrap_finalist(save)? {
                return Ok(());
            }
        }
        save(self)
    }
    pub fn finalists(&self) -> Vec<&Trial> {
        let mut trials = self
            .trials
            .iter()
            .filter(|t| self.measured_here(&t.identity) && t.feasible && t.score.is_some())
            .collect::<Vec<_>>();
        trials.sort_by(|a, b| a.score.unwrap().total_cmp(&b.score.unwrap()));
        trials.truncate(3);
        trials
    }
}
