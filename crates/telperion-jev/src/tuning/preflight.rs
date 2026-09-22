//! Read-only execution plan. Takes no lock, writes no run state, loads no key
//! and dispatches nothing; every number comes from the engine's own estimators.
use super::{
    command::{prepare, Prepared},
    continuation::Basis,
    engine::{Run, Services},
    evaluation::Trial,
    live::{Config, Live},
    priority::requirements,
    state::Budget,
};
use crate::caller::{HttpRequest, HttpResponse, Transport};
use serde_json::{json, Value};
use std::{fs, path::Path};

/// Refuses every send. A preflight that dispatched would be a paid call.
struct NoTransport;
impl Transport for NoTransport {
    fn send(&self, _: &HttpRequest) -> Result<HttpResponse, String> {
        Err("preflight dispatches nothing".into())
    }
}

/// The worst case the config permits, and the leanest round.
fn candidate_counts(config: &Config) -> [u64; 2] {
    [
        config
            .max_candidates
            .unwrap_or(super::live::CANDIDATE_LIMIT),
        1,
    ]
}

fn step(label: &str, evaluations: u64, images: u64, tokens: u64, visual: u64) -> Value {
    json!({"step":label,"evaluations":evaluations,"images":images,"tokens":tokens,"visual_passes":visual})
}

/// A stand-in candidate when the run has not evaluated one yet.
fn placeholder(config: &Config, identity: &str) -> Trial {
    Trial {
        progress: None,
        adopted_over: vec![],
        bundle: None,
        parent_bundle: None,
        sheet: None,
        vetoed: None,
        key: String::new(),
        identity: identity.into(),
        seed: config.seed,
        round: 0,
        label: "baseline".into(),
        overrides: config.initial_overrides.clone(),
        ledger: None,
        feasible: false,
        reason: None,
        measurement: Value::Null,
        comparisons: vec![],
        score: None,
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
    }
}

fn round_basis(state: &Run, services: &dyn Services) -> (Basis, bool) {
    match state.round_basis(services) {
        Ok(basis) => (basis, false),
        Err(_) => {
            let mut basis = state.basis("targeted tuning round");
            basis.next_tokens = Some(1);
            basis.estimate_basis = "estimate_from_current_state".into();
            (basis, true)
        }
    }
}

pub fn plan(config_path: &Path, out: &Path, resume: Option<&Path>) -> Result<Value, String> {
    let config: Config = serde_json::from_slice(&fs::read(config_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let identity = config.identity()?;
    let path = out.join("run.json");
    let (state, interrupted) = match prepare(&config, &path, resume, &identity)? {
        Prepared::Ready(state) => (*state, false),
        Prepared::Interrupted(state) => (*state, true),
    };
    let verified = config.verify();
    let authority = state.pilot_authority();
    let opening = state.budget.clone();
    let mut post = opening.clone();
    // A blocker is reported as a field; the reservations are still worth having.
    let preparation = config.preparation();
    let mut charge_record = state.preparation_charge.clone();
    if let Ok(Some(charge)) = &preparation {
        super::reference_first::charge_preparation(&mut post, &mut charge_record, charge)?;
    }

    let services = Live {
        config: &config,
        transport: &NoTransport,
        key: "",
    };
    let trial = state
        .current
        .and_then(|i| state.trials.get(i))
        .cloned()
        .unwrap_or_else(|| placeholder(&config, &identity));
    let estimated = state.current.is_none();
    let approval = state.approved_priorities();
    let base = config.required.clone();
    let approved_cells = requirements(&base, approval);
    let (basis, basis_estimated) = round_basis(&state, &services);

    let all_cell_images = services.visual_images_for(&trial, &approved_cells, approval);
    let all_cell_tokens = services.visual_tokens_for(&trial, approval);
    let priorities = approval.map_or(0, |a| a.ordered.len()) as u64;
    let continuation = services.continuation_tokens(&basis);
    // Worst case per round: the router, one risk question per approved
    // priority that could become a handoff, at most one evidence question, and
    // the proposals.
    let evidence = services.evidence_tokens(&state.evidence_state());
    let round_tokens = services
        .route_tokens(&state)
        .checked_add(continuation.checked_mul(priorities).ok_or("overflow")?)
        .and_then(|n| n.checked_add(evidence))
        .and_then(|n| n.checked_add(services.proposal_tokens(&state)))
        .ok_or("reservation overflow")?;

    let bundle = services.selection() == super::progress::Selection::Bundle;
    let review = (!services.selection().is_score() && !bundle)
        .then(|| super::progress::skeleton(&state, &config.references));
    let review_tokens = review
        .as_ref()
        .map(|r| 30_000 + serde_json::to_vec(r).unwrap().len() as u64)
        .unwrap_or(0);
    // One bundle at every configured strength, one sheet over it, and the
    // worst case of the split loop: two halves and a sheet per split review.
    let strengths = services.bundle_strengths().len() as u64;
    let splits = services.max_split_reviews();
    let sheet_tokens = if bundle {
        30_000
            + serde_json::to_vec(&super::sheet::skeleton(&state, &config.references))
                .unwrap()
                .len() as u64
    } else {
        0
    };
    let sequence = |candidates: u64| {
        let mut steps = vec![
            step("baseline evaluation", 1, services.evaluation_images(), 0, 0),
            step(
                "initial visual",
                0,
                services.visual_images(&trial),
                services.visual_tokens(&trial),
                1,
            ),
            step(
                "post-approval all-cell visual",
                0,
                all_cell_images,
                all_cell_tokens,
                1,
            ),
            step(
                &format!("one round, {candidates} candidates"),
                if bundle { 0 } else { candidates },
                if bundle {
                    0
                } else {
                    candidates * services.evaluation_images()
                },
                round_tokens,
                0,
            ),
            step(
                "closing all-cell visual",
                0,
                all_cell_images,
                all_cell_tokens,
                1,
            ),
        ];
        if bundle {
            steps.push(step(
                &format!("one bundle at {strengths} strengths"),
                strengths,
                strengths * services.evaluation_images(),
                0,
                0,
            ));
            steps.push(step("contact sheet review", 0, 0, sheet_tokens, 1));
            steps.push(step(
                &format!("worst case {splits} split sheets"),
                2 * splits,
                2 * splits * services.evaluation_images(),
                splits * sheet_tokens,
                splits,
            ));
        }
        if review_tokens > 0 {
            steps.push(step(
                &format!("progress review, {candidates} candidates"),
                0,
                0,
                candidates * review_tokens,
                candidates,
            ));
        }
        steps
    };
    let summed = |steps: &[Value], field: &str| -> u64 {
        steps
            .iter()
            .filter_map(|s| s[field].as_u64())
            .fold(0, u64::saturating_add)
    };
    let totals_for = |steps: &[Value]| {
        totals(
            &post,
            summed(steps, "evaluations"),
            summed(steps, "images"),
            summed(steps, "tokens"),
            summed(steps, "visual_passes"),
        )
    };
    let counts = candidate_counts(&config);
    let steps = sequence(counts[0]);
    let lean = sequence(counts[1]);
    Ok(json!({
        "meaning":"Worst-case reservations for the full sequence. No state was written, no lock taken, no key loaded and nothing dispatched.",
        "identity":identity,
        "interrupted_attempt":interrupted,
        "calibration_verified":verified.is_ok(),
        "calibration_error":verified.err(),
        "visual_bootstrap":config.visual_bootstrap,
        "owner_relabels":config.owner_relabels.iter().map(|r| json!({
            "case_id":r.case_id,"by":r.by,"verdict":r.verdict,
            "evidence":r.evidence,"verified":r.verify().is_ok(),
            "error":r.verify().err()})).collect::<Vec<_>>(),
        "pilot_authority":authority.is_ok(),
        "pilot_authority_error":authority.err(),
        "estimate_from_current_state":{
            "candidate":estimated,
            "round_basis":basis_estimated,
            "note":"No evaluated candidate yet, so per-candidate sizes are estimated from current state."},
        "required_cells":{"base":base.len(),"with_approval":approved_cells.len(),
            "approved_priorities":priorities,"cells":approved_cells},
        "balances":{"opening":opening,"post_preparation":post,
            "preparation_charge":preparation.as_ref().ok().cloned().flatten(),
            "preparation_error":preparation.as_ref().err()},
        "steps":steps,
        "totals":totals_for(&steps),
        "totals_one_candidate":totals_for(&lean),
    }))
}

fn totals(budget: &Budget, evaluations: u64, images: u64, tokens: u64, visual: u64) -> Value {
    let fits = |spent: u64, need: u64, cap: u64| json!({"spent":spent,"needed":need,"cap":cap,"fits":spent.saturating_add(need) <= cap});
    json!({
        "evaluations":fits(budget.evaluations, evaluations, budget.max_evaluations),
        "images":fits(budget.images, images, budget.max_images),
        "tokens":fits(budget.tokens, tokens, budget.max_tokens),
        "visual_passes":fits(budget.visual_passes.unwrap_or(0), visual, budget.max_visual_passes.unwrap_or(0)),
    })
}
