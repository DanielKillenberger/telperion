mod common;

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::cases::run_labelled_cases;
use telperion_jev::cite::{
    carries_number, cite, format_report, load_claim_source, looks_like_height_at_age,
    parse_research, research_markdown, ResearchClaim, SourceLoad,
};
use telperion_jev::questions::{citation_cases, severity_level, thresholds, triage_cases};
use telperion_jev::triage::{format_proposal, triage};

use common::{ledger_dir, CaseTransport};

#[test]
fn citation_lists_o1_and_passes_the_six_true_claims() {
    let cases = citation_cases();
    assert_eq!(cases.len(), 9);
    let claims: Vec<ResearchClaim> = cases
        .iter()
        .map(|case| ResearchClaim {
            claim: case.claim.clone(),
            url: format!("file:{}", case.id),
            source_id: case.id.clone(),
            unresolved: None,
        })
        .collect();
    let loads: Vec<SourceLoad> = cases
        .iter()
        .map(|case| SourceLoad::Bytes(case.section.as_bytes().to_vec()))
        .collect();
    let report = cite(
        &CaseTransport,
        "k",
        &ledger_dir("cite-nine"),
        &claims,
        &loads,
    )
    .unwrap();
    assert_eq!(report.rows.len(), 9);
    let mut hits = 0;
    for (case, row) in cases.iter().zip(report.rows.iter()) {
        if carries_number(&case.claim) {
            assert!(
                row.kind.is_some(),
                "{} is numeric but skipped the compose screen",
                case.id
            );
        }
        let expect_listed = !case.true_claim;
        assert_eq!(
            row.listed, expect_listed,
            "{} listed={} reason={}",
            case.id, row.listed, row.reason
        );
        if carries_number(&case.claim) {
            assert!(
                !row.screen_ledger.is_empty(),
                "{} numeric claim missing screen ledger",
                case.id
            );
            assert_ne!(row.ledger, row.screen_ledger, "{}", case.id);
        }
        if case.id == "o1-as-typical" {
            assert!(row.listed);
            assert_eq!(row.kind.as_deref(), Some("site_quality_criterion"));
        }
        if case.id == "spruce-five-years" {
            assert_eq!(row.relation, "contradicts");
        }
        if case.id == "oak-sprouts" || case.id == "oak-usual-rate" {
            assert!(!row.listed, "{} must still pass: {}", case.id, row.reason);
        }
        if expect_listed == row.listed {
            hits += 1;
        }
    }
    assert_eq!(hits, 9);
}

#[test]
fn citation_unreachable_and_no_section() {
    let claims = vec![
        ResearchClaim {
            claim: "A claim about nothing botanical.".into(),
            url: "https://example.invalid/missing".into(),
            source_id: "missing".into(),
            unresolved: None,
        },
        ResearchClaim {
            claim: "Taproot depth on a page about weather only.".into(),
            url: "https://example.test/weather".into(),
            source_id: "weather".into(),
            unresolved: None,
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
fn low_confidence_measured_size_is_listed_with_both_ledgers() {
    struct LowKind;
    impl Transport for LowKind {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
                .map_err(|err| err.to_string())?;
            let answers = if body["questions"].get("kind").is_some() {
                json!({
                    "kind": {
                        "type": "choice",
                        "choice": "measured_size_at_age",
                        "probabilities": { "measured_size_at_age": 0.31 },
                        "confidence": 0.31
                    },
                    "condition": {
                        "type": "choice",
                        "choice": "unstated",
                        "probabilities": { "unstated": 0.9 },
                        "confidence": 0.8
                    },
                    "anchor_usable": { "type": "noul", "noul": 0.8 }
                })
            } else {
                json!({
                    "relation": {
                        "type": "choice",
                        "choice": "supports",
                        "probabilities": { "supports": 0.99 },
                        "confidence": 0.99
                    }
                })
            };
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": answers,
                    "usage": {"input_tokens": 1, "output_tokens": 1}
                }))
                .unwrap(),
            })
        }
    }
    let claims = [ResearchClaim {
        claim: "Norway spruce reaches 23 m in 50 years.".into(),
        url: "file:x".into(),
        source_id: "x".into(),
        unresolved: None,
    }];
    let loads = [SourceLoad::Bytes(
        b"Norway spruce may grow to 75 feet in 50 years.".to_vec(),
    )];
    let report = cite(&LowKind, "k", &ledger_dir("low-kind"), &claims, &loads).unwrap();
    assert!(report.rows[0].listed);
    assert!(
        report.rows[0].reason.contains("screen kind confidence"),
        "{}",
        report.rows[0].reason
    );
    assert!(!report.rows[0].ledger.is_empty());
    assert!(!report.rows[0].screen_ledger.is_empty());
    assert_ne!(report.rows[0].ledger, report.rows[0].screen_ledger);
    let printed = format_report(&report);
    assert!(printed.contains("cite="), "{printed}");
    assert!(printed.contains("screen="), "{printed}");
}

#[test]
fn whole_spec_uses_only_the_research_section() {
    let spec = "\
## Acceptance Criteria\n\
- **R1:** Every bullet here is not a research claim. Source: https://example.test/not-research\n\
## Resolved via Research\n\
- **Iowa** — about 23 m in 50 years. Source: https://example.test/spruce\n\
## Boundaries\n\
- Jev never runs in the generator. Source: https://example.test/boundary\n\
";
    let section = research_markdown(spec);
    assert!(section.contains("23 m"));
    assert!(!section.contains("**R1:**"));
    assert!(!section.contains("Jev never runs"));
    let claims = parse_research(section);
    assert_eq!(claims.len(), 1);
    assert!(claims[0].claim.contains("23 m"));
    let only = "- **Iowa** — about 23 m in 50 years. Source: https://example.test/spruce\n";
    assert_eq!(research_markdown(only), only);
    assert_eq!(parse_research(research_markdown(only)).len(), 1);
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

    let mut true_ps = Vec::new();
    let mut false_ps = Vec::new();
    for pair in &cases.duplicates {
        let proposal = triage(
            &CaseTransport,
            "k",
            &ledger_dir("dup-rank"),
            &pair.new_observation,
            &cases.open_specs,
            std::slice::from_ref(&pair.prior_finding),
            "",
        )
        .unwrap();
        let p = proposal.same_defect.expect(pair.id.as_str());
        assert_eq!(
            proposal.same_defect_match,
            Some(p >= thresholds().duplicate_same_defect),
            "{}",
            pair.id
        );
        if pair.true_pair {
            true_ps.push(p);
        } else {
            false_ps.push(p);
        }
    }
    assert_eq!(true_ps.len(), 2);
    assert_eq!(false_ps.len(), 5);
    assert!(
        true_ps.iter().all(|t| false_ps.iter().all(|f| t > f)),
        "true={true_ps:?} false={false_ps:?}"
    );

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
fn unassessable_observation_is_unassessed() {
    struct Vague;
    impl Transport for Vague {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
                .map_err(|err| err.to_string())?;
            if body["questions"].get("assessable").is_some()
                || body["questions"].get("severity").is_some()
            {
                return Ok(HttpResponse {
                    status: 200,
                    body: serde_json::to_vec(&json!({
                        "model": "jev-latest",
                        "answers": {
                            "assessable": { "type": "noul", "noul": 0.08 },
                            "severity": {
                                "type": "score",
                                "score": 1.5,
                                "confidence": 0.4,
                                "probabilities": { "1": 0.6 }
                            }
                        },
                        "usage": {"input_tokens": 1, "output_tokens": 1}
                    }))
                    .unwrap(),
                });
            }
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({
                    "model": "jev-latest",
                    "answers": {
                        "spec": {
                            "type": "choice",
                            "choice": "fn-31",
                            "probabilities": { "fn-31": 0.86, "new_spec": 0.04 },
                            "confidence": 0.8
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
        &Vague,
        "k",
        &ledger_dir("unassessed"),
        "",
        &cases.open_specs,
        &[],
        "The owner's recorded standard.",
    )
    .unwrap();
    assert_eq!(proposal.severity_level.as_deref(), Some("unassessed"));
    assert!(proposal.severity.is_none());
    let printed = format_proposal(&proposal);
    assert!(printed.contains("unassessed"), "{printed}");
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

#[test]
fn parse_research_keeps_url_less_bullets_and_reuses_same_paper() {
    let md = "\
## Resolved via Research\n\
- **Palubicki** — one iteration is one season. Source: https://algorithmicbotany.org/papers/selforg.sig2009.html\n\
- **Vigour** — resource splits by λ. Source: same paper\n\
- **Shedding** — light over size. Source: same paper, section on branch shedding\n\
- **Weber and Penn 1995** has no time dimension. Source: the paper, and openalea/weberpenn below\n\
- **No single equation** gives per-year increments. Source: the scout's synthesis\n";
    let claims = parse_research(md);
    assert_eq!(claims.len(), 5, "{claims:?}");
    assert_eq!(
        claims[0].url,
        "https://algorithmicbotany.org/papers/selforg.sig2009.html"
    );
    assert!(claims[0].unresolved.is_none());
    assert_eq!(claims[1].url, claims[0].url);
    assert!(claims[1].unresolved.is_none(), "{:?}", claims[1]);
    assert_eq!(claims[2].url, claims[0].url);
    assert!(claims[2].unresolved.is_none());
    assert!(claims[3].url.is_empty());
    assert!(
        claims[3]
            .unresolved
            .as_deref()
            .unwrap()
            .contains("openalea/weberpenn"),
        "{:?}",
        claims[3].unresolved
    );
    assert!(claims[4].url.is_empty());
    assert!(
        claims[4]
            .unresolved
            .as_deref()
            .unwrap()
            .contains("scout's synthesis"),
        "{:?}",
        claims[4].unresolved
    );

    let load = load_claim_source(&CaseTransport, &claims[3]);
    match load {
        SourceLoad::Unreachable(err) => assert!(err.contains("openalea/weberpenn"), "{err}"),
        other => panic!("{other:?}"),
    }
    let report = cite(
        &CaseTransport,
        "k",
        &ledger_dir("unresolved"),
        std::slice::from_ref(&claims[4]),
        &[load_claim_source(&CaseTransport, &claims[4])],
    )
    .unwrap();
    assert_eq!(report.rows[0].relation, "unchecked");
    assert!(report.rows[0].reason.contains("scout's synthesis"));

    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let fn11 = std::fs::read_to_string(repo.join(".flow/specs/fn-11-growth-over-time.md")).unwrap();
    let research = fn11.split("## Resolved via Research").nth(1).unwrap();
    let parsed = parse_research(research);
    let same = parsed
        .iter()
        .find(|claim| claim.claim.contains("resource splits"))
        .expect("same-paper vigour bullet");
    assert_eq!(
        same.url,
        "https://algorithmicbotany.org/papers/selforg.sig2009.html"
    );
    let synthesis = parsed
        .iter()
        .find(|claim| claim.claim.contains("No single equation"))
        .expect("synthesis bullet kept");
    assert!(synthesis.url.is_empty());
    assert!(synthesis
        .unresolved
        .as_deref()
        .unwrap()
        .contains("scout's synthesis"));
}

#[test]
fn labelled_cases_meet_pilot_offline() {
    let sets = run_labelled_cases(&CaseTransport, "k", &ledger_dir("cases")).unwrap();
    for set in &sets {
        assert!(
            set.meets_pilot(),
            "{} {}/{} ranking={:?}",
            set.name,
            set.hits,
            set.required,
            set.ranking_ok
        );
    }
}
