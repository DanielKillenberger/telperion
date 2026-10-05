//! The wood surfaced on the device against the CPU reference at the same
//! view and error (fn-208, host decision 5): the same clusters drawn, the
//! same counts, and every vertex where the reference puts it.
use telperion_core::{pipeline::executor, presets::Preset, surface::Curve};
use telperion_render::{
    curve_viewer, hero_pose, render, Camera, Renderer, GROUND_REACH, STILL_FORMAT,
};

mod common;
use common::gpu;

const SIZE: (u32, u32) = (480, 360);

/// A preset's mesh, which carries its wood's curve.
fn tree(id: &str) -> (telperion_core::mesh::TreeMesh, Curve) {
    let family = Preset::from_id(id).unwrap().parameters();
    let mesh = executor::grow(&family)
        .unwrap()
        .expansion()
        .unwrap()
        .mesh()
        .unwrap();
    let curve = mesh.curve.clone();
    (mesh, curve)
}

/// How far the device's wood stands from the reference's: the counts, the
/// largest position gap in metres and the smallest normals' cosine, at the
/// error scale the device's budget chose.
fn compare(
    renderer: &Renderer,
    curve: &Curve,
    camera: &Camera,
    size: (u32, u32),
    scale: f64,
) -> String {
    let (vertices, tubes, ribbons) = renderer.curve_mesh().unwrap();
    let cpu = curve
        .tessellate(&curve_viewer(camera, size), 0.5 * scale, None)
        .unwrap();
    let counts =
        |v: usize, t: usize, r: usize| format!("{v} vertices, {} + {} triangles", t / 3, r / 3);
    let gpu = counts(vertices.len() / 9, tubes.len(), ribbons.len());
    let reference = counts(cpu.radii.len(), cpu.indices.len(), cpu.ribbons.len());
    // A stretch whose piece count sits on an integer boundary (its argument
    // within float32's reach of a square) may be cut once more on the device
    // than in float64: the exact-boundary class, deferred (host). Such a
    // view must still agree in count to a thousandth, and vertex for
    // vertex up to the first stretch that differs.
    let same = gpu == reference && tubes == cpu.indices && ribbons == cpu.ribbons;
    let (n, m) = (cpu.radii.len(), vertices.len() / 9);
    let matches = |g: usize, c: usize| {
        (0..3).all(|k| (vertices[g * 9 + k] - cpu.positions[c * 3 + k]).abs() < 1e-3)
    };
    // Vertex for vertex from the front up to the stretch that differs, and
    // from the back after it.
    let front = (0..n.min(m)).find(|&i| !matches(i, i)).unwrap_or(n.min(m));
    let back = (0..n.min(m) - front)
        .find(|&i| !matches(m - 1 - i, n - 1 - i))
        .unwrap_or(n.min(m) - front);
    if !same {
        let off = |g: usize, c: usize| g.abs_diff(c) as f64 / c.max(1) as f64;
        assert!(
            off(m, n) <= 1e-3
                && off(tubes.len(), cpu.indices.len()) <= 1e-3
                && off(ribbons.len(), cpu.ribbons.len()) <= 1e-3,
            "the device drew other clusters or rings than the reference: {gpu} against {reference}"
        );
    }
    assert!(
        front + back + 64 >= n,
        "the device parted from the reference for {} vertices",
        n - front - back
    );
    let (mut gap, mut cosine) = (0.0f32, 1.0f32);
    let pairs = (0..front)
        .map(|i| (i, i))
        .chain((0..back).map(|i| (m - 1 - i, n - 1 - i)));
    for (g, c) in pairs {
        let v = &vertices[g * 9..g * 9 + 9];
        let p = &cpu.positions[c * 3..c * 3 + 3];
        let q = &cpu.normals[c * 3..c * 3 + 3];
        gap = gap.max((0..3).map(|k| (v[k] - p[k]).abs()).fold(0.0, f32::max));
        cosine = cosine.min(v[3] * q[0] + v[4] * q[1] + v[5] * q[2]);
    }
    assert!(gap < 1e-3, "a vertex stands {gap} m from the reference's");
    assert!(
        cosine > 0.999,
        "a normal turns {cosine} from the reference's"
    );
    let boundary = if same {
        String::new()
    } else {
        format!(
            " (a boundary stretch: the reference drew {reference}; {} vertices apart)",
            n - front - back
        )
    };
    format!("{gpu}; gap {gap:.2e} m, cosine {cosine:.6}{boundary}")
}

/// The device's wood is the reference's at the hero view and close in, for
/// a preset of round wood and the palm's shaped cells.
#[test]
fn the_device_surfaces_the_wood_the_cpu_reference_does() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for id in ["ordinary", "date-palm"] {
        let (mesh, curve) = tree(id);
        renderer.submit(&mesh).unwrap();
        let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
        let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
        let close = Camera {
            position: hero.target + (hero.position - hero.target) * 0.15,
            ..hero
        };
        for (name, camera) in [("hero", hero), ("close", close)] {
            render(&mut renderer, &camera, SIZE.0, SIZE.1).unwrap();
            let report = renderer.curve_report().unwrap();
            assert_eq!(report.overrun, 0, "{id} {name}: over budget");
            let found = compare(&renderer, &curve, &camera, SIZE, report.scale);
            println!(
                "{id} {name}: scale {}, demand {:?}: {found}",
                report.scale, report.demand
            );
        }
    }
}

/// A view small enough that its screen-sized budget is crossed halfway, so
/// every record, vertex and index past the first slice is written through
/// the second binding (host decision 21), and still the reference's.
#[test]
fn the_second_slices_hold_what_the_first_could_not() {
    let Some(gpu) = gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let (mesh, curve) = tree("ordinary");
    renderer.submit(&mesh).unwrap();
    // The smallest view whose finest scale still fits, and so fills its
    // budget furthest: the demand a pixel rises as the view shrinks.
    let mut crossed = None;
    for width in (40..=96).step_by(4) {
        let size = (width, width * 3 / 4);
        let aspect = f64::from(size.0) / f64::from(size.1);
        let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
        let close = Camera {
            position: hero.target + (hero.position - hero.target) * 0.15,
            ..hero
        };
        render(&mut renderer, &close, size.0, size.1).unwrap();
        let report = renderer.curve_report().unwrap();
        let half = |k: usize| report.budget[k] / 2;
        if report.scale == 1.0 && report.demand[0] > half(0) && report.vertices > half(1) {
            assert_eq!(report.overrun, 0, "over budget: {report:?}");
            crossed = Some((size, close, report));
            break;
        }
    }
    let (size, close, report) = crossed.expect("no view reached the second slices at scale 1");
    println!("{}", compare(&renderer, &curve, &close, size, report.scale));
}
