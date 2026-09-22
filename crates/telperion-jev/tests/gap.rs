//! The gap loop end to end (fn-63): a halt, an option set, a route, a spec,
//! a review, a landing, the stages that rerun, and the run's three numbers.
//! Jev answers through a fixture transport; nothing here reaches the network.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::decision::{
    append_decisions, read_decisions, write_decisions, Decision, DecisionParts, Resolution, Status,
};
use telperion_jev::pipeline::gap::option::{parse_options, Author, Written};
use telperion_jev::pipeline::gap::table::Route;
use telperion_jev::pipeline::gap::{self, metrics, option, resume, route as routing};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::{Context, Paths};

/// Answers the gap set from the option ids in the request: the first option
/// wins the best match by `top`, every option is judged a generator change,
/// and no option carries a prior verdict or a judged pin.
struct GapTransport {
    top: f64,
}

impl Transport for GapTransport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}"))
            .map_err(|err| err.to_string())?;
        let ids: Vec<String> = body["state"]["options"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|o| o["option"].as_str().map(str::to_string))
            .collect();
        let mut answers = Map::new();
        for id in &ids {
            answers.insert(
                format!("change_kind:{id}"),
                json!({"type": "choice", "choice": "generator",
                       "probabilities": {"generator": 0.95}, "confidence": 0.9}),
            );
            answers.insert(
                format!("prior_verdict:{id}"),
                json!({"type": "choice", "choice": "none",
                       "probabilities": {"none": 0.95}, "confidence": 0.9}),
            );
            answers.insert(
                format!("generalizes:{id}"),
                json!({"type": "noul", "noul": 0.8}),
            );
            answers.insert(
                format!("moves_pin:{id}"),
                json!({"type": "noul", "noul": 0.1}),
            );
        }
        let rest = (1.0 - self.top) / (ids.len().max(2) - 1) as f64;
        let mut probabilities = Map::new();
        for (index, id) in ids.iter().enumerate() {
            probabilities.insert(id.clone(), json!(if index == 0 { self.top } else { rest }));
        }
        answers.insert(
            "best_match".into(),
            json!({"type": "choice", "choice": ids.first().cloned().unwrap_or_default(),
                   "probabilities": probabilities, "confidence": 0.9}),
        );
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({
                "model": "jev-latest",
                "answers": answers,
                "usage": {"input_tokens": 40, "output_tokens": 9}
            }))
            .unwrap(),
        })
    }
}

fn scratch(tag: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-gap-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn write_manifest(dir: &Path) {
    let manifest = json!({
        "schema": "manifest", "schema_version": 1,
        "species": "silver-birch",
        "taxon": {"scientific_name": "Betula pendula", "common_name": "Silver birch", "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "silver-birch", "profile_id": "silver-birch", "seed": 1,
        "sources": [{"id": "S1", "url": "https://example.test/s1", "title": "Silvics", "rights": "public domain"}],
        "fields": [{"field": "height_m", "condition": "open_grown", "required_ages_years": [10],
                    "bar": "partial", "question": "What height does a tree reach at a stated age?"}],
        "versions": {"question_sets": {"screen": 1}, "tools": {"species-pipeline": "0.1.0"}},
        "model": "jev-latest"
    });
    std::fs::write(
        dir.join("manifest.json"),
        serde_json::to_vec(&manifest).unwrap(),
    )
    .unwrap();
}

/// The halting decision the gate files when a capability is unreachable.
fn file_halt(paths: &Paths, field: &str) -> String {
    let decision = Decision::new(
        DecisionParts {
            species: "silver-birch",
            stage: "gate",
            kind: "onboarding-gate",
            field: Some(field),
            age_years: None,
        },
        &["generate"],
        [("select.json".to_string(), "abc".to_string())].into(),
        vec![],
        json!({"gate": "capability", "detail": "the curtain does not reach the ground"}),
        &["resolve", "waive"],
        "",
    );
    let id = decision.id.clone();
    append_decisions(&paths.decisions(), vec![decision]).unwrap();
    id
}

fn options(gap: &str, moves_pin: bool) -> Vec<option::GapOption> {
    let raw = json!({"options": [
        {"gap": gap, "option": "curtain-rows", "change_kind": "generator",
         "touches": "twig layer", "moves_pin": moves_pin, "changes_preset_output": [],
         "serves_species": ["silver-birch", "weeping-willow"], "reversible": true,
         "summary": "The twig layer gains hang, length and separation as rows."},
        {"gap": gap, "option": "raise-the-constant", "change_kind": "value_table",
         "touches": "twig droop cap", "moves_pin": false, "changes_preset_output": [],
         "serves_species": ["silver-birch"], "reversible": true,
         "summary": "The birch's table pushes its existing secondary rise further below zero."}
    ]});
    parse_options(&raw).unwrap()
}

fn judge<'a>(transport: &'a GapTransport, paths: &Paths) -> Judge<'a> {
    Judge {
        transport,
        key: "test-key",
        ledger_dir: paths.ledger().join("entries"),
    }
}

/// The key a stage would compute right now, with no artifact written.
fn key_for(paths: &Paths, stage: &str) -> String {
    let (ctx, _) = Context::open(paths, stage).unwrap();
    ctx.header(stage, "schema", BTreeMap::new(), vec![])
        .idempotence_key
}

#[test]
fn an_empty_agent_set_escalates_and_the_stronger_models_set_routes_proceeds_and_resumes() {
    let dir = scratch("loop");
    write_manifest(&dir);
    let paths = Paths::new(&dir);
    let halt = file_halt(&paths, "capability");
    // Keys before anything lands, so the landing's effect is measurable.
    let gate_before = key_for(&paths, "gate");
    let fetch_before = key_for(&paths, "fetch");

    let mut record = gap::open(&paths, &halt).unwrap();
    assert_eq!(record["halt"]["kind"], "onboarding-gate");
    assert_eq!(
        option::add_set(&mut record, Author::Agent, "claude-opus-5", vec![]).unwrap(),
        Written::EmptyToStronger
    );
    assert_eq!(record["route"], "stronger");
    option::add_set(
        &mut record,
        Author::Stronger,
        "claude-fable-5-1",
        options(&halt, false),
    )
    .unwrap();
    gap::write(&paths, &record).unwrap();

    let transport = GapTransport { top: 0.8 };
    let routed = routing::route(&paths, &judge(&transport, &paths), &halt, &[]).unwrap();
    assert_eq!(routed.route, Route::Proceed);
    assert_eq!(routed.chosen.as_deref(), Some("curtain-rows"));
    let filed = routed.decision.expect("proceed files its own gap-fix");
    // The loop resolved its own choice; nothing is left open for the owner.
    let fix = read_decisions(&paths.decisions())
        .unwrap()
        .into_iter()
        .find(|d| d.id == filed)
        .unwrap();
    assert_eq!(fix.status, Status::Resolved);
    assert_eq!(fix.resolution.as_ref().unwrap().by, "loop");
    assert_eq!(fix.resolution.unwrap().option, "curtain-rows");

    resume::record_spec(&paths, &halt, "fn-37-pendulous-shoots-as-rows").unwrap();
    assert_eq!(
        resume::record_review(&paths, &halt, "ship").unwrap(),
        resume::Reviewed::Recorded { needs_work: 0 }
    );
    let resumed = resume::resume(&paths, &halt, "abc1234", None, None).unwrap();
    assert_eq!(resumed.spec, "fn-37-pendulous-shoots-as-rows");
    // The halted stage and every stage after it rerun; the earlier ones do not.
    assert_eq!(
        resumed.reruns,
        vec!["gate", "generate", "document", "report"]
    );
    assert_ne!(key_for(&paths, "gate"), gate_before);
    assert_eq!(key_for(&paths, "fetch"), fetch_before);
    // The same spec landing twice is refused; the record already carries it.
    assert!(resume::resume(&paths, &halt, "def5678", None, None)
        .unwrap_err()
        .to_string()
        .contains("already resumed"));
    // A halt that stood after the landing runs another round and mints
    // another spec: that spec lands too, the earlier landing moves into the
    // record's history, and the rerun's key carries both fixes.
    let gate_after_first = key_for(&paths, "gate");
    resume::record_spec(&paths, &halt, "fn-109-the-apical-rosette").unwrap();
    let again = resume::resume(&paths, &halt, "def5678", None, None).unwrap();
    assert_eq!(again.spec, "fn-109-the-apical-rosette");
    let record = gap::read(&paths, &halt).unwrap();
    assert_eq!(record["landed"]["spec"], "fn-109-the-apical-rosette");
    assert_eq!(
        record["landings"][0]["spec"],
        "fn-37-pendulous-shoots-as-rows"
    );
    let tools = resume::landed_tools(&paths.dir, "gate");
    assert_eq!(
        tools
            .get("fix:fn-37-pendulous-shoots-as-rows")
            .map(String::as_str),
        Some("abc1234")
    );
    assert_eq!(
        tools
            .get("fix:fn-109-the-apical-rosette")
            .map(String::as_str),
        Some("def5678")
    );
    assert_ne!(key_for(&paths, "gate"), gate_after_first);
    // Two specs minted in one round: the landing names which one lands, a
    // name the gap never minted is refused, and the earlier landing cannot
    // be repeated under the other name.
    resume::record_spec(&paths, &halt, "fn-110-the-palms-trunk-organs").unwrap();
    resume::record_spec(&paths, &halt, "fn-111-the-palms-infructescence").unwrap();
    assert!(
        resume::resume(&paths, &halt, "0123abc", None, Some("fn-999-never-minted"))
            .unwrap_err()
            .to_string()
            .contains("minted no spec")
    );
    let third = resume::resume(
        &paths,
        &halt,
        "0123abc",
        None,
        Some("fn-110-the-palms-trunk-organs"),
    )
    .unwrap();
    assert_eq!(third.spec, "fn-110-the-palms-trunk-organs");
    let record = gap::read(&paths, &halt).unwrap();
    assert_eq!(record["landed"]["spec"], "fn-110-the-palms-trunk-organs");
    assert_eq!(record["landings"].as_array().unwrap().len(), 2);
    assert!(resume::resume(
        &paths,
        &halt,
        "4567def",
        None,
        Some("fn-109-the-apical-rosette")
    )
    .unwrap_err()
    .to_string()
    .contains("already resumed"));

    let numbers = metrics::write(&paths, "silver-birch").unwrap();
    assert_eq!(numbers["autonomy"]["gaps"], 1);
    assert_eq!(numbers["autonomy"]["share_taken"], 1.0);
    assert!(numbers["quality"]["reversals"]
        .as_array()
        .unwrap()
        .is_empty());
    assert!(numbers["efficiency"]["input_tokens"].as_u64().unwrap() >= 40);
}

#[test]
fn a_fix_that_moves_a_pin_is_the_owners_and_lands_only_with_the_pin_note() {
    let dir = scratch("pin");
    write_manifest(&dir);
    let paths = Paths::new(&dir);
    let halt = file_halt(&paths, "capability");
    let mut record = gap::open(&paths, &halt).unwrap();
    option::add_set(&mut record, Author::Agent, "m", options(&halt, true)).unwrap();
    gap::write(&paths, &record).unwrap();

    let transport = GapTransport { top: 0.9 };
    let routed = routing::route(&paths, &judge(&transport, &paths), &halt, &[]).unwrap();
    assert_eq!(routed.route, Route::Owner);
    assert!(routed.why.contains("moves a pin"), "{}", routed.why);
    // The owner's decision is open; the loop wrote no resolution of its own.
    let filed = routed.decision.expect("the owner's route files a decision");
    let open = read_decisions(&paths.decisions())
        .unwrap()
        .into_iter()
        .find(|d| d.id == filed)
        .unwrap();
    assert_eq!(open.status, Status::Open);
    // Nothing is minted until the owner names an option.
    assert!(resume::record_spec(&paths, &halt, "fn-37").is_err());

    resolve(&paths, &filed, "curtain-rows", "owner");
    resume::record_spec(&paths, &halt, "fn-37-pendulous-shoots-as-rows").unwrap();
    let err = resume::resume(&paths, &halt, "abc1234", None, None)
        .unwrap_err()
        .to_string();
    assert!(err.contains("--pin-note"), "{err}");
    resume::resume(
        &paths,
        &halt,
        "abc1234",
        Some("silver-birch: the curtain reaches the ground: fn-37 landed"),
        None,
    )
    .unwrap();
}

#[test]
fn a_gap_spec_reviewed_needs_work_twice_files_for_the_owner() {
    let dir = scratch("review");
    write_manifest(&dir);
    let paths = Paths::new(&dir);
    let halt = file_halt(&paths, "capability");
    let mut record = gap::open(&paths, &halt).unwrap();
    option::add_set(&mut record, Author::Agent, "m", options(&halt, false)).unwrap();
    gap::write(&paths, &record).unwrap();
    let transport = GapTransport { top: 0.8 };
    routing::route(&paths, &judge(&transport, &paths), &halt, &[]).unwrap();
    resume::record_spec(&paths, &halt, "fn-37-pendulous-shoots-as-rows").unwrap();

    assert_eq!(
        resume::record_review(&paths, &halt, "needs-work").unwrap(),
        resume::Reviewed::Recorded { needs_work: 1 }
    );
    let second = resume::record_review(&paths, &halt, "needs-work").unwrap();
    let resume::Reviewed::Owner { decision } = second else {
        panic!("the second needs-work is the owner's: {second:?}");
    };
    assert_eq!(
        decision,
        "silver-birch/gap/gap-review/gate-onboarding-gate-capability"
    );
    assert!(resume::record_review(&paths, &halt, "approve").is_err());
}

#[test]
fn a_reroute_reads_the_recorded_signals_again_and_an_owner_override_is_a_reversal() {
    let dir = scratch("reroute");
    write_manifest(&dir);
    let paths = Paths::new(&dir);
    let halt = file_halt(&paths, "capability");
    let mut record = gap::open(&paths, &halt).unwrap();
    option::add_set(&mut record, Author::Agent, "m", options(&halt, false)).unwrap();
    gap::write(&paths, &record).unwrap();
    let transport = GapTransport { top: 0.8 };
    routing::route(&paths, &judge(&transport, &paths), &halt, &[]).unwrap();
    let calls = std::fs::read_dir(paths.ledger().join("entries"))
        .unwrap()
        .count();

    let again = routing::reroute(&paths, &halt).unwrap();
    assert_eq!(again.route, Route::Proceed);
    let after = std::fs::read_dir(paths.ledger().join("entries"))
        .unwrap()
        .count();
    assert_eq!(after, calls, "a reroute asks nothing");
    let record = gap::read(&paths, &halt).unwrap();
    let routes = record["routes"].as_array().unwrap();
    assert_eq!(routes.len(), 2);
    assert_eq!(routes[1]["reroute_of"], 0);
    assert_eq!(routes[1]["signals"], routes[0]["signals"]);

    // The owner names the other option: a reversal, not a failure.
    let fix = "silver-birch/gap/gap-fix/gate-onboarding-gate-capability";
    resolve(&paths, fix, "raise-the-constant", "owner");
    let found = metrics::reversals(&paths).unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].chose, "curtain-rows");
    assert_eq!(found[0].instead, "raise-the-constant");
    let numbers = metrics::write(&paths, "silver-birch").unwrap();
    assert_eq!(numbers["quality"]["reversals"][0]["decision"], fix);
}

#[test]
fn a_decision_that_is_not_a_halt_never_starts_the_loop() {
    let dir = scratch("kind");
    let paths = Paths::new(&dir);
    let decision = Decision::new(
        DecisionParts {
            species: "silver-birch",
            stage: "fetch",
            kind: "unavailable-source",
            field: Some("M1"),
            age_years: None,
        },
        &[],
        BTreeMap::new(),
        vec![],
        json!({}),
        &["retry"],
        "",
    );
    let id = decision.id.clone();
    append_decisions(&paths.decisions(), vec![decision]).unwrap();
    let err = gap::open(&paths, &id).unwrap_err().to_string();
    assert!(err.contains("onboarding-gate or level-miss"), "{err}");
    let err = gap::open(&paths, "silver-birch/gate/onboarding-gate/never-filed")
        .unwrap_err()
        .to_string();
    assert!(err.contains("not a decision of this run"), "{err}");
}

/// Writes a person's resolution onto a filed decision.
fn resolve(paths: &Paths, id: &str, option: &str, by: &str) {
    let mut decisions = read_decisions(&paths.decisions()).unwrap();
    let decision = decisions.iter_mut().find(|d| d.id == id).expect(id);
    decision.status = Status::Resolved;
    decision.resolution = Some(Resolution {
        id: id.to_string(),
        inputs_sha256: decision.inputs_sha256.clone(),
        option: option.into(),
        by: by.into(),
        at: "2026-09-18T00:00:00Z".into(),
        note: String::new(),
        payload: Value::Null,
    });
    write_decisions(&paths.decisions(), &decisions).unwrap();
}
