//! `Family::validate` against the build. Every row is walked past its bound on
//! every family and refused with no tree grown, and the build refuses the same
//! value by the same name.
use crate::{
    mesh::{self, Detail},
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

/// A family at a size that grows in moments: few attractors and a small node
/// budget change how much is grown, never what a row is judged by.
fn small(mut f: Family) -> Family {
    f.skeleton.attractors = 64;
    f.skeleton.growth.max_nodes = Some(256);
    f
}

/// What the build says of a family restated on the wire: the wire's own
/// checks, then the skeleton grown, the wood swept and the leaves placed.
fn built(f: &Family) -> Result<(), Error> {
    let f = params::parse(&params::metadata(f))?;
    mesh::build(&f, Detail::Full).map(drop)
}

/// Values at absurd magnitudes the build refuses from the geometry it grew,
/// where no row's rail refuses them: float32 overflow of the swept wood,
/// triangles collapsed in float32, a scaffold too long to step, a leaf
/// transform past float32. They stay the build's to raise (fn-123, host
/// design), and are named here with the families they reach, so a new one
/// fails this test.
const BUILD_ONLY: &[(&str, &str, &[&str])] = &[
    ("/radii/lengthTaper", "10000000.0", SWEPT),
    ("/radii/lengthTaper", "4294967295", SWEPT),
    ("/radii/lengthTaper", "18446744073709551615", SWEPT),
    ("/radii/lengthTaper", "1e+300", SWEPT),
    ("/radii/trunkRadius", "18446744073709551615", ALL),
    ("/radii/trunkRadius", "1e+300", ALL),
    ("/skeleton/envelope/height", "18446744073709551615", TALL),
    ("/skeleton/envelope/spread", "1e+300", &["european-beech"]),
    ("/skeleton/growth/trunkHeight", "18446744073709551615", LONG),
    ("/skeleton/growth/trunkHeight", "1e+300", ALL),
];
const ALL: &[&str] = &[
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "silver-birch",
    "telperion",
    "laurelin",
    "european-beech",
    "date-palm",
];
const SWEPT: &[&str] = &["oregon-white-oak", "norway-spruce", "silver-birch"];
const TALL: &[&str] = &[
    "ordinary",
    "norway-spruce",
    "telperion",
    "laurelin",
    "date-palm",
];
const LONG: &[&str] = &[
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "silver-birch",
    "date-palm",
];

/// Refused by both under different names: the build finds nothing to sweep
/// ("mesh has no geometry"), where `validate` names the zero ceiling.
const RENAMED: &[(&str, &str)] = &[("/skeleton/growth/maxNodes", "0")];

/// Every ladder value on every row of every family, made small: the build and
/// `validate` agree on it, error for error, save the values named above.
#[test]
fn the_build_and_validate_refuse_the_same_values_by_the_same_name() {
    let mut build_only = Vec::new();
    for (id, f) in families() {
        let base = small(f);
        built(&base).unwrap_or_else(|e| panic!("small {id} is refused: {e}"));
        for row in every_row(&base) {
            for (value, f) in walked(&base, &row) {
                let (build, validate) = (built(&f).err(), f.validate().err());
                let renamed = RENAMED.contains(&(row.as_str(), &*value.to_string()));
                if renamed {
                    assert!(
                        build.is_some() && validate.is_some(),
                        "{id}: {row} at {value}"
                    );
                } else if build.is_some() && validate.is_none() {
                    build_only.push((id, row.clone(), value.to_string()));
                } else {
                    assert_eq!(build, validate, "{id}: {row} at {value}");
                }
            }
        }
    }
    let mut pinned = Vec::new();
    for &(row, value, ids) in BUILD_ONLY {
        for &id in ids {
            pinned.push((id, row.to_string(), value.to_string()));
        }
    }
    build_only.sort();
    pinned.sort();
    assert_eq!(build_only, pinned);
}
