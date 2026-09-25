//! What the palm's field agreement can reach (fn-150). On the quarter-metre
//! grid at the four seeds the owner tried, three fields are held to each
//! other: the placed field (each leaf's world-aligned box, what the targets
//! are measured against), the exact leaflets (each leaf's own oriented box,
//! cell to box) and the plan. No conservative plan can agree with the placed
//! field better than the exact leaflets do.
//!
//! `cargo run --profile ci -p telperion-core --example palm_field_ceiling`
use std::collections::HashSet;
use telperion_core::{
    field::Field,
    foliage::transform_point,
    math::Vec3,
    pipeline::{self, Request},
    presets::Preset,
};

fn main() {
    for seed in [1u32, 7, 1407, 4242] {
        let mut f = Preset::from_id("date-palm").unwrap().parameters();
        f.skeleton.seed = seed;
        let b = pipeline::build(
            &f,
            Request {
                leaves: true,
                field: Some(None),
                ..Request::default()
            },
        )
        .unwrap();
        let o = &b.outputs;
        let planned = o.field.as_ref().unwrap();
        let el = o.element.as_ref().unwrap();
        let leaves = &o.leaves.as_ref().unwrap().instances;
        let placed = Field::new(&b.skeleton.tree, Some((leaves, el))).unwrap();
        let bb = planned.bounds().unwrap();
        let cell = 0.25;
        let (mut lo, mut hi) = (el.positions[0], el.positions[0]);
        for p in &el.positions {
            lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
            hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
        }
        // exact: per-leaf oriented box, cell-to-box SAT over 6 face axes
        let mut exact = HashSet::new();
        for m in leaves.matrices() {
            let col =
                |c: usize| Vec3::new(m[c * 4] as f64, m[c * 4 + 1] as f64, m[c * 4 + 2] as f64);
            let corners: Vec<Vec3> = (0..8)
                .map(|i| {
                    let pick = |bit, a: f64, b: f64| if i & bit == 0 { a } else { b };
                    transform_point(
                        &m,
                        Vec3::new(
                            pick(1, lo.x, hi.x),
                            pick(2, lo.y, hi.y),
                            pick(4, lo.z, hi.z),
                        ),
                    )
                })
                .collect();
            let span = |u: Vec3| {
                corners
                    .iter()
                    .fold((f64::INFINITY, f64::NEG_INFINITY), |r, c| {
                        (r.0.min(c.dot(u)), r.1.max(c.dot(u)))
                    })
            };
            let axes: Vec<(Vec3, (f64, f64))> = [col(0), col(1), col(2)]
                .iter()
                .map(|u| u.normalized())
                .map(|u| (u, span(u)))
                .collect();
            let (sx, sy, sz) = (span(Vec3::X), span(Vec3::Y), span(Vec3::Z));
            let idx = |v: f64, m: f64| ((v - m) / cell).floor() as i64;
            for i in idx(sx.0, bb.min.x)..=idx(sx.1, bb.min.x) {
                for j in idx(sy.0, bb.min.y)..=idx(sy.1, bb.min.y) {
                    for k in idx(sz.0, bb.min.z)..=idx(sz.1, bb.min.z) {
                        let c = bb.min
                            + Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5) * cell;
                        let h = cell / 2.;
                        let ok = axes.iter().all(|(u, (a, b))| {
                            let r = h * (u.x.abs() + u.y.abs() + u.z.abs());
                            c.dot(*u) + r >= *a && c.dot(*u) - r <= *b
                        });
                        if ok {
                            exact.insert((i, j, k));
                        }
                    }
                }
            }
        }
        let n = |lo: f64, hi: f64| ((hi - lo) / cell).ceil() as i64 + 1;
        let (nx, ny, nz) = (
            n(bb.min.x, bb.max.x),
            n(bb.min.y, bb.max.y),
            n(bb.min.z, bb.max.z),
        );
        let (mut cells, mut ea, mut pa, mut pe, mut a_n, mut e_n, mut p_n) =
            (0usize, 0usize, 0usize, 0usize, 0usize, 0usize, 0usize);
        for i in 0..nx {
            for j in 0..ny {
                for k in 0..nz {
                    let c =
                        bb.min + Vec3::new(i as f64 + 0.5, j as f64 + 0.5, k as f64 + 0.5) * cell;
                    let a = placed.query(c, 0.125).unwrap().foliage;
                    let p = planned.query(c, 0.125).unwrap().foliage;
                    let e = exact.contains(&(i, j, k));
                    cells += 1;
                    ea += usize::from(e == a);
                    pa += usize::from(p == a);
                    pe += usize::from(p == e);
                    a_n += usize::from(a);
                    e_n += usize::from(e);
                    p_n += usize::from(p);
                }
            }
        }
        let f = |x: usize| x as f64 / cells as f64;
        println!("seed {seed}: cells {cells}; exact-vs-placed agree {:.4} ratio {:.3}; plan-vs-placed agree {:.4} ratio {:.3}; plan-vs-exact agree {:.4} ratio {:.3}",
            f(ea), e_n as f64 / a_n as f64, f(pa), p_n as f64 / a_n as f64, f(pe), p_n as f64 / e_n as f64);
    }
}
