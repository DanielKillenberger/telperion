//! Smooth bark's three layers as pictures: lichen, lenticels and peel each
//! change the close-up alone, none of them moves a vertex, and all three back
//! at zero draw the frame they started from byte for byte. The two tables
//! that state them hold the resolution and redraw contracts every bark does.
mod common;
#[path = "common/resolution.rs"]
mod resolution;
use telperion_core::{
    material::MaterialParams,
    mesh::{self, Detail, TreeMesh},
    presets::Preset,
    surface::SurfaceMesh,
};
use telperion_render::{
    render, shot_pose, Camera, Renderer, Shot, Still, View, GROUND_REACH, STILL_FORMAT,
};

/// fn-34's close-ups of the two barks, as their reference records state them.
const BEECH_BASE: &str =
    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.04,"fov":45,"distance":1.2}"#;
const BIRCH_BARK: &str =
    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.06,"fov":45,"distance":1.5}"#;

fn hash(mesh: &SurfaceMesh) -> u64 {
    mesh.positions
        .iter()
        .chain(&mesh.normals)
        .chain(&mesh.coords)
        .flat_map(|v| v.to_le_bytes())
        .chain(mesh.indices.iter().flat_map(|v| v.to_le_bytes()))
        .fold(14695981039346656037, |h, b| {
            (h ^ u64::from(b)).wrapping_mul(1099511628211)
        })
}

fn changed(a: &Still, b: &Still) -> f64 {
    a.rgba
        .iter()
        .zip(&b.rgba)
        .map(|(x, y)| f64::from(x.abs_diff(*y)))
        .sum::<f64>()
        / a.rgba.len() as f64
}

/// A tree of the preset at seed 7 on stage, and the close-up's pose.
fn staged(
    renderer: &mut Renderer,
    preset: Preset,
    shot: &str,
    size: (u32, u32),
) -> (TreeMesh, Camera) {
    let mut family = preset.parameters();
    family.skeleton.seed = 7;
    let tree = mesh::build(&family, Detail::Full).unwrap();
    renderer.submit(&tree).unwrap();
    renderer.set_material(family.material);
    renderer.set_figure(false);
    let bounds = renderer.bounds().unwrap();
    let aspect = f64::from(size.0) / f64::from(size.1);
    let camera = shot_pose(bounds, aspect, GROUND_REACH, &Shot::parse(shot).unwrap());
    (tree, camera)
}

#[test]
fn each_smooth_layer_changes_the_close_up_and_moves_no_wood() {
    let Some(gpu) = common::gpu() else { return };
    let size = (480, 360);
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let (tree, camera) = staged(&mut renderer, Preset::SilverBirch, BIRCH_BARK, size);
    renderer.set_view(View::Bare);
    let shipped = Preset::SilverBirch.parameters().material;
    let plain = MaterialParams {
        lichen_strength: 0.0,
        lenticel_strength: 0.0,
        peel_curl: 0.0,
        ..shipped
    };
    renderer.set_material(plain);
    let before = render(&mut renderer, &camera, size.0, size.1).unwrap();
    for (name, row) in [
        (
            "lichen",
            MaterialParams {
                lichen_strength: shipped.lichen_strength,
                ..plain
            },
        ),
        (
            "lenticel",
            MaterialParams {
                lenticel_strength: shipped.lenticel_strength,
                ..plain
            },
        ),
        (
            "peel",
            MaterialParams {
                peel_curl: shipped.peel_curl,
                ..plain
            },
        ),
    ] {
        let mut family = Preset::SilverBirch.parameters();
        family.skeleton.seed = 7;
        family.material = row;
        assert_eq!(
            hash(&tree.wood),
            hash(&mesh::build(&family, Detail::Full).unwrap().wood),
            "{name} moved the wood the core hands up"
        );
        renderer.set_material(row);
        let on = render(&mut renderer, &camera, size.0, size.1).unwrap();
        let moved = changed(&before, &on);
        eprintln!("{name}: mean {moved:.4}/255 over the birch's close-up");
        assert!(moved > 0.2, "{name} changed almost nothing: {moved}/255");
    }
    renderer.set_material(plain);
    let again = render(&mut renderer, &camera, size.0, size.1).unwrap();
    assert_eq!(
        before.rgba, again.rgba,
        "the layers at zero did not return the frame"
    );
}

#[test]
fn the_smooth_barks_hold_the_resolution_and_redraw_contracts() {
    let Some(gpu) = common::gpu() else { return };
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let mut measured = Vec::new();
    for (preset, shot) in [
        (Preset::EuropeanBeech, BEECH_BASE),
        (Preset::SilverBirch, BIRCH_BARK),
    ] {
        let (_, camera) = staged(&mut renderer, preset, shot, (1600, 1000));
        renderer.set_view(View::Clay);
        let clay = render(&mut renderer, &camera, 800, 500).unwrap();
        let mask = resolution::wood_mask(&clay);
        assert!(
            mask.len() > 20_000,
            "{preset:?}: too little wood in the close-up: {}",
            mask.len()
        );
        renderer.set_view(View::Bare);
        let high = render(&mut renderer, &camera, 1600, 1000).unwrap();
        let low = render(&mut renderer, &camera, 800, 500).unwrap();
        let again = render(&mut renderer, &camera, 800, 500).unwrap();
        assert_eq!(
            low.rgba, again.rgba,
            "{preset:?}: the identical close-up changed"
        );
        let (mean, p95) = resolution::masked_agreement(&high, &low, &mask);
        eprintln!(
            "{preset:?} close-up: {} pixels, mean {mean:.4}/255, p95 {p95:.2}/255",
            mask.len()
        );
        measured.push((format!("{preset:?} close-up"), mean, p95, 3.0));
    }
    resolution::record("smooth_bark", &measured);
    assert!(
        measured
            .iter()
            .all(|(_, mean, p95, bound)| mean <= bound && *p95 <= 12.0),
        "smooth bark aliases across resolution: {measured:?}"
    );
}
