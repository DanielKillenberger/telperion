use super::*;
use crate::pipeline::surface::curve::CurveRun;

/// Pixels a metre spans at one metre for the stills: 720 rows under 38°.
const PIXELS: f64 = 360.0 / 0.344_327_613_289_665_3;

/// The reference segment: a twig bent through 60° on an arc of 0.2 m, a node
/// every centimetre, tapering from 2 mm to 1 mm, its frame transported
/// exactly (the arc is planar, so its normal holds).
fn twig() -> (Curve, impl Fn(Vec3) -> Option<f64>) {
    const R: f64 = 0.2;
    const SWEEP: f64 = std::f64::consts::FRAC_PI_3;
    let sweep = SWEEP;
    let count = 22;
    let radius = |phi: f64| 0.002 - 0.001 * phi / SWEEP;
    let points: Vec<CurvePoint> = (0..count)
        .map(|i| {
            let phi = sweep * i as f64 / (count - 1) as f64;
            let (sin, cos) = phi.sin_cos();
            let tangent = Vec3::new(-sin, cos, 0.0);
            let normal = Vec3::new(0.0, 0.0, 1.0);
            CurvePoint {
                centre: Vec3::new(R * cos, R * sin, 0.0),
                radius: radius(phi),
                along: R * phi,
                normal,
                binormal: tangent.cross(normal),
            }
        })
        .collect();
    let run = CurveRun {
        first: 0,
        count: count as u32,
        trunk: false,
        section: None,
        largest_radius: 0.002,
    };
    let mut curve = Curve {
        points,
        runs: vec![run],
        height: 10.0,
        ..Curve::default()
    };
    let mut clusters = Vec::new();
    super::super::clusters(&curve, 0, &run, &mut clusters);
    curve.clusters = clusters;
    // How far a point stands off the exact tube: from the arc's axis, at the
    // nearest point of the arc, against the radius there.
    let off = move |p: Vec3| {
        let phi = p.y.atan2(p.x);
        if !(0.0..=SWEEP).contains(&phi) {
            return None;
        }
        let axis = Vec3::new(R * phi.cos(), R * phi.sin(), 0.0);
        Some(((p - axis).length() - radius(phi)).abs())
    };
    (curve, off)
}

/// A viewer `distance` from the twig's middle, looking straight at it.
fn viewer(distance: f64) -> Viewer {
    let (sin, cos) = std::f64::consts::FRAC_PI_6.sin_cos();
    let middle = Vec3::new(0.2 * cos, 0.2 * sin, 0.0);
    Viewer {
        eye: middle + Vec3::new(0.0, 0.0, distance),
        forward: Vec3::new(0.0, 0.0, -1.0),
        pixels_per_metre: PIXELS,
        near: 0.005,
        orthographic: false,
        planes: None,
    }
}

/// The most, in pixels at its own depth, that the surface stands off the
/// exact tube: sampled across every triangle that is not a cap.
fn worst(t: &Tessellation, viewer: &Viewer, off: &impl Fn(Vec3) -> Option<f64>) -> f64 {
    let at = |i: u32| {
        let p = &t.positions[i as usize * 3..i as usize * 3 + 3];
        Vec3::new(f64::from(p[0]), f64::from(p[1]), f64::from(p[2]))
    };
    let mut worst = 0.0f64;
    for tri in t.indices.chunks(3) {
        let [a, b, c] = [at(tri[0]), at(tri[1]), at(tri[2])];
        if [a, b, c].iter().any(|&v| off(v).is_none_or(|d| d > 1e-5)) {
            continue; // a cap, or past the arc's ends
        }
        for i in 0..=6 {
            for j in 0..=6 - i {
                let (u, v) = (i as f64 / 6.0, j as f64 / 6.0);
                let p = a * (1.0 - u - v) + b * u + c * v;
                if (p - viewer.eye).dot(viewer.forward) < viewer.near {
                    continue;
                }
                if let Some(d) = off(p) {
                    worst = worst.max(d * viewer.pixels_at(p));
                }
            }
        }
    }
    worst
}

/// The most, in pixels, a ribbon's edge stands off the exact tube's
/// silhouette: its half-width against the radius.
fn ribbon_worst(t: &Tessellation, viewer: &Viewer, off: &impl Fn(Vec3) -> Option<f64>) -> f64 {
    let mut worst = 0.0f64;
    for i in 0..t.radii.len() {
        if !t.ribbons.contains(&(i as u32)) {
            continue;
        }
        let p = Vec3::new(
            t.positions[3 * i].into(),
            t.positions[3 * i + 1].into(),
            t.positions[3 * i + 2].into(),
        );
        // A ribbon's middle vertex stands on the axis: only its edges trace
        // the silhouette.
        let axis = Vec3::new(p.x, p.y, 0.0).length() - 0.2;
        if axis.hypot(p.z) < 0.5 * f64::from(t.radii[i]) {
            continue;
        }
        if let Some(d) = off(p) {
            worst = worst.max(d * viewer.pixels_at(p));
        }
    }
    worst
}

/// R2: the twig's surface stands within half a pixel of the exact tube at
/// 5 cm, 50 cm, 5 m and 50 m, and its rings and sides grow as the eye nears.
#[test]
fn a_twig_stands_within_half_a_pixel_and_gains_detail_as_the_eye_nears() {
    let (curve, off) = twig();
    let mut last = (0usize, 0usize);
    for distance in [50.0, 5.0, 0.5, 0.05] {
        let viewer = viewer(distance);
        let t = curve.tessellate(&viewer, 0.5, None).unwrap();
        let error = worst(&t, &viewer, &off).max(ribbon_worst(&t, &viewer, &off));
        assert!(
            error <= 0.5,
            "at {distance} m the surface is {error:.2} px off"
        );
        let now = (t.rings, t.positions.len() / 3);
        assert!(
            now.0 >= last.0 && now.1 >= last.1,
            "detail fell nearer: {last:?} to {now:?}"
        );
        last = now;
    }
}

/// A budget the view would overrun is met by a coarser error, which the
/// tessellation names; one no scale meets is refused, never truncated.
#[test]
fn a_budget_coarsens_the_error_and_names_it_or_refuses() {
    let (curve, _) = twig();
    let near = viewer(0.05);
    let full = curve.tessellate(&near, 0.5, None).unwrap();
    assert_eq!(full.scale, 1.0);
    let budget = |tube| Budget {
        vertices: usize::MAX,
        tube_indices: tube,
        ribbon_indices: usize::MAX,
    };
    let tight = curve
        .tessellate(&near, 0.5, Some(budget(full.indices.len() - 1)))
        .unwrap();
    assert!(tight.scale > 1.0 && tight.indices.len() < full.indices.len());
    assert!(matches!(
        curve.tessellate(&near, 0.5, Some(budget(1))),
        Err(Error::ResourceLimit(_))
    ));
}

/// Every values preset's wood surfaces whole at its hero distance, every
/// index in range and every vertex finite, its shaped cells included.
#[test]
fn every_presets_wood_surfaces_at_a_hero_view() {
    use crate::pipeline::{clothe, skeleton, GrowInput, Inputs};
    use crate::presets::{Preset, CATALOGUE, IN_WORK};
    for &(_, id, _, _) in CATALOGUE.iter().chain(IN_WORK) {
        let family = Preset::from_id(id).unwrap().parameters();
        let mut tree = skeleton(GrowInput::of(&family)).unwrap().tree;
        let inputs = Inputs::of(&family);
        clothe(&mut tree, &inputs).unwrap();
        let s = inputs.surface;
        let curve = crate::pipeline::surface::curve(&tree, s.height, &s.params).unwrap();
        let top = curve.points.iter().map(|p| p.centre.y).fold(0.0, f64::max);
        let middle = Vec3::new(0.0, 0.5 * top, 0.0);
        // The whole height in 720 rows with the stills' margin of 1.15.
        let away = 1.15 * top * PIXELS / 720.0;
        let viewer = Viewer {
            eye: middle + Vec3::new(away * 0.906, 0.0, away * -0.423),
            forward: Vec3::new(-0.906, 0.0, 0.423),
            pixels_per_metre: PIXELS,
            near: 0.1,
            orthographic: false,
            planes: None,
        };
        // No view asks more than the tree's demand at its nearest pixels a
        // metre (host decision 23), at the hero distance and close in. Each
        // view's surface is checked and dropped before the next, and the
        // close view culls to its frustum as every real view does, so the
        // test holds one view's wood at a time (CI's runner memory).
        let demand = curve.demand();
        let asked = |t: &super::Tessellation| {
            [
                t.rings as u64,
                (t.positions.len() / 3) as u64,
                t.indices.len() as u64,
                t.ribbons.len() as u64,
            ]
        };
        let t = curve.tessellate(&viewer, 0.5, None).unwrap();
        let most = demand.at(viewer.pixels_per_metre / viewer.near, 0.5);
        assert!(
            asked(&t).iter().zip(most).all(|(&a, m)| a <= m),
            "{id}: {:?} past {most:?} at the hero distance",
            asked(&t)
        );
        let vertices = (t.positions.len() / 3) as u32;
        assert!(
            t.indices.iter().all(|&i| i < vertices),
            "{id}: an index past the vertices"
        );
        assert!(
            t.positions.iter().chain(&t.normals).all(|v| v.is_finite()),
            "{id}"
        );
        eprintln!(
            "{id}: {} tube and {} ribbon triangles, {} rings at the hero distance",
            t.indices.len() / 3,
            t.ribbons.len() / 3,
            t.rings
        );
        drop(t);
        let eye = middle + (viewer.eye - middle) * 0.1;
        let close = Viewer {
            eye,
            planes: Some(frustum(
                eye,
                viewer.forward,
                25f64.to_radians(),
                viewer.near,
            )),
            ..viewer
        };
        let t = curve.tessellate(&close, 0.5, None).unwrap();
        let most = demand.at(close.pixels_per_metre / close.near, 0.5);
        assert!(
            asked(&t).iter().zip(most).all(|(&a, m)| a <= m),
            "{id}: {:?} past {most:?} close in",
            asked(&t)
        );
        assert!(t.rings > 0, "{id}: the close view sees no wood");
    }
}

/// The planes of a square frustum from `eye` along `forward`, `half` its
/// half-angle, inward-facing as `Viewer::planes` reads them.
fn frustum(eye: Vec3, forward: Vec3, half: f64, near: f64) -> [[f64; 4]; 6] {
    let right = forward.cross(Vec3::new(0.0, 1.0, 0.0)).normalized();
    let up = right.cross(forward);
    let (sin, cos) = half.sin_cos();
    let plane = |n: Vec3, d: f64| [n.x, n.y, n.z, d];
    let side = |n: Vec3| plane(n, -n.dot(eye));
    [
        side(right * cos + forward * sin),
        side(right * -cos + forward * sin),
        side(up * cos + forward * sin),
        side(up * -cos + forward * sin),
        plane(forward, -forward.dot(eye) - near),
        plane(forward * -1.0, forward.dot(eye) + 1e4),
    ]
}
