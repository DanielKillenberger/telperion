//! What the reviewer answers, and the same answer read as what the candidate
//! did against the tree it came from.
use super::{Request, UNCALIBRATED};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Choice {
    ABetter,
    BBetter,
    Same,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Judgment {
    pub priority_id: String,
    pub verdict: Choice,
}

/// Something one render breaks that the other does not, and which render has
/// it. The reviewer answers in sides, because it is not told which is which.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Regression {
    pub render: String,
    pub text: String,
}

/// What the reviewer answers, in its own terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Answer {
    pub verdicts: Vec<Judgment>,
    pub improved: String,
    pub missing: String,
    pub regressions: Vec<Regression>,
}

/// Which tree a regression note is about, once the sides are mapped back.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum On {
    Candidate,
    Current,
    /// A note from before the sides were recorded. It blocks, because that is
    /// what it meant when it was written.
    Unknown,
}

/// A regression note with the tree it is about. A record written before the
/// sides were attributed is a bare string, and still loads.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Note {
    pub on: On,
    pub text: String,
}

impl<'de> Deserialize<'de> for Note {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum Raw {
            Attributed { on: On, text: String },
            Legacy(String),
        }
        Ok(match Raw::deserialize(deserializer)? {
            Raw::Attributed { on, text } => Note { on, text },
            Raw::Legacy(text) => Note {
                on: On::Unknown,
                text,
            },
        })
    }
}

/// The same answer, read as what the candidate did.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Movement {
    Better,
    Same,
    Worse,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Verdict {
    pub per_priority: BTreeMap<String, Movement>,
    pub improved: String,
    pub missing: String,
    pub regressions: Vec<Note>,
    pub ledger: String,
    pub model: String,
    /// Which side the candidate was shown as, so the mapping is auditable.
    pub candidate_is: String,
    pub uncalibrated: String,
    /// True when code settled this attempt without asking anyone, because the
    /// candidate's render is the current tree's render byte for byte.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub inert: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl Verdict {
    pub fn better(&self) -> usize {
        self.count(Movement::Better)
    }
    pub fn worse(&self) -> usize {
        self.count(Movement::Worse)
    }
    fn count(&self, m: Movement) -> usize {
        self.per_priority.values().filter(|v| **v == m).count()
    }
    /// A candidate is adopted when the reviewer saw it do some good, saw it do
    /// no harm, and named nothing it breaks.
    pub fn adoptable(&self) -> bool {
        !self.inert && self.better() > 0 && self.worse() == 0 && !self.breaks_something()
    }
    /// Notes about the current tree are reasons the candidate is better, not
    /// reasons to refuse it. A note nobody attributed still refuses.
    pub fn breaks_something(&self) -> bool {
        self.regressions.iter().any(|r| r.on != On::Current)
    }
}

/// Strict: every requested priority answered exactly once and nothing else.
/// An answer that does not bind is a failed attempt, not a dropped row.
pub fn bind(
    request: &Request,
    answer: &Answer,
    candidate_is: &str,
    ledger: String,
    model: String,
) -> Result<Verdict, String> {
    let mut per_priority = BTreeMap::new();
    for judgment in &answer.verdicts {
        if !request
            .priorities
            .iter()
            .any(|p| p.id == judgment.priority_id)
        {
            return Err(format!(
                "progress verdict for an unrequested priority {}",
                judgment.priority_id
            ));
        }
        let movement = match (judgment.verdict, candidate_is) {
            (Choice::Same, _) => Movement::Same,
            (Choice::Unknown, _) => Movement::Unknown,
            (Choice::ABetter, "a") | (Choice::BBetter, "b") => Movement::Better,
            _ => Movement::Worse,
        };
        if per_priority
            .insert(judgment.priority_id.clone(), movement)
            .is_some()
        {
            return Err(format!("progress verdict repeats {}", judgment.priority_id));
        }
    }
    if per_priority.len() != request.priorities.len() {
        return Err("progress answer does not cover every priority".into());
    }
    if answer.improved.trim().is_empty() || answer.missing.trim().is_empty() {
        return Err("progress answer lacks what improved or what is missing".into());
    }
    let mut regressions = Vec::new();
    for note in &answer.regressions {
        if note.render != "a" && note.render != "b" {
            return Err(format!(
                "progress regression names no render: {}",
                note.render
            ));
        }
        regressions.push(Note {
            on: if note.render == candidate_is {
                On::Candidate
            } else {
                On::Current
            },
            text: note.text.clone(),
        });
    }
    Ok(Verdict {
        per_priority,
        improved: answer.improved.clone(),
        missing: answer.missing.clone(),
        regressions,
        ledger,
        model,
        candidate_is: candidate_is.into(),
        uncalibrated: UNCALIBRATED.into(),
        inert: false,
        note: None,
    })
}
