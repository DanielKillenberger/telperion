//! fn-131 R4 to R8: verify acts on what it flags, a retired decision stays
//! retired, an age stated in a sentence is a point, no required field is
//! passed silently, and a stage's key carries the code that ran it. The
//! palm's rows and Jev's answers come from `judged`; no network.

mod common;
mod judged;

use std::path::{Path, PathBuf};

use judged::{palm, score, Palm, A1_RATE, F1_FROND};
use serde_json::{json, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::build_id::{tree_digest, BUILD_ID};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::decision::{
    append_decisions, reconcile, retire_unfiled, Decision, DecisionParts, Status,
};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::stage::{idempotence_key, Context, Paths};
use telperion_jev::pipeline::stages::{inputs, quality, select, verify};

const FROND: &str = "/profiles/0/metrics/frond_length_m";
const LEAFLET: &str = "/profiles/0/metrics/leaflet_length_m";
const FRONT: &str = "/profiles/0/appearance/leaf_front_colour";

/// Jev over F1 alone: frond and leaflet lengths picked from F1's sentence,
/// the upper face read grey-green, and every citation claim answered by
/// `relation`.
struct Checker {
    palm: Palm,
    relation: &'static str,
}

impl Transport for Checker {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
        if body["questions"].get("relation").is_none() {
            return self.palm.send(request);
        }
        let r = self.relation;
        let answers = json!({"relation": {"type": "choice", "choice": r, "confidence": 0.95, "probabilities": {r: 0.95}}});
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

fn f1_picks(question: &str, candidates: &[String]) -> Option<String> {
    let span = if question.contains("frond_length_m") {
        "20 feet"
    } else if question.contains("leaflet_length_m") {
        "1 to 2 feet"
    } else {
        return None;
    };
    candidates
        .iter()
        .find(|c| c.ends_with(&format!(": {span}")))
        .cloned()
}

fn checker(relation: &'static str) -> Checker {
    let mut palm = Palm::new(f1_picks);
    palm.levels = |name| {
        (name == "leaf_front_colour").then(|| {
            score(
                json!({"0": 0.0, "1": 0.0, "2": 0.0, "3": 0.97, "4": 0.0, "5": 0.03}),
                3.0,
            )
        })
    };
    Checker { palm, relation }
}

fn judge(t: &dyn Transport) -> Judge<'_> {
    Judge {
        transport: t,
        key: "test-key",
        ledger_dir: common::ledger_dir("verify-judged"),
    }
}

/// F1 fetched and screened, quality and select run: frond and leaflet
/// length filled from F1, and the upper face's colour.
fn filled(tag: &str, t: &dyn Transport) -> PathBuf {
    let dir = judged::fetched_f1(tag);
    let paths = Paths::new(&dir);
    let row = json!({"source": "F1", "sentence": F1_FROND, "kind": "frond_size", "condition": "unstated", "anchor_usable": false, "ledger": "live"});
    let mut leaflet = row.clone();
    leaflet["kind"] = json!("leaflet_size");
    let (ctx, _) = Context::open(&paths, "screen").unwrap();
    ctx.write(
        &ctx.header("screen", "screen", inputs(&[]), vec![]),
        json!({"rows": [row, leaflet]}),
    )
    .unwrap();
    quality::run(&paths, &judge(t)).unwrap();
    select::run(&paths, &judge(t)).unwrap();
    dir
}

fn decision(dir: &Path, id: &str) -> Option<Value> {
    let list = read_json(&dir.join("decisions.json")).unwrap();
    list["decisions"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["id"] == id)
        .cloned()
}

fn claim_id(pointer: &str) -> String {
    format!("date-palm/verify/claim-unsupported/{pointer}")
}

/// R4: two values of F1 file two claims, one per pointer; the appearance
/// claim states the level, so a source that says nothing of it fails it.
#[test]
fn claims_are_per_pointer_and_the_appearance_claim_can_fail() {
    let t = checker("says_nothing");
    let dir = filled("claims", &t);
    verify::run(&Paths::new(&dir), &judge(&t)).unwrap();
    for pointer in [FROND, LEAFLET, FRONT] {
        assert!(decision(&dir, &claim_id(pointer)).is_some(), "{pointer}");
    }
    assert!(decision(&dir, "date-palm/verify/claim-unsupported/F1").is_none());
    let claims = &read_json(&dir.join("verify.json")).unwrap()["body"]["claims"];
    let front = claims
        .as_array()
        .unwrap()
        .iter()
        .find(|c| {
            c["claim"]
                .as_str()
                .unwrap()
                .starts_with("leaf_front_colour")
        })
        .unwrap();
    let summary = telperion_jev::pipeline::requirements::table()
        .level("leaf_front_colour", "grey_green")
        .unwrap()
        .summary
        .clone();
    assert!(
        front["claim"].as_str().unwrap().contains(&summary),
        "{front}"
    );
}

/// R4: `drop-value` takes the flagged value out of the packet, and the
/// field, which the table requires, is filed `requirements-unmet` again.
#[test]
fn a_dropped_value_leaves_the_packet_and_refiles_its_requirement() {
    let t = checker("contradicts");
    let dir = filled("drop", &t);
    let paths = Paths::new(&dir);
    verify::run(&paths, &judge(&t)).unwrap();
    let flagged = decision(
        &dir,
        &format!("date-palm/verify/claim-contradicted/{FROND}"),
    )
    .unwrap();
    write_canonical(
        &paths.resolutions(),
        &json!({"resolutions": [{"id": flagged["id"], "inputs_sha256": flagged["inputs_sha256"],
                                 "option": "drop-value", "by": "cheap-driver", "at": "2026-09-23"}]}),
    )
    .unwrap();
    select::run(&paths, &judge(&t)).unwrap();
    let body = read_json(&dir.join("select.json")).unwrap()["body"].clone();
    assert!(body["filled"].get(FROND).is_none(), "{body}");
    assert!(body["filled"].get(LEAFLET).is_some(), "{body}");
    let profile = read_json(&paths.packet("profile")).unwrap();
    assert_eq!(
        profile["profiles"][0]["metrics"]["frond_length_m"]["classification"],
        "unavailable"
    );
    let unmet = decision(&dir, "date-palm/select/requirements-unmet/frond_length_m").unwrap();
    assert_eq!(unmet["status"], "open");
    let consumed = decision(&dir, flagged["id"].as_str().unwrap()).unwrap();
    assert_eq!(consumed["consumed_by"], "select");
}

/// R5: a decision its stage superseded stays resolved when a person's
/// resolution written against its old inputs is read.
#[test]
fn a_superseded_decision_ignores_a_stale_resolution() {
    let dir = palm("retired");
    let paths = Paths::new(&dir);
    let filed = Decision::new(
        DecisionParts {
            species: "date-palm",
            stage: "verify",
            kind: "claim-unsupported",
            field: Some("A1"),
            age_years: None,
        },
        &["generate"],
        inputs(&[("select.json", "old")]),
        vec![],
        json!({}),
        &["accept", "replace-source", "drop-value"],
        "live",
    );
    let id = filed.id.clone();
    append_decisions(&paths.decisions(), vec![filed]).unwrap();
    retire_unfiled(
        &paths.decisions(),
        "verify",
        &[],
        &inputs(&[("select.json", "new")]),
        "2026-09-23",
    )
    .unwrap();
    write_canonical(
        &paths.resolutions(),
        &json!({"resolutions": [{"id": id, "inputs_sha256": {"select.json": "older"},
                                 "option": "accept", "by": "owner", "at": "2026-09-22"}]}),
    )
    .unwrap();
    let list = reconcile(&paths).unwrap();
    let d = list.iter().find(|d| d.id == id).unwrap();
    assert_eq!(d.status, Status::Resolved);
    assert_eq!(d.resolution.as_ref().unwrap().option, "superseded");
}

/// R6: A1's "reaching 5 m (20 feet) in 15 to 20 years" is a height point
/// at 15 to 20 years, though the screen called the sentence a rate. The
/// palm row asks height mature (fn-132), so the manifest here predates the
/// requirements table and asks it at an age.
#[test]
fn a_size_reached_at_a_stated_age_is_a_point() {
    let dir = palm("age");
    let mut legacy = judged::manifest();
    legacy["schema_version"] = json!(1);
    write_canonical(&dir.join("manifest.json"), &legacy).unwrap();
    let palm = Palm::new(|_, _| None);
    quality::run(&Paths::new(&dir), &judged::judge(&palm)).unwrap();
    let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
    let point = fields["height_m"]["points"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["sentence"] == A1_RATE)
        .cloned()
        .unwrap_or_else(|| panic!("no A1 point: {}", fields["height_m"]));
    assert_eq!(point["ages_years"], json!([15.0, 20.0]));
}

/// R7: a mature field with no point whose dominant gap is `no_mature_size`
/// fails its bar whatever level was scored, and files `requirements-unmet`.
/// A field with a point never carries that gap (fn-133): the crown's two
/// points pass its `proxy_only` bar.
#[test]
fn no_mature_size_fails_a_mature_field() {
    struct NoMature;
    impl Transport for NoMature {
        fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
            let body: Value = serde_json::from_slice(request.body.as_deref().unwrap()).unwrap();
            let answers = if body["questions"].get("mature_size").is_some() {
                json!({"mature_size": score(json!({"0": 0.05, "1": 0.9, "2": 0.05, "3": 0.0}), 1.0),
                       "mature_gap": {"type": "choice", "choice": "no_mature_size", "confidence": 0.9, "probabilities": {}}})
            } else {
                json!({"sufficiency": score(json!({"0": 1.0}), 0.0),
                       "dominant_gap": {"type": "choice", "choice": "no_age_indexed_points", "confidence": 0.9, "probabilities": {}}})
            };
            Ok(HttpResponse {
                status: 200,
                body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers}))
                    .unwrap(),
            })
        }
    }
    let dir = palm("no-mature");
    quality::run(&Paths::new(&dir), &judge(&NoMature)).unwrap();
    let fields = &read_json(&dir.join("quality.json")).unwrap()["body"]["fields"];
    let dbh = &fields["dbh_m"];
    assert_eq!(dbh["level"], "proxy_only");
    assert_eq!(dbh["points"], json!([]));
    assert_eq!(dbh["passed"], false, "{fields}");
    let id = "date-palm/quality/requirements-unmet/dbh_m";
    assert_eq!(decision(&dir, id).unwrap()["status"], "open");
    let crown = &fields["crown_width_m"];
    assert_eq!(crown["level"], "proxy_only");
    assert_ne!(crown["dominant_gap"], "no_mature_size", "{crown}");
    assert_eq!(crown["passed"], true, "{fields}");
}

/// R8: a stage's key carries the build identity, a digest of the crate's
/// code and data; a record written by other code is not current.
#[test]
fn a_code_change_expires_a_stages_key() {
    assert_eq!(BUILD_ID, tree_digest(Path::new(env!("CARGO_MANIFEST_DIR"))));
    let dir = palm("key");
    let (ctx, _) = Context::open(&Paths::new(&dir), "extract").unwrap();
    let header = ctx.header("extract", "candidates", inputs(&[]), vec![]);
    assert_eq!(header.tools["species-pipeline-build"], BUILD_ID);
    let mut older = header.tools.clone();
    older.insert("species-pipeline-build".into(), "other-code".into());
    let m = &ctx.admitted.manifest;
    let stale = idempotence_key(
        &header.inputs,
        &ctx.admitted.sha256,
        &m.versions.question_sets,
        &m.model,
        &older,
    );
    let mut written = header.clone();
    written.idempotence_key = stale;
    ctx.write(&written, json!({"candidates": []})).unwrap();
    assert!(!ctx.is_current("extract", &header.idempotence_key));
}
