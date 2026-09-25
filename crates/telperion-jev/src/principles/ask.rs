//! The two question sets. Phase one asks, for every candidate in one batch,
//! which breach mechanism the change introduces and which excerpt shows it;
//! each offers `none` and `insufficient_evidence`. Phase two asks only about
//! the candidates phase one selected above their cut, stating the clause,
//! its legitimate readings and the registered exceptions.

use serde_json::{json, Map, Value};

use super::candidates::{Candidate, Class};
use super::policy::{Exception, Policy};

/// Bumped whenever a question's wording or options change.
pub const QUESTIONS_VERSION: &str = "questions-2";

pub const NONE: &str = "none";
pub const INSUFFICIENT: &str = "insufficient_evidence";

/// Mechanisms a class can select, each with the principle it breaches.
pub fn mechanisms(class: Class) -> &'static [(&'static str, &'static str, &'static str)] {
    const SWITCH: &[(&str, &str, &str)] = &[
        ("selects_builder", "P-NO-SWITCH", "A setting picks between different ways of building: one value runs one builder, another value runs a different one."),
        ("suppresses_structure", "P-NO-SWITCH", "Crossing a value of a setting removes or hides structure the tree otherwise has, instead of shaping it by degree."),
        ("redundant_stop", "P-BLOCKING-STEP", "The branch stops the work for a defect that a neighbouring step already catches."),
    ];
    const DUPLICATE: &[(&str, &str, &str)] = &[
        ("surviving_duplicate", "P-ONE-PIPELINE", "A second implementation of the same responsibility now exists beside the first, and both survive."),
        ("fallback_path", "P-NO-FALLBACK", "An input the main path cannot represent is routed to another path instead of failing with an explicit error."),
        ("redundant_stop", "P-BLOCKING-STEP", "A new step stops the work for a defect that a neighbouring step already catches."),
    ];
    const UNREAD: &[(&str, &str, &str)] = &[
        ("unread_output", "P-CONSUMER-READS", "The change generates or uploads an output that no consumer shown in the evidence reads."),
    ];
    const PROPOSAL: &[(&str, &str, &str)] = &[
        SWITCH[0], SWITCH[1], DUPLICATE[0], DUPLICATE[1], DUPLICATE[2], UNREAD[0],
    ];
    match class {
        Class::Switch => SWITCH,
        Class::Duplicate => DUPLICATE,
        Class::Unread => UNREAD,
        Class::Proposal => PROPOSAL,
    }
}

pub fn principle_of(class: Class, mechanism: &str) -> Option<&'static str> {
    mechanisms(class).iter().find(|m| m.0 == mechanism).map(|m| m.1)
}

fn clean_reading(class: Class) -> &'static str {
    match class {
        Class::Switch => "No breach: dormancy (a term that adds nothing while its structure is absent and grows from zero), a count step of one unit, validation that rejects input with an error, a backend or schedule choice that yields the same tree, or ordinary control flow.",
        Class::Duplicate => "No breach: code extracted into one shared function both callers use, a call delegated to the pipeline, two functions with different responsibilities that merely look alike, necessary validation, or an escalation the rules require.",
        Class::Unread => "No breach: a consumer in the evidence reads the output, or it is the contract a test or measurement in the change reads.",
        Class::Proposal => "No breach: the proposal keeps one path, fails unsupported input with an error, lets a term lie dormant, reads what a consumer needs, or removes a copy rather than adding one.",
    }
}

fn state_candidate(c: &Candidate) -> Value {
    let evidence: Vec<Value> = c
        .evidence
        .iter()
        .map(|e| json!({"id": e.id, "side": e.side, "at": format!("{}:{}-{}", e.path, e.lines.0, e.lines.1), "text": e.text}))
        .collect();
    json!({"class": c.class, "location": c.location, "shape": c.shape, "evidence": evidence})
}

fn clauses(policy: &Policy, candidates: &[&Candidate]) -> Value {
    let mut out = Map::new();
    for p in &policy.principles {
        if candidates.iter().any(|c| c.class.principles().contains(&p.id.as_str())) {
            out.insert(p.id.clone(), json!(p.clause));
        }
    }
    Value::Object(out)
}

/// One batched request: every candidate's mechanism and evidence question.
pub fn phase_one(policy: &Policy, candidates: &[&Candidate]) -> (Value, Value) {
    let mut questions = Map::new();
    let mut state = Map::new();
    for c in candidates {
        state.insert(c.id.clone(), state_candidate(c));
        let mut criteria = Map::new();
        for (name, _, text) in mechanisms(c.class) {
            criteria.insert((*name).into(), json!(text));
        }
        criteria.insert(NONE.into(), json!(clean_reading(c.class)));
        criteria.insert(INSUFFICIENT.into(), json!("The excerpts do not show enough to tell either way."));
        questions.insert(
            format!("{}_mechanism", c.id),
            json!({"type": "choice", "instructions": format!("Read `candidates.{id}`: what code found in a pushed change, with excerpts from before and after it. Against the clauses in `principles`, which breach does the change introduce there, if any?", id = c.id), "criteria": criteria}),
        );
        let mut ids = Map::new();
        for e in &c.evidence {
            ids.insert(e.id.clone(), json!(format!("{} {}:{}-{}", e.side, e.path, e.lines.0, e.lines.1)));
        }
        ids.insert(NONE.into(), json!("No excerpt shows a breach."));
        questions.insert(
            format!("{}_evidence", c.id),
            json!({"type": "choice", "instructions": format!("Which excerpt of `candidates.{}` shows the breach you would name? none when no excerpt shows one.", c.id), "criteria": ids}),
        );
    }
    let state = json!({"principles": clauses(policy, candidates), "candidates": state});
    (state, Value::Object(questions))
}

/// The confirming request for the selected candidates.
pub fn phase_two(policy: &Policy, exceptions: &[Exception], picked: &[(&Candidate, &str, &str)]) -> (Value, Value) {
    let mut questions = Map::new();
    let mut state = Map::new();
    for (c, mechanism, cited) in picked {
        let principle = principle_of(c.class, mechanism).unwrap_or("");
        let p = policy.principle(principle);
        let mut own = (*c).clone();
        own.evidence.retain(|e| e.id == *cited || e.side != "after");
        let mut entry = state_candidate(&own);
        entry["alleged"] = json!(mechanisms(c.class).iter().find(|m| m.0 == *mechanism).map(|m| m.2));
        entry["clause"] = json!(p.map(|p| p.clause.clone()));
        entry["legitimate_readings"] = json!(p.map(|p| p.allowances.clone()).unwrap_or_default());
        state.insert(c.id.clone(), entry);
        questions.insert(
            format!("{}_shown", c.id),
            json!({"type": "noul", "instructions": format!("Do the excerpts of `candidates.{id}` show the change doing what its `alleged` says? Judge only what the excerpts show.", id = c.id), "criteria": {"true": "An excerpt shows the change doing what `alleged` says.", "false": "No excerpt shows it, or the excerpts show something else."}}),
        );
        questions.insert(
            format!("{}_covered", c.id),
            json!({"type": "noul", "instructions": format!("Does one of the `legitimate_readings` of `candidates.{id}`, or one of the registered `exceptions`, describe this change?", id = c.id), "criteria": {"true": "A legitimate reading or a registered exception describes this change.", "false": "None of them describes this change."}}),
        );
    }
    let exceptions: Vec<Value> = exceptions
        .iter()
        .map(|e| json!({"id": e.id, "principle": e.principle, "callers": e.callers, "rationale": e.rationale, "source": e.source}))
        .collect();
    (json!({"exceptions": exceptions, "candidates": state}), Value::Object(questions))
}
