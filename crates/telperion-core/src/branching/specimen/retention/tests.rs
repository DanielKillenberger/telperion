use super::*;
use crate::presets::Preset;

#[test]
fn history_cap_compacts_dead_wood_and_preserves_every_retained_read() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 12.25;
    let original = Specimen::build(&f).unwrap();
    assert!(original.shed > 0);
    for cap in [0.0, 0.25, 2.25, 100.0] {
        let mut s = original.clone();
        let earliest = (f.age - cap).max(0.0);
        let removed = s
            .tree
            .nodes
            .iter()
            .filter(|n| {
                n.shoot
                    .death_year
                    .is_some_and(|death| (death as f64) < earliest)
            })
            .count();
        s.set_history_cap(cap).unwrap();
        assert_eq!(s.tree.nodes.len(), original.tree.nodes.len() - removed);
        assert_eq!(s.read().unwrap(), original.read().unwrap());
        assert_eq!(s.buffers().unwrap(), original.buffers().unwrap());
        for age in [earliest, (earliest + f.age) / 2.0, f.age] {
            assert_eq!(
                s.read_at_age(age).unwrap(),
                original.read_at_age(age).unwrap()
            );
            assert_eq!(
                s.changes_between(age, f.age).unwrap(),
                original.changes_between(age, f.age).unwrap()
            );
        }
        for n in &original.tree.nodes {
            if n.shoot
                .death_year
                .is_some_and(|death| (death as f64) < earliest)
            {
                assert!(s.node(n.identity).is_err());
                assert!(!s.tree.nodes.iter().any(|live| live.identity == n.identity));
            }
        }
    }
}

#[test]
fn history_cap_refuses_expired_ages_and_invalid_settings_atomically() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 12.25;
    let mut s = Specimen::build(&f).unwrap();
    s.set_history_cap(2.0).unwrap();
    let before = s.read().unwrap();
    assert!(s.read_at_age(10.25).is_ok());
    for (from, to) in [(10.0, 12.25), (12.25, 10.0)] {
        let error = s.changes_between(from, to).unwrap_err().to_string();
        assert!(error.contains("cap") && error.contains('2'), "{error}");
    }
    let error = s.read_at_age(10.0).unwrap_err().to_string();
    assert!(error.contains("cap") && error.contains('2'), "{error}");
    for cap in [-1.0, f64::NAN, f64::INFINITY, crate::growth::MAX_AGE + 1.0] {
        let error = s.set_history_cap(cap).unwrap_err().to_string();
        assert!(error.contains("cap"), "{error}");
        assert_eq!(s.read().unwrap(), before);
    }
    s.set_history_cap(100.0).unwrap();
    let error = s.read_at_age(10.0).unwrap_err().to_string();
    assert!(error.contains("cap") && error.contains("100"), "{error}");
    assert_eq!(s.read().unwrap(), before);
}

#[test]
fn history_cap_preserves_future_growth_identities_and_records() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 4.0;
    let original = Specimen::build(&f).unwrap();
    for cap in [0.0, 1.25, 3.0] {
        let mut s = original.clone();
        let mut unbounded = original.clone();
        s.set_history_cap(cap).unwrap();
        for delta in [4.0, 0.25, 0.75, 3.0, 5.0] {
            let record = s.advance(delta).unwrap();
            let expected = unbounded.advance(delta).unwrap();
            assert_eq!(record, expected);
            assert_eq!(s.read().unwrap(), unbounded.read().unwrap());
            assert_eq!(s.next_identity, unbounded.next_identity);
            let cutoff = s.age() - cap;
            assert!(s.tree.nodes.iter().all(|n| n
                .shoot
                .death_year
                .is_none_or(|death| death as f64 >= cutoff)));
        }
    }
}

#[test]
fn history_cap_build_and_node_ceiling_keep_the_same_complete_slice() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 12.25;
    let mut capped = Specimen::build_with_history_cap(&f, 0.0).unwrap();
    let mut full = Specimen::build(&f).unwrap();
    assert_eq!(capped.history_cap(), 0.0);
    assert!(full.history_cap() >= 1000.0);
    assert!(capped.tree.nodes.len() < full.tree.nodes.len());
    assert_eq!(capped.read().unwrap(), full.read().unwrap());
    f.age = 13.0;
    let limit = Specimen::build(&f).unwrap().tree.nodes.len() - 1;
    capped.set_node_ceiling(limit).unwrap();
    full.set_node_ceiling(limit).unwrap();
    assert_eq!(capped.advance(10.0).unwrap(), full.advance(10.0).unwrap());
    assert!(full.tree.diagnostics.node_capped);
    assert_eq!(capped.age(), full.age());
    assert_eq!(capped.read().unwrap(), full.read().unwrap());
    for s in [&mut capped, &mut full] {
        assert!(s
            .advance(1.0)
            .unwrap_err()
            .to_string()
            .contains("node ceiling"));
        s.set_node_ceiling(DEFAULT_MAX_NODES).unwrap();
    }
    assert_eq!(capped.advance(5.0).unwrap(), full.advance(5.0).unwrap());
    assert_eq!(capped.read().unwrap(), full.read().unwrap());
}
