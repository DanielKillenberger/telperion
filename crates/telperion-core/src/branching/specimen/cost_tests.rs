use super::*;
use crate::presets::Preset;

#[test]
fn inactive_wood_is_not_evaluated_when_shedding_is_disabled() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 2.0;
    let mut s = Specimen::build(&f).unwrap();
    // Root is not a shoot in either frontier. An environment visit must leave
    // its last sampled state alone when no survival decision needs it.
    s.tree.nodes[0].shoot.vigour = 0.123;
    s.environment(25);
    assert_eq!(s.tree.nodes[0].shoot.vigour, 0.123);
}

#[test]
fn crown_cache_reuses_geometry_and_matches_exhaustive_profile() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 0.0;
    let mut s = Specimen::build(&f).unwrap();
    let mut crown = crown::Crown::default();
    for preset in [
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::Ordinary,
    ] {
        let envelope = preset.parameters().skeleton.envelope;
        crown.prepare(envelope);
        let profile = envelope.profile();
        for i in 0..1000 {
            let n = &mut s.tree.nodes[0];
            n.position = Vec3::new(
                (i % 31) as f64 * envelope.max_radius() / 30.0,
                (i % 37) as f64 * envelope.height / 36.0,
                0.0,
            );
            let radial = n.position.x.hypot_fixed(n.position.z);
            let depth = (envelope.radius_at(n.position.y) - radial)
                .min(distance_to_profile(&profile, radial, n.position.y))
                .max(0.0);
            let expected = (1.0 - depth / envelope.max_radius().max(1e-9)).clamp(0.0, 1.0);
            assert_eq!(crown.exposure(n), expected);
        }
        assert!(
            crown.segments < 16_000,
            "profile still scans every segment: {}",
            crown.segments
        );
        crown.prepare(envelope);
        crown.exposure(&s.tree.nodes[0]);
        assert_eq!(crown.evaluated, 0, "unchanged geometry recomputed");
    }
}

#[test]
fn identity_maintenance_without_structural_birth_visits_only_new_locals() {
    let mut f = Preset::NorwaySpruce.parameters();
    f.age = 0.0;
    f.growth.rate = 1.0;
    let mut s = Specimen::build(&f).unwrap();
    for month in 1..200 {
        let old_len = s.tree.nodes.len();
        s.advance(1.0 / 12.0).unwrap();
        if f.growth.budget(month) > 0 && old_len > 500 && s.cost.storage == 0 {
            assert_eq!(
                s.cost.identities,
                s.tree.nodes.len() - old_len,
                "identity maintenance swept surviving wood"
            );
            assert!(
                s.timeline.as_ref().unwrap().crown.evaluated
                    <= s.scaffold.visited().count() + s.local.visited().count(),
                "vigour sampled shoots this slice never visited"
            );
            s.tree.validate_solved().unwrap();
            return;
        }
    }
    panic!("fixture never reached an active month with no structural insertion");
}

#[test]
fn structural_births_do_not_move_existing_local_storage_inside_a_slice() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 12.0;
    let mut s = Specimen::build(&f).unwrap();
    for _ in 0..120 {
        let month = s.timeline.as_ref().unwrap().age.month + 1;
        let budget = f.growth.budget(month);
        let locals: Vec<_> = s
            .tree
            .nodes
            .iter()
            .enumerate()
            .filter(|(_, n)| n.kind != NodeKind::Structural)
            .map(|(i, n)| (i, n.identity))
            .collect();
        let crossover = s.tree.crossover;
        if budget > 0 {
            s.month(month, budget).unwrap();
        }
        s.timeline.as_mut().unwrap().age = crate::growth::Age {
            month,
            remainder: 0,
        };
        if !locals.is_empty() && s.tree.crossover > crossover {
            for (i, id) in locals {
                assert_eq!(
                    s.identities[id.key], i,
                    "structural birth shifted an existing local node"
                );
                assert_eq!(s.tree.nodes[i].identity, id);
            }
            assert_eq!(s.cost.storage, 0, "slice moved existing storage");
            return;
        }
    }
    panic!("fixture never grew structure after local wood");
}

#[test]
fn packing_between_advances_does_not_change_grown_and_shed_tree_bytes() {
    let mut f = Preset::Ordinary.parameters();
    f.age = 12.25;
    let fresh = Specimen::build(&f).unwrap();
    assert!(fresh.shed > 0, "fixture must shed as well as grow");
    assert!(fresh.tree.nodes.len() > 100);
    f.age = 0.0;
    let mut chain = Specimen::build(&f).unwrap();
    for years in [4.0, 4.0, 4.25] {
        chain.advance(years).unwrap();
        chain.pack_storage();
    }
    assert_eq!(chain.shed, fresh.shed);
    assert_eq!(chain.next_identity, fresh.next_identity);
    assert!(
        super::tests::bytes(chain.tree()) == super::tests::bytes(fresh.tree()),
        "packing boundaries changed structure, radii, shoot state or generational identities"
    );
}

#[test]
fn packing_keeps_append_headroom_for_the_next_slice() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 12.0;
    let mut s = Specimen::build(&f).unwrap();
    let month = s.timeline.as_ref().unwrap().age.month + 1;
    s.month(month, f.growth.budget(month)).unwrap();
    assert!(s.timeline.as_ref().unwrap().unpacked);
    s.tree.nodes.reserve(s.tree.nodes.len());
    let capacity = s.tree.nodes.capacity();
    s.pack_storage();
    assert!(
        s.tree.nodes.capacity() >= capacity,
        "packing discarded headroom and forces the next birth to copy the entire tree"
    );
}

#[test]
fn an_internal_slice_does_not_finalize_existing_local_widths() {
    let mut f = Preset::OregonWhiteOak.parameters();
    f.age = 20.0;
    let mut s = Specimen::build(&f).unwrap();
    let month = s.timeline.as_ref().unwrap().age.month + 1;
    s.month(month, f.growth.budget(month)).unwrap();
    assert_eq!(s.cost.widths, 0, "slice finalized local output widths");
}
