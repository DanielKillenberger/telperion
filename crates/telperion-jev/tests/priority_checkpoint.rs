use serde_json::json;
use telperion_jev::{
    sha256_hex,
    tuning::{
        evaluation::Image,
        priority::*,
        state::{Cell, Visual},
    },
};

#[test]
fn owner_ranking_is_explicit_scoped_and_does_not_rewrite_review() {
    let path = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    std::fs::write(&path, b"fixture").unwrap();
    let image = Image {
        path: path.clone(),
        sha256: sha256_hex(b"fixture"),
        view: "whole".into(),
        seed: 1,
    };
    let evidence = vec![
        Evidence {
            id: "render-0".into(),
            role: "render".into(),
            image: image.clone(),
        },
        Evidence {
            id: "reference-0".into(),
            role: "reference".into(),
            image: image.clone(),
        },
    ];
    let visual:Visual=serde_json::from_value(json!({"identity":"candidate","model":"reviewer","ledger":"review-1","cells":[[{"item":"character","view":"whole","seed":1},"pass"]],"defects":[],"findings":[{"observation":"Possible gap","evidence_ids":["render-0","reference-0"],"impact":"optional","uncertain":true}]})).unwrap();
    let required = vec![Cell {
        item: "character".into(),
        view: "whole".into(),
        seed: 1,
    }];
    let scope = scope("species", "objectives", &required, &[image]);
    let checkpoint = Checkpoint::new("run", &scope, visual.clone(), evidence).unwrap();
    let original = serde_json::to_value(&checkpoint.visual).unwrap();
    let approval = Approval {
        checkpoint_sha256: checkpoint.hash(),
        scope_sha256: scope.clone(),
        ordered: vec![Gap {
            id: "owner-missed".into(),
            observation: "Missed defining feature".into(),
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            views: vec!["whole".into()],
        }],
    };
    approval.verify(&checkpoint, &scope).unwrap();
    assert_eq!(serde_json::to_value(&checkpoint.visual).unwrap(), original);
    assert_eq!(requirements(&required, Some(&approval)).len(), 2);
    assert!(approval.verify(&checkpoint, "changed objectives").is_err());
    for change in 0..4 {
        let mut bad = approval.clone();
        match change {
            0 => bad.checkpoint_sha256 = "stale".into(),
            1 => bad.ordered.push(bad.ordered[0].clone()),
            2 => bad.ordered[0].evidence_ids = vec!["missing".into()],
            _ => bad.ordered[0].observation.clear(),
        };
        assert!(bad.verify(&checkpoint, &scope).is_err());
    }
    std::fs::write(&path, b"changed").unwrap();
    assert!(approval.verify(&checkpoint, &scope).is_err());
    std::fs::remove_file(path).unwrap();
}
