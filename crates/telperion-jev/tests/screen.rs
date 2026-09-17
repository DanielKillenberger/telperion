mod common;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::extract::STATE_SENTENCE_LIMIT;
use telperion_jev::ledger::SourceRef;
use telperion_jev::questions::{screen_cases, selection_cases};
use telperion_jev::screen::{format_report, screen};
use telperion_jev::select::select;
use telperion_jev::sha256_hex;

use common::{ledger_dir, CaseTransport};

#[test]
fn screen_scores_twelve_of_twelve_and_o1_is_a_site_criterion() {
    let cases = screen_cases();
    assert_eq!(cases.len(), 12);
    let mut hits = 0;
    let mut o1_kind = None;
    for case in &cases {
        let document = if case.sentence.ends_with('.') {
            case.sentence.clone()
        } else {
            format!("{}.", case.sentence)
        };
        let bytes = document.as_bytes();
        let source = SourceRef {
            id: case.source_id.clone(),
            url: "file:pilot".into(),
            sha256: sha256_hex(bytes),
            bytes: bytes.len() as u64,
        };
        let report = screen(
            &CaseTransport,
            "k",
            &ledger_dir("screen"),
            &source,
            bytes,
            "oak",
        )
        .unwrap();
        assert_eq!(report.rows.len(), 1, "{}", case.id);
        assert_eq!(report.rows[0].kind, case.expect_kind, "{}", case.id);
        if case.id == "o1" {
            o1_kind = Some(report.rows[0].kind.clone());
        }
        hits += 1;
    }
    assert_eq!(hits, 12);
    assert_eq!(o1_kind.as_deref(), Some("site_quality_criterion"));
}

#[test]
fn empty_source_prints_checksum_and_no_rows() {
    let bytes = b"Nothing measured.";
    let source = SourceRef {
        id: "empty".into(),
        url: String::new(),
        sha256: sha256_hex(bytes),
        bytes: bytes.len() as u64,
    };
    let report = screen(
        &CaseTransport,
        "k",
        &ledger_dir("empty"),
        &source,
        bytes,
        "oak",
    )
    .unwrap();
    assert!(report.rows.is_empty());
    let printed = format_report(&report);
    assert!(printed.contains(&source.sha256));
    assert!(printed.contains("no candidate sentence"));
}

#[test]
fn selection_scores_five_of_five() {
    let mut hits = 0;
    for case in selection_cases() {
        let report = select(
            &CaseTransport,
            "k",
            &ledger_dir("sel"),
            &case.document,
            &case.question,
            None,
        )
        .unwrap();
        assert_eq!(report.chosen, case.expect_span, "{}", case.id);
        hits += 1;
    }
    assert_eq!(hits, 5);
}

#[test]
fn spread_none_prints_the_candidate_list() {
    struct Spread;
    impl Transport for Spread {
        fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, String> {
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": {
                        "span": {
                            "type": "choice",
                            "choice": "none",
                            "probabilities": { "none": 0.49, "8 to 11 years": 0.46 },
                            "confidence": 0.49
                        }
                    },
                    "usage": {"input_tokens": 1, "output_tokens": 1}
                }))
                .unwrap(),
            })
        }
    }
    let document = "8 to 11 years are required to grow a 6-7 foot tree.";
    let report = select(
        &Spread,
        "k",
        &ledger_dir("spread"),
        document,
        "Which candidate span is the height?",
        None,
    )
    .unwrap();
    assert!(report.spread);
    let printed = telperion_jev::select::format_report(&report);
    assert!(printed.contains("6-7 foot"), "{printed}");
    assert!(printed.contains("candidates="), "{printed}");
}

#[test]
fn forged_span_is_a_contract_failure() {
    struct Forge;
    impl Transport for Forge {
        fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, String> {
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": {
                        "span": {
                            "type": "choice",
                            "choice": "9.99 m",
                            "probabilities": { "9.99 m": 0.91 },
                            "confidence": 0.9
                        }
                    },
                    "usage": {"input_tokens": 1, "output_tokens": 1}
                }))
                .unwrap(),
            })
        }
    }
    let document = "8 to 11 years are required to grow a 6-7 foot tree.";
    let report = select(
        &Forge,
        "k",
        &ledger_dir("forge"),
        document,
        "Which candidate span is the height?",
        None,
    )
    .unwrap();
    assert!(report.contract_failure);
    assert!(report.chosen.is_empty());
    let printed = telperion_jev::select::format_report(&report);
    assert!(printed.contains("contract failure"), "{printed}");
    assert!(printed.contains("candidates="), "{printed}");
    assert!(!printed.contains("chosen=9.99"), "{printed}");
    assert!(printed.contains("chosen=<rejected>"), "{printed}");
}

#[test]
fn questions_always_offer_a_no_match() {
    let screen = telperion_jev::screen_questions();
    assert!(screen["kind"]["criteria"]
        .get("not_about_tree_size")
        .is_some());
    assert!(screen["condition"]["criteria"].get("unstated").is_some());
    let cite = telperion_jev::citation_questions();
    assert!(cite["relation"]["criteria"].get("says_nothing").is_some());
    let select = telperion_jev::questions::selection_questions("q", &["1 ft".into()]);
    assert!(select["span"]["criteria"].get("none").is_some());
    let severity = telperion_jev::questions::severity_questions();
    assert!(severity["assessable"]["criteria"].get("false").is_some());
}

#[test]
fn screen_state_fields_stay_within_the_limit() {
    use std::sync::Mutex;
    struct Capture {
        states: Mutex<Vec<Value>>,
    }
    impl Transport for Capture {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
                .map_err(|err| err.to_string())?;
            self.states.lock().unwrap().push(body["state"].clone());
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": {
                        "kind": {
                            "type": "choice",
                            "choice": "not_about_tree_size",
                            "probabilities": { "not_about_tree_size": 0.9 },
                            "confidence": 0.9
                        },
                        "condition": {
                            "type": "choice",
                            "choice": "unstated",
                            "probabilities": { "unstated": 0.9 },
                            "confidence": 0.8
                        },
                        "anchor_usable": { "type": "noul", "noul": 0.05 }
                    },
                    "usage": {"input_tokens": 1, "output_tokens": 1}
                }))
                .unwrap(),
            })
        }
    }
    let mut sentence = String::from("Measured 12 ft ");
    while sentence.len() <= STATE_SENTENCE_LIMIT {
        sentence.push_str("and another increment ");
    }
    sentence.push('.');
    let bytes = sentence.as_bytes();
    let source = SourceRef {
        id: "long".into(),
        url: String::new(),
        sha256: sha256_hex(bytes),
        bytes: bytes.len() as u64,
    };
    let cap = Capture {
        states: Mutex::new(Vec::new()),
    };
    let report = screen(&cap, "k", &ledger_dir("long-state"), &source, bytes, "oak").unwrap();
    assert!(report.rows.len() > 1);
    fn walk(value: &Value, limit: usize) {
        match value {
            Value::String(text) => {
                assert!(
                    text.len() <= limit,
                    "state string is {} bytes, limit {limit}",
                    text.len()
                );
            }
            Value::Array(items) => items.iter().for_each(|item| walk(item, limit)),
            Value::Object(map) => map.values().for_each(|item| walk(item, limit)),
            _ => {}
        }
    }
    let states = cap.states.lock().unwrap();
    assert!(!states.is_empty());
    for state in states.iter() {
        walk(state, STATE_SENTENCE_LIMIT);
        assert!(state["candidate"].get("whole").is_none());
    }
}
