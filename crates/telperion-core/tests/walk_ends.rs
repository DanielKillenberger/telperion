//! fn-308 R6: a walk's ends are its ends. `families(a, b, 0)` is `a` and
//! `families(a, b, 1)` is `b`, row for row, on every film walk, so the
//! rule fn-308 applies between the ends never reaches them.
use telperion_core::blend::families;
use telperion_core::presets::{by_identity, Family};

/// The film's walks (fn-272's `release.json`), as `units_bound` names them.
const WALKS: [(&str, &str, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0),
    ("norway-spruce", "silver-birch", 40.0),
    ("silver-birch", "oregon-white-oak", 40.0),
    ("oregon-white-oak", "date-palm", 40.0),
    ("date-palm", "laurelin", 1.0),
    ("laurelin", "telperion", 80.0),
];

fn at(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

#[test]
fn every_film_walk_returns_its_ends_unchanged() {
    for (a, b, age) in WALKS {
        for seed in [1, 7, 42] {
            let (x, y) = (at(a, age, seed), at(b, age, seed));
            for (t, end) in [(0.0, &x), (1.0, &y)] {
                let f = families(&x, &y, t).expect("an end");
                // Debug text, as `walk_midpoint::the_ends_written` compares.
                assert_eq!(
                    format!("{f:?}"),
                    format!("{end:?}"),
                    "{a} > {b} at {age}, seed {seed}, t {t}"
                );
            }
        }
    }
}
