//! The described and reference routes, over a measurer the test authors.
//!
//! No test calls Jev, reaches the network, needs a GPU or needs a built
//! example: the render-and-measure step is a table of dial values mapped to
//! measured metrics, and every call it takes is logged.

/// Named so `cargo test -p telperion-jev routes` runs every test below.
mod routes {
    use std::{path::PathBuf, sync::Mutex};

    use serde_json::{json, Value};
    use telperion_jev::pipeline::render::{family_for, metric, Measured, Measurer, RenderError};
    use telperion_jev::pipeline::routes::{
        described, ranking, transfer, Candidate, DescribedEntry, DescribedInput, DescribedOutcome,
        Level, Reference, RelationLevel, TransferEntry, TransferInput, TransferOutcome, ValueTable,
    };

    /// A render-and-measure step spelled out as a table: each dial value measures
    /// what the row says, or refuses when the row says `None`.
    struct MockMeasurer {
        dial: String,
        measured_metric: String,
        rows: Vec<(f64, Option<f64>)>,
        calls: Mutex<Vec<(String, u32, Value)>>,
    }

    impl MockMeasurer {
        fn new(dial: &str, measured_metric: &str, rows: &[(f64, Option<f64>)]) -> Self {
            Self {
                dial: dial.into(),
                measured_metric: measured_metric.into(),
                rows: rows.to_vec(),
                calls: Mutex::new(Vec::new()),
            }
        }

        fn calls(&self) -> Vec<(String, u32, Value)> {
            self.calls.lock().unwrap().clone()
        }
    }

    fn dial_value(family: &Value, dial: &str) -> Option<f64> {
        let mut node = family;
        for key in dial.split('.') {
            node = node.get(key)?;
        }
        node.as_f64()
    }

    impl Measurer for MockMeasurer {
        fn measure(
            &self,
            preset: &str,
            seed: u32,
            family: &Value,
        ) -> Result<Measured, RenderError> {
            self.calls
                .lock()
                .unwrap()
                .push((preset.to_owned(), seed, family.clone()));
            let value = dial_value(family, &self.dial)
                .ok_or_else(|| RenderError::Measure(format!("no {} in the family", self.dial)))?;
            let row = self
                .rows
                .iter()
                .find(|(candidate, _)| (candidate - value).abs() < 1e-9)
                .ok_or_else(|| RenderError::Measure(format!("no row for {value}")))?;
            let measured = row
                .1
                .ok_or_else(|| RenderError::Measure(format!("generation refused {value}")))?;
            let mut metrics = json!({});
            metrics[self.measured_metric.as_str()] =
                json!({"status": "measured", "value": measured});
            Ok(Measured {
                metrics,
                receipt_path: PathBuf::from(format!("receipts/{value}.jsonl")),
            })
        }
    }

    const DIAL: &str = "skeleton.envelope.spread";
    const METRIC: &str = "crown_width_height_ratio";

    const LEVEL: &str = "as_wide_as_tall";
    const LEDGER: &str = "ledger/described/ab12cd";

    fn input(candidates: &[f64], target_range: [f64; 2], level_key: &str) -> DescribedInput {
        DescribedInput {
            table: ValueTable {
                trait_name: "crown habit".into(),
                measured_metric: METRIC.into(),
                dial: DIAL.into(),
                levels: vec![Level {
                    key: LEVEL.into(),
                    summary: "a crown about as wide as the tree is tall".into(),
                    target_range,
                    candidates: candidates.to_vec(),
                }],
            },
            level_key: level_key.into(),
            sentence: "The crown is broad and rounded, about as wide as the tree is tall.".into(),
            ledger: LEDGER.into(),
            preset: "oregon-white-oak".into(),
            seed: 1,
        }
    }

    /// The described route over the scored level, which is the ordinary case.
    fn run(candidates: &[f64], range: [f64; 2], measurer: &MockMeasurer) -> DescribedOutcome {
        described(&input(candidates, range, LEVEL), measurer).unwrap()
    }

    fn shipped(outcome: DescribedOutcome) -> DescribedEntry {
        match outcome {
            DescribedOutcome::Shipped(entry) => entry,
            other => panic!("expected a shipped value, got {other:?}"),
        }
    }

    #[test]
    fn described_ships_the_in_range_candidate_nearest_the_midpoint() {
        let measurer = MockMeasurer::new(
            DIAL,
            METRIC,
            &[
                (0.30, Some(0.42)),
                (0.40, Some(0.52)),
                (0.50, Some(0.58)),
                (0.60, Some(0.71)),
            ],
        );
        let entry = shipped(run(&[0.30, 0.40, 0.50, 0.60], [0.40, 0.60], &measurer));
        assert_eq!(entry.route, "described");
        assert_eq!(entry.shipped, 0.40);
        assert_eq!(entry.level, LEVEL);
        assert_eq!(entry.dial, DIAL);
        assert_eq!(entry.ledger, vec![LEDGER.to_owned()]);
        assert_eq!(entry.candidates[0].measured, Some(0.42));
        assert!(entry.candidates[0].in_range);
        assert!(!entry.candidates[3].in_range);
        assert_eq!(entry.candidates[1].receipt, "receipts/0.4.jsonl");
    }

    #[test]
    fn described_breaks_a_tie_by_table_order() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, Some(0.40)), (0.50, Some(0.60))]);
        let entry = shipped(run(&[0.30, 0.50], [0.40, 0.60], &measurer));
        assert_eq!(entry.shipped, 0.30);
        assert_eq!(entry.ranking, vec![0, 1]);
    }

    #[test]
    fn described_with_no_candidate_in_range_is_a_level_miss() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, Some(0.20)), (0.50, Some(0.90))]);
        let outcome = run(&[0.30, 0.50], [0.40, 0.60], &measurer);
        let DescribedOutcome::LevelMiss(miss) = outcome else {
            panic!("expected a level miss, got {outcome:?}");
        };
        assert_eq!(miss.trait_name, "crown habit");
        assert_eq!(miss.level, "as_wide_as_tall");
        assert_eq!(miss.target_range, [0.40, 0.60]);
        let measured: Vec<_> = miss.candidates.iter().map(|c| c.measured).collect();
        assert_eq!(measured, vec![Some(0.20), Some(0.90)]);
        assert!(miss.candidates.iter().all(|c| !c.in_range));
    }

    #[test]
    fn described_with_an_unknown_level_is_unavailable() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, Some(0.50))]);
        let outcome =
            described(&input(&[0.30], [0.40, 0.60], "clearly_narrow"), &measurer).unwrap();
        let DescribedOutcome::Unavailable { reason } = outcome else {
            panic!("expected unavailable, got {outcome:?}");
        };
        assert!(reason.contains("clearly_narrow"), "{reason}");
        assert!(measurer.calls().is_empty());
    }

    #[test]
    fn described_with_no_candidate_values_is_unavailable() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[]);
        let outcome = run(&[], [0.40, 0.60], &measurer);
        let DescribedOutcome::Unavailable { reason } = outcome else {
            panic!("expected unavailable, got {outcome:?}");
        };
        assert!(reason.contains("no candidate value"), "{reason}");
        assert!(measurer.calls().is_empty());
    }

    #[test]
    fn described_with_a_reversed_range_is_unavailable() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, Some(0.50))]);
        let outcome = run(&[0.30], [0.60, 0.40], &measurer);
        assert!(matches!(outcome, DescribedOutcome::Unavailable { .. }));
        assert!(measurer.calls().is_empty());
    }

    #[test]
    fn described_measures_every_candidate_exactly_once() {
        let measurer = MockMeasurer::new(
            DIAL,
            METRIC,
            &[(0.30, Some(0.42)), (0.40, Some(0.52)), (0.50, Some(0.58))],
        );
        run(&[0.30, 0.40, 0.50], [0.40, 0.60], &measurer);
        let calls = measurer.calls();
        assert_eq!(calls.len(), 3);
        for (call, value) in calls.iter().zip([0.30, 0.40, 0.50]) {
            assert_eq!(call.0, "oregon-white-oak");
            assert_eq!(call.1, 1);
            assert_eq!(call.2, family_for(DIAL, value));
        }
    }

    #[test]
    fn described_keeps_the_others_selectable_past_a_measurement_error() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, None), (0.40, Some(0.52))]);
        let entry = shipped(run(&[0.30, 0.40], [0.40, 0.60], &measurer));
        assert_eq!(entry.shipped, 0.40);
        assert_eq!(entry.candidates[0].measured, None);
        assert!(!entry.candidates[0].in_range);
        assert!(
            entry.candidates[0].receipt.starts_with("unmeasured: "),
            "{}",
            entry.candidates[0].receipt
        );
        assert_eq!(measurer.calls().len(), 2);
    }

    #[test]
    fn ranking_puts_in_range_first_then_distance_then_table_order() {
        let candidate = |value: f64, measured: Option<f64>, in_range: bool| Candidate {
            value,
            measured,
            in_range,
            receipt: "receipts/r.jsonl".into(),
        };
        let candidates = [
            candidate(0.1, Some(0.90), false),
            candidate(0.2, Some(0.58), true),
            candidate(0.3, None, false),
            candidate(0.4, Some(0.50), true),
            candidate(0.5, Some(0.62), false),
        ];
        assert_eq!(ranking(&candidates, [0.40, 0.60]), vec![3, 1, 4, 0, 2]);
    }

    fn relation_levels() -> Vec<RelationLevel> {
        [
            ("clearly_smaller", 0.5),
            ("smaller", 0.75),
            ("about_the_same", 1.0),
            ("larger", 1.33),
            ("clearly_larger", 2.0),
        ]
        .into_iter()
        .map(|(key, multiplier)| RelationLevel {
            key: key.into(),
            summary: format!("a crown {key} than the reference's"),
            multiplier,
        })
        .collect()
    }

    fn transfer_input(nearest: &str, relation: &str, references: Vec<Reference>) -> TransferInput {
        TransferInput {
            dial: DIAL.into(),
            references,
            nearest: nearest.into(),
            second: None,
            relation: relation.into(),
            levels: relation_levels(),
            target_range: None,
            measured_metric: Some(METRIC.into()),
            forbid_other_growth_form: false,
            growth_form: "tree".into(),
            ledger: vec!["ledger/transfer/9f01".into()],
            preset: "european-beech".into(),
            seed: 1,
        }
    }

    fn oak() -> Reference {
        Reference {
            template: "oregon-white-oak".into(),
            shipped_value: 0.40,
            growth_form: "tree".into(),
        }
    }

    fn transferred(outcome: TransferOutcome) -> TransferEntry {
        match outcome {
            TransferOutcome::Shipped(entry) => entry,
            other => panic!("expected a shipped value, got {other:?}"),
        }
    }

    #[test]
    fn transfer_computes_the_candidate_from_the_reference_and_the_level() {
        for (relation, expected) in [
            ("clearly_smaller", 0.20),
            ("smaller", 0.30),
            ("about_the_same", 0.40),
            ("larger", 0.40 * 1.33),
            ("clearly_larger", 0.80),
        ] {
            let measurer = MockMeasurer::new(DIAL, METRIC, &[(expected, Some(0.55))]);
            let entry = transferred(
                transfer(
                    &transfer_input("oregon-white-oak", relation, vec![oak()]),
                    &measurer,
                )
                .unwrap(),
            );
            assert_eq!(entry.route, "reference");
            assert_eq!(entry.reference, "oregon-white-oak");
            assert_eq!(entry.relation, relation);
            assert!((entry.candidate - expected).abs() < 1e-9, "{relation}");
            assert_eq!(entry.measured, Some(0.55));
            assert_eq!(entry.second, None);
            assert_eq!(measurer.calls().len(), 1);
        }
    }

    #[test]
    fn transfer_interpolates_between_two_bracketing_references() {
        let spruce = Reference {
            template: "norway-spruce".into(),
            shipped_value: 1.00,
            growth_form: "tree".into(),
        };
        let mut bracketing = transfer_input(
            "oregon-white-oak",
            "clearly_larger",
            vec![oak(), spruce.clone()],
        );
        bracketing.second = Some("norway-spruce".into());
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.90, Some(0.61))]);
        let entry = transferred(transfer(&bracketing, &measurer).unwrap());
        assert!((entry.candidate - 0.90).abs() < 1e-9, "{}", entry.candidate);
        assert_eq!(entry.second.as_deref(), Some("norway-spruce"));

        // A second reference on the reference's own side of the scaled value does
        // not bracket the new species, and the candidate stands unchanged.
        let birch = Reference {
            template: "silver-birch".into(),
            shipped_value: 0.60,
            growth_form: "tree".into(),
        };
        let mut beside = transfer_input("oregon-white-oak", "clearly_larger", vec![oak(), birch]);
        beside.second = Some("silver-birch".into());
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.80, Some(0.59))]);
        let entry = transferred(transfer(&beside, &measurer).unwrap());
        assert!((entry.candidate - 0.80).abs() < 1e-9, "{}", entry.candidate);
        assert_eq!(entry.second, None);
    }

    #[test]
    fn transfer_with_a_nearest_answer_of_none_files_no_reference() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[]);
        let outcome = transfer(
            &transfer_input("none", "about_the_same", vec![oak()]),
            &measurer,
        )
        .unwrap();
        let TransferOutcome::NoReference { dial, reason } = outcome else {
            panic!("expected no-reference, got {outcome:?}");
        };
        assert_eq!(dial, DIAL);
        assert!(reason.contains("none"), "{reason}");
        assert!(measurer.calls().is_empty());
    }

    fn shrub_only() -> Vec<Reference> {
        vec![Reference {
            template: "hazel".into(),
            shipped_value: 0.50,
            growth_form: "shrub".into(),
        }]
    }

    #[test]
    fn transfer_files_no_reference_when_the_manifest_forbids_the_other_growth_form() {
        let mut forbidden = transfer_input("hazel", "about_the_same", shrub_only());
        forbidden.forbid_other_growth_form = true;
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.50, Some(0.55))]);
        let outcome = transfer(&forbidden, &measurer).unwrap();
        let TransferOutcome::NoReference { reason, .. } = outcome else {
            panic!("expected no-reference, got {outcome:?}");
        };
        assert!(reason.contains("growth form"), "{reason}");
        assert!(measurer.calls().is_empty());
    }

    #[test]
    fn transfer_takes_another_growth_form_when_the_manifest_allows_it() {
        let allowed = transfer_input("hazel", "about_the_same", shrub_only());
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.50, Some(0.55))]);
        let entry = transferred(transfer(&allowed, &measurer).unwrap());
        assert_eq!(entry.reference, "hazel");
        assert!((entry.candidate - 0.50).abs() < 1e-9);
    }

    #[test]
    fn transfer_outside_the_levels_range_is_unavailable() {
        let mut ranged = transfer_input("oregon-white-oak", "about_the_same", vec![oak()]);
        ranged.target_range = Some([0.40, 0.60]);
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.40, Some(0.91))]);
        let outcome = transfer(&ranged, &measurer).unwrap();
        let TransferOutcome::Unavailable { reason } = outcome else {
            panic!("expected unavailable, got {outcome:?}");
        };
        assert_eq!(reason, "candidate measured outside the level's range");
        assert_eq!(measurer.calls().len(), 1);
    }

    #[test]
    fn family_for_nests_a_dotted_dial() {
        assert_eq!(
            family_for(DIAL, 0.4),
            json!({"skeleton": {"envelope": {"spread": 0.4}}})
        );
        assert_eq!(family_for("shellDepth", 1.5), json!({"shellDepth": 1.5}));
        assert_eq!(family_for("", 1.5), json!({}));
    }

    #[test]
    fn metric_reads_every_shape_the_species_example_writes() {
        let metrics = json!({
            "height_m": {"status": "measured", "value": 12.5},
            "dbh_m": {"status": "measured_proxy", "value": 0.31, "stems": 1, "diameters_m": [0.31]},
            "foliage_length_m": {"status": "measured", "min": 0.02, "max": 0.09, "median": 0.05},
            "crown_width_m": {"status": "unavailable", "reason": "no retained foliage"},
            "foliage_unit": "leaf",
            "bare": 1.5,
        });
        assert_eq!(metric(&metrics, "height_m"), Some(12.5));
        assert_eq!(metric(&metrics, "dbh_m"), Some(0.31));
        assert_eq!(metric(&metrics, "foliage_length_m"), Some(0.05));
        assert_eq!(metric(&metrics, "crown_width_m"), None);
        assert_eq!(metric(&metrics, "foliage_unit"), None);
        assert_eq!(metric(&metrics, "bare"), Some(1.5));
        assert_eq!(metric(&metrics, "crown_base_m"), None);
    }

    #[test]
    fn a_described_entry_round_trips_and_carries_no_probability() {
        let measurer = MockMeasurer::new(DIAL, METRIC, &[(0.30, Some(0.42)), (0.40, Some(0.52))]);
        let entry = shipped(run(&[0.30, 0.40], [0.40, 0.60], &measurer));
        let wire = serde_json::to_string(&entry).unwrap();
        assert!(!wire.contains("probabilit"), "{wire}");
        assert!(!wire.contains("confidence"), "{wire}");
        let back: DescribedEntry = serde_json::from_str(&wire).unwrap();
        assert_eq!(back, entry);
    }
}
