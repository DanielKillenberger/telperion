mod common;

use telperion_jev::cases::{format_scores, CaseRow, SetScore};
use telperion_jev::pipeline::sets::cases::run_pipeline_cases;
use telperion_jev::pipeline::sets::{
    described_cases, described_questions, level_from_score, mature_cases, mature_questions,
    missed_ids, obligation_cases, obligation_questions, ranking_cases, ranking_questions,
    set_version, sufficiency_cases, DescribedLevel, DESCRIBED_UNSTATED, OBLIGATION_NAMES,
    RANKING_NONE, SUFFICIENCY_LEVELS,
};

use common::{ledger_dir, CaseTransport};
use telperion_jev::pipeline::requirements::table;

/// Every base name the runner scores, labelled and held out.
const SET_NAMES: [&str; 9] = [
    "sufficiency level",
    "sufficiency gap",
    "mature size level",
    "mature size gap",
    "ranking source",
    "described level",
    "obligation inspected_image",
    "obligation measurement_not_invention",
    "obligation appearance_supported",
];

#[test]
fn every_set_is_version_one_and_its_cases_carry_the_fields_the_runner_reads() {
    for name in [
        "sufficiency",
        "mature_size",
        "ranking",
        "described",
        "obligations",
    ] {
        assert_eq!(set_version(name), 1, "{name}");
    }
    let gaps = [
        "no_age_indexed_points",
        "wrong_condition",
        "wrong_taxon",
        "age_range_uncovered",
        "none",
    ];
    let sufficiency = sufficiency_cases();
    for case in &sufficiency {
        assert!(
            SUFFICIENCY_LEVELS.contains(&case.expect_level.as_str()),
            "{}",
            case.id
        );
        assert!(gaps.contains(&case.expect_gap.as_str()), "{}", case.id);
        assert!(
            case.requirement["required_ages_years"].is_array(),
            "{}",
            case.id
        );
        assert!(case.evidence.is_array(), "{}", case.id);
        assert!(case.counts["measured_points"].is_number(), "{}", case.id);
    }
    // fn-127: the mature-size set asks no age and answers on the same levels.
    let mature = mature_cases();
    let mature_gaps = mature_questions()["mature_gap"]["criteria"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    assert!(mature_gaps.contains(&"none".to_string()), "a no-match gap");
    for case in &mature {
        assert!(
            SUFFICIENCY_LEVELS.contains(&case.expect_level.as_str()),
            "{}",
            case.id
        );
        assert!(mature_gaps.contains(&case.expect_gap), "{}", case.id);
        assert!(
            case.requirement.get("required_ages_years").is_none(),
            "{}",
            case.id
        );
        assert!(case.counts["sentences"].is_number(), "{}", case.id);
    }
    for id in ["palm-leaflet-length-a1", "palm-crown-width-a1"] {
        assert!(
            mature.iter().any(|c| c.id == id),
            "A1's sentences label {id}"
        );
    }
    let ranking = ranking_cases();
    for case in &ranking {
        assert!(!case.candidates.is_empty(), "{}", case.id);
        let known = case.expect_source == RANKING_NONE
            || case.candidates.iter().any(|c| c.id == case.expect_source);
        assert!(known, "{} admits a source not in its candidates", case.id);
    }
    let described = described_cases();
    for case in &described {
        assert!(!case.sentences.is_empty(), "{}", case.id);
        let known = case.expect_level == DESCRIBED_UNSTATED
            || case.levels.iter().any(|l| l.key == case.expect_level);
        assert!(known, "{} admits a level not in its table", case.id);
    }
    let obligations = obligation_cases();
    for case in &obligations.inspected_image {
        assert!(!case.observation.is_empty(), "{}", case.id);
    }
    for case in &obligations.measurement_not_invention {
        assert!(!case.value_statement.is_empty(), "{}", case.id);
        assert!(!case.source_excerpt.is_empty(), "{}", case.id);
    }
    for case in &obligations.appearance_supported {
        assert!(!case.sentence.is_empty(), "{}", case.id);
        let level = table().level(&case.trait_name, &case.level);
        assert!(level.is_some(), "{} names a level the table has", case.id);
    }

    let counts: [(&str, usize, usize, usize); 7] = [
        (
            "mature_size",
            mature.len(),
            mature.iter().filter(|c| c.holdout).count(),
            mature.iter().filter(|c| c.negative).count(),
        ),
        (
            "sufficiency",
            sufficiency.len(),
            sufficiency.iter().filter(|c| c.holdout).count(),
            sufficiency.iter().filter(|c| c.negative).count(),
        ),
        (
            "ranking",
            ranking.len(),
            ranking.iter().filter(|c| c.holdout).count(),
            ranking.iter().filter(|c| c.negative).count(),
        ),
        (
            "described",
            described.len(),
            described.iter().filter(|c| c.holdout).count(),
            described.iter().filter(|c| c.negative).count(),
        ),
        (
            "inspected_image",
            obligations.inspected_image.len(),
            obligations
                .inspected_image
                .iter()
                .filter(|c| c.holdout)
                .count(),
            obligations
                .inspected_image
                .iter()
                .filter(|c| c.negative)
                .count(),
        ),
        (
            "measurement_not_invention",
            obligations.measurement_not_invention.len(),
            obligations
                .measurement_not_invention
                .iter()
                .filter(|c| c.holdout)
                .count(),
            obligations
                .measurement_not_invention
                .iter()
                .filter(|c| c.negative)
                .count(),
        ),
        (
            "appearance_supported",
            obligations.appearance_supported.len(),
            obligations
                .appearance_supported
                .iter()
                .filter(|c| c.holdout)
                .count(),
            obligations
                .appearance_supported
                .iter()
                .filter(|c| c.negative)
                .count(),
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
    let ranking = sets
        .iter()
        .find(|set| set.name == "ranking source")
        .unwrap();
    assert_eq!(
        ranking.required,
        (0.8 * ranking.total as f64).ceil() as usize
    );
    let level = sets
        .iter()
        .find(|set| set.name == "sufficiency level")
        .unwrap();
    assert_eq!(level.required, (0.9 * level.total as f64).ceil() as usize);
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

    let candidates = vec!["S1".to_string(), "G1".to_string()];
    let ranking = ranking_questions(&candidates);
    let criteria = ranking["source"]["criteria"]
        .as_object()
        .expect("criteria map");
    for id in &candidates {
        assert!(criteria.contains_key(id), "{id} is a candidate");
    }
    assert!(
        criteria[RANKING_NONE].is_string(),
        "the no-match key carries its text"
    );
    assert_eq!(
        criteria.keys().next_back().map(String::as_str),
        Some(RANKING_NONE)
    );

    for name in OBLIGATION_NAMES {
        let noul = obligation_questions(name);
        assert_eq!(noul.as_object().unwrap().len(), 1, "{name} is asked alone");
        assert_eq!(noul[name]["type"], "noul");
        assert!(
            noul[name]["criteria"]["false"].is_string(),
            "{name} has a false criterion"
        );
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
