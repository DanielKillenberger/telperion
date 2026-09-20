use serde_json::json;
use telperion_jev::{
    sha256_hex,
    tuning::{
        joint::{Finding, Impact, Packet},
        reference_first::*,
        state::{ready, CellStatus},
        vision,
    },
};

fn request() -> vision::Request {
    let p = std::env::temp_dir().join(format!(
        "reference-first-{}.png",
        telperion_jev::ledger::new_entry_id()
    ));
    std::fs::write(&p, b"reference fixture").unwrap();
    let image = json!({"path":p,"sha256":sha256_hex(b"reference fixture"),"view":"whole","seed":1});
    let mut r:vision::Request=serde_json::from_value(json!({"schema":"tuning-vision-v3","identity":"CANDIDATE_SECRET","target_species":"Example species","required":[{"item":"OWNER_SECRET","view":"whole","seed":1}],"images":[image.clone()],"references":[image.clone()],"quality_anchors":[{"image":image,"provenance":"OWNER_SECRET","scope":"finish"}],"checklist":"OWNER_SECRET"})).unwrap();
    r.joint = Some(Packet::from_request(&r));
    r
}
fn inventory(r: &vision::Request) -> Inventory {
    let request = ReferenceRequest::from_comparison(r);
    Inventory {
        request_sha256: request.hash(),
        prompt_sha256: request.prompt_hash(),
        request,
        model: "mock".into(),
        effort: "medium".into(),
        ledger: "immutable-receipt".into(),
        traits: vec![Trait {
            id: "trait-1".into(),
            priority: Priority::Core,
            observation: "Visible reference-defining trait".into(),
            reference_ids: vec!["reference-0".into()],
            uncertain: false,
        }],
        observations: vec![],
    }
}
#[test]
fn candidate_owner_isolation_and_reference_hash_binding() {
    let mut r = request();
    let a = ReferenceRequest::from_comparison(&r);
    a.verify().unwrap();
    assert!(!serde_json::to_string(&a).unwrap().contains("SECRET"));
    r.identity = "OTHER_CANDIDATE".into();
    r.checklist = "OTHER_OWNER".into();
    r.images.clear();
    assert_eq!(a.hash(), ReferenceRequest::from_comparison(&r).hash());
    assert_eq!(a.specimen_relationship, "unknown");
    let mut i = inventory(&request());
    i.verify().unwrap();
    i.traits[0].reference_ids = vec!["render-0".into()];
    assert!(i.verify().is_err());
    let mut i = inventory(&request());
    i.prompt_sha256 = "stale".into();
    assert!(i.verify().is_err());
    let mut i = inventory(&request());
    i.request.references[0].image.sha256 = sha256_hex(b"changed");
    assert!(i.verify().is_err());
}
#[test]
fn coverage_unknown_and_positive_finish_are_enforced() {
    let original = request();
    let request = ComparisonRequest::new(&original, inventory(&original));
    request.verify().unwrap();
    let r = request.comparison.clone();
    let finding = Finding {
        observation: "Supported match".into(),
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        impact: Impact::Supported,
        uncertain: false,
        causal_hypothesis: None,
    };
    let visual:vision::Result=serde_json::from_value(json!({"request_sha256":r.hash(),"assessment":{"identity":r.identity,"model":"mock","ledger":"receipt","cells":[[r.required[0],"pass"]],"defects":[],"findings":[finding]},"effort":"medium","usage":{"input_tokens":1,"output_tokens":1},"observations":[]})).unwrap();
    let base = ComparisonResult {
        request_sha256: request.hash(),
        visual,
        coverage: vec![Coverage {
            trait_id: "trait-1".into(),
            status: CellStatus::Pass,
            evidence_ids: vec!["render-0".into(), "reference-0".into()],
            explanation: "Visible match; no defining mismatch".into(),
        }],
    };
    let mut good = base.clone();
    good.bind(&request).unwrap();
    let first_bound = serde_json::to_value(&good).unwrap();
    good.bind(&request).unwrap();
    assert_eq!(first_bound, serde_json::to_value(&good).unwrap());
    assert!(ready(&r.required, &r.identity, &good.visual.assessment));
    assert!(good
        .visual
        .assessment
        .observations
        .iter()
        .any(|s| s.contains("trait-1")));
    for status in [CellStatus::Unknown, CellStatus::Fail] {
        let mut bad = base.clone();
        bad.coverage[0].status = status;
        bad.bind(&request).unwrap();
        assert!(!ready(&r.required, &r.identity, &bad.visual.assessment));
    }
    let mut missing = base.clone();
    missing.coverage.clear();
    missing.bind(&request).unwrap();
    assert!(!ready(&r.required, &r.identity, &missing.visual.assessment));
    let mut missing_cell = base.clone();
    missing_cell.visual.assessment.cells.clear();
    assert!(missing_cell.bind(&request).is_err());
    let mut invalid = base.clone();
    invalid.coverage[0].evidence_ids = vec!["invented".into()];
    assert!(invalid.bind(&request).is_err());
    let mut two = request.clone();
    two.comparison
        .references
        .push(two.comparison.references[0].clone());
    two.comparison.joint = Some(Packet::from_request(&two.comparison));
    two.inventory.request = ReferenceRequest::from_comparison(&two.comparison);
    two.inventory.request_sha256 = two.inventory.request.hash();
    let mut wrong_source = base.clone();
    wrong_source.request_sha256 = two.hash();
    wrong_source.visual.request_sha256 = two.comparison.hash();
    wrong_source.coverage[0].evidence_ids = vec!["render-0".into(), "reference-1".into()];
    assert!(wrong_source.bind(&two).is_err());
    let mut duplicate = base.clone();
    duplicate.coverage.push(duplicate.coverage[0].clone());
    assert!(duplicate.bind(&request).is_err());
    let mut uncertain_request = request.clone();
    uncertain_request.inventory.traits[0].uncertain = true;
    let mut uncertain = base.clone();
    uncertain.request_sha256 = uncertain_request.hash();
    uncertain.bind(&uncertain_request).unwrap();
    assert!(!ready(
        &r.required,
        &r.identity,
        &uncertain.visual.assessment
    ));
}
