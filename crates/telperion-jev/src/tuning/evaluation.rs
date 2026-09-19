use super::state::distance;
use crate::{pipeline::render::Measurer, sha256_hex};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{fs, path::PathBuf, time::Instant};

pub const METRICS: [&str; 5] = [
    "width_over_height",
    "crown_base",
    "occupied",
    "outline_deviation",
    "centre",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Image {
    pub path: PathBuf,
    pub sha256: String,
    pub view: String,
    pub seed: u32,
}

impl Image {
    pub fn verify(&self) -> Result<(), String> {
        let bytes = fs::read(&self.path).map_err(|e| e.to_string())?;
        if bytes.is_empty() || sha256_hex(&bytes) != self.sha256 {
            return Err(format!("missing or changed image {}", self.path.display()));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub reference: String,
    pub reference_weight: f64,
    pub metric_weights: [f64; 5],
    pub target: [f64; 5],
    pub observed: [Option<f64>; 5],
    pub images: Vec<Image>,
}

pub trait MatchedRenderer {
    fn render(
        &self,
        preset: &str,
        seed: u32,
        family: &Value,
        key: &str,
    ) -> Result<Vec<Comparison>, String>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trial {
    pub key: String,
    pub identity: String,
    pub seed: u32,
    pub round: u64,
    pub label: String,
    pub overrides: Value,
    pub ledger: Option<String>,
    pub feasible: bool,
    pub reason: Option<String>,
    pub measurement: Value,
    pub comparisons: Vec<Comparison>,
    pub score: Option<f64>,
    pub seconds: f64,
}

fn completed(receipt: &str) -> Result<Value, String> {
    let events = receipt
        .lines()
        .map(serde_json::from_str::<Value>)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("incomplete measurement receipt: {e}"))?;
    events
        .iter()
        .find(|e| e["event"] == "completed")
        .cloned()
        .ok_or_else(|| "measurement did not complete".into())
}

fn validate_gates(completed: &Value) -> Result<(), String> {
    for flag in ["node_capped", "level_capped", "attraction_capped"] {
        if completed["metrics"]["growth"][flag].as_bool() != Some(false) {
            return Err(format!("truncated or unknown measurement: {flag}"));
        }
    }
    if completed["numeric_status"] != "pass" {
        return Err("numeric gate failed".into());
    }
    let checks = completed["checks"]
        .as_object()
        .ok_or("missing gate checks")?;
    if !checks.values().any(|c| c["status"] == "pass")
        || checks
            .values()
            .any(|c| c["status"] != "pass" && c["status"] != "contextual")
    {
        return Err("failed or unassessed gate".into());
    }
    Ok(())
}

pub fn gates(receipt: &str) -> Result<Value, String> {
    let value = completed(receipt)?;
    validate_gates(&value)?;
    Ok(value)
}

pub fn evaluate(
    measurer: &dyn Measurer,
    renderer: &dyn MatchedRenderer,
    preset: &str,
    identity: &str,
    seed: u32,
    round: u64,
    label: &str,
    overrides: Value,
    ledger: Option<String>,
) -> Trial {
    let start = Instant::now();
    let key = sha256_hex(format!("{identity}:{seed}:{}", overrides).as_bytes());
    let mut trial = Trial {
        key,
        identity: identity.into(),
        seed,
        round,
        label: label.into(),
        overrides,
        ledger,
        feasible: false,
        reason: None,
        measurement: Value::Null,
        comparisons: vec![],
        score: None,
        seconds: 0.,
    };
    let result = (|| {
        let base = telperion_core::presets::Preset::from_id(preset).ok_or("unknown preset")?;
        telperion_core::params::overlay(&base.parameters(), &trial.overrides)
            .map_err(|e| format!("generator refused: {e:?}"))?;
        let measured = measurer
            .measure(preset, seed, &trial.overrides)
            .map_err(|e| e.to_string())?;
        let receipt = fs::read_to_string(&measured.receipt_path).map_err(|e| e.to_string())?;
        trial.measurement = completed(&receipt)?;
        validate_gates(&trial.measurement)?;
        trial.comparisons = renderer.render(preset, seed, &trial.overrides, &trial.key)?;
        if trial.comparisons.is_empty() {
            return Err("no matched references".into());
        }
        let mut terms = vec![];
        for comparison in &trial.comparisons {
            if comparison.images.len() < 2 {
                return Err("missing still or twin".into());
            }
            for image in &comparison.images {
                image.verify()?;
            }
            for i in 0..5 {
                terms.push((
                    comparison.observed[i],
                    comparison.target[i],
                    comparison.reference_weight * comparison.metric_weights[i],
                ));
            }
        }
        trial.score = Some(distance(&terms)?);
        Ok::<_, String>(())
    })();
    match result {
        Ok(()) => trial.feasible = true,
        Err(e) => trial.reason = Some(e),
    }
    trial.seconds = start.elapsed().as_secs_f64();
    trial
}
