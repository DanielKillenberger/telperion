//! Evidence-only one-round Jev harness. Original run.json is not written.
//! Calls existing Live route/continuation/propose/evaluate. No execute().
use serde_json::Value;
use std::{fs, path::Path};
use telperion_jev::{
    caller::{load_key, UreqTransport},
    sha256_hex,
    tuning::{
        actions::candidate,
        continuation,
        engine::{Proposal, Run, Services},
        live::{Config, Live},
        state::{Cell, Visual},
    },
};

const RUNTIME: &str = ".flow/tmp/fn68-pilot-run/run.json";
const RUNTIME_SHA: &str = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e";
const SIDE: &str = ".flow/tmp/fn68-r7-side/run.json";
const VISUAL: &str = ".flow/tmp/fn68-r7-side/visual.json";
const REQUIRED: &str = ".flow/tmp/fn68-r7-side/required.json";
const CONFIG: &str =
    ".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev/pilot-config-final-diagnosed.json";
const OUT: &str = ".flow/tmp/fn68-r7-side/jev-round.json";

fn write(path: &str, value: &Value) {
    fs::write(path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
}

fn main() {
    eprintln!("Archived invalid diagnostic experiment: execution disabled. This harness bypassed Config.verify, reset persisted spend, omitted owner priority approval, and paired a baseline trial with a different candidate visual. Repair the production scoped-resume path; do not rerun this example.");
    std::process::exit(2);
}

fn run() -> Result<(), String> {
    assert_eq!(
        sha256_hex(&fs::read(RUNTIME).map_err(|e| e.to_string())?),
        RUNTIME_SHA
    );
    let visual: Visual = serde_json::from_slice(&fs::read(VISUAL).map_err(|e| e.to_string())?)
        .map_err(|e| format!("visual: {e}"))?;
    let required: Vec<Cell> =
        serde_json::from_slice(&fs::read(REQUIRED).map_err(|e| e.to_string())?)
            .map_err(|e| format!("required: {e}"))?;
    let mut state: Run = serde_json::from_slice(&fs::read(SIDE).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    state.pause = None;
    state.pending = None;
    state.machine_ready = false;
    state.visual = Some(visual);
    state.required = required;
    state.budget.tokens = 677247;
    state.budget.max_tokens = 902431;
    state.budget.visual_passes = Some(25);
    state.budget.max_visual_passes = Some(26);
    state.budget.evaluations = 6;
    state.budget.max_evaluations = 13;
    state.budget.images = 30;
    state.budget.max_images = 52;
    state.budget.rounds = 2;
    state.budget.max_rounds = 3;
    state.usage_known = true;
    let mut save = |s: &Run| {
        write(SIDE, &serde_json::to_value(s).unwrap());
        Ok::<(), String>(())
    };
    save(&state)?;
    let config: Config = serde_json::from_slice(&fs::read(CONFIG).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if let Err(reason) = config.verify() {
        if reason != "joint visual protocol requires fresh calibration" {
            write(
                OUT,
                &serde_json::json!({"status":"interface_blocker","where":"Config.verify","reason":reason}),
            );
            return Err(format!("Config.verify: {reason}"));
        }
        write(
            ".flow/tmp/fn68-r7-side/visual-verify-skip.json",
            &serde_json::json!({
                "reason": reason,
                "meaning": "Live.visual_for is unused on this seam. r7-current already used the existing Astra adapter. Final, if any, uses that adapter. Magnitude, direction, and continuation calibration already passed inside verify before this visual-replay check."
            }),
        );
    }
    let key = load_key().map_err(|e| e.to_string())?;
    let mut live = Live {
        config: &config,
        transport: &UreqTransport,
        key: &key,
    };
    let route_need = live.route_tokens(&state);
    if route_need > 28000 {
        write(
            OUT,
            &serde_json::json!({"status":"stop_before_call","stage":"route","serialized_plus_1024":route_need,"ceiling":28000}),
        );
        return Err(format!("route serialized+1024 {route_need} exceeds 28000"));
    }
    state.budget.reserve(0, 0, route_need, 0)?;
    state.pending = Some("defect routing".into());
    save(&state)?;
    write(
        ".flow/tmp/fn68-r7-side/route-reservation.json",
        &serde_json::json!({"stage":"route","ceiling":28000,"serialized_plus_1024":route_need,"prior_tokens":677247}),
    );
    let routed = live.route(&state)?;
    let route_used = routed.tokens.ok_or("unknown route usage")?;
    if route_used > route_need {
        return Err("route exceeded reservation".into());
    }
    state.budget.tokens = state
        .budget
        .tokens
        .checked_sub(route_need)
        .and_then(|n| n.checked_add(route_used))
        .ok_or("route settle overflow")?;
    state.pending = None;
    let route = routed.value;
    state.routes.push(route.clone());
    save(&state)?;
    if route != "tuning" {
        write(
            OUT,
            &serde_json::json!({"status":"typed_stop","stage":"route","route":route,"used":route_used}),
        );
        return Err(format!("typed stop after route: {route}"));
    }
    let mut basis = state.round_basis(&live)?;
    let cont_need = live.continuation_tokens(&basis);
    if cont_need > 32000 {
        write(
            OUT,
            &serde_json::json!({"status":"stop_before_call","stage":"continuation","serialized_plus_1024":cont_need,"ceiling":32000}),
        );
        return Err(format!(
            "continuation serialized+1024 {cont_need} exceeds 32000"
        ));
    }
    basis.next_tokens = Some(
        live.proposal_tokens(&state)
            .checked_add(45000)
            .ok_or("reservation overflow")?,
    );
    state.budget.reserve(0, 0, cont_need, 0)?;
    state.pending = Some("continuation judgment".into());
    save(&state)?;
    let continued = live.continuation(&basis)?;
    let actual = continued.tokens.ok_or("unknown continuation usage")?;
    state.budget.tokens = state
        .budget
        .tokens
        .checked_sub(cont_need)
        .and_then(|n| n.checked_add(actual))
        .ok_or("continuation settle overflow")?;
    state.pending = None;
    let assessment = continued.value;
    save(&state)?;
    continuation::assess(&basis, &state.budget, Some(&assessment), true).map_err(|e| {
        write(
            OUT,
            &serde_json::json!({"status":"typed_stop","stage":"continuation","reason":e,"used":actual,"assessment":assessment}),
        );
        e
    })?;
    let prop_need = live.proposal_tokens(&state);
    if prop_need > 36000 {
        write(
            OUT,
            &serde_json::json!({"status":"stop_before_call","stage":"proposal-magnitude","serialized_plus_1024":prop_need,"ceiling":36000}),
        );
        return Err(format!(
            "proposal serialized+1024 {prop_need} exceeds 36000"
        ));
    }
    state.budget.reserve(0, 0, prop_need, 1)?;
    state.pending = Some("targeted proposals".into());
    save(&state)?;
    let proposed = live.propose(&state)?;
    let actual = proposed.tokens.ok_or("unknown proposal usage")?;
    state.budget.tokens = state
        .budget
        .tokens
        .checked_sub(prop_need)
        .and_then(|n| n.checked_add(actual))
        .ok_or("proposal settle overflow")?;
    state.pending = None;
    let proposals: Vec<Proposal> = proposed.value;
    save(&state)?;
    if proposals.is_empty() {
        write(
            OUT,
            &serde_json::json!({"status":"typed_stop","stage":"proposal-magnitude","reason":"no supported proposal"}),
        );
        return Err("no supported proposal; bounded diagnosis required".into());
    }
    if proposals.len() > 4 {
        return Err("more than four proposals refused".into());
    }
    if !Path::new("target/release/examples/species_measure").exists() {
        let src = Path::new("/home/daniel/Projects/telperion/target/release/examples");
        fs::create_dir_all("target/release/examples").map_err(|e| e.to_string())?;
        for name in ["species_measure", "headless"] {
            let dest = Path::new("target/release/examples").join(name);
            if !dest.exists() {
                std::os::unix::fs::symlink(src.join(name), dest).map_err(|e| e.to_string())?;
            }
        }
    }
    let old = state.current.ok_or("no current trial")?;
    let mut best = old;
    let mut best_effective = state.effective.clone();
    let mut evals = Vec::new();
    for proposal in &proposals {
        let dial = state
            .dials
            .iter()
            .find(|d| d.id == proposal.dial)
            .ok_or("unsupported dial")?;
        let patch = match candidate(
            &state.preset,
            &state.effective,
            dial,
            proposal.action.clone(),
        ) {
            Ok(p) => p,
            Err(reason) => {
                evals.push(serde_json::json!({"dial":proposal.dial,"invalid":reason}));
                continue;
            }
        };
        let mut overrides = state.overrides.clone();
        merge(&mut overrides, &patch);
        let images = live.evaluation_images();
        state.budget.reserve(1, images, 0, 0)?;
        state.pending = Some("candidate evaluation".into());
        save(&state)?;
        let trial = live.evaluate(
            overrides,
            state.budget.rounds,
            &proposal.dial,
            Some(proposal.ledger.clone()),
        );
        state.pending = None;
        evals.push(serde_json::json!({
            "dial": proposal.dial,
            "action": proposal.action,
            "feasible": trial.feasible,
            "score": trial.score,
            "reason": trial.reason,
            "key": trial.key
        }));
        if trial.feasible
            && trial
                .score
                .zip(state.trials[best].score)
                .is_some_and(|(s, b)| s < b)
        {
            best = state.trials.len();
            best_effective = state.effective.clone();
            merge(&mut best_effective, &patch);
        }
        state.trials.push(trial);
        save(&state)?;
    }
    let winner = best != old;
    if winner {
        state.current = Some(best);
        state.effective = best_effective;
        state.overrides = state.trials[best].overrides.clone();
    }
    save(&state)?;
    write(
        OUT,
        &serde_json::json!({
            "status": if winner { "numeric_winner" } else { "numeric_stall" },
            "route": route,
            "route_used": route_used,
            "continuation": assessment,
            "proposals": proposals,
            "evaluations": evals,
            "winner": winner,
            "tokens": state.budget.tokens,
            "evaluations_used": state.budget.evaluations,
            "images": state.budget.images,
            "rounds": state.budget.rounds,
            "original_runtime_sha256": RUNTIME_SHA,
            "original_untouched": sha256_hex(&fs::read(RUNTIME).unwrap()) == RUNTIME_SHA
        }),
    );
    if !winner {
        return Err("numeric stall; reassess remaining defect and recent failed attempts".into());
    }
    Ok(())
}

fn merge(dst: &mut Value, patch: &Value) {
    match (dst, patch) {
        (Value::Object(d), Value::Object(p)) => {
            for (k, v) in p {
                merge(d.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (d, p) => *d = p.clone(),
    }
}
