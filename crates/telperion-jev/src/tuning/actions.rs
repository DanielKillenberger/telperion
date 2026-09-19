use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const QUESTION_VERSION: &str = "tuning-adjustment-v1";
pub const DIRECTION_VERSION: &str = "tuning-direction-v1";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    SmallDecrease,
    SubstantialDecrease,
    Hold,
    SmallIncrease,
    SubstantialIncrease,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Dial {
    pub id: String,
    /// JSON pointer into the generator's wire object.
    pub path: String,
    pub meaning: String,
    pub min: f64,
    pub max: f64,
    pub integer: bool,
    pub small: f64,
    pub substantial: f64,
}

impl Dial {
    pub fn direction_question(&self, current: f64) -> Result<Value, String> {
        self.value(current, Action::Hold)?;
        let mut criteria = serde_json::Map::new();
        criteria.insert(
            "hold".into(),
            json!("Evidence indicates no change is needed."),
        );
        criteria.insert(
            "insufficient_evidence".into(),
            json!("The observations cannot support a direction."),
        );
        if self.value(current, Action::SmallDecrease).is_ok() {
            criteria.insert(
                "decrease".into(),
                json!("Decrease this dial to address the sourced observation."),
            );
        }
        if self.value(current, Action::SmallIncrease).is_ok() {
            criteria.insert(
                "increase".into(),
                json!("Increase this dial to address the sourced observation."),
            );
        }
        Ok(
            json!({"type":"choice","instructions":{"task":"Select the direction supported by the sourced observations for this dial, without selecting magnitude. Owner observations take precedence.",
            "dial":self.id,"meaning":self.meaning,"current":current,"version":DIRECTION_VERSION},"criteria":criteria}),
        )
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.id.is_empty()
            || self.meaning.is_empty()
            || !self.path.starts_with('/')
            || self.path.split('/').skip(1).any(str::is_empty)
            || [self.min, self.max, self.small, self.substantial]
                .iter()
                .any(|v| !v.is_finite())
            || self.min >= self.max
            || self.small <= 0.
            || self.substantial < self.small
            || (self.integer
                && [self.min, self.max, self.small, self.substantial]
                    .iter()
                    .any(|v| v.fract() != 0. || v.abs() > i32::MAX as f64))
        {
            return Err(format!("invalid authored dial {}", self.id));
        }
        Ok(())
    }

    pub fn value(&self, current: f64, action: Action) -> Result<Option<Value>, String> {
        self.validate()?;
        if !current.is_finite()
            || current < self.min
            || current > self.max
            || (self.integer && current.fract() != 0.)
        {
            return Err(format!("invalid current value for {}", self.id));
        }
        let delta = match action {
            Action::SmallDecrease => -self.small,
            Action::SubstantialDecrease => -self.substantial,
            Action::SmallIncrease => self.small,
            Action::SubstantialIncrease => self.substantial,
            Action::Hold | Action::InsufficientEvidence => return Ok(None),
        };
        let value = current + delta;
        if !value.is_finite() || value < self.min || value > self.max || value == current {
            return Err(format!("unavailable adjustment for {}", self.id));
        }
        Ok(Some(if self.integer {
            json!(value as i64)
        } else {
            json!(value)
        }))
    }

    pub fn options(&self, current: f64) -> Result<Value, String> {
        self.value(current, Action::Hold)?;
        let mut criteria = serde_json::Map::new();
        let mut values = Vec::new();
        for action in [
            Action::SmallDecrease,
            Action::SubstantialDecrease,
            Action::Hold,
            Action::SmallIncrease,
            Action::SubstantialIncrease,
            Action::InsufficientEvidence,
        ] {
            if let Ok(value) = self.value(current, action) {
                if let Some(ref v) = value {
                    if values.contains(v) {
                        continue;
                    }
                    values.push(v.clone());
                }
                let label = serde_json::to_value(action)
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .to_owned();
                let description = match action {
                    Action::Hold => "Evidence indicates no change is needed.".to_owned(),
                    Action::InsufficientEvidence => {
                        "Evidence cannot support an adjustment or its size; request reassessment."
                            .to_owned()
                    }
                    _ => format!(
                        "{}: code changes {} from {} to {} within [{}, {}].",
                        label,
                        self.meaning,
                        current,
                        value.unwrap(),
                        self.min,
                        self.max
                    ),
                };
                criteria.insert(label, json!(description));
            }
        }
        Ok(Value::Object(criteria))
    }

    pub fn question(&self, current: f64) -> Result<Value, String> {
        Ok(json!({"type":"choice", "instructions": {
            "task":"Select one supported adjustment for this dial. Owner observations take precedence over model observations. Prior failures or overshoot constrain magnitude. If a size is unsupported choose insufficient_evidence. Never compare candidate scores or invent values.",
            "dial":self.id,"meaning":self.meaning,"current":current,
            "version":QUESTION_VERSION
        },"criteria":self.options(current)?}))
    }
}

pub fn candidate(
    preset: &str,
    effective: &Value,
    dial: &Dial,
    action: Action,
) -> Result<Value, String> {
    let current = effective
        .pointer(&dial.path)
        .and_then(Value::as_f64)
        .ok_or_else(|| format!("unknown numeric row {}", dial.path))?;
    let value = dial
        .value(current, action)?
        .ok_or("action proposes no candidate")?;
    let mut wire = effective.clone();
    *wire.pointer_mut(&dial.path).ok_or("row disappeared")? = value.clone();
    let base = telperion_core::presets::Preset::from_id(preset).ok_or("unknown preset")?;
    telperion_core::params::overlay(&base.parameters(), &wire)
        .map_err(|e| format!("generator refused candidate: {e:?}"))?;
    let mut partial = value;
    for key in dial
        .path
        .split('/')
        .skip(1)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        partial = json!({key.replace("~1", "/").replace("~0", "~"): partial});
    }
    Ok(partial)
}
