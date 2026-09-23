//! The pipeline's own tests: one sweep and one element a request, the same
//! bytes under either schedule, and the earliest failing stage's error.
use super::*;
use crate::{
    presets::{Preset, CATALOGUE},
    surface, Error,
};

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

/// R7: a family whose leaves sit on the wood sweeps its rings once for wood
/// and leaves together, under either schedule, and the wood built from them
/// is the wood a sweep of its own builds; every request builds its element
/// at most once.
#[test]
fn one_sweep_and_one_element_serve_a_request() {
    let mut family = ordinary();
    family.canopy.surface_contact = 1.0;
    let skeleton = skeleton(&family).unwrap();
    let tree = &skeleton.tree;
    let own = surface::build(tree, family.skeleton.envelope.height, &family.surface).unwrap();
    for schedule in [Schedule::Serial, Schedule::Concurrent] {
        let count = |r: Request| {
            let o = outputs(tree, &family, Request { schedule, ..r }).unwrap();
            (o.stages.sweeps, o.stages.elements)
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
        let shared = outputs(tree, &family, request).unwrap().wood.unwrap();
        assert_eq!(shared, own, "{schedule:?}");
    }
    family.canopy.surface_contact = 0.0;
    let o = outputs(tree, &family, Request::mesh()).unwrap();
    assert_eq!((o.stages.sweeps, o.stages.elements), (1, 1));
}

/// R8: every shipped family builds the same bytes whether wood and leaves
/// run side by side or one after the other, and side by side is what a
/// native target with threads does.
#[test]
fn every_family_builds_the_same_bytes_under_either_schedule() {
    for &(_, id, _, _) in CATALOGUE {
        let family = crate::presets::by_identity(id).unwrap();
        let tree = skeleton(&family).unwrap().tree;
        let run = |schedule| {
            let request = Request {
                schedule,
                ..Request::mesh()
            };
            outputs(&tree, &family, request).unwrap_or_else(|e| panic!("{id}: {e}"))
        };
        let (serial, concurrent) = (run(Schedule::Serial), run(Schedule::Concurrent));
        assert!(!serial.stages.concurrent, "{id}");
        assert!(
            concurrent.stages.concurrent,
            "{id}: wood and leaves ran in turn"
        );
        assert_eq!(serial.wood, concurrent.wood, "{id}: wood");
        assert_eq!(serial.element, concurrent.element, "{id}: element");
        let leaves = |o: &Outputs| {
            let l = o.leaves.as_ref().unwrap();
            (l.instances.clone(), l.placed, l.retained, l.bounds)
        };
        assert_eq!(leaves(&serial), leaves(&concurrent), "{id}: leaves");
    }
}

/// R8: with the wood and the leaves both failing, either schedule answers
/// with the wood's error, the earlier stage; the leaves alone answer with
/// their own.
#[test]
fn the_earliest_failing_stage_answers() {
    let mut family = ordinary();
    family.surface.radial_segments = 0;
    family.shell_depth = 2.0;
    let tree = skeleton(&family).unwrap().tree;
    for schedule in [Schedule::Serial, Schedule::Concurrent] {
        let request = Request {
            schedule,
            ..Request::mesh()
        };
        let error = outputs(&tree, &family, request).err();
        assert_eq!(error, Some(Error::InvalidInput("surface parameters")));
    }
    let error = outputs(&tree, &family, request(false, true, false)).err();
    assert_eq!(error, Some(Error::InvalidInput("shell depth")));
    // Leaves seated on the wood: a failing sweep and a failing element
    // answer with the element's error, the Plan's, under either schedule.
    family.canopy.surface_contact = 1.0;
    family.element.axial_segments = 0;
    for schedule in [Schedule::Serial, Schedule::Concurrent] {
        let request = Request {
            schedule,
            ..Request::mesh()
        };
        let error = outputs(&tree, &family, request).err();
        assert_eq!(
            error,
            Some(Error::InvalidInput("leaf segments")),
            "{schedule:?}"
        );
    }
}
