use super::*;
use crate::presets::Preset;

fn fresh_read(s: &Specimen) -> SpecimenRead {
    let mut shed: Vec<_> = s
        .tree
        .nodes
        .iter()
        .filter(|n| n.shoot.death_year.is_some())
        .map(|n| n.identity)
        .collect();
    shed.sort_unstable();
    SpecimenRead {
        tree: s.tree().clone(),
        envelope: s.envelope(),
        placements: s.placements().unwrap(),
        shed,
    }
}

fn equal(a: &SpecimenRead, b: &SpecimenRead) {
    assert_eq!(a.envelope, b.envelope, "historical envelope differs");
    assert_eq!(a.shed, b.shed, "historical shed identities differ");
    assert!(
        super::super::tests::bytes(&a.tree) == super::super::tests::bytes(&b.tree),
        "historical packed topology, radii or shoot state differs"
    );
    assert_eq!(
        a.placements, b.placements,
        "historical placement bytes differ"
    );
}

#[test]
fn every_prior_read_matches_fresh_growth_including_shedding_and_contacts() {
    for contact in [0.0, 1.0] {
        let mut f = Preset::Ordinary.parameters();
        f.age = 12.25;
        f.growth.leaf_lifetime = 6.0;
        f.canopy.surface_contact = contact;
        let s = Specimen::build(&f).unwrap();
        assert!(s.shed > 0, "fixture must include shedding");
        let storage = super::super::tests::bytes(&s.tree);
        let next = s.next_identity;
        let cost = s.cost.stages;
        for half in (0..=24).rev() {
            let age = half as f64 / 2.0;
            f.age = age;
            let fresh = fresh_read(&Specimen::build(&f).unwrap());
            let historical = s.read_at_age(age).unwrap();
            equal(&historical, &fresh);
            historical.tree.validate_solved().unwrap();
            assert!(historical
                .tree
                .nodes
                .iter()
                .all(|n| n.shoot.death_year.is_none()));
        }
        equal(&fresh_read(&s), &s.read().unwrap());
        assert_eq!(s.age(), 12.25);
        assert_eq!(s.next_identity, next);
        assert_eq!(s.cost.stages, cost, "read ran simulation");
        assert_eq!(
            super::super::tests::bytes(&s.tree),
            storage,
            "read mutated chronicle"
        );
    }
}

#[test]
fn historical_read_refuses_invalid_ages_and_names_the_frontier() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 3.25;
    let s = Specimen::build(&f).unwrap();
    let before = s.read().unwrap();
    for value in [-1.0, f64::NAN, crate::growth::MAX_AGE + 1.0] {
        let error = s.read_at_age(value).unwrap_err().to_string();
        assert!(error.contains("age"), "{error}");
    }
    let error = s.read_at_age(3.5).unwrap_err().to_string();
    assert!(
        error.contains("frontier") && error.contains("3.25"),
        "{error}"
    );
    equal(&before, &s.read().unwrap());
}

#[test]
fn ten_year_jump_and_yearly_advances_keep_all_intermediate_reads() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 10.0;
    let jump = Specimen::build(&f).unwrap();
    f.age = 0.0;
    let mut yearly = Specimen::build(&f).unwrap();
    for year in 0..=10 {
        if year > 0 {
            yearly.advance(1.0).unwrap();
        }
        for age in 0..=year {
            equal(
                &jump.read_at_age(age as f64).unwrap(),
                &yearly.read_at_age(age as f64).unwrap(),
            );
        }
    }
}

#[test]
fn historical_reads_cover_every_preset_blends_and_do_not_change_future_growth() {
    let mut families: Vec<_> = [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Telperion,
        Preset::Laurelin,
    ]
    .into_iter()
    .map(Preset::parameters)
    .collect();
    families.push(crate::blend::families(&families[1], &families[2], 0.37).unwrap());
    for mut f in families {
        f.age = 10.25;
        let mut s = Specimen::build(&f).unwrap();
        let mut unread = s.clone();
        for age in [0.0, 1.25, 3.0, 7.5, 10.25, 2.0, 9.0] {
            f.age = age;
            equal(
                &s.read_at_age(age).unwrap(),
                &fresh_read(&Specimen::build(&f).unwrap()),
            );
        }
        s.advance(1.75).unwrap();
        unread.advance(1.75).unwrap();
        equal(&s.read().unwrap(), &fresh_read(&unread));
        f.age = 12.0;
        equal(
            &s.read().unwrap(),
            &fresh_read(&Specimen::build(&f).unwrap()),
        );
    }
}

#[test]
fn historical_read_keeps_zero_budget_envelopes_and_saturated_cohorts() {
    let mut f = Preset::Ordinary.parameters();
    f.growth.rate = 10.0;
    f.growth.leaf_lifetime = 6.0;
    f.age = 1000.0;
    let s = Specimen::build(&f).unwrap();
    assert!(s.timeline.as_ref().unwrap().years.len() < 10);
    for age in [0.0, 0.5, 1.0, 1.5, 2.0, 3.5, 7.0, 1000.0] {
        f.age = age;
        equal(
            &s.read_at_age(age).unwrap(),
            &fresh_read(&Specimen::build(&f).unwrap()),
        );
    }
    // A plateau before growth has begun must retain the seedling envelope.
    f.growth.rate = 0.001;
    f.growth.shape = 8.0;
    f.age = 100.0;
    let s = Specimen::build(&f).unwrap();
    assert!(s.timeline.as_ref().unwrap().years.is_empty());
    f.age = 50.0;
    equal(
        &s.read_at_age(50.0).unwrap(),
        &fresh_read(&Specimen::build(&f).unwrap()),
    );
}

#[test]
fn historical_read_cannot_poison_clock_only_cohort_changes() {
    let mut f = Preset::Ordinary.parameters();
    f.growth.rate = 1.0;
    f.growth.leaf_lifetime = 20.0;
    f.canopy.surface_contact = 1.0;
    f.age = f.growth.mature_slice() as f64;
    let s = Specimen::build(&f).unwrap();
    let before = s.buffers().unwrap();
    let mature = f.age as usize;
    f.age += 100.0;
    let fresh = Specimen::build(&f).unwrap().buffers().unwrap();
    for age in 0..mature {
        let mut trial = s.clone();
        trial.read_at_age(age as f64).unwrap();
        let changes = trial.advance(100.0).unwrap();
        assert!(
            !changes.born_placements.is_empty(),
            "fixture must fill cohorts"
        );
        changes.validate(&before, &fresh).unwrap();
    }
}

#[test]
fn default_read_keeps_exact_frontier_ticks_near_maximum_age() {
    let mut f = Preset::Ordinary.parameters();
    f.growth.rate = 10.0;
    f.age = crate::growth::MAX_AGE - 1.0;
    let mut s = Specimen::build(&f).unwrap();
    s.advance(1.0 / 12_000_000_000.0).unwrap();
    assert_eq!(s.timeline.as_ref().unwrap().age.remainder, 1);
    equal(
        &s.read()
            .expect("default read rounded its frontier into the future"),
        &fresh_read(&s),
    );
}
