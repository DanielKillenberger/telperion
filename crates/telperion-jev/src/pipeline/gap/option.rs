//! Options are written, not judged into being (R1).
//!
//! An option is one candidate fix in a fixed shape: what changes, whether it
//! is a value-table, generator or appearance change, what it touches, whether
//! it moves a pin or changes any preset's output, which species it serves and
//! whether it is reversible. The three owner flags beyond the contract's
//! fields, a capture budget spent, a data-quality bar lowered and a Boundary
//! changed, default to false. A set holds two to four options; an empty set
//! from the agent goes to the stronger model and an empty set from both
//! files for the owner.

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::table::Route;
use super::{now, GapError};

pub const MIN_OPTIONS: usize = 2;
pub const MAX_OPTIONS: usize = 4;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChangeKind {
    ValueTable,
    Generator,
    Appearance,
}

impl ChangeKind {
    pub fn key(self) -> &'static str {
        match self {
            Self::ValueTable => "value_table",
            Self::Generator => "generator",
            Self::Appearance => "appearance",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Author {
    Agent,
    Stronger,
}

impl Author {
    pub fn key(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Stronger => "stronger",
        }
    }

    pub fn parse(word: &str) -> Option<Self> {
        match word {
            "agent" => Some(Self::Agent),
            "stronger" => Some(Self::Stronger),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GapOption {
    pub gap: String,
    pub option: String,
    pub change_kind: ChangeKind,
    pub touches: String,
    pub moves_pin: bool,
    pub changes_preset_output: Vec<String>,
    pub serves_species: Vec<String>,
    pub reversible: bool,
    pub summary: String,
    #[serde(default)]
    pub spends_captures: bool,
    #[serde(default)]
    pub lowers_bar: bool,
    #[serde(default)]
    pub changes_boundary: bool,
}

/// The shape an option set file carries: `{"options": [...]}` or a bare list.
pub fn parse_options(value: &Value) -> Result<Vec<GapOption>, GapError> {
    let list = value
        .get("options")
        .cloned()
        .unwrap_or_else(|| value.clone());
    serde_json::from_value(list)
        .map_err(|err| GapError::Invalid(format!("option set: {err}; the shape is fixed")))
}

/// Two to four options for `gap`, each with a unique id, a touch and a
/// summary. An empty list is valid and routes by itself.
pub fn validate(gap: &str, options: &[GapOption]) -> Result<(), GapError> {
    if options.is_empty() {
        return Ok(());
    }
    if options.len() < MIN_OPTIONS || options.len() > MAX_OPTIONS {
        return Err(GapError::Invalid(format!(
            "{} options; a set holds {MIN_OPTIONS} to {MAX_OPTIONS}",
            options.len()
        )));
    }
    for (index, option) in options.iter().enumerate() {
        if option.gap != gap {
            return Err(GapError::Invalid(format!(
                "option {} is written for {}, not {gap}",
                option.option, option.gap
            )));
        }
        if option.option.trim().is_empty()
            || option.touches.trim().is_empty()
            || option.summary.trim().is_empty()
        {
            return Err(GapError::Invalid(format!(
                "option {index} leaves its id, touch or summary empty"
            )));
        }
        if options[..index].iter().any(|o| o.option == option.option) {
            return Err(GapError::Invalid(format!(
                "option {} is listed twice",
                option.option
            )));
        }
    }
    Ok(())
}

/// What writing a set decided by itself, before any question is asked.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Written {
    /// A full set, ready to route.
    Ready,
    /// An empty set from the agent: the gap goes to the stronger model.
    EmptyToStronger,
    /// An empty set from the stronger model too: the gap files for the owner.
    EmptyToOwner,
}

/// Appends the set to the record. A set from the stronger model needs an
/// agent set before it, empty or not: the route sends it there, it never
/// starts there.
pub fn add_set(
    record: &mut Value,
    author: Author,
    model: &str,
    options: Vec<GapOption>,
) -> Result<Written, GapError> {
    let gap = record["gap"].as_str().unwrap_or_default().to_string();
    validate(&gap, &options)?;
    let sets = record["sets"].as_array().cloned().unwrap_or_default();
    let last = sets.last().and_then(|s| s["author"].as_str());
    match (author, last) {
        (Author::Stronger, None) => {
            return Err(GapError::Invalid(
                "the stronger model writes a set only after the agent's; write the agent's first"
                    .into(),
            ))
        }
        (Author::Agent, Some(_)) => {
            return Err(GapError::Invalid(
                "the agent's set is already written; a second set comes from the stronger model"
                    .into(),
            ))
        }
        (Author::Stronger, Some("stronger")) => {
            return Err(GapError::Invalid(
                "the stronger model's set is already written".into(),
            ))
        }
        _ => {}
    }
    let written = match (options.is_empty(), author) {
        (false, _) => Written::Ready,
        (true, Author::Agent) => Written::EmptyToStronger,
        (true, Author::Stronger) => Written::EmptyToOwner,
    };
    let at = now();
    record["sets"]
        .as_array_mut()
        .expect("sets is a list")
        .push(json!({
            "author": author.key(),
            "model": model,
            "at": at,
            "options": options,
        }));
    if written != Written::Ready {
        let route = match written {
            Written::EmptyToStronger => Route::Stronger,
            _ => Route::Owner,
        };
        record["routes"]
            .as_array_mut()
            .expect("routes is a list")
            .push(json!({
                "author": author.key(),
                "at": at,
                "options": [],
                "judgments": {},
                "signals": {},
                "route": route.key(),
                "row": Value::Null,
                "why": format!("empty option set from the {}", author.key()),
                "table_version": super::table::load().version,
                "ledger": [],
                "chosen": Value::Null,
            }));
        record["route"] = json!(route.key());
    }
    Ok(written)
}

/// The latest set's options and author.
pub fn latest_set(record: &Value) -> Result<(Author, String, Vec<GapOption>), GapError> {
    let Some(set) = record["sets"].as_array().and_then(|s| s.last()) else {
        return Err(GapError::Invalid("no option set is written yet".into()));
    };
    let author = set["author"]
        .as_str()
        .and_then(Author::parse)
        .ok_or_else(|| GapError::Invalid("the latest set names no author".into()))?;
    let model = set["model"].as_str().unwrap_or_default().to_string();
    let options: Vec<GapOption> = serde_json::from_value(set["options"].clone())
        .map_err(|err| GapError::Invalid(format!("the latest set: {err}")))?;
    Ok((author, model, options))
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) fn option(gap: &str, id: &str, kind: ChangeKind) -> GapOption {
        GapOption {
            gap: gap.into(),
            option: id.into(),
            change_kind: kind,
            touches: "shoot placement".into(),
            moves_pin: false,
            changes_preset_output: vec![],
            serves_species: vec!["silver-birch".into()],
            reversible: true,
            summary: "Rows of short shoots hang under a gravity term.".into(),
            spends_captures: false,
            lowers_bar: false,
            changes_boundary: false,
        }
    }

    fn record() -> Value {
        json!({"gap": "g", "sets": [], "routes": [], "route": null})
    }

    #[test]
    fn a_set_holds_two_to_four_options_for_its_own_gap() {
        let one = vec![option("g", "a", ChangeKind::Generator)];
        assert!(validate("g", &one)
            .unwrap_err()
            .to_string()
            .contains("2 to 4"));
        let five: Vec<GapOption> = (0..5)
            .map(|i| option("g", &format!("o{i}"), ChangeKind::Generator))
            .collect();
        assert!(validate("g", &five).is_err());
        let other = vec![
            option("g", "a", ChangeKind::Generator),
            option("h", "b", ChangeKind::Generator),
        ];
        assert!(validate("g", &other)
            .unwrap_err()
            .to_string()
            .contains("written for h"));
        let twice = vec![
            option("g", "a", ChangeKind::Generator),
            option("g", "a", ChangeKind::ValueTable),
        ];
        assert!(validate("g", &twice)
            .unwrap_err()
            .to_string()
            .contains("twice"));
        assert!(validate("g", &[]).is_ok());
    }

    #[test]
    fn an_empty_agent_set_goes_to_the_stronger_model_and_an_empty_stronger_set_to_the_owner() {
        let mut record = record();
        assert_eq!(
            add_set(&mut record, Author::Agent, "m", vec![]).unwrap(),
            Written::EmptyToStronger
        );
        assert_eq!(record["route"], "stronger");
        assert_eq!(
            add_set(&mut record, Author::Stronger, "m", vec![]).unwrap(),
            Written::EmptyToOwner
        );
        assert_eq!(record["route"], "owner");
        assert_eq!(record["routes"].as_array().unwrap().len(), 2);
        assert!(add_set(&mut record, Author::Stronger, "m", vec![]).is_err());
    }

    #[test]
    fn the_stronger_model_never_writes_first_and_the_agent_never_twice() {
        let mut record = record();
        assert!(add_set(&mut record, Author::Stronger, "m", vec![]).is_err());
        let pair = vec![
            option("g", "a", ChangeKind::Generator),
            option("g", "b", ChangeKind::ValueTable),
        ];
        assert_eq!(
            add_set(&mut record, Author::Agent, "m", pair.clone()).unwrap(),
            Written::Ready
        );
        assert!(add_set(&mut record, Author::Agent, "m", pair).is_err());
        let (author, _, options) = latest_set(&record).unwrap();
        assert_eq!(author, Author::Agent);
        assert_eq!(options.len(), 2);
    }

    #[test]
    fn the_owner_flags_default_to_false_when_the_file_omits_them() {
        let value = json!({"options": [{
            "gap": "g", "option": "a", "change_kind": "generator", "touches": "t",
            "moves_pin": true, "changes_preset_output": ["norway-spruce"],
            "serves_species": [], "reversible": false, "summary": "s"
        }]});
        let options = parse_options(&value).unwrap();
        assert!(!options[0].spends_captures && !options[0].lowers_bar);
        assert!(options[0].moves_pin);
        assert!(parse_options(&json!({"options": [{"option": "a"}]})).is_err());
    }
}
