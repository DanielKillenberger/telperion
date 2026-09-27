//! The rails of Troll's two scaffold rows, `pitchByHeight` and `raggedReach`:
//! a value off either is refused by the row's own name wherever a family is
//! judged - the build, the wire, a walk between two tables - and a value on
//! it is admitted. The rows' geometry is tested beside the scaffold.
use serde_json::json;
use telperion_core::{blend, branching, params, presets::Preset, Error, Family};

/// Each row with values off its rail and the two ends of it.
const RAILS: [(&str, &[f64], [f64; 2]); 2] = [
    (
        "pitchByHeight",
        &[-180.5, 180.5, f64::NAN, f64::INFINITY],
        [-180.0, 180.0],
    ),
    (
        "raggedReach",
        &[-0.01, 1.01, f64::NAN, f64::NEG_INFINITY],
        [0.0, 1.0],
    ),
];

fn with(row: &str, value: f64) -> Family {
    let mut f = Preset::EuropeanBeech.parameters();
    match row {
        "pitchByHeight" => f.skeleton.habit.pitch_by_height = value,
        _ => f.skeleton.habit.ragged_reach = value,
    }
    f
}

fn refused_by(row: &'static str) -> impl Fn(Result<(), Error>) -> bool {
    move |result| matches!(result, Err(Error::InvalidValue { field, .. }) if field == row)
}

#[test]
fn off_the_rail_the_build_refuses_the_row_by_name() {
    for (row, off, ends) in RAILS {
        for value in off {
            let f = with(row, *value);
            let built = branching::generate(&f.skeleton, f.radii).map(|_| ());
            assert!(refused_by(row)(built), "{row} = {value} was built");
            assert!(
                refused_by(row)(f.validate()),
                "{row} = {value} was admitted"
            );
        }
        for value in ends {
            assert_eq!(
                with(row, value).validate(),
                Ok(()),
                "{row} = {value} is on the rail"
            );
        }
    }
}

#[test]
fn off_the_rail_the_wire_refuses_the_row_by_name() {
    let base = Preset::EuropeanBeech.parameters();
    for (row, _, [low, high]) in RAILS {
        for value in [low - 1.0, high + 1.0] {
            let wire = json!({"skeleton": {"habit": {row: value}}});
            let read = params::overlay(&base, &wire).and_then(|f| f.validate());
            assert!(refused_by(row)(read), "{row} = {value} crossed the wire");
        }
        let wire = json!({"skeleton": {"habit": {row: high}}});
        let read = params::overlay(&base, &wire).expect("the rail's end crosses the wire");
        assert_eq!(
            params::metadata(&read)["skeleton"]["habit"][row],
            json!(high)
        );
    }
}

#[test]
fn a_walk_to_a_table_off_the_rail_is_refused_by_name() {
    for (row, _, [_, high]) in RAILS {
        let on = with(row, high);
        let midway =
            blend::families(&on, &with(row, 3.0 * high + 1.0), 0.5).expect("the walk is a family");
        assert!(
            refused_by(row)(midway.validate()),
            "{row}: the walk was admitted"
        );
        let within = blend::families(&with(row, 0.0), &on, 0.5).expect("the walk is a family");
        assert_eq!(
            within.validate(),
            Ok(()),
            "{row}: a walk on the rail was refused"
        );
    }
}
