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
    assert_eq!(
        gpu, reference,
        "the device drew other clusters or rings than the reference"
    );
    assert!(
        tubes == cpu.indices && ribbons == cpu.ribbons,
        "the triangles differ"
    );
    let (mut gap, mut cosine) = (0.0f32, 1.0f32);
    for i in 0..cpu.radii.len() {
        let v = &vertices[i * 9..i * 9 + 9];
        let p = &cpu.positions[i * 3..i * 3 + 3];
        let n = &cpu.normals[i * 3..i * 3 + 3];
        gap = gap.max((0..3).map(|k| (v[k] - p[k]).abs()).fold(0.0, f32::max));
        cosine = cosine.min(v[3] * n[0] + v[4] * n[1] + v[5] * n[2]);
    }
    assert!(gap < 1e-3, "a vertex stands {gap} m from the reference's");
    assert!(
        cosine > 0.999,
        "a normal turns {cosine} from the reference's"
    );
    format!("{gpu}; gap {gap:.2e} m, cosine {cosine:.6}")
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
    let size = (40, 30);
    let aspect = f64::from(size.0) / f64::from(size.1);
    let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
    let close = Camera {
        position: hero.target + (hero.position - hero.target) * 0.15,
        ..hero
    };
    render(&mut renderer, &close, size.0, size.1).unwrap();
    let report = renderer.curve_report().unwrap();
    assert_eq!(report.overrun, 0, "over budget: {report:?}");
    let half = |k: usize| report.budget[k] / 2;
    // At the finest scale the demand is the drawn view's own.
    assert_eq!(report.scale, 1.0);
    assert!(
        report.demand[0] > half(0) && report.vertices > half(1),
        "the view did not reach the second slices: {report:?}"
    );
    println!("{}", compare(&renderer, &curve, &close, size, report.scale));
}
