//! The reviewer's side-by-side verdict: what it is shown, how strictly its
//! answer binds, and which candidate that answer adopts. Offline throughout.
mod fixture;

use serde_json::{json, Value};
use std::fs;
use telperion_jev::tuning::{
    evaluation::{Comparison, Image, Trial},
    live::Config,
    priority::Gap,
    progress::Look,
    progress::{self, Answer, Choice, Judgment, Movement, On, Regression, Request},
};

fn image(view: &str, seed: u32, body: &str) -> Image {
    let path = std::env::temp_dir().join(format!(
        "progress-{}-{}.png",
        body,
        telperion_jev::ledger::new_entry_id()
    ));
    fs::write(&path, body.as_bytes()).unwrap();
    Image {
        path,
        sha256: telperion_jev::sha256_hex(body.as_bytes()),
        view: view.into(),
        seed,
    }
}

fn trial(key: &str, images: Vec<Image>) -> Trial {
    Trial {
        key: key.into(),
        identity: "run".into(),
        seed: 1,
        round: 1,
        label: "twig_hang".into(),
        overrides: json!({}),
        ledger: None,
        feasible: true,
        reason: None,
        measurement: json!({}),
        comparisons: vec![Comparison {
            reference: "whole".into(),
            reference_weight: 1.,
            metric_weights: [1., 1., 1., 1., 1.],
            target: [1., 1., 1., 1., 1.],
            observed: [None; 5],
            images,
        }],
        score: Some(0.2),
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
        progress: None,
        adopted_over: vec![],
        bundle: None,
        parent_bundle: None,
    }
}

fn gap(id: &str) -> Gap {
    Gap {
        id: id.into(),
        observation: format!("observation for {id}"),
        evidence_ids: vec!["render-0".into()],
        views: vec!["whole".into()],
    }
}

fn request(a: Image, b: Image, ids: &[&str]) -> Request {
    Request {
        schema: progress::VERSION.into(),
        target_species: "european-beech".into(),
        view: "whole".into(),
        seed: 1,
        references: vec![image("whole", 1, "reference")],
        a,
        b,
        priorities: ids
            .iter()
            .map(|id| progress::Priority {
                id: (*id).into(),
                observation: format!("observation for {id}"),
            })
            .collect(),
        owner_notes: "owner notes".into(),
    }
}

fn answer(pairs: &[(&str, Choice)], regressions: Vec<Regression>) -> Answer {
    Answer {
        verdicts: pairs
            .iter()
            .map(|(id, verdict)| Judgment {
                priority_id: (*id).into(),
                verdict: *verdict,
            })
            .collect(),
        improved: "the outer foliage hangs".into(),
        missing: "the crown is still enclosed".into(),
        regressions,
    }
}

#[test]
fn the_reviewer_is_never_told_which_render_is_the_candidate() {
    let (current, candidate) = (
        trial("current-key", vec![image("whole", 1, "current")]),
        trial("candidate-key", vec![image("whole", 1, "candidate")]),
    );
    let side = progress::candidate_side(&current.key, &candidate.key);
    assert!(side == "a" || side == "b");
    assert_eq!(
        side,
        progress::candidate_side(&current.key, &candidate.key),
        "the side is decided by the two keys, so it is the same every time"
    );
    // Both keys decide it: swapping them can move the candidate to the other
    // side, and neither key alone says which side it will take.
    let sides: Vec<String> = ["a1", "b2", "c3", "d4", "e5", "f6"]
        .iter()
        .map(|k| progress::candidate_side("current-key", k))
        .collect();
    assert!(sides.contains(&"a".to_string()) && sides.contains(&"b".to_string()));

    let request = request(
        current.comparisons[0].images[0].clone(),
        candidate.comparisons[0].images[0].clone(),
        &["finding-0"],
    );
    let wire = serde_json::to_string(&request).unwrap();
    for word in [
        "candidate",
        "current",
        "round",
        "score",
        "newer",
        "base",
        "action",
    ] {
        assert!(
            !wire.contains(&format!("\"{word}\"")),
            "the request names {word}"
        );
    }
    assert!(!wire.contains("candidate_is"));
}

#[test]
fn an_answer_binds_only_when_it_covers_exactly_what_was_asked() {
    let request = request(
        image("whole", 1, "a-side"),
        image("whole", 1, "b-side"),
        &["finding-0", "finding-1"],
    );
    let good = answer(
        &[("finding-0", Choice::ABetter), ("finding-1", Choice::Same)],
        vec![],
    );
    let bound = progress::bind(&request, &good, "a", "receipt".into(), "mock".into()).unwrap();
    assert_eq!(bound.per_priority["finding-0"], Movement::Better);
    assert_eq!(bound.per_priority["finding-1"], Movement::Same);
    assert_eq!(bound.candidate_is, "a");
    assert!(bound.uncalibrated.contains("uncalibrated"));
    // The same answer with the candidate on the other side reads the other way.
    let mirrored = progress::bind(&request, &good, "b", "receipt".into(), "mock".into()).unwrap();
    assert_eq!(mirrored.per_priority["finding-0"], Movement::Worse);

    let missing = answer(&[("finding-0", Choice::ABetter)], vec![]);
    assert!(progress::bind(&request, &missing, "a", "r".into(), "m".into()).is_err());
    let unknown_id = answer(
        &[
            ("finding-0", Choice::ABetter),
            ("crown-character", Choice::Same),
        ],
        vec![],
    );
    assert!(progress::bind(&request, &unknown_id, "a", "r".into(), "m".into()).is_err());
    let repeated = answer(
        &[("finding-0", Choice::ABetter), ("finding-0", Choice::Same)],
        vec![],
    );
    assert!(progress::bind(&request, &repeated, "a", "r".into(), "m".into()).is_err());
    let mut wordless = good.clone();
    wordless.missing = "  ".into();
    assert!(progress::bind(&request, &wordless, "a", "r".into(), "m".into()).is_err());
}

fn verdict(pairs: &[(&str, Choice)], regressions: &[(&str, &str)]) -> progress::Verdict {
    let ids: Vec<&str> = pairs.iter().map(|(id, _)| *id).collect();
    let request = request(image("whole", 1, "a"), image("whole", 1, "b"), &ids);
    progress::bind(
        &request,
        &answer(
            pairs,
            regressions
                .iter()
                .map(|(render, text)| Regression {
                    render: (*render).into(),
                    text: (*text).into(),
                })
                .collect(),
        ),
        "a",
        "receipt".into(),
        "mock".into(),
    )
    .unwrap()
}

#[test]
fn a_candidate_is_adopted_only_when_it_helped_somewhere_and_hurt_nowhere() {
    let better = verdict(&[("g0", Choice::ABetter), ("g1", Choice::Same)], &[]);
    let two_better = verdict(&[("g0", Choice::ABetter), ("g1", Choice::ABetter)], &[]);
    let mixed = verdict(&[("g0", Choice::ABetter), ("g1", Choice::BBetter)], &[]);
    let all_same = verdict(&[("g0", Choice::Same), ("g1", Choice::Same)], &[]);
    let unknown = verdict(&[("g0", Choice::Unknown), ("g1", Choice::Unknown)], &[]);
    // The candidate is on side a in these verdicts, so a note about b is a
    // note about the current tree.
    let breaks = verdict(
        &[("g0", Choice::ABetter), ("g1", Choice::Same)],
        &[("a", "the crown loses its clear bole")],
    );
    let faults_current = verdict(
        &[("g0", Choice::ABetter), ("g1", Choice::ABetter)],
        &[("b", "the current tree's limbs cross")],
    );
    assert!(better.adoptable() && two_better.adoptable());
    assert!(
        faults_current.adoptable(),
        "a note about the current tree is a reason the candidate is better"
    );
    assert_eq!(faults_current.regressions[0].on, On::Current);
    assert_eq!(breaks.regressions[0].on, On::Candidate);
    for refused in [&mixed, &all_same, &unknown, &breaks] {
        assert!(!refused.adoptable(), "{refused:?}");
    }
    assert_eq!((better.better(), better.worse()), (1, 0));
    assert_eq!((mixed.better(), mixed.worse()), (1, 1));

    // Most better wins; ties keep the order the proposals arrived in.
    let mut trials = vec![];
    for (i, v) in [&better, &two_better, &mixed, &breaks]
        .into_iter()
        .enumerate()
    {
        let mut t = trial(&format!("k{i}"), vec![image("whole", 1, "still")]);
        t.progress = Some(v.clone());
        trials.push(t);
    }
    let reviewed: Vec<(usize, Value)> = (0..4).map(|i| (i, json!({"i":i}))).collect();
    assert_eq!(progress::adopt(&reviewed, &trials), Some(1));
    let first_two = verdict(&[("g0", Choice::ABetter), ("g1", Choice::Same)], &[]);
    trials[1].progress = Some(first_two);
    assert_eq!(
        progress::adopt(&reviewed, &trials),
        Some(0),
        "a tie keeps the earlier proposal, which carried the stronger direction"
    );
    // An infeasible candidate is never adopted, whatever the reviewer said.
    trials[0].feasible = false;
    trials[1].feasible = false;
    assert_eq!(progress::adopt(&reviewed, &trials), None);
    // Nothing eligible is a visual stall, not a silent adoption.
    let none: Vec<Trial> = vec![];
    assert_eq!(progress::adopt(&[], &none), None);
}

#[test]
fn visual_selection_needs_its_adapter_and_the_bootstrap_authority() {
    let f = fixture::verifying_fixture(
        json!({"evaluations":0,"images":0,"tokens":0,"rounds":0,"max_evaluations":13,
            "max_images":52,"max_tokens":902_431,"max_rounds":3,"visual_passes":0,
            "max_visual_passes":26}),
    );
    let mut raw: Value = serde_json::from_slice(&fs::read(&f.config_path).unwrap()).unwrap();
    let base: Config = serde_json::from_value(raw.clone()).unwrap();
    base.verify().expect("the fixture verifies as it is");
    assert!(base.selection.is_score(), "score is the default");
    let identity = base.identity().unwrap();

    raw["selection"] = json!("visual");
    let without: Config = serde_json::from_value(raw.clone()).unwrap();
    assert!(without
        .verify()
        .unwrap_err()
        .contains("requires a progress adapter"));

    let protocol = f.root.join("progress-protocol.md");
    fs::write(&protocol, "the reviewer compares two renders").unwrap();
    raw["progress"] = json!({"adapter":{"program":"python3","args":[],"model":"mock",
        "effort":"medium","timeout_seconds":600,"ledger":f.root.join("ledger")},
        "protocol":protocol});
    raw["visual_bootstrap"] = json!(false);
    let unbootstrapped: Config = serde_json::from_value(raw.clone()).unwrap();
    assert_eq!(
        unbootstrapped.verify().unwrap_err(),
        "visual selection is uncalibrated; bootstrap authority required"
    );

    raw["visual_bootstrap"] = json!(true);
    let visual: Config = serde_json::from_value(raw.clone()).unwrap();
    visual.verify().expect("visual under bootstrap verifies");
    assert_ne!(
        visual.identity().unwrap(),
        identity,
        "the protocol's bytes join the run's identity"
    );
    let before = visual.identity().unwrap();
    fs::write(&protocol, "a different protocol").unwrap();
    let changed: Config = serde_json::from_value(raw).unwrap();
    assert_ne!(
        changed.identity().unwrap(),
        before,
        "rewriting the protocol changes what the run is"
    );

    // The score default serializes away, so no existing config's identity moves.
    let round_trip = serde_json::to_value(&base).unwrap();
    assert!(round_trip.get("selection").is_none() && round_trip.get("progress").is_none());
    assert_eq!(
        Config::identity(&serde_json::from_value(round_trip).unwrap()).unwrap(),
        identity
    );
    f.cleanup();
}

#[test]
fn the_request_uses_a_still_both_trials_already_hold() {
    let shared = |body: &str| vec![image("bark", 1, body), image("whole", 1, body)];
    let current = trial("current", shared("current"));
    let candidate = trial("candidate", shared("candidate"));
    let mut lonely = trial("lonely", vec![image("bark", 1, "lonely")]);
    lonely.key = "lonely".into();
    let state = fixture::progress_run(vec![current, candidate, lonely]);
    let priorities = vec![gap("finding-0")];
    let Look::Ask(request, side, from_priority) = progress::request(
        &state,
        "european-beech",
        &[image("whole", 1, "reference")],
        &state.trials[0],
        &state.trials[1],
        &priorities,
    )
    .unwrap() else {
        panic!("two different renders are a question for the reviewer")
    };
    assert!(from_priority);
    assert_eq!(request.view, "whole");
    assert_eq!(request.priorities.len(), 1);
    assert_eq!(request.references.len(), 1);
    assert_eq!(
        side,
        progress::candidate_side(&state.trials[0].key, &state.trials[1].key)
    );
    // No priority view differs, so the review falls back to the other view the
    // two trials share and says so.
    let Look::Ask(fallback, _, from_priority) = progress::request(
        &state,
        "european-beech",
        &[image("bark", 1, "reference")],
        &state.trials[0],
        &state.trials[2],
        &priorities,
    )
    .unwrap() else {
        panic!("the bark stills differ, so there is something to ask about")
    };
    assert_eq!((fallback.view.as_str(), from_priority), ("bark", false));

    // A candidate that draws the current tree is settled by code alone.
    let twin = trial("twin", state.trials[0].comparisons[0].images.clone());
    let twins = fixture::progress_run(vec![state.trials[0].clone(), twin]);
    assert!(matches!(
        progress::request(
            &twins,
            "european-beech",
            &[image("whole", 1, "reference")],
            &twins.trials[0],
            &twins.trials[1],
            &priorities,
        )
        .unwrap(),
        Look::Inert
    ));
    let settled = progress::inert();
    assert!(settled.inert && !settled.adoptable() && settled.model == "code");
    assert!(settled.note.unwrap().contains("byte-identical"));

    // Nothing shared at all is still an error, and so is an empty priority set.
    let alone = trial("alone", vec![image("crown", 1, "alone")]);
    let apart = fixture::progress_run(vec![state.trials[0].clone(), alone]);
    assert!(progress::request(
        &apart,
        "european-beech",
        &[image("whole", 1, "reference")],
        &apart.trials[0],
        &apart.trials[1],
        &priorities
    )
    .unwrap_err()
    .contains("no shared still"));
    assert!(progress::request(
        &state,
        "european-beech",
        &[image("whole", 1, "reference")],
        &state.trials[0],
        &state.trials[1],
        &[]
    )
    .unwrap_err()
    .contains("no tuning-routed priority"));
}

#[test]
fn a_regression_note_refuses_the_candidate_only_when_it_is_about_the_candidate() {
    let request = request(
        image("whole", 1, "a-side"),
        image("whole", 1, "b-side"),
        &["finding-0", "finding-1"],
    );
    let both_better = &[
        ("finding-0", Choice::ABetter),
        ("finding-1", Choice::ABetter),
    ];
    // The live case: better on both, and the only note faults the other render.
    let about_current = Answer {
        regressions: vec![Regression {
            render: "b".into(),
            text: "the crown in b is flat on one side".into(),
        }],
        ..answer(both_better, vec![])
    };
    let kept = progress::bind(&request, &about_current, "a", "r".into(), "m".into()).unwrap();
    assert_eq!(kept.regressions[0].on, On::Current);
    assert!(
        kept.adoptable() && !kept.breaks_something(),
        "a note about the current tree refused the candidate"
    );
    // Read from the other side, the same words are about the candidate.
    let mirrored = progress::bind(&request, &about_current, "b", "r".into(), "m".into()).unwrap();
    assert_eq!(mirrored.regressions[0].on, On::Candidate);
    assert!(!mirrored.adoptable() && mirrored.breaks_something());

    let unnamed = Answer {
        regressions: vec![Regression {
            render: "neither".into(),
            text: "something".into(),
        }],
        ..answer(both_better, vec![])
    };
    assert!(
        progress::bind(&request, &unnamed, "a", "r".into(), "m".into())
            .unwrap_err()
            .contains("names no render")
    );

    // A record written before the sides were attributed still refuses.
    let mut stored = serde_json::to_value(&kept).unwrap();
    stored["regressions"] = json!(["the crown is flat on one side"]);
    let legacy: progress::Verdict = serde_json::from_value(stored).unwrap();
    assert_eq!(legacy.regressions[0].on, On::Unknown);
    assert!(!legacy.adoptable() && legacy.breaks_something());

    // The schema the reviewer answers in is the one the version names, and
    // neither it nor the prompt says which render is the candidate.
    assert_eq!(progress::VERSION, "tuning-progress-v2");
    assert!(progress::PROMPT.contains("which render has the problem"));
    for word in ["candidate", "current tree"] {
        assert!(
            !progress::PROMPT.to_lowercase().contains(word),
            "the prompt says {word}"
        );
    }
    assert!(
        progress::PROMPT.contains("nothing here says which"),
        "the prompt must say that neither label means anything"
    );
}
