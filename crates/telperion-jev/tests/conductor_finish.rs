//! fn-136: converged is a finish. A tuning revision the no-progress guard
//! stopped, or one that stopped on no progress with every drawable objective
//! passing, is recorded as converged rather than carried to the host, and
//! the conductor assembles the owner's packet instead of asking for another
//! revision. The packet's checklist lists every known gap with its specs.
use std::path::Path;

use serde_json::{json, Value};
use telperion_jev::conductor::plan::Next;
use telperion_jev::conductor::state::Run;
use telperion_jev::conductor::step::{Executor, StageOutcome};
use telperion_jev::conductor::Config;
use telperion_jev::tuning::result::EndResult;
use telperion_jev::tuning::unexpressed::Unexpressed;

mod conductor_support;
use conductor_support::*;

const RUNAWAY: &str = "runaway: 5 rounds in a row kept nothing (rounds 4, 5, 6, 7, 8); \
                       they spent 900 tokens, 10 evaluations, 40 images and 5 visual passes";
const NO_PROPOSAL: &str = "no supported proposal; bounded diagnosis required";
const FRUIT_SPEC: &str = "fn-111-the-palms-infructescence-a-hanging-date";

/// A tuning run that stops as `tuning-loop run` does: the result is written,
/// `run.json` holds the pause, and a paused exit is an error.
struct Stopping {
    result: EndResult,
    reason: &'static str,
}

impl Executor for Stopping {
    fn stage(&self, _: &Config, _: &str) -> Result<StageOutcome, String> {
        Ok(StageOutcome::Ran)
    }
    fn search(&self, _: &Config) -> Result<String, String> {
        panic!("no requirement is unmet in a finished run")
    }
    fn tune(
        &self,
        _: &Config,
        _: u64,
        _: &[String],
        out: &Path,
        _: Option<&Path>,
    ) -> Result<(), String> {
        std::fs::create_dir_all(out).unwrap();
        write(&out.join("result.json"), &json!(self.result));
        write(
            &out.join("run.json"),
            &json!({"pause": {
            "id": "tuning-stop", "identity": "tuning-identity", "reason": self.reason,
            "basis": {"identity": "tuning-identity", "proposed_action": "reassess or diagnose",
                      "evidence": [], "recent_outcomes": [], "next_tokens": null,
                      "estimate_basis": "", "usage_known": true},
            "decision_requested": "Provide a scoped decision for reassess or diagnose.",
        }, "pending": null}),
        );
        Err("paused; see run.json".into())
    }
}

/// The species folder with the gate's known gap and every artifact the
/// packet asks for, and a run whose first revision stops for `reason`.
fn stopped(tag: &str, reason: &'static str, statuses: &[&str]) -> (Config, Run, Stopping) {
    let root = scratch(tag);
    let config = config(&root);
    write(
        &config.dir.join("gate.json"),
        &json!({"stage": "gate", "body": {"status": "complete", "capability": {"known_gaps": [
            {"capability": "infructescence", "reason": "adds realism",
             "captured_by": ["fn-33-flowers-cones-and-compound-leaves-as", FRUIT_SPEC]}]}}}),
    );
    write(&config.dir.join("metrics.json"), &json!({"autonomy": {}}));
    std::fs::write(config.dir.join("ARTICLE.md"), "# Palm\n").unwrap();
    let gaps = statuses
        .iter()
        .enumerate()
        .map(|(i, status)| gap(&format!("trait-{i}"), "a drawable trait", status, None))
        .collect();
    let mut result = result(&root, "run-1", false, gaps);
    result.known_gaps = vec![Unexpressed {
        trait_id: "fruit-clusters-pendent".into(),
        spec: FRUIT_SPEC.into(),
    }];
    let run = Run::open(&config).unwrap();
    (config, run, Stopping { result, reason })
}

/// Drives to the first revision and through its stop.
fn tune_once(config: &Config, run: &mut Run, executor: &Stopping) -> (String, Next) {
    let script = Script::new();
    let (_, next) = drive(&script, config, run, executor);
    assert!(matches!(next, Next::Tune { revision: 1, .. }), "{next:?}");
    drive(&script, config, run, executor)
}

#[test]
fn a_run_the_guard_stopped_goes_to_the_owners_packet_with_every_known_gap() {
    let passing = "passing on the current tree";
    let (config, mut run, executor) = stopped("guard", RUNAWAY, &[passing, "stalled in tuning"]);
    let (word, next) = tune_once(&config, &mut run, &executor);
    assert!(word.contains("converged"), "{word}");
    assert!(run.pause.is_none());
    assert_eq!(run.tuning_pause, None);
    assert!(run.tuning[0]
        .converged
        .as_deref()
        .unwrap()
        .contains("runaway"));
    // No gap check, and no second revision: the packet is next.
    assert_eq!(next, Next::Packet);
    let (word, next) = drive(&Script::new(), &config, &mut run, &executor);
    assert!(word.contains("packet ready"), "{word}");
    assert_eq!(next, Next::Ready);
    let packet: Value =
        serde_json::from_slice(&std::fs::read(config.packet_file()).unwrap()).unwrap();
    assert!(packet["converged"].as_str().unwrap().contains("runaway"));
    assert_eq!(packet["machine_readiness"], "not ready");
    let checklist = packet["checklist"].as_array().unwrap();
    let status = |id: &str| {
        checklist
            .iter()
            .find(|c| c["id"] == id)
            .unwrap_or_else(|| panic!("{id} missing from {checklist:?}"))
            .clone()
    };
    assert_eq!(status("trait-1")["status"], "stalled in tuning");
    let fruit = status("fruit-clusters-pendent");
    assert_eq!(fruit["status"], "known gap");
    assert_eq!(fruit["captured_by"], json!([FRUIT_SPEC]));
    let organ = status("infructescence");
    assert_eq!(organ["status"], "known gap");
    assert_eq!(
        organ["captured_by"],
        json!(["fn-33-flowers-cones-and-compound-leaves-as", FRUIT_SPEC])
    );
}

/// The article lives where the document stage recorded it (the catalogue
/// folder the script writes), not only in the run folder: the palm's packet
/// was withheld for a missing ARTICLE.md that sat in `catalogue/date-palm`.
#[test]
fn the_packet_reads_the_article_where_the_document_stage_recorded_it() {
    let passing = "passing on the current tree";
    let (config, mut run, executor) = stopped("article", RUNAWAY, &[passing]);
    std::fs::remove_file(config.dir.join("ARTICLE.md")).unwrap();
    let (_, next) = tune_once(&config, &mut run, &executor);
    assert_eq!(next, Next::Packet);
    let catalogue = config.dir.join("catalogue-palm");
    std::fs::create_dir_all(&catalogue).unwrap();
    std::fs::write(catalogue.join("ARTICLE.md"), "# Palm\n").unwrap();
    let document = config.paths().artifact("document");
    let mut record: Value = serde_json::from_slice(&std::fs::read(&document).unwrap()).unwrap();
    record["body"]["article"] = json!(catalogue.join("ARTICLE.md"));
    write(&document, &record);
    let (word, next) = drive(&Script::new(), &config, &mut run, &executor);
    assert!(word.contains("packet ready"), "{word}");
    assert_eq!(next, Next::Ready);
}

/// A no-progress stop finishes the run only when every drawable objective
/// passes; with one still failing, the stop is the host's as before.
#[test]
fn no_progress_is_a_finish_only_with_every_drawable_objective_passing() {
    let passing = "passing on the current tree";
    let (config, mut run, executor) = stopped("all-passing", NO_PROPOSAL, &[passing, passing]);
    let (word, next) = tune_once(&config, &mut run, &executor);
    assert!(word.contains("converged"), "{word}");
    assert_eq!(next, Next::Packet);

    let (config, mut run, executor) =
        stopped("one-failing", NO_PROPOSAL, &[passing, "stalled in tuning"]);
    let (word, next) = tune_once(&config, &mut run, &executor);
    assert!(word.contains("paused tuning-stop"), "{word}");
    assert!(run.tuning.is_empty());
    assert_eq!(
        next,
        Next::Paused {
            id: "tuning-stop".into(),
            reason: NO_PROPOSAL.into()
        }
    );
}

/// R7: the palm's revision 3 was a bootstrap, so its reviewer is unqualified
/// for positives. Stopped by the guard, it still reaches the owner's packet:
/// machine readiness reads `unqualified reviewer`, reviewer qualification is
/// a known gap, and the owner's verdict is the acceptance.
#[test]
fn the_palms_bootstrap_revision_reaches_the_packet_with_an_unqualified_reviewer() {
    let (config, mut run, mut executor) = stopped("bootstrap", RUNAWAY, &[]);
    let mut palm: EndResult =
        serde_json::from_str(include_str!("fixtures/fn136-palm-revision-3-result.json")).unwrap();
    assert!(palm.outcome.bootstrap && !palm.outcome.machine_ready);
    // The still lives in the fn-80 worktree; the packet only asks that it is on disk.
    let still = config.dir.join("P-WHOLE.png");
    std::fs::write(&still, b"png").unwrap();
    palm.outcome.current.as_mut().unwrap().stills[0].path = still.display().to_string();
    executor.result = palm;
    let (word, next) = tune_once(&config, &mut run, &executor);
    assert!(word.contains("converged"), "{word}");
    assert_eq!(next, Next::Packet);
    let (word, _) = drive(&Script::new(), &config, &mut run, &executor);
    assert!(word.contains("packet ready"), "{word}");
    let packet: Value =
        serde_json::from_slice(&std::fs::read(config.packet_file()).unwrap()).unwrap();
    assert_eq!(packet["ready_for_owner_review"], true);
    assert_eq!(packet["machine_readiness"], "unqualified reviewer");
    let checklist = packet["checklist"].as_array().unwrap();
    let qualification = checklist
        .iter()
        .find(|c| c["id"] == "reviewer-qualification")
        .unwrap();
    assert_eq!(qualification["status"], "known gap");
    let outstanding = packet["outstanding"].as_array().unwrap();
    assert_eq!(outstanding.len(), 2, "{outstanding:?}");
}
