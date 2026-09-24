//! The four downstream stages over hand-written upstream artifacts.
//!
//! Nothing here calls Jev, reaches the network, needs a GPU or needs a built
//! binary: the fetch, quality and select artifacts are written through a
//! `Context` so their headers are real, the render-and-measure step is a table
//! of dial values, and the one Jev request the transfer route makes is
//! answered by a transport this file wraps around the shared mock.

mod common;

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use common::{ledger_dir, CaseTransport};
use serde_json::{json, Map, Value};
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::gap::metrics;
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::render::{Measured, Measurer, RenderError};
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::gate::GateChecks;
use telperion_jev::pipeline::stages::{fit, gate, generate, inputs, report};

/// The dial the described route lays its candidates on.
const SPREAD: &str = "skeleton.envelope.spread";
/// The dial the reference transfer fills from a tuned template.
const RISE: &str = "skeleton.habit.rise_secondary";
/// fn-30's oak height table, in metres by age.
const OAK_HEIGHT_M: [(f64, f64); 18] = [
    (30.0, 9.0),
    (40.0, 12.0),
    (50.0, 14.6),
    (60.0, 16.9),
    (70.0, 18.8),
    (80.0, 20.3),
    (90.0, 21.6),
    (100.0, 22.8),
    (110.0, 23.8),
    (120.0, 24.8),
    (130.0, 25.8),
    (140.0, 26.8),
    (150.0, 27.7),
    (160.0, 28.3),
    (170.0, 28.9),
    (180.0, 29.4),
    (190.0, 29.8),
    (200.0, 30.2),
];
/// The Gould anchor table: 7.2 cm at 30 years, which fn-30 anchored on.
const OAK_DBH_CM: [(f64, f64); 2] = [(30.0, 7.2), (60.0, 20.0)];

// ---------------------------------------------------------------- the manifest

fn manifest() -> Value {
    json!({
        "schema": "manifest", "schema_version": 1,
        "species": "oregon-white-oak",
        "taxon": {"scientific_name": "Quercus garryana", "common_name": "Oregon white oak",
                  "rank": "species"},
        "context": "mature open-grown", "growth_form": "broadleaf",
        "preset": "oregon-white-oak", "profile_id": "oregon-white-oak", "seed": 7,
        "sources": [{"id": "S1", "url": "https://example.test/s1", "title": "Silvics",
                     "rights": "public domain"}],
        "fields": [{"field": "height_m", "condition": "open_grown",
                    "required_ages_years": [26.7, 56.1, 112.0], "bar": "partial",
                    "question": "What height does a tree reach at a stated age?"}],
        "curves": {
            "reference_ages_years": [26.7, 56.1, 112.0],
            "tolerance_percent": 15.0,
            "envelope_height_m": 24.0,
            "mature_dbh_m": 0.836,
            "height": {"method": "table", "table": "E1", "below_first_row": "linear-from-zero"},
            "dbh": {"method": "gould-integration", "anchor_table": "G1",
                    "anchor_age_years": 30.0, "site_index_m": 35.0, "max_age_years": 600.0,
                    "below_anchor": "linear-from-zero",
                    "coefficients": {"intercept": -1.33299, "ln_dbh": 1.66609, "dbh2": -0.00154,
                                     "bal": -0.00326, "ba": -0.00204, "ln_si": 0.14995}},
        },
        "described": [{
            "trait_name": "crown_spread", "sources": ["S1"],
            "table": {"trait_name": "crown_spread",
                      "measured_metric": "crown_width_height_ratio", "dial": SPREAD,
                      "levels": [{"key": "broad", "summary": "as broad as it is tall",
                                  "target_range": [0.9, 1.1], "candidates": [0.4, 0.6]}]},
        }],
        "transfers": [{"dial": RISE, "forbid_other_growth_form": false,
                       "levels": [{"key": "about_the_same", "summary": "the same tier",
                                   "multiplier": 1.0}]}],
        "engineering": {
            "required_capabilities": {"value": ["woody-axes"],
                                      "rationale": "the fn-19 protocol names it"},
            "transfer:skeleton.habit.rise_secondary": {
                "value": [{"template": "norway-spruce", "shipped_value": -0.2,
                           "growth_form": "conifer"}],
                "rationale": "the spruce is the one tuned template with this dial"},
        },
        "versions": {"question_sets": {"described": 1}, "tools": {}},
        "model": "jev-latest"
    })
}

// ------------------------------------------------------- the upstream artifacts

fn rows(table: &[(f64, f64)]) -> Vec<Value> {
    table
        .iter()
        .map(|&(age_years, value)| json!({"age_years": age_years, "value": value}))
        .collect()
}

fn table(dimension: &str, unit: &str, table: &[(f64, f64)]) -> Value {
    json!({
        "source": "S1", "dimension": dimension, "unit": unit, "condition": "open_grown",
        "taxon": "Quercus garryana", "expected_rows": table.len(), "found_rows": table.len(),
        "rows": rows(table),
    })
}

/// A scratch pipeline directory with the manifest, the sidecar, the packet's
/// profile and the fetch, quality and select artifacts a downstream stage
/// reads. `dbh` says how many rows the Gould anchor table yields.
fn scratch(tag: &str, dbh_rows: usize) -> PathBuf {
    scratch_with(tag, dbh_rows, manifest())
}

/// The same directory over a manifest the caller wrote.
fn scratch_with(tag: &str, dbh_rows: usize, manifest: Value) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-downstream-{tag}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let (ctx, _) = Context::open(&Paths::new(&dir), "fetch").unwrap();
    let fetch = json!({
        "sources": {"S1": {"content_type": "text/html", "final_url": "https://example.test/s1",
                           "markdown_bytes": 12, "markdown_sha256": "aa", "raw_bytes": 12,
                           "raw_sha256": "bb"}},
        "tables": {
            "E1": table("height_m", "m", &OAK_HEIGHT_M),
            "G1": table("dbh_m", "cm", &OAK_DBH_CM[..dbh_rows]),
        },
    });
    write(&ctx, "fetch", "sources", fetch);
    let quality = json!({"fields": {"height_m": {
        "level": "partial", "bar": "partial", "passed": true, "dominant_gap": "none",
        "points": [], "required_ages_covered": [26.7, 56.1], "required_ages_uncovered": [],
        "ledger": "q1"}}});
    write(&ctx, "quality", "quality", quality);
    let select = json!({
        "filled": {"/profiles/0/metrics/height_m": {"unit": "m", "range": [15.24, 27.432],
                   "classification": "gating", "source": ["S1"], "confidence": "pipeline",
                   "note": "50 to 90 ft tall"}},
        "unavailable": {},
        "described": {"crown_spread": {"level": "broad",
                      "sentence": "The crown is as broad as the tree is tall.", "ledger": "d1"}},
    });
    write(&ctx, "select", "select", select);
    write_canonical(
        &ctx.paths.sidecar(),
        &json!({"schema": "provenance", "schema_version": 1, "entries": {}, "unavailable": {}}),
    )
    .unwrap();
    write_canonical(
        &ctx.paths.packet("profile"),
        &json!({"schema_version": 1, "profiles": []}),
    )
    .unwrap();
    dir
}

/// One upstream artifact, written through the context so its header is real.
fn write(ctx: &Context, stage: &str, schema: &str, body: Value) {
    let header = ctx.header(stage, schema, inputs(&[]), vec![]);
    ctx.write(&header, body).unwrap();
}

fn body_of(dir: &Path, stage: &str) -> Value {
    read_json(&dir.join(format!("{stage}.json"))).unwrap()["body"].clone()
}

fn decisions(dir: &Path) -> Vec<Value> {
    let path = dir.join("decisions.json");
    if !path.exists() {
        return Vec::new();
    }
    read_json(&path).unwrap()["decisions"]
        .as_array()
        .cloned()
        .unwrap_or_default()
}

fn of_kind(dir: &Path, kind: &str) -> Vec<Value> {
    decisions(dir)
        .into_iter()
        .filter(|d| d["kind"] == kind)
        .collect()
}

// ------------------------------------------------------------- the mock answers

/// The render-and-measure step as a table: each dial value measures what its
/// row says, and the preset with an empty family measures its own height.
struct MockMeasurer {
    rows: Vec<(&'static str, f64, f64)>,
    receipt: PathBuf,
}

fn dial_value(family: &Value, dial: &str) -> Option<f64> {
    let mut node = family;
    for key in dial.split('.') {
        node = node.get(key)?;
    }
    node.as_f64()
}

impl Measurer for MockMeasurer {
    fn measure(&self, _preset: &str, _seed: u32, family: &Value) -> Result<Measured, RenderError> {
        let measured = |value: f64| Measured {
            metrics: json!({"crown_width_height_ratio": {"status": "measured", "value": value},
                            "height_m": {"status": "measured", "value": 24.0}}),
            receipt_path: self.receipt.clone(),
        };
        if family.as_object().is_some_and(Map::is_empty) {
            return Ok(measured(1.0));
        }
        for (dial, value, reads) in &self.rows {
            if dial_value(family, dial).is_some_and(|v| (v - value).abs() < 1e-9) {
                return Ok(measured(*reads));
            }
        }
        Err(RenderError::Measure(format!("no row for {family}")))
    }
}

/// The shared mock with the transfer set added: the nearest-reference Choice
/// answers what the test names, and the relation Score picks the first level.
struct TransferTransport {
    inner: CaseTransport,
    nearest: &'static str,
}

impl Transport for TransferTransport {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value =
            serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap_or(json!({}));
        if body["questions"].get("nearest").is_none() {
            return self.inner.send(request);
        }
        let answers = json!({
            "nearest": {"type": "choice", "choice": self.nearest,
                        "probabilities": {self.nearest: 0.9}, "confidence": 0.85},
            "relation": {"type": "score", "score": 0.0, "probabilities": {"0": 0.9},
                         "confidence": 0.85},
        });
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers,
                                             "usage": {"input_tokens": 1, "output_tokens": 1}}))
            .unwrap(),
        })
    }
}

struct Checks {
    registered: bool,
    /// What the preset's own value table produces, or why the probe that
    /// reads it could not say.
    derived: Result<Vec<String>, String>,
}

impl Checks {
    fn producing(names: &[&str]) -> Self {
        Self {
            registered: true,
            derived: Ok(names.iter().map(|name| (*name).to_string()).collect()),
        }
    }
}

impl GateChecks for Checks {
    fn registry(&self, _preset: &str) -> Result<bool, String> {
        Ok(self.registered)
    }
    fn capabilities(&self, _preset: &str) -> Result<Vec<String>, String> {
        self.derived.clone()
    }
}

/// The gate that passes, with the audited seeds already on disk.
fn pass_the_gate(dir: &Path) {
    pass_the_seeds(dir);
    let checks = Checks::producing(&["woody-axes"]);
    assert!(matches!(
        gate::run(&Paths::new(dir), &checks).unwrap(),
        gate::Outcome::Ran { .. }
    ));
}

/// The audited seeds on disk, so the seed gate answers and a test reads the
/// capability gate on its own.
fn pass_the_seeds(dir: &Path) {
    let cases: Vec<Value> = [1u32, 2, 3, 40, 41, 42]
        .iter()
        .enumerate()
        .map(|(index, seed)| {
            let role = if index < 3 {
                "regression"
            } else {
                "holdout-at-freeze"
            };
            json!({"id": format!("oregon-white-oak-{seed}"), "species_id": "oregon-white-oak",
                   "seed": seed, "seed_role": role, "parameter_species_id": "oregon-white-oak"})
        })
        .collect();
    write_canonical(
        &dir.join("packet").join("specimens.json"),
        &json!({"benchmark_id": "fn19-v1", "cases": cases, "seed_evidence": "unaudited",
                "fresh_for_future_tuning": false, "receipts": [],
                "generation_status": "not-run-by-this-packet", "expert_status": "unassessed"}),
    )
    .unwrap();
}

/// The generate stage over the mock measurer and one transfer answer.
fn run_generate(
    dir: &Path,
    rows: Vec<(&'static str, f64, f64)>,
    nearest: &'static str,
) -> generate::Outcome {
    let transport = TransferTransport {
        inner: CaseTransport,
        nearest,
    };
    let judge = Judge {
        transport: &transport,
        key: "test-key",
        ledger_dir: ledger_dir("downstream"),
    };
    let measurer = MockMeasurer {
        rows,
        receipt: dir.join("cache").join("measure.jsonl"),
    };
    generate::run(&Paths::new(dir), &judge, &measurer, None).unwrap()
}

// -------------------------------------------------------------------- the fit

#[test]
fn the_fit_reproduces_fn30s_oak_and_files_a_tolerance_miss_per_age() {
    let dir = scratch("fit", 2);
    assert!(matches!(
        fit::run(&Paths::new(&dir)).unwrap(),
        fit::Outcome::Ran { .. }
    ));
    let body = body_of(&dir, "fit");
    assert_eq!(body["rate"], json!(0.032));
    assert_eq!(body["shape"], json!(2.0));
    assert_eq!(body["derived_mature_age_years"], json!(432));
    assert_eq!(body["points"]["E1"].as_array().unwrap().len(), 18);
    assert_eq!(body["compositions"]["dbh"]["method"], "gould-integration");

    let misses = of_kind(&dir, "tolerance-miss");
    assert_eq!(misses.len(), 3, "{misses:?}");
    for miss in &misses {
        assert_eq!(miss["field"], "dbh_m");
        assert_eq!(miss["blocks"], json!(["generate", "report"]));
        assert_eq!(miss["payload"]["sources"], json!(["G1"]));
        assert_eq!(miss["payload"]["field"], "dbh_m");
        for key in ["age_years", "measured", "reference", "error_percent"] {
            assert!(miss["payload"][key].is_f64(), "{key} in {miss}");
        }
        assert!(miss["payload"]["error_percent"].as_f64().unwrap().abs() > 15.0);
        assert_eq!(
            miss["options"],
            json!(["accept-composed-reference", "anchor-elsewhere", "reject"])
        );
    }
    assert_eq!(
        misses[0]["id"],
        "oregon-white-oak/fit/tolerance-miss/dbh_m/112"
    );
}

#[test]
fn a_one_row_table_files_missing_curve_and_the_other_dimension_still_fits() {
    let dir = scratch("missing", 1);
    fit::run(&Paths::new(&dir)).unwrap();
    let body = body_of(&dir, "fit");
    assert_eq!(body["rate"], json!(0.032), "the height still fits");
    assert_eq!(body["misses"], json!([]));
    let unavailable = body["unavailable"].as_array().unwrap();
    assert_eq!(unavailable.len(), 3, "every diameter age is unavailable");
    assert!(unavailable.iter().all(|u| u["field"] == "dbh_m"));

    let missing = of_kind(&dir, "missing-curve");
    assert_eq!(missing.len(), 1);
    assert_eq!(missing[0]["field"], "dbh_m");
    assert_eq!(missing[0]["blocks"], json!(["generate"]));
    assert_eq!(
        missing[0]["payload"],
        json!({"dimension": "dbh_m", "ages_with_no_point": [26.7, 56.1, 112.0]})
    );
}

#[test]
fn a_fit_with_no_curve_is_skipped_once_and_current_after() {
    let mut manifest = manifest();
    manifest["curves"] = Value::Null;
    let dir = scratch_with("no-curve", 2, manifest);
    assert!(matches!(
        fit::run(&Paths::new(&dir)).unwrap(),
        fit::Outcome::Skipped
    ));
    assert_eq!(
        body_of(&dir, "fit")["skipped"],
        "the manifest names no curve"
    );
    assert!(matches!(
        fit::run(&Paths::new(&dir)).unwrap(),
        fit::Outcome::Current
    ));
}

#[test]
fn an_unchanged_rerun_of_the_fit_is_current_and_writes_nothing_new() {
    let dir = scratch("current", 2);
    fit::run(&Paths::new(&dir)).unwrap();
    let before = std::fs::read(dir.join("fit.json")).unwrap();
    let filed: Vec<Value> = decisions(&dir).iter().map(|d| d["id"].clone()).collect();
    assert!(matches!(
        fit::run(&Paths::new(&dir)).unwrap(),
        fit::Outcome::Current
    ));
    let after = std::fs::read(dir.join("fit.json")).unwrap();
    assert_eq!(
        String::from_utf8_lossy(&before),
        String::from_utf8_lossy(&after)
    );
    let again: Vec<Value> = decisions(&dir).iter().map(|d| d["id"].clone()).collect();
    assert_eq!(filed, again, "a current rerun files no new decision");
}

// ------------------------------------------------------------------- the gate

/// The date palm's recorded requirement, from
/// `.flow/evidence/date-palm/pipeline/manifest.json` on the fn-82 branch.
const DATE_PALM: [&str; 6] = [
    "woody-axes",
    "apical-rosette",
    "pinnate-frond",
    "acanthophyll",
    "persistent-leaf-base",
    "infructescence",
];

/// The manifest with another species' recorded requirement on its
/// engineering row.
fn requiring(names: &[&str]) -> Value {
    let mut manifest = manifest();
    manifest["engineering"]["required_capabilities"]["value"] = json!(names);
    manifest
}

#[test]
fn the_gate_files_onboarding_gate_for_an_unregistered_preset() {
    let dir = scratch("unregistered", 2);
    let checks = Checks {
        registered: false,
        derived: Ok(vec![]),
    };
    gate::run(&Paths::new(&dir), &checks).unwrap();
    let body = body_of(&dir, "gate");
    assert_eq!(body["registry"], json!(false));
    // The generator expresses what this species requires whether or not a
    // preset is registered for it, and the unregistered preset is not asked
    // what its table produces.
    assert_eq!(body["capability"]["missing"], json!([]));
    assert_eq!(body["capability"]["expressed"], json!(["woody-axes"]));
    assert_eq!(body["capability"]["preset"], json!({"registered": false}));
    assert_eq!(body["seeds"]["status"], "unresolved");
    let filed = of_kind(&dir, "onboarding-gate");
    let fields: Vec<&str> = filed.iter().filter_map(|d| d["field"].as_str()).collect();
    assert_eq!(fields, vec!["registry", "seeds"]);
    assert!(
        filed
            .iter()
            .all(|d| d["blocks"] == json!(["generate"])
                && d["options"] == json!(["resolve", "waive"]))
    );
    assert_eq!(filed[0]["payload"]["gate"], "registry");
}

#[test]
fn the_date_palms_recorded_needs_are_one_missing_organ_and_no_false_positive() {
    let dir = scratch_with("date-palm", 2, requiring(&DATE_PALM));
    let checks = Checks {
        registered: false,
        derived: Ok(vec![]),
    };
    gate::run(&Paths::new(&dir), &checks).unwrap();
    let capability = body_of(&dir, "gate")["capability"].clone();
    // fn-109 gave the generator the frond crown and fn-110 the trunk organs,
    // so the palm's gate halts on the fruiting cluster alone.
    assert_eq!(
        capability["expressed"],
        json!([
            "woody-axes",
            "apical-rosette",
            "pinnate-frond",
            "acanthophyll",
            "persistent-leaf-base"
        ])
    );
    assert_eq!(capability["missing"], json!(["infructescence"]));
    assert_eq!(capability["unrecognised"], json!([]));
    let filed = of_kind(&dir, "onboarding-gate");
    let capability_gate = filed.iter().find(|d| d["field"] == "capability").unwrap();
    assert_eq!(
        capability_gate["payload"]["detail"],
        "the generator does not express infructescence"
    );
}

/// fn-136: with the date cluster classed an improvement on the palm's
/// assessment, the gate files no capability decision and records the gap
/// with the specs that capture it.
#[test]
fn a_missing_improvement_passes_the_gate_as_a_known_gap_with_its_specs() {
    let dir = scratch_with("date-palm-classed", 2, requiring(&DATE_PALM));
    std::fs::write(
        dir.join("packet/capability.json"),
        include_str!("fixtures/fn136-palm-capability.json"),
    )
    .unwrap();
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    let capability = body_of(&dir, "gate")["capability"].clone();
    assert_eq!(capability["missing"], json!(["infructescence"]));
    assert_eq!(
        capability["known_gaps"],
        json!([{"capability": "infructescence",
                "captured_by": ["fn-33-flowers-cones-and-compound-leaves-as",
                                "fn-111-the-palms-infructescence-a-hanging-date"],
                "reason": "The date cluster adds realism; the palm is recognisable without it. The owner put the palm's date clusters in the backlog on 2026-09-24."}])
    );
    let filed = of_kind(&dir, "onboarding-gate");
    assert!(
        filed.iter().all(|d| d["field"] != "capability"),
        "{filed:?}"
    );
}

#[test]
fn the_gate_records_the_vocabulary_version_it_compared_against() {
    let dir = scratch("vocabulary-version", 2);
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    assert_eq!(
        body_of(&dir, "gate")["capability"]["vocabulary_version"],
        json!(telperion_core::capability::version())
    );
}

#[test]
fn a_required_name_the_vocabulary_does_not_carry_is_unrecognised_not_missing() {
    let dir = scratch_with("unrecognised", 2, requiring(&["woody-axes", "woody-axe"]));
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    let capability = body_of(&dir, "gate")["capability"].clone();
    assert_eq!(capability["missing"], json!([]));
    assert_eq!(capability["unrecognised"], json!(["woody-axe"]));
    let filed = of_kind(&dir, "onboarding-gate");
    let detail = filed
        .iter()
        .find(|d| d["field"] == "capability")
        .map(|d| d["payload"]["detail"].clone())
        .unwrap();
    assert_eq!(
        detail,
        "woody-axe is not a name the capability vocabulary carries"
    );
}

#[test]
fn a_species_with_no_recorded_assessment_does_not_pass_the_capability_gate() {
    let dir = scratch_with("unassessed", 2, requiring(&[]));
    pass_the_seeds(&dir);
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    let filed = of_kind(&dir, "onboarding-gate");
    let fields: Vec<&str> = filed.iter().filter_map(|d| d["field"].as_str()).collect();
    assert_eq!(fields, vec!["capability"]);
    assert!(filed[0]["payload"]["detail"]
        .as_str()
        .unwrap()
        .starts_with("no capability assessment is recorded"));
}

#[test]
fn a_registered_table_that_does_not_produce_what_the_species_requires_is_its_own_gate() {
    let dir = scratch_with("table", 2, requiring(&["woody-axes", "lobed-blade"]));
    pass_the_seeds(&dir);
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    let body = body_of(&dir, "gate");
    // The generator expresses both; the oak's own table does not draw the
    // second, which is the preset's question and never the vocabulary's.
    assert_eq!(body["capability"]["missing"], json!([]));
    assert_eq!(
        body["capability"]["preset"]["unproduced"],
        json!(["lobed-blade"])
    );
    let filed = of_kind(&dir, "onboarding-gate");
    let fields: Vec<&str> = filed.iter().filter_map(|d| d["field"].as_str()).collect();
    assert_eq!(fields, vec!["preset-capability"]);
}

#[test]
fn a_table_probe_that_errors_files_the_error_and_never_an_empty_list() {
    let dir = scratch("probe-error", 2);
    pass_the_seeds(&dir);
    let checks = Checks {
        registered: true,
        derived: Err("--support oregon-white-oak: no such file".into()),
    };
    gate::run(&Paths::new(&dir), &checks).unwrap();
    let body = body_of(&dir, "gate");
    assert_eq!(
        body["capability"]["preset"],
        json!({"registered": true, "error": "--support oregon-white-oak: no such file"})
    );
    let filed = of_kind(&dir, "onboarding-gate");
    let fields: Vec<&str> = filed.iter().filter_map(|d| d["field"].as_str()).collect();
    assert_eq!(fields, vec!["preset-capability"]);
}

#[test]
fn an_empty_packet_list_does_not_erase_the_manifests_recorded_assessment() {
    let dir = scratch_with("packet-placeholder", 2, requiring(&["woody-axes"]));
    pass_the_seeds(&dir);
    // The generate stage writes this list empty for every species, so a gate
    // rerun after generate reads the assessment the manifest records, not the
    // placeholder; read the other way the gate would fail closed on a species
    // that had already passed it.
    write_canonical(
        &dir.join("packet").join("species.json"),
        &json!({"required_capabilities": []}),
    )
    .unwrap();
    gate::run(&Paths::new(&dir), &Checks::producing(&["woody-axes"])).unwrap();
    let body = body_of(&dir, "gate");
    assert_eq!(body["capability"]["required"], json!(["woody-axes"]));
    assert_eq!(body["capability"]["expressed"], json!(["woody-axes"]));
    assert_eq!(body["unresolved"], json!([]));
}

#[test]
fn the_gate_files_nothing_when_every_gate_is_answered() {
    let dir = scratch("gate-pass", 2);
    pass_the_gate(&dir);
    let body = body_of(&dir, "gate");
    assert_eq!(body["registry"], json!(true));
    assert_eq!(body["unresolved"], json!([]));
    assert_eq!(
        body["seeds"],
        json!({"status": "resolved", "fixed": 3, "holdout": 3})
    );
    assert!(!dir.join("decisions.json").exists());
}

// --------------------------------------------------------------- the generation

#[test]
fn generate_ships_a_described_value_into_the_sidecar_at_the_dials_pointer() {
    let dir = scratch("described", 2);
    pass_the_gate(&dir);
    let rows = vec![(SPREAD, 0.4, 0.5), (SPREAD, 0.6, 1.0), (RISE, -0.2, 0.3)];
    run_generate(&dir, rows, "norway-spruce");

    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    let entry = &sidecar["entries"]["/parameters/skeleton/envelope/spread"];
    assert_eq!(entry["route"], "described");
    assert_eq!(entry["level"], "broad");
    assert_eq!(entry["shipped"], json!(0.6));
    assert_eq!(entry["ledger"], json!(["d1"]));
    assert!(of_kind(&dir, "level-miss").is_empty());

    let transferred = &sidecar["entries"]["/parameters/skeleton/habit/rise_secondary"];
    assert_eq!(transferred["route"], "reference");
    assert_eq!(transferred["reference"], "norway-spruce");
    assert_eq!(transferred["candidate"], json!(-0.2));

    let species = read_json(&dir.join("packet").join("species.json")).unwrap();
    assert_eq!(
        species["parameters"]["skeleton"]["envelope"]["spread"],
        json!(0.6)
    );
    assert!(!body_of(&dir, "generate").to_string().contains("probabilit"));
}

#[test]
fn generate_files_level_miss_when_no_candidate_lands_in_range() {
    let dir = scratch("level-miss", 2);
    pass_the_gate(&dir);
    let rows = vec![(SPREAD, 0.4, 0.2), (SPREAD, 0.6, 0.3), (RISE, -0.2, 0.3)];
    run_generate(&dir, rows, "norway-spruce");
    let filed = of_kind(&dir, "level-miss");
    assert_eq!(filed.len(), 1, "{filed:?}");
    assert_eq!(filed[0]["field"], "crown_spread");
    assert_eq!(filed[0]["blocks"], json!(["report"]));
    assert_eq!(filed[0]["payload"]["target_range"], json!([0.9, 1.1]));
    assert_eq!(
        filed[0]["payload"]["candidates"].as_array().unwrap().len(),
        2
    );
    let sidecar = read_json(&dir.join("provenance.json")).unwrap();
    assert!(sidecar["entries"]["/parameters/skeleton/envelope/spread"].is_null());
}

#[test]
fn generate_files_no_reference_when_the_nearest_answer_is_none() {
    let dir = scratch("no-reference", 2);
    pass_the_gate(&dir);
    let rows = vec![(SPREAD, 0.4, 0.5), (SPREAD, 0.6, 1.0)];
    run_generate(&dir, rows, "none");
    let filed = of_kind(&dir, "no-reference");
    assert_eq!(filed.len(), 1, "{filed:?}");
    assert_eq!(filed[0]["field"], RISE);
    assert_eq!(filed[0]["blocks"], json!(["report"]));
    assert_eq!(filed[0]["payload"]["dial"], RISE);
    assert!(filed[0]["payload"]["reason"].is_string());
    assert_eq!(body_of(&dir, "generate")["transfers"][RISE]["dial"], RISE);
}

#[test]
fn the_packets_species_record_carries_exactly_the_closed_keys() {
    let dir = scratch("packet", 2);
    pass_the_gate(&dir);
    run_generate(
        &dir,
        vec![(SPREAD, 0.4, 1.0), (RISE, -0.2, 0.3)],
        "norway-spruce",
    );
    let species = read_json(&dir.join("packet").join("species.json")).unwrap();
    let keys: Vec<&String> = species.as_object().unwrap().keys().collect();
    assert_eq!(
        keys,
        vec![
            "context",
            "cultivar",
            "fixed_seeds",
            "holdout_seeds",
            "id",
            "parameters",
            "preset",
            "profile_id",
            "profile_path",
            "profile_sha256",
            "reference_ids",
            "required_capabilities",
            "scientific_name",
            "taxon_rank",
        ]
    );
    assert_eq!(species["fixed_seeds"], json!([7, 8, 9]));
    assert_eq!(species["profile_path"], "packet/profile.json");
    assert_eq!(species["profile_sha256"].as_str().unwrap().len(), 64);
    let specimens = read_json(&dir.join("packet").join("specimens.json")).unwrap();
    assert_eq!(specimens["generation_status"], "measured-by-pipeline");
    assert_eq!(specimens["cases"].as_array().unwrap().len(), 3);
}

// ----------------------------------------------------------------- the report

#[test]
fn the_report_is_halted_with_an_open_decision_and_complete_once_it_is_resolved() {
    let dir = scratch("report", 2);
    fit::run(&Paths::new(&dir)).unwrap();
    assert!(matches!(
        report::run(&Paths::new(&dir)).unwrap(),
        report::Outcome::Ran { .. }
    ));
    let body = body_of(&dir, "report");
    assert_eq!(body["status"], "halted");
    assert_eq!(body["sources"][0]["raw_sha256"], "bb");
    assert_eq!(body["fields"]["height_m"]["level"], "partial");
    assert_eq!(body["curves"]["rate"], json!(0.032));
    assert_eq!(body["decisions"].as_array().unwrap().len(), 3);
    let page = std::fs::read_to_string(dir.join("report.md")).unwrap();
    for heading in [
        "## Sources",
        "## Fields",
        "## Curves",
        "## Decisions",
        "## Stills",
    ] {
        assert!(page.contains(heading), "{heading} is missing");
    }
    assert!(!page.contains('\u{2014}'), "the page has an em dash");
    assert!(!page.to_lowercase().contains("probabilit"));

    let resolutions: Vec<Value> = decisions(&dir)
        .iter()
        .map(|d| {
            json!({"id": d["id"], "inputs_sha256": d["inputs_sha256"],
                   "option": "accept-composed-reference", "by": "test", "at": "2026-09-18"})
        })
        .collect();
    write_canonical(
        &dir.join("resolutions.json"),
        &json!({"schema": "resolutions", "schema_version": 1, "resolutions": resolutions}),
    )
    .unwrap();
    assert!(matches!(
        report::run(&Paths::new(&dir)).unwrap(),
        report::Outcome::Ran { .. }
    ));
    // Every decision is resolved, but the run's three numbers are not written
    // yet: the report names the missing record rather than calling it done.
    let body = body_of(&dir, "report");
    assert_eq!(body["status"], "incomplete");
    assert!(body["metrics"]["missing"]
        .as_str()
        .is_some_and(|said| said.contains("metrics.json")));
    let page = std::fs::read_to_string(dir.join("report.md")).unwrap();
    assert!(page.contains("## The run's numbers"));
    assert!(page.contains("Missing: metrics.json"), "{page}");

    metrics::write(&Paths::new(&dir), "oregon-white-oak").unwrap();
    assert!(matches!(
        report::run(&Paths::new(&dir)).unwrap(),
        report::Outcome::Ran { .. }
    ));
    let body = body_of(&dir, "report");
    assert_eq!(body["status"], "complete");
    assert_eq!(body["metrics"]["autonomy"]["gaps"], 0);
    let page = std::fs::read_to_string(dir.join("report.md")).unwrap();
    assert!(page.contains("0 gaps, 0 routed"), "{page}");
}

#[test]
fn the_reports_inputs_name_every_artifact_it_read() {
    let dir = scratch("report-inputs", 2);
    fit::run(&Paths::new(&dir)).unwrap();
    report::run(&Paths::new(&dir)).unwrap();
    let header = read_json(&dir.join("report.json")).unwrap();
    let recorded: BTreeMap<String, String> = serde_json::from_value(header["inputs"].clone())
        .expect("the header records the inputs it read");
    for name in [
        "fetch.json",
        "quality.json",
        "select.json",
        "fit.json",
        "decisions.json",
        "provenance.json",
    ] {
        assert!(recorded.contains_key(name), "{name} is not recorded");
    }
    assert!(
        !recorded.contains_key("generate.json"),
        "generate never ran"
    );
}
