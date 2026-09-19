//! Rounds are bounded (R6).
//!
//! A verdict that is not yet accepting allows `rounds_per_verdict` value
//! rounds, two as the table ships, before the loop names a gap or files for
//! the owner. The rounds live in `DIR/rounds.json`, one entry per verdict,
//! and a round past the bound is refused and files a `value-rounds` decision
//! the owner resolves. The loop counts rounds; it judges no still.

use serde_json::{json, Value};

use super::table::load as table;
use super::{now, GapError, STAGE};
use crate::pipeline::canon::{read_json, write_canonical};
use crate::pipeline::decision::{append_decisions, Decision, DecisionParts};
use crate::pipeline::stage::Paths;

pub const ROUNDS_SCHEMA_VERSION: u32 = 1;
/// What the owner picks from when a verdict has spent its rounds.
pub const SPENT_OPTIONS: [&str; 3] = ["name-gap", "more-rounds", "accept"];

pub fn rounds_path(paths: &Paths) -> std::path::PathBuf {
    paths.dir.join("rounds.json")
}

/// The record, empty on first use.
pub fn read(paths: &Paths) -> Result<Value, GapError> {
    let path = rounds_path(paths);
    if !path.exists() {
        return Ok(json!({
            "schema": "rounds",
            "schema_version": ROUNDS_SCHEMA_VERSION,
            "verdicts": [],
        }));
    }
    Ok(read_json(&path)?)
}

pub fn write(paths: &Paths, record: &Value) -> Result<(), GapError> {
    write_canonical(&rounds_path(paths), record)?;
    Ok(())
}

fn entry_index(record: &Value, verdict: &str) -> Option<usize> {
    record["verdicts"]
        .as_array()?
        .iter()
        .position(|e| e["verdict"] == verdict)
}

/// What opening a round decided.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Opened {
    /// The round is open; this is its number, one-based.
    Round(usize),
    /// The bound is spent: the decision filed for the owner.
    Spent { round: usize, decision: String },
}

/// Opens one value round on `verdict`. The round past the table's bound is
/// refused and files for the owner; `note` is what the round would have
/// changed, recorded either way.
pub fn open_round(
    paths: &Paths,
    species: &str,
    verdict: &str,
    note: &str,
) -> Result<Opened, GapError> {
    if verdict.trim().is_empty() {
        return Err(GapError::Invalid("the verdict id is empty".into()));
    }
    let limit = table().rounds_per_verdict;
    let mut record = read(paths)?;
    let known = entry_index(&record, verdict);
    let list = record["verdicts"]
        .as_array_mut()
        .ok_or_else(|| GapError::Invalid("rounds.json carries no verdict list".into()))?;
    let index = known.unwrap_or_else(|| {
        list.push(json!({
            "verdict": verdict,
            "species": species,
            "rounds": [],
            "accepted": Value::Null,
            "spent": Value::Null,
        }));
        list.len() - 1
    });
    if !list[index]["accepted"].is_null() {
        return Err(GapError::Invalid(format!(
            "{verdict} is already accepting; it takes no further round"
        )));
    }
    let taken = list[index]["rounds"].as_array().map_or(0, Vec::len);
    let round = taken + 1;
    if taken >= limit {
        let decision = spent(species, verdict, round, limit);
        let id = decision.id.clone();
        list[index]["spent"] = json!({"at": now(), "round": round, "decision": id, "note": note});
        append_decisions(&paths.decisions(), vec![decision])?;
        write(paths, &record)?;
        return Ok(Opened::Spent {
            round,
            decision: id,
        });
    }
    list[index]["rounds"]
        .as_array_mut()
        .expect("rounds is a list")
        .push(json!({"round": round, "at": now(), "note": note}));
    write(paths, &record)?;
    Ok(Opened::Round(round))
}

/// Records that `verdict` now accepts, at the round it took.
pub fn accept(paths: &Paths, verdict: &str) -> Result<usize, GapError> {
    let mut record = read(paths)?;
    let Some(index) = entry_index(&record, verdict) else {
        return Err(GapError::Invalid(format!(
            "no value round is recorded for {verdict}"
        )));
    };
    let list = record["verdicts"]
        .as_array_mut()
        .expect("verdicts is a list");
    let taken = list[index]["rounds"].as_array().map_or(0, Vec::len);
    list[index]["accepted"] = json!({"at": now(), "round": taken});
    write(paths, &record)?;
    Ok(taken)
}

/// Verdict -> the round it accepted at, for the metrics record.
pub fn to_acceptance(paths: &Paths) -> Result<Vec<(String, usize)>, GapError> {
    let record = read(paths)?;
    Ok(record["verdicts"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let round = entry["accepted"]["round"].as_u64()?;
            Some((entry["verdict"].as_str()?.to_string(), round as usize))
        })
        .collect())
}

/// Every value round recorded, accepting or not.
pub fn total(paths: &Paths) -> Result<usize, GapError> {
    let record = read(paths)?;
    Ok(record["verdicts"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|entry| entry["rounds"].as_array().map_or(0, Vec::len))
        .sum())
}

fn spent(species: &str, verdict: &str, round: usize, limit: usize) -> Decision {
    Decision::new(
        DecisionParts {
            species,
            stage: STAGE,
            kind: "value-rounds",
            field: Some(verdict),
            age_years: None,
        },
        &[],
        [("verdict".to_string(), verdict.to_string())].into(),
        vec![],
        json!({"verdict": verdict, "round": round, "limit": limit}),
        &SPENT_OPTIONS,
        "The verdict spent its value rounds without accepting; the owner decides whether the \
         loop names a gap, takes another round, or the verdict accepts as it stands.",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pipeline::decision::read_decisions;

    fn scratch() -> Paths {
        let dir = std::env::temp_dir().join(format!(
            "jev-rounds-{}-{}",
            std::process::id(),
            crate::ledger::new_entry_id()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        Paths::new(&dir)
    }

    #[test]
    fn two_rounds_are_allowed_and_the_third_is_refused_for_the_owner() {
        let paths = scratch();
        assert_eq!(
            open_round(&paths, "silver-birch", "crown/whole", "widen the crown").unwrap(),
            Opened::Round(1)
        );
        assert_eq!(
            open_round(&paths, "silver-birch", "crown/whole", "more droop").unwrap(),
            Opened::Round(2)
        );
        let third = open_round(&paths, "silver-birch", "crown/whole", "again").unwrap();
        let Opened::Spent { round, decision } = third else {
            panic!("the third round is refused: {third:?}");
        };
        assert_eq!(round, 3);
        assert_eq!(decision, "silver-birch/gap/value-rounds/crown/whole");
        let filed = read_decisions(&paths.decisions()).unwrap();
        assert_eq!(filed.len(), 1);
        assert_eq!(filed[0].options, SPENT_OPTIONS);
        assert_eq!(filed[0].payload["limit"], 2);
        // The refused round is recorded, never counted as a round taken.
        assert_eq!(total(&paths).unwrap(), 2);
    }

    #[test]
    fn a_second_verdict_keeps_its_own_count_and_acceptance_is_recorded_per_verdict() {
        let paths = scratch();
        open_round(&paths, "silver-birch", "crown/whole", "").unwrap();
        assert_eq!(
            open_round(&paths, "silver-birch", "bark/base", "").unwrap(),
            Opened::Round(1)
        );
        assert_eq!(accept(&paths, "crown/whole").unwrap(), 1);
        assert_eq!(
            to_acceptance(&paths).unwrap(),
            vec![("crown/whole".into(), 1)]
        );
        assert!(open_round(&paths, "silver-birch", "crown/whole", "")
            .unwrap_err()
            .to_string()
            .contains("already accepting"));
        assert!(accept(&paths, "never-seen").is_err());
        assert!(open_round(&paths, "silver-birch", "  ", "").is_err());
    }
}
