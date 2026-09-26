//! The pipeline's own tests: one sweep and one element a request, the same
//! bytes under either schedule, and the earliest failing stage's error.
use super::*;
use crate::{
    pipeline::surface,
    presets::{Preset, CATALOGUE, IN_WORK},
    Error,
};
use std::cell::Cell;

/// The shared work a request ran, tallied on the thread that called it.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub(super) struct Tally {
    pub(super) sweeps: u32,
    pub(super) elements: u32,
    /// Whether a stage ran on a thread of its own.
    pub(super) split: bool,
}
thread_local!(static TALLY: Cell<Tally> = Cell::default());

pub(super) fn count(f: impl FnOnce(&mut Tally)) {
    TALLY.with(|t| {
        let mut tally = t.get();
        f(&mut tally);
        t.set(tally);
    });
}

/// `outputs`, and what it tallied.
fn tallied(tree: &Tree, family: &Family, request: Request) -> (Outputs, Tally) {
    TALLY.take();
    let outputs = outputs(tree, &Inputs::of(family), request).unwrap();
    (outputs, TALLY.take())
}

fn ordinary() -> Family {
    Preset::from_id("ordinary").unwrap().parameters()
}

fn request(wood: bool, leaves: bool, field: bool) -> Request {
    Request {
        wood,
        leaves,
        field: field.then_some(None),
        ..Request::default()
    }
}

/// A family whose leaves sit on the wood sweeps its rings once a request:
/// the wood's where wood is asked for, which the leaves read in place, else
/// the leaves' own. Every request builds its element at most once, and the
/// wood is the wood a plain build makes.
#[test]
fn one_sweep_and_one_element_serve_a_request() {
    let mut family = ordinary();
    family.canopy.surface_contact = 1.0;
    let skeleton = skeleton(GrowInput::of(&family)).unwrap();
    let tree = &skeleton.tree;
    let own = surface::build(tree, family.skeleton.envelope.height, &family.surface).unwrap();
    for schedule in [Schedule::Serial, Schedule::Concurrent] {
        let count = |r: Request| {
            let t = tallied(tree, &family, Request { schedule, ..r }).1;
            (t.sweeps, t.elements)
        };
        assert_eq!(count(request(true, true, false)), (1, 1));
        assert_eq!(count(request(true, true, true)), (1, 1));
        assert_eq!(count(request(true, false, false)), (1, 0));
        assert_eq!(count(request(false, true, false)), (1, 1));
        assert_eq!(count(request(false, false, true)), (0, 1));
        let request = Request {
            schedule,
            ..Request::mesh()
        };
        let seated = tallied(tree, &family, request).0;
        assert_eq!(seated.wood.unwrap(), own, "{schedule:?}");
    }
    family.canopy.surface_contact = 0.0;
    let t = tallied(tree, &family, Request::mesh()).1;
    assert_eq!((t.sweeps, t.elements), (1, 1));
}

/// Every shipped and in-work family, its leaves seated on the wood or not,
/// builds the same bytes whether wood and leaves run side by side or one
/// after the other, and side by side is what a native target with threads
/// does.
#[test]
fn every_family_builds_the_same_bytes_under_either_schedule() {
    for &(_, id, _, _) in CATALOGUE.iter().chain(IN_WORK) {
        let family = crate::presets::by_identity(id)
            .unwrap_or_else(|_| Preset::from_id(id).unwrap().parameters());
        let tree = skeleton(GrowInput::of(&family)).unwrap().tree;
        let run = |schedule| {
            let request = Request {
                schedule,
                ..Request::mesh()
            };
            tallied(&tree, &family, request)
        };
        let ((serial, apart), (concurrent, split)) =
            (run(Schedule::Serial), run(Schedule::Concurrent));
        assert!(!apart.split, "{id}");
        assert!(split.split, "{id}");
        assert_eq!(serial.wood, concurrent.wood, "{id}: wood");
        assert_eq!(serial.element, concurrent.element, "{id}: element");
        let leaves = |o: &Outputs| {
            let l = o.leaves.as_ref().unwrap();
            (l.instances.clone(), l.placed, l.retained, l.bounds)
        };
        assert_eq!(leaves(&serial), leaves(&concurrent), "{id}: leaves");
    }
}

/// With the wood and the leaves both failing, either schedule answers
/// with the wood's error, the earlier stage; the leaves alone answer with
/// their own.
#[test]
fn the_earliest_failing_stage_answers() {
    let mut family = ordinary();
    family.surface.radial_segments = 0;
    family.shell_depth = 2.0;
    let tree = skeleton(GrowInput::of(&family)).unwrap().tree;
    for schedule in [Schedule::Serial, Schedule::Concurrent] {
        let request = Request {
            schedule,
            ..Request::mesh()
        };
        let error = outputs(&tree, &Inputs::of(&family), request).err();
        assert_eq!(error, Some(Error::InvalidInput("surface parameters")));
    }
    let error = outputs(&tree, &Inputs::of(&family), request(false, true, false)).err();
    assert_eq!(error, Some(Error::InvalidInput("shell depth")));
}

/// The wood fails before any of the Plan stage, the twig rows and the
/// element among it, seated or not, as the build always ran them.
#[test]
fn the_wood_fails_before_the_plan() {
    let mut family = ordinary();
    let tree = skeleton(GrowInput::of(&family)).unwrap().tree;
    family.skeleton.twigs.twig.diameter = -1.0;
    family.element.axial_segments = 0;
    family.surface.radial_segments = 0;
    for contact in [0.0, 1.0] {
        family.canopy.surface_contact = contact;
        for schedule in [Schedule::Serial, Schedule::Concurrent] {
            let request = Request {
                schedule,
                ..Request::mesh()
            };
            let error = outputs(&tree, &Inputs::of(&family), request).err();
            let surface = Some(Error::InvalidInput("surface parameters"));
            assert_eq!(error, surface, "{contact} {schedule:?}");
        }
    }
}

/// Within the Plan stage the element fails before the twig rows.
#[test]
fn the_element_fails_before_the_twig_rows() {
    let mut family = ordinary();
    let tree = skeleton(GrowInput::of(&family)).unwrap().tree;
    family.skeleton.twigs.twig.diameter = -1.0;
    family.element.axial_segments = 0;
    let error = outputs(&tree, &Inputs::of(&family), request(false, true, false)).err();
    assert_eq!(error, Some(Error::InvalidInput("leaf segments")));
}

/// Leaves that sweep their own rings, with no wood to share them, fail
/// after the Plan stage and as the leaves.
#[test]
fn the_leaves_own_rings_fail_after_the_plan() {
    let mut family = ordinary();
    let tree = skeleton(GrowInput::of(&family)).unwrap().tree;
    family.canopy.surface_contact = 1.0;
    family.surface.radial_segments = 0;
    let leaves = request(false, true, false);
    let surface = Some(Error::InvalidInput("surface parameters"));
    assert_eq!(outputs(&tree, &Inputs::of(&family), leaves).err(), surface);
    family.element.axial_segments = 0;
    let element = Some(Error::InvalidInput("leaf segments"));
    assert_eq!(outputs(&tree, &Inputs::of(&family), leaves).err(), element);
}

/// Every shipped preset builds every artifact kind through the pipeline, one
/// request each: the skeleton, the wood surface, the leaves, the field and
/// the structure (docs/principles.md). A failure names the preset and the
/// artifact. The package entries are held by the binding tests.
#[test]
fn every_shipped_preset_builds_every_artifact_through_the_pipeline() {
    for &(_, id, _, _) in CATALOGUE {
        let family = crate::presets::by_identity(id).unwrap();
        let tree = skeleton(GrowInput::of(&family))
            .unwrap_or_else(|e| panic!("preset {id}: skeleton: {e}"))
            .tree;
        assert!(tree.nodes.len() > 1, "preset {id}: skeleton: no wood");
        let kinds: [(&str, Request); 4] = [
            (
                "surface",
                Request {
                    wood: true,
                    ..Request::default()
                },
            ),
            (
                "leaves",
                Request {
                    leaves: true,
                    ..Request::default()
                },
            ),
            (
                "field",
                Request {
                    field: Some(None),
                    ..Request::default()
                },
            ),
            (
                "structure",
                Request {
                    structure: true,
                    ..Request::default()
                },
            ),
        ];
        for (kind, request) in kinds {
            let out = outputs(&tree, &Inputs::of(&family), request)
                .unwrap_or_else(|e| panic!("preset {id}: {kind}: {e}"));
            let built = match kind {
                "surface" => out.wood.is_some_and(|w| !w.indices.is_empty()),
                "leaves" => out.leaves.is_some_and(|l| l.retained > 0),
                "field" => out.field.is_some(),
                _ => out.structure.is_some_and(|s| !s.nodes.is_empty()),
            };
            assert!(built, "preset {id}: {kind}: nothing built");
        }
    }
}
