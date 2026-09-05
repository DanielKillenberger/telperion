#[path = "../examples/species_metrics/mod.rs"]
mod species_metrics;

use serde_json::Value;
use telperion_core::{
    branching::{self, BranchHabit},
    foliage::{self, Attachment, ElementAnatomy, TwigPlacement},
    math::Vec3,
    presets::Preset,
    surface,
};

fn profiles() -> Value {
    serde_json::from_str(include_str!("../../../.flow/evidence/fn9/profiles.json")).unwrap()
}

#[test]
fn oak_identity_resolves_to_frozen_profile_and_native_anatomy() {
    let preset = Preset::from_id("oregon-white-oak").unwrap();
    assert_eq!(preset, Preset::OregonWhiteOak);
    let manifest = profiles();
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    assert_eq!(profile["scientific_name"], "Quercus garryana");
    assert_eq!(profile["readiness"], "ready");
    let family = preset.parameters();
    assert!(matches!(family.skeleton.habit, BranchHabit::Spreading(_)));
    assert!(!family.skeleton.bias.supernatural.enabled);
    assert_eq!(family.element.anatomy, ElementAnatomy::LobedBlade);
    assert_eq!(family.canopy.attachment, Attachment::Alternate);
    assert_eq!(family.skeleton.twigs.twig.stations_per_internode, 1);
    assert!(Preset::from_id("Quercus garryana").is_none());
    assert!(Preset::from_id("unknown").is_none());
    for (id, preset) in [
        ("ordinary", Preset::Ordinary),
        ("telperion", Preset::Telperion),
        ("laurelin", Preset::Laurelin),
    ] {
        assert_eq!(Preset::from_id(id), Some(preset));
        assert_eq!(preset.profile_id(), None);
    }
}

#[test]
fn fixed_oaks_pass_geometry_and_profile_gates_with_repeatable_varied_specimens() {
    let manifest = profiles();
    let preset = Preset::OregonWhiteOak;
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    let mut heights = Vec::new();
    let mut widths = Vec::new();
    let mut leaf_counts = std::collections::BTreeSet::new();
    for seed in manifest["protocol"]["fixed_seeds"].as_array().unwrap() {
        let seed = seed.as_u64().unwrap() as u32;
        let mut family = preset.parameters();
        family.skeleton.seed = seed;
        let a = branching::generate(&family.skeleton, family.radii).unwrap();
        let b = branching::generate(&family.skeleton, family.radii).unwrap();
        assert_eq!(a, b, "seed {seed}: skeleton repeatability");
        a.tree.validate_solved().unwrap();
        assert!(a.tree.diagnostics.complete(), "seed {seed}: truncation");
        assert!(a.tree.nodes.len() > a.tree.crossover);
        for node in a.tree.nodes.iter().skip(a.tree.crossover) {
            assert!(family.skeleton.envelope.contains(node.position, 1e-8));
        }
        let wood =
            surface::build(&a.tree, family.skeleton.envelope.height, &family.surface).unwrap();
        let repeat =
            surface::build(&b.tree, family.skeleton.envelope.height, &family.surface).unwrap();
        assert_eq!(wood, repeat, "seed {seed}: surface repeatability");
        assert!(!wood.indices.is_empty());
        assert!(wood
            .positions
            .iter()
            .chain(&wood.normals)
            .all(|v| v.is_finite()));
        assert!(wood
            .indices
            .iter()
            .all(|i| (*i as usize) < wood.positions.len() / 3));
        let point = |i: u32| {
            let p = &wood.positions[i as usize * 3..];
            Vec3::new(p[0] as f64, p[1] as f64, p[2] as f64)
        };
        for triangle in wood.indices.chunks_exact(3) {
            let a = point(triangle[0]);
            assert!(
                (point(triangle[1]) - a)
                    .cross(point(triangle[2]) - a)
                    .length()
                    > 0.0,
                "seed {seed}: degenerate wood triangle"
            );
        }
        let element = foliage::build_element(family.element).unwrap();
        element.validate().unwrap();
        let twig = family.skeleton.twigs.resolved().unwrap().twig;
        let place = || {
            foliage::place(
                &a.tree,
                family.skeleton.envelope,
                seed,
                family.canopy,
                Some(TwigPlacement {
                    internode_length: twig.internode_length,
                    stations_per_internode: twig.stations_per_internode,
                }),
            )
            .unwrap()
        };
        let placed = place();
        assert_eq!(placed, place(), "seed {seed}: attachment repeatability");
        let kept = foliage::cull(
            &placed,
            &element,
            family.skeleton.envelope,
            family.shell_depth,
        )
        .unwrap();
        assert!(!kept.matrices.is_empty());
        let metrics = species_metrics::measure(
            &a.tree,
            &wood.positions,
            &element,
            placed.matrices.len(),
            &kept,
        )
        .unwrap();
        let (pass, checks) = species_metrics::compare(profile, &metrics).unwrap();
        assert!(pass, "seed {seed}: {checks:#}");
        assert_eq!(metrics["units_per_instance"]["value"], 1);
        heights.push(metrics["height_m"]["value"].as_f64().unwrap());
        widths.push(metrics["crown_width_m"]["value"].as_f64().unwrap());
        leaf_counts.insert(kept.matrices.len());
    }
    // Engineering regression thresholds for specimen variation, not botanical ranges.
    for values in [heights, widths] {
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            max - min > 1.0,
            "crown dimensions should vary by over a metre"
        );
    }
    assert!(
        leaf_counts.len() >= 6,
        "seeds must change retained foliage abundance"
    );
}
