//! How far a round moves: the size of the gap the owner or the reviewer names.
//!
//! On the palm's first tuning revision the reviewer said every round that the
//! fronds were far too short, and the frond length moved 3.5 to 3.75 m in
//! seven rounds, because a round's move was a fixed ladder of each dial's
//! small step whatever the words said. Now code offers a class for the gap,
//! Jev or the owner selects one, and code maps it to a multiplier on the
//! configured ladder. Jev never supplies the number.
mod decide;
pub use decide::lead;
pub(in crate::tuning) use decide::{decide, settle, Decision};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

pub const QUESTION: &str = "gap_magnitude";
pub const VERSION: &str = "gap-magnitude-v1";
/// The label the question carries wherever it is recorded.
pub const LABEL: &str = "gap magnitude question";
/// The question as sent, and the labelled cases it is trusted against.
pub const QUESTIONS_JSON: &str = include_str!("../../data/questions/gap_magnitude.json");
pub const CASES_JSON: &str = include_str!("../../data/cases/gap_magnitude.json");

/// The size of the gap the words name, smallest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Class {
    Near,
    ClearlyOff,
    FarOff,
}

impl Class {
    /// What code multiplies the configured ladder by. Code's number, never Jev's.
    pub fn multiplier(self) -> f64 {
        match self {
            Class::Near => 1.,
            Class::ClearlyOff => 2.,
            Class::FarOff => 4.,
        }
    }
    /// One level down; near is the floor.
    pub fn lower(self) -> Class {
        match self {
            Class::FarOff => Class::ClearlyOff,
            _ => Class::Near,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Class::Near => "near",
            Class::ClearlyOff => "clearly_off",
            Class::FarOff => "far_off",
        }
    }
    /// A class from a criterion name; `no_match` and anything else is none.
    pub fn parse(name: &str) -> Option<Class> {
        [Class::Near, Class::ClearlyOff, Class::FarOff]
            .into_iter()
            .find(|c| c.name() == name)
    }
}

/// Where one track stands between rounds.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Standing {
    /// The highest class allowed until a bundle on this track is adopted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap: Option<Class>,
    /// The class the track's last adopted bundle was drawn at.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adopted: Option<Class>,
}

/// One answer to the gap question, as the service read it.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Judged {
    pub choice: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f64>,
    pub threshold: f64,
}

pub fn questions() -> Value {
    serde_json::from_str(QUESTIONS_JSON).expect("gap_magnitude.json")
}

/// What Jev is shown: words only, the shape the labelled cases carry.
pub fn shown(priority: &str, finding: &str) -> Value {
    json!({"priority":priority,"finding":finding})
}

/// The labelled set: its cases and the confidence cut they set.
#[derive(Debug, Clone, Deserialize)]
pub struct Labelled {
    pub min_confidence: f64,
    pub cases: Vec<Value>,
}

/// The labelled set as authored; its cut is the one a run acts at.
pub fn labelled() -> Labelled {
    serde_json::from_str(CASES_JSON).expect("cases/gap_magnitude.json")
}
