use super::*;
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
