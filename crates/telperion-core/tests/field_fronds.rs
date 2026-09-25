//! The date palm's field from the leaf plan (fn-150): every frond, living and
//! dead, walked in chords along its rachis. Conservative against the leaves
//! placement hangs, exact in its counts over a grid, and measured against the
//! field read from the placed leaves.
use std::collections::HashSet;
use telperion_core::{
    field::Field,
    foliage::transform_point,
    math::Vec3,
    pipeline::{self, Request},
    presets::{Family, Preset},
};

const SEEDS: [u32; 4] = [1, 7, 1407, 4242];

fn palm(seed: u32) -> Family {
    let mut family = Preset::from_id("date-palm").unwrap().parameters();
    family.skeleton.seed = seed;
    family
}

/// One palm's build with its leaves and its planned field, and the field read
/// from those placed leaves.
fn fields(seed: u32) -> (pipeline::Built, Field) {
    let request = Request {
        leaves: true,
        field: Some(None),
        ..Request::default()
    };
    let built = pipeline::build(&palm(seed), request).unwrap();
    let o = &built.outputs;
    let leaves = &o.leaves.as_ref().unwrap().instances;
    let element = o.element.as_ref().unwrap();
    let placed = Field::new(&built.skeleton.tree, Some((leaves, element))).unwrap();
    (built, placed)
}

/// Cell centres of a grid of `cell` metres over `field`'s bounds.
fn grid(field: &Field, cell: f64) -> Vec<Vec3> {
    let b = field.bounds().unwrap();
    let n = |lo: f64, hi: f64| ((hi - lo) / cell).ceil() as usize + 1;
    let (nx, ny, nz) = (
        n(b.min.x, b.max.x),
        n(b.min.y, b.max.y),
        n(b.min.z, b.max.z),
    );
    let mut out = Vec::with_capacity(nx * ny * nz);
    for i in 0..nx {
        for j in 0..ny {
            for k in 0..nz {
                let at = Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5);
                out.push(b.min + at * cell);
            }
        }
    }
    out
}

/// A field-only build answers from the plan and places no leaf for it.
#[test]
fn the_palm_field_is_planned_and_places_nothing() {
    let request = Request {
        field: Some(None),
        ..Request::default()
    };
    let built = pipeline::build(&palm(1), request).unwrap();
    assert!(built.outputs.field.unwrap().is_planned());
    assert!(built.outputs.leaves.is_none());
}

/// Every vertex of every retained leaflet, living and withered, lies in a
/// cell the plan reports as foliage, at a decimetre, a quarter metre and a
/// metre, at every seed the owner tried.
#[test]
fn every_placed_leaflet_vertex_lies_in_a_planned_foliage_cell() {
    for seed in SEEDS {
        let (built, _) = fields(seed);
        let o = &built.outputs;
        let planned = o.field.as_ref().unwrap();
        let element = o.element.as_ref().unwrap();
        let leaves = &o.leaves.as_ref().unwrap().instances;
        let origin = planned.bounds().unwrap().min;
        for cell in [0.1, 0.25, 1.0] {
            let mut cells = HashSet::new();
            for m in leaves.matrices() {
                for v in &element.positions {
                    let p = (transform_point(&m, *v) - origin) / cell;
                    cells.insert([p.x.floor() as i64, p.y.floor() as i64, p.z.floor() as i64]);
                }
            }
            let half = Vec3::new(0.5, 0.5, 0.5);
            let missed = cells
                .iter()
                .filter(|c| {
                    let centre = Vec3::new(c[0] as f64, c[1] as f64, c[2] as f64) + half;
                    !planned
                        .query(origin + centre * cell, cell / 2.)
                        .unwrap()
                        .foliage
                })
                .count();
            let of = cells.len();
            assert_eq!(missed, 0, "seed {seed} at {cell} m: {missed} of {of} cells");
        }
    }
}

/// Over a non-overlapping grid the count estimates sum to the plan's total,
/// which is every leaflet placement hangs before the cull.
#[test]
fn the_palm_estimates_sum_to_every_leaflet() {
    let (built, _) = fields(7);
    let o = &built.outputs;
    let planned = o.field.as_ref().unwrap();
    let total = f64::from(o.plan.as_ref().unwrap().total);
    assert_eq!(total, o.leaves.as_ref().unwrap().placed as f64);
    let sum: f64 = grid(planned, 1.0)
        .into_iter()
        .map(|c| planned.query(c, 0.5).unwrap().leaves)
        .sum();
    assert!((sum - total).abs() <= total * 0.01, "{sum} against {total}");
}

/// The planned crown against the placed one on the same quarter-metre grid:
/// the share of cells whose foliage flag agrees, the planned foliage cells
/// against the placed, and the placed cells the plan does not report. The
/// placed field answers from each leaf's box, whose corners stand past its
/// vertices, so a few such cells hold no leaflet; the vertex test above is
/// the plan's conservative contract.
#[test]
fn the_planned_palm_agrees_with_the_placed_one() {
    for seed in SEEDS {
        let (built, placed) = fields(seed);
        let planned = built.outputs.field.as_ref().unwrap();
        let [mut agree, mut cells, mut base, mut plan, mut missed] = [0usize; 5];
        for c in grid(planned, 0.25) {
            let p = planned.query(c, 0.125).unwrap().foliage;
            let q = placed.query(c, 0.125).unwrap().foliage;
            cells += 1;
            agree += usize::from(p == q);
            base += usize::from(q);
            plan += usize::from(p);
            missed += usize::from(q && !p);
        }
        let share = agree as f64 / cells as f64;
        let ratio = plan as f64 / base as f64;
        eprintln!(
            "palm seed {seed}: {agree} of {cells} cells agree ({share:.4}); foliage cells \
             planned {plan}, placed {base}, ratio {ratio:.2}; placed only {missed}"
        );
        assert!(share >= 0.75, "seed {seed}: {share}");
        assert!(ratio <= 3.5, "seed {seed}: {ratio}");
        assert!(missed * 1000 <= base, "seed {seed}: {missed} of {base}");
    }
}
