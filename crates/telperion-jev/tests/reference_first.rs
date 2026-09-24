use serde_json::json;
use telperion_jev::{
    sha256_hex,
    tuning::{
        joint::{Finding, Impact, Packet},
        reference_first::*,
        state::{ready, CellStatus},
        unexpressed::Unexpressed,
        veto::worsened,
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
    let r = request();
    let blind = ComparisonRequest::new(&r, inventory(&r));
    let production = ComparisonRequest::production(&r, inventory(&r));
    production.verify().unwrap();
    assert!(!serde_json::to_string(&blind.comparison)
        .unwrap()
        .contains("OWNER_SECRET"));
    assert!(serde_json::to_string(&production.comparison)
        .unwrap()
        .contains("OWNER_SECRET"));
    assert_ne!(blind.hash(), production.hash());
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
    let replay = Replay {
        schema: "reference-first-replay-v1".into(),
        model: "mock".into(),
        effort: "medium".into(),
        protocol_sha256: sha256_hex(b"adapter"),
        cases: vec![ReplayCase {
            id: "positive".into(),
            provenance: "authored positive control, not model calibration".into(),
            expected_ready: true,
            request: request.clone(),
        }],
    };
    let bytes = serde_json::to_vec(&replay).unwrap();
    let receipt = ReplayResult {
        manifest_sha256: sha256_hex(&bytes),
        results: vec![base.clone()],
    };
    assert_eq!(
        replay_score(&bytes, &receipt).unwrap().1.false_rejections,
        0
    );
    let mut wrong_role = replay.clone();
    wrong_role.cases[0].request.inventory.model = "another-model".into();
    let role_bytes = serde_json::to_vec(&wrong_role).unwrap();
    let mut role_result = receipt.clone();
    role_result.manifest_sha256 = sha256_hex(&role_bytes);
    role_result.results[0].request_sha256 = wrong_role.cases[0].request.hash();
    assert!(replay_score(&role_bytes, &role_result).is_err());
    let mut legacy = serde_json::to_value(&replay).unwrap();
    legacy["schema"] = json!("tuning-vision-v3");
    let bytes = serde_json::to_vec(&legacy).unwrap();
    let mut legacy_result = receipt.clone();
    legacy_result.manifest_sha256 = sha256_hex(&bytes);
    assert!(replay_score(&bytes, &legacy_result).is_err());
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
        // Listing the core trait as one the generator cannot draw yet keeps
        // the adoption; the tree still does not read ready.
        let listed = [Unexpressed {
            trait_id: "trait-1".into(),
            spec: "fn-111".into(),
        }];
        let kept = worsened(
            &good.visual.assessment,
            &bad.visual.assessment,
            &r.required,
            &listed,
        );
        assert!(kept.reasons.is_empty(), "{kept:?}");
        assert_eq!(kept.notes.len(), usize::from(status == CellStatus::Fail));
        assert_eq!(
            bad.visual.assessment.cells, base.visual.assessment.cells,
            "global coverage does not rewrite per-view observations"
        );
    }
    let mut per_view_unknown = base.clone();
    per_view_unknown.visual.assessment.cells[0].1 = CellStatus::Unknown;
    per_view_unknown.coverage[0].status = CellStatus::Fail;
    per_view_unknown.bind(&request).unwrap();
    assert_eq!(
        per_view_unknown.visual.assessment.cells[0].1,
        CellStatus::Unknown
    );
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

#[test]
fn preparation_is_verified_charged_once_and_never_inferred_from_a_floor() {
    let r = request();
    let inventory = inventory(&r);
    let dir = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    std::fs::create_dir(&dir).unwrap();
    let ip = dir.join("inventory.json");
    let pp = dir.join("preparation.json");
    let receipt = json!({"status":"ok","model":"mock","effort":"medium","request_sha256":inventory.request_sha256,"prompt_sha256":inventory.prompt_sha256,"usage":{"input_tokens":7,"output_tokens":3},"answer":{"traits":inventory.traits,"observations":inventory.observations}});
    std::fs::write(&ip, serde_json::to_vec(&inventory).unwrap()).unwrap();
    std::fs::write(&pp, serde_json::to_vec(&receipt).unwrap()).unwrap();
    let config = RuntimeConfig {
        inventory: FilePin {
            path: ip.clone(),
            sha256: sha256_hex(&std::fs::read(&ip).unwrap()),
        },
        preparation: FilePin {
            path: pp.clone(),
            sha256: sha256_hex(&std::fs::read(&pp).unwrap()),
        },
    };
    let (_, charge) = config.load("mock", "medium").unwrap();
    let mut budget:telperion_jev::tuning::state::Budget=serde_json::from_value(json!({"evaluations":0,"images":0,"tokens":100,"rounds":0,"max_evaluations":2,"max_images":8,"max_tokens":120,"max_rounds":1,"visual_passes":0,"max_visual_passes":2})).unwrap();
    let mut proof = None;
    assert!(verify_preparation(Some(&charge), proof.as_ref()).is_err());
    charge_preparation(&mut budget, &mut proof, &charge).unwrap();
    assert_eq!(budget.tokens, 110);
    assert_eq!(budget.visual_passes, Some(1));
    charge_preparation(&mut budget, &mut proof, &charge).unwrap();
    assert_eq!(budget.tokens, 110);
    let mut changed = charge.clone();
    changed.preparation_sha256 = sha256_hex(b"different");
    assert!(charge_preparation(&mut budget, &mut proof, &changed).is_err());
    assert_eq!(budget.tokens, 110);
    let mut low = budget.clone();
    low.max_tokens = Some(115);
    let mut empty = None;
    assert!(charge_preparation(&mut low, &mut empty, &charge).is_err());
    assert_eq!(low.tokens, 110);
    assert!(empty.is_none());
    std::fs::write(&pp, b"changed").unwrap();
    assert!(config.load("mock", "medium").is_err());
    let mut unknown = receipt.clone();
    unknown["usage"] = serde_json::Value::Null;
    std::fs::write(&pp, serde_json::to_vec(&unknown).unwrap()).unwrap();
    let mut unknown_config = config;
    unknown_config.preparation.sha256 = sha256_hex(&std::fs::read(&pp).unwrap());
    assert!(unknown_config.load("mock", "medium").is_err());
    assert!(verify_preparation(None, None).is_ok());
    std::fs::remove_dir_all(dir).unwrap();
}

#[test]
fn live_reference_first_preserves_owner_requirements_and_projects_trait_evidence() {
    use telperion_jev::tuning::{
        engine::Services,
        evaluation::{Comparison, Trial},
        live::{Config, Live},
    };
    struct Never;
    impl telperion_jev::caller::Transport for Never {
        fn send(
            &self,
            _: &telperion_jev::caller::HttpRequest,
        ) -> Result<telperion_jev::caller::HttpResponse, String> {
            panic!("no paid text request")
        }
    }
    let r = request();
    let i = inventory(&r);
    let dir = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    std::fs::create_dir(&dir).unwrap();
    let inv = dir.join("inventory.json");
    let prep = dir.join("prep.json");
    let shots = dir.join("shots.json");
    let protocol = dir.join("adapter.py");
    std::fs::write(&inv, serde_json::to_vec(&i).unwrap()).unwrap();
    std::fs::write(&prep,serde_json::to_vec(&json!({"status":"ok","model":"mock","effort":"medium","request_sha256":i.request_sha256,"prompt_sha256":i.prompt_sha256,"usage":{"input_tokens":7,"output_tokens":3},"answer":{"traits":i.traits,"observations":i.observations}})).unwrap()).unwrap();
    std::fs::write(
        &shots,
        b"{\"references\":[{\"id\":\"whole\",\"shot\":{\"foliage\":\"leaf-on\"}}]}",
    )
    .unwrap();
    let script = r#"import json,sys
e=json.load(sys.stdin);r=e['request'];c=r['comparison']
assert r['production_requirements'] is True
assert 'OWNER_SECRET' in c['checklist'] and 'authoritative goal' in c['checklist']
assert c['required'][0]['item']=='OWNER_SECRET'
print(json.dumps({'status':'ok','model':'mock','effort':'medium','request_sha256':e['request_sha256'],'prompt_sha256':e['prompt_sha256'],'usage':{'input_tokens':11,'output_tokens':3},'answer':{'passes':['pass'],'defects':[],'observations':['literal joint observation'],'findings':[{'observation':'Supported reference match','evidence_ids':['render-0','reference-0'],'impact':'supported','uncertain':False,'causal_hypothesis':None}],'coverage':[{'trait_id':'trait-1','status':'pass','evidence_ids':['render-0','reference-0'],'explanation':'visible match'}]}}))
"#;
    std::fs::write(&protocol, script).unwrap();
    let pin =
        |p: &std::path::Path| json!({"path":p,"sha256":sha256_hex(&std::fs::read(p).unwrap())});
    let validation =
        json!({"manifest":dir.join("not-qualified"),"result":dir.join("not-qualified")});
    let mut config:Config=serde_json::from_value(json!({"preset":r.target_species,"seed":1,"initial_overrides":{},"dials":[],"owner_notes":"authoritative goal","measure_binary":protocol,"profiles":shots,"profile_id":"unused","matched":{"headless":protocol,"compare_script":protocol,"references":shots,"refs":dir,"catalogue":dir,"scratch":dir,"height":1440,"numeric_references":["whole"]},"vision":{"program":"python3","args":[protocol],"model":"mock","effort":"medium","timeout_seconds":10,"ledger":dir.join("ledger")},"references":r.references,"required":r.required,"checklist":r.checklist,"quality_anchors":r.quality_anchors,"adjustments":validation,"direction":validation,"continuation":validation,"visual_validation":validation,"vision_protocol":protocol,"reference_first":{"inventory":pin(&inv),"preparation":pin(&prep)},"convergence_run":null,"judgment_model":"mock","ledger":dir,"budget":{"evaluations":0,"images":0,"tokens":0,"rounds":0,"max_evaluations":13,"max_images":52,"max_tokens":100000,"max_rounds":3,"visual_passes":0,"max_visual_passes":5}})).unwrap();
    std::fs::write(
        &config.visual_validation.manifest,
        serde_json::to_vec(&Replay {
            schema: "reference-first-replay-v1".into(),
            model: "mock".into(),
            effort: "medium".into(),
            protocol_sha256: sha256_hex(script.as_bytes()),
            cases: vec![],
        })
        .unwrap(),
    )
    .unwrap();
    let identity = config.identity().unwrap();
    let trial = Trial {
        progress: None,
        adopted_over: vec![],
        bundle: None,
        parent_bundle: None,
        sheet: None,
        vetoed: None,
        adopted: false,
        step: None,
        key: r.identity.clone(),
        identity: identity.clone(),
        seed: 1,
        round: 1,
        label: "mock".into(),
        overrides: json!({}),
        ledger: None,
        feasible: true,
        reason: None,
        measurement: json!({}),
        comparisons: vec![Comparison {
            reference: "whole".into(),
            reference_weight: 1.,
            metric_weights: [1.; 5],
            target: [0.; 5],
            observed: [Some(0.); 5],
            images: r.images.clone(),
        }],
        score: Some(0.),
        seconds: 0.,
        base: None,
        action: None,
        evidence: None,
        direction_mass: None,
        rule: None,
    };
    let mut live = Live {
        config: &config,
        transport: &Never,
        key: "never-used",
    };
    let result = live.visual(&trial).unwrap();
    assert_eq!(result.tokens, Some(14));
    assert!(ready(&config.required, &trial.key, &result.value));
    verify_convergence(
        &config.vision,
        config.reference_first.as_ref().unwrap(),
        &result.value,
        &config.unexpressed,
    )
    .unwrap();
    let mut legacy = result.value.clone();
    legacy.ledger = shots.to_string_lossy().into_owned();
    assert!(verify_convergence(
        &config.vision,
        config.reference_first.as_ref().unwrap(),
        &legacy,
        &config.unexpressed,
    )
    .is_err());
    assert!(result
        .value
        .observations
        .iter()
        .any(|s| s.contains("trait-1")));
    assert!(result
        .value
        .observations
        .iter()
        .any(|s| s.contains("literal joint observation")));
    let mut state:telperion_jev::tuning::engine::Run=serde_json::from_value(json!({"identity":identity,"preset":"fixture","seed":1,"effective":{},"overrides":{},"dials":[],"owner_notes":"authoritative goal","required":config.required,"budget":config.budget,"usage_known":true,"trials":[],"current":null,"visual":result.value,"pause":null,"machine_ready":false,"pending":null,"routes":[]})).unwrap();
    let projection = telperion_jev::tuning::judgments::summary(&state);
    assert!(projection["visual"]["observations"]
        .to_string()
        .contains("trait-1"));
    state.visual = None;
    let before = state.budget.tokens;
    state.execute(&mut live, &mut |_| Ok(())).unwrap();
    assert!(state
        .pause
        .as_ref()
        .unwrap()
        .reason
        .contains("preparation charge"));
    assert_eq!(state.budget.tokens, before);
    let mut ranked_config = config.clone();
    ranked_config.reference_first = None;
    let mut bark = r.images[0].clone();
    bark.view = "bark".into();
    ranked_config.references.push(bark.clone());
    ranked_config
        .required
        .push(telperion_jev::tuning::state::Cell {
            item: "material".into(),
            view: "bark".into(),
            seed: 1,
        });
    let ranked_script = dir.join("ranked.py");
    std::fs::write(&ranked_script,r#"import json,sys
e=json.load(sys.stdin);r=e['request']
assert len(r['required'])==4 and {i['view'] for i in r['images']}=={'whole','bark'}
assert all('leafy' not in c['item'] for c in r['required'] if c['view']=='bark')
assert 'Owner-approved priorities' in r['checklist']
findings=[{'observation':'Supported '+i['view'],'evidence_ids':['render-'+str(n),'reference-'+str(n)],'impact':'supported','uncertain':False,'causal_hypothesis':None} for n,i in enumerate(r['images'])]
print(json.dumps({'request_sha256':e['request_sha256'],'assessment':{'identity':r['identity'],'model':'mock','ledger':'stub','cells':[[c,'pass'] for c in r['required']],'defects':[],'findings':findings},'effort':'medium','usage':{'input_tokens':10,'output_tokens':2},'observations':[]}))
"#).unwrap();
    ranked_config.vision.args = vec![ranked_script.to_string_lossy().into_owned()];
    std::fs::write(&shots,b"{\"references\":[{\"id\":\"whole\",\"shot\":{\"foliage\":\"leaf-on\"}},{\"id\":\"bark\",\"shot\":{\"foliage\":\"hidden\"}}]}").unwrap();
    let mut ranked_trial = trial.clone();
    ranked_trial.round = 0;
    let mut comparison = ranked_trial.comparisons[0].clone();
    comparison.reference = "bark".into();
    comparison.images = vec![bark];
    ranked_trial.comparisons.push(comparison);
    let approval:telperion_jev::tuning::priority::Approval=serde_json::from_value(json!({"checkpoint_sha256":"fixture","scope_sha256":"fixture","ordered":[{"id":"owner-leafy","observation":"leafy form","evidence_ids":["render-0","reference-0"],"views":["whole"]},{"id":"owner-bark","observation":"material","evidence_ids":["render-1","reference-1"],"views":["bark"]}]})).unwrap();
    let required =
        telperion_jev::tuning::priority::requirements(&ranked_config.required, Some(&approval));
    let mut ranked_live = Live {
        config: &ranked_config,
        transport: &Never,
        key: "never-used",
    };
    assert_eq!(
        ranked_live.visual_images_for(&ranked_trial, &required, Some(&approval)),
        0
    );
    assert_eq!(
        ranked_live.visual_tokens_for(&ranked_trial, Some(&approval)),
        40000
            + serde_json::to_vec(&approval.ordered).unwrap().len() as u64
            + serde_json::to_vec(&required).unwrap().len() as u64
            + 256
    );
    let ranked = ranked_live
        .visual_for(&ranked_trial, &required, Some(&approval))
        .unwrap();
    assert!(ready(&required, &ranked_trial.key, &ranked.value));
    let scope = ranked_config.priority_scope(&state);
    ranked_config.checklist.push_str(" changed objective");
    assert_ne!(scope, ranked_config.priority_scope(&state));
    // Restore the original source fixture for the independent stale-adapter check.
    std::fs::write(
        &shots,
        b"{\"references\":[{\"id\":\"whole\",\"shot\":{\"foliage\":\"leaf-on\"}}]}",
    )
    .unwrap();
    std::fs::write(&protocol, format!("{script}\n# changed")).unwrap();
    assert_ne!(identity, config.identity().unwrap());
    assert!(live
        .visual(&trial)
        .err()
        .unwrap()
        .contains("protocol changed"));
    std::fs::write(&inv, b"changed").unwrap();
    assert!(config.identity().is_err());
    assert!(config.preparation().is_err());
    config.reference_first = None;
    assert!(config.preparation().unwrap().is_none());
    std::fs::remove_dir_all(dir).unwrap();
}

/// A coverage row naming something that is not an inventory trait - the live
/// wide-table run answered with the required cell's item name - costs the
/// whole paid pass today. It is dropped and recorded instead, and it can
/// never stand in for a trait the inventory does state.
#[test]
fn a_coverage_row_for_an_unknown_trait_is_dropped_and_recorded() {
    let original = request();
    let request = ComparisonRequest::new(&original, inventory(&original));
    let r = request.comparison.clone();
    let finding = Finding {
        observation: "Supported match".into(),
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        impact: Impact::Supported,
        uncertain: false,
        causal_hypothesis: None,
    };
    let visual:vision::Result=serde_json::from_value(json!({"request_sha256":r.hash(),"assessment":{"identity":r.identity,"model":"mock","ledger":"receipt","cells":[[r.required[0],"pass"]],"defects":[],"findings":[finding]},"effort":"medium","usage":{"input_tokens":1,"output_tokens":1},"observations":[]})).unwrap();
    let known = Coverage {
        trait_id: "trait-1".into(),
        status: CellStatus::Pass,
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        explanation: "Visible match; no defining mismatch".into(),
    };
    let stray = Coverage {
        trait_id: "crown-character-density-droop".into(),
        status: CellStatus::Fail,
        evidence_ids: vec!["render-0".into(), "reference-0".into()],
        explanation: "the crown reads too dark and too dense".into(),
    };
    let base = ComparisonResult {
        request_sha256: request.hash(),
        visual,
        coverage: vec![known.clone()],
    };

    let mut extra = base.clone();
    extra.coverage.push(stray.clone());
    extra.bind(&request).unwrap();
    assert_eq!(
        extra
            .coverage
            .iter()
            .map(|c| c.trait_id.clone())
            .collect::<Vec<_>>(),
        vec![known.trait_id.clone()],
        "a row for an unknown trait must leave the bound coverage"
    );
    let recorded = extra
        .visual
        .observations
        .iter()
        .find(|o| o.starts_with("dropped coverage row for unknown trait"))
        .expect("the dropped row is recorded verbatim");
    assert!(
        recorded.contains("crown-character-density-droop")
            && recorded.contains("Fail")
            && recorded.contains("the crown reads too dark and too dense"),
        "{recorded}"
    );
    assert!(
        ready(&r.required, &r.identity, &extra.visual.assessment),
        "a dropped row must not touch readiness"
    );
    let once = serde_json::to_value(&extra).unwrap();
    extra.bind(&request).unwrap();
    assert_eq!(once, serde_json::to_value(&extra).unwrap());

    // It cannot stand in for the trait the inventory does state.
    let mut only_stray = base.clone();
    only_stray.coverage = vec![stray.clone()];
    only_stray.bind(&request).unwrap();
    assert!(!ready(
        &r.required,
        &r.identity,
        &only_stray.visual.assessment
    ));

    // A missing inventory trait still fails, and the rest stays strict.
    let mut missing = base.clone();
    missing.coverage.clear();
    missing.bind(&request).unwrap();
    assert!(!ready(&r.required, &r.identity, &missing.visual.assessment));
    let mut duplicate = base.clone();
    duplicate.coverage.push(known.clone());
    duplicate.coverage.push(stray.clone());
    assert!(duplicate.bind(&request).is_err());
    let mut invalid = base.clone();
    invalid.coverage[0].evidence_ids = vec!["invented".into()];
    invalid.coverage.push(stray.clone());
    assert!(invalid.bind(&request).is_err());
    let mut too_many = base.clone();
    too_many.coverage = (0..17)
        .map(|i| Coverage {
            trait_id: format!("unknown-{i}"),
            ..stray.clone()
        })
        .collect();
    assert!(too_many.bind(&request).is_err());
}
