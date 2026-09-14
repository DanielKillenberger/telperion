use super::*;
use crate::presets::Preset;

pub(in crate::branching::specimen) fn buffers(s: &Specimen, age: f64) -> SpecimenBuffers {
    let read = s.read_at_age(age).unwrap();
    let mut out = SpecimenBuffers::default();
    for n in &read.tree.nodes {
        let id = read.tree.nodes[n.branch as usize].identity;
        out.runs
            .entry(id)
            .or_insert_with(|| Run {
                identity: id,
                nodes: Vec::new(),
            })
            .nodes
            .push(RunNode {
                identity: n.identity,
                parent: n.parent.map(|p| read.tree.nodes[p as usize].identity),
                position: n.position,
                radii: [n.radius, n.start_radius, n.base_radius],
                kind: n.kind,
            });
    }
    for run in out.runs.values_mut() {
        run.nodes.sort_by_key(|n| n.identity);
    }
    out.placements = read
        .placements
        .into_iter()
        .map(|p| (p.identity, p))
        .collect();
    out
}

#[test]
fn interval_records_reconcile_all_age_pairs_presets_and_blend() {
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
    for mut family in families {
        family.age = 12.25;
        family.growth.leaf_lifetime = 6.0;
        let s = Specimen::build(&family).unwrap();
        let ages = [0.0, 1.0, 4.25, 8.0, 12.25];
        let reads: Vec<_> = ages.iter().map(|&a| buffers(&s, a)).collect();
        for (i, &from) in ages.iter().enumerate() {
            for (j, &to) in ages.iter().enumerate() {
                let record = s.changes_between(from, to).unwrap();
                record.validate(&reads[i], &reads[j]).unwrap();
                let mut applied = reads[i].clone();
                record.apply(&mut applied).unwrap();
                assert_eq!(applied, reads[j], "interval {from} -> {to}");
            }
        }
    }
}

#[test]
fn interval_records_are_canonical_for_every_intermediate_year() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 10.0;
    let jump = Specimen::build(&family).unwrap();
    family.age = 0.0;
    let mut yearly = Specimen::build(&family).unwrap();
    for year in 0..=10 {
        if year > 0 {
            yearly.advance(1.0).unwrap();
        }
        for a in 0..=year {
            assert_eq!(
                jump.read_at_age(a as f64).unwrap(),
                yearly.read_at_age(a as f64).unwrap()
            );
            for b in 0..=year {
                let record = jump.changes_between(a as f64, b as f64).unwrap();
                assert_eq!(record, yearly.changes_between(a as f64, b as f64).unwrap());
                record
                    .validate(&buffers(&yearly, a as f64), &buffers(&yearly, b as f64))
                    .unwrap();
            }
        }
    }
}

/// Bring the small contact fixture into the chronicle without running growth.
pub(in crate::branching::specimen) fn stamp(s: &mut Specimen, year: u64) {
    for n in &s.tree.nodes {
        // Synthetic clock fixtures relocate births; keep the event index in sync.
        s.births.record(n.shoot.birth_year as u64, n.identity.key);
        s.keyframes.record(
            n.identity,
            year,
            [n.radius, n.start_radius, n.base_radius],
            0.0,
        );
    }
    let t = s.timeline.as_mut().unwrap();
    t.age.slice = year;
    t.years.push(history::Year {
        year,
        envelope: t.envelope,
        diagnostics: s.tree.diagnostics,
    });
}

#[test]
fn interval_records_follow_sibling_growth_and_removal_contacts() {
    for removal in [false, true] {
        let (_, mut s, shoot) = super::super::foliage_tests::fixture(1.0);
        let sibling = super::super::foliage_tests::sibling(&mut s);
        let (shoot, sibling) = if removal {
            (sibling, shoot)
        } else {
            (shoot, sibling)
        };
        stamp(&mut s, 1);
        let before = buffers(&s, 1.0);
        let own = s.tree.nodes[s.identities[shoot.key]].clone();
        if removal {
            s.stamp_deaths(&[sibling], 2);
        } else {
            let n = &mut s.tree.nodes[s.identities[sibling.key]];
            n.radius *= 1.5;
            n.start_radius *= 1.5;
        }
        stamp(&mut s, 2);
        s.read.take();
        assert_eq!(s.tree.nodes[s.identities[shoot.key]], own);
        let after = buffers(&s, 2.0);
        let leaves = |b: &SpecimenBuffers| {
            b.placements
                .values()
                .filter(|p| p.identity.shoot == shoot)
                .cloned()
                .collect::<Vec<_>>()
        };
        assert_ne!(
            leaves(&before),
            leaves(&after),
            "contact fixture did not move"
        );
        let record = s.changes_between(1.0, 2.0).unwrap();
        assert!(record
            .moved_placements
            .iter()
            .any(|p| p.identity.shoot == shoot));
        record.validate(&before, &after).unwrap();
        s.changes_between(2.0, 1.0)
            .unwrap()
            .validate(&after, &before)
            .unwrap();
    }
}

#[test]
fn interval_records_validate_both_ages_without_running_simulation() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 4.0;
    let s = Specimen::build(&f).unwrap();
    let original = super::super::tests::bytes(&s.tree);
    let next = s.next_identity;
    let cost = s.cost.stages;
    for (a, b) in [(5.0, 2.0), (2.0, 5.0), (-1.0, 0.0), (0.0, f64::NAN)] {
        let error = s.changes_between(a, b).unwrap_err().to_string();
        assert!(error.contains("age"), "{error}");
        if a == 5.0 || b == 5.0 {
            assert!(error.contains("frontier") && error.contains('4'));
        }
    }
    let record = s.changes_between(0.0, 4.0).unwrap();
    record
        .validate(&buffers(&s, 0.0), &buffers(&s, 4.0))
        .unwrap();
    assert_eq!(super::super::tests::bytes(&s.tree), original);
    assert_eq!(s.next_identity, next);
    assert_eq!(s.cost.stages, cost);
}

#[test]
fn interval_advance_cannot_reuse_older_cached_cohort_transforms() {
    let (mut f, mut s, _) = super::super::foliage_tests::fixture(1.0);
    f.growth.leaf_lifetime = 6.0;
    s.tree.nodes[2].shoot.birth_year = crate::growth::MAX_AGE - 2.0;
    s.timeline.as_mut().unwrap().foliage = crate::foliage::timeline::Foliage::new(&f).unwrap();
    stamp(&mut s, 999_998);
    let mut consumer = s.buffers().unwrap();
    assert!(!consumer.placements.is_empty());
    s.tree.nodes[2].radius *= 1.5;
    s.tree.nodes[2].start_radius *= 1.5;
    stamp(&mut s, 999_999);
    s.read.take();
    s.changes_between(999_998.0, 999_999.0)
        .unwrap()
        .apply(&mut consumer)
        .unwrap();
    // No frontier read in between: only records update the consumer.
    let filled = s.advance(1.0).unwrap();
    assert!(!filled.born_placements.is_empty());
    filled.apply(&mut consumer).unwrap();
    let fresh = crate::foliage::timeline::Foliage::new(&f)
        .unwrap()
        .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
        .unwrap();
    assert!(
        consumer.placements.values().cloned().collect::<Vec<_>>() == fresh,
        "clock-only cohorts used transforms cached before the last wood interval"
    );
}

#[test]
fn empty_late_interval_visits_no_unrelated_chronicle_nodes() {
    let mut family = Preset::Ordinary.parameters();
    family.age = 200.0;
    let s = Specimen::build(&family).unwrap();
    s.cost.event_visits.set(0);
    assert_eq!(
        s.changes_between(198.0, 199.0).unwrap(),
        ChangeRecord::default()
    );
    assert_eq!(
        s.cost.event_visits.get(),
        0,
        "empty interval scanned the chronicle"
    );
}

#[test]
fn envelope_growth_without_wood_keyframes_moves_no_placement() {
    let (_, mut s, shoot) = super::super::foliage_tests::fixture(1.0);
    stamp(&mut s, 1);
    let before = buffers(&s, 1.0);
    s.timeline.as_mut().unwrap().envelope.height *= 1.1;
    stamp(&mut s, 2);
    assert!(!s.keyframes.changed(shoot, 1, 2));
    let after = buffers(&s, 2.0);
    assert_eq!(
        before.placements, after.placements,
        "envelope growth moved unchanged wood contacts"
    );
    assert!(s
        .changes_between(1.0, 2.0)
        .unwrap()
        .moved_placements
        .is_empty());
    s.changes_between(1.0, 2.0)
        .unwrap()
        .validate(&before, &after)
        .unwrap();
}

#[test]
fn sparse_spruce_projects_only_changed_contact_paths() {
    let mut f = Preset::NorwaySpruce.parameters();
    f.skeleton.seed = 7;
    // Place year 66 in the sparse phase of the retained-bud growth rule.
    f.growth.rate = 0.12;
    f.growth.shape = 2.0;
    // Mature-radius fixture: secondary thickening would deliberately move contacts.
    f.growth.juvenile_radius = 1.0;
    f.growth.seedling_height = 0.0;
    f.growth.juvenile_height = 0.0;
    f.growth.seedling_radius = 0.0;
    f.growth.crown_base_retention = 0.0;
    f.age = 66.0;
    let mut s = Specimen::build(&f).unwrap();
    let record = s.advance(1.0).unwrap();
    assert!(record.moved_placements.len() < 20_000);
    assert!(
        s.cost.interval_nodes.get() < s.tree.nodes.len() / 4,
        "sparse contact interval projected {} nodes",
        s.cost.interval_nodes.get()
    );
    record
        .validate(&buffers(&s, 66.0), &buffers(&s, 67.0))
        .unwrap();
}
