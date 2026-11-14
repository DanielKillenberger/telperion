//! fn-364 R8 (host, 14 Nov 2026): a tree the group lengthens has its
//! potential height lengthened with it, by the stem's supernatural
//! internode over its natural one, on every age.
use crate::presets::Preset;
use crate::Family;

/// Every age's inverse height of `f` as every stage reads it.
fn inverse_heights(f: &Family) -> Vec<f64> {
    let applied = f.applied();
    let species = applied.space.species.as_ref().expect("a species");
    species.states.iter().map(|s| s.inverse_height).collect()
}

/// The beech with the group on, its stem's internode `k` times its own.
fn lengthened(k: f64) -> (Family, Vec<f64>) {
    let mut f = Preset::EuropeanBeech.parameters();
    let natural = inverse_heights(&f);
    let stem = f.space.trunk[0];
    let own = f.space.species.as_ref().unwrap().states[stem].internode;
    f.supernatural.enabled = true;
    f.supernatural.internode.set(stem, k * own);
    (f, natural)
}

/// A stem laid 2.5 times its natural internode stands against 2.5 times
/// its potential height on every age; with the group off, or with no stem
/// internode written, against its own.
#[test]
fn a_lengthened_tree_has_its_potential_height_lengthened() {
    let (mut f, natural) = lengthened(2.5);
    assert!(natural.iter().any(|&h| h > 0.0));
    let applied = inverse_heights(&f);
    for (a, n) in applied.iter().zip(&natural) {
        assert!((a - n / 2.5).abs() <= 1e-12 * n.max(1e-300), "{a} against {n}");
    }
    f.supernatural.enabled = false;
    assert_eq!(inverse_heights(&f), natural);
    let mut other = Preset::EuropeanBeech.parameters();
    other.supernatural.enabled = true;
    let fine = *other.space.trunk.last().unwrap() + 1;
    other.supernatural.internode.set(fine, 0.5);
    assert_eq!(inverse_heights(&other), natural);
}

/// The Two Trees stand against the beech's 40 m lengthened 2.6 times, by
/// their stems' 0.182 m over the beech's 0.07 m.
#[test]
fn the_two_trees_stand_against_their_lengthened_height() {
    let beech = inverse_heights(&Preset::EuropeanBeech.parameters());
    for preset in [Preset::Laurelin, Preset::Telperion] {
        for (a, b) in inverse_heights(&preset.parameters()).iter().zip(&beech) {
            assert!((a * 0.182 / 0.07 - b).abs() <= 1e-12, "{preset:?}: {a} against {b}");
        }
    }
}
