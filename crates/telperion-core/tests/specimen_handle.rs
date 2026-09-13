#![cfg(feature = "json")]
use telperion_core::{branching::Specimen, presets::Preset, specimen::SpecimenStore};

fn family() -> telperion_core::presets::Family {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 5.25;
    f.skeleton.envelope.height = 4.0;
    f.skeleton.attractors = 40;
    f
}

#[test]
fn specimen_handle_recovery_and_ownership() {
    let f = family();
    let mut store = SpecimenStore::default();
    let handle = store.build(&f, 100.0).unwrap();
    let before = store.read(handle, None).unwrap();
    let mut invalid = f.clone();
    invalid.age = -1.0;
    assert!(store
        .build(&invalid, 100.0)
        .unwrap_err()
        .to_string()
        .contains("age"));
    assert_eq!(before, store.read(handle, None).unwrap());
    for (age, field) in [(6.0, "frontier"), (-1.0, "age")] {
        assert!(store
            .read(handle, Some(age))
            .unwrap_err()
            .to_string()
            .contains(field));
    }
    assert!(store
        .advance(handle, -1.0)
        .unwrap_err()
        .to_string()
        .contains("-1"));
    let record = store.advance(handle, 1.0).unwrap();
    let previous = Specimen::build(&f).unwrap();
    record
        .validate(
            &previous.buffers().unwrap(),
            &store.specimen(handle).unwrap().buffers().unwrap(),
        )
        .unwrap();
    assert_eq!(before, previous.read().unwrap());
    let next = store.build(&f, 1.0).unwrap();
    assert!(store.read(handle, None).is_err());
    assert!(store
        .read(next, Some(1.0))
        .unwrap_err()
        .to_string()
        .contains("cap"));
    store.release();
    assert!(store.read(next, None).is_err());
    assert_eq!(before, previous.read().unwrap());
}

#[test]
fn specimen_snapshot_round_trip_reads_and_continuation() {
    let f = family();
    let s = Specimen::build(&f).unwrap();
    let bytes = s.snapshot().unwrap();
    let mut imported = Specimen::from_snapshot(&bytes).unwrap();
    assert_eq!(s.read().unwrap(), imported.read().unwrap());
    for age in [0.0, 1.0, 3.75, f.age] {
        let mut earlier = f.clone();
        earlier.age = age;
        assert_eq!(
            imported.read_at_age(age).unwrap(),
            Specimen::build(&earlier).unwrap().read().unwrap()
        );
    }
    imported.advance(4.75).unwrap();
    let mut later = f.clone();
    later.age = 10.0;
    assert_eq!(
        imported.read().unwrap(),
        Specimen::build(&later).unwrap().read().unwrap()
    );
    for length in [0, 1, bytes.len() / 2, bytes.len() - 1] {
        assert!(Specimen::from_snapshot(&bytes[..length]).is_err());
    }
    let mut corrupt = bytes.clone();
    corrupt[0] ^= 1;
    assert!(Specimen::from_snapshot(&corrupt).is_err());
    let mut store = SpecimenStore::default();
    let h = store.build(&f, 100.0).unwrap();
    assert!(store.import(&corrupt).is_err());
    assert_eq!(store.read(h, None).unwrap(), s.read().unwrap());
    let h2 = store.import(&store.snapshot(h).unwrap()).unwrap();
    assert!(store.read(h, None).is_err());
    assert_eq!(store.read(h2, None).unwrap(), s.read().unwrap());
}

#[test]
fn specimen_snapshot_detects_payload_corruption_and_keeps_the_handle() {
    let f = family();
    let mut store = SpecimenStore::default();
    let handle = store.build(&f, 100.0).unwrap();
    let before = store.read(handle, None).unwrap();
    let mut bytes = store.snapshot(handle).unwrap();
    // A bit flip in the root's position is still a well-formed f64. Decode
    // success alone must not publish a corrupted chronicle.
    bytes[70] ^= 1;
    assert!(store.import(&bytes).is_err());
    assert_eq!(before, store.read(handle, None).unwrap());
}

#[test]
fn specimen_handle_ceiling_can_resume_and_snapshot_keeps_compacted_history() {
    let mut f = family();
    f.skeleton.growth.max_nodes = Some(0);
    let mut store = SpecimenStore::default();
    let h = store.build(&f, 2.0).unwrap();
    assert!(store
        .advance(h, 1.0)
        .unwrap_err()
        .to_string()
        .contains("ceiling"));
    store.set_node_ceiling(h, 250_000).unwrap();
    store.advance(h, 12.25).unwrap();
    let bytes = store.snapshot(h).unwrap();
    let imported = store.import(&bytes).unwrap();
    assert!(store
        .read(imported, Some(0.0))
        .unwrap_err()
        .to_string()
        .contains("cap"));
    let before = store.read(imported, None).unwrap();
    store.set_history_cap(imported, 100.0).unwrap();
    assert!(store.read(imported, Some(0.0)).is_err());
    assert_eq!(before, store.read(imported, None).unwrap());
    store.advance(imported, 0.75).unwrap();
    f.age = 13.0;
    f.skeleton.growth.max_nodes = None;
    assert_eq!(
        store.read(imported, None).unwrap(),
        Specimen::build(&f).unwrap().read().unwrap()
    );
}
