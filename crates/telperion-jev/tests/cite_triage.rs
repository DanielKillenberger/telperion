mod common;

use serde_json::json;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::cases::run_labelled_cases;
use telperion_jev::cite::{
    cite, load_claim_source, looks_like_height_at_age, parse_research, ResearchClaim, SourceLoad,
};
use telperion_jev::questions::{citation_cases, severity_level, thresholds, triage_cases};
use telperion_jev::triage::triage;

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
    let mut passed = 0;
    let mut listed_o1 = false;
    for (case, row) in cases.iter().zip(report.rows.iter()) {
        if case.id == "o1-misuse" {
            assert!(row.listed, "{}: {}", case.id, row.reason);
            listed_o1 = true;
        }
        if case.true_claim {
            assert!(!row.listed, "{} listed: {}", case.id, row.reason);
            passed += 1;
        }
        if case.height_at_age {
            assert!(row.kind.is_some(), "{} skipped the compose screen", case.id);
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
