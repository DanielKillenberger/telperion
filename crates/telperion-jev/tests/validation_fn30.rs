//! R7: the fetch and fit stages over the oak and spruce fixtures under
//! `.flow/evidence/fn58/validation`, compared with the expected fit read off
//! fn30's report. The dispositions of every difference live beside the
//! fixtures in `DISPOSITIONS.md`. Offline: no Jev, no network, no binary.

use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Value};
use telperion_jev::pipeline::adapter::FixtureAdapter;
use telperion_jev::pipeline::canon::read_json;
use telperion_jev::pipeline::stage::{Context, Paths};
use telperion_jev::pipeline::stages::{fetch, fit, inputs};

fn validation_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../.flow/evidence/fn58/validation")
}

/// A scratch pipeline directory holding the species' admitted manifest and
/// a quality artifact that passes its one field, so the fit runs.
fn prepare(species: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-validation-{species}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    fs::create_dir_all(&dir).unwrap();
    let source = validation_root().join(species);
    fs::copy(source.join("manifest.json"), dir.join("manifest.json")).unwrap();
    let adapter = FixtureAdapter::new(source.join("fixtures"));
    assert!(matches!(
        fetch::run(&Paths::new(&dir), &adapter).unwrap(),
        fetch::Outcome::Ran { decisions } if decisions.is_empty()
    ));
    let (ctx, _) = Context::open(&Paths::new(&dir), "quality").unwrap();
    let header = ctx.header("quality", "quality", inputs(&[]), vec![]);
    ctx.write(
        &header,
        json!({"fields": {"height_m": {"passed": true, "level": "proxy_only"}}}),
    )
    .unwrap();
    dir
}

fn close(actual: &Value, expected: &Value, tolerance: f64) -> bool {
    match (actual.as_f64(), expected.as_f64()) {
        (Some(a), Some(e)) => (a - e).abs() <= tolerance,
        _ => false,
    }
}

#[test]
fn the_fit_over_the_fixtures_reproduces_fn30_for_the_oak_and_the_spruce() {
    for species in ["oregon-white-oak", "norway-spruce"] {
        let dir = prepare(species);
        assert!(
            matches!(
                fit::run(&Paths::new(&dir)).unwrap(),
                fit::Outcome::Ran { .. }
            ),
            "{species}"
        );
        let body = read_json(&dir.join("fit.json")).unwrap()["body"].clone();
        let expected =
            read_json(&validation_root().join(species).join("expected/fit.json")).unwrap();
        assert!(
            close(&body["rate"], &expected["rate"], 1e-9),
            "{species} rate {}",
            body["rate"]
        );
        assert!(
            close(&body["shape"], &expected["shape"], 1e-9),
            "{species} shape {}",
            body["shape"]
        );
        assert_eq!(
            body["derived_mature_age_years"], expected["derived_mature_age_years"],
            "{species}"
        );
        for (row, want) in body["rows"]
            .as_array()
            .unwrap()
            .iter()
            .zip(expected["rows"].as_array().unwrap())
        {
            assert_eq!(row["age_years"], want["age_years"], "{species}");
            assert!(
                close(
                    &row["reference_height_m"],
                    &want["reference_height_m"],
                    0.02
                ),
                "{species} height at {}: {}",
                want["age_years"],
                row["reference_height_m"]
            );
            assert!(
                close(&row["reference_dbh_m"], &want["reference_dbh_m"], 0.001),
                "{species} dbh at {}: {}",
                want["age_years"],
                row["reference_dbh_m"]
            );
        }
        let misses: Vec<(String, String)> = body["misses"]
            .as_array()
            .unwrap()
            .iter()
            .map(|m| {
                (
                    m["field"].as_str().unwrap().to_string(),
                    m["age_years"].to_string(),
                )
            })
            .collect();
        let want_dbh: Vec<String> = expected["misses"]["dbh_m"]
            .as_array()
            .unwrap()
            .iter()
            .map(|a| a.as_str().unwrap().to_string())
            .collect();
        assert_eq!(
            misses.iter().filter(|(f, _)| f == "height_m").count(),
            0,
            "{species} height misses: {misses:?}"
        );
        let dbh_ages: Vec<String> = misses
            .iter()
            .filter(|(f, _)| f == "dbh_m")
            .map(|(_, a)| a.clone())
            .collect();
        assert_eq!(dbh_ages, want_dbh, "{species}");
        let decisions = read_json(&dir.join("decisions.json")).unwrap();
        let kinds: Vec<&str> = decisions["decisions"]
            .as_array()
            .unwrap()
            .iter()
            .map(|d| d["kind"].as_str().unwrap())
            .collect();
        assert_eq!(kinds, vec!["tolerance-miss"; 3], "{species}");
    }
}

/// Two isolated directories over the same fixtures and manifest produce
/// byte-identical decision lists and fit records.
#[test]
fn two_isolated_runs_over_the_same_fixtures_compare_identical() {
    let left = prepare("oregon-white-oak");
    let right = prepare("oregon-white-oak");
    fit::run(&Paths::new(&left)).unwrap();
    fit::run(&Paths::new(&right)).unwrap();
    for file in ["decisions.json", "fit.json"] {
        let bytes = |dir: &std::path::Path| std::fs::read(dir.join(file)).unwrap();
        assert_eq!(bytes(&left), bytes(&right), "{file}");
    }
}
