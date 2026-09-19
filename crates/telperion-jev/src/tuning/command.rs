use super::{
    continuation::{Basis, HumanDecision, Pause},
    engine::Run,
    live::{Config, Live},
};
use crate::caller::{load_key, UreqTransport};
use serde_json::{json, Value};
use std::{fs, io::Write, path::Path};

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

pub fn run(config_path: &Path, out: &Path, resume: Option<&Path>) -> Result<(), String> {
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
    let mut state: Run = if path.exists() {
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
            write(&path, &serde_json::to_value(&old).unwrap())?;
            return Err(
                "interruption recorded; supply scoped resume decision from run.json".into(),
            );
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
        if old.identity != identity && decision.next_identity.as_deref() != Some(&identity) {
            return Err("changed inputs require a scoped decision naming next_identity".into());
        }
        let previous_cap = old.budget.max_tokens;
        if let Some(extension) = &decision.token_cap_extension {
            if extension.previous != previous_cap
                || extension.next != config.budget.max_tokens
                || extension.next <= extension.previous
            {
                return Err("token extension must name exact previous and increased cap".into());
            }
            old.budget.max_tokens = extension.next;
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
            let mut original = config.clone();
            original.budget.max_tokens = previous_cap;
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
                || trial.identity != old.identity
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
        old.seed = config.seed;
        old.dials = config.dials.clone();
        old.owner_notes = config.owner_notes.clone();
        old.required = config.required.clone();
        let base =
            telperion_core::presets::Preset::from_id(&config.preset).ok_or("unknown preset")?;
        let family = telperion_core::params::overlay(&base.parameters(), &old.overrides)
            .map_err(|e| format!("resumed family: {e:?}"))?;
        old.effective = telperion_core::params::metadata(&family);
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
        }
    };
    let mut save = |state: &Run| -> Result<(), String> {
        write(&path, &serde_json::to_value(state).unwrap())?;
        write(
            &out.join("finalists.json"),
            &json!({"owner_acceptance":"pending",
            "machine_ready":state.machine_ready,"candidates":state.finalists()}),
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
    let key = load_key().map_err(|e| e.to_string())?;
    let mut services = Live {
        config: &config,
        transport: &UreqTransport,
        key: &key,
    };
    state.execute(&mut services, &mut save)?;
    if state.pause.is_some() {
        return Err("paused; see run.json".into());
    }
    println!("machine ready; owner acceptance remains pending");
    Ok(())
}
