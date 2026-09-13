use super::*;
use crate::{
    foliage::Placement,
    presets::{Family, Preset},
};

pub(super) fn fixture(contact: f64) -> (Family, Specimen, NodeIdentity) {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 0.0;
    f.canopy.surface_contact = contact;
    let mut s = Specimen::build(&f).unwrap();
    s.tree.nodes.push(Node {
        position: Vec3::new(0.0, 1.0, 0.0),
        radius: 0.02,
        start_radius: 0.02,
        base_radius: 0.02,
        parent: Some(0),
        branch: 1,
        ..Node::root()
    });
    s.tree.crossover = 2;
    s.tree.nodes.push(Node {
        position: Vec3::new(0.2, 1.15, 0.0),
        radius: 0.002,
        start_radius: 0.004,
        base_radius: 0.004,
        parent: Some(1),
        branch: 2,
        kind: NodeKind::Twig,
        ..Node::root()
    });
    s.identify();
    s.tree.nodes[2].shoot.birth_year = 1.0;
    let t = s.timeline.as_mut().unwrap();
    t.envelope = f.skeleton.envelope;
    t.age = crate::growth::Age::from_years(1.0).unwrap();
    let id = s.tree.nodes[2].identity;
    // This fixture is a chronicle too: its hand-built wood needs birth frames.
    super::interval::tests::stamp(&mut s, 1);
    (f, s, id)
}

fn ids(placed: &[Placement]) -> Vec<crate::foliage::PlacementIdentity> {
    placed.iter().map(|p| p.identity).collect()
}

#[test]
fn leaves_hold_across_the_exact_lifetime_boundary_without_wood_growth() {
    let (_, mut s, shoot) = fixture(0.0);
    let born = s.placements().unwrap();
    assert!(!born.is_empty(), "new shoot must bear leaves");
    assert!(born.iter().all(|p| p.identity.shoot == shoot));
    s.timeline.as_mut().unwrap().age =
        crate::growth::Age::from_years(2.0 - 1.0 / 12_000_000_000.0).unwrap();
    assert_eq!(s.placements().unwrap(), born, "leaves expired early");
    s.timeline.as_mut().unwrap().age = crate::growth::Age::from_years(2.0).unwrap();
    assert_eq!(
        s.placements().unwrap(),
        born,
        "cohort replacement lost stations"
    );
    assert!(s.node(shoot).is_ok(), "cohorts must leave wood alive");
}

#[test]
fn placement_identities_survive_thickening_and_match_fresh_surface_contacts() {
    for contact in [0.0, 1.0] {
        let (f, mut s, shoot) = fixture(contact);
        let old = s.placements().unwrap();
        assert!(!old.is_empty(), "surface fixture must bear leaves");
        s.tree.nodes[2].start_radius *= 2.0;
        s.tree.nodes[2].radius *= 2.0;
        s.read.take();
        let moved = s.placements().unwrap();
        assert_eq!(ids(&old), ids(&moved));
        assert_ne!(
            old, moved,
            "thickening did not move stations at contact={contact}"
        );
        let fresh = crate::foliage::timeline::Foliage::new(&f)
            .unwrap()
            .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
            .unwrap();
        assert_eq!(moved, fresh);
        s.retire(&[shoot]);
        s.read.take();
        assert!(
            s.placements().unwrap().is_empty(),
            "retired shoot retained foliage"
        );
    }
}

pub(super) fn sibling(s: &mut Specimen) -> NodeIdentity {
    let mut node = s.tree.nodes[2].clone();
    node.identity = NodeIdentity::default();
    node.branch = s.tree.nodes.len() as u32;
    node.position = Vec3::new(-0.2, 1.15, 0.0);
    s.tree.nodes.push(node);
    s.identify();
    s.tree.nodes.last_mut().unwrap().shoot.birth_year = 1.0;
    s.read.take();
    s.tree.nodes.last().unwrap().identity
}

#[test]
fn foliage_rederives_only_shoots_whose_wood_or_contact_changed() {
    for contact in [0.0, 1.0] {
        let (_, mut s, _) = fixture(contact);
        sibling(&mut s);
        let old = s.placements().unwrap();
        assert!(!old.is_empty());
        assert_eq!(s.timeline.as_ref().unwrap().foliage.derived(), 2);
        assert_eq!(s.placements().unwrap(), old);
        assert_eq!(
            s.timeline.as_ref().unwrap().foliage.derived(),
            0,
            "unchanged wood re-derived leaf matrices"
        );
        if contact == 0.0 {
            s.tree.nodes[2].radius *= 1.5;
            s.read.take();
            assert_ne!(s.placements().unwrap(), old);
            assert_eq!(
                s.timeline.as_ref().unwrap().foliage.derived(),
                1,
                "a changed shoot re-derived its unchanged sibling"
            );
        }
    }
}

#[test]
fn surviving_leaf_randomness_is_independent_of_compaction_and_retired_generations() {
    // Isolate randomness from a legitimately changed fork contact polygon.
    let (f, mut s, first) = fixture(0.0);
    let survivor = sibling(&mut s);
    let before: Vec<_> = s
        .placements()
        .unwrap()
        .into_iter()
        .filter(|p| p.identity.shoot == survivor)
        .collect();
    assert!(!before.is_empty());
    s.retire(&[first]);
    s.read.take();
    // Use a cold cache so a retained transform cannot hide a storage-keyed RNG.
    let fresh = crate::foliage::timeline::Foliage::new(&f)
        .unwrap()
        .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
        .unwrap();
    assert_eq!(
        fresh, before,
        "storage compaction changed surviving leaf bytes"
    );
    let new = sibling(&mut s);
    assert_ne!(new, first);
    assert!(s.node(first).is_err());
    assert!(s
        .placements()
        .unwrap()
        .iter()
        .all(|p| p.identity.shoot != first));
}

#[test]
fn leaf_lifetime_zero_fractional_and_maximum_are_valid() {
    for lifetime in [0.0, 1.25, crate::growth::MAX_AGE] {
        let (mut f, mut s, _) = fixture(0.0);
        f.growth.leaf_lifetime = lifetime;
        let foliage = crate::foliage::timeline::Foliage::new(&f).unwrap();
        assert_eq!(
            foliage
                .read(
                    s.tree(),
                    s.envelope(),
                    crate::growth::Age::from_years(1.0).unwrap()
                )
                .unwrap()
                .is_empty(),
            lifetime == 0.0
        );
        if lifetime == 1.25 {
            let before = foliage
                .read(
                    s.tree(),
                    s.envelope(),
                    crate::growth::Age::from_years(2.25 - 1.0 / 12_000_000_000.0).unwrap(),
                )
                .unwrap();
            assert!(!before.is_empty());
            assert_eq!(
                foliage
                    .read(
                        s.tree(),
                        s.envelope(),
                        crate::growth::Age::from_years(2.25).unwrap()
                    )
                    .unwrap(),
                before,
                "fractional lifetime expired a cohort"
            );
        }
        // Keep the last valid age within the clock's supported range.
        s.timeline.as_mut().unwrap().age =
            crate::growth::Age::from_years(crate::growth::MAX_AGE).unwrap();
        assert_eq!(
            foliage
                .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
                .unwrap()
                .is_empty(),
            lifetime == 0.0
        );
    }
}

#[test]
fn annual_leaf_buffers_replay_exactly_across_reads_for_all_presets_and_a_blend() {
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
        // Find a leaf-bearing stage: an annual preset may legitimately have no
        // living leaf shoots at a particular age. Keep the nonempty assertion.
        f.growth.leaf_lifetime = crate::growth::MAX_AGE;
        f.age = 0.0;
        let mut stage = Specimen::build(&f).unwrap();
        for _ in 0..30 {
            stage.advance(1.0).unwrap();
            if !stage.placements().unwrap().is_empty() {
                break;
            }
        }
        assert!(
            !stage.placements().unwrap().is_empty(),
            "replay fixture must bear leaves"
        );
        let years = stage.age() as usize;
        f.age = stage.age() + 0.25;
        let fresh = Specimen::build(&f).unwrap();
        f.age = 0.0;
        let mut replay = Specimen::build(&f).unwrap();
        for _ in 0..years {
            for delta in [0.4, 0.6] {
                replay.advance(delta).unwrap();
                replay.placements().unwrap();
            }
        }
        replay.advance(0.25).unwrap();
        assert_eq!(replay.age(), fresh.age());
        assert_eq!(replay.placements().unwrap(), fresh.placements().unwrap());
    }
}

#[test]
fn changed_neighboring_contact_polygons_move_an_unchanged_surviving_shoot() {
    let (f, mut s, first) = fixture(1.0);
    let survivor = sibling(&mut s);
    let before: Vec<_> = s
        .placements()
        .unwrap()
        .into_iter()
        .filter(|p| p.identity.shoot == survivor)
        .collect();
    assert!(!before.is_empty());
    let wood = s.node(survivor).unwrap().clone();
    s.retire(&[first]);
    s.read.take();
    let live = s.node(survivor).unwrap();
    assert_eq!(
        (wood.position, wood.radius, wood.start_radius),
        (live.position, live.radius, live.start_radius)
    );
    let fresh = crate::foliage::timeline::Foliage::new(&f)
        .unwrap()
        .read(s.tree(), s.envelope(), s.timeline.as_ref().unwrap().age)
        .unwrap();
    assert_eq!(ids(&before), ids(&fresh));
    assert_ne!(
        before, fresh,
        "fixture must change the surviving socket surface"
    );
    assert_eq!(
        s.placements().unwrap(),
        fresh,
        "cache missed a changed neighboring polygon"
    );
    assert_eq!(s.timeline.as_ref().unwrap().foliage.derived(), 1);
}

// The cohort redesign moves the exact-tick check from expiry to fill-in.
#[test]
fn cohort_fill_preserves_integer_ticks_near_the_maximum_age() {
    let (mut f, mut s, _) = fixture(0.0);
    f.growth.leaf_lifetime = 2.0;
    s.tree.nodes[2].shoot.birth_year = crate::growth::MAX_AGE - 2.0;
    let t = s.timeline.as_mut().unwrap();
    t.foliage = crate::foliage::timeline::Foliage::new(&f).unwrap();
    t.age = crate::growth::Age {
        slice: 999_998,
        remainder: 11_999_999_999,
    };
    let before = s.placements().unwrap();
    assert!(!before.is_empty());
    let t = s.timeline.as_mut().unwrap();
    t.age = t.age.advanced(1.0 / 12_000_000_000.0).unwrap();
    let after = s.placements().unwrap();
    assert!(after.len() > before.len(), "cohort filled one tick early");
    assert!(before.iter().all(|p| after.contains(p)));
}
