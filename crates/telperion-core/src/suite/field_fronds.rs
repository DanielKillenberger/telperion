//! The date palm's field from the leaf plan (fn-150): one oriented box a
//! leaflet, living and dead, drawn as placement draws it. Conservative
//! against the leaves placement hangs, exact in its counts over the grid,
//! and held to the real leaflet shapes, each placed leaf's own oriented box.
mod oriented;
use super::field_plan::{grid, holding};
use telperion_core::{
    field::{self, Frame},
    foliage::transform_point,
    pipeline::{self, FieldRequest, Request},
    presets::{Family, Preset},
};

const SEEDS: [u32; 4] = [1, 7, 1407, 4242];

fn palm(seed: u32) -> Family {
    let mut family = Preset::from_id("date-palm").unwrap().parameters();
    family.skeleton.seed = seed;
    family
}

/// One palm's build with its leaves, its plan and its field.
fn built(seed: u32) -> pipeline::Built {
    let request = Request {
        leaves: true,
        field: Some(FieldRequest {
            n: 24,
            limb_order: None,
        }),
        ..Request::default()
    };
    pipeline::build(&palm(seed), request).unwrap()
}

/// A field-only build reads the plan and places no leaf for it.
#[test]
fn the_palm_field_places_nothing() {
    let request = Request {
        field: Some(FieldRequest {
            n: 24,
            limb_order: None,
        }),
        ..Request::default()
    };
    let built = pipeline::build(&palm(1), request).unwrap();
    assert!(built.outputs.field.is_some());
    assert!(built.outputs.leaves.is_none());
}

/// Every vertex of every retained leaflet, living and withered, lies in a
/// cell the plan reports as foliage, on grids of 24, 48 and 96 cells a
/// side, at every seed the owner tried.
#[test]
fn every_placed_leaflet_vertex_lies_in_a_planned_foliage_cell() {
    for seed in SEEDS {
        let built = built(seed);
        let o = &built.outputs;
        let (tree, plan) = (&built.skeleton.tree, o.plan.as_ref().unwrap());
        let element = o.element.as_ref().unwrap();
        let leaves = &o.leaves.as_ref().unwrap().instances;
        for n in [24, 48, 96] {
            let g = grid(tree, plan, n);
            let (mut missed, mut all) = (0, 0);
            for m in leaves.matrices() {
                for v in &element.positions {
                    all += 1;
                    missed += usize::from(!holding(&g, transform_point(&m, *v)).foliage);
                }
            }
            assert_eq!(missed, 0, "seed {seed} at {n}: {missed} of {all}");
        }
    }
}

/// Over the grid the count estimates sum to the plan's total, which is
/// every leaflet placement hangs before the cull.
#[test]
fn the_palm_estimates_sum_to_every_leaflet() {
    let built = built(7);
    let o = &built.outputs;
    let total = f64::from(o.plan.as_ref().unwrap().total);
    assert_eq!(total, o.leaves.as_ref().unwrap().placed as f64);
    let sum: f64 = o.field.as_ref().unwrap().cells.iter().map(|c| c.leaves).sum();
    assert!((sum - total).abs() <= total * 0.01, "{sum} against {total}");
}

/// The planned crown against the real leaflet shapes on the same grid of
/// about a quarter metre (host target, 2026-09-25): at least 99 % of cells
/// agree and the plan reports at most 1.05 times the leaflets' foliage cells.
#[test]
fn the_planned_palm_agrees_with_its_leaflets() {
    for seed in SEEDS {
        let built = built(seed);
        let o = &built.outputs;
        let (tree, plan) = (&built.skeleton.tree, o.plan.as_ref().unwrap());
        let leaves = &o.leaves.as_ref().unwrap().instances;
        let exact = oriented::leaflets(leaves, o.element.as_ref().unwrap());
        let b = field::bounds(tree, plan).unwrap().unwrap();
        let size = b.max - b.min;
        let n = (size.x.max(size.y).max(size.z) / 0.25).ceil() as u32 + 2;
        let frame = Frame::over(b, n).unwrap();
        let planned = field::read(tree, plan, frame).unwrap();
        let exact = field::read(tree, &exact, frame).unwrap();
        let [mut agree, mut kept, mut ours] = [0usize; 3];
        for (p, e) in planned.cells.iter().zip(&exact.cells) {
            agree += usize::from(p.foliage == e.foliage);
            kept += usize::from(e.foliage);
            ours += usize::from(p.foliage);
        }
        let share = agree as f64 / planned.cells.len() as f64;
        let ratio = ours as f64 / kept as f64;
        assert!(share >= 0.99, "seed {seed}: {share}");
        assert!(ratio <= 1.05, "seed {seed}: {ratio}");
    }
}
