//! `Family::validate` against the build. Every row is walked past its bound on
//! every family and refused with no tree grown, and the build refuses the same
//! value by the same name.
use crate::{
    mesh,
    params::{self, decode},
    presets::{Preset, CATALOGUE, IN_WORK},
    Error, Family,
};
use serde_json::{json, Value};

/// Rows no value on the wire can put out of bounds. Any clumping order is an
/// order, a leaf budget is refused only by the crown that overruns it (a
/// resource limit, not a rail), any seed is a seed, and the twig divergence
/// is refused only when it is not finite, which the wire cannot write.
const UNBOUNDED: &[&str] = &[
    "/canopy/clumpSystemOrder",
    "/canopy/maxInstances",
    "/skeleton/seed",
    "/skeleton/twigs/divergence",
];

/// Values that walk a row past any rail it has, below and above. A value the
/// row's type cannot hold is the wire's refusal, not the row's, and is skipped.
fn ladder() -> [Value; 9] {
    [
        json!(-1e300),
        json!(-1.0),
        json!(0),
        json!(0.5),
        json!(2.0),
        json!(1e7),
        json!(u32::MAX),
        json!(u64::MAX),
        json!(1e300),
    ]
}

fn families() -> Vec<(&'static str, Family)> {
    CATALOGUE
        .iter()
        .chain(IN_WORK)
        .map(|entry| (entry.1, Preset::from_id(entry.1).unwrap().parameters()))
        .collect()
}

/// Every numeric row on the wire, by pointer; an unset optional row counts.
fn rows(v: &Value, at: String, out: &mut Vec<String>) {
    match v {
        Value::Object(map) => {
            for (k, v) in map {
                rows(v, format!("{at}/{k}"), out);
            }
        }
        Value::Number(_) | Value::Null => out.push(at),
        _ => {}
    }
}

/// Each value on the ladder the wire can hold for this row, as a family.
fn walked(f: &Family, row: &str) -> Vec<(Value, Family)> {
    let wire = params::metadata(f);
    ladder()
        .into_iter()
        .filter_map(|value| {
            let mut w = wire.clone();
            *w.pointer_mut(row).unwrap() = value.clone();
            decode(&w).ok().map(|g| (value, g))
        })
        .collect()
}

fn every_row(f: &Family) -> Vec<String> {
    let mut out = Vec::new();
    rows(&params::metadata(f), String::new(), &mut out);
    out
}

#[test]
fn every_row_past_its_bound_is_refused_on_every_family() {
    for (id, f) in families() {
        f.validate()
            .unwrap_or_else(|e| panic!("{id} is refused: {e}"));
        let unbounded: Vec<String> = every_row(&f)
            .into_iter()
            .filter(|row| walked(&f, row).iter().all(|(_, g)| g.validate().is_ok()))
            .collect();
        assert_eq!(unbounded, UNBOUNDED, "{id}: rows validate never refuses");
    }
}

/// The ordinary family at a size that grows in moments: few attractors and a
/// small node budget change how much is grown, never what a row is judged by.
fn small() -> Family {
    let mut f = Preset::from_id("ordinary").unwrap().parameters();
    f.skeleton.attractors = 64;
    f.skeleton.growth.max_nodes = Some(256);
    f
}

/// What the build says of a family restated on the wire: the wire's own
/// checks, then the skeleton grown, the wood swept and the leaves placed.
fn built(f: &Family) -> Result<(), Error> {
    let f = params::parse(&params::metadata(f))?;
    mesh::build(&f).map(drop)
}

/// Values the build refuses from the geometry it grew, where no row's rail
/// refuses them: float32 overflow of the swept wood, an empty mesh, a scaffold
/// too long to step, and radii left unsolved at a twig tip taper of zero.
/// Whether any of them becomes a rail is open (fn-123, escalated to the host);
/// until then they are named here, so a new one fails this test.
const BUILD_ONLY: &[(&str, &str)] = &[
    ("/radii/trunkRadius", "18446744073709551615"),
    ("/radii/trunkRadius", "1e+300"),
    ("/skeleton/envelope/height", "18446744073709551615"),
    ("/skeleton/growth/maxNodes", "0"),
    ("/skeleton/growth/trunkHeight", "0"),
    ("/skeleton/growth/trunkHeight", "18446744073709551615"),
    ("/skeleton/growth/trunkHeight", "1e+300"),
    ("/skeleton/habit/twigTipTaper", "0"),
];

/// Every ladder value on every row of the small family: the build and
/// `validate` agree on it, error for error, save the values named above,
/// which only the build refuses.
#[test]
fn the_build_and_validate_refuse_the_same_values_by_the_same_name() {
    let base = small();
    built(&base).expect("the small family builds");
    let mut build_only = Vec::new();
    for row in every_row(&base) {
        for (value, f) in walked(&base, &row) {
            let (build, validate) = (built(&f).err(), f.validate().err());
            if build.is_some() && validate.is_none() {
                build_only.push((row.clone(), value.to_string()));
            } else {
                assert_eq!(build, validate, "{row} at {value}");
            }
        }
    }
    let pinned: Vec<_> = BUILD_ONLY
        .iter()
        .map(|&(row, value)| (row.to_string(), value.to_string()))
        .collect();
    assert_eq!(build_only, pinned);
}
