//! fn-157, host decision 1: a value comes from the best kind of source that
//! agrees. On the recorded beech pages (`fixtures/tiers`: the Woodland Trust,
//! Morton Arboretum, NC State and Oregon State pages the 2026-09-26 run
//! quoted, and the EUFORGEN guideline's height passage the 2026-09-25 run
//! read) the height is the forestry tier's: the Woodland Trust's 40 m and
//! EUFORGEN's 30-35 m agree, and the American landscape pages below them
//! fill nothing the forestry tier states. Start takes the median (host
//! decision 4). No network, no key.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_core::params;
use telperion_core::presets::Preset;
use telperion_jev::caller::{HttpRequest, HttpResponse, Transport};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::{read_json, write_canonical};
use telperion_jev::pipeline::judge::Judge;
use telperion_jev::pipeline::requirements::table;
use telperion_jev::pipeline::stage::Paths;
use telperion_jev::pipeline::stages::gather::GATHERED;
use telperion_jev::pipeline::stages::{aggregate, fetch, read};
use telperion_jev::runner::derive;

const PAGES: [(&str, &str, &str); 5] = [
    (
        "P1",
        "https://www.woodlandtrust.org.uk/trees-woods-and-wildlife/british-trees/a-z-of-british-trees/common-beech/",
        "forestry",
    ),
    (
        "P2",
        "https://www.euforgen.org/uploads/tx_news/1322_European_beech__Fagus_sylvatica_.pdf",
        "forestry",
    ),
    (
        "P3",
        "https://mortonarb.org/plant-and-protect/trees-and-plants/european-beech/",
        "garden",
    ),
    ("P4", "https://plants.ces.ncsu.edu/plants/fagus-sylvatica/", "extension"),
    (
        "P5",
        "https://landscapeplants.oregonstate.edu/plants/fagus-sylvatica",
        "extension",
    ),
];

/// The spans a person reads on these pages: (a word of the host, span,
/// field, basis). Every other span measures nothing asked.
const SPANS: [(&str, &str, &str, &str); 10] = [
    ("woodlandtrust", "40m", "height_m", "typical"),
    ("euforgen", "30–35 m", "height_m", "typical"),
    ("mortonarb", "50-60 feet", "height_m", "typical"),
    ("mortonarb", "50 to 60 feet", "height_m", "typical"),
    ("ncsu", "50 to 60 feet", "height_m", "typical"),
    ("ncsu", "50 ft", "height_m", "typical"),
    ("oregonstate", "50-75 ft", "height_m", "typical"),
    ("oregonstate", "15-23 m", "height_m", "typical"),
    ("oregonstate", "40-60 ft", "crown_width_m", "typical"),
    ("oregonstate", "12-18 m", "crown_width_m", "typical"),
];

/// Jev as a person reads these pages: each document's kind, each span's
/// label, and every appearance trait unstated.
struct Reader;

impl Transport for Reader {
    fn send(&self, request: &HttpRequest) -> Result<HttpResponse, String> {
        let body: Value = serde_json::from_slice(request.body.as_deref().unwrap_or(b"{}")).unwrap();
        let (questions, state) = (&body["questions"], &body["state"]);
        let choice = |key: &str| json!({"type": "choice", "choice": key, "confidence": 0.9, "probabilities": {key: 0.9}});
        let url = state["source"]["url"].as_str().unwrap_or_default();
        let answers = if questions.get("document").is_some() {
            let kind = PAGES.iter().find(|p| p.1 == url).map_or("other", |p| p.2);
            json!({"document": choice(kind)})
        } else if questions.get("field").is_some() {
            let span = state["candidate"]["span"].as_str().unwrap_or_default();
            let (field, basis) = SPANS
                .iter()
                .find(|s| url.contains(s.0) && s.1 == span)
                .map_or(("none", "unclear"), |s| (s.2, s.3));
            json!({"field": choice(field), "basis": choice(basis), "age": choice("mature"),
                   "condition": choice("unstated")})
        } else {
            let name = state["trait"].as_str().unwrap_or_default();
            let unstated = table().levels(name).map_or(0, |l| l.len()).to_string();
            json!({"level": {"type": "score", "score": 0.0, "confidence": 0.9, "probabilities": {unstated: 0.9}}})
        };
        Ok(HttpResponse {
            status: 200,
            body: serde_json::to_vec(&json!({"model": "jev-latest", "answers": answers})).unwrap(),
        })
    }
}

fn beech() -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-tiers-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let seed = root.join("tests/fixtures/replay/european-beech/seed/manifest.json");
    let mut manifest = read_json(&seed).unwrap();
    let sources: Vec<Value> = PAGES
        .iter()
        .map(|(id, url, _)| json!({"id": id, "url": url, "title": id, "rights": GATHERED}))
        .collect();
    manifest["sources"] = json!(sources);
    write_canonical(&dir.join("manifest.json"), &manifest).unwrap();
    let paths = Paths::new(&dir);
    fetch::run(
        &paths,
        &FixtureAdapter::new(root.join("tests/fixtures/tiers")),
    )
    .unwrap();
    let judge = Judge {
        transport: &Reader,
        key: "test-key",
        ledger_dir: dir.join("ledger"),
    };
    read::run(&paths, &judge).unwrap();
    aggregate::run(&paths, &judge).unwrap();
    dir
}

fn metric(dir: &Path, field: &str) -> Value {
    read_json(&dir.join("packet/profile.json")).unwrap()["profiles"][0]["metrics"][field].clone()
}

/// The Woodland Trust's 40 m decides the height with EUFORGEN's 30-35 m;
/// the landscape pages' 50-75 ft never enter it.
#[test]
fn the_best_tier_that_agrees_decides_the_height() {
    let dir = beech();
    let height = metric(&dir, "height_m");
    assert_eq!(height["tier"], "forestry", "{height}");
    assert_eq!(height["source"], json!(["P1", "P2"]), "{height}");
    assert!(
        (height["value"].as_f64().unwrap() - 36.25).abs() < 1e-9,
        "{height}"
    );
    assert_eq!(height["confidence"], "agreed", "{height}");
    assert_eq!(height["range"], json!([30.0, 40.0]), "{height}");
}

/// A field no higher tier states is the lower tier's: the crown width is
/// Oregon State's alone, and says so.
#[test]
fn a_lower_tier_fills_only_what_the_higher_ones_leave_empty() {
    let dir = beech();
    let crown = metric(&dir, "crown_width_m");
    assert_eq!(crown["tier"], "extension", "{crown}");
    assert_eq!(crown["source"], json!(["P5"]), "{crown}");
    assert_eq!(crown["confidence"], "thin", "{crown}");
}

/// Host decision 4: Start takes the aggregate's typical value, the median,
/// never the midpoint of its range.
#[test]
fn start_takes_the_median_never_the_midpoint_of_the_range() {
    let dir = beech();
    let profile = read_json(&dir.join("packet/profile.json")).unwrap();
    let family = params::metadata(&Preset::from_id("european-beech").unwrap().parameters());
    let derived = derive::derive(&profile["profiles"][0], &family).unwrap();
    let height = derived
        .rows
        .iter()
        .find(|r| r.path == "/skeleton/envelope/height")
        .unwrap_or_else(|| panic!("no height row: {derived:?}"));
    assert!((height.value - 36.25).abs() < 1e-9, "{height:?}");
}

/// A run from a name has no profile manifest for the measurer: Start writes
/// the profile the tree was derived from where the tuning config names it,
/// so Tune gates on it (fn-157; the beech's first Tune revision found none).
#[test]
fn start_gives_the_measurer_the_profile_it_derived_from() {
    let dir = beech();
    let profiles = dir.join("profiles-european-beech.json");
    let tuning = dir.join("tuning.json");
    write_canonical(
        &tuning,
        &json!({"preset": "european-beech", "profile_id": "european-beech",
                "profiles": profiles.display().to_string(), "initial_overrides": {}}),
    )
    .unwrap();
    let packet = dir.join("packet/profile.json");
    telperion_jev::runner::start::run(&packet, &tuning, &dir.join("out")).unwrap();
    let written = read_json(&profiles).unwrap();
    assert_eq!(written["profiles"][0]["id"], "european-beech");
    // Frozen and ready, as the measurer requires; the packet stays draft.
    assert_eq!(written["status"], "ready");
    assert!(written["frozen_at"].is_string());
    assert_eq!(written["profiles"][0]["readiness"], "ready");
    for (key, metric) in written["profiles"][0]["metrics"].as_object().unwrap() {
        let class = metric["classification"].as_str().unwrap();
        assert!(class == "gating" || class == "contextual", "{key}: {class}");
    }
    assert_eq!(
        read_json(&packet).unwrap()["profiles"][0]["readiness"],
        "draft"
    );
    assert_eq!(
        written["profiles"][0]["metrics"]["height_m"]["value"],
        read_json(&packet).unwrap()["profiles"][0]["metrics"]["height_m"]["value"]
    );
}

/// Host decision of 2026-09-26: a value gates only when its deciding tier
/// agrees. The height's two agreeing forestry sources gate; the crown width,
/// one extension site alone, is contextual. Each says why.
#[test]
fn a_value_gates_only_when_its_tier_agrees() {
    let dir = beech();
    let height = metric(&dir, "height_m");
    assert_eq!(height["classification"], "gating", "{height}");
    assert!(height["classified"].as_str().unwrap().contains("agree"), "{height}");
    let crown = metric(&dir, "crown_width_m");
    assert_eq!(crown["classification"], "contextual", "{crown}");
    assert!(crown["classified"].as_str().unwrap().contains("one"), "{crown}");
}
