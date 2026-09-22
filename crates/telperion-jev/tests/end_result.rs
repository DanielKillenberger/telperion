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
        t.progress = serde_json::from_value(review).ok();
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
            cell("owner-priority:owner-hanging: Hanging outer foliage", CellStatus::Fail),
            cell("owner-priority:owner-crown: Crown shape", CellStatus::Pass),
        ],
        defects: vec![],
        observations: vec![],
        findings: vec![finding("foliage forms a veil"), finding("no hanging masses")],
        joint: None,
        coverage: vec![],
    };
    let evidence = vec![
        Evidence { id: "render-0".into(), role: "render".into(), image: image("B-WHOLE") },
        Evidence { id: "reference-0".into(), role: "reference".into(), image: image("ref") },
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
    run.authorizations = vec![serde_json::from_value(json!({
        "pause_id":"p","identity":"progress-fixture","action":"approve gap priorities",
        "by":"fixture owner","rationale":"synthetic","priority_approval":approval
    }))
    .unwrap()];
    run.routes = vec![
        "owner-hanging=tuning".into(),
        "owner-crown=tuning".into(),
        "adoption kept; uncalibrated side-effect question answered".into(),
        "adoption rolled back: trait x went from Pass to Fail".into(),
    ];
    run.trials[1].progress = Some(
        serde_json::from_value(json!({
            "per_priority":{"owner-hanging":"better","owner-crown":"same"},
            "improved":"a little","missing":"weighted droop","regressions":[],
            "ledger":"l","model":"vision-fixture-1","candidate_is":"B",
            "uncalibrated":"uncalibrated"
        }))
        .unwrap(),
    );

    let r = run.end_result();
    let ids: Vec<&str> = r.gaps.iter().map(|g| g.id.as_str()).collect();
    assert_eq!(ids, ["owner-hanging", "owner-crown", "owner-bark"], "nothing approved is forgotten");
    assert_eq!(r.gaps[0].status, "stalled in tuning");
    assert_eq!(r.gaps[1].status, "passing on the current tree");
    assert_eq!(r.gaps[2].status, "not assessed");
    assert_eq!(r.gaps[0].latest_route.as_deref(), Some("tuning"));
    assert!(r.gaps.iter().all(|g| g.check == CHECK_PENDING));
    assert_eq!(r.gaps[0].attempts.len(), 1, "only the bundle move is an attempt");
    assert_eq!(r.gaps[0].reviewer_words, vec!["weighted droop".to_string()]);
    assert!(r.gaps[1].attempts.is_empty(), "a 'same' grade is not a move on that priority");
    assert_eq!(r.outcome.adoptions_kept, 1);
    assert_eq!(r.outcome.adoptions_rolled_back, 1);
    assert_eq!(r.outcome.stopped, "ended");
    assert!(!r.outcome.machine_ready && r.outcome.owner_acceptance == "pending");
    let current = r.outcome.current.as_ref().unwrap();
    assert_eq!(current.key, "cand");
    assert_eq!(current.stills[0].view, "B-WHOLE");
    assert_eq!(r.gaps[0].stills, current.stills, "a gap points at the tree it was judged on");

    let page = result::markdown(&r);
    for needle in ["owner-hanging", "owner-bark", "not assessed", "stalled in tuning", "cand", CHECK_PENDING] {
        assert!(page.contains(needle), "RESULT.md lacks {needle}");
    }
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
    assert!(result::markdown(&r).contains("No approved priorities"));
}
