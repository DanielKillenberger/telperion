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
