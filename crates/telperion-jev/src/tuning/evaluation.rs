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
    /// The candidate key this round started from. Absent on trials recorded
    /// before the field existed, and treated as unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base: Option<String>,
    /// The adjustment the router proposed, so the same move is not re-bought.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action: Option<super::actions::Action>,
    /// The visual receipt the round acted on, so "new evidence" is decidable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence: Option<String>,
    /// The probability mass behind the accepted direction, and the rule.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction_mass: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    /// Owner-facing telemetry: the other candidates of this round the reviewer
    /// also judged adoptable, when this one was the move that was kept.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub adopted_over: Vec<String>,
    /// The bundle this trial drew, when the round moved every supported dial
    /// together. Absent for a single-dial attempt.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bundle: Option<super::bundle::Bundle>,
    /// The bundle this one is a half of, while a split isolates what breaks.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_bundle: Option<String>,
    /// This variant's row of the contact sheet that judged it, or the reason
    /// code kept it off one. Present only under bundle selection.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sheet: Option<super::sheet::Outcome>,
    /// Present when this move was adopted and the closing all-view review
    /// took it back. The attempt stands as tried; the tree does not.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vetoed: Option<super::veto::Veto>,
    /// True once a round made this attempt the tree the loop stands on,
    /// whether or not the closing review then let it stand.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub adopted: bool,
    /// The one dial a single-dial attempt moved, from where to where. A bundle
    /// attempt's moves live on its bundle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub step: Option<super::bundle::Move>,
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

/// Which gates are not passing, with what they measured and what they wanted.
///
/// `species_measure` writes one row per profile metric as
/// `{status, target, actual, reason}`: the status is `pass`, `fail`,
/// `unassessed` or `contextual`, the target carries the gating `range`, and
/// the actual carries `value` or `min`/`max`. A bundle round that only says
/// "numeric gate failed" tells the router nothing it can act on.
fn failing(checks: &Value) -> String {
    let Some(checks) = checks.as_object() else {
        return "no gate checks were recorded".into();
    };
    let measured = |actual: &Value| {
        actual["value"]
            .as_f64()
            .map(|v| format!("{v}"))
            .or_else(|| {
                actual["min"]
                    .as_f64()
                    .zip(actual["max"].as_f64())
                    .map(|(lo, hi)| format!("{lo} to {hi}"))
            })
            .unwrap_or_else(|| "unmeasured".into())
    };
    let named = checks
        .iter()
        .filter(|(_, c)| c["status"] != "pass" && c["status"] != "contextual")
        .map(|(id, c)| {
            let range = c["target"]["range"]
                .as_array()
                .filter(|a| a.len() == 2)
                .map(|a| format!(" outside {} to {}", a[0], a[1]))
                .unwrap_or_default();
            format!(
                "{id} {}{range} ({})",
                measured(&c["actual"]),
                c["status"].as_str().unwrap_or("unknown")
            )
        })
        .collect::<Vec<_>>();
    if named.is_empty() {
        "no gate is passing".into()
    } else {
        named.join("; ")
    }
}

fn validate_gates(completed: &Value) -> Result<(), String> {
    for flag in ["node_capped", "level_capped", "attraction_capped"] {
        if completed["metrics"]["growth"][flag].as_bool() != Some(false) {
            return Err(format!("truncated or unknown measurement: {flag}"));
        }
    }
    if completed["numeric_status"] != "pass" {
        return Err(format!(
            "numeric gate failed: {}",
            failing(&completed["checks"])
        ));
    }
    let checks = completed["checks"]
        .as_object()
        .ok_or("missing gate checks")?;
    if !checks.values().any(|c| c["status"] == "pass")
        || checks
            .values()
            .any(|c| c["status"] != "pass" && c["status"] != "contextual")
    {
        return Err(format!(
            "failed or unassessed gate: {}",
            failing(&completed["checks"])
        ));
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
        adopted_over: vec![],
        bundle: None,
        parent_bundle: None,
        sheet: None,
        vetoed: None,
        adopted: false,
        step: None,
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
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
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
