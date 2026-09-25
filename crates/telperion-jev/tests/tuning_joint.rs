use serde_json::json;
use telperion_jev::{
    sha256_hex,
    tuning::{
        joint::{Finding, Impact, Packet},
        vision::Request,
    },
};

#[test]
fn blind_packet_labels_never_reach_request_and_metadata_is_bound() {
    let path = std::env::temp_dir().join(format!("joint-{}.png", std::process::id()));
    std::fs::write(&path, b"fixture").unwrap();
    let image = json!({"path":path,"sha256":sha256_hex(b"fixture"),"view":"whole","seed":1});
    let mut request:Request=serde_json::from_value(json!({"schema":"tuning-vision-v2","identity":"candidate","required":[{"item":"character","view":"whole","seed":1}],"images":[image.clone()],"references":[image.clone()],"quality_anchors":[{"image":image,"provenance":"fixture","scope":"finish"}],"checklist":"SECRET OWNER VERDICT"})).unwrap();
    request.required[0].item = "SECRET CELL VERDICT".into();
    request.target_species = "European beech / Fagus sylvatica".into();
    request.quality_anchors[0].scope = "SECRET ANCHOR VERDICT".into();
    request.quality_anchors[0].provenance = "SECRET PROVENANCE".into();
    let packet = Packet::from_request(&request);
    request.joint = Some(packet);
    let mut fixture = telperion_jev::tuning::vision::ReplayCase {
        id: "fixture".into(),
        provenance: "SECRET LABEL SOURCE".into(),
        expected_ready: false,
        request,
    };
    let blind = fixture.blind_request();
    let hash = blind.hash();
    assert_eq!(blind.target_species, "European beech / Fagus sylvatica");
    let mut species = blind.clone();
    species.target_species = "another species".into();
    assert_ne!(hash, species.hash());
    species.target_species.clear();
    assert!(species.verify().is_err());
    fixture.expected_ready = true;
    fixture.provenance = "ANOTHER SECRET LABEL".into();
    assert_eq!(hash, fixture.blind_request().hash());
    assert!(!serde_json::to_string(&blind).unwrap().contains("SECRET"));
    assert_eq!(blind.joint.as_ref().unwrap().reference_relation, "unknown");
    blind.verify().unwrap();
    let shots = path.with_extension("json");
    std::fs::write(
        &shots,
        serde_json::to_vec(&json!({"references":[{"id":"whole","shot":{"foliage":"hidden"}}]}))
            .unwrap(),
    )
    .unwrap();
    let mut factual = blind.clone();
    factual.joint = Some(Packet::from_request(&factual).with_shots(&shots).unwrap());
    factual.verify().unwrap();
    assert_eq!(
        factual.blind().joint.unwrap().inputs[0].visibility,
        "hidden"
    );
    std::fs::write(&shots, b"{\"references\":[{\"id\":\"whole\",\"shot\":{}}]}").unwrap();
    assert!(factual.verify().is_err());
    assert_eq!(
        Packet::from_request(&blind)
            .with_shots(&shots)
            .unwrap()
            .inputs[0]
            .visibility,
        "unknown"
    );
    std::fs::write(
        &shots,
        b"{\"references\":[{\"id\":\"whole\",\"shot\":{\"foliage\":\"bogus\"}}]}",
    )
    .unwrap();
    assert!(Packet::from_request(&blind).with_shots(&shots).is_err());
    std::fs::remove_file(shots).unwrap();
    let mut changed = blind.clone();
    changed.joint.as_mut().unwrap().inputs[0].framing =
        telperion_jev::tuning::joint::Framing::Clipped;
    assert_ne!(hash, changed.hash());
    assert_eq!(
        changed.blind().joint.unwrap().inputs[0].framing,
        telperion_jev::tuning::joint::Framing::Clipped
    );
    let finding = Finding {
        observation: "joint gap".into(),
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        impact: Impact::Blocker,
        uncertain: false,
        causal_hypothesis: Some("unproven cause".into()),
        trait_id: None,
    };
    blind
        .joint
        .as_ref()
        .unwrap()
        .verify_findings(&[finding.clone()])
        .unwrap();
    let mut invalid = finding.clone();
    invalid.evidence_ids = vec!["invented".into()];
    assert!(blind
        .joint
        .as_ref()
        .unwrap()
        .verify_findings(&[invalid])
        .is_err());
    let mut duplicate = finding.clone();
    duplicate.evidence_ids = vec!["render-0".into(), "render-0".into()];
    assert!(blind
        .joint
        .as_ref()
        .unwrap()
        .verify_findings(&[duplicate])
        .is_err());
    let mut result:telperion_jev::tuning::vision::Result=serde_json::from_value(json!({"request_sha256":blind.hash(),"assessment":{"identity":"candidate","model":"mock","ledger":"receipt","cells":[[blind.required[0],"pass"]],"defects":[],"findings":[finding]},"effort":"medium","usage":{"input_tokens":1,"output_tokens":1},"observations":["survives routing"]})).unwrap();
    result.bind(&blind).unwrap();
    assert_eq!(result.assessment.observations, vec!["survives routing"]);
    assert!(!telperion_jev::tuning::state::ready(
        &blind.required,
        "candidate",
        &result.assessment
    ));
    result.assessment.findings[0].impact = Impact::Optional;
    result.assessment.findings[0].uncertain = true;
    assert!(telperion_jev::tuning::state::ready(
        &blind.required,
        "candidate",
        &result.assessment
    ));
    result.assessment.findings[0].impact = Impact::Supported;
    result.assessment.findings[0].uncertain = false;
    assert!(telperion_jev::tuning::state::ready(
        &blind.required,
        "candidate",
        &result.assessment
    ));
    result.assessment.findings[0].uncertain = true;
    assert!(!telperion_jev::tuning::state::ready(
        &blind.required,
        "candidate",
        &result.assessment
    ));
    result.assessment.findings[0].uncertain = false;
    let mut absent = result.clone();
    absent.assessment.findings.clear();
    absent.bind(&blind).unwrap();
    assert!(!telperion_jev::tuning::state::ready(
        &blind.required,
        "candidate",
        &absent.assessment
    ));
    result.request_sha256 = changed.hash();
    result.bind(&changed).unwrap();
    assert!(!telperion_jev::tuning::state::ready(
        &changed.required,
        "candidate",
        &result.assessment
    ));
    let mut missing = blind.clone();
    missing.images.clear();
    assert!(missing.verify().is_err());
    let mut relationship = blind.clone();
    relationship.joint.as_mut().unwrap().reference_relation = "same_specimen".into();
    assert!(relationship.verify().is_err());
    relationship.joint.as_mut().unwrap().relation_source =
        Some(telperion_jev::tuning::joint::RelationSource {
            path: path.clone(),
            sha256: sha256_hex(b"fixture"),
            excerpt: "SECRET OWNER DIAGNOSIS".into(),
        });
    assert!(!serde_json::to_string(&relationship.blind())
        .unwrap()
        .contains("SECRET"));
    std::fs::remove_file(path).unwrap();
}

#[test]
fn render_condition_accepts_only_the_authored_vocabulary() {
    let path = std::env::temp_dir().join(format!("joint-condition-{}.png", std::process::id()));
    std::fs::write(&path, b"fixture").unwrap();
    let image = json!({"path":path,"sha256":sha256_hex(b"fixture"),"view":"whole","seed":1});
    let mut request:Request=serde_json::from_value(json!({"schema":"tuning-vision-v3","target_species":"Silver birch / Betula pendula","identity":"candidate","required":[{"item":"reference_character","view":"whole","seed":1}],"images":[image.clone()],"references":[image.clone()],"quality_anchors":[{"image":image,"provenance":"fixture","scope":"finish"}],"checklist":"catalogue finish floor"})).unwrap();
    request.joint = Some(Packet::from_request(&request));

    // from_request still emits only the production literal.
    assert_eq!(
        request.joint.as_ref().unwrap().inputs[0].condition,
        "same_geometry_visibility_view_not_unloaded_leaf_off"
    );
    request.verify().unwrap();

    // A truthful historical label is now a declared member, not a mismatch.
    for condition in [
        "historical_reconstructed_still",
        "historical_reconstructed_still_camera_reframe",
    ] {
        let mut declared = request.clone();
        declared.joint.as_mut().unwrap().inputs[0].condition = condition.into();
        declared
            .verify()
            .unwrap_or_else(|e| panic!("{condition}: {e}"));
    }

    // Anything outside the vocabulary is still refused.
    let mut invented = request.clone();
    invented.joint.as_mut().unwrap().inputs[0].condition = "looks_fine_probably".into();
    assert!(invented.verify().is_err());

    // A reference input may not borrow a render's condition, so packet and
    // request must still agree about which input is which.
    let mut swapped = request.clone();
    swapped.joint.as_mut().unwrap().inputs[1].condition = "historical_reconstructed_still".into();
    assert!(swapped.verify().is_err());
}

/// fn-80, 2026-09-24: under comparison v2 the reviewer wrote "Fruit clusters are
/// a known generator gap (fn-111) and were not assessed" with no evidence, and
/// the receipt refused the whole paid pass. A finding citing nothing is dropped
/// and noted; one citing evidence is kept.
#[test]
fn a_finding_with_no_evidence_is_dropped_and_noted_not_refused() {
    use telperion_jev::tuning::joint::{Finding, Impact};
    use telperion_jev::tuning::tidy;
    let finding = |obs: &str, ids: &[&str]| Finding {
        observation: obs.into(),
        evidence_ids: ids.iter().map(|s| s.to_string()).collect(),
        impact: Impact::Optional,
        uncertain: false,
        causal_hypothesis: None,
        trait_id: Some("fruit-clusters-pendent".into()),
    };
    let mut findings = vec![
        finding(
            "Fruit clusters are a known generator gap (fn-111) and were not assessed.",
            &[],
        ),
        finding(
            "The crown reads as a palm rosette.",
            &["render-0", "reference-0"],
        ),
    ];
    let untidy = tidy::findings(&mut findings, tidy::MAX_FINDINGS, "");
    assert_eq!(findings.len(), 1);
    assert_eq!(
        findings[0].observation,
        "The crown reads as a palm rosette."
    );
    assert_eq!(untidy.len(), 1);
    assert_eq!(untidy[0].violation, "finding 1 of 2: no evidence cited");
    assert!(untidy[0]
        .note
        .starts_with("dropped finding with no evidence: Fruit clusters"));
}
