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
    fixed_species(Preset::OregonWhiteOak);
}

#[test]
fn fixed_spruces_pass_geometry_and_profile_gates_with_repeatable_varied_specimens() {
    fixed_species(Preset::NorwaySpruce);
}

fn fixed_species(preset: Preset) {
    let manifest = profiles();
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    let mut heights = Vec::new();
    let mut widths = Vec::new();
    let mut leaf_counts = std::collections::BTreeSet::new();
    let mut seeds: Vec<u32> = manifest["protocol"]["fixed_seeds"]
        .as_array()
        .unwrap()
        .iter()
        .map(|seed| seed.as_u64().unwrap() as u32)
        .collect();
    if preset == Preset::NorwaySpruce {
        // Original retained crown-width failure; never replace with a showcase seed.
        seeds.push(4_250_668_600);
    }
    for seed in seeds {
        let mut family = preset.parameters();
        family.skeleton.seed = seed;
        let a = branching::generate(&family.skeleton, family.radii).unwrap();
        let b = branching::generate(&family.skeleton, family.radii).unwrap();
        assert_eq!(a, b, "seed {seed}: skeleton repeatability");
        a.tree.validate_solved().unwrap();
        if preset == Preset::NorwaySpruce {
            let structural = &a.tree.nodes[..a.tree.crossover];
            let axial = |p: Vec3| p.x.hypot(p.z) < 1e-10;
            let mut tiers = std::collections::BTreeMap::<u32, usize>::new();
            let mut hanging = 0;
            let mut upturned = 0;
            assert!(structural.iter().any(|n| axial(n.position)
                && (n.position.y - family.skeleton.envelope.height).abs() < 1e-9));
            for n in structural.iter().skip(1) {
                let parent_id = n.parent.unwrap();
                let parent = &structural[parent_id as usize];
                let delta = n.position - parent.position;
                if axial(n.position) {
                    assert!(axial(parent.position) && delta.y > 0.0);
                } else if axial(parent.position) {
                    *tiers.entry(parent_id).or_default() += 1;
                }
                hanging += usize::from(delta.y < -0.05 && -delta.y > delta.x.hypot(delta.z));
                upturned += usize::from(delta.y > 0.01 && delta.x.hypot(delta.z) > delta.y);
            }
            // Structural engineering invariants, independent of foliage AABB gates.
            assert!(
                tiers.len() >= 12 && tiers.values().all(|n| *n >= 3),
                "seed {seed}: tiers {tiers:?}"
            );
            assert!(tiers
                .keys()
                .any(|i| structural[*i as usize].position.y < 2.0));
            assert!(
                hanging > 50 && upturned > 20,
                "seed {seed}: hanging {hanging}, upturned {upturned}"
            );
        }
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
        for triangle in wood.indices.as_chunks::<3>().0.iter() {
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
        if preset == Preset::NorwaySpruce {
            assert_eq!(metrics["foliage_unit"], "needle");
            assert_eq!(metrics["foliage_units"]["value"], kept.matrices.len());
            assert!(metrics["needle_surface_area_m2"]["value"].as_f64().unwrap() > 0.0);
            assert!(metrics["crown_base_m"]["value"].as_f64().unwrap() < 3.0);
        }
        heights.push(metrics["height_m"]["value"].as_f64().unwrap());
        widths.push(metrics["crown_width_m"]["value"].as_f64().unwrap());
        leaf_counts.insert(kept.matrices.len());
    }
    // Engineering regression thresholds for specimen variation, not botanical ranges.
    let dimensions = if preset == Preset::NorwaySpruce {
        // Tiered leader height is authored; azimuth, curtains and crown width vary.
        vec![widths]
    } else {
        vec![heights, widths]
    };
    for values in dimensions {
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            max - min
                > if preset == Preset::NorwaySpruce {
                    0.1
                } else {
                    1.0
                },
            "crown dimensions should vary across specimens"
        );
    }
    assert!(
        leaf_counts.len() >= 6,
        "seeds must change retained foliage abundance"
    );
}

#[test]
fn spruce_identity_resolves_to_frozen_profile_and_native_anatomy() {
    let preset = Preset::from_id("norway-spruce").unwrap();
    assert_eq!(preset, Preset::NorwaySpruce);
    assert_eq!(preset.profile_id(), Some("norway-spruce"));
    let manifest = profiles();
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    assert_eq!(profile["scientific_name"], "Picea abies");
    assert_eq!(profile["readiness"], "ready");
    let family = preset.parameters();
    assert!(matches!(family.skeleton.habit, BranchHabit::Tiered(_)));
    assert!(!family.skeleton.bias.supernatural.enabled);
    assert_eq!(family.element.anatomy, ElementAnatomy::FourSidedNeedle);
    assert!(family.element.connector_length > 0.0);
    assert_eq!(family.canopy.attachment, Attachment::RadialNeedles);
    assert_eq!(family.skeleton.twigs.twig.stations_per_internode, 1);
    assert!(Preset::from_id("Picea abies").is_none());
}

#[test]
fn scaffold_reaches_and_hanging_secondaries_subdivide_before_their_tips() {
    for preset in [Preset::OregonWhiteOak, Preset::NorwaySpruce] {
        let family = preset.parameters();
        let report = branching::generate(&family.skeleton, family.radii).unwrap();
        let nodes = &report.tree.nodes[..report.tree.crossover];
        let mut children = vec![Vec::new(); nodes.len()];
        for (i, node) in nodes.iter().enumerate().skip(1) {
            children[node.parent.unwrap() as usize].push(i);
        }
        let mut mid_axis_forks = 0;
        for (i, node) in nodes.iter().enumerate().skip(1) {
            if children[i].len() < 2 {
                continue;
            }
            let from = (node.position - nodes[node.parent.unwrap() as usize].position).normalized();
            if preset == Preset::NorwaySpruce && from.y > -0.8 {
                continue;
            }
            let alignments: Vec<_> = children[i]
                .iter()
                .map(|&j| from.dot((nodes[j].position - node.position).normalized()))
                .collect();
            // A continuing axis plus a departing side axis, not a terminal fork
            // or foliage merely placed directly on an exposed scaffold.
            if alignments.iter().any(|&dot| dot > 0.95) && alignments.iter().any(|&dot| dot < 0.93)
            {
                mid_axis_forks += 1;
            }
        }
        assert!(
            mid_axis_forks > 30,
            "{preset:?}: only {mid_axis_forks} intermediate forks"
        );
    }
}
