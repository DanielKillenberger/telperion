//! R12: the ratified cohort rule replaces birth-plus-lifetime expiry.
use super::*;
use crate::{foliage::timeline::Foliage, growth::Age, presets::Preset};

#[test]
fn mature_cohorts_hold_for_twice_the_lifetime() {
    for preset in [Preset::OregonWhiteOak, Preset::NorwaySpruce] {
        let mut f = preset.parameters();
        f.age = f.growth.mature_slice() as f64;
        let mut s = Specimen::build(&f).unwrap();
        let before = s.placements().unwrap();
        assert!(!before.is_empty());
        let wood = super::tests::bytes(s.tree());
        s.advance(2.0 * f.growth.leaf_lifetime).unwrap();
        let after = s.placements().unwrap();
        eprintln!(
            "R12 {preset:?} mature_age={} counts={}/{}",
            f.age,
            before.len(),
            after.len()
        );
        assert!(
            (after.len() as f64 / before.len() as f64 - 1.0).abs() <= 0.1,
            "{preset:?} mature foliage thinned: {} -> {}",
            before.len(),
            after.len()
        );
        assert_eq!(wood, super::tests::bytes(s.tree()));
    }
}

#[test]
fn young_cohorts_fill_annually_and_keep_station_identities() {
    let (mut f, s, shoot) = super::foliage_tests::fixture(0.0);
    f.growth.leaf_lifetime = 6.0;
    let foliage = Foliage::new(&f).unwrap();
    let read = |age| {
        foliage
            .read(s.tree(), s.envelope(), Age::from_years(age).unwrap())
            .unwrap()
    };
    let full = read(6.0);
    assert!(
        full.len() >= 6,
        "fixture needs at least one station per cohort"
    );
    let mut previous = Vec::new();
    for age in 1..=6 {
        let now = read(age as f64);
        assert!(
            now.len() > previous.len(),
            "shoot did not fill at year {age}"
        );
        assert!(now.len() <= full.len());
        for placement in &previous {
            assert!(
                now.contains(placement),
                "cohort changed a surviving station"
            );
        }
        assert!(now.iter().all(|p| p.identity.shoot == shoot));
        previous = now;
    }
    assert_eq!(previous, full);
    assert_eq!(
        read(100.0),
        full,
        "mature station identities or matrices changed"
    );
}

#[test]
fn thickened_shoot_stops_bearing_at_anatomical_diameter() {
    let (f, mut s, shoot) = super::foliage_tests::fixture(0.0);
    assert!(!s.placements().unwrap().is_empty());
    let limit = f.skeleton.twigs.resolved().unwrap().twig.bearing_diameter / 2.0;
    s.tree.nodes[2].radius = limit;
    s.tree.nodes[2].start_radius = limit;
    s.read.take();
    assert!(
        !s.placements().unwrap().is_empty(),
        "bearing boundary is inclusive"
    );
    s.tree.nodes[2].start_radius = limit + 1e-9;
    s.read.take();
    assert!(
        s.placements().unwrap().is_empty(),
        "thick twig retained leaves"
    );
    assert!(s.node(shoot).is_ok());
}

#[test]
fn cohort_budget_counts_visible_stations_and_failed_fill_keeps_age() {
    let (mut f, mut s, _) = super::foliage_tests::fixture(0.0);
    f.growth.leaf_lifetime = 6.0;
    f.canopy.max_instances = 3;
    s.tree.nodes[2].shoot.birth_year = 999_998.0;
    // Mirror this synthetic birth relocation in the chronicle index.
    s.births.record(
        s.tree.nodes[2].shoot.birth_year as u64,
        s.tree.nodes[2].identity.key,
    );
    let t = s.timeline.as_mut().unwrap();
    t.foliage = Foliage::new(&f).unwrap();
    t.age = Age::from_years(999_998.0).unwrap();
    let before = s
        .placements()
        .expect("hidden cohorts consumed the visible instance budget");
    assert_eq!(before.len(), 3);
    let error = s.advance(1.0).unwrap_err().to_string();
    assert!(error.contains("foliage instance budget"), "{error}");
    assert_eq!(s.age(), 999_998.0);
    assert_eq!(s.placements().unwrap(), before);
}
