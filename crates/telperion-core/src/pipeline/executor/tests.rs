//! The executor interface and request subsets against the one build, over a
//! small table of families: leaves free of the wood and seated on it, and an
//! optional growth override stated and left unset.
use super::*;
use crate::{
    pipeline::{build, Outputs, Request},
    presets::Preset,
};

fn cases() -> Vec<(&'static str, Family)> {
    let base = Preset::Ordinary.parameters();
    let mut free = base.clone();
    free.canopy.surface_contact = 0.0;
    let mut seated = base.clone();
    seated.canopy.surface_contact = 1.0;
    let mut stated = base.clone();
    stated.skeleton.growth.max_turn_per_step = Some(20.0);
    let mut unset = base;
    unset.skeleton.growth.max_turn_per_step = None;
    vec![
        ("free", free),
        ("seated", seated),
        ("stated", stated),
        ("unset", unset),
    ]
}

fn field(o: &Outputs) -> Option<(Vec<f64>, Vec<f64>, Vec<u32>)> {
    let s = o.field.as_ref()?.snapshot().unwrap();
    Some((s.wood, s.plan, s.plan_stations))
}

/// The expansion of a grown tree, and of the same tree handed back, is the
/// build's own: its tree, its wood and its mesh.
#[test]
fn the_executor_expands_what_the_pipeline_builds() {
    for (name, f) in cases() {
        let built = build(&f, Request::mesh()).unwrap();
        let x = grow(&f).unwrap().expansion().unwrap();
        assert_eq!(x.tree(), &built.skeleton.tree, "{name}: tree");
        let wood = built.outputs.wood.unwrap();
        assert_eq!(x.wood().unwrap(), wood, "{name}: wood");
        let leaves = built.outputs.leaves.unwrap().instances;
        let mesh = x.mesh().unwrap();
        assert_eq!(mesh.wood, wood, "{name}: mesh wood");
        assert_eq!(mesh.foliage.instances, leaves, "{name}: mesh leaves");
        assert_eq!(
            x.element(),
            &built.outputs.element.unwrap(),
            "{name}: element"
        );
        let handed = expand(x.tree().clone(), &f).unwrap().mesh().unwrap();
        assert_eq!(handed.foliage.instances, leaves, "{name}: handed tree");
        assert_eq!(x.reference(), leaves.reference, "{name}: leaf box");
    }
}

/// Each output asked for alone is the output the whole request builds.
#[test]
fn a_request_subset_builds_what_the_whole_request_builds() {
    let whole = Request {
        wood: true,
        leaves: true,
        field: Some(None),
        structure: true,
        ..Request::default()
    };
    for (name, f) in cases() {
        let all = build(&f, whole).unwrap().outputs;
        let alone = |r: Request| build(&f, r).unwrap().outputs;
        let wood = alone(Request {
            wood: true,
            ..Request::default()
        });
        assert_eq!(wood.wood, all.wood, "{name}: wood");
        let leaves = alone(Request {
            leaves: true,
            ..Request::default()
        });
        let instances = |o: &Outputs| o.leaves.as_ref().map(|l| l.instances.clone());
        assert_eq!(instances(&leaves), instances(&all), "{name}: leaves");
        let planned = alone(Request {
            field: Some(None),
            ..Request::default()
        });
        assert_eq!(field(&planned), field(&all), "{name}: field");
        let structure = alone(Request {
            structure: true,
            ..Request::default()
        });
        let records = |o: &Outputs| {
            o.structure
                .as_ref()
                .map(|s| (s.nodes.clone(), s.topology.clone()))
        };
        assert_eq!(records(&structure), records(&all), "{name}: structure");
    }
}
