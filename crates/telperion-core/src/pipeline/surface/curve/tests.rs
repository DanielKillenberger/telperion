use super::*;
use crate::pipeline::{clothe, skeleton, GrowInput, Inputs};
use crate::presets::{Preset, CATALOGUE, IN_WORK};

/// Every values preset's clothed skeleton, its surface rows and its name.
fn trees() -> Vec<(&'static str, Tree, f64, SurfaceParams)> {
    CATALOGUE
        .iter()
        .chain(IN_WORK)
        .map(|&(_, id, _, _)| {
            let family = Preset::from_id(id).unwrap().parameters();
            let mut tree = skeleton(GrowInput::of(&family)).unwrap().tree;
            let inputs = Inputs::of(&family);
            clothe(&mut tree, &inputs).unwrap();
            (id, tree, inputs.surface.height, inputs.surface.params)
        })
        .collect()
}

/// The curve carries the whole wood: the sweep's rings, drawn from the curve
/// alone, are the sweep's own to the bit at every preset's side count (12
/// for all but the lobed legends), its shaped leaf-base cells included.
#[test]
fn every_presets_rings_are_drawn_from_its_curve_to_the_bit() {
    for (id, tree, height, params) in trees() {
        let mesh = build(&tree, height, &params).unwrap();
        let curve = curve(&tree, height, &params).unwrap();
        let (positions, coords) = curve.rings(segments(&params)).unwrap();
        let bits = |v: &[f32]| v.iter().map(|x| x.to_bits()).collect::<Vec<_>>();
        assert_eq!(positions.len(), mesh.positions.len(), "{id}: vertex count");
        assert!(
            bits(&positions) == bits(&mesh.positions),
            "{id}: positions differ"
        );
        assert!(bits(&coords) == bits(&mesh.coords), "{id}: coords differ");
        assert_eq!(curve.runs.len(), mesh.runs, "{id}: runs");
        if id == "date-palm" {
            assert!(
                !curve.sections.is_empty(),
                "the palm's leaf bases are shaped runs"
            );
        }
    }
}

/// Clusters tile every run with shared ends, hold every ring in their
/// sphere and keep a monotone error ladder that is zero at full detail.
#[test]
fn clusters_bound_every_ring_and_their_ladder_rises() {
    for (id, tree, height, params) in trees() {
        let curve = curve(&tree, height, &params).unwrap();
        for (r, run) in curve.runs.iter().enumerate() {
            let mut next = run.first;
            for c in curve.clusters.iter().filter(|c| c.run as usize == r) {
                assert_eq!(c.first, next, "{id}: clusters tile run {r}");
                assert!(c.count as usize <= CLUSTER && c.count >= 2 || run.count == 1);
                next = c.first + c.count - 1;
                assert_eq!(c.errors[0], 0.0);
                assert!(c.errors.windows(2).all(|w| w[0] <= w[1]), "{id}: ladder");
                for p in &curve.points[c.first as usize..(c.first + c.count) as usize] {
                    assert!(p.centre.distance(c.centre) + p.radius <= c.reach * (1.0 + 1e-9));
                }
            }
            assert_eq!(next, run.first + run.count - 1, "{id}: run {r} covered");
        }
    }
}

/// The 28-byte point: rings drawn from the packed curve stand within the
/// error the packing states: the octahedral frame's 1e-4 radians on the
/// run's radius, plus float32's rounding of a position (2 units in the last
/// place, 2.4e-7 of its distance from the origin).
#[test]
fn packed_points_stand_within_their_stated_error() {
    for (id, tree, height, params) in trees() {
        let curve = curve(&tree, height, &params).unwrap();
        let segments = segments(&params);
        let (exact, _) = curve.rings(segments).unwrap();
        let (packed, _) = curve.as_packed().rings(segments).unwrap();
        let (mut worst, mut share, mut at) = (0.0f64, 0.0f64, 0);
        for run in &curve.runs {
            let points = curve.run_points(run);
            let largest = points.iter().map(|p| p.radius).fold(0.0, f64::max);
            let lobe = 1.0 + curve.lobe_depth.abs();
            let vertices = points.len() * segments + 2;
            for v in at..at + vertices {
                let e = |k: usize| f64::from(exact[v * 3 + k]);
                let d = (0..3)
                    .map(|k| (e(k) - f64::from(packed[v * 3 + k])).powi(2))
                    .sum::<f64>()
                    .sqrt();
                let far = (0..3).map(|k| e(k) * e(k)).sum::<f64>().sqrt();
                let stated = 1e-4 * largest * lobe + 2.4e-7 * far.max(1.0);
                worst = worst.max(d);
                share = share.max(d / stated);
            }
            at += vertices;
        }
        eprintln!("{id}: packed off by at most {worst:.2e} m, {share:.2} of the stated error");
        assert!(share <= 1.0, "{id}: {share}");
    }
}
