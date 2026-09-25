//! fn-151's advisory reviewer, offline: the replay of frozen extractions and
//! recorded answers (R4, R5), the run's failure modes under a mock transport
//! (R6), and spec mode (R7). The key stays unset; nothing calls Jev.

use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::principles::ask::{phase_one, phase_two};
use telperion_jev::principles::candidates::{bound, Candidate, Class, Evidence, Extraction};
use telperion_jev::principles::policy::{exceptions, policy};
use telperion_jev::principles::replay::{metrics, read, replay_all, Corpus, Verdict};
use telperion_jev::principles::review::{decide, may_advance, select, Mode};
use telperion_jev::principles::run::{review, Settings};
use telperion_jev::principles::spec;

fn data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("data/principles")
}

#[test]
fn the_replay_keeps_clean_pushes_clean_and_names_every_positive() {
    let policy = policy();
    let corpus: Corpus = read(&data().join("corpus.json")).unwrap();
    let results = replay_all(&data(), &policy).unwrap();
    assert_eq!(results.len(), 39);
    for r in &results {
        println!("{} {} {} {:?} findings {}", r.id, r.group, r.label, r.mechanisms, r.findings.len());
        let case = corpus.cases.iter().find(|c| c.id == r.id).unwrap();
        if r.label == "clean" {
            assert!(r.findings.is_empty(), "{}: {:?}", r.id, r.findings);
            continue;
        }
        assert_eq!(r.mechanisms.len(), case.mechanisms.len(), "{}", r.id);
        for (id, verdict) in &r.mechanisms {
            let m = case.mechanisms.iter().find(|m| &m.id == id).unwrap();
            if m.owner.starts_with("guard:") {
                assert!(matches!(verdict, Verdict::CaughtByGuard(_)), "{id}: {verdict:?}");
            }
        }
    }
    for group in ["calibration", "holdout"] {
        let m = metrics(&results, &corpus, group);
        println!("{group}: recall {} unsupported {} precision {} clean flagged {}", m.recall, m.unsupported, m.precision, m.clean_flagged);
        assert!(m.recall.hits + m.unsupported <= m.recall.of, "unsupported never counts as caught");
    }
}

#[test]
fn every_principle_starts_in_shadow_and_moves_only_on_measured_precision() {
    for p in &policy().principles {
        assert_eq!(Mode::of(&p.mode), Mode::Shadow, "{}: no Jev finding blocks at launch", p.id);
    }
    assert!(!may_advance(19, 0), "too few reviewed findings");
    assert!(may_advance(19, 1));
    assert!(!may_advance(18, 2), "90% is under 95%");
}

#[test]
fn every_question_offers_a_no_match_answer() {
    let policy = policy();
    let c = candidate("c1", Class::Switch);
    let (_, questions) = phase_one(&policy, &[&c]);
    for q in ["c1_mechanism", "c1_evidence"] {
        assert!(questions[q]["criteria"]["none"].is_string(), "{q}");
    }
    assert!(questions["c1_mechanism"]["criteria"]["insufficient_evidence"].is_string());
    let (_, questions) = phase_two(&policy, &exceptions(), &[(&c, "selects_builder", "e1")]);
    for q in ["c1_shown", "c1_covered"] {
        assert!(questions[q]["criteria"]["false"].is_string(), "{q}");
    }
}

fn candidate(id: &str, class: Class) -> Candidate {
    Candidate {
        id: id.into(),
        class,
        location: "crates/telperion-core/src/x.rs:3".into(),
        shape: "arms run different builders on a setting".into(),
        weight: 3.0,
        evidence: vec![Evidence {
            id: "e1".into(),
            side: "after".into(),
            path: "crates/telperion-core/src/x.rs".into(),
            lines: (3, 5),
            text: "if p.habit == 1 { a() } else { b() }".into(),
            added: true,
        }],
    }
}

/// Answers every question it is asked: a breach, shown and not covered.
struct Answers {
    fail: bool,
}

impl Transport for Answers {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        if self.fail {
            return Err("connection refused".into());
        }
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
        let mut answers = serde_json::Map::new();
        for q in body["questions"].as_object().unwrap().keys() {
            let a = if q.ends_with("_mechanism") {
                json!({"choice": "selects_builder", "probabilities": {"selects_builder": 0.95}})
            } else if q.ends_with("_evidence") {
                json!({"choice": "e1", "probabilities": {"e1": 0.9}})
            } else if q.ends_with("_shown") {
                json!({"noul": 0.9})
            } else {
                json!({"noul": 0.1})
            };
            answers.insert(q.clone(), a);
        }
        let out = json!({"model": "jev-test", "answers": answers, "usage": {"input_tokens": 100, "output_tokens": 10}});
        Ok(HttpResponse { status: 200, body: serde_json::to_vec(&out).unwrap() })
    }
}

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("fn151-{name}-{}", telperion_jev::ledger::new_entry_id()));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn extraction(n: usize) -> Extraction {
    let found = (0..n).map(|i| candidate(&format!("x{i}"), Class::Switch)).collect();
    let (candidates, dropped) = bound(found, &policy());
    Extraction { candidates, dropped, ..Extraction::default() }
}

#[test]
fn a_run_answers_in_two_calls_and_a_cached_run_in_none() {
    let (ledger, cache) = (scratch("ledger"), scratch("cache"));
    let settings = Settings {
        key: &|| Some("mock-key".into()),
        ledger: &ledger,
        cache: Some(&cache),
        record_all: false,
        deadline: Instant::now() + Duration::from_secs(10),
        phase_one: None,
    };
    let x = extraction(2);
    let first = review(&Answers { fail: false }, &settings, &x, &policy(), &exceptions());
    assert_eq!((first.calls, first.outcome.findings.len()), (2, 2));
    assert!(first.outcome.findings.iter().all(|f| f.mode == Mode::Shadow));
    let again = review(&Answers { fail: true }, &settings, &x, &policy(), &exceptions());
    assert_eq!((again.calls, again.outcome.findings.len()), (0, 2), "served from the cache");
    assert!(again.outcome.incomplete.is_none());
}

#[test]
fn a_failed_call_a_passed_deadline_or_an_overflow_is_incomplete() {
    let ledger = scratch("ledger");
    let policy = policy();
    let mut settings = Settings {
        key: &|| Some("mock-key".into()),
        ledger: &ledger,
        cache: None,
        record_all: false,
        deadline: Instant::now() + Duration::from_secs(10),
        phase_one: None,
    };
    let failed = review(&Answers { fail: true }, &settings, &extraction(1), &policy, &exceptions());
    assert!(failed.outcome.incomplete.as_deref().is_some_and(|w| w.contains("connection refused")));
    settings.deadline = Instant::now() - Duration::from_millis(1);
    let late = review(&Answers { fail: false }, &settings, &extraction(1), &policy, &exceptions());
    assert!(late.outcome.incomplete.as_deref().is_some_and(|w| w.contains("deadline")));
    settings.deadline = Instant::now() + Duration::from_secs(10);
    let over = review(&Answers { fail: false }, &settings, &extraction(20), &policy, &exceptions());
    assert!(over.outcome.incomplete.as_deref().is_some_and(|w| w.contains("over the cap")));
}

#[test]
fn a_finding_needs_both_cuts_and_an_excerpt_the_change_added() {
    let policy = policy();
    let mut x = extraction(1);
    let p1 = json!({"c1_mechanism": {"choice": "selects_builder", "probabilities": {"selects_builder": 0.95}}, "c1_evidence": {"choice": "e1"}});
    let (picked, _) = select(&x, &p1, &policy, true);
    let verdict = |shown: f64, covered: f64, x: &Extraction| {
        decide(x, &picked, &json!({"c1_shown": {"noul": shown}, "c1_covered": {"noul": covered}}), &policy).findings.len()
    };
    assert_eq!(verdict(0.9, 0.1, &x), 1);
    assert_eq!(verdict(0.6, 0.1, &x), 0, "not shown above P-NO-SWITCH's cut");
    assert_eq!(verdict(0.9, 0.7, &x), 0, "a legitimate reading covers it");
    x.candidates[0].evidence[0].added = false;
    let (picked, _) = select(&x, &p1, &policy, true);
    let old = decide(&x, &picked, &json!({"c1_shown": {"noul": 0.9}, "c1_covered": {"noul": 0.1}}), &policy);
    assert!(old.findings.is_empty() && old.abstained[0].1.contains("nothing the change added"));
}

#[test]
fn spec_mode_judges_current_proposals_and_abstains_without_context() {
    let policy = policy();
    let dir = data().join("specs");
    let read_spec = |name: &str| std::fs::read_to_string(dir.join(format!("{name}.md"))).unwrap();
    let (x, abstained) = spec::extract("fn-150-final.md", &read_spec("fn-150-final"), &policy);
    assert!(abstained.is_empty());
    assert!(x.candidates.iter().all(|c| !c.shape.contains("ribbon")), "superseded designs are left out");
    assert!(x.candidates.iter().any(|c| c.shape.contains("one oriented box per leaflet")));
    let answers: Value = read(&dir.join("fn-150-final.answers.json")).unwrap();
    let shapes: Vec<&str> = x.candidates.iter().map(|c| c.shape.as_str()).collect();
    assert_eq!(answers["candidates"], json!(shapes), "the answers were recorded on these proposals");
    let (picked, _) = select(&x, &answers["phase_one"], &policy, true);
    assert!(decide(&x, &picked, &answers["phase_two"], &policy).findings.is_empty(), "fn-150's final design");
    let (x, abstained) = spec::extract("no-context.md", &read_spec("no-context"), &policy);
    assert!(x.candidates.is_empty());
    assert_eq!(abstained[0].1, "no decision context");
    let (x, _) = spec::extract("duplicate-path.md", &read_spec("duplicate-path"), &policy);
    let answers: Value = read(&dir.join("duplicate-path.answers.json")).unwrap();
    let shapes: Vec<&str> = x.candidates.iter().map(|c| c.shape.as_str()).collect();
    assert_eq!(answers["candidates"], json!(shapes));
    let (picked, _) = select(&x, &answers["phase_one"], &policy, true);
    assert_eq!(picked[0].mechanism, "surviving_duplicate");
    let outcome = decide(&x, &picked, &answers["phase_two"], &policy);
    println!("duplicate-path: findings {:?} abstained {:?}", outcome.findings, outcome.abstained);
}
