//! The species runner (fn-149): stages in order, reruns only on changed
//! content, and the three stops.
use std::cell::RefCell;
use std::path::{Path, PathBuf};

use telperion_jev::pipeline::stage::Paths;
use telperion_jev::runner::{self, Done, Run, Stage, Stages, Stop, STAGES};

fn scratch(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!(
        "jev-runner-{name}-{}-{}",
        std::process::id(),
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    dir
}

fn run_in(dir: &Path) -> Run {
    Run {
        species: "test-tree".into(),
        paths: Paths::new(dir),
        catalogue: dir.join("catalogue"),
        tuning: dir.join("tuning.json"),
        adapter: "fixture:none".into(),
        accept: false,
    }
}

/// Each stage reads the previous stage's file (the first reads `seed`) and
/// writes its own; claims, identity gaps and acceptance are files a test
/// writes.
#[derive(Default)]
struct Fake {
    ran: RefCell<Vec<&'static str>>,
}

fn file(dir: &Path, stage: Stage) -> PathBuf {
    dir.join(format!("{}.out", stage.name()))
}

impl Stages for Fake {
    fn inputs(&self, run: &Run, stage: Stage) -> Result<Vec<PathBuf>, String> {
        let dir = &run.paths.dir;
        let at = STAGES.iter().position(|s| *s == stage).unwrap();
        Ok(match at {
            0 => vec![dir.join("seed")],
            n => vec![file(dir, STAGES[n - 1])],
        })
    }
    fn outputs(&self, run: &Run, stage: Stage) -> Vec<PathBuf> {
        vec![file(&run.paths.dir, stage)]
    }
    fn run(&self, run: &Run, stage: Stage) -> Result<Done, String> {
        self.ran.borrow_mut().push(stage.name());
        let read = std::fs::read(&self.inputs(run, stage)?[0]).unwrap_or_default();
        std::fs::write(file(&run.paths.dir, stage), read).unwrap();
        Ok(Done::Ran("wrote".into()))
    }
    fn stop(&self, run: &Run, stage: Stage) -> Result<Option<Stop>, String> {
        let dir = &run.paths.dir;
        Ok(match stage {
            Stage::Profile if dir.join("claim").exists() => Some(Stop::Claims(vec!["c".into()])),
            Stage::Gaps if dir.join("identity").exists() => {
                Some(Stop::IdentityGaps(vec!["t".into()]))
            }
            Stage::Accept if !dir.join("accepted").exists() => Some(Stop::OwnerLook),
            _ => None,
        })
    }
}

#[test]
fn eight_stages_run_in_order_and_a_second_run_reruns_nothing() {
    let dir = scratch("order");
    std::fs::write(dir.join("seed"), "a").unwrap();
    let fake = Fake::default();
    let run = run_in(&dir);
    assert_eq!(runner::run(&run, &fake).unwrap(), Some(Stop::OwnerLook));
    let names: Vec<&str> = STAGES.iter().map(|s| s.name()).collect();
    assert_eq!(*fake.ran.borrow(), names);
    fake.ran.borrow_mut().clear();
    assert_eq!(runner::run(&run, &fake).unwrap(), Some(Stop::OwnerLook));
    assert!(fake.ran.borrow().is_empty(), "{:?}", fake.ran.borrow());
}

#[test]
fn changed_content_reruns_its_readers_and_the_same_bytes_rerun_nothing() {
    let dir = scratch("content");
    std::fs::write(dir.join("seed"), "a").unwrap();
    let fake = Fake::default();
    let run = run_in(&dir);
    runner::run(&run, &fake).unwrap();
    // A file rewritten with the same bytes is not a change.
    std::fs::write(dir.join("seed"), "a").unwrap();
    fake.ran.borrow_mut().clear();
    runner::run(&run, &fake).unwrap();
    assert!(fake.ran.borrow().is_empty());
    // An edit to the start overlay reruns Start's readers, never Start.
    std::fs::write(file(&dir, Stage::Start), "edited").unwrap();
    runner::run(&run, &fake).unwrap();
    assert_eq!(*fake.ran.borrow(), vec!["tune", "gaps", "accept"]);
    // An output removed by hand reruns the stage that writes it.
    fake.ran.borrow_mut().clear();
    std::fs::remove_file(file(&dir, Stage::Capability)).unwrap();
    runner::run(&run, &fake).unwrap();
    assert_eq!(fake.ran.borrow()[0], "capability");
}

#[test]
fn a_run_stops_for_claims_identity_gaps_and_the_owners_look_only() {
    let dir = scratch("stops");
    std::fs::write(dir.join("seed"), "a").unwrap();
    let fake = Fake::default();
    let run = run_in(&dir);
    std::fs::write(dir.join("claim"), "").unwrap();
    assert_eq!(
        runner::run(&run, &fake).unwrap(),
        Some(Stop::Claims(vec!["c".into()]))
    );
    assert_eq!(*fake.ran.borrow(), vec!["sources", "profile"]);
    std::fs::remove_file(dir.join("claim")).unwrap();
    std::fs::write(dir.join("identity"), "").unwrap();
    assert_eq!(
        runner::run(&run, &fake).unwrap(),
        Some(Stop::IdentityGaps(vec!["t".into()]))
    );
    std::fs::remove_file(dir.join("identity")).unwrap();
    assert_eq!(runner::run(&run, &fake).unwrap(), Some(Stop::OwnerLook));
    std::fs::write(dir.join("accepted"), "").unwrap();
    assert_eq!(runner::run(&run, &fake).unwrap(), None);
    let log = std::fs::read_to_string(run.out().join("log.jsonl")).unwrap();
    assert!(log.contains("stopped: claims to settle"), "{log}");
}

#[test]
fn a_failed_stage_names_itself_and_is_not_recorded() {
    struct Failing;
    impl Stages for Failing {
        fn inputs(&self, run: &Run, _: Stage) -> Result<Vec<PathBuf>, String> {
            Ok(vec![run.paths.dir.join("seed")])
        }
        fn outputs(&self, run: &Run, _: Stage) -> Vec<PathBuf> {
            vec![run.paths.dir.join("never")]
        }
        fn run(&self, _: &Run, _: Stage) -> Result<Done, String> {
            Err("tool failed".into())
        }
        fn stop(&self, _: &Run, _: Stage) -> Result<Option<Stop>, String> {
            Ok(None)
        }
    }
    let dir = scratch("failed");
    let run = run_in(&dir);
    let err = runner::run(&run, &Failing).unwrap_err();
    assert_eq!(err, "sources: tool failed");
    let records = std::fs::read_to_string(run.out().join("records.json"));
    assert!(records.is_err(), "a failed stage wrote a record");
    let log = std::fs::read_to_string(run.out().join("log.jsonl")).unwrap();
    assert!(log.contains("failed: tool failed"));
}
