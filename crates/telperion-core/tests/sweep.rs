//! The walk through tree space. Every pair of shipped presets in ten steps,
//! each step a family that validates and grows a tree with finite positions;
//! the oak-to-spruce walk with no frame where the tree changes kind; and every
//! wire parameter a preset moves proved to be walked at all. No device is
//! needed; this is the core's own arithmetic.
mod specimens;
use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;
use telperion_core::{
    blend, branching, foliage, mesh, params,
    presets::{Family, Preset},
};

const IDS: [&str; 7] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "telperion",
    "laurelin",
];
const SEED: u32 = 7;
/// Ten linear steps, endpoints included, which is what the spec walks.
const STEPS: usize = 10;
/// The sweep proves that every step of every walk validates and generates, not
/// that it does so at full size: every step alike is grown under one node cap,
/// which keeps the walk inside a test suite.
const SWEEP_NODES: usize = 8_000;

/// Wire paths every shipped row agrees on, so no pair of presets moves them and
/// the sweep cannot prove the walk carries them. Every other path in the wire is
/// moved by some pair below and proved to walk; a parameter added to the wire is
/// either moved by a preset or named here.
// fn-11 publishes the timeline before species calibration: rate, shape and
// shedding traits are still shared. Age now differs on the new species so the
// sweep walks it. Annual and thickening tests independently walk the rest.
// Leaf lifetime varies between oak and spruce, so the sweep itself checks
// that dimension among the moved paths.
// The chronicle adds resizeTolerance at the same 1e-9 metre default in every
// preset. params::tests::resize_tolerance_is_a_validated_blended_wire_trait
// checks its non-default walk; no preset geometry or identity pin changes.
// fn-37 turns the curtain into four twig rows. Hang, the pendulous length and
// the curtain's shoot separation differ between the oak, the spruce and the
// birch, so the sweep walks all three. The radius threshold is 1 on every
// shipped row - every shoot under a descending limb hangs - and is held here.
// fn-38 makes the clump three habit rows, and the silver birch is the table
// that declares one: two stems, their bearings and their lean. Every other
// table stands on one stem, so the sweep walks all three.
// fn-44 adds the sag, the fifth curtain row. The birch is the table that
// states one, so the oak-to-birch walk moves it and the sweep walks it.
// fn-45 makes the twig layer's depth a row. The beech states two generations
// and every other table leaves it at the top of its rail, so the sweep walks
// it and it is not held here.
// Round 6b gives the beech its own local departure angle, and round 6c its
// own two-ranked local divergence, so both leave this list.
// Round 11 moves the beech's twig-layer threshold, where the twig layer
// starts on the scaffold, so the sweep walks it and it leaves this list.
// fn-47 adds the pendulous variation, the sixth. The birch states one, so the
// sweep walks it.
// fn-51 adds the drop and the clearance, the seventh and eighth. The birch
// states both, so the sweep walks them.
// fn-48 adds the clump's lean spread, and the birch states the whole of it, so
// the sweep walks it. At the whole spread the birch's first stem stands
// upright and has no bearing to part from, so its divergence goes to none and
// every table agrees on it: the divergence is held here, and the clump tests
// walk it from nothing to a hundred degrees.
// fn-48.2 adds the clump's fork height, and the birch states half the bole,
// so the sweep walks it.
// fn-109's ten rosette rows and fn-110's eight trunk-organ rows are the date
// palm's, and the palm is not a preset the sweep walks: every other table
// leaves them at their neutral, so they are held here.
const HELD: [&str; 60] = [
    "/canopy/clumpSystemOrder",
    "/canopy/rosetteFronds",
    "/canopy/rosetteDivergence",
    "/canopy/rosettePitch",
    "/canopy/rosettePitchSpread",
    "/canopy/rosetteDepth",
    "/canopy/leafletCount",
    "/canopy/rachisLength",
    "/canopy/leafletPitch",
    "/canopy/rachisArch",
    "/canopy/terminalLeaflet",
    "/canopy/leafBases",
    "/canopy/leafBaseLength",
    "/canopy/leafBaseRadius",
    "/canopy/leafBasePitch",
    "/canopy/leafBaseWeathering",
    "/canopy/acanthophylls",
    "/canopy/acanthophyllLength",
    "/canopy/acanthophyllPitch",
    "/canopy/clumpNeighbours",
    "/surface/socketContainment",
    "/skeleton/habit/reachProbeSteps",
    "/skeleton/samplingAttemptsPerAttractor",
    "/growth/workBudget",
    "/radii/maxTaperExponent",
    "/skeleton/bias/supernatural/maxWritheMagnitude",
    "/skeleton/twigs/maxInternodes",
    "/skeleton/twigs/maxDroop",
    "/skeleton/twigs/curtainStepClearance",
    "/material/barkReflectance",
    "/material/leafReflectance",
    "/material/plateFurrowWidth",
    "/canopy/maxInstances",
    "/element/card",
    "/element/cup",
    "/element/curl",
    "/growth/apicalControlLoss",
    "/growth/sheddingTolerance",
    "/growth/rate",
    "/growth/resizeTolerance",
    "/growth/shape",
    "/skeleton/growth/influenceRadius",
    "/skeleton/growth/killDistance",
    "/skeleton/growth/maxNodes",
    "/skeleton/growth/stepDistance",
    "/skeleton/growth/trunkHeight",
    "/skeleton/habit/stemDivergence",
    "/skeleton/seed",
    "/skeleton/step",
    "/skeleton/twigs/angleVariation",
    "/skeleton/twigs/pendulousRadius",
    "/skeleton/twigs/ratioPower",
    "/skeleton/twigs/reach",
    "/skeleton/twigs/twig/stationsPerInternode",
    "/skeleton/twigs/vigourVariation",
    "/surface/flareDepth",
    "/surface/flareFalloff",
    "/surface/forkSocket",
    "/surface/forkSwell",
    "/surface/radialSegments",
];

fn family(id: &str) -> Family {
    let mut f = Preset::from_id(id).expect("preset identity").parameters();
    f.skeleton.seed = SEED;
    f
}

fn pairs() -> impl Iterator<Item = (&'static str, &'static str)> {
    IDS.iter()
        .enumerate()
        .flat_map(|(i, a)| IDS[i + 1..].iter().map(move |b| (*a, *b)))
}

/// The step's parameter, 0 at the first and 1 at the last.
fn at(step: usize) -> f64 {
    step as f64 / (STEPS - 1) as f64
}

fn walk(a: &str, b: &str, step: usize) -> Family {
    blend::families(&family(a), &family(b), at(step)).unwrap_or_else(|e| panic!("{a}->{b}: {e}"))
}

/// FNV-1a over the bytes, the pattern the identity pins already hash with.
fn fnv(bytes: impl IntoIterator<Item = u8>) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in bytes {
        hash = (hash ^ byte as u64).wrapping_mul(1099511628211);
    }
    hash
}

/// Every leaf of the wire object by its pointer path.
fn flat(value: &Value) -> BTreeMap<String, Value> {
    fn walk(value: &Value, at: &str, out: &mut BTreeMap<String, Value>) {
        match value {
            Value::Object(map) => {
                for (key, value) in map {
                    walk(value, &format!("{at}/{key}"), out);
                }
            }
            leaf => {
                out.insert(at.to_owned(), leaf.clone());
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(value, "", &mut out);
    out
}

#[test]
fn the_ends_of_a_walk_are_the_rows_themselves_and_a_row_walked_with_itself_holds() {
    for (a, b) in pairs() {
        let (from, to) = (family(a), family(b));
        for (t, end) in [(0.0, &from), (1.0, &to)] {
            assert_eq!(
                flat(&params::metadata(&blend::families(&from, &to, t).unwrap())),
                flat(&params::metadata(end)),
                "{a}->{b} at {t} is not the row itself"
            );
        }
    }
    for id in IDS {
        let row = family(id);
        assert_eq!(
            flat(&params::metadata(
                &blend::families(&row, &row, 0.37).unwrap()
            )),
            flat(&params::metadata(&row)),
            "{id} walked with itself moved"
        );
    }
    // A parameter outside the walk is not a point in the space.
    for t in [-0.001, 1.001, f64::NAN, f64::INFINITY] {
        assert_eq!(
            blend::families(&family("ordinary"), &family("telperion"), t).err(),
            Some(telperion_core::Error::InvalidInput("blend parameter")),
            "{t} was accepted"
        );
    }
}

#[test]
fn every_wire_parameter_a_preset_moves_is_walked() {
    let rows: Vec<(&str, BTreeMap<String, Value>)> = IDS
        .iter()
        .map(|id| (*id, flat(&params::metadata(&family(id)))))
        .collect();
    let (mut moved, mut still) = (BTreeSet::new(), BTreeSet::new());
    for (index, (a, from)) in rows.iter().enumerate() {
        for (b, to) in &rows[index + 1..] {
            let steps: Vec<_> = [0.25, 0.5, 0.75]
                .iter()
                .map(|t| {
                    flat(&params::metadata(
                        &blend::families(&family(a), &family(b), *t).unwrap(),
                    ))
                })
                .collect();
            for (path, value) in from {
                if to[path] == *value {
                    continue;
                }
                moved.insert(path.clone());
                if steps.iter().all(|step| step[path] == *value) {
                    still.insert(format!("{path} ({a}->{b})"));
                }
            }
        }
    }
    assert!(still.is_empty(), "the walk never moves {still:#?}");
    let held: BTreeSet<&str> = rows[0]
        .1
        .keys()
        .map(String::as_str)
        .filter(|path| !moved.contains(*path))
        .collect();
    assert_eq!(
        held,
        BTreeSet::from(HELD),
        "the parameters no preset moves have changed"
    );
}

#[test]
fn every_pair_of_presets_grows_a_tree_at_every_step() {
    for (a, b) in pairs() {
        for step in 0..STEPS {
            let mut family = walk(a, b, step);
            family.skeleton.growth.max_nodes = Some(SWEEP_NODES);
            let at = at(step);
            let mesh = mesh::build(&family).unwrap_or_else(|e| panic!("{a}->{b} at {at}: {e}"));
            assert!(
                mesh.wood.positions.iter().all(|v| v.is_finite())
                    && mesh.bounds.min.is_finite()
                    && mesh.bounds.max.is_finite(),
                "{a}->{b} at {at}: a position is not finite"
            );
            // A node cap this low truncates the tree before its twigs, and a
            // step that keeps no leaf under it still keeps its wood.
            assert!(
                mesh.wood_vertices() > 0,
                "{a}->{b} at {at}: nothing was built"
            );
        }
    }
}

#[test]
fn every_shipped_preset_carries_foliage() {
    for id in IDS {
        let mesh = specimens::mesh(&family(id));
        assert!(
            !mesh.foliage.instances.is_empty(),
            "{id}: crown carries no foliage"
        );
    }
}

#[test]
fn the_oak_to_birch_walk_ramps_the_curtain_up_from_nothing() {
    // The oak never hangs and the birch does, so this is the walk that crosses
    // the curtain. Hang is a row and not a switch: it rises through the walk,
    // every step grows a tree the step before it did not, and no step is the
    // frame where the curtain arrives.
    let (mut hangs, mut skeletons) = (Vec::new(), Vec::new());
    for step in 0..STEPS {
        let mut family = walk("oregon-white-oak", "silver-birch", step);
        family.skeleton.growth.max_nodes = Some(SWEEP_NODES);
        hangs.push(family.skeleton.twigs.hang);
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap_or_else(|e| panic!("step {step}: {e}"))
            .tree;
        skeletons.push(fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
        })));
    }
    assert_eq!(hangs[0], family("oregon-white-oak").skeleton.twigs.hang);
    assert_eq!(hangs[0], 0.0, "the oak hangs");
    assert_eq!(
        hangs[STEPS - 1],
        family("silver-birch").skeleton.twigs.hang,
        "the walk did not reach the birch's own row"
    );
    for step in 1..STEPS {
        assert!(
            hangs[step] > hangs[step - 1],
            "step {step} did not raise the hang row"
        );
        assert_ne!(
            skeletons[step - 1],
            skeletons[step],
            "step {step} grew the tree the step before it did"
        );
    }
}

#[test]
fn the_oak_to_spruce_walk_has_no_switch_frame() {
    let (mut skeletons, mut elements, mut sections) = (Vec::new(), Vec::new(), Vec::new());
    for step in 0..STEPS {
        let mut family = walk("oregon-white-oak", "norway-spruce", step);
        family.skeleton.growth.max_nodes = Some(SWEEP_NODES);
        let tree = branching::generate(&family.skeleton, family.radii)
            .unwrap_or_else(|e| panic!("step {step}: {e}"))
            .tree;
        skeletons.push(fnv(tree.nodes.iter().skip(1).flat_map(|n| {
            [n.position.x, n.position.y, n.position.z]
                .into_iter()
                .flat_map(f64::to_le_bytes)
        })));
        let element =
            foliage::build_element(family.element).unwrap_or_else(|e| panic!("step {step}: {e}"));
        elements.push(fnv(element
            .positions
            .iter()
            .flat_map(|p| [p.x, p.y, p.z].into_iter().flat_map(f64::to_le_bytes))
            .chain(element.indices.iter().flat_map(|i| i.to_le_bytes()))));
        sections.push(family.element.axial_segments);
    }
    for step in 1..STEPS {
        assert_ne!(
            skeletons[step - 1],
            skeletons[step],
            "step {step} grew the tree the step before it did"
        );
        assert_ne!(
            elements[step - 1],
            elements[step],
            "step {step} built the leaf the step before it did"
        );
    }
    // Sections are rounded up out of a linear walk, so one step moves them by
    // the walk's own stride and at most one more.
    let (from, to) = (
        family("oregon-white-oak").element.axial_segments,
        family("norway-spruce").element.axial_segments,
    );
    let stride = (f64::from(from.abs_diff(to)) / (STEPS - 1) as f64).ceil() as u32 + 1;
    for step in 1..STEPS {
        assert!(
            sections[step - 1].abs_diff(sections[step]) <= stride,
            "step {step}: sections jumped from {} to {}",
            sections[step - 1],
            sections[step]
        );
    }
}
