//! fn-149 R6: the runner's proof is a recorded run replayed offline. The
//! European beech was recorded live from a bare seed on 2026-09-25
//! (`species european-beech --record`); this test replays it from the same
//! bare seed through Start with no network and no key, and a second run
//! reruns nothing. Every Firecrawl, Jev, Commons and vision answer comes from
//! `tests/fixtures/replay/european-beech/tape`; a request the recording
//! lacks fails the run, naming it. The beech's values are not the proof
//! (fn-157 replaces the literature stages); the machinery is.
use std::path::{Path, PathBuf};
use std::process::Command;

use serde_json::Value;

fn repo() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay/european-beech")
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
fn seeded(dir: &Path) {
    let folder = dir.join("catalogue/european-beech/packet");
    std::fs::create_dir_all(&folder).unwrap();
    let seed = fixture().join("seed");
    std::fs::copy(
        seed.join("manifest.json"),
        dir.join("catalogue/european-beech/manifest.json"),
    )
    .unwrap();
    std::fs::copy(
        seed.join("packet/capability.json"),
        folder.join("capability.json"),
    )
    .unwrap();
    let mut tuning: Value =
        serde_json::from_slice(&std::fs::read(fixture().join("tuning.json")).unwrap()).unwrap();
    let at = |name: &str| dir.join(name).display().to_string();
    tuning["profiles"] = at("profiles-european-beech.json").into();
    tuning["ledger"] = at("run/ledger").into();
    tuning["vision"]["ledger"] = at("run/vision-ledger").into();
    tuning["sheet"]["adapter"]["ledger"] = at("run/vision-ledger").into();
    std::fs::write(dir.join("tuning.json"), tuning.to_string()).unwrap();
}

/// One replayed run through Start, which must succeed: each stage's line.
fn replay(dir: &Path) -> Vec<String> {
    let out = Command::new(env!("CARGO_BIN_EXE_species"))
        .current_dir(repo())
        .env_remove("TYPESAFE_API_KEY")
        .env_remove("FIRECRAWL_API_KEY")
        .args(["european-beech", "--until", "start", "--replay"])
        .arg(fixture().join("tape"))
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

#[test]
fn the_recorded_beech_replays_offline_through_start_and_a_second_run_reruns_nothing() {
    let dir = std::env::temp_dir().join(format!(
        "jev-replay-beech-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    seeded(&dir);
    let lines = replay(&dir);
    let word = |stage: &str| {
        lines
            .iter()
            .find(|l| l.starts_with(&format!("{stage}: ")))
            .cloned()
            .unwrap_or_default()
    };
    assert!(
        word("sources").starts_with("sources: ran: discover ran"),
        "{lines:?}"
    );
    let profile = word("profile");
    assert!(
        profile.starts_with("profile: ran: extract ran"),
        "{profile}"
    );
    assert!(
        profile.contains("reference photographs: 12 candidates, 11 open-licence, 1 kept"),
        "{profile}"
    );
    assert!(profile.contains("reference inventory built"), "{profile}");
    assert!(
        word("capability").starts_with("capability: ran: gate ran"),
        "{lines:?}"
    );
    assert!(
        word("catalogue").starts_with("catalogue: ran: generate ran, gate ran, document ran"),
        "{lines:?}"
    );
    assert!(
        word("start").starts_with("start: ran: derived"),
        "{lines:?}"
    );
    assert_eq!(lines.last().map(String::as_str), Some("reached start"));

    let again = replay(&dir);
    let stages = ["sources", "profile", "capability", "catalogue", "start"];
    let current: Vec<String> = stages.iter().map(|s| format!("{s}: current")).collect();
    assert_eq!(again[..stages.len()], current[..], "{again:?}");
}

/// The repository is public: a committed recording republishes no page that
/// is not openly licensed. Each such page keeps only the passages the run
/// quoted to Jev (`tape::trim`), and its bytes only its licence statements.
#[test]
fn the_recording_keeps_only_the_passages_the_run_quoted() {
    let over = telperion_jev::tape::trim::only_quoted(&fixture().join("tape"), &[]).unwrap();
    assert!(over.is_empty(), "{over:#?}");
}
