use super::*;
use crate::presets::Preset;

#[test]
fn change_record_reconciles_birth_extension_shedding_and_expiry() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 0.0;
    f.growth.leaf_lifetime = 6.0;
    let mut s = Specimen::build(&f).unwrap();
    let mut previous = s.buffers().unwrap();
    let mut saw = [false; 6];
    for years in [4.0, 4.0, 4.25, 0.75, 4.0, 10.0] {
        let changes = s.advance(years).unwrap();
        let fresh = s.buffers().unwrap();
        let flags = [
            !changes.born_runs.is_empty(),
            !changes.resized_runs.is_empty(),
            !changes.shed_runs.is_empty(),
            !changes.born_placements.is_empty(),
            !changes.moved_placements.is_empty(),
            !changes.shed_placements.is_empty(),
        ];
        for i in 0..6 {
            saw[i] |= flags[i];
        }
        changes.validate(&previous, &fresh).unwrap();
        changes.apply(&mut previous).unwrap();
        assert_eq!(previous, fresh, "record did not reproduce fresh buffers");
    }
    assert!(
        saw.into_iter().all(|v| v),
        "fixture missed a change category: {saw:?}"
    );
}

#[test]
fn change_record_includes_contact_motion_from_sibling_growth() {
    let (_, mut s, shoot) = super::foliage_tests::fixture(1.0);
    let sibling = super::foliage_tests::sibling(&mut s);
    let mut previous = s.buffers().unwrap();
    let own = s.node(shoot).unwrap().clone();
    let i = s.identities[sibling.key];
    s.tree.nodes[i].radius *= 1.5;
    s.tree.nodes[i].start_radius *= 1.5;
    s.read.take();
    assert_eq!(s.node(shoot).unwrap(), &own);
    let fresh = s.buffers().unwrap();
    assert_ne!(
        previous.placements, fresh.placements,
        "contact fixture did not move"
    );
    let record = ChangeRecord::between(&previous, &fresh);
    assert!(
        record
            .moved_placements
            .iter()
            .any(|p| p.identity.shoot == shoot),
        "unchanged shoot missed sibling contact motion"
    );
    record.validate(&previous, &fresh).unwrap();
    record.apply(&mut previous).unwrap();
    assert_eq!(previous, fresh);
}

#[test]
fn change_record_rejects_missing_and_wrong_runs_naming_identity_without_mutation() {
    let (_, mut s, shoot) = super::foliage_tests::fixture(0.0);
    let previous = s.buffers().unwrap();
    s.tree.nodes[2].radius *= 1.5;
    let fresh = s.buffers().unwrap();
    let record = ChangeRecord::between(&previous, &fresh);
    assert!(!record.resized_runs.is_empty());
    for wrong in [
        ChangeRecord::default(),
        {
            let mut r = record.clone();
            r.resized_runs[0].nodes[0].radii[0] = 99.0;
            r
        },
        {
            let mut r = record.clone();
            r.resized_runs.push(r.resized_runs[0].clone());
            r
        },
    ] {
        let buffer = previous.clone();
        let message = wrong.validate(&buffer, &fresh).unwrap_err().to_string();
        assert!(
            message.contains("run") && message.contains(&shoot.birth_order().to_string()),
            "{message}"
        );
        assert_eq!(buffer, previous);
    }
}

// The chronicle redesign replaces consumer rounding with the family's canonical
// radius frames. Preserve the noise/accumulation contract at its sole writer.
#[test]
fn change_record_tolerance_suppresses_noise_without_drift() {
    let (_, mut s, _) = super::foliage_tests::fixture(0.0);
    super::interval::tests::stamp(&mut s, 1);
    let tolerance = s.timeline.as_ref().unwrap().traits.resize_tolerance;
    let previous = super::interval::tests::buffers(&s, 1.0);
    let id = s.tree.nodes[1].identity;
    let mut radii = s.keyframes.at(id, 1).unwrap();
    radii[1] += tolerance * 0.1;
    s.keyframes.record(id, 2, radii, tolerance);
    s.timeline.as_mut().unwrap().age.slice = 2;
    let after = super::interval::tests::buffers(&s, 2.0);
    let record = s.changes_between(1.0, 2.0).unwrap();
    assert!(record.resized_runs.is_empty());
    assert_eq!(previous.runs, after.runs);
    for _ in 0..10 {
        radii[1] += tolerance * 0.1;
    }
    s.keyframes.record(id, 3, radii, tolerance);
    s.timeline.as_mut().unwrap().age.slice = 3;
    let after = super::interval::tests::buffers(&s, 3.0);
    let record = s.changes_between(1.0, 3.0).unwrap();
    assert!(
        !record.resized_runs.is_empty(),
        "small increments accumulated without a measurable resize"
    );
    record.validate(&previous, &after).unwrap();
}

#[test]
fn change_record_cohort_boundary_matches_cached_and_cold_reads() {
    for cached in [false, true] {
        let (mut f, mut s, _) = super::foliage_tests::fixture(1.0);
        f.growth.leaf_lifetime = 1.25;
        // Saturated wood isolates the clock-only cohort path at an annual boundary.
        s.tree.nodes[2].shoot.birth_year = crate::growth::MAX_AGE - 1.0;
        // Mirror this synthetic birth relocation in the chronicle index.
        s.births.record(
            s.tree.nodes[2].shoot.birth_year as u64,
            s.tree.nodes[2].identity.key,
        );
        s.timeline.as_mut().unwrap().age = crate::growth::Age {
            slice: 999_999,
            remainder: 0,
        };
        s.timeline.as_mut().unwrap().foliage =
            crate::pipeline::foliage::timeline::Foliage::new(&f).unwrap();
        let mut previous = s.buffers().unwrap();
        assert!(!previous.placements.is_empty());
        if !cached {
            s.timeline.as_mut().unwrap().foliage =
                crate::pipeline::foliage::timeline::Foliage::new(&f).unwrap();
        }
        let before = super::tests::bytes(s.tree());
        let pause = s.advance(0.0).unwrap();
        assert_eq!(pause, ChangeRecord::default());
        let early = s.advance(1.0 - 1.0 / 12_000_000_000.0).unwrap();
        assert_eq!(early, ChangeRecord::default());
        let filled = s.advance(1.0 / 12_000_000_000.0).unwrap();
        assert!(!filled.born_placements.is_empty());
        assert!(filled.shed_placements.is_empty());
        assert!(filled.born_runs.is_empty() && filled.resized_runs.is_empty());
        let fresh = s.buffers().unwrap();
        assert_eq!(
            fresh.placements.len(),
            previous.placements.len() + filled.born_placements.len()
        );
        filled.validate(&previous, &fresh).unwrap();
        filled.apply(&mut previous).unwrap();
        assert_eq!(previous, fresh);
        assert_eq!(super::tests::bytes(s.tree()), before);
    }
}

#[test]
fn change_record_carries_every_word_a_leaf_moved_by() {
    let (_, s, _) = super::foliage_tests::fixture(0.0);
    let before = s.buffers().unwrap();
    let mut after = before.clone();
    // The smallest move a stored leaf can make: one code on one axis of the
    // position word. A record that compared anything looser than the words
    // themselves would not see it.
    let leaf = after.placements.values_mut().next().unwrap();
    leaf.leaf[1] ^= 1;
    let record = ChangeRecord::between(&before, &after);
    assert_eq!(
        record.moved_placements.len(),
        1,
        "a one-code move was not carried"
    );
    let mut applied = before;
    record.apply(&mut applied).unwrap();
    assert_eq!(
        applied.placements.values().next().unwrap().leaf,
        after.placements.values().next().unwrap().leaf
    );
}

#[test]
fn change_record_bad_placement_is_atomic_and_names_its_run() {
    let (_, s, shoot) = super::foliage_tests::fixture(0.0);
    let before = s.buffers().unwrap();
    let mut fresh = before.clone();
    fresh.placements.clear();
    let mut record = ChangeRecord::between(&before, &fresh);
    let retired = record.shed_placements[0];
    record.shed_placements.push(retired);
    let mut applied = before.clone();
    let error = record.apply(&mut applied).unwrap_err().to_string();
    assert!(
        error.contains(&format!("run {}", shoot.birth_order())),
        "{error}"
    );
    assert_eq!(applied, before);
    let empty = ChangeRecord::default();
    let error = empty.validate(&before, &fresh).unwrap_err().to_string();
    assert!(
        error.contains(&format!("run {}", shoot.birth_order())),
        "{error}"
    );
}

#[test]
fn change_record_fresh_reads_reuse_contacts_until_neighbor_geometry_changes() {
    let (f, mut s, _) = super::foliage_tests::fixture(1.0);
    let sibling = super::foliage_tests::sibling(&mut s);
    let before = s.buffers().unwrap();
    assert_eq!(s.timeline.as_ref().unwrap().foliage.surfaces(), 1);
    assert_eq!(s.buffers().unwrap(), before);
    assert_eq!(
        s.timeline.as_ref().unwrap().foliage.surfaces(),
        1,
        "unchanged output read rebuilt the contact surface"
    );
    let i = s.identities[sibling.key];
    s.tree.nodes[i].radius *= 1.5;
    s.tree.nodes[i].start_radius *= 1.5;
    s.read.take();
    let after = s.buffers().unwrap();
    assert_eq!(s.timeline.as_ref().unwrap().foliage.surfaces(), 2);
    let cold = crate::pipeline::foliage::timeline::Foliage::new(&f)
        .unwrap()
        .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
        .unwrap();
    assert_eq!(after.placements.values().cloned().collect::<Vec<_>>(), cold);
    assert_ne!(before.placements, after.placements);
}

#[test]
fn change_record_preserves_run_position_bits() {
    let (_, s, _) = super::foliage_tests::fixture(0.0);
    let before = s.buffers().unwrap();
    let mut after = before.clone();
    after.runs.values_mut().next().unwrap().nodes[0].position.x = -0.0;
    let record = ChangeRecord::between(&before, &after);
    assert_eq!(
        record.resized_runs.len(),
        1,
        "numeric equality lost changed wood bits"
    );
    let mut applied = before;
    record.apply(&mut applied).unwrap();
    assert_eq!(
        applied.runs.values().next().unwrap().nodes[0]
            .position
            .x
            .to_bits(),
        (-0.0_f64).to_bits()
    );
}

#[test]
fn change_record_replay_buffers_match_fresh_build_for_every_preset_and_blend() {
    let mut families: Vec<_> = [
        Preset::Ordinary,
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::SilverBirch,
        Preset::Telperion,
        Preset::Laurelin,
    ]
    .into_iter()
    .map(Preset::parameters)
    .collect();
    families.push(crate::blend::families(&families[1], &families[2], 0.37).unwrap());
    for mut f in families {
        f.age = 12.25;
        f.growth.leaf_lifetime = 6.0;
        let fresh = Specimen::build(&f).unwrap();
        f.age = 0.0;
        let mut replay = Specimen::build(&f).unwrap();
        let mut buffers = replay.buffers().unwrap();
        for part in [4.0, 0.4, 0.6, 3.0, 4.25] {
            let record = replay.advance(part).unwrap();
            let read = replay.buffers().unwrap();
            record.validate(&buffers, &read).unwrap();
            record.apply(&mut buffers).unwrap();
            assert!(buffers == read, "advance record disagreed with read");
        }
        assert!(
            buffers == fresh.buffers().unwrap(),
            "partition changed consumer buffers"
        );
        assert_eq!(replay.age(), fresh.age());
    }
}
