//! The gap-magnitude question and the labelled set it is trusted against.
//! No call is made: the ledgers here are written by the test.
use serde_json::json;
use std::{fs, path::Path};
use telperion_jev::{
    ledger::derived_identity,
    sha256_hex,
    tuning::{
        calibration::{self, Manifest},
        live::Validation,
        stride::{self, Class, QUESTION},
    },
};

const TABLE: &str = "4f53cda18c2baa0c0354bb5f9a3ecbe5ed12ab4d8e11ba873c2f11161202b945";
const MODEL: &str = "jev-1.13.0";

/// The labelled set frozen the way `tuning-loop freeze` freezes it.
fn frozen() -> Manifest {
    let mut m = stride::labelled();
    m.table_sha256 = TABLE.into();
    for case in &mut m.cases {
        case.questions = calibration::questions(&m.kind, &case.state).unwrap();
    }
    m
}

#[test]
fn the_labelled_set_covers_every_class_and_the_palm_s_words() {
    let m = frozen();
    calibration::validate_manifest(&m).unwrap();
    assert_eq!(m.kind, QUESTION);
    assert_eq!(m.question_version, stride::VERSION);
    let criteria = stride::questions()[QUESTION]["criteria"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(criteria.contains(&"no_match".to_string()));
    for split in ["tuning", "heldout"] {
        for answer in &criteria {
            assert!(
                m.cases
                    .iter()
                    .any(|c| c.split == split && &c.expected[QUESTION] == answer),
                "the {split} split has no {answer} case"
            );
        }
    }
    // The runtime shows Jev exactly the shape the cases were labelled on.
    for c in &m.cases {
        let (p, f) = (&c.state["priority"], &c.state["finding"]);
        assert_eq!(
            stride::shown(p.as_str().unwrap(), f.as_str().unwrap()),
            c.state
        );
    }
    let labelled = |finding: &str| {
        m.cases
            .iter()
            .find(|c| c.state["finding"].as_str().unwrap().contains(finding))
            .map(|c| c.expected[QUESTION].clone())
    };
    for words in ["far too short", "two to three times longer"] {
        assert_eq!(labelled(words).as_deref(), Some("far_off"), "{words}");
    }
    assert!(m.cases.iter().any(|c| c.state["priority"]
        .as_str()
        .unwrap()
        .contains("the fronds are far too thin and short today")));
}

#[test]
fn a_class_is_code_s_multiplier_and_no_match_is_none() {
    for (name, class, multiplier, lower) in [
        ("near", Class::Near, 1., Class::Near),
        ("clearly_off", Class::ClearlyOff, 2., Class::Near),
        ("far_off", Class::FarOff, 4., Class::ClearlyOff),
    ] {
        assert_eq!(Class::parse(name), Some(class));
        assert_eq!(class.name(), name);
        assert_eq!(class.multiplier(), multiplier);
        assert_eq!(class.lower(), lower);
    }
    assert_eq!(Class::parse("no_match"), None);
}

/// A ledger per case answering `answer(case)` at 0.9, and the result set.
fn answered(root: &Path, m: &Manifest, answer: impl Fn(&str) -> String) -> Validation {
    let bytes = serde_json::to_vec_pretty(m).unwrap();
    let manifest = root.join("gap_magnitude.manifest.json");
    fs::write(&manifest, &bytes).unwrap();
    let mut ledgers = serde_json::Map::new();
    for c in &m.cases {
        let state_sha256 = sha256_hex(&serde_json::to_vec(&c.state).unwrap());
        let entry = json!({"id":c.id,"tool":"tuning-calibration","state_sha256":state_sha256,
            "source":null,"model":m.model,"questions":c.questions,
            "answers":{QUESTION:{"choice":answer(&c.expected[QUESTION]),"confidence":0.9}},
            "usage":{"input_tokens":10,"output_tokens":5},"elapsed_ms":1,
            "recorded_at":"2026-09-23T00:00:00Z",
            "identity":derived_identity(&state_sha256, &c.questions, &m.model)});
        let path = root.join(format!("{}.ledger.json", c.id));
        fs::write(&path, serde_json::to_vec(&entry).unwrap()).unwrap();
        ledgers.insert(c.id.clone(), json!(path));
    }
    let result = root.join("gap_magnitude.result.json");
    let set = json!({"manifest_sha256":sha256_hex(&bytes),"ledgers":ledgers});
    fs::write(&result, serde_json::to_vec(&set).unwrap()).unwrap();
    Validation { manifest, result }
}

#[test]
fn the_set_is_trusted_only_once_it_qualifies_for_the_run() {
    let root = std::env::temp_dir().join(telperion_jev::ledger::new_entry_id());
    fs::create_dir_all(&root).unwrap();
    let cut = stride::labelled().min_confidence;
    assert_eq!(stride::calibration(None, MODEL, TABLE), (cut, false));

    let right = answered(&root, &frozen(), str::to_string);
    assert_eq!(stride::calibration(Some(&right), MODEL, TABLE), (cut, true));
    for (why, model, table) in [
        ("another model", "jev-other", TABLE),
        ("another dial table", MODEL, &"0".repeat(64)[..]),
    ] {
        assert!(!stride::calibration(Some(&right), model, table).1, "{why}");
    }
    // Calling a near gap far off is the unsafe answer: it overshoots.
    let rash = answered(&root, &frozen(), |e: &str| {
        if e == "near" { "far_off" } else { e }.to_string()
    });
    assert!(!stride::calibration(Some(&rash), MODEL, TABLE).1);
    fs::remove_dir_all(&root).unwrap();
}
