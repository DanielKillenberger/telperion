use std::{collections::BTreeMap, fs, path::Path};
use serde_json::{json, Value};
use telperion_jev::{
    sha256_hex,
    tuning::{
        judgments, priority::{requirements, Approval, Checkpoint, Gap},
        state::Cell,
    },
};

const ROOT: &str = ".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev";
const RUNTIME: &str = ".flow/tmp/fn68-pilot-run/run.json";
const RUNTIME_SHA: &str = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e";
const CHECKPOINT: &str = "dcc6a63fbfa782fb4970fb5a9fbf0a1228c39a65b6ec3c1a9736e7c6e02a8010";
const SCOPE: &str = "903276d3348f3e5f40aa2c8ec25659c7d16f3880e61c559d2fabef33c985bed3";

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
            id: "owner-bark-material".into(),
            observation: "Materials with clearer bark evidence where needed".into(),
            evidence_ids: vec!["render-1".into(), "reference-1".into()],
            views: vec!["B-BARE".into()],
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
    let visual_reserve = 40000
        + serde_json::to_vec(&approval.ordered).unwrap().len() as u64
        + serde_json::to_vec(&required).unwrap().len() as u64
        + 256;

    let dials: Value = serde_json::from_slice(&fs::read(root.join("cases/dials.json")).unwrap()).unwrap();
    let findings = checkpoint
        .visual
        .findings
        .iter()
        .map(|f| json!({"impact":f.impact,"uncertain":f.uncertain,"observation":f.observation,"evidence_ids":f.evidence_ids}))
        .collect::<Vec<_>>();
    let route_state = json!({
        "owner_priorities":{"approval":approval,"authority":"Explicit owner ranking outranks model severity. It selects objectives, not implementation or resolved status; all original findings remain below."},
        "preserved_findings":findings,
        "dials":dials,
        "scope":"seed-1 packet only; fresh seed 42 is not in this approval"
    });
    let route_questions = judgments::routes(&BTreeMap::new());
    let route_reserve = judgments::allowance(&route_state, &route_questions);

    let birch = Path::new("/tmp/fn68-reconstruction-images/silver-birch-S-WHOLE.png");
    let beech_neg = Path::new("/tmp/fn68-reconstruction-images/european-beech-B-WHOLE.png");
    let birch_sha = sha256_hex(&fs::read(birch).unwrap());
    let beech_neg_sha = sha256_hex(&fs::read(beech_neg).unwrap());
    assert_eq!(birch_sha, "2745f4c37a4410236f2adfb0d6b0c3903c7312bd8a15872ef8e58d4c35293d1e");
    assert_eq!(beech_neg_sha, "7e69fda3e95b317fcb9d89e15e32179c54fdd316a57d1c3223d6e26639b8b490");

    let durable = root.join("local/fixtures");
    fs::create_dir_all(&durable).unwrap();
    let birch_dst = durable.join("birch-round26-S-WHOLE.png");
    let beech_dst = durable.join("beech-round22-B-WHOLE.png");
    if !birch_dst.exists() { fs::copy(birch, &birch_dst).unwrap(); }
    if !beech_dst.exists() { fs::copy(beech_neg, &beech_dst).unwrap(); }
    assert_eq!(sha256_hex(&fs::read(&birch_dst).unwrap()), birch_sha);
    assert_eq!(sha256_hex(&fs::read(&beech_dst).unwrap()), beech_neg_sha);

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
        "rationale": "Host proposed (1) Crown shape and foliage organization, (2) Hanging outer foliage, (3) Materials with clearer bark evidence where needed. Owner: \"the 1,2,3 yes\". Recorded as owner- IDs, not model source-order finding-0/1/2.",
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
            "ordered_bytes": serde_json::to_vec(&approval.ordered).unwrap().len(),
            "required_bytes": serde_json::to_vec(&required).unwrap().len(),
            "jev_route_tokens": route_reserve,
            "jev_formula": "serde(state)+serde(questions)+1024",
            "route_state_bytes": serde_json::to_vec(&route_state).unwrap().len(),
            "route_questions_bytes": serde_json::to_vec(&route_questions).unwrap().len(),
            "new_overlay_images": 4,
            "reassessment_extra_images_this_candidate": 0
        }
    });
    let out = root.join("priority-approval.json");
    let bytes = serde_json::to_vec_pretty(&event).unwrap();
    if out.exists() {
        assert_eq!(serde_json::from_slice::<Value>(&fs::read(&out).unwrap()).unwrap(), event);
        println!("verified approval {}", CHECKPOINT);
    } else {
        fs::write(&out, bytes).unwrap();
        println!("wrote {}", out.display());
    }

    let preflight = json!({
        "status": "offline preflight, not authorization",
        "approval": "priority-approval.json",
        "positive_fixture": {
            "id": "birch-round26-whole",
            "expected_ready": true,
            "sha256": birch_sha,
            "durable_path": "local/fixtures/birch-round26-S-WHOLE.png",
            "provenance": "Owner accepted round26 silver birch as species. Exact reconstructed still matches round26-birch-crown. vision-replay-v2.json; REPORT recovered historical SHA 2745f4c3…",
            "protocol": "tuning-vision-v2 whole-crown seed1 only",
            "not": "Not a European-beech owner-priority positive. Not reference-first. Not hanging/bark/fresh-seed coverage. P1/P2 were desired target outcomes, not this fixture."
        },
        "negative_fixtures": [
            {
                "id": "current-reframed-beech",
                "expected_ready": false,
                "sha256": "e0d5bc93369f281dc39e9fa008a1cec647b2c040de47baa89e11bc31d1fe6f68",
                "note": "Current failing limbs-only beech. Not a positive."
            },
            {
                "id": "beech-round22-whole",
                "expected_ready": false,
                "sha256": beech_neg_sha,
                "durable_path": "local/fixtures/beech-round22-B-WHOLE.png",
                "provenance": "Owner rejected round22. Exact reconstructed still."
            }
        ],
        "r8_blocker": "No owner-accepted European beech render exists for the approved owner-priority cells. Birch-round26 is a genuine known-positive for v2 silver-birch whole-crown seed1 only. Always-reject cannot pass PROTOCOL. Owner-priority R8 qualification is blocked until a beech-positive fixture exists.",
        "seed_scope": "This approval binds seed 1 views in the frozen packet. Pilot-config still lists B-WHOLE/B-BARE/B-BASE at seeds 1 and 42. Adding seed 42 or B-BASE would change scope and invalidate this approval. Fixed/fresh verification remains a separate R8 gate, not automatic qualification.",
        "route_request": {"state": route_state, "questions": route_questions},
        "current_caps": {"tokens": 552431, "token_cap": 570000, "remaining": 17569, "visual": 20, "visual_cap": 20},
        "proposed_not_granted": {
            "meaning": "Proposal only. This file grants nothing.",
            "r7": {
                "evaluations": 1,
                "images": 4,
                "visual_passes": 1,
                "jev_route_tokens": route_reserve,
                "visual_tokens": visual_reserve,
                "token_add": route_reserve + visual_reserve,
                "proposed_token_cap": 552431 + route_reserve + visual_reserve,
                "proposed_visual_cap": 21
            },
            "r8_owner_priority": {
                "blocked": true,
                "reason": "missing beech-positive fixture for approved cells"
            },
            "r8_v2_replay_already_done": {
                "blocked_as_new_qualification": true,
                "reason": "v2 birch-accept/beech-reject already spent; replay would not qualify owner-priority or fresh-seed protocol"
            }
        }
    });
    let pf = root.join("r7-r8-offline-preflight.json");
    let pf_bytes = serde_json::to_vec_pretty(&preflight).unwrap();
    if pf.exists() {
        assert_eq!(serde_json::from_slice::<Value>(&fs::read(&pf).unwrap()).unwrap(), preflight);
        println!("verified preflight");
    } else {
        fs::write(&pf, pf_bytes).unwrap();
        println!("wrote {}", pf.display());
    }
    assert_eq!(sha256_hex(&fs::read(RUNTIME).unwrap()), RUNTIME_SHA);
    println!("visual_reserve={visual_reserve} route_reserve={route_reserve} cells={}", required.len());
}
