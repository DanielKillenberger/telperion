//! fn-80, owner 2026-09-24: a tidiness violation goes back to the reviewer
//! once, then code trims and records what is left; a trust violation still
//! refuses. The adapter is a fixture script: no Jev, no model.
use serde_json::{json, Value};
use std::path::{Path, PathBuf};
use telperion_jev::{
    sha256_hex,
    tuning::{
        joint::{Finding, Impact, Packet},
        reference_first::*,
        state::{ready, CellStatus},
        vision,
    },
};

fn scratch() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "receipt-repair-{}",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn request(dir: &Path) -> ComparisonRequest {
    let p = dir.join("reference.png");
    std::fs::write(&p, b"reference fixture").unwrap();
    let image = json!({"path":p,"sha256":sha256_hex(b"reference fixture"),"view":"whole","seed":1});
    let mut r: vision::Request = serde_json::from_value(json!({"schema":"tuning-vision-v3",
        "identity":"candidate","target_species":"Example species",
        "required":[{"item":"reference_character","view":"whole","seed":1}],
        "images":[image.clone()],"references":[image.clone()],
        "quality_anchors":[{"image":image,"provenance":"anchor","scope":"finish"}],
        "checklist":"checklist"}))
    .unwrap();
    r.joint = Some(Packet::from_request(&r));
    ComparisonRequest::new(&r, inventory(&r))
}

fn inventory(r: &vision::Request) -> Inventory {
    let request = ReferenceRequest::from_comparison(r);
    Inventory {
        request_sha256: request.hash(),
        prompt_sha256: request.prompt_hash(),
        request,
        model: "mock".into(),
        effort: "medium".into(),
        ledger: "immutable-receipt".into(),
        traits: vec![Trait {
            id: "trait-1".into(),
            priority: Priority::Core,
            observation: "Visible reference-defining trait".into(),
            reference_ids: vec!["reference-0".into()],
            uncertain: false,
        }],
        observations: vec![],
    }
}

fn findings(n: usize) -> Value {
    (0..n)
        .map(|i| {
            json!({"observation":format!("Supported match {}", i + 1),
            "evidence_ids":["render-0","reference-0"],"impact":"supported","uncertain":false,
            "causal_hypothesis":null,"trait_id":null})
        })
        .collect()
}

fn answer(findings: Value) -> Value {
    json!({"passes":["pass"],"defects":[],"observations":[],"findings":findings,
        "coverage":[{"trait_id":"trait-1","status":"pass","evidence_ids":["render-0","reference-0"],
            "explanation":"visible match"}]})
}

const SCRIPT: &str = r#"import json,pathlib,sys
e=json.load(sys.stdin)
answers=json.loads(pathlib.Path(sys.argv[1]).read_text())
with open(sys.argv[2],'a') as log:
    log.write(json.dumps({'stage':e['stage'],'violations':e.get('violations'),'answer':e.get('answer')})+'\n')
print(json.dumps({'status':'ok','model':'mock','effort':'medium','request_sha256':e['request_sha256'],
    'prompt_sha256':e['prompt_sha256'],'usage':{'input_tokens':11 if e['stage']=='comparison' else 5,'output_tokens':3},
    'answer':answers[e['stage']]}))
"#;

/// A fixture adapter answering each stage from `answers`; returns it and the
/// path of the log it appends one line per call to.
fn adapter(dir: &Path, answers: Value) -> (vision::Adapter, PathBuf) {
    let script = dir.join("adapter.py");
    let answers_path = dir.join("answers.json");
    let log = dir.join("calls.jsonl");
    std::fs::write(&script, SCRIPT).unwrap();
    std::fs::write(&answers_path, serde_json::to_vec(&answers).unwrap()).unwrap();
    let args = [&script, &answers_path, &log]
        .map(|p| p.to_string_lossy().into_owned())
        .to_vec();
    let adapter = vision::Adapter {
        program: "python3".into(),
        args,
        model: "mock".into(),
        effort: "medium".into(),
        timeout_seconds: 10,
        ledger: dir.join("ledger"),
    };
    (adapter, log)
}

fn calls(log: &Path) -> Vec<Value> {
    std::fs::read_to_string(log)
        .unwrap_or_default()
        .lines()
        .map(|l| serde_json::from_str(l).unwrap())
        .collect()
}

/// The palm's live run stopped on "too many joint findings". The reviewer
/// is told the exact rule and its corrected answer is bound; the repair is
/// charged and recorded, and convergence rebinds the repaired receipt.
#[test]
fn seventeen_findings_are_repaired_by_the_same_reviewer() {
    let dir = scratch();
    let request = request(&dir);
    let (adapter, log) = adapter(
        &dir,
        json!({"comparison":answer(findings(17)),"repair":answer(findings(16))}),
    );
    let result = assess(&adapter, &request).unwrap();
    let calls = calls(&log);
    assert_eq!(calls.len(), 2);
    assert_eq!(calls[1]["stage"], "repair");
    assert_eq!(
        calls[1]["violations"],
        json!(["finding 17 of 17: at most 16 findings allowed"])
    );
    assert_eq!(calls[1]["answer"]["findings"].as_array().unwrap().len(), 17);
    let assessment = &result.visual.assessment;
    assert_eq!(assessment.findings.len(), 16);
    let usage = result.visual.usage.as_ref().unwrap();
    assert_eq!((usage.input_tokens, usage.output_tokens), (16, 6));
    assert!(assessment
        .observations
        .iter()
        .any(|o| o.starts_with("reviewer repaired its answer once for 1 tidiness violation(s)")));
    let record: Value =
        serde_json::from_slice(&std::fs::read(&assessment.ledger).unwrap()).unwrap();
    assert_eq!(record["label"], "repair");
    let r = &request.comparison;
    assert!(ready(&r.required, &r.identity, assessment));
    std::fs::remove_dir_all(dir).unwrap();
}

/// A repair that still breaks the rule is trimmed, first items kept, and
/// every trimmed finding is recorded. There is never a second repair.
#[test]
fn a_repair_that_still_exceeds_is_trimmed_and_recorded_after_one_call() {
    let dir = scratch();
    let request = request(&dir);
    let (adapter, log) = adapter(
        &dir,
        json!({"comparison":answer(findings(17)),"repair":answer(findings(18))}),
    );
    let result = assess(&adapter, &request).unwrap();
    assert_eq!(calls(&log).len(), 2, "one repair at most");
    let assessment = &result.visual.assessment;
    assert_eq!(assessment.findings.len(), 16);
    assert_eq!(assessment.findings[15].observation, "Supported match 16");
    for n in [17, 18] {
        let note =
            format!("dropped finding {n} of 18 (at most 16 findings allowed): Supported match {n}");
        assert!(assessment.observations.contains(&note), "{note}");
    }
    std::fs::remove_dir_all(dir).unwrap();
}

/// An invented evidence id is a trust violation: refused, and never repaired.
#[test]
fn an_invented_evidence_id_is_refused_without_a_repair() {
    let dir = scratch();
    let request = request(&dir);
    let mut bad = findings(17);
    bad[0]["evidence_ids"] = json!(["render-0", "reference-9"]);
    let (adapter, log) = adapter(
        &dir,
        json!({"comparison":answer(bad),"repair":answer(findings(16))}),
    );
    let refused = assess(&adapter, &request).unwrap_err();
    assert_eq!(refused, "invalid joint finding evidence");
    assert_eq!(calls(&log).len(), 1);
    std::fs::remove_dir_all(dir).unwrap();
}

/// A repair that moves a verdict it was only asked to tidy is set aside: the
/// first answer stands, trimmed and recorded, and both calls are charged.
#[test]
fn a_repair_that_changes_a_verdict_leaves_the_first_answer_trimmed() {
    let dir = scratch();
    let request = request(&dir);
    let mut flipped = answer(findings(16));
    flipped["passes"] = json!(["fail"]);
    let (adapter, log) = adapter(
        &dir,
        json!({"comparison":answer(findings(17)),"repair":flipped}),
    );
    let result = assess(&adapter, &request).unwrap();
    assert_eq!(calls(&log).len(), 2);
    let assessment = &result.visual.assessment;
    assert_eq!(assessment.cells[0].1, CellStatus::Pass);
    assert_eq!(assessment.findings.len(), 16);
    assert!(
        assessment
            .observations
            .iter()
            .any(|o| o
                == "dropped finding 17 of 17 (at most 16 findings allowed): Supported match 17")
    );
    let usage = result.visual.usage.as_ref().unwrap();
    assert_eq!((usage.input_tokens, usage.output_tokens), (16, 6));
    std::fs::remove_dir_all(dir).unwrap();
}

/// The live stop itself: sixteen reviewer findings plus the code-derived
/// coverage gate's own. The gate finding stands; the reviewer's sixteenth
/// makes room and is recorded.
#[test]
fn the_coverage_gate_finding_never_overflows_the_cap() {
    let dir = scratch();
    let request = request(&dir);
    let r = &request.comparison;
    let found: Vec<Finding> = serde_json::from_value(findings(16)).unwrap();
    let visual: vision::Result = serde_json::from_value(json!({"request_sha256":r.hash(),
        "assessment":{"identity":r.identity,"model":"mock","ledger":"receipt",
            "cells":[[r.required[0],"pass"]],"defects":[],"findings":found},
        "effort":"medium","usage":{"input_tokens":1,"output_tokens":1},"observations":[]}))
    .unwrap();
    let mut result = ComparisonResult {
        request_sha256: request.hash(),
        visual,
        coverage: vec![Coverage {
            trait_id: "trait-1".into(),
            status: CellStatus::Fail,
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            explanation: "defining mismatch".into(),
        }],
    };
    let untidy = result.bind_tidy(&request).unwrap();
    let findings = &result.visual.assessment.findings;
    assert_eq!(findings.len(), 16);
    assert_eq!(findings[15].impact, Impact::Blocker);
    assert!(findings[15]
        .observation
        .starts_with("Code-derived reference-first coverage gate"));
    assert_eq!(untidy.len(), 1);
    assert!(untidy[0]
        .violation
        .starts_with("finding 16 of 16: at most 15 findings allowed"));
    assert!(!ready(&r.required, &r.identity, &result.visual.assessment));
    let once = serde_json::to_value(&result).unwrap();
    result.bind(&request).unwrap();
    assert_eq!(
        once,
        serde_json::to_value(&result).unwrap(),
        "rebinding is stable"
    );
    std::fs::remove_dir_all(dir).unwrap();
}

/// The plain vision receipt has no repair path; it trims and records.
#[test]
fn the_plain_vision_receipt_trims_over_cap_findings() {
    let dir = scratch();
    let request = request(&dir).comparison;
    let found: Vec<Finding> = serde_json::from_value(findings(17)).unwrap();
    let mut result: vision::Result =
        serde_json::from_value(json!({"request_sha256":request.hash(),
        "assessment":{"identity":request.identity,"model":"mock","ledger":"receipt",
            "cells":[[request.required[0],"pass"]],"defects":[],"findings":found},
        "effort":"medium","usage":null,"observations":[]}))
        .unwrap();
    result.bind(&request).unwrap();
    assert_eq!(result.assessment.findings.len(), 16);
    assert!(
        result
            .assessment
            .observations
            .iter()
            .any(|o| o
                == "dropped finding 17 of 17 (at most 16 findings allowed): Supported match 17")
    );
    std::fs::remove_dir_all(dir).unwrap();
}
