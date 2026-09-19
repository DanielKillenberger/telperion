use super::*;
fn placement(birth: u64, station: u32, word: u32) -> Placement {
    Placement {
        identity: PlacementIdentity {
            shoot: NodeIdentity {
                birth,
                ..Default::default()
            },
            station,
        },
        leaf: [word; 3],
    }
}
fn foliage_pages(read: &PackedRead, pages: &mut std::collections::BTreeSet<usize>) {
    read.foliage.collect_pages(pages);
    for stations in read.foliage.iter() {
        stations.collect_pages(pages);
    }
}
#[test]
fn packed_placement_pages_are_dense_in_birth_order() {
    let mut read = PackedRead::default();
    read.apply(&ChangeRecord {
        born_placements: (0..256).map(|birth| placement(birth, 0, 0)).collect(),
        ..Default::default()
    });
    let mut pages = std::collections::BTreeSet::new();
    foliage_pages(&read, &mut pages);
    assert!(
        pages.len() <= 256 * 2,
        "one station per birth allocated {} radix pages",
        pages.len()
    );
}

#[test]
#[ignore = "bounded packed-cache cost measurement; run without competing suites"]
fn packed_placement_cost_measurement() {
    use std::{collections::BTreeSet, time::Instant};
    for sample in 0..3 {
        let record = ChangeRecord {
            born_placements: (0..4096)
                .flat_map(|birth| (0..16).map(move |station| placement(birth, station, 0)))
                .collect(),
            ..Default::default()
        };
        let edits = ChangeRecord {
            moved_placements: (0..4096).map(|birth| placement(birth, 0, 1)).collect(),
            ..Default::default()
        };
        let load = std::fs::read_to_string("/proc/loadavg").unwrap();
        let mut read = PackedRead::default();
        let clock = Instant::now();
        read.apply(&record);
        let build_ms = clock.elapsed().as_secs_f64() * 1000.;
        let before = read.clone();
        let clock = Instant::now();
        read.apply(&edits);
        let edit_ms = clock.elapsed().as_secs_f64() * 1000.;
        let mut pages = BTreeSet::new();
        foliage_pages(&before, &mut pages);
        let before_pages = pages.len();
        foliage_pages(&read, &mut pages);
        println!(
            "{}",
            serde_json::json!({"layout":"packed", "sample":sample, "births":4096,"stations":16,"loadavg":load.trim(),"build_ms":build_ms,"edit_ms":edit_ms,"pages":before_pages,"retained_edit_pages":pages.len()})
        );
        assert_eq!(
            before.placement(placement(1, 0, 0).identity).unwrap().leaf,
            [0; 3]
        );
        assert_eq!(read.placement_count(), 4096 * 16);

        // Historical dense packing is valid for this bounded <=512-station fixture only.
        let key =
            |p: &Placement| p.identity.shoot.birth_order() * 512 + u64::from(p.identity.station);
        let mut dense = Map::default();
        let clock = Instant::now();
        for p in &record.born_placements {
            dense.set(key(p), Some(p.clone()));
        }
        let build_ms = clock.elapsed().as_secs_f64() * 1000.;
        let before = dense.clone();
        let clock = Instant::now();
        for p in &edits.moved_placements {
            dense.set(key(p), Some(p.clone()));
        }
        let edit_ms = clock.elapsed().as_secs_f64() * 1000.;
        let mut pages = BTreeSet::new();
        before.collect_pages(&mut pages);
        let before_pages = pages.len();
        dense.collect_pages(&mut pages);
        println!(
            "{}",
            serde_json::json!({"layout":"historical-dense", "sample":sample,"births":4096,"stations":16,"loadavg":load.trim(),"build_ms":build_ms,"edit_ms":edit_ms,"pages":before_pages,"retained_edit_pages":pages.len()})
        );
    }
}
#[test]
fn placement_keys_preserve_stations_above_512() {
    let mut read = PackedRead::default();
    let entries = [
        placement(1, 512, 1),
        placement(2, 0, 2),
        placement(u64::MAX, u32::MAX, 3),
    ];
    for p in &entries {
        read.put_placement(p.clone());
    }
    for p in &entries {
        assert_eq!(read.placement(p.identity), Some(p));
    }
    assert_eq!(read.placement_count(), entries.len());
}
#[test]
fn placement_edits_preserve_owned_snapshots_counts_and_order() {
    let entries = [placement(2, 0, 2), placement(1, 512, 5), placement(1, 0, 1)];
    let mut read = PackedRead::default();
    for p in &entries {
        read.put_placement(p.clone());
    }
    let mut unique_pages = std::collections::BTreeSet::new();
    foliage_pages(&read, &mut unique_pages);
    read.put_placement(entries[2].clone());
    let mut edited_pages = std::collections::BTreeSet::new();
    foliage_pages(&read, &mut edited_pages);
    assert_eq!(
        unique_pages, edited_pages,
        "an unshared edit should reuse its radix pages"
    );
    let mut stale = entries[2].identity;
    stale.shoot.key = crate::tree::NodeKey::from(slotmap::KeyData::from_ffi(1));
    assert_ne!(stale, entries[2].identity);
    assert!(read.placement(stale).is_none());
    read.remove_placement(stale);
    assert_eq!(read.placement_count(), 3);
    let before = read.clone();
    let untouched = before.placement(entries[1].identity).unwrap();
    read.apply(&ChangeRecord {
        born_placements: vec![placement(1, 1, 3)],
        moved_placements: vec![placement(1, 0, 9)],
        shed_placements: vec![entries[0].identity],
        ..Default::default()
    });
    assert_eq!(read.placement_count(), 3);
    assert_eq!(
        read.placements()
            .map(|p| (
                p.identity.shoot.birth_order(),
                p.identity.station,
                p.leaf[0]
            ))
            .collect::<Vec<_>>(),
        [(1, 0, 9), (1, 1, 3), (1, 512, 5)]
    );
    assert!(std::ptr::eq(
        untouched,
        read.placement(entries[1].identity).unwrap()
    ));
    assert_eq!(before.placement_count(), 3);
    for p in &entries {
        assert_eq!(before.placement(p.identity), Some(p));
    }
    let ids: Vec<_> = read.placements().map(|p| p.identity).collect();
    for id in ids {
        read.remove_placement(id);
        read.remove_placement(id);
    }
    assert_eq!(read.placement_count(), 0);
    assert_eq!(read.foliage.len(), 0);
    assert!(read.foliage.get(1).is_none());
    assert_eq!(read.placements().count(), 0);
    assert_eq!(before.placements().count(), 3);
}
#[test]
fn shared_reads_keep_unchanged_payloads_and_reconcile_every_age() {
    let mut f = crate::presets::Preset::Ordinary.parameters();
    f.age = 0.0;
    let mut s = Specimen::build(&f).unwrap();
    let mut before = s.read_packed().unwrap();
    for year in 1..=12 {
        let record = s.advance(1.0).unwrap();
        let after = s.read_packed().unwrap();
        let named: std::collections::BTreeSet<_> = record
            .born_runs
            .iter()
            .chain(&record.resized_runs)
            .map(|r| r.identity)
            .chain(record.shed_runs.iter().copied())
            .collect();
        for n in before.nodes().filter(|n| !named.contains(&n.run)) {
            assert!(
                std::ptr::eq(n, after.node(n.wood.identity).unwrap()),
                "read copied a node absent from the change record"
            );
        }
        let moved: std::collections::BTreeSet<_> = record
            .born_placements
            .iter()
            .chain(&record.moved_placements)
            .map(|p| p.identity)
            .chain(record.shed_placements.iter().copied())
            .collect();
        for p in before.placements().filter(|p| !moved.contains(&p.identity)) {
            assert!(
                std::ptr::eq(p, after.placement(p.identity).unwrap()),
                "read copied a placement on unrelated wood"
            );
        }
        assert_eq!(s.cost.remapped_nodes, 0);
        assert_eq!(s.cost.packed_nodes, 0);
        for age in 0..=year {
            let view = s.read_packed_at_age(age as f64).unwrap();
            f.age = age as f64;
            let fresh = Specimen::build(&f).unwrap();
            let expected = super::super::interval::tests::buffers(&fresh, age as f64);
            let mut actual = SpecimenBuffers::default();
            for n in view.nodes() {
                actual
                    .runs
                    .entry(n.run)
                    .or_insert_with(|| Run {
                        identity: n.run,
                        nodes: Vec::new(),
                    })
                    .nodes
                    .push(n.wood.clone());
            }
            actual.placements = view.placements().map(|p| (p.identity, p.clone())).collect();
            assert_eq!(actual, expected);
            assert!(view
                .nodes()
                .take(view.crossover())
                .all(|n| n.wood.kind == NodeKind::Structural));
        }
        before = after;
    }
}

#[test]
fn fractional_contact_advance_does_not_materialize_numeric_offsets() {
    let mut f = crate::presets::Preset::NorwaySpruce.parameters();
    f.age = 12.0;
    let mut s = Specimen::build(&f).unwrap();
    let before = s.read_packed().unwrap();
    assert!(s.read.get().is_none());
    let record = s.advance(0.25).unwrap();
    assert_eq!(record, ChangeRecord::default());
    let after = s.read_packed().unwrap();
    assert!(
        s.read.get().is_none(),
        "clock-only advance materialized all numeric offsets"
    );
    for (a, b) in before.nodes().zip(after.nodes()) {
        assert!(std::ptr::eq(a, b));
    }
    for (a, b) in before.placements().zip(after.placements()) {
        assert!(std::ptr::eq(a, b));
    }
}
