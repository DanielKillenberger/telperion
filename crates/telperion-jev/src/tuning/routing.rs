//! Per-priority routing and the handoffs it grounds. Nothing here dispatches
//! repair work; fn-89 owns that.
use super::{
    continuation,
    engine::{Run, Services},
    handoff::{self, Attempt, CellOutcome, Handoff, PriorityRoute},
    judgments,
    priority::Gap,
    state::CellStatus,
};

impl Run {
    pub(super) fn route_remaining(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
    ) -> Result<(), String> {
        if !self.priority_gate(services, save)? {
            return Err("owner priority approval required before fix routing".into());
        }
        let allowance = services.route_tokens(self);
        self.push_judgment_input("defect routing", services.route_state(self));
        self.reserve(0, 0, allowance, 0, "defect routing", save)?;
        let answer = services.route(self)?;
        let routes = self.settle(answer, allowance)?;
        self.record_ledger(routes.first().map(|r| r.ledger.clone()));
        for route in &routes {
            self.routes.push(label(route));
        }
        save(self)?;
        let questions = services.route_questions(self);
        let mut tuning = false;
        for route in &routes {
            if route.route == "tuning" {
                tuning = true;
                continue;
            }
            let gap = route.gap_id.as_ref().and_then(|id| {
                self.approved_priorities()
                    .and_then(|a| a.ordered.iter().find(|g| &g.id == id))
                    .cloned()
            });
            let criterion = route.gap_id.as_ref().and_then(|id| {
                judgments::criterion(&questions, &format!("route:{id}"), &route.route)
            });
            // An unchanged candidate re-routed after a numeric stall reuses the
            // judgment it already paid for rather than asking it again.
            let authorized = match self.settled_handoff(route) {
                Some(settled) => settled,
                None => {
                    let action = handoff_action(route, gap.as_ref(), criterion.as_deref());
                    self.pre_dispatch(services, save, route, &action)?
                }
            };
            if let Some(gap) = &gap {
                let built = self.build_handoff(route, gap, criterion.as_deref(), authorized)?;
                self.record_handoff(built);
            }
            save(self)?;
        }
        if tuning {
            return Ok(());
        }
        let list = routes.iter().map(label).collect::<Vec<_>>().join(", ");
        if routes.iter().all(|r| !r.grounded()) {
            return Err(format!(
                "uncertainty pause: {list}; no supported diagnosis and no dispatch authorized"
            ));
        }
        Err(format!(
            "fn-89 handoff: {list}; await verified repair then reassess"
        ))
    }

    /// One bounded pre-dispatch judgment per grounded non-tuning route. An
    /// unsupported answer withholds authorization; it never forces a retry.
    fn pre_dispatch(
        &mut self,
        services: &mut dyn Services,
        save: &mut dyn FnMut(&Self) -> Result<(), String>,
        route: &PriorityRoute,
        action: &str,
    ) -> Result<bool, String> {
        if !route.grounded() {
            return Ok(false);
        }
        let mut basis = self.basis(action);
        basis.next_tokens = Some(services.continuation_tokens(&basis));
        basis.estimate_basis =
            "bounded pre-dispatch assessment only; host owns repair estimate".into();
        let allowance = services.continuation_tokens(&basis);
        self.push_judgment_input(
            "pre-dispatch continuation",
            serde_json::to_value(&basis).unwrap(),
        );
        self.reserve(0, 0, allowance, 0, "pre-dispatch continuation", save)?;
        let answer = services.continuation(&basis)?;
        let assessment = self.settle(answer, allowance)?;
        self.record_ledger(Some(assessment.ledger.clone()));
        Ok(continuation::assess(&basis, &self.budget, Some(&assessment), true).is_ok())
    }

    /// Replaces this revision's handoff for the same priority instead of
    /// accumulating stale duplicates.
    fn record_handoff(&mut self, built: Handoff) {
        self.handoffs
            .retain(|h| !(h.run_identity == built.run_identity && h.gap_id == built.gap_id));
        self.handoffs.push(built);
    }

    /// True when this exact priority, candidate and route already carry a
    /// settled handoff for this revision.
    fn settled_handoff(&self, route: &PriorityRoute) -> Option<bool> {
        let key = self.candidate_key();
        self.handoffs
            .iter()
            .find(|h| {
                h.run_identity == self.identity
                    && h.candidate_key == key
                    && h.gap_id == route.gap_id
                    && h.route == route.route
            })
            .map(|h| h.dispatch_authorized)
    }

    fn candidate_key(&self) -> String {
        self.current
            .and_then(|i| self.trials.get(i))
            .map(|t| t.key.clone())
            .unwrap_or_default()
    }

    fn build_handoff(
        &self,
        route: &PriorityRoute,
        gap: &Gap,
        criterion: Option<&str>,
        dispatch_authorized: bool,
    ) -> Result<Handoff, String> {
        let checkpoint = self
            .priority_checkpoints
            .last()
            .ok_or("no approved priority checkpoint")?;
        let approval = self
            .approved_priorities()
            .ok_or("handoff requires owner approval")?;
        let findings = handoff::findings_for(checkpoint, gap);
        let proposed_investigation = if route.grounded() {
            criterion
                .map(str::to_string)
                .unwrap_or_else(|| route.route.clone())
        } else {
            handoff::UNCERTAIN_INVESTIGATION.into()
        };
        Ok(Handoff {
            run_identity: self.identity.clone(),
            candidate_key: self.candidate_key(),
            checkpoint_sha256: approval.checkpoint_sha256.clone(),
            gap_id: route.gap_id.clone(),
            rank: route.rank,
            priority: gap.observation.clone(),
            observed_defect: handoff::observed_defect(checkpoint, gap),
            route: route.route.clone(),
            existing_spec: route.existing_spec(),
            judgment_ledger: route.ledger.clone(),
            raw_choice: route.raw_choice.clone(),
            confidence: route.confidence,
            threshold: route.threshold,
            dispatch_authorized,
            dials_in_router_state: self.dials.iter().map(|d| d.id.clone()).collect(),
            dials_note: handoff::DIALS_NOTE.into(),
            attempts: self.attempts_for(gap),
            observations: findings.iter().map(|f| f.observation.clone()).collect(),
            hypotheses: findings
                .iter()
                .filter_map(|f| f.causal_hypothesis.clone())
                .collect(),
            hypotheses_note: handoff::HYPOTHESES_NOTE.into(),
            unknowns: handoff::unknowns(checkpoint, gap, self.visual.as_ref()),
            proposed_investigation,
            proposed_spending: None,
            proposed_spending_note: handoff::SPENDING_NOTE.into(),
        })
    }

    /// Only candidates this run actually evaluated, named by the dial they
    /// moved. Dials never tried simply do not appear.
    fn attempts_for(&self, gap: &Gap) -> Vec<Attempt> {
        self.trials
            .iter()
            .enumerate()
            .filter(|(_, t)| {
                t.identity == self.identity && self.dials.iter().any(|d| d.id == t.label)
            })
            .map(|(i, t)| Attempt {
                dial: t.label.clone(),
                round: t.round,
                action_ledger: t.ledger.clone(),
                score_before_round: self
                    .trials
                    .iter()
                    .filter(|p| p.identity == self.identity && p.round < t.round && p.feasible)
                    .filter_map(|p| p.score)
                    .min_by(f64::total_cmp),
                score_after: t.score,
                feasible: t.feasible,
                reason: t.reason.clone(),
                visual_outcome: self
                    .visual
                    .as_ref()
                    .filter(|v| self.current == Some(i) && v.identity == t.key)
                    .map(|v| {
                        v.cells
                            .iter()
                            .filter(|(c, _)| gap.views.contains(&c.view))
                            .map(|(c, status)| CellOutcome {
                                item: c.item.clone(),
                                view: c.view.clone(),
                                seed: c.seed,
                                status: *status,
                            })
                            .collect::<Vec<_>>()
                    })
                    .filter(|cells: &Vec<CellOutcome>| !cells.is_empty()),
            })
            .collect()
    }

    /// Priorities that have not been resolved: an open handoff, or an owner
    /// cell that is not passing.
    pub fn unresolved_priorities(&self) -> Vec<String> {
        let mut out: Vec<String> = self
            .handoffs
            .iter()
            .filter(|h| h.run_identity == self.identity)
            .filter_map(|h| h.gap_id.clone())
            .collect();
        if let (Some(approval), Some(visual)) = (self.approved_priorities(), self.visual.as_ref()) {
            for gap in &approval.ordered {
                let tag = format!("owner-priority:{}: ", gap.id);
                let open = visual
                    .cells
                    .iter()
                    .any(|(c, s)| c.item.starts_with(&tag) && *s != CellStatus::Pass);
                if open && !out.contains(&gap.id) {
                    out.push(gap.id.clone());
                }
            }
        }
        out
    }
}

/// Says what the judgment is actually being asked about. A bare route name
/// left an earlier live pilot unable to tell what was proposed.
fn handoff_action(route: &PriorityRoute, gap: Option<&Gap>, criterion: Option<&str>) -> String {
    let Some(gap) = gap else {
        return route.route.clone();
    };
    let criterion = criterion.map(|c| format!(" = {c}")).unwrap_or_default();
    format!(
        "fn-89 handoff for owner priority {} ({}): {}. Route: {}{}. No repair is dispatched by this run; the judgment only decides whether the handoff may be marked dispatch-authorized.",
        route.rank, gap.id, gap.observation, route.route, criterion
    )
}

fn label(route: &PriorityRoute) -> String {
    match &route.gap_id {
        Some(id) => format!("{id}={}", route.route),
        None => route.route.clone(),
    }
}
