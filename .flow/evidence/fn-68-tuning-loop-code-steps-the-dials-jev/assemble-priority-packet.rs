use std::{fs, path::Path};
use serde_json::{json, Value};
use telperion_jev::{
    sha256_hex,
    tuning::{
        evaluation::Image,
        priority::{evidence, scope, Checkpoint},
        state::{Cell, Visual},
    },
};

const ROOT: &str = ".flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev";
const RUNTIME: &str = ".flow/tmp/fn68-pilot-run/run.json";
const RUNTIME_SHA: &str = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e";
const OBJECTIVES: &str = "Reference-first mature European beech character at catalogue finish on required B-WHOLE and B-BARE seed 1 views. Owner acceptance remains separate. This packet is the frozen initial review, not a model ranking or owner approval.";

fn image(path: &str, sha256: &str, view: &str) -> Image {
    let image = Image {
        path: path.into(),
        sha256: sha256.into(),
        view: view.into(),
        seed: 1,
    };
    image.verify().unwrap();
    image
}

fn main() {
    let root = Path::new(ROOT);
    let runtime_sha = sha256_hex(&fs::read(RUNTIME).unwrap());
    assert_eq!(runtime_sha, RUNTIME_SHA, "original runtime must stay frozen");
    let bound: Value = serde_json::from_slice(&fs::read(root.join("reframed-result-bound-v2.json")).unwrap()).unwrap();
    let visual: Visual = serde_json::from_value(bound["result"]["visual"]["assessment"].clone()).unwrap();
    let renders = [
        image(
            &format!("{ROOT}/local/reframed/B-WHOLE.png"),
            "e0d5bc93369f281dc39e9fa008a1cec647b2c040de47baa89e11bc31d1fe6f68",
            "B-WHOLE",
        ),
        image(
            &format!("{ROOT}/local/reframed/B-BARE.png"),
            "9c64f46459c824458951c465832feead33a730182cfcffdd8ab5a786d7fe9dde",
            "B-BARE",
        ),
    ];
    let references = [
        image(
            &format!("{ROOT}/local/beech-whole-reference.jpg"),
            "855fddf7d2974aff8f0bd9421223fdec990f9b6b9d229e714e511082e81157e6",
            "B-WHOLE",
        ),
        image(
            &format!("{ROOT}/local/beech-bare-reference.jpg"),
            "7a269a2b43154bf2641fde94aba6853a8d5459d997e1fda29bf2d80733134e1d",
            "B-BARE",
        ),
    ];
    let anchors = [image(
        ".flow/evidence/fn9/final/preview/norway-spruce-1-whole.png",
        "8e10ac1cf993a4cc005f7a1374e5763274c5dc636f42204e140464242ba027f1",
        "whole",
    )];
    let required = vec![
        Cell {
            item: "reference_character".into(),
            view: "B-WHOLE".into(),
            seed: 1,
        },
        Cell {
            item: "reference_character".into(),
            view: "B-BARE".into(),
            seed: 1,
        },
    ];
    let scope = scope("european-beech", OBJECTIVES, &required, &references);
    let links = evidence(&visual, &renders, &references, &anchors).unwrap();
    let identity = visual.identity.clone();
    let checkpoint = Checkpoint::new(&identity, &scope, visual, links).unwrap();
    let packet = json!({
        "checkpoint_sha256": checkpoint.hash(),
        "checkpoint": checkpoint,
        "approval": Value::Null,
        "ordering": "First three eligible findings in reviewer source order, not a new model ranking or owner approval",
        "meaning": "Owner chooses what matters. Approval is neither readiness nor final acceptance; all other findings remain in checkpoint.visual.",
        "source": {
            "typed_result": "reframed-result-bound-v2.json",
            "raw_receipt": "reframed-b-receipt.json",
            "receipt_sha256": sha256_hex(&fs::read(root.join("reframed-b-receipt.json")).unwrap()),
            "bound_result_sha256": sha256_hex(&fs::read(root.join("reframed-result-bound-v2.json")).unwrap()),
            "inventory_sha256": "f98f4325752a95103c510cc3f1871af2fcd12a2053a9359e6f770b128853c20f",
            "original_runtime_sha256": runtime_sha,
            "original_runtime_identity": "524b7cb170ad8fcbb70c00cedf51ab045fc284abf43e73ec8def1afe96835081",
            "candidate_identity": "c1781092148dc4c69cbf4b60c37be26a59dcd59f6e43d014f409f0c29c269d57"
        },
        "owner_history": {
            "status": "sourced history, not approval of this packet",
            "path": "owner-reference-correction.md",
            "note": "Crown architecture and leaf-bearing weighted droop were named core; material follows. That history may inform a reorder or addition. It does not confirm this packet."
        },
        "accounting": {
            "tokens": 552431,
            "token_cap": 570000,
            "remaining_tokens": 17569,
            "visual_passes": 20,
            "visual_cap": 20,
            "evaluations": 6,
            "image_reservations": 28,
            "side_import": "not applied"
        }
    });
    let out = root.join("priority-review.json");
    let bytes = serde_json::to_vec_pretty(&packet).unwrap();
    if out.exists() {
        let existing: Value = serde_json::from_slice(&fs::read(&out).unwrap()).unwrap();
        assert_eq!(existing, packet, "frozen packet drifted");
        println!("verified {}", checkpoint.hash());
    } else {
        fs::write(&out, bytes).unwrap();
        println!("wrote {} {}", out.display(), checkpoint.hash());
    }
    assert!(packet["approval"].is_null());
    assert_eq!(
        packet["checkpoint"]["proposed_top_three"],
        json!(["finding-0", "finding-1", "finding-2"])
    );
    assert_eq!(sha256_hex(&fs::read(RUNTIME).unwrap()), RUNTIME_SHA);
}
