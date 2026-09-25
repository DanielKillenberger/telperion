//! The species runner (fn-149): stages in order, reruns only on changed
//! content, the stops, a run up to a stage or of one alone, and the status.
use std::path::{Path, PathBuf};

use telperion_jev::pipeline::stage::Paths;
use telperion_jev::runner::record::State;
use telperion_jev::runner::{self, Done, Run, Scope, Stage, Stop, STAGES};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-runner-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("seed"), "a").unwrap();
    dir
}

fn run_in(dir: &Path) -> Run {
    Run::new(
        "test-tree",
        Paths::new(dir),
        dir.join("catalogue"),
        dir.join("tuning.json"),
    )
}

/// A stage double at `at` in the real order: it reads the previous stage's
/// file (the first reads `seed`), copies it to its own, and appends its name
/// to `ran`; claims, identity gaps and acceptance are files a test writes.
struct Fake(usize);

const FAKES: [&dyn Stage; 8] = [
    &Fake(0),
    &Fake(1),
    &Fake(2),
    &Fake(3),
    &Fake(4),
    &Fake(5),
    &Fake(6),
    &Fake(7),
];

fn file(dir: &Path, at: usize) -> PathBuf {
    dir.join(format!("{}.out", STAGES[at].name()))
}

fn ran(dir: &Path) -> Vec<String> {
    let read = std::fs::read_to_string(dir.join("ran")).unwrap_or_default();
    let _ = std::fs::remove_file(dir.join("ran"));
    read.lines().map(str::to_string).collect()
}

impl Stage for Fake {
    fn name(&self) -> &'static str {
        STAGES[self.0].name()
    }
    fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        let dir = &run.paths.dir;
        Ok(match self.0 {
            0 => vec![dir.join("seed")],
            n => vec![file(dir, n - 1)],
        })
    }
    fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
        Ok(vec![file(&run.paths.dir, self.0)])
    }
    fn run(&self, run: &Run) -> Result<Done, String> {
        let dir = &run.paths.dir;
        let log = std::fs::read_to_string(dir.join("ran")).unwrap_or_default();
        std::fs::write(dir.join("ran"), format!("{log}{}\n", self.name())).unwrap();
        let read = std::fs::read(&self.inputs(run)?[0]).unwrap_or_default();
        std::fs::write(file(dir, self.0), read).unwrap();
        Ok(Done::Ran("wrote".into()))
    }
    fn stop(&self, run: &Run) -> Result<Option<Stop>, String> {
        let dir = &run.paths.dir;
        Ok(match self.name() {
            "profile" if dir.join("claim").exists() => Some(Stop::Claims(vec!["c".into()])),
            "gaps" if dir.join("identity").exists() => Some(Stop::IdentityGaps(vec!["t".into()])),
            "accept" if !dir.join("accepted").exists() => Some(Stop::OwnerLook),
            _ => None,
        })
    }
}

fn all(run: &Run) -> Option<Stop> {
    runner::run(run, &FAKES, &Scope::All).unwrap()
}

#[test]
fn eight_stages_run_in_order_and_a_second_run_reruns_nothing() {
    let dir = scratch("order");
    let run = run_in(&dir);
    assert_eq!(all(&run), Some(Stop::OwnerLook));
    let names: Vec<&str> = STAGES.iter().map(|s| s.name()).collect();
    assert_eq!(ran(&dir), names);
    assert_eq!(all(&run), Some(Stop::OwnerLook));
    assert!(ran(&dir).is_empty());
}

#[test]
fn changed_content_reruns_its_readers_and_the_same_bytes_rerun_nothing() {
    let dir = scratch("content");
    let run = run_in(&dir);
    all(&run);
    ran(&dir);
    // A file rewritten with the same bytes is not a change.
    std::fs::write(dir.join("seed"), "a").unwrap();
    all(&run);
    assert!(ran(&dir).is_empty());
    // An edit to the start overlay reruns Start's readers, never Start.
    std::fs::write(file(&dir, 4), "edited").unwrap();
    all(&run);
    assert_eq!(ran(&dir), ["tune", "gaps", "accept"]);
    // An output removed by hand reruns the stage that writes it.
    std::fs::remove_file(file(&dir, 2)).unwrap();
    all(&run);
    assert_eq!(ran(&dir)[0], "capability");
}

#[test]
fn a_run_stops_for_claims_identity_gaps_and_the_owners_look_only() {
    let dir = scratch("stops");
    let run = run_in(&dir);
    std::fs::write(dir.join("claim"), "").unwrap();
    assert_eq!(all(&run), Some(Stop::Claims(vec!["c".into()])));
    assert_eq!(ran(&dir), ["sources", "profile"]);
    std::fs::remove_file(dir.join("claim")).unwrap();
    std::fs::write(dir.join("identity"), "").unwrap();
    assert_eq!(all(&run), Some(Stop::IdentityGaps(vec!["t".into()])));
    std::fs::remove_file(dir.join("identity")).unwrap();
    assert_eq!(all(&run), Some(Stop::OwnerLook));
    std::fs::write(dir.join("accepted"), "").unwrap();
    assert_eq!(all(&run), None);
    let log = std::fs::read_to_string(run.out().join("log.jsonl")).unwrap();
    assert!(log.contains("stopped: claims to settle"), "{log}");
}

#[test]
fn until_stops_after_its_stage_and_one_stage_runs_alone_once_its_inputs_exist() {
    let dir = scratch("scope");
    let run = run_in(&dir);
    let scoped = |scope| runner::run(&run, &FAKES, &scope);
    let err = scoped(Scope::Only("start".into())).unwrap_err();
    let wanted = format!(
        "start needs {}, which catalogue writes",
        file(&dir, 3).display()
    );
    assert!(err.starts_with(&wanted), "{err}");
    assert!(ran(&dir).is_empty());
    assert_eq!(scoped(Scope::Until("catalogue".into())).unwrap(), None);
    assert_eq!(ran(&dir), ["sources", "profile", "capability", "catalogue"]);
    assert_eq!(scoped(Scope::Only("start".into())).unwrap(), None);
    assert_eq!(ran(&dir), ["start"]);
    let err = scoped(Scope::Until("prune".into())).unwrap_err();
    assert!(err.starts_with("no stage prune"), "{err}");
}

#[test]
fn status_names_each_stage_current_stale_or_missing_and_runs_nothing() {
    let dir = scratch("status");
    let run = run_in(&dir);
    runner::run(&run, &FAKES, &Scope::Until("tune".into())).unwrap();
    ran(&dir);
    std::fs::write(file(&dir, 1), "edited").unwrap();
    let states = runner::status(&run, &FAKES).unwrap();
    assert!(ran(&dir).is_empty());
    let shown = |name: &str| states.iter().find(|(n, _)| *n == name).unwrap().1.clone();
    assert_eq!(shown("sources"), State::Current);
    assert_eq!(shown("profile"), State::Current);
    let edited = file(&dir, 1).display().to_string();
    assert_eq!(
        shown("capability"),
        State::Stale(format!("changed: {edited}"))
    );
    assert_eq!(shown("gaps"), State::Missing("never ran".into()));
}

#[test]
fn a_failed_stage_names_itself_and_is_not_recorded() {
    struct Failing;
    impl Stage for Failing {
        fn name(&self) -> &'static str {
            "sources"
        }
        fn inputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
            Ok(vec![run.paths.dir.join("seed")])
        }
        fn outputs(&self, run: &Run) -> Result<Vec<PathBuf>, String> {
            Ok(vec![run.paths.dir.join("never")])
        }
        fn run(&self, _: &Run) -> Result<Done, String> {
            Err("tool failed".into())
        }
        fn stop(&self, _: &Run) -> Result<Option<Stop>, String> {
            Ok(None)
        }
    }
    let dir = scratch("failed");
    let run = run_in(&dir);
    let err = runner::run(&run, &[&Failing], &Scope::All).unwrap_err();
    assert_eq!(err, "sources: tool failed");
    let records = std::fs::read_to_string(run.out().join("records.json"));
    assert!(records.is_err(), "a failed stage wrote a record");
    let log = std::fs::read_to_string(run.out().join("log.jsonl")).unwrap();
    assert!(log.contains("failed: tool failed"));
}
