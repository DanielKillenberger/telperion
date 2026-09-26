//! One tuning revision into its own directory: the config read and checked,
//! a fresh record, the rounds run, and after every save `run.json` and
//! `result.json`. A revision is never resumed: the runner starts the next one
//! from the last kept tree (`runner::tune`).
use super::{
    engine::Run,
    live::{Config, Live},
};
use crate::caller::Transport;
use serde_json::Value;
use std::{fs, io::Write, path::Path};

/// Supplies the API key at dispatch time, after the config has been checked.
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

/// The record a revision opens on: the config's overlay applied to its
/// preset, and nothing spent.
pub fn fresh(config: &Config, identity: &str) -> Result<Run, String> {
    let base = telperion_core::presets::Preset::from_id(&config.preset).ok_or("unknown preset")?;
    let family = telperion_core::params::overlay(&base.parameters(), &config.initial_overrides)
        .map_err(|e| format!("initial family: {e:?}"))?;
    Ok(Run {
        identity: identity.into(),
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
        stopped: None,
        machine_ready: false,
        pending: None,
        routes: vec![],
        approval: None,
        preparation_charge: None,
        priority_checkpoints: vec![],
        judgment_inputs: vec![],
        visual_bootstrap: config.visual_bootstrap,
        reviewer_passed_unqualified: false,
        strides: Default::default(),
        unkept: None,
    })
}

/// Runs one revision from the config at `config_path` into `out`, which it
/// must not have run into before.
pub fn run_with(
    config_path: &Path,
    out: &Path,
    transport: &dyn Transport,
    key: KeySource<'_>,
) -> Result<(), String> {
    let config: Config = serde_json::from_slice(&fs::read(config_path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if fs::symlink_metadata(out).is_ok_and(|m| m.file_type().is_symlink()) {
        return Err("output cannot be a symlink".into());
    }
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let lock_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(false)
        .open(out.join("run.lock"))
        .map_err(|e| format!("run locked: {e}"))?;
    lock_file
        .try_lock()
        .map_err(|e| format!("another process owns this run: {e}"))?;
    let path = out.join("run.json");
    if path.exists() {
        return Err(format!(
            "{} already holds a revision; a revision runs once",
            out.display()
        ));
    }
    config.verify()?;
    let mut state = fresh(&config, &config.identity()?)?;
    let mut save = |state: &Run| -> Result<(), String> {
        write(&path, &serde_json::to_value(state).unwrap())?;
        let mut result = state.end_result();
        result.known_gaps = config.unexpressed.clone();
        write(
            &out.join("result.json"),
            &serde_json::to_value(&result).unwrap(),
        )
    };
    save(&state)?;
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
    state.execute(&mut services, &mut save)
}
