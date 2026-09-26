//! fn-149 R6, fn-157 R1 and R5: the runner's proof is a recorded run
//! replayed offline. The European beech and the Oregon white oak were
//! recorded live from bare seeds on 2026-09-26 (`species <id> --record`);
//! these tests replay each from the same bare seed through Start with no
//! network and no key, and a second run reruns nothing. Every Firecrawl,
//! Jev, Commons and vision answer comes from
//! `tests/fixtures/replay/<id>/tape`; a request the recording lacks fails
//! the run, naming it.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

const BEECH: &str = "european-beech";
const OAK: &str = "oregon-white-oak";

fn fixture(species: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/replay")
        .join(species)
}

/// The render examples the workspace build made beside this binary.
fn tools() -> PathBuf {
    let bin = Path::new(env!("CARGO_BIN_EXE_species"));
    let dir = bin.parent().unwrap().join("examples");
    for tool in ["species_measure", "geometry_benchmark", "headless"] {
        assert!(
            dir.join(tool).exists(),
            "{} is missing: run the workspace gate, which builds the examples",
            dir.join(tool).display()
        );
    }
    dir
}

/// A run directory holding the bare seed, the host's capability assessment
/// and the recorded tuning config pointed at it.
fn seeded(dir: &Path, species: &str) {
    let folder = dir.join("catalogue").join(species).join("packet");
    std::fs::create_dir_all(&folder).unwrap();
    let seed = fixture(species).join("seed");
    std::fs::copy(
        seed.join("manifest.json"),
        folder.parent().unwrap().join("manifest.json"),
    )
    .unwrap();
    std::fs::copy(
        seed.join("packet/capability.json"),
        folder.join("capability.json"),
    )
    .unwrap();
    let mut tuning: Value =
        serde_json::from_slice(&std::fs::read(fixture(species).join("tuning.json")).unwrap())
            .unwrap();
    let at = |name: &str| dir.join(name).display().to_string();
    tuning["profiles"] = at(&format!("profiles-{species}.json")).into();
    tuning["ledger"] = at("run/ledger").into();
    tuning["vision"]["ledger"] = at("run/vision-ledger").into();
    tuning["sheet"]["adapter"]["ledger"] = at("run/vision-ledger").into();
    std::fs::write(dir.join("tuning.json"), tuning.to_string()).unwrap();
}

/// One replayed run through Start, which must succeed: each stage's line.
fn replay(dir: &Path, species: &str) -> Vec<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_species"))
        .current_dir(repo())
        .env_remove("TYPESAFE_API_KEY")
        .env_remove("FIRECRAWL_API_KEY")
        .args([species, "--until", "start", "--replay"])
        .arg(fixture(species).join("tape"))
        .arg("--catalogue")
        .arg(dir.join("catalogue"))
        .arg("--run-dir")
        .arg(dir.join("run"))
        .arg("--tuning")
        .arg(dir.join("tuning.json"))
        .arg("--tools")
        .arg(tools())
        .output()
        .unwrap();
    let printed = String::from_utf8_lossy(&out.stdout).into_owned();
    assert!(
        out.status.success(),
        "{printed}{}",
        String::from_utf8_lossy(&out.stderr)
    );
    printed.lines().map(str::to_string).collect()
}

/// A scratch directory seeded for `species`, replayed through Start; the
/// stage lines and the replayed profile's metrics. A second replay reruns
/// nothing.
fn replayed(species: &str) -> (PathBuf, Vec<String>, Value) {
    let dir = std::env::temp_dir().join(format!(
        "jev-replay-{species}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    seeded(&dir, species);
    let lines = replay(&dir, species);
    for (stage, ran) in [
        ("profile", "profile: ran: read ran, aggregate ran"),
        ("capability", "capability: ran: gate ran"),
        (
            "catalogue",
            "catalogue: ran: generate ran, gate ran, document ran",
        ),
        ("start", "start: ran: derived"),
    ] {
        assert!(word(&lines, stage).starts_with(ran), "{lines:?}");
    }
    assert_eq!(lines.last().map(String::as_str), Some("reached start"));
    let again = replay(&dir, species);
    let stages = ["sources", "profile", "capability", "catalogue", "start"];
    let current: Vec<String> = stages.iter().map(|s| format!("{s}: current")).collect();
    assert_eq!(again[..stages.len()], current[..], "{again:?}");
    let path = dir
        .join("catalogue")
        .join(species)
        .join("packet/profile.json");
    let profile: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let metrics = profile["profiles"][0]["metrics"].clone();
    (dir, lines, metrics)
}

fn word(lines: &[String], stage: &str) -> String {
    let prefix = format!("{stage}: ");
    lines
        .iter()
        .find(|l| l.starts_with(&prefix))
        .cloned()
        .unwrap_or_default()
}

/// R1 on the beech from its native range: the height is the forestry
/// tier's, three agreeing sources, and gates; the trunk diameter rests on
/// one nursery page and is contextual (host decision, 2026-09-26). R7: the
/// photograph search kept a mature, open-grown tree in leaf.
#[test]
fn the_recorded_beech_replays_offline_through_start_and_a_second_run_reruns_nothing() {
    let (dir, lines, metrics) = replayed(BEECH);
    assert!(
        word(&lines, "sources").starts_with("sources: ran: gather ran (24 documents), fetch ran"),
        "{lines:?}"
    );
    let profile = word(&lines, "profile");
    assert!(
        profile.contains("reference photographs: 12 candidates, 7 open-licence, 1 kept"),
        "{profile}"
    );
    let height = &metrics["height_m"];
    assert_eq!(height["tier"], "forestry", "{height}");
    assert_eq!(height["sources_agreeing"], 3, "{height}");
    assert_eq!(height["classification"], "gating", "{height}");
    let dbh = &metrics["dbh_m"];
    assert_eq!(dbh["range"], serde_json::json!([1.5, 1.5]), "{dbh}");
    assert_eq!(dbh["tier"], "nursery", "{dbh}");
    assert_eq!(dbh["classification"], "contextual", "{dbh}");
    let path = dir.join("catalogue/european-beech/packet/references.json");
    let references: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let views: Vec<&str> = references["references"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|r| r["view"].as_str())
        .collect();
    assert!(views.contains(&"leaf-on"), "{views:?}");
}

/// R5: the Oregon white oak, a shipped species, run from its bare seed:
/// every value the aggregate settled lands within the catalogue profile's
/// range for that field.
#[test]
fn the_recorded_oak_replays_offline_and_lands_within_its_catalogue_ranges() {
    let (_, lines, metrics) = replayed(OAK);
    assert!(
        word(&lines, "sources").starts_with("sources: ran: gather ran (22 documents), fetch ran"),
        "{lines:?}"
    );
    let path = repo().join("catalogue/oregon-white-oak/packet/profile.json");
    let catalogue: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let shipped = &catalogue["profiles"][0]["metrics"];
    let mut compared = 0;
    for (field, shipped_as) in [
        ("height_m", "height_m"),
        ("dbh_m", "dbh_m"),
        ("leaf_length_m", "foliage_length_m"),
        ("leaf_width_m", "foliage_width_m"),
    ] {
        let (Some(value), Some(range)) = (
            metrics[field]["value"].as_f64(),
            shipped[shipped_as]["range"].as_array(),
        ) else {
            continue;
        };
        let (lo, hi) = (range[0].as_f64().unwrap(), range[1].as_f64().unwrap());
        assert!(
            (lo..=hi).contains(&value),
            "{field} {value} outside {lo}..{hi}"
        );
        compared += 1;
    }
    assert_eq!(compared, 3, "{metrics}");
}

/// The repository is public: a committed recording republishes no page that
/// is not openly licensed. Each such page keeps only the passages the run
/// quoted to Jev (`tape::trim`), and its bytes only its licence statements.
#[test]
fn the_recordings_keep_only_the_passages_the_runs_quoted() {
    for species in [BEECH, OAK] {
        let tape = fixture(species).join("tape");
        let over = telperion_jev::tape::trim::only_quoted(&tape, &[]).unwrap();
        assert!(over.is_empty(), "{species}: {over:#?}");
    }
}
