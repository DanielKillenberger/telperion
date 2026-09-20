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
