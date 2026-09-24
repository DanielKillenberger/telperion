//! fn-135: the conductor writes the overlay the sourced profile derives into
//! the tuning config before tuning revision 1, and again once the profile has
//! changed; a manual `initial_overrides` entry wins over a derived one, and a
//! metric the measurer cannot read stops gating the tuning profile.
use std::path::Path;
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::conductor::plan::Next;
use telperion_jev::conductor::questions::Asker;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{Executor, StageOutcome};
use telperion_jev::conductor::{overlay, tuning, Config};
use telperion_jev::pipeline::judge::Judge;

mod conductor_support;
use conductor_support::*;

fn fixture(name: &str) -> Value {
    let path = format!("{}/tests/fixtures/palm/{name}", env!("CARGO_MANIFEST_DIR"));
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}

/// Records the tuning config each revision starts from, then ends it.
struct Recording {
    root: std::path::PathBuf,
    seen: Mutex<Vec<Value>>,
}

impl Executor for Recording {
    fn stage(&self, _: &Config, _: &str) -> Result<StageOutcome, String> {
        Ok(StageOutcome::Ran)
    }
    fn search(&self, _: &Config) -> Result<String, String> {
        Err("no search here".into())
    }
    fn tune(
        &self,
        config: &Config,
        revision: u64,
        _: &[String],
        out: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        self.seen.lock().unwrap().push(read(&config.tuning_config));
        let ended = result(&self.root, &format!("run-{revision}"), false, vec![]);
        write(&out.join("result.json"), &json!(ended));
        Ok(())
    }
}

/// The beech scaffolding with the palm's tuning config, packet profile and
/// a tuning profile that still gates on the frond length.
fn palm_run(root: &Path) -> (Config, Recording) {
    let config = config(root);
    let tuning_profiles = root.join("tuning-profiles.json");
    let mut profiles = fixture("tuning-profiles.json");
    profiles["profiles"][0]["metrics"]["frond_length_m"] =
        fixture("profile.json")["profiles"][0]["metrics"]["frond_length_m"].clone();
    write(&tuning_profiles, &profiles);
    let mut tuning = fixture("tuning-config.json");
    tuning["profiles"] = json!(tuning_profiles);
    write(&config.tuning_config, &tuning);
    write(&config.paths().packet("profile"), &fixture("profile.json"));
    let executor = Recording {
        root: root.to_path_buf(),
        seen: Mutex::new(Vec::new()),
    };
    (config, executor)
}

#[test]
fn revision_one_starts_from_the_derived_overlay() {
    let root = scratch("overlay-first");
    let (config, executor) = palm_run(&root);
    let mut run = Run::open(&config).unwrap();
    let script = Script::new();
    let (_, next) = drive(&script, &config, &mut run, &executor);
    assert!(matches!(next, Next::Tune { revision: 1, .. }), "{next:?}");
    drive(&script, &config, &mut run, &executor);
    let seen = executor.seen.lock().unwrap();
    let overrides = &seen[0]["initial_overrides"];
    assert_eq!(overrides["material"]["barkRed"], json!(0.25));
    let height = overrides["skeleton"]["envelope"]["height"]
        .as_f64()
        .unwrap();
    assert!((height - 22.86).abs() < 1e-9, "{height}");
    assert_eq!(overrides["canopy"]["rachisLength"], json!(6.0));
    // Everything else in the tuning config is as the person wrote it.
    assert_eq!(seen[0]["preset"], "date-palm");
    let provenance = read(&overlay::provenance_file(&config));
    let bark = provenance["rows"]
        .as_array()
        .unwrap()
        .iter()
        .find(|r| r["path"] == "/material/barkRed")
        .unwrap();
    assert_eq!(bark["source"], "appearance.bark_colour.bark_red");
    assert_eq!(bark["formula"], "midpoint");
    let profiles = read(&root.join("tuning-profiles.json"));
    let metrics = &profiles["profiles"][0]["metrics"];
    assert_eq!(metrics["frond_length_m"]["classification"], "contextual");
    assert_eq!(metrics["height_m"]["classification"], "gating");
}

#[test]
fn a_changed_profile_is_derived_again_and_a_manual_entry_wins() {
    let root = scratch("overlay-again");
    let (config, executor) = palm_run(&root);
    overlay::refresh(&config).unwrap();
    // Refreshing an unchanged profile writes the same bytes.
    let first = std::fs::read(&config.tuning_config).unwrap();
    overlay::refresh(&config).unwrap();
    assert_eq!(std::fs::read(&config.tuning_config).unwrap(), first);
    // The person sets the bark by hand; the literature revises the height.
    let mut tuning = read(&config.tuning_config);
    tuning["initial_overrides"]["material"]["barkRed"] = json!(0.4);
    write(&config.tuning_config, &tuning);
    let mut profile = fixture("profile.json");
    profile["profiles"][0]["metrics"]["height_m"]["range"] = json!([20.0, 30.0]);
    write(&config.paths().packet("profile"), &profile);
    let mut run = Run::open(&config).unwrap();
    let script = Script::new();
    let asker = Asker {
        judge: Judge {
            transport: &script,
            key: "k",
            ledger_dir: config.ledger_dir(),
        },
        config: &config,
    };
    tuning::tune(&asker, &config, &mut run, &executor, 1, &[]).unwrap();
    let seen = executor.seen.lock().unwrap();
    let overrides = &seen[0]["initial_overrides"];
    assert_eq!(overrides["material"]["barkRed"], json!(0.4));
    assert_eq!(overrides["skeleton"]["envelope"]["height"], json!(25.0));
    let provenance = read(&overlay::provenance_file(&config));
    assert_eq!(provenance["manual"], json!(["/material/barkRed"]));
    // A second refresh keeps the manual entry as manual.
    drop(seen);
    overlay::refresh(&config).unwrap();
    let tuning = read(&config.tuning_config);
    assert_eq!(
        tuning["initial_overrides"]["material"]["barkRed"],
        json!(0.4)
    );
}
