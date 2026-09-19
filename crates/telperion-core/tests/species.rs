#[path = "../examples/species_metrics/mod.rs"]
mod species_metrics;

use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Mutex,
    },
};

use serde_json::Value;
use telperion_core::{
    branching,
    foliage::{self, Instances, TwigPlacement},
    math::Vec3,
    presets::Preset,
    surface::{self, SurfaceMesh},
    tree::Tree,
};

/// The digest of every fixed seed of every species, committed beside this
/// file. A generator change that moves one updates the value in the same
/// commit, with the reason in the message.
fn digests() -> Value {
    serde_json::from_str(include_str!("species/digests.json")).unwrap()
}

/// FNV-1a over the skeleton, the wood and the placed foliage: what two
/// generations used to be compared on, as one number a run on another machine
/// can be compared on too.
fn digest(tree: &Tree, wood: &SurfaceMesh, placed: &Instances) -> u64 {
    let mut hash = 14695981039346656037_u64;
    let mut take = |bytes: &[u8]| {
        for byte in bytes {
            hash = (hash ^ u64::from(*byte)).wrapping_mul(1099511628211);
        }
    };
    take(&bincode::serialize(tree).unwrap());
    for floats in [&wood.positions, &wood.normals, &wood.coords] {
        for v in floats {
            take(&v.to_le_bytes());
        }
    }
    for i in &wood.indices {
        take(&i.to_le_bytes());
    }
    for m in &placed.matrices {
        for v in m {
            take(&v.to_le_bytes());
        }
    }
    hash
}

/// A seed whose digest is not the committed one, named with both values.
fn check(preset: Preset, seed: u32, expected: Option<&str>, actual: u64) -> Result<(), String> {
    let id = preset.profile_id().unwrap();
    let actual = format!("{actual:016x}");
    match expected {
        Some(expected) if expected == actual => Ok(()),
        Some(expected) => Err(format!(
            "{id} seed {seed}: digest {actual}, the committed digest is {expected}"
        )),
        None => Err(format!(
            "{id} seed {seed}: digest {actual}, no committed digest"
        )),
    }
}

fn profiles() -> Value {
    let mut root: Value =
        serde_json::from_str(include_str!("../../../.flow/evidence/fn9/profiles.json")).unwrap();
    let extra: Value =
        serde_json::from_str(include_str!("../../../.flow/evidence/fn34/profiles.json")).unwrap();
    let profiles = root["profiles"].as_array_mut().unwrap();
    for profile in extra["profiles"].as_array().unwrap() {
        profiles.push(profile.clone());
    }
    root
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
    // The oak is a row in the trait table: a leader that yields early, crooked
    // axes, no attractor pull.
    let habit = family.skeleton.habit;
    assert!(habit.apical_dominance < 0.25 && habit.crookedness > 12.0);
    assert_eq!(habit.attractor_weight, 0.0);
    assert!(!family.skeleton.bias.supernatural.enabled);
    assert!(family.element.lobe_count == 5 && family.element.lobe_depth > 0.5);
    assert_eq!(family.element.section_roundness, 0.0);
    // Blades lean a quarter of the radial along the shoot and sit clear of
    // the wood; nothing about them is a mode.
    assert_eq!(family.canopy.forward_lean, 0.25);
    assert_eq!(family.canopy.lean_rise, 0.0);
    assert_eq!(family.canopy.surface_contact, 0.0);
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

#[test]
fn beech_identity_resolves_to_frozen_profile_and_native_anatomy() {
    let preset = Preset::from_id("european-beech").unwrap();
    assert_eq!(preset, Preset::EuropeanBeech);
    assert_eq!(preset.profile_id(), Some("european-beech"));
    let manifest = profiles();
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    assert_eq!(profile["scientific_name"], "Fagus sylvatica");
    assert_eq!(profile["readiness"], "ready");
    let family = preset.parameters();
    assert_eq!(family.age, 120.0);
    assert!((family.skeleton.envelope.height - 32.0).abs() < 1e-9);
    // Round 5b (owner, 2026-09-15): the reference beech "grows relatively
    // straight up and out", its trunk running up through the crown. Round 6
    // (fn-45) lets the leader carry the crown to nine tenths of the height,
    // which is what the scaffold's apical dominance measures; the bound moves
    // from 0.6 to 0.95, which still refuses the spruce's excurrent 1.0 - a
    // leader that never yields to its limbs at all.
    assert!(family.skeleton.habit.apical_dominance < 0.95);
    assert!(family.skeleton.habit.crookedness < 16.0);
    assert_eq!(family.skeleton.habit.attractor_weight, 0.0);
    assert!(!family.skeleton.bias.supernatural.enabled);
    assert_eq!(family.element.lobe_count, 0);
    assert_eq!(family.element.section_roundness, 0.0);
    assert_eq!(family.canopy.divergence, 180.0);
    assert!(Preset::from_id("Fagus sylvatica").is_none());
    assert!(Preset::from_id("european-ash").is_none());
}

#[test]
fn birch_identity_resolves_to_frozen_profile_and_native_anatomy() {
    let preset = Preset::from_id("silver-birch").unwrap();
    assert_eq!(preset, Preset::SilverBirch);
    assert_eq!(preset.profile_id(), Some("silver-birch"));
    let manifest = profiles();
    let profile = manifest["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["id"] == preset.profile_id().unwrap())
        .unwrap();
    assert_eq!(profile["scientific_name"], "Betula pendula");
    assert_eq!(profile["readiness"], "ready");
    let family = preset.parameters();
    assert_eq!(family.age, 70.0);
    assert!((family.skeleton.envelope.height - 18.0).abs() < 1e-9);
    assert!(family.skeleton.habit.rise_secondary < 0.0);
    assert_eq!(family.skeleton.habit.attractor_weight, 0.0);
    assert!(!family.skeleton.bias.supernatural.enabled);
    // A serrate margin, drawn as shallow notches, not lobes, under a pointed tip.
    assert_eq!(family.element.lobe_count, 8);
    assert!(family.element.lobe_depth > 0.0 && family.element.lobe_depth <= 0.15);
    assert!(family.element.tip_sharpness > 1.0);
    assert_eq!(family.canopy.divergence, 180.0);
    assert!(Preset::from_id("Betula pendula").is_none());
}

#[test]
fn fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens() {
    fixed_species(Preset::EuropeanBeech);
}

#[test]
fn fixed_birches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens() {
    fixed_species(Preset::SilverBirch);
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
    let expected = digests();
    // Every seed's checks run on a thread of their own, four at a time, and
    // are read back in seed order: a failure is the first seed that failed,
    // however the threads finished.
    let grown = in_flight(&seeds, |seed| {
        let committed = expected[preset.profile_id().unwrap()][seed.to_string()].as_str();
        grow_and_check(preset, profile, seed, committed)
    });
    let mut moved = Vec::new();
    for outcome in grown {
        let grown = outcome.unwrap_or_else(|message| panic!("{message}"));
        heights.push(grown.height);
        widths.push(grown.width);
        leaf_counts.insert(grown.leaves);
        if let Err(report) = grown.digest {
            moved.push(report);
        }
    }
    // Engineering regression thresholds for specimen variation, not botanical ranges.
    let dimensions = match preset {
        Preset::NorwaySpruce => {
            // A persistent leader reaches the authored height on every seed;
            // azimuth, curtains and crown width are what vary.
            vec![widths]
        }
        Preset::EuropeanBeech | Preset::SilverBirch => {
            // A full crown fills its envelope on every seed, so neither
            // dimension is pinned to vary; whichever of height or plan width
            // the seeds move more is the one judged. fn-37 gave the birch the
            // same property the beech already had: its curtain reaches the
            // shell on every seed.
            let range = |values: &Vec<f64>| {
                values.iter().copied().fold(f64::NEG_INFINITY, f64::max)
                    - values.iter().copied().fold(f64::INFINITY, f64::min)
            };
            if range(&heights) >= range(&widths) {
                vec![heights]
            } else {
                vec![widths]
            }
        }
        _ => vec![heights, widths],
    };
    // A specimen regression, not a botanical range, and scale-free like every
    // other length the library states: the moving dimension has to move by
    // more than a thirtieth of the tree's authored height. On the beech's 32 m
    // envelope that is the metre this rule asked for before fn-37 stated it as
    // a fraction; on the birch's 18 m one it is a little over half of it.
    let apart = match preset {
        Preset::NorwaySpruce => 0.1,
        _ => preset.parameters().skeleton.envelope.height / 30.0,
    };
    for values in dimensions {
        let min = values.iter().copied().fold(f64::INFINITY, f64::min);
        let max = values.iter().copied().fold(f64::NEG_INFINITY, f64::max);
        assert!(
            max - min > apart,
            "crown dimensions should vary by more than {apart} m across specimens: {values:?}"
        );
    }
    assert!(
        leaf_counts.len() >= 6,
        "seeds must change retained foliage abundance"
    );
    assert!(moved.is_empty(), "digests moved:\n{}", moved.join("\n"));
}

/// What one fixed seed contributes to the specimen-variation checks.
struct Grown {
    height: f64,
    width: f64,
    leaves: usize,
    digest: Result<(), String>,
}

/// Grows one fixed seed and holds it to every gate.
fn grow_and_check(preset: Preset, profile: &Value, seed: u32, committed: Option<&str>) -> Grown {
    let mut family = preset.parameters();
    family.skeleton.seed = seed;
    let a = branching::generate(&family.skeleton, family.radii).unwrap();
    a.tree.validate_solved().unwrap();
    if preset == Preset::NorwaySpruce {
        let structural = &a.tree.nodes[..a.tree.crossover];
        let axial = |p: Vec3| p.x.hypot(p.z) < 1e-10;
        let mut tiers = std::collections::BTreeMap::<u32, usize>::new();
        let mut hanging = 0;
        let mut upturned = 0;
        assert!(structural
            .iter()
            .any(|n| axial(n.position)
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
    // Inside the shell, or a hanging shoot in the band below it.
    let (envelope, twigs) = (family.skeleton.envelope, family.skeleton.twigs);
    for node in a.tree.nodes.iter().skip(a.tree.crossover) {
        let p = node.position;
        assert!(
            envelope.contains(p, 1e-8, seed)
                || branching::in_curtain_band(&envelope, &twigs, seed, p, 1e-8),
            "seed {seed}: {p:?} is outside the shell and the curtain's band"
        );
    }
    let wood = surface::build(&a.tree, family.skeleton.envelope.height, &family.surface).unwrap();
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
    let placed = foliage::place(
        &a.tree,
        family.skeleton.envelope,
        seed,
        family.canopy,
        Some(TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        }),
    )
    .unwrap();
    let digest = check(preset, seed, committed, digest(&a.tree, &wood, &placed));
    let placed_count = placed.matrices.len();
    let kept = foliage::cull(
        placed,
        &element,
        family.skeleton.envelope,
        family.shell_depth,
    )
    .unwrap();
    assert!(!kept.matrices.is_empty());
    let metrics =
        species_metrics::measure(&a.tree, &wood.positions, &element, placed_count, &kept).unwrap();
    let (pass, checks) = species_metrics::compare(profile, &metrics).unwrap();
    assert!(pass, "seed {seed}: {checks:#}");
    assert_eq!(metrics["units_per_instance"]["value"], 1);
    if preset == Preset::NorwaySpruce {
        assert_eq!(metrics["foliage_unit"], "needle");
        assert_eq!(metrics["foliage_units"]["value"], kept.matrices.len());
        assert!(metrics["needle_surface_area_m2"]["value"].as_f64().unwrap() > 0.0);
        assert!(metrics["crown_base_m"]["value"].as_f64().unwrap() < 3.0);
    }
    Grown {
        height: metrics["height_m"]["value"].as_f64().unwrap(),
        width: metrics["crown_width_m"]["value"].as_f64().unwrap(),
        leaves: kept.matrices.len(),
        digest,
    }
}

/// Seeds in flight at once: the runner's four cores, and a spruce placement is
/// about 350 MB.
const SEEDS_IN_FLIGHT: usize = 4;

/// Runs `check` on every seed, `SEEDS_IN_FLIGHT` at a time, and returns the
/// outcomes in seed order; a seed whose checks panicked carries the message.
fn in_flight<T: Send>(seeds: &[u32], check: impl Fn(u32) -> T + Sync) -> Vec<Result<T, String>> {
    let next = AtomicUsize::new(0);
    let done = Mutex::new(Vec::new());
    std::thread::scope(|scope| {
        for _ in 0..SEEDS_IN_FLIGHT.min(seeds.len()) {
            scope.spawn(|| loop {
                let i = next.fetch_add(1, Ordering::Relaxed);
                let Some(&seed) = seeds.get(i) else { break };
                let outcome = catch_unwind(AssertUnwindSafe(|| check(seed))).map_err(message);
                done.lock().unwrap().push((i, outcome));
            });
        }
    });
    let mut done = done.into_inner().unwrap();
    done.sort_by_key(|(i, _)| *i);
    done.into_iter().map(|(_, outcome)| outcome).collect()
}

fn message(payload: Box<dyn Any + Send>) -> String {
    payload
        .downcast_ref::<String>()
        .cloned()
        .or_else(|| payload.downcast_ref::<&str>().map(|s| (*s).to_owned()))
        .unwrap_or_else(|| "a seed's checks panicked".to_owned())
}

#[test]
fn a_digest_that_moved_names_the_preset_the_seed_and_both_digests() {
    assert_eq!(
        check(Preset::SilverBirch, 3, Some("00000000000000ff"), 255),
        Ok(())
    );
    assert_eq!(
        check(Preset::SilverBirch, 3, Some("00000000000000fe"), 255),
        Err("silver-birch seed 3: digest 00000000000000ff, the committed digest is 00000000000000fe".into())
    );
    assert_eq!(
        check(Preset::NorwaySpruce, 4_250_668_600, None, 255),
        Err("norway-spruce seed 4250668600: digest 00000000000000ff, no committed digest".into())
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
    // The spruce is the opposite row: one leader all the way up, whorled
    // stations, hanging secondaries.
    let habit = family.skeleton.habit;
    assert_eq!(habit.apical_dominance, 1.0);
    assert!(habit.whorl_strength > 0.8 && habit.rise_secondary < -0.5);
    assert!(!family.skeleton.bias.supernatural.enabled);
    assert_eq!(family.element.section_roundness, 1.0);
    assert_eq!(family.element.lobe_count, 0);
    assert!(family.element.connector_length > 0.0);
    // Needles are seated on the wood itself and the upper ones lean hardest
    // toward the tip.
    assert_eq!(family.canopy.surface_contact, 1.0);
    assert_eq!(family.canopy.lean_rise, 1.2);
    assert!(family.canopy.forward_lean > 0.0 && family.canopy.shoot_radius > 0.0);
    assert_eq!(family.skeleton.twigs.twig.stations_per_internode, 1);
    assert!(Preset::from_id("Picea abies").is_none());
}

#[test]
fn scaffold_reaches_and_hanging_secondaries_subdivide_before_their_tips() {
    for preset in [
        Preset::OregonWhiteOak,
        Preset::NorwaySpruce,
        Preset::EuropeanBeech,
        Preset::SilverBirch,
    ] {
        let family = preset.parameters();
        let report = branching::generate(&family.skeleton, family.radii).unwrap();
        let twigs: Vec<_> = report
            .tree
            .nodes
            .iter()
            .filter(|n| n.kind == telperion_core::tree::NodeKind::Twig)
            .collect();
        assert!(!twigs.is_empty());
        assert!(
            twigs.iter().all(|n| n.radius < n.start_radius * 0.5),
            "species twig ends must narrow"
        );
        if preset == Preset::NorwaySpruce {
            let all = &report.tree.nodes;
            let mut pendant = vec![false; all.len()];
            let mut descendants = 0;
            for (i, n) in all.iter().enumerate().skip(1) {
                let parent = n.parent.unwrap() as usize;
                let direction = (n.position - all[parent].position).normalized();
                pendant[i] = if i < report.tree.crossover {
                    direction.y < -0.5
                } else {
                    pendant[parent]
                };
                if i >= report.tree.crossover && pendant[i] {
                    assert!(direction.y < 0.0, "pendant descendant {i} turned upward");
                    descendants += 1;
                }
            }
            assert!(descendants > 100);
        }
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
