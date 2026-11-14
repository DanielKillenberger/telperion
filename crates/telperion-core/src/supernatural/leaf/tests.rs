use crate::presets::Preset;
use crate::{Error, Family};

/// The oak at 2, the group on at leaf size `size`.
fn oak(size: f64) -> Family {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 2.0;
    f.supernatural.enabled = true;
    f.supernatural.canopy.size = Some(size);
    f
}

/// The field an error names and its value.
fn named(e: Error) -> (&'static str, String) {
    match e {
        Error::InvalidValue { field, value } => (field, value),
        other => panic!("{other:?}"),
    }
}

/// The largest size a refusal at `supernatural.canopy.size` states.
fn most(f: &Family) -> f64 {
    let (field, value) = named(f.validate().unwrap_err());
    assert_eq!(field, "supernatural.canopy.size", "{value}");
    let (_, most) = value.rsplit_once("at most ").expect(&value);
    most.parse().expect(&value)
}

/// The review's repro: the oak's leaf at size 100 bears 34 m², past the
/// species' 5 m² edge; it is refused at the user's setting with the
/// largest size the oak admits, the same on the build's path, and that
/// size is the one the engine's edge admits to the last step.
#[test]
fn a_size_whose_leaf_passes_the_edge_is_refused_at_the_size() {
    let most = most(&oak(100.0));
    assert!((38.0..38.3).contains(&most), "{most}");
    let refused = oak(100.0).validate().unwrap_err();
    assert_eq!(crate::mesh::build(&oak(100.0)).unwrap_err(), refused);
    let under = oak(most * (1.0 - 1e-9));
    assert_eq!(under.validate(), Ok(()));
    assert!(crate::mesh::build(&under).is_ok());
    let over = oak(most * (1.0 + 1e-9));
    assert_eq!(
        named(over.validate().unwrap_err()).0,
        "supernatural.canopy.size"
    );
    let built = crate::mesh::build(&over).unwrap_err();
    assert_eq!(named(built).0, "supernatural.canopy.size");
}

/// A natural leaf past the edge keeps the engine's own refusal, the group
/// on or off; the group off, a size changes nothing.
#[test]
fn a_natural_leaf_past_the_edge_keeps_the_engine_s_refusal() {
    for enabled in [true, false] {
        let mut f = oak(100.0);
        f.supernatural.enabled = enabled;
        f.space.species.as_mut().unwrap().states[0].leaf_area = 6.0;
        for e in [
            f.validate().unwrap_err(),
            crate::mesh::build(&f).unwrap_err(),
        ] {
            let (field, value) = named(e);
            assert_eq!(field, "engine");
            assert!(value.starts_with("states[0].leaf_area"), "{value}");
        }
    }
    let mut off = oak(100.0);
    off.supernatural.enabled = false;
    assert_eq!(off.validate(), Ok(()));
}
