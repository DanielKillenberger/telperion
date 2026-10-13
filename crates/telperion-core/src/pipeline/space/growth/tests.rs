//! fn-270 R2, R2a', R2b through the pipeline: a tree grown on from a pause
//! is the pipeline's tree grown at once, and a request the state cannot
//! meet is refused by name.
use super::*;
use crate::presets::Preset;

const H: u32 = 12;

fn beech() -> (Family, Params) {
    let family = Preset::EuropeanBeech.parameters();
    let params = Params::of(&family).unwrap();
    (family, params)
}

/// R2, R2a': paused at 5 and the year before the horizon, drawn twice,
/// advanced within its year, and grown on; and grown by quarters through
/// a year: every draw at the horizon is the tree the pipeline grows to it
/// in one call.
#[test]
fn a_tree_grown_on_is_the_pipelines_tree_grown_at_once() {
    let (family, params) = beech();
    let input = super::super::GrowInput { age: f64::from(H), seed: 7, ..super::super::GrowInput::of(&family) };
    let whole = super::super::grow(input).unwrap();
    for a in [5, H - 1] {
        let mut s = begin(&params, 7, f64::from(H)).unwrap();
        grow_to(&mut s, f64::from(a)).unwrap();
        let first = settle(&s, f64::from(a)).unwrap();
        grow_to(&mut s, f64::from(a) - 0.5).unwrap();
        assert_eq!(s.committed(), a);
        assert!(settle(&s, f64::from(a)).unwrap() == first, "a draw at {a} repeats");
        grow_to(&mut s, f64::from(H)).unwrap();
        assert!(settle(&s, f64::from(H)).unwrap() == whole, "split at {a}");
    }
    let mut s = begin(&params, 7, f64::from(H)).unwrap();
    for age in [10.0, 10.25, 10.5, 11.0, 12.0] {
        grow_to(&mut s, age).unwrap();
        assert_eq!(s.committed(), age.ceil() as u32);
        settle(&s, age).unwrap();
    }
    assert!(settle(&s, f64::from(H)).unwrap() == whole);
}

/// R2b: a horizon that is no whole year, an age not finite, past the
/// horizon or a year back, and a draw outside the last year grown are
/// refused by name, the state left to grow on.
#[test]
fn a_request_the_state_cannot_meet_is_refused_by_name() {
    let (_, params) = beech();
    let named = |r: Result<()>, name: &str| match r {
        Err(Error::InvalidValue { value, .. }) => assert!(value.starts_with(name), "{value}"),
        other => panic!("{name}: {other:?}"),
    };
    match begin(&params, 7, 12.5).map(drop) {
        Err(Error::InvalidValue { field, .. }) => assert_eq!(field, "horizon"),
        other => panic!("{other:?}"),
    }
    let mut s = begin(&params, 7, f64::from(H)).unwrap();
    grow_to(&mut s, 6.0).unwrap();
    named(grow_to(&mut s, f64::NAN), "age:");
    named(grow_to(&mut s, 12.5), "age.horizon");
    named(grow_to(&mut s, 5.0), "age.backward");
    named(settle(&s, 6.5).map(drop), "at:");
    named(settle(&s, 5.0).map(drop), "at:");
    assert_eq!(s.committed(), 6);
    grow_to(&mut s, f64::from(H)).unwrap();
    assert_eq!(s.committed(), H);
}
