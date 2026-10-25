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
    let input = super::super::GrowInput {
        age: f64::from(H),
        seed: 7,
        ..super::super::GrowInput::of(&family)
    };
    let whole = super::super::grow(input).unwrap();
    for a in [5, H - 1] {
        let mut s = begin(&params, 7, f64::from(H)).unwrap();
        grow_to(&mut s, f64::from(a)).unwrap();
        let first = settle(&s, f64::from(a)).unwrap();
        grow_to(&mut s, f64::from(a) - 0.5).unwrap();
        assert_eq!(s.committed(), a);
        assert!(
            settle(&s, f64::from(a)).unwrap() == first,
            "a draw at {a} repeats"
        );
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

/// fn-270 R5c: the oak through one year, its leaves' state moving by
/// degree: in full leaf just past the whole age, falling and colouring
/// through autumn, bare through winter, every leaf growing again from
/// budburst, and in full leaf again just before the next whole age.
#[cfg(feature = "geometry")]
#[test]
fn the_oaks_year_runs_from_full_leaf_to_full_leaf_by_degree() {
    let family = Preset::OregonWhiteOak.parameters();
    let params = Params::of(&family).unwrap();
    let mut s = begin(&params, 7, 10.0).unwrap();
    grow_to(&mut s, 10.0).unwrap();
    let at = |f: f64| settle(&s, 9.0 + f).unwrap();
    let state = |t: &Tree| {
        t.unfolding
            .iter()
            .map(|u| (u.fall, u.colour, u.regrow))
            .fold((2.0f32, 0.0f32, 2.0f32), |m, u| {
                (m.0.min(u.0), m.1.max(u.1), m.2.min(u.2))
            })
    };
    let (start, end) = (at(1e-6), at(1.0 - 1e-6));
    assert!(
        start
            .unfolding
            .iter()
            .all(|u| u.fall < 1e-3 && u.colour == 0.0),
        "full leaf at the start: {:?}",
        state(&start)
    );
    // Every leaf whole, but a gone subtree's, faded with its wood.
    let whole = |u: &crate::tree::Unfolded| {
        u.grow < 1e-3 || u.grow > 0.999 && (u.life == 0 || u.regrow > 0.999)
    };
    assert!(end.unfolding.iter().all(whole), "full leaf at the end");
    let winter = at(0.6);
    // Bare in winter: every old leaf fallen, none grown again, none of
    // this year's grown yet (the root bears none).
    let bare = |u: &crate::tree::Unfolded| {
        if u.life > 0 {
            u.fall == 1.0 && u.regrow == 0.0
        } else {
            u.grow == 0.0
        }
    };
    assert!(winter.unfolding.iter().skip(1).all(bare), "bare in winter");
    let drawn = mesh(&s, 9.6, &family).unwrap().foliage.instances;
    assert!(
        drawn
            .leaves
            .iter()
            .all(|l| drawn.reference.scale(*l) == 0.0),
        "no leaf drawn in winter"
    );
    assert!(
        winter.unfolding.iter().any(|u| u.life == u16::MAX),
        "the oak's leaves pass"
    );
    // Every node's state moves by degree: half the step, about half the move.
    for f in [0.25, 0.3, 0.35, 0.9] {
        let a = state(&at(f));
        let steps: Vec<f32> = [2e-3, 1e-3, 5e-4]
            .iter()
            .map(|d| {
                let b = state(&at(f + d));
                (a.0 - b.0)
                    .abs()
                    .max((a.1 - b.1).abs())
                    .max((a.2 - b.2).abs())
            })
            .collect();
        assert!(
            steps[1] <= 0.6 * steps[0] + 1e-6 && steps[2] <= 0.6 * steps[1] + 1e-6,
            "at {f}: {steps:?}"
        );
    }
}
