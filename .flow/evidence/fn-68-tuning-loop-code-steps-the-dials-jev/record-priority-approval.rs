use std::{collections::BTreeMap, fs, path::Path};
use serde_json::{json, Value};
use telperion_jev::{
    sha256_hex,
    tuning::{
        continuation::{Basis, HumanDecision},
        engine::Run,
        judgments,
        priority::{requirements, Approval, Checkpoint, Gap},
        state::Cell,
    },
};

const ROOT: &str = ".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev";
const RUNTIME: &str = ".flow/tmp/fn68-pilot-run/run.json";
const RUNTIME_SHA: &str = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e";
const CHECKPOINT: &str = "dcc6a63fbfa782fb4970fb5a9fbf0a1228c39a65b6ec3c1a9736e7c6e02a8010";
const SCOPE: &str = "903276d3348f3e5f40aa2c8ec25659c7d16f3880e61c559d2fabef33c985bed3";
const BIRCH_POS: &str = "2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e";
const BEECH_NEG: &str = "7e69fda3e95b317fcb9d89e15e32179c54fdd316a57d1c3223d6e26639b8b490";
const BIRCH_REF_WHOLE: &str = "cae1627afa33d8a881fe927090b222a37fc0dfa2febffa3e3f1edcc52aa309be";
const BIRCH_REF_BARE: &str = "097c70624dd9a75df67d82b07857794b7bf082581544ee08789e2e8562063f6c";
const BIRCH_REF_BARK: &str = "72d282e3ea593d2568aae43bc9da924626c2c965bae4b177edfb63f1db30f7ec";

fn main() {
    let root = Path::new(ROOT);
    assert_eq!(sha256_hex(&fs::read(RUNTIME).unwrap()), RUNTIME_SHA);
    let packet: Value = serde_json::from_slice(&fs::read(root.join("priority-review.json")).unwrap()).unwrap();
    let checkpoint: Checkpoint = serde_json::from_value(packet["checkpoint"].clone()).unwrap();
    assert_eq!(checkpoint.hash(), CHECKPOINT);
    assert_eq!(checkpoint.scope_sha256, SCOPE);
    assert!(packet["approval"].is_null());
    let original_visual = serde_json::to_value(&checkpoint.visual).unwrap();

    let ordered = vec![
        Gap {
            id: "owner-crown-foliage".into(),
            observation: "Crown shape and foliage organization".into(),
            evidence_ids: vec!["render-0".into(), "render-1".into(), "reference-0".into(), "reference-1".into()],
            views: vec!["B-WHOLE".into(), "B-BARE".into()],
        },
        Gap {
            id: "owner-hanging-foliage".into(),
            observation: "Hanging outer foliage".into(),
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            views: vec!["B-WHOLE".into()],
        },
        Gap {
            id: "owner-materials".into(),
            observation: "Materials, including bark and foliage material".into(),
            evidence_ids: vec!["render-0".into(), "render-1".into(), "reference-0".into(), "reference-1".into()],
            views: vec!["B-WHOLE".into(), "B-BARE".into()],
        },
    ];
    let approval = Approval {
        checkpoint_sha256: CHECKPOINT.into(),
        scope_sha256: SCOPE.into(),
        ordered,
    };
    approval.verify(&checkpoint, SCOPE).unwrap();
    assert_eq!(serde_json::to_value(&checkpoint.visual).unwrap(), original_visual);

    let base = vec![
        Cell { item: "reference_character".into(), view: "B-WHOLE".into(), seed: 1 },
        Cell { item: "reference_character".into(), view: "B-BARE".into(), seed: 1 },
    ];
    let required = requirements(&base, Some(&approval));
    let ordered_bytes = serde_json::to_vec(&approval.ordered).unwrap().len() as u64;
    let required_bytes = serde_json::to_vec(&required).unwrap().len() as u64;
    let visual_reserve = 40000 + ordered_bytes + required_bytes + 256;

    let mut run: Run = serde_json::from_slice(&fs::read(RUNTIME).unwrap()).unwrap();
    run.visual = Some(checkpoint.visual.clone());
    run.priority_checkpoints.push(checkpoint.clone());
    run.authorizations.push(HumanDecision {
        pause_id: "offline-packet-dcc6a63f".into(),
        identity: run.identity.clone(),
        action: "approve gap priorities".into(),
        by: "human owner, relayed by host".into(),
        rationale: "offline overlay for serialized reservations; not written to run.json".into(),
        next_identity: None,
        recover_interrupted: false,
        preserve_evidence: true,
        token_cap_extension: None,
        round_cap_extension: None,
        visual_cap_extension: None,
        visual_reconciliation: None,
        baseline_amendment: None,
        experimental_pilot: None,
        diagnosis: None,
        external_usage: None,
        priority_approval: Some(approval.clone()),
    });
    assert_eq!(
        run.approved_priorities().unwrap().checkpoint_sha256,
        CHECKPOINT
    );
    assert_eq!(sha256_hex(&fs::read(RUNTIME).unwrap()), RUNTIME_SHA);

    let summary = judgments::summary(&run);
    let route_q = judgments::routes(&BTreeMap::new());
    let proposal_q = judgments::proposals(&run).unwrap();
    let mut direction_q = serde_json::Map::new();
    for dial in &run.dials {
        let current = run
            .effective
            .pointer(&dial.path)
            .and_then(Value::as_f64)
            .expect("dial current");
        direction_q.insert(dial.id.clone(), dial.direction_question(current).unwrap());
    }
    let direction_q = Value::Object(direction_q);
    let route_reserve = judgments::allowance(&summary, &route_q);
    let proposal_reserve = judgments::allowance(&summary, &proposal_q);
    let direction_reserve = judgments::allowance(&summary, &direction_q);

    let mut basis = basis_from(&run, "targeted tuning round");
    let next = proposal_reserve
        .checked_add(visual_reserve)
        .expect("next_tokens");
    basis.next_tokens = Some(next);
    basis.estimate_basis = format!(
        "proposal serialized-request bound {proposal_reserve} + all-cell visual reservation {visual_reserve}; actual usage may exceed estimate and then pauses"
    );
    let continuation_reserve = judgments::allowance(
        &serde_json::to_value(&basis).unwrap(),
        &telperion_jev::tuning::continuation::questions(),
    );

    let exact_round = route_reserve + continuation_reserve + proposal_reserve + visual_reserve;
    let recent = summary["recent_attempts"].as_array().unwrap();
    let recent_item = recent
        .iter()
        .map(|t| serde_json::to_vec(t).unwrap().len())
        .max()
        .unwrap_or(0) as u64;
    let future_visual = serde_json::to_vec(run.visual.as_ref().unwrap()).unwrap().len() as u64;
    let unresolved_second_round = future_visual + 4 * recent_item;

    let birch = Path::new("/tmp/fn68-reconstruction-images/silver-birch-S-WHOLE.png");
    let beech_neg = Path::new("/tmp/fn68-reconstruction-images/european-beech-B-WHOLE.png");
    let birch_ref = Path::new(
        "/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/silver-birch/bepe339A.jpg",
    );
    let birch_bare_ref = Path::new(
        "/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/silver-birch/bepe340A.jpg",
    );
    let birch_bark_ref = Path::new(
        "/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/silver-birch/bepe3243.jpg",
    );
    assert_eq!(sha256_hex(&fs::read(birch).unwrap()), BIRCH_POS);
    assert_eq!(sha256_hex(&fs::read(beech_neg).unwrap()), BEECH_NEG);
    assert_eq!(sha256_hex(&fs::read(birch_ref).unwrap()), BIRCH_REF_WHOLE);
    assert_eq!(sha256_hex(&fs::read(birch_bare_ref).unwrap()), BIRCH_REF_BARE);
    assert_eq!(sha256_hex(&fs::read(birch_bark_ref).unwrap()), BIRCH_REF_BARK);

    let durable = root.join("local/fixtures");
    fs::create_dir_all(&durable).unwrap();
    let birch_dst = durable.join("birch-round26-S-WHOLE.png");
    let beech_dst = durable.join("beech-round22-B-WHOLE.png");
    if !birch_dst.exists() {
        fs::copy(birch, &birch_dst).unwrap();
    }
    if !beech_dst.exists() {
        fs::copy(beech_neg, &beech_dst).unwrap();
    }
    assert_eq!(sha256_hex(&fs::read(&birch_dst).unwrap()), BIRCH_POS);
    assert_eq!(sha256_hex(&fs::read(&beech_dst).unwrap()), BEECH_NEG);

    let birch_base = vec![Cell {
        item: "reference_character".into(),
        view: "S-WHOLE".into(),
        seed: 1,
    }];
    let birch_ordered = vec![
        Gap {
            id: "owner-crown-foliage".into(),
            observation: "Crown shape and foliage organization".into(),
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            views: vec!["S-WHOLE".into()],
        },
        Gap {
            id: "owner-hanging-foliage".into(),
            observation: "Hanging outer foliage".into(),
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            views: vec!["S-WHOLE".into()],
        },
        Gap {
            id: "owner-materials".into(),
            observation: "Materials, including bark and foliage material".into(),
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            views: vec!["S-WHOLE".into()],
        },
    ];
    let birch_required = requirements(
        &birch_base,
        Some(&Approval {
            checkpoint_sha256: CHECKPOINT.into(),
            scope_sha256: SCOPE.into(),
            ordered: birch_ordered.clone(),
        }),
    );
    let birch_visual = 40000
        + serde_json::to_vec(&birch_ordered).unwrap().len() as u64
        + serde_json::to_vec(&birch_required).unwrap().len() as u64
        + 256;

    let event = json!({
        "event": "owner-priority-approval",
        "applied_to_runtime": false,
        "pause_id": "offline-packet-dcc6a63f",
        "identity": checkpoint.run_identity,
        "action": "approve gap priorities",
        "by": "human owner, relayed by host",
        "quotation": "the 1,2,3 yes",
        "host_proposal": [
            "Crown shape and foliage organization",
            "Hanging outer foliage",
            "Materials with clearer bark evidence where needed"
        ],
        "rationale": "Host proposed (1) Crown shape and foliage organization, (2) Hanging outer foliage, (3) Materials with clearer bark evidence where needed. Owner: \"the 1,2,3 yes\". Priority 3 recorded as broad materials on whole and bare; foliage material stays assessable or unknown. Owner- IDs, not finding-0/1/2.",
        "preserve_evidence": true,
        "priority_approval": approval,
        "verification": {
            "checkpoint_sha256": CHECKPOINT,
            "scope_sha256": SCOPE,
            "checkpoint_hash_matches": true,
            "approval_verified": true,
            "visual_unchanged": true,
            "original_runtime_sha256": RUNTIME_SHA
        },
        "required_cells_seed1": required,
        "reservations": {
            "visual_tokens_for": visual_reserve,
            "visual_formula": "40000 + serde(ordered) + serde(required) + 256",
            "ordered_bytes": ordered_bytes,
            "required_bytes": required_bytes,
            "jev_formula": "serde(state)+serde(questions)+1024",
            "route_tokens": route_reserve,
            "continuation_tokens": continuation_reserve,
            "proposal_magnitude_tokens": proposal_reserve,
            "direction_tokens_not_in_execute": direction_reserve,
            "exact_one_round_tokens": exact_round,
            "evaluations_max": 4,
            "evaluation_images_each": 4,
            "evaluation_images_max": 16,
            "final_coverage_images": 4,
            "final_coverage_visual_passes": 1
        }
    });
    fs::write(root.join("priority-approval.json"), serde_json::to_vec_pretty(&event).unwrap()).unwrap();

    let preflight = json!({
        "status": "offline preflight, not authorization",
        "approval": "priority-approval.json",
        "r8_representative": {
            "blocked": false,
            "meaning": "Generic protocol can use accepted birch as the known-positive with matching birch references. Beech is assessed against approved goals; it is not required to be a finished-beech positive.",
            "positive": {
                "id": "birch-round26-whole",
                "expected_ready": true,
                "label": "Owner accepted round26 silver birch as species",
                "render": {"path": "local/fixtures/birch-round26-S-WHOLE.png", "sha256": BIRCH_POS, "view": "S-WHOLE", "seed": 1},
                "reference": {"path": "/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/silver-birch/bepe339A.jpg", "id": "S-WHOLE", "sha256": BIRCH_REF_WHOLE, "caption": "plant habit, summer"}
            },
            "negative": {
                "id": "beech-round22-whole",
                "expected_ready": false,
                "label": "Owner rejected round22",
                "render": {"path": "local/fixtures/beech-round22-B-WHOLE.png", "sha256": BEECH_NEG, "view": "B-WHOLE", "seed": 1}
            },
            "beech_progress": {
                "id": "current-reframed-beech",
                "expected_ready": false,
                "assessed_against": ["owner-crown-foliage", "owner-hanging-foliage", "owner-materials"],
                "sha256_whole": "e0d5bc93369f281dc39e9fa008a1cec647b2c040de47baa89e11bc31d1fe6f68",
                "not": "Not a positive fixture. Progress against approved goals only."
            },
            "protocol_adaptation": [
                "Species view IDs: S-WHOLE for birch, B-WHOLE/B-BARE for beech",
                "Birch case uses birch references, not beech photographs",
                "Owner-priority cells attach only to views present on that case",
                "Checklist language is the approved goals, not a finished-beech requirement",
                "v2 replay already spent; a new owner-priority request is required for this qualification"
            ],
            "assembly": {
                "captures": 0,
                "visual_passes": 2,
                "birch_whole_visual_tokens": birch_visual,
                "beech_seed1_visual_tokens": visual_reserve,
                "token_add": birch_visual + visual_reserve,
                "note": "Reuse existing stills. Two separate visual reservations, one per case."
            },
            "present_unused_without_matching_render": [
                {"id": "S-BARE", "file": "bepe340A.jpg", "sha256": BIRCH_REF_BARE, "caption": "plant habit, winter"},
                {"id": "S-BARK", "file": "bepe3243.jpg", "sha256": BIRCH_REF_BARK, "caption": "trunk, bark"}
            ],
            "missing_exact_assets": [
                "No reconstructed owner-accepted silver-birch S-BARE still. Recovery licensed S-WHOLE only. Birch-positive cannot cover bare or materials-on-bare cells.",
                "No reconstructed birch bark-close-up render matching S-BARK."
            ]
        },
        "seed_scope": "This approval binds seed 1 views in the frozen packet. Pilot-config still lists B-WHOLE/B-BARE/B-BASE at seeds 1 and 42. Adding seed 42 or B-BASE would change scope and invalidate this approval. Fixed/fresh verification remains a separate R8 gate, not automatic qualification.",
        "stages": {
            "route": {"tokens": route_reserve, "state_bytes": serde_json::to_vec(&summary).unwrap().len(), "questions_bytes": serde_json::to_vec(&route_q).unwrap().len(), "in_execute_inner": true},
            "continuation": {"tokens": continuation_reserve, "state_bytes": serde_json::to_vec(&basis).unwrap().len(), "questions_bytes": serde_json::to_vec(&telperion_jev::tuning::continuation::questions()).unwrap().len(), "in_execute_inner": true},
            "proposal_magnitude": {"tokens": proposal_reserve, "questions_bytes": serde_json::to_vec(&proposal_q).unwrap().len(), "in_execute_inner": true, "note": "Live propose() is one magnitude call over all authored dials."},
            "direction": {"tokens": direction_reserve, "questions_bytes": serde_json::to_vec(&direction_q).unwrap().len(), "in_execute_inner": false, "note": "Direction questions exist and are serialized here. execute_inner does not dispatch a separate direction call."},
            "evaluations": {"max": 4, "images_each": 4, "images_max": 16, "token_reservation": 0},
            "final_coverage": {"visual_tokens": visual_reserve, "visual_passes": 1, "images": 4, "note": "visual_tokens_for depends on approval and required cells, not future result text."}
        },
        "unresolved_bytes": {
            "label": "One-round whole-attempt is exact from current overlay. A stall second loop would re-serialize summary with an unknown new Visual and up to four new trial summaries. Those bytes are not invented here.",
            "current_visual_bytes": future_visual,
            "largest_recent_attempt_bytes": recent_item,
            "conservative_second_round_state_bytes": unresolved_second_round,
            "not_included_in_proposed_cap": true
        },
        "current_caps": {"tokens": 552431, "token_cap": 570000, "remaining": 17569, "visual": 20, "visual_cap": 20, "evaluations": 6, "evaluation_cap": 13, "images": 28, "image_cap": 52},
        "proposed_not_granted": {
            "meaning": "Proposal only. This file grants nothing. One automatic round: route + continuation + magnitude proposals + up to four evaluations + one final coverage visual.",
            "r7_whole_attempt": {
                "route_tokens": route_reserve,
                "continuation_tokens": continuation_reserve,
                "proposal_magnitude_tokens": proposal_reserve,
                "visual_tokens": visual_reserve,
                "token_add": exact_round,
                "proposed_token_cap": 552431 + exact_round,
                "evaluations": 4,
                "evaluation_images": 16,
                "final_coverage_images": 4,
                "images_add": 20,
                "visual_passes": 1,
                "proposed_visual_cap": 21,
                "proposed_image_cap": 48,
                "direction_not_added": direction_reserve
            },
            "r8_representative": {
                "blocked": false,
                "token_add": birch_visual + visual_reserve,
                "proposed_token_cap_if_only_r8": 552431 + birch_visual + visual_reserve,
                "visual_passes": 2,
                "proposed_visual_cap": 22,
                "captures": 0
            }
        }
    });
    fs::write(
        root.join("r7-r8-offline-preflight.json"),
        serde_json::to_vec_pretty(&preflight).unwrap(),
    )
    .unwrap();
    assert_eq!(sha256_hex(&fs::read(RUNTIME).unwrap()), RUNTIME_SHA);
    println!(
        "route={route_reserve} continuation={continuation_reserve} proposal={proposal_reserve} direction={direction_reserve} visual={visual_reserve} exact_round={exact_round} birch_visual={birch_visual} cells={}",
        required.len()
    );
}

fn basis_from(run: &Run, action: &str) -> Basis {
    let projection = judgments::summary(run);
    let mut evidence = run
        .visual
        .as_ref()
        .map(|v| v.defects.clone())
        .unwrap_or_default();
    evidence.push(
        json!({
            "current_identity": projection["current_identity"],
            "owner_priorities": projection["owner_priorities"],
            "visual_evidence": projection["visual"],
            "resource_limit": projection["resource_limit"],
            "resource_amendments": projection["resource_amendments"],
            "agent_diagnoses": projection["agent_diagnoses"],
            "verified_evidence_reuse": projection["verified_evidence_reuse"]
        })
        .to_string(),
    );
    Basis {
        identity: run.identity.clone(),
        proposed_action: action.into(),
        evidence,
        recent_outcomes: projection["recent_attempts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|t| t.to_string())
            .collect(),
        next_tokens: None,
        estimate_basis: String::new(),
        usage_known: run.usage_known,
    }
}
