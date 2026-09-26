mod common;

use serde_json::json;
use telperion_jev::cases::{format_scores, CaseRow, SetScore};
use telperion_jev::pipeline::sets::cases::run_pipeline_cases;
use telperion_jev::pipeline::sets::{
    chosen_level, described_cases, described_questions, kind_cases, kind_questions, label_cases,
    label_questions, level_from_score, missed_ids, set_version, DescribedLevel, DESCRIBED_UNSTATED,
    KINDS, KIND_UNCLEAR, LABEL_NONE, LABEL_QUESTIONS,
};

use common::{ledger_dir, CaseTransport};
use telperion_jev::pipeline::requirements::table;

/// Every base name the runner scores, labelled and held out.
const SET_NAMES: [&str; 7] = [
    "described level",
    "label field",
    "label basis",
    "label age",
    "label condition",
    "document kind",
    "rights class",
];

#[test]
fn every_set_carries_its_version_and_its_cases_carry_the_fields_the_runner_reads() {
    for (name, version) in [("described", 1), ("label", 1), ("kind", 1)] {
        assert_eq!(set_version(name), version, "{name}");
    }
    let described = described_cases();
    for case in &described {
        assert!(!case.sentences.is_empty(), "{}", case.id);
        let known = case.expect_level == DESCRIBED_UNSTATED
            || case.levels.iter().any(|l| l.key == case.expect_level);
        assert!(known, "{} admits a level not in its table", case.id);
    }
    let labels = label_cases();
    let questions = label_questions(&[]);
    for case in &labels {
        for field in &case.fields {
            assert!(table().fields.contains_key(field), "{}: {field}", case.id);
        }
        let known = case.expect_field == LABEL_NONE || case.fields.contains(&case.expect_field);
        assert!(known, "{} admits a field it does not offer", case.id);
        for (name, want) in LABEL_QUESTIONS.iter().zip([
            &case.expect_basis,
            &case.expect_age,
            &case.expect_condition,
        ]) {
            assert!(
                questions[name]["criteria"].get(want.as_str()).is_some(),
                "{}: {name} {want}",
                case.id
            );
        }
        assert_eq!(
            case.negative,
            case.expect_field == LABEL_NONE,
            "{}",
            case.id
        );
    }
    let kinds = kind_cases();
    let offered = kind_questions()["document"]["criteria"].clone();
    for case in &kinds {
        assert!(offered.get(&case.expect_kind).is_some(), "{}", case.id);
        assert_eq!(
            case.negative,
            case.expect_kind == KIND_UNCLEAR,
            "{}",
            case.id
        );
    }
    for kind in KINDS {
        assert!(offered.get(kind).is_some(), "{kind} is offered");
    }
    let counts = [
        (
            "described",
            described.len(),
            described.iter().filter(|c| c.holdout).count(),
            described.iter().filter(|c| c.negative).count(),
        ),
        (
            "kind",
            kinds.len(),
            kinds.iter().filter(|c| c.holdout).count(),
            kinds.iter().filter(|c| c.negative).count(),
        ),
        (
            "label",
            labels.len(),
            labels.iter().filter(|c| c.holdout).count(),
            labels.iter().filter(|c| c.negative).count(),
        ),
    ];
    for (name, total, holdout, negative) in counts {
        assert!(total >= 10, "{name} has {total} cases");
        assert!(holdout >= 3, "{name} holds out {holdout}");
        assert!(negative >= 3, "{name} has {negative} negatives");
    }
}

#[test]
fn the_runner_scores_every_set_labelled_and_held_out_against_its_bound() {
    // The mock transport answers every call; nothing here reaches the network.
    let dir = ledger_dir("pipeline-sets");
    let sets = run_pipeline_cases(&CaseTransport, "test-key", &dir).expect("pipeline cases");
    assert_eq!(sets.len(), SET_NAMES.len() * 2);
    for name in SET_NAMES {
        for full in [name.to_string(), format!("{name} (held-out)")] {
            let set = sets
                .iter()
                .find(|set| set.name == full)
                .unwrap_or_else(|| panic!("{full} is scored"));
            assert!(set.total > 0, "{full} has no cases");
            assert_eq!(set.hits, set.total, "{full} missed {:?}", missed_ids(set));
            assert!(set.meets_pilot(), "{full} is below its bound");
        }
    }
    let field = sets.iter().find(|set| set.name == "label field").unwrap();
    assert_eq!(field.required, (0.9 * field.total as f64).ceil() as usize);
    assert!(format_scores(&sets).contains("conf min="));
}

#[test]
fn level_from_score_rounds_to_the_nearest_level_and_clamps() {
    let table = [
        (-4.0, 4, Some(0)),
        (0.0, 4, Some(0)),
        (0.4, 4, Some(0)),
        (0.6, 4, Some(1)),
        (1.5, 4, Some(2)),
        (2.49, 4, Some(2)),
        (3.0, 4, Some(3)),
        (9.0, 4, Some(3)),
        (2.0, 1, Some(0)),
        (f64::NAN, 4, None),
        (f64::INFINITY, 4, None),
    ];
    for (score, levels, expected) in table {
        assert_eq!(
            level_from_score(score, levels),
            expected,
            "{score} over {levels}"
        );
    }
    assert_eq!(level_from_score(1.0, 0), None);
}

/// fn-131 R3 and R9: a level is the most probable one; a tie, no map or a
/// most probable level below the floor is the no-match level.
#[test]
fn chosen_level_is_the_most_probable_or_the_no_match() {
    let table = [
        (
            json!({"0": 0.0, "1": 0.0, "2": 0.46, "3": 0.0, "4": 0.54}),
            5,
            4,
            0.0,
            4,
        ),
        (
            json!({"0": 0.17, "1": 0.09, "2": 0.0, "3": 0.74}),
            4,
            3,
            0.0,
            3,
        ),
        (
            json!({"0": 0.6, "1": 0.0, "2": 0.0, "3": 0.4}),
            4,
            0,
            0.0,
            0,
        ),
        (json!({"0": 0.1, "1": 0.2, "2": 0.7}), 3, 0, 0.0, 2),
        (json!({"0": 0.45, "1": 0.45, "2": 0.1}), 3, 0, 0.0, 0),
        (json!({"0": 0.5, "1": 0.5}), 2, 1, 0.0, 1),
        (json!({"0": 0.5, "1": 0.3, "2": 0.2}), 3, 2, 0.56, 2),
        (json!({}), 3, 2, 0.0, 2),
    ];
    for (probabilities, count, no_match, floor, expected) in table {
        assert_eq!(
            chosen_level(Some(&probabilities), count, no_match, floor),
            expected,
            "{probabilities} floor {floor}"
        );
    }
    assert_eq!(chosen_level(None, 3, 0, 0.0), 0);
}

#[test]
fn built_questions_keep_the_table_order_and_offer_a_no_match_answer() {
    let levels = vec![
        DescribedLevel {
            key: "narrow_upright".into(),
            summary: "taller than wide".into(),
        },
        DescribedLevel {
            key: "rounded".into(),
            summary: "as wide as tall".into(),
        },
    ];
    let described = described_questions(&levels);
    let criteria = described["level"]["criteria"]
        .as_array()
        .expect("criteria list");
    let keys: Vec<&str> = criteria
        .iter()
        .map(|c| c["key"].as_str().unwrap())
        .collect();
    assert_eq!(keys, ["narrow_upright", "rounded", DESCRIBED_UNSTATED]);
    assert!(criteria.last().unwrap()["summary"]
        .as_str()
        .unwrap()
        .contains("not describe"));

    // The label offers the manifest's fields and `none`, and a no-match
    // answer to each of its other questions (fn-157).
    let fields = [
        (
            "leaf_length_m".to_string(),
            "The length of a leaf".to_string(),
        ),
        (
            "height_m".to_string(),
            "The height of the whole tree".to_string(),
        ),
    ];
    let label = label_questions(&fields);
    let criteria = label["field"]["criteria"]
        .as_object()
        .expect("criteria map");
    let mut keys: Vec<&str> = criteria.keys().map(String::as_str).collect();
    keys.sort();
    assert_eq!(keys, ["height_m", "leaf_length_m", LABEL_NONE]);
    assert!(
        criteria[LABEL_NONE].is_string(),
        "the no-match key carries its text"
    );
    assert!(label["basis"]["criteria"]["unclear"].is_string());
    assert!(label["condition"]["criteria"]["unstated"].is_string());
    for name in LABEL_QUESTIONS {
        assert_eq!(label[name]["type"], "choice", "{name}");
    }
}

#[test]
fn missed_ids_names_the_cases_a_set_missed() {
    let row = |id: &str, hit: bool| CaseRow {
        set: "described level".into(),
        id: id.into(),
        expected: "rounded".into(),
        answered: if hit { "rounded".into() } else { "oval".into() },
        top_probability: 0.9,
        confidence: 0.85,
        ledger: "described:1".into(),
        hit,
    };
    let set = SetScore {
        name: "described level".into(),
        hits: 1,
        total: 3,
        required: 3,
        ranking_ok: None,
        confidences: vec![0.85; 3],
        rows: vec![row("a", false), row("b", true), row("c", false)],
    };
    assert_eq!(missed_ids(&set), vec!["a".to_string(), "c".to_string()]);
    assert!(!set.meets_pilot());
}

/// fn-129 R4: the rights classes are labelled on the palm's own sources and
/// the question offers a no-match answer that some cases need.
#[test]
fn the_rights_set_labels_the_palms_sources_and_offers_a_no_match_answer() {
    use telperion_jev::pipeline::rights::{
        rights_questions, set_version, OPEN_LICENCE, PUBLIC_CITE_ONLY, RESTRICTED, RIGHTS_NONE,
    };
    assert_eq!(set_version(), 1);
    let criteria = rights_questions()["rights"]["criteria"]
        .as_object()
        .unwrap()
        .clone();
    let classes = [OPEN_LICENCE, PUBLIC_CITE_ONLY, RESTRICTED, RIGHTS_NONE];
    assert_eq!(criteria.len(), classes.len());
    for class in classes {
        assert!(criteria[class].is_string(), "{class} is offered");
    }
    let cases = telperion_jev::pipeline::rights::rights_cases();
    for class in classes {
        assert!(
            cases.iter().any(|c| c.expect_class == class),
            "{class} has a labelled case"
        );
    }
    for source in ["f1", "a1", "m1", "p4", "p5", "p6", "p7"] {
        let prefix = format!("palm-{source}-");
        assert!(
            cases.iter().any(|c| c.id.starts_with(&prefix)),
            "the palm's {source} is labelled"
        );
    }
    let negatives = cases.iter().filter(|c| c.negative).count();
    let rejects = cases
        .iter()
        .filter(|c| c.expect_class == RESTRICTED || c.expect_class == RIGHTS_NONE)
        .count();
    assert_eq!(negatives, rejects, "a negative case is one that rejects");
    assert!(cases.len() >= 10 && negatives >= 3);
    assert!(cases.iter().filter(|c| c.holdout).count() >= 3);
}
