//! fn-121: a tuning revision pauses by design (pilot authority, priority
//! approval, a preflight). The conductor carries that pause as its own under
//! the same id and request, resumes the tuning run in its directory with the
//! same decision, and records the revision that ends and checks every gap in
//! its result. Jev answers through the scripted transport.
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use serde_json::{json, Value};
use telperion_jev::conductor::plan::Next;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{Executor, StageOutcome};
use telperion_jev::conductor::{handoff, tuning, Config};
use telperion_jev::tuning::result::EndResult;

mod conductor_support;
use conductor_support::*;

const PAUSE: &str = "tuning-pause-1";
const ACTION: &str = "authorize bounded experimental pilot";

/// A tuning run that pauses on its first call, as `tuning-loop run` does:
/// `run.json` holds the pause, `result.json` is written anyway, and the exit
/// is an error. A resume naming the pause ends the run with the staged
/// result; any other decision leaves the pause standing.
struct Pausing {
    ended: EndResult,
    resumes: Mutex<Vec<(PathBuf, PathBuf)>>,
}

impl Executor for Pausing {
    fn stage(&self, _: &Config, _: &str) -> Result<StageOutcome, String> {
        Ok(StageOutcome::Ran)
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        out: &Path,
        resume: Option<&Path>,
    ) -> Result<(), String> {
        std::fs::create_dir_all(out).unwrap();
        let Some(decision) = resume else {
            write(&out.join("result.json"), &json!(self.ended));
            write(&out.join("run.json"), &paused_record());
            return Err("paused; see run.json".into());
        };
        self.resumes
            .lock()
            .unwrap()
            .push((out.to_path_buf(), decision.to_path_buf()));
        let named: Value = serde_json::from_slice(&std::fs::read(decision).unwrap()).unwrap();
        if named["pause_id"] != PAUSE {
            return Err("resume requires the scoped human decision".into());
        }
        write(&out.join("result.json"), &json!(self.ended));
        write(
            &out.join("run.json"),
            &json!({"pause": null, "pending": null}),
        );
        Ok(())
    }
}

fn paused_record() -> Value {
    json!({"pause": {
        "id": PAUSE, "identity": "tuning-identity",
        "reason": "magnitude live efficacy unvalidated",
        "basis": {"identity": "tuning-identity", "proposed_action": ACTION, "evidence": [],
                  "recent_outcomes": [], "next_tokens": null, "estimate_basis": "", "usage_known": true},
        "decision_requested": "Provide explicit scoped experimental authority",
    }, "pending": null})
}

fn decision(root: &Path, pause_id: &str) -> PathBuf {
    let path = root.join(format!("decision-{pause_id}.json"));
    write(
        &path,
        &json!({"pause_id": pause_id, "identity": "tuning-identity", "action": ACTION,
                "by": "owner", "rationale": "pilot authorized"}),
    );
    path
}

fn paused_run(root: &Path) -> (Config, Run, Pausing) {
    let config = config(root);
    let gaps = vec![
        gap(
            "finding-0",
            "reachable trait: crown too narrow",
            "stalled in tuning",
            None,
        ),
        gap(
            "finding-1",
            "covered trait: leaders lose girth",
            "stalled in tuning",
            None,
        ),
        gap(
            "finding-2",
            "new trait: bark plates peel",
            "stalled in tuning",
            None,
        ),
        gap("finding-3", "fine", "passing on the current tree", None),
    ];
    let executor = Pausing {
        ended: result(root, "run-1", false, gaps),
        resumes: Mutex::new(Vec::new()),
    };
    let mut run = Run::open(&config).unwrap();
    let script = Script::new();
    let (_, next) = drive(&script, &config, &mut run, &executor);
    assert!(matches!(next, Next::Tune { revision: 1, .. }), "{next:?}");
    let (word, next) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains(PAUSE), "{word}");
    assert_eq!(
        next,
        Next::Paused {
            id: PAUSE.into(),
            reason: "magnitude live efficacy unvalidated".into()
        }
    );
    (config, run, executor)
}

#[test]
fn a_tuning_pause_becomes_the_conductors_pause_with_its_handoff() {
    let root = scratch("tuning-pause");
    let (config, run, _) = paused_run(&root);
    let pause = run.pause.as_ref().unwrap();
    assert_eq!(pause.identity, "tuning-identity");
    assert_eq!(pause.basis.proposed_action, ACTION);
    assert_eq!(
        pause.decision_requested,
        "Provide explicit scoped experimental authority"
    );
    // Nothing is recorded as a revision, and the run record holds the pause.
    assert!(run.tuning.is_empty());
    assert_eq!(run.tuning_pause, Some(1));
    let handoff: Value =
        serde_json::from_slice(&std::fs::read(handoff::handoff_file(&config, PAUSE)).unwrap())
            .unwrap();
    assert_eq!(handoff["pause_id"], PAUSE);
    assert_eq!(handoff["signals"]["tuning_pause"], PAUSE);
    assert_eq!(handoff["signals"]["tuning_exit"], "paused; see run.json");
    assert_eq!(
        handoff["decision_requested"],
        "Provide explicit scoped experimental authority"
    );
}

#[test]
fn resume_runs_the_tuning_run_in_its_directory_then_every_gap_is_checked() {
    let root = scratch("tuning-resume");
    let (config, mut run, executor) = paused_run(&root);
    let file = decision(&root, PAUSE);
    let word = tuning::resume(&config, &mut run, &executor, &file).unwrap();
    assert!(
        word.contains("tuning revision 1: ended (4 gaps listed)"),
        "{word}"
    );
    assert_eq!(
        *executor.resumes.lock().unwrap(),
        vec![(config.tuning_dir(1), file)]
    );
    assert!(run.pause.is_none() && run.tuning_pause.is_none());
    assert_eq!(run.tuning.len(), 1);
    assert_eq!(run.authorizations[0].pause_id, PAUSE);
    let script = Script::new();
    let (word, _) = drive(&script, &config, &mut run, &executor);
    assert!(word.contains("finding-0: reachable with"), "{word}");
    assert!(word.contains("finding-1: covered by fn-103"), "{word}");
    assert!(word.contains("finding-2: new, packaged"), "{word}");
    let checked: Vec<&String> = run.gap_checks.keys().collect();
    assert_eq!(checked, ["finding-0", "finding-1", "finding-2"]);
}

#[test]
fn a_decision_the_tuning_loop_refuses_leaves_the_run_paused() {
    let root = scratch("tuning-refused");
    let (config, mut run, executor) = paused_run(&root);
    let before = serde_json::to_value(&run).unwrap();
    // The conductor refuses a decision that names another pause, and the
    // tuning run is never started with it.
    assert!(tuning::resume(&config, &mut run, &executor, &decision(&root, "other")).is_err());
    assert!(executor.resumes.lock().unwrap().is_empty());
    // A decision the tuning loop refuses leaves its pause, and the run as it was.
    let file = decision(&root, PAUSE);
    let err = tuning::resume(&config, &mut run, &RefusingResume, &file).unwrap_err();
    assert!(err.to_string().contains("kept pause"), "{err}");
    assert_eq!(serde_json::to_value(&run).unwrap(), before);
}

/// The tuning loop refusing a decision the conductor accepted, as it does
/// for a priority pause that carries no approval: the exit is an error and
/// `run.json` still holds the same pause.
struct RefusingResume;

impl Executor for RefusingResume {
    fn stage(&self, _: &Config, _: &str) -> Result<StageOutcome, String> {
        Ok(StageOutcome::Ran)
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        _: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        Err("explicit owner priority approval required".into())
    }
}
