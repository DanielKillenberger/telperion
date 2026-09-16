mod common;

use serde_json::json;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::cite::{
    cite, list_reason, looks_like_height_at_age, parse_research, ResearchClaim, SourceLoad,
};
use telperion_jev::questions::{citation_cases, severity_level, thresholds, triage_cases};
use telperion_jev::triage::triage;

use common::{ledger_dir, CaseTransport};

#[test]
fn citation_lists_o1_and_passes_the_six_true_claims() {
    let cuts = thresholds();
    let mut passed = 0;
    let mut listed_o1 = false;
    for case in citation_cases() {
        let kind = case.expect_kind.as_deref();
        let confidence = case.expect_confidence.unwrap_or(0.99);
        let (listed, reason) = list_reason(
            &case.expect_relation,
            confidence,
            kind,
            case.height_at_age,
            cuts.citation_auto_accept,
        );
        if case.id == "o1-misuse" {
            assert!(listed, "{reason}");
            listed_o1 = true;
        }
        if case.true_claim {
            assert!(!listed, "{} listed: {reason}", case.id);
            passed += 1;
        }
    }
    assert!(listed_o1);
    assert_eq!(passed, 6);
}

#[test]
fn citation_unreachable_and_no_section() {
    let claims = vec![
        ResearchClaim {
            claim: "A claim about nothing botanical.".into(),
            url: "https://example.invalid/missing".into(),
            source_id: "missing".into(),
        },
        ResearchClaim {
            claim: "Taproot depth on a page about weather only.".into(),
            url: "https://example.test/weather".into(),
            source_id: "weather".into(),
        },
    ];
    let loads = vec![
        SourceLoad::Unreachable("connection refused".into()),
        SourceLoad::Bytes(b"Today the sky is grey and the rain is light.".to_vec()),
    ];
    let report = cite(
        &CaseTransport,
        "k",
        &ledger_dir("cite-err"),
        &claims,
        &loads,
    )
    .unwrap();
    assert_eq!(report.rows[0].relation, "unchecked");
    assert!(report.rows[0].reason.contains("connection refused"));
    assert_eq!(report.rows[1].relation, "says_nothing");
    assert_eq!(report.rows[1].confidence, 0.0);
}

#[test]
fn citation_parses_research_bullets() {
    let md = "- **Oak height** — Norway spruce in the landscape: moderate to fast when young, about 23 m in 50 years. Source: https://example.test/spruce\n";
    let claims = parse_research(md);
    assert_eq!(claims.len(), 1);
    assert!(looks_like_height_at_age(&claims[0].claim));
}

#[test]
fn triage_routes_eight_of_eight_and_ranks_true_pairs() {
    let cases = triage_cases();
    let mut routed = 0;
    for route in &cases.routes {
        let proposal = triage(
            &CaseTransport,
            "k",
            &ledger_dir("route"),
            &route.observation,
            &cases.open_specs,
            &[],
            "",
        )
        .unwrap();
        assert!(
            route.expect.contains(&proposal.spec),
            "{} -> {}",
            route.id,
            proposal.spec
        );
        routed += 1;
    }
    assert_eq!(routed, 8);

    let findings: Vec<String> = cases
        .duplicates
        .iter()
        .map(|item| item.prior_finding.clone())
        .collect();
    let observation = cases.duplicates[0].new_observation.clone();
    let proposal = triage(
        &CaseTransport,
        "k",
        &ledger_dir("dup"),
        &observation,
        &cases.open_specs,
        &findings,
        "",
    )
    .unwrap();
    assert!(proposal.same_defect.unwrap() > 0.5);

    let mut labels = 0;
    for sev in &cases.severity {
        let proposal = triage(
            &CaseTransport,
            "k",
            &ledger_dir("sev"),
            &sev.observation,
            &cases.open_specs,
            &[],
            &sev.standard,
        )
        .unwrap();
        assert_eq!(
            proposal.severity_level.as_deref(),
            Some(sev.expect_level.as_str()),
            "{}",
            sev.id
        );
        labels += 1;
    }
    assert_eq!(labels, 4);
    assert_eq!(severity_level(0.01), "cosmetic");
    assert_eq!(severity_level(1.79), "noticeable");
    assert_eq!(severity_level(2.0), "blocking");
}

#[test]
fn unmatched_observation_returns_new_spec() {
    struct Low;
    impl Transport for Low {
        fn send(&self, _request: &HttpRequest) -> Result<HttpResponse, String> {
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": {
                        "spec": {
                            "type": "choice",
                            "choice": "fn-10",
                            "probabilities": { "fn-10": 0.22, "new_spec": 0.18 },
                            "confidence": 0.1
                        }
                    },
                    "usage": {"input_tokens": 1, "output_tokens": 1}
                }))
                .unwrap(),
            })
        }
    }
    let cases = triage_cases();
    let proposal = triage(
        &Low,
        "k",
        &ledger_dir("new"),
        "The skybox colour is wrong.",
        &cases.open_specs,
        &[],
        "",
    )
    .unwrap();
    assert!(proposal.new_spec);
    assert_eq!(proposal.spec, "new_spec");
    assert!(proposal.spec_probabilities.get("fn-10").is_some());
}

#[test]
fn triage_writes_only_the_ledger() {
    let root = ledger_dir("state");
    let memory = root.join("memory");
    let receipts = root.join("receipts");
    std::fs::create_dir_all(&memory).unwrap();
    std::fs::create_dir_all(&receipts).unwrap();
    let ledger = root.join("ledger");
    let cases = triage_cases();
    let _ = triage(
        &CaseTransport,
        "k",
        &ledger,
        &cases.routes[0].observation,
        &cases.open_specs,
        &[],
        "",
    )
    .unwrap();
    assert!(std::fs::read_dir(&memory).unwrap().next().is_none());
    assert!(std::fs::read_dir(&receipts).unwrap().next().is_none());
    assert!(std::fs::read_dir(&ledger).unwrap().next().is_some());
}
