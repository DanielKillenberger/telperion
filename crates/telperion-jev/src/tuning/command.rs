use super::{
    continuation::{Basis, HumanDecision, Pause},
    engine::Run,
    live::{Config, Live},
};
use crate::caller::{load_key, Transport, UreqTransport};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

/// Supplies the API key at dispatch time. Never called before the run has
/// passed every pre-dispatch pause, so an absent key cannot mask a pause.
pub type KeySource<'a> = &'a dyn Fn() -> Result<String, String>;

fn write(path: &Path, value: &Value) -> Result<(), String> {
    let temp = path.with_extension(format!("pending-{}", crate::ledger::new_entry_id()));
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)
        .map_err(|e| e.to_string())?;
    file.write_all(&serde_json::to_vec_pretty(value).unwrap())
        .map_err(|e| e.to_string())?;
    file.sync_all().map_err(|e| e.to_string())?;
    fs::rename(temp, path).map_err(|e| e.to_string())
}

/// Loads the fresh or resumed run exactly as the command does, without
/// writing anything. The caller decides whether an interruption is persisted.
pub enum Prepared {
    Ready(Box<Run>),
    Interrupted(Box<Run>),
}

pub fn prepare(
    config: &Config,
    path: &Path,
    resume: Option<&Path>,
    identity: &str,
) -> Result<Prepared, String> {
    let identity = identity.to_string();
    config.budget.validate()?;
    let state: Run = if path.exists() {
        let mut old: Run = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if old.pending.is_some() && old.pause.is_none() {
            old.pause = Some(Pause {
                id: crate::ledger::new_entry_id(),
                identity: old.identity.clone(),
                reason: "interrupted attempt; spend retained".into(),
                basis: Basis {
                    identity: old.identity.clone(),
                    proposed_action: "recover interrupted attempt".into(),
                    evidence: vec![],
                    recent_outcomes: vec![],
                    next_tokens: None,
                    estimate_basis: String::new(),
                    usage_known: old.usage_known,
                },
                decision_requested:
                    "Confirm scoped recovery; unknown model spend still requires reconciliation."
                        .into(),
            });
            return Ok(Prepared::Interrupted(Box::new(old)));
        }
        let decision: HumanDecision = serde_json::from_slice(
            &fs::read(resume.ok_or("existing run needs scoped --resume decision")?)
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        old.pause
            .as_ref()
            .ok_or("run is not paused")?
            .resume(&decision)?;
        if old
            .pause
            .as_ref()
            .is_some_and(|p| p.basis.proposed_action == "approve gap priorities")
            && decision.priority_approval.is_none()
        {
            return Err("explicit owner priority approval required".into());
        }
        if let Some(diagnosis) = &decision.diagnosis {
            diagnosis.verify(&identity)?;
        }
        if old.identity != identity && decision.next_identity.as_deref() != Some(&identity) {
            return Err("changed inputs require a scoped decision naming next_identity".into());
        }
        let previous_budget = old.budget.clone();
        let previous_cap = previous_budget.max_tokens;
        if let Some(extension) = &decision.token_cap_extension {
            if extension.previous != previous_cap
                || extension.next != config.budget.max_tokens
                || extension.next <= extension.previous
            {
                return Err("token extension must name exact previous and increased cap".into());
            }
            old.budget.max_tokens = extension.next;
        }
        if let Some(reconciliation) = &decision.visual_reconciliation {
            if old.budget.visual_passes.is_some()
                || old.budget.max_visual_passes.is_some()
                || reconciliation.reason.trim().is_empty()
                || reconciliation.paid_ledgers.is_empty()
            {
                return Err(
                    "visual accounting reconciliation is initial and evidence-backed only".into(),
                );
            }
            let mut unique = std::collections::HashSet::new();
            for path in &reconciliation.paid_ledgers {
                if !unique.insert(fs::canonicalize(path).map_err(|e| e.to_string())?) {
                    return Err("duplicate visual ledger".into());
                }
                let record: Value =
                    serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
                        .map_err(|e| e.to_string())?;
                if record.get("status").is_none() {
                    return Err("not a visual attempt ledger".into());
                }
            }
            let spent = unique.len() as u64;
            if spent > reconciliation.previous_cap {
                return Err("prior visual cap exceeded".into());
            }
            old.budget.visual_passes = Some(spent);
            old.budget.max_visual_passes = Some(reconciliation.previous_cap);
        }
        for (label, extension, previous, next) in [
            (
                "round",
                decision.round_cap_extension.as_ref(),
                old.budget.max_rounds,
                config.budget.max_rounds,
            ),
            (
                "visual",
                decision.visual_cap_extension.as_ref(),
                old.budget.max_visual_passes.unwrap_or(0),
                config.budget.max_visual_passes.unwrap_or(0),
            ),
            (
                "image",
                decision.image_cap_extension.as_ref(),
                old.budget.max_images,
                config.budget.max_images,
            ),
            (
                "evaluation",
                decision.evaluation_cap_extension.as_ref(),
                old.budget.max_evaluations,
                config.budget.max_evaluations,
            ),
        ] {
            if let Some(extension) = extension {
                if extension.previous != previous || extension.next != next || next <= previous {
                    return Err(format!(
                        "{label} extension must name exact previous and increased cap"
                    ));
                }
                match label {
                    "round" => old.budget.max_rounds = next,
                    "visual" => old.budget.max_visual_passes = Some(next),
                    "image" => old.budget.max_images = next,
                    _ => old.budget.max_evaluations = next,
                }
            }
        }
        if old.preset != config.preset
            || [
                old.budget.max_tokens,
                old.budget.max_images,
                old.budget.max_rounds,
                old.budget.max_evaluations,
            ] != [
                config.budget.max_tokens,
                config.budget.max_images,
                config.budget.max_rounds,
                config.budget.max_evaluations,
            ]
        {
            return Err("resume cannot silently change species or budget caps".into());
        }
        if old.budget.max_visual_passes != config.budget.max_visual_passes {
            return Err("resume cannot silently change visual cap".into());
        }
        if let Some(usage) = &decision.external_usage {
            let imported = old
                .authorizations
                .iter()
                .filter_map(|a| a.external_usage.as_ref())
                .flat_map(|u| u.ledgers.iter().map(|e| e.id.clone()))
                .collect();
            old.budget.tokens =
                usage.verify(old.budget.tokens, old.budget.max_tokens, &imported)?;
        }
        if let Some(amendment) = &decision.baseline_amendment {
            if decision.preserve_evidence
                || amendment.previous != old.overrides
                || amendment.next != config.initial_overrides
            {
                return Err(
                    "baseline amendment requires exact old/new overlay and fresh evidence".into(),
                );
            }
            let base =
                telperion_core::presets::Preset::from_id(&config.preset).ok_or("unknown preset")?;
            telperion_core::params::overlay(&base.parameters(), &amendment.next)
                .map_err(|e| format!("baseline amendment: {e:?}"))?;
            old.overrides = amendment.next.clone();
        } else if config.initial_overrides
            != *old
                .authorizations
                .iter()
                .rev()
                .find_map(|d| d.baseline_amendment.as_ref().map(|a| &a.next))
                .or_else(|| {
                    old.trials
                        .iter()
                        .find(|t| t.label == "baseline")
                        .map(|t| &t.overrides)
                })
                .unwrap_or(&old.overrides)
        {
            return Err("changed baseline overlay requires explicit amendment".into());
        }
        if let Some(authority) = &decision.experimental_pilot {
            authority.verify(&identity, &old.budget)?;
        }
        if decision.recover_interrupted
            && matches!(
                old.pending.as_deref(),
                Some("baseline" | "candidate evaluation")
            )
        {
            old.pending = None;
        }
        if old.pending.is_some() || !old.usage_known {
            return Err("interrupted or unknown spend must be reconciled before resume".into());
        }
        if decision.preserve_evidence {
            // Computed before this decision joins the chain it would extend.
            let reusable = old.evidence_identities();
            let mut original = config.clone();
            original.budget.max_tokens = previous_cap;
            original.budget.max_rounds = previous_budget.max_rounds;
            original.budget.max_visual_passes = previous_budget.max_visual_passes;
            original.budget.max_images = previous_budget.max_images;
            original.budget.max_evaluations = previous_budget.max_evaluations;
            if original.identity()? != old.identity {
                return Err(
                    "evidence reuse requires unchanged original config and artifact bytes".into(),
                );
            }
            let trial = old
                .current
                .and_then(|i| old.trials.get(i))
                .ok_or("no reusable current trial")?;
            let visual = old.visual.as_ref().ok_or("no reusable visual evidence")?;
            if !trial.feasible
                || visual.identity != trial.key
                || !reusable.iter().any(|k| k == &trial.identity)
                || trial.seed != config.seed
                || visual.model != config.vision.model
                || trial.comparisons.is_empty()
            {
                return Err("stale reusable evidence".into());
            }
            for image in &config.references {
                image.verify()?;
            }
            for anchor in &config.quality_anchors {
                anchor.image.verify()?;
            }
            for c in &trial.comparisons {
                for image in &c.images {
                    image.verify()?;
                }
            }
        }
        old.pause = None;
        old.machine_ready = false;
        if !decision.preserve_evidence {
            old.visual = None;
            old.current = None;
        }
        old.identity = identity.clone();
        old.visual_bootstrap = config.visual_bootstrap;
        old.seed = config.seed;
        old.dials = config.dials.clone();
        old.owner_notes = config.owner_notes.clone();
        old.required = config.required.clone();
        old.accept_priorities(&decision, &config.priority_scope(&old))?;
        let base =
            telperion_core::presets::Preset::from_id(&config.preset).ok_or("unknown preset")?;
        let family = telperion_core::params::overlay(&base.parameters(), &old.overrides)
            .map_err(|e| format!("resumed family: {e:?}"))?;
        old.effective = telperion_core::params::metadata(&family);
        old.authorizations.push(decision);
        old.verify_diagnoses()?;
        old.budget.validate()?;
        old
    } else {
        if resume.is_some() {
            return Err("cannot resume absent state".into());
        }
        let base =
            telperion_core::presets::Preset::from_id(&config.preset).ok_or("unknown preset")?;
        let family = telperion_core::params::overlay(&base.parameters(), &config.initial_overrides)
            .map_err(|e| format!("initial family: {e:?}"))?;
        Run {
            identity: identity.clone(),
            preset: config.preset.clone(),
            seed: config.seed,
            effective: telperion_core::params::metadata(&family),
            overrides: config.initial_overrides.clone(),
            dials: config.dials.clone(),
            owner_notes: config.owner_notes.clone(),
            required: config.required.clone(),
            budget: config.budget.clone(),
            usage_known: true,
            trials: vec![],
            current: None,
            visual: None,
            pause: None,
            machine_ready: false,
            pending: None,
            routes: vec![],
            authorizations: vec![],
            preparation_charge: None,
            priority_checkpoints: vec![],
            handoffs: vec![],
            judgment_inputs: vec![],
            visual_bootstrap: config.visual_bootstrap,
            reviewer_passed_unqualified: false,
        }
    };
    Ok(Prepared::Ready(Box::new(state)))
}
pub fn run(config_path: &Path, out: &Path, resume: Option<&Path>) -> Result<(), String> {
    run_with(config_path, out, resume, &UreqTransport, &|| {
        load_key().map_err(|e| e.to_string())
    })
}

pub fn run_with(
    config_path: &Path,
    out: &Path,
    resume: Option<&Path>,
    transport: &dyn Transport,
    key: KeySource<'_>,
) -> Result<(), String> {
    let config: Config = serde_json::from_slice(&fs::read(config_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if fs::symlink_metadata(out).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("output cannot be a symlink".into());
    }
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let lock = out.join("run.lock");
    let lock_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock)
        .map_err(|e| format!("run locked: {e}"))?;
    lock_file
        .try_lock()
        .map_err(|e| format!("another process owns this run: {e}"))?;
    let path = out.join("run.json");
    let identity = config.identity()?;
    let mut state = match prepare(&config, &path, resume, &identity)? {
        Prepared::Interrupted(old) => {
            write(&path, &serde_json::to_value(&old).unwrap())?;
            return Err(
                "interruption recorded; supply scoped resume decision from run.json".into(),
            );
        }
        Prepared::Ready(state) => *state,
    };
    let mut save = |state: &Run| -> Result<(), String> {
        write(&path, &serde_json::to_value(state).unwrap())?;
        if let Some(checkpoint) = state.priority_checkpoints.last() {
            write(
                &out.join("priority-review.json"),
                &json!({"checkpoint_sha256":checkpoint.hash(),"checkpoint":checkpoint,"approval":state.approved_priorities(),"ordering":"First three eligible findings in reviewer source order, not a new model ranking or owner approval","meaning":"Owner chooses what matters. Approval is neither readiness nor final acceptance; all other findings remain in checkpoint.visual."}),
            )?;
        }
        if !state.current_handoffs().is_empty() {
            write(
                &out.join("handoffs.json"),
                &json!({"meaning":"Evidence-backed gap handoffs for fn-89. An outstanding gap is never machine readiness, and an unauthorized handoff dispatches nothing.",
                "bootstrap":state.visual_bootstrap,
                "unresolved_priorities":state.unresolved_priorities(),"handoffs":state.current_handoffs()}),
            )?;
        }
        write(
            &out.join("finalists.json"),
            &json!({"owner_acceptance":"pending","bootstrap":state.visual_bootstrap,
            "machine_ready":state.machine_ready,
            "reviewer_passed_unqualified":state.reviewer_passed_unqualified,
            "meaning":"reviewer has never been shown to pass an owner-accepted tree",
            "candidates":state.finalists()}),
        )?;
        Ok(())
    };
    save(&state)?;
    if let Err(reason) = config.verify() {
        state.pause=Some(Pause {id:crate::ledger::new_entry_id(),identity:identity.clone(),reason,
            basis:Basis {identity,proposed_action:"supply verified calibration".into(),evidence:vec![],recent_outcomes:vec![],
                next_tokens:None,estimate_basis:String::new(),usage_known:state.usage_known},
            decision_requested:"Supply frozen calibration and a scoped resume decision. No unattended work is authorized.".into()});
        save(&state)?;
        return Err("paused: calibration prerequisite unavailable; see run.json".into());
    }
    if let Err(reason) = state.pilot_authority() {
        state.pause=Some(Pause {id:crate::ledger::new_entry_id(),identity:identity.clone(),reason,
            basis:Basis {identity,proposed_action:"authorize bounded experimental pilot".into(),evidence:vec![],recent_outcomes:vec![],next_tokens:None,estimate_basis:String::new(),usage_known:state.usage_known},
            decision_requested:"Provide explicit scoped experimental authority; general magnitude efficacy remains unvalidated.".into()});
        save(&state)?;
        return Err(
            "magnitude live efficacy unvalidated; scoped experimental authority required".into(),
        );
    }
    if let Some(charge) = config.preparation()? {
        super::reference_first::charge_preparation(
            &mut state.budget,
            &mut state.preparation_charge,
            &charge,
        )?;
        save(&state)?;
    }
    let key = key()?;
    let mut services = Live {
        config: &config,
        transport,
        key: &key,
    };
    state.execute(&mut services, &mut save)?;
    if state.pause.is_some() {
        return Err("paused; see run.json".into());
    }
    if state.visual_bootstrap {
        println!("bootstrap run; no readiness is claimed and an owner look is required");
    } else {
        println!("machine ready; owner acceptance remains pending");
    }
    Ok(())
}
