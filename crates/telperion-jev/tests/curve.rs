//! Curve composition and fit, checked against fn-30's recorded numbers.

use telperion_jev::pipeline::curve::{
    fit, fraction, gould_decade_growth_cm, interpolate, mature_age, reference_at, to_metres,
    BelowFirstRow, Composition, CurveError, FitInput, GouldCoefficients, Row, Table, Unit,
};

const OAK_HEIGHT_M: &[(f64, f64)] = &[
    (30.0, 9.0),
    (40.0, 12.0),
    (50.0, 14.6),
    (60.0, 16.9),
    (70.0, 18.8),
    (80.0, 20.3),
    (90.0, 21.6),
    (100.0, 22.8),
    (110.0, 23.8),
    (120.0, 24.8),
    (130.0, 25.8),
    (140.0, 26.8),
    (150.0, 27.7),
    (160.0, 28.3),
    (170.0, 28.9),
    (180.0, 29.4),
    (190.0, 29.8),
    (200.0, 30.2),
];

const SPRUCE_HEIGHT_M: &[(f64, f64)] = &[
    (20.0, 7.1),
    (30.0, 11.5),
    (40.0, 16.6),
    (50.0, 21.2),
    (60.0, 24.7),
    (70.0, 27.4),
    (80.0, 29.7),
    (90.0, 31.6),
    (100.0, 33.3),
    (110.0, 34.8),
    (120.0, 35.9),
];

const SPRUCE_DG_CM: &[(f64, f64)] = &[
    (20.0, 7.5),
    (30.0, 11.5),
    (40.0, 15.5),
    (50.0, 19.3),
    (60.0, 23.0),
    (70.0, 26.9),
    (80.0, 30.7),
    (90.0, 34.2),
    (100.0, 37.6),
    (110.0, 40.9),
    (120.0, 44.3),
];

fn table(source: &str, unit: Unit, rows: &[(f64, f64)]) -> Table {
    Table {
        source: source.to_string(),
        unit,
        rows: rows
            .iter()
            .map(|&(age_years, value)| Row { age_years, value })
            .collect(),
    }
}

fn gould() -> GouldCoefficients {
    GouldCoefficients {
        intercept: -1.33299,
        ln_dbh: 1.66609,
        dbh2: -0.00154,
        bal: -0.00326,
        ba: -0.00204,
        ln_si: 0.14995,
    }
}

fn oak_input(reference_ages_years: [f64; 3], tolerance_percent: f64) -> FitInput {
    FitInput {
        envelope_height_m: 24.0,
        mature_dbh_m: 0.836,
        reference_ages_years,
        height: Composition::Table {
            table: table("E1 Juettner oak II", Unit::M, OAK_HEIGHT_M),
            below_first_row: BelowFirstRow::LinearFromZero,
        },
        dbh: Composition::GouldIntegration {
            anchor_age_years: 30.0,
            anchor_dbh_cm: 7.2,
            coefficients: gould(),
            site_index_m: 35.0,
            max_age_years: 600.0,
            below_anchor: BelowFirstRow::LinearFromZero,
        },
        tolerance_percent,
    }
}

fn spruce_input() -> FitInput {
    FitInput {
        envelope_height_m: 15.0,
        mature_dbh_m: 0.422,
        reference_ages_years: [14.1, 26.6, 36.9],
        height: Composition::Table {
            table: table("E1 Wiedemann spruce I", Unit::M, SPRUCE_HEIGHT_M),
            below_first_row: BelowFirstRow::LinearFromZero,
        },
        dbh: Composition::ScaledTable {
            table: table("E1 Wiedemann DG I", Unit::Cm, SPRUCE_DG_CM),
            factor: 2.0,
            below_first_row: BelowFirstRow::LinearFromZero,
        },
        tolerance_percent: 15.0,
    }
}

fn close(actual: f64, expected: f64, epsilon: f64) {
    assert!(
        (actual - expected).abs() <= epsilon,
        "{actual} is not within {epsilon} of {expected}"
    );
}

#[test]
fn oak_fit_matches_fn30() {
    let report = fit(&oak_input([26.7, 56.1, 112.0], 15.0)).expect("the oak fits");
    close(report.rate, 0.032, 1e-9);
    close(report.shape, 2.0, 1e-9);
    assert_eq!(report.derived_mature_age_years, 432);
    for (row, (height, dbh)) in
        report
            .rows
            .iter()
            .zip([(8.00, 0.064), (16.00, 0.112), (24.00, 0.236)])
    {
        close(row.reference_height_m.expect("a height"), height, 0.01);
        close(row.reference_dbh_m.expect("a diameter"), dbh, 0.001);
        let f = fraction(report.rate, report.shape, row.age_years);
        assert_eq!(row.model_height_m, 24.0 * f);
        assert_eq!(row.model_dbh_m, 0.836 * f);
    }
    assert!(report.unavailable.is_empty());
}

#[test]
fn spruce_fit_matches_fn30() {
    let report = fit(&spruce_input()).expect("the spruce fits");
    close(report.rate, 0.091, 1e-9);
    close(report.shape, 3.4, 1e-9);
    assert_eq!(report.derived_mature_age_years, 158);
    for (row, (height, dbh)) in
        report
            .rows
            .iter()
            .zip([(5.00, 0.106), (10.00, 0.203), (15.00, 0.285)])
    {
        close(row.reference_height_m.expect("a height"), height, 0.02);
        close(row.reference_dbh_m.expect("a diameter"), dbh, 0.001);
        let f = fraction(report.rate, report.shape, row.age_years);
        assert_eq!(row.model_height_m, 15.0 * f);
    }
    assert!(report.unavailable.is_empty());
}

#[test]
fn gould_worked_case_grows_2_82_cm_per_decade_at_40_cm() {
    close(gould_decade_growth_cm(40.0, &gould(), 35.0), 2.82, 0.005);
}

#[test]
fn mature_age_is_the_first_saturated_integer_year() {
    assert_eq!(mature_age(0.032, 2.0), 432);
    assert_eq!(mature_age(0.091, 3.4), 158);
    assert!(fraction(0.032, 2.0, 431.0) < 1.0);
    assert_eq!(fraction(0.032, 2.0, 432.0), 1.0);
}

#[test]
fn unit_conversion_runs_in_code() {
    for (value, unit, metres) in [
        (1.0, Unit::M, 1.0),
        (100.0, Unit::Cm, 1.0),
        (1.0, Unit::Ft, 0.3048),
        (1.0, Unit::In, 0.0254),
    ] {
        close(to_metres(value, unit), metres, 1e-12);
    }
}

#[test]
fn interpolation_is_linear_between_rows() {
    let spruce = table("E1", Unit::M, SPRUCE_HEIGHT_M);
    close(
        interpolate(&spruce, 25.0, BelowFirstRow::LinearFromZero).expect("a value"),
        9.3,
        1e-12,
    );
    close(
        interpolate(&spruce, 30.0, BelowFirstRow::LinearFromZero).expect("a value"),
        11.5,
        1e-12,
    );
    close(
        interpolate(&spruce, 10.0, BelowFirstRow::LinearFromZero).expect("a value"),
        3.55,
        1e-12,
    );
}

#[test]
fn below_the_first_row_is_unavailable_when_the_manifest_says_so() {
    let spruce = table("E1", Unit::M, SPRUCE_HEIGHT_M);
    assert_eq!(
        interpolate(&spruce, 10.0, BelowFirstRow::Unavailable),
        Err(CurveError::BelowFirstRow {
            age_years: 10.0,
            first_row_age_years: 20.0,
        })
    );
}

#[test]
fn a_table_of_one_row_is_too_few_points() {
    let single = table("one row", Unit::M, &[(30.0, 9.0)]);
    assert_eq!(
        single.validate(),
        Err(CurveError::TooFewPoints { points: 1 })
    );
    assert_eq!(
        interpolate(&single, 30.0, BelowFirstRow::LinearFromZero),
        Err(CurveError::TooFewPoints { points: 1 })
    );
    let mut input = oak_input([26.7, 56.1, 112.0], 15.0);
    input.height = Composition::Table {
        table: single,
        below_first_row: BelowFirstRow::LinearFromZero,
    };
    assert_eq!(fit(&input), Err(CurveError::TooFewPoints { points: 0 }));
}

#[test]
fn a_dimension_with_too_few_points_is_skipped_not_fitted() {
    let mut input = oak_input([26.7, 56.1, 112.0], 15.0);
    input.dbh = Composition::Table {
        table: table("one row", Unit::Cm, &[(30.0, 7.2)]),
        below_first_row: BelowFirstRow::LinearFromZero,
    };
    let report = fit(&input).expect("the height still fits");
    close(report.rate, 0.032, 1e-9);
    assert_eq!(report.unavailable.len(), 3);
    assert!(report.unavailable.iter().all(|u| u.field == "dbh_m"));
    assert!(report.rows.iter().all(|r| r.reference_dbh_m.is_none()));
}

#[test]
fn an_age_beyond_the_last_row_is_unavailable_never_extrapolated() {
    let oak = table("E1 Juettner oak II", Unit::M, OAK_HEIGHT_M);
    assert_eq!(
        interpolate(&oak, 250.0, BelowFirstRow::LinearFromZero),
        Err(CurveError::BeyondLastRow {
            age_years: 250.0,
            last_row_age_years: 200.0,
        })
    );
    let report = fit(&oak_input([26.7, 56.1, 250.0], 15.0)).expect("two ages still fit");
    assert_eq!(report.unavailable.len(), 1);
    assert_eq!(report.unavailable[0].field, "height_m");
    assert_eq!(report.unavailable[0].age_years, 250.0);
    assert!(report.rows[2].reference_height_m.is_none());
    assert!(report.rows[2].height_error_percent.is_none());
    assert!(report.rows[2].reference_dbh_m.is_some());
}

#[test]
fn the_gould_composition_stops_at_its_last_age() {
    let dbh = Composition::GouldIntegration {
        anchor_age_years: 30.0,
        anchor_dbh_cm: 7.2,
        coefficients: gould(),
        site_index_m: 35.0,
        max_age_years: 600.0,
        below_anchor: BelowFirstRow::Unavailable,
    };
    assert_eq!(
        reference_at(&dbh, 601.0),
        Err(CurveError::BeyondLastRow {
            age_years: 601.0,
            last_row_age_years: 600.0,
        })
    );
    assert_eq!(
        reference_at(&dbh, 29.0),
        Err(CurveError::BelowFirstRow {
            age_years: 29.0,
            first_row_age_years: 30.0,
        })
    );
    close(reference_at(&dbh, 30.0).expect("the anchor"), 0.072, 1e-12);
}

#[test]
fn a_tie_keeps_the_first_minimum_in_grid_order() {
    let ages = [10.0, 20.0, 30.0];
    let synthetic = |asymptote: f64| {
        let rows: Vec<(f64, f64)> = ages
            .iter()
            .map(|&age| (age, asymptote * fraction(0.05, 2.0, age)))
            .collect();
        Composition::Table {
            table: table("synthetic", Unit::M, &rows),
            below_first_row: BelowFirstRow::LinearFromZero,
        }
    };
    let input = FitInput {
        envelope_height_m: 20.0,
        mature_dbh_m: 0.4,
        reference_ages_years: ages,
        height: synthetic(20.0),
        dbh: synthetic(0.4),
        tolerance_percent: 15.0,
    };
    let report = fit(&input).expect("the synthetic reference fits");
    close(report.rate, 0.05, 1e-9);
    close(report.shape, 2.0, 1e-9);
    assert_eq!(report.objective, 0.0);
    assert!(report.misses.is_empty());
}

#[test]
fn a_miss_beyond_tolerance_is_filed_and_one_inside_it_is_not() {
    let ages = [10.0, 20.0, 30.0];
    let height_rows: Vec<(f64, f64)> = ages
        .iter()
        .map(|&age| (age, 20.0 * fraction(0.05, 2.0, age)))
        .collect();
    for (overshoot, expected) in [(1.153_f64, 3), (1.149_f64, 0)] {
        let dbh_rows: Vec<(f64, f64)> = ages
            .iter()
            .map(|&age| (age, 0.4 * fraction(0.05, 2.0, age) / overshoot))
            .collect();
        let input = FitInput {
            envelope_height_m: 20.0,
            mature_dbh_m: 0.4,
            reference_ages_years: ages,
            height: Composition::Table {
                table: table("synthetic height", Unit::M, &height_rows),
                below_first_row: BelowFirstRow::LinearFromZero,
            },
            dbh: Composition::Table {
                table: table("synthetic dbh", Unit::M, &dbh_rows),
                below_first_row: BelowFirstRow::LinearFromZero,
            },
            tolerance_percent: 15.0,
        };
        let report = fit(&input).expect("the synthetic reference fits");
        assert_eq!(report.misses.len(), expected, "overshoot {overshoot}");
        assert!(report.misses.iter().all(|m| m.field == "dbh_m"));
    }
}

#[test]
fn height_and_diameter_misses_are_independent() {
    let report = fit(&oak_input([26.7, 56.1, 112.0], 5.0)).expect("the oak fits");
    let height: Vec<&str> = report
        .misses
        .iter()
        .filter(|m| m.field == "height_m")
        .map(|m| m.field.as_str())
        .collect();
    let dbh = report.misses.iter().filter(|m| m.field == "dbh_m").count();
    assert_eq!(height.len(), 1);
    assert_eq!(dbh, 3);
    let miss = report
        .misses
        .iter()
        .find(|m| m.field == "height_m")
        .expect("a height miss");
    assert_eq!(miss.age_years, 112.0);
    assert!(miss.error_percent.abs() > 5.0);
}

#[test]
fn unsorted_duplicate_or_non_finite_rows_are_rejected() {
    for rows in [
        &[(40.0, 12.0), (30.0, 9.0)][..],
        &[(30.0, 9.0), (30.0, 12.0)][..],
        &[(30.0, f64::NAN), (40.0, 12.0)][..],
        &[(f64::INFINITY, 9.0), (40.0, 12.0)][..],
    ] {
        let rejected = table("bad", Unit::M, rows).validate();
        assert!(
            matches!(rejected, Err(CurveError::Invalid { .. })),
            "{rows:?} was not rejected"
        );
    }
}
