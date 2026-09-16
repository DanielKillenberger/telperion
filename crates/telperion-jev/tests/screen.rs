mod common;

use serde_json::json;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
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
}
