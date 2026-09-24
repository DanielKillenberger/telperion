//! fn-135: a species' first tuned tree starts from its sourced profile. The
//! derivation table turns the profile's sizes and appearance into wire
//! values, clamped to each dial and recorded with their source; the table
//! names no species, and a metric the measurer cannot read never gates.
use serde_json::{json, Value};
use telperion_core::params;
use telperion_core::presets::{Preset, CATALOGUE, IN_WORK};
use telperion_jev::conductor::derive::{self, Derived, MEASURED, TABLE_JSON};

fn fixture(name: &str) -> Value {
    let path = format!("{}/tests/fixtures/palm/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

fn repo_profile(path: &str) -> Value {
    let path = format!("{}/../../{path}", env!("CARGO_MANIFEST_DIR"));
    let manifest: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    manifest["profiles"][0].clone()
}

fn family(preset: &str) -> Value {
    params::metadata(&Preset::from_id(preset).unwrap().parameters())
}

fn palm() -> Value {
    fixture("profile.json")["profiles"][0].clone()
}

fn value(derived: &Derived, path: &str) -> f64 {
    derived
        .rows
        .iter()
        .find(|r| r.path == path)
        .unwrap_or_else(|| panic!("{path} not derived: {derived:?}"))
        .value
}

#[test]
fn the_palm_derives_its_colours_and_the_sizes_the_table_maps() {
    let derived = derive::derive(&palm(), &family("date-palm")).unwrap();
    let expected = [
        ("/material/barkRed", 0.25),
        ("/material/barkGreen", 0.235),
        ("/material/barkBlue", 0.21),
        ("/material/barkRoughness", 0.965),
        ("/material/leafFrontRed", 0.12),
        ("/material/leafFrontGreen", 0.16),
        ("/material/leafFrontBlue", 0.11),
        ("/material/leafBackRed", 0.17),
        ("/material/leafBackGreen", 0.21),
        ("/material/leafBackBlue", 0.16),
        ("/material/hueRangeLow", -0.0275),
        ("/material/hueRangeHigh", 0.0275),
        ("/material/brightnessRangeLow", -0.04),
        ("/material/brightnessRangeHigh", 0.04),
        ("/skeleton/envelope/height", 22.86),
        ("/radii/trunkRadius", 0.01),
        ("/canopy/rachisLength", 6.0),
        ("/element/length", 0.4572),
        ("/element/width", 0.02),
    ];
    for (path, want) in expected {
        let got = value(&derived, path);
        assert!((got - want).abs() < 1e-9, "{path}: {got} != {want}");
    }
    assert_eq!(derived.rows.len(), expected.len(), "{derived:?}");
    // Every value names its profile entry, its citations and its formula.
    let bark = derived
        .rows
        .iter()
        .find(|r| r.path == "/material/barkRed")
        .unwrap();
    assert_eq!(bark.source, "appearance.bark_colour.bark_red");
    assert_eq!(bark.sources, ["A1"]);
    assert_eq!(bark.formula, "midpoint");
    let trunk = derived
        .rows
        .iter()
        .find(|r| r.path == "/radii/trunkRadius")
        .unwrap();
    assert_eq!(trunk.source, "metrics.dbh_m");
    assert_eq!(trunk.formula, "ratio");
    // What the table leaves out is recorded with its reason.
    let skipped: Vec<&str> = derived.skipped.iter().map(|s| s.source.as_str()).collect();
    assert_eq!(
        skipped,
        ["metrics.crown_width_m", "metrics.height_growth_m_per_year"]
    );
}

#[test]
fn a_value_past_its_dial_is_clamped_and_says_so() {
    let mut profile = palm();
    profile["metrics"]["height_m"]["range"] = json!([60.0, 80.0]);
    let derived = derive::derive(&profile, &family("date-palm")).unwrap();
    let height = derived
        .rows
        .iter()
        .find(|r| r.path == "/skeleton/envelope/height")
        .unwrap();
    assert_eq!(height.value, 40.5);
    assert_eq!(height.clamped_from, Some(70.0));
}

#[test]
fn the_table_names_no_species_and_its_conditions_read_the_family() {
    let table: Value = serde_json::from_str(TABLE_JSON).unwrap();
    let text = TABLE_JSON.to_lowercase();
    for (_, id, name, scientific) in CATALOGUE.iter().chain(IN_WORK) {
        for word in [id, name, scientific] {
            let word = word.to_lowercase();
            assert!(!text.contains(&word), "the table names {word}");
        }
    }
    for row in table["rows"].as_array().unwrap() {
        if let Some(when) = row.get("when") {
            let path = when["path"].as_str().unwrap();
            assert!(family("ordinary").pointer(path).is_some(), "{path}");
        }
    }
}

#[test]
fn a_broadleaf_derives_without_touching_the_rosette_rows() {
    for (profile, preset) in [
        (
            ".flow/evidence/fn19/onboarding-examples/oregon-white-oak/profile.json",
            "oregon-white-oak",
        ),
        ("catalogue/european-ash/packet/profile.json", "ordinary"),
    ] {
        let derived = derive::derive(&repo_profile(profile), &family(preset)).unwrap();
        assert!(
            derived
                .rows
                .iter()
                .all(|r| r.path != "/canopy/rachisLength" && !r.source.contains("leaflet")),
            "{profile}: {derived:?}"
        );
        assert!(value(&derived, "/skeleton/envelope/height") > 0.0);
        assert!(value(&derived, "/element/length") > 0.0);
    }
    // The ash's crown width reaches the envelope: 27.5 m wide on 27.5 m tall.
    let ash = derive::derive(
        &repo_profile("catalogue/european-ash/packet/profile.json"),
        &family("ordinary"),
    )
    .unwrap();
    assert!((value(&ash, "/skeleton/envelope/spread") - 0.5).abs() < 1e-9);
}

#[test]
fn a_metric_the_measurer_cannot_read_is_contextual() {
    let mut profile = palm();
    let changed = derive::measurable(&mut profile);
    assert_eq!(
        changed,
        [
            "frond_length_m",
            "height_growth_m_per_year",
            "leaflet_length_m",
            "leaflet_width_m"
        ]
    );
    let metrics = profile["metrics"].as_object().unwrap();
    for (key, metric) in metrics {
        if metric["classification"] == "gating" {
            assert!(MEASURED.contains(&key.as_str()), "{key} gates unread");
        }
    }
    for key in ["height_m", "dbh_m", "crown_width_m"] {
        assert_eq!(metrics[key]["classification"], "gating", "{key}");
    }
    // A second pass changes nothing.
    assert!(derive::measurable(&mut profile).is_empty());
}

#[test]
fn every_metric_on_the_list_is_one_the_measurer_writes() {
    let source = std::fs::read_to_string(format!(
        "{}/../telperion-core/examples/species_metrics/mod.rs",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap();
    for key in MEASURED {
        assert!(source.contains(&format!("\"{key}\"")), "{key}");
    }
}
