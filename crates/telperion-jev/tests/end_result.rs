//! The run's end result: every approved priority is listed whatever became
//! of it, the current tree is named with its stills, and the check on each
//! gap is left for the invoker. Offline throughout.
mod fixture;

use serde_json::{json, Value};
use telperion_jev::tuning::{
    evaluation::{Comparison, Image, Trial},
    joint::{Finding, Impact},
    priority::{Checkpoint, Evidence},
    result::{self, CHECK_PENDING},
    state::{Cell, CellStatus, Visual},
};

fn image(view: &str) -> Image {
    let path = std::env::temp_dir().join(format!(
        "end-result-{view}-{}.png",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::write(&path, view.as_bytes()).unwrap();
    Image {
        path,
        sha256: telperion_jev::sha256_hex(view.as_bytes()),
        view: view.into(),
        seed: 1,
    }
}

fn trial(key: &str, round: u64, label: &str, review: Option<Value>) -> Trial {
    let mut value = json!({
        "key":key,"identity":"progress-fixture","seed":1,"round":round,"label":label,
        "overrides":{"twigs":{"hang":0.75}},"ledger":null,"feasible":true,"reason":null,
        "measurement":{},"comparisons":[],"score":0.3,"seconds":1.0
    });
    if label != "baseline" {
        value["bundle"] = json!({"id":"b","strength":1.0,
            "moves":[{"dial":"twig_hang","direction":"up","from":0.5,"to":0.75}]});
    }
    let mut t: Trial = serde_json::from_value(value).unwrap();
    t.comparisons = vec![Comparison {
        reference: "B-WHOLE".into(),
        reference_weight: 1.0,
        metric_weights: [1.0; 5],
        target: [1.0; 5],
        observed: [Some(1.0); 5],
        images: vec![image("B-WHOLE")],
    }];
    if let Some(review) = review {
        t.sheet = serde_json::from_value(review).ok();
    }
    t
}

fn finding(text: &str) -> Finding {
    Finding {
        observation: text.into(),
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        impact: Impact::Blocker,
        uncertain: false,
        causal_hypothesis: None,
        trait_id: None,
    }
}

fn cell(item: &str, status: CellStatus) -> (Cell, CellStatus) {
    (
        serde_json::from_value(json!({"item":item,"view":"B-WHOLE","seed":1})).unwrap(),
        status,
    )
}

#[test]
fn every_approved_priority_is_a_gap_entry_with_its_status_and_the_check_left_open() {
    let mut run = fixture::progress_run(vec![
        trial("base", 0, "baseline", None),
        trial("cand", 1, "bundle@1", None),
    ]);
    run.current = Some(1);
    let visual = Visual {
        identity: "cand".into(),
        model: "vision-fixture-1".into(),
        ledger: "ledger".into(),
        cells: vec![
            cell(
                "owner-priority:owner-hanging: Hanging outer foliage",
                CellStatus::Fail,
            ),
            cell("owner-priority:owner-crown: Crown shape", CellStatus::Pass),
        ],
        defects: vec![],
        observations: vec![],
        findings: vec![
            finding("foliage forms a veil"),
            finding("no hanging masses"),
        ],
        joint: None,
        known_gaps: vec![],
        coverage: vec![],
    };
    let evidence = vec![
        Evidence {
            id: "render-0".into(),
            role: "render".into(),
            image: image("B-WHOLE"),
        },
        Evidence {
            id: "reference-0".into(),
            role: "reference".into(),
            image: image("ref"),
        },
    ];
    let scope = telperion_jev::sha256_hex(b"scope");
    let checkpoint = Checkpoint::new("progress-fixture", &scope, visual.clone(), evidence).unwrap();
    let approval = json!({"checkpoint_sha256":checkpoint.hash(),"scope_sha256":scope,"ordered":[
        {"id":"owner-hanging","observation":"Hanging outer foliage","evidence_ids":["render-0"],"views":["B-WHOLE"]},
        {"id":"owner-crown","observation":"Crown shape","evidence_ids":["render-0"],"views":["B-WHOLE"]},
        {"id":"owner-bark","observation":"Smooth grey bark","evidence_ids":["render-0"],"views":["B-WHOLE"]}
    ]});
    run.priority_checkpoints = vec![checkpoint];
    run.visual = Some(visual);
    run.approval = Some(serde_json::from_value(approval).unwrap());
    run.routes = vec![
        "adoption kept; uncalibrated side-effect question answered".into(),
        "adoption rolled back: trait x went from Pass to Fail".into(),
    ];
    run.trials[1].sheet = Some(
        serde_json::from_value(json!({
            "label":"1","per_priority":{"owner-hanging":"slight","owner-crown":"none"},
            "improved":"a little","missing":"weighted droop",
            "ledger":"l","model":"vision-fixture-1","uncalibrated":"uncalibrated"
        }))
        .unwrap(),
    );

    let r = run.end_result();
    let ids: Vec<&str> = r.gaps.iter().map(|g| g.id.as_str()).collect();
    assert_eq!(
        ids,
        ["owner-hanging", "owner-crown", "owner-bark"],
        "nothing approved is forgotten"
    );
    assert_eq!(r.gaps[0].status, "failing on the current tree");
    assert_eq!(r.gaps[1].status, "passing on the current tree");
    assert_eq!(r.gaps[2].status, "not assessed");
    assert!(r.gaps.iter().all(|g| g.check == CHECK_PENDING));
    assert_eq!(
        r.gaps[0].attempts.len(),
        1,
        "only the bundle move is an attempt"
    );
    assert_eq!(r.gaps[0].reviewer_words, vec!["weighted droop".to_string()]);
    assert!(
        r.gaps[1].attempts.is_empty(),
        "a 'same' grade is not a move on that priority"
    );
    assert_eq!(r.outcome.adoptions_kept, 1);
    assert_eq!(r.outcome.adoptions_rolled_back, 1);
    assert_eq!(r.outcome.stopped, "ended");
    assert!(!r.outcome.machine_ready && r.outcome.owner_acceptance == "pending");
    let current = r.outcome.current.as_ref().unwrap();
    assert_eq!(current.key, "cand");
    assert_eq!(current.stills[0].view, "B-WHOLE");
    assert_eq!(
        r.gaps[0].stills, current.stills,
        "a gap points at the tree it was judged on"
    );

    let round_trip: result::EndResult =
        serde_json::from_value(serde_json::to_value(&r).unwrap()).unwrap();
    assert_eq!(round_trip, r);
}

#[test]
fn a_run_with_no_approval_says_so_instead_of_inventing_gaps() {
    let mut run = fixture::progress_run(vec![trial("base", 0, "baseline", None)]);
    run.pending = Some("candidate evaluation".into());
    let r = run.end_result();
    assert!(r.gaps.is_empty());
    assert_eq!(r.outcome.stopped, "interrupted during candidate evaluation");
}

fn gap() -> telperion_jev::tuning::priority::Gap {
    serde_json::from_value(
        json!({"id":"g","observation":"o","evidence_ids":[],"views":["B-WHOLE"]}),
    )
    .unwrap()
}

/// Each attempt says what it moved and what became of it: a bundle rolled
/// back with its reason, a single dial kept, a single dial never adopted.
#[test]
fn every_attempt_carries_its_moves_its_adoption_and_whether_it_stood() {
    let mut run = fixture::progress_run(vec![
        trial("base", 0, "baseline", None),
        trial("bundle", 1, "bundle@1", None),
        trial("kept", 2, "twig_hang", None),
        trial("passed", 2, "twig_hang", None),
    ]);
    run.dials = vec![
        serde_json::from_value(json!({"id":"twig_hang","path":"/twigs/hang",
        "meaning":"m","min":0.0,"max":1.0,"integer":false,"small":0.1,"substantial":0.2}))
        .unwrap(),
    ];
    run.trials[1].adopted = true;
    run.trials[1].vetoed =
        Some(serde_json::from_value(json!({"reasons":["crown went from Pass to Fail"]})).unwrap());
    for i in [2, 3] {
        run.trials[i].bundle = None;
        run.trials[i].step = serde_json::from_value(
            json!({"dial":"twig_hang","direction":"up","from":0.75,"to":0.85}),
        )
        .unwrap();
    }
    run.trials[2].adopted = true;
    run.trials[1].base = Some("base".into());
    let attempts = run.attempts_for(&gap());
    // Both sides of the comparison travel with the attempt.
    assert_eq!(attempts[0].before.len(), 1, "the tree it moved from");
    assert_eq!(attempts[0].after[0].view, "B-WHOLE");
    assert!(attempts[1].before.is_empty(), "no base, no before");
    let seen: Vec<_> = attempts
        .iter()
        .map(|a| (a.dial.as_str(), a.moves.len(), a.adopted, a.stood))
        .collect();
    assert_eq!(
        seen,
        [
            ("bundle@1", 1, true, Some(false)),
            ("twig_hang", 1, true, Some(true)),
            ("twig_hang", 1, false, None),
        ]
    );
    assert_eq!(attempts[0].moves[0].to, 0.75);
    assert_eq!(attempts[1].moves[0].from, 0.75);
    assert_eq!(
        attempts[0].rolled_back.as_deref(),
        Some("crown went from Pass to Fail")
    );
    assert!(attempts[1].rolled_back.is_none());
    let json = serde_json::to_value(&attempts[2]).unwrap();
    for key in ["moves", "adopted", "stood"] {
        assert!(json.get(key).is_some(), "the record lacks {key}");
    }
    assert_eq!(
        attempts[0].summary(),
        "round 1 bundle@1 (twig_hang 0.5 to 0.75), adopted and rolled back: crown went from Pass to Fail"
    );
}

/// A result written before attempts kept their moves still reads.
#[test]
fn a_result_written_before_moves_were_kept_still_reads() {
    let run = fixture::progress_run(vec![trial("base", 0, "baseline", None)]);
    let mut value = serde_json::to_value(run.end_result()).unwrap();
    value["gaps"] = json!([{"id":"g","rank":1,"priority":"o","status":"failing on the current tree",
        "reviewer_words":[],"stills":[],
        "check":CHECK_PENDING,"attempts":[{"dial":"twig_hang","round":1,"action_ledger":null,
        "score_before_round":null,"score_after":0.3,"feasible":true,"reason":null,
        "visual_outcome":null}]}]);
    let old: result::EndResult = serde_json::from_value(value).unwrap();
    let a = &old.gaps[0].attempts[0];
    assert!(a.moves.is_empty() && !a.adopted && a.stood.is_none());
    assert_eq!(a.summary(), "round 1 twig_hang");
}
