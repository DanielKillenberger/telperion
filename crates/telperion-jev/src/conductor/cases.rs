//! The labelled cases the policy is held to (R5): routing cases record the
//! signals and the route the owner's allocation expects; continuation cases
//! record a basis, a budget and an assessment and expect ok or the pause
//! reason. Both run offline: the policy is code over recorded judgments, so
//! no case spends a call. Held-out cases are reported apart from the rest.
use serde::Deserialize;
use serde_json::Value;

use super::policy;
use crate::tuning::continuation::{self, Assessment, Basis};
use crate::tuning::state::Budget;

pub const CASES_JSON: &str = include_str!("../../data/cases/conductor.json");

#[derive(Debug, Clone, Deserialize)]
pub struct RouteCase {
    pub id: String,
    pub holdout: bool,
    pub note: String,
    pub signals: Value,
    pub expect: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContinuationCase {
    pub id: String,
    pub holdout: bool,
    pub validated: bool,
    pub basis: Basis,
    pub budget: Value,
    pub assessment: Option<Value>,
    pub expect: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Cases {
    pub version: u32,
    pub note: String,
    pub routes: Vec<RouteCase>,
    pub continuation: Vec<ContinuationCase>,
}

pub fn load() -> Cases {
    serde_json::from_str(CASES_JSON).expect("cases/conductor.json parses")
}

#[derive(Debug, Clone, PartialEq)]
pub struct Scored {
    pub family: &'static str,
    pub id: String,
    pub holdout: bool,
    pub expected: String,
    pub selected: String,
    pub agrees: bool,
}

fn budget(value: &Value) -> Budget {
    let get = |key: &str, default: u64| value[key].as_u64().unwrap_or(default);
    Budget {
        evaluations: get("evaluations", 0),
        images: get("images", 0),
        tokens: get("tokens", 0),
        rounds: get("rounds", 0),
        max_evaluations: get("max_evaluations", u64::MAX),
        max_images: get("max_images", u64::MAX),
        max_tokens: get("max_tokens", u64::MAX),
        max_rounds: get("max_rounds", u64::MAX),
        visual_passes: None,
        max_visual_passes: None,
    }
}

fn assessment(case: &ContinuationCase) -> Option<Assessment> {
    let a = case.assessment.as_ref()?;
    Some(Assessment {
        identity: a["identity"]
            .as_str()
            .unwrap_or(&case.basis.identity)
            .to_string(),
        ledger: a["ledger"].as_str().unwrap_or("ledger:case").to_string(),
        tractability: a["tractability"].as_str().unwrap_or_default().to_string(),
        progress: a["progress"].as_str().unwrap_or_default().to_string(),
        risk: a["risk"].as_str().unwrap_or_default().to_string(),
    })
}

/// Scores every case against the shipped policy and contract.
pub fn score(cases: &Cases, table: &policy::Table) -> Vec<Scored> {
    let mut out = Vec::new();
    for case in &cases.routes {
        let decided = table.route(&case.signals);
        out.push(Scored {
            family: "route",
            id: case.id.clone(),
            holdout: case.holdout,
            expected: case.expect.clone(),
            selected: decided.route.clone(),
            agrees: decided.route == case.expect,
        });
    }
    for case in &cases.continuation {
        let assessment = assessment(case);
        let selected = match continuation::assess(
            &case.basis,
            &budget(&case.budget),
            assessment.as_ref(),
            case.validated,
        ) {
            Ok(()) => "ok".to_string(),
            Err(reason) => reason,
        };
        let agrees = if case.expect == "ok" {
            selected == "ok"
        } else {
            selected.contains(&case.expect)
        };
        out.push(Scored {
            family: "continuation",
            id: case.id.clone(),
            holdout: case.holdout,
            expected: case.expect.clone(),
            selected,
            agrees,
        });
    }
    out
}

/// One row per case, then the tally per family, tuned and held out apart.
pub fn table(scored: &[Scored]) -> String {
    let mut s = String::from("| family | case | held out | expected | selected | agrees |\n| --- | --- | --- | --- | --- | --- |\n");
    for r in scored {
        s.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} |\n",
            r.family, r.id, r.holdout, r.expected, r.selected, r.agrees
        ));
    }
    for family in ["route", "continuation"] {
        for holdout in [false, true] {
            let rows: Vec<&Scored> = scored
                .iter()
                .filter(|r| r.family == family && r.holdout == holdout)
                .collect();
            if rows.is_empty() {
                continue;
            }
            let agree = rows.iter().filter(|r| r.agrees).count();
            s.push_str(&format!(
                "\n{family} {}: {agree} of {} agree",
                if holdout { "held out" } else { "tuned" },
                rows.len()
            ));
        }
    }
    s.push('\n');
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_labelled_case_routes_as_the_owner_expects_held_out_included() {
        let cases = load();
        let scored = score(&cases, &policy::load());
        let missed: Vec<&Scored> = scored.iter().filter(|r| !r.agrees).collect();
        assert!(missed.is_empty(), "{}\n{missed:#?}", table(&scored));
        assert!(scored.iter().any(|r| r.holdout && r.family == "route"));
        assert!(scored
            .iter()
            .any(|r| r.holdout && r.family == "continuation"));
        // The six shapes R5 names are all present.
        for id in [
            "routine-progress",
            "design-done-cheap-implementation",
            "still-complex-implementation",
            "high-uncertainty-pauses",
            "unusual-risk-pauses-before-frontier",
            "repeated-no-progress-budget-remaining",
        ] {
            assert!(cases.routes.iter().any(|c| c.id == id), "{id}");
        }
    }
}
