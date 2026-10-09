//! fn-236: the 0.1.4 (old generator) Oregon white oak in the engine
//! stills' shots, camera and sun. Scratch, never committed on master.
use telperion_core::{math::Vec3, mesh::TreeMesh, presets::Preset, surface::Bounds, Family};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};
const SIZE: (u32, u32) = (960, 720);
const SUN: (f64, f64) = (115.0, 60.0);
struct Tuning;
impl Tuning {
    fn shoots(&self, _: &str) -> bool {
        true
    }
}
fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = &args[0];
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for seed in [1u32, 7, 42] {
        let mut family = Preset::from_id("oregon-white-oak").unwrap().parameters();
        family.skeleton.seed = seed;
        let mesh = telperion_core::mesh::build(&family).map_err(|e| e.to_string())?;
        stills(&mut renderer, &mesh, &family, &format!("{out}/old-{seed}"), &Tuning)?;
        println!("seed {seed}: {} leaves", mesh.foliage_instances());
    }
    Ok(())
}
/// Every shot of one dressed tree, framed on the mesh, its leaves and
/// fronds included, so a crown that stands above the wood is in frame:
/// `<path>-<shot>.png`.
fn stills(
    renderer: &mut Renderer,
    mesh: &TreeMesh,
    family: &Family,
    path: &str,
    tuning: &Tuning,
) -> Result<(), String> {
    // Before the GPU sees it, so a refused allocation still says what it
    // was asked to hold.
    eprintln!(
        "{path}: mesh {} wood vertices, {} wood triangles, {} leaves",
        mesh.wood_vertices(),
        mesh.wood_triangles(),
        mesh.foliage_instances()
    );
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let camera = hero_pose(mesh.bounds, aspect, GROUND_REACH);
    let (base, limb, crown) = close_ups(&mesh.bounds, &camera);
    let shots = [
        (View::Bare, "bare", &camera),
        (View::Whole, "whole", &camera),
        (View::Bare, "base", &base),
        (View::Bare, "limb", &limb),
        // The limb close-up in leaf: how the needles dress a spray.
        (View::Whole, "spray", &limb),
        (View::Whole, "crown", &crown),
    ];
    for (view, shot, camera) in shots.into_iter().filter(|s| tuning.shoots(s.1)) {
        renderer
            .submit_at(mesh, Level::Chosen)
            .map_err(|e| e.to_string())?;
        renderer.set_material(family.material);
        renderer.set_view(view);
        renderer.set_scene(SceneRow {
            sun_azimuth: SUN.0,
            sun_elevation: SUN.1,
            ..SceneRow::default()
        });
        let still = render(renderer, camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
        let file = format!("{path}-{shot}.png");
        write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// Close-ups from the hero camera's side: the trunk base from 6 m, the
/// crown a third of its width off the stem at half its height from 9 m,
/// and the top of the crown, a quarter of the height below the top, from
/// three quarters of the height away.
fn close_ups(bounds: &Bounds, hero: &Camera) -> (Camera, Camera, Camera) {
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length().max(1e-9));
    let shot = |target: Vec3, distance: f64| Camera {
        position: target + toward * distance + Vec3::new(0.0, 0.5, 0.0),
        target,
        field_of_view: 38.0,
        near: 0.05,
        far: 500.0,
    };
    let height = bounds.max.y - bounds.min.y;
    let width = bounds.max.x - bounds.min.x;
    let side = Vec3::new(toward.z, 0.0, -toward.x);
    let limb = Vec3::new(0.0, 0.5 * height, 0.0) + side * (width / 3.0);
    let top = Vec3::new(0.0, bounds.max.y - 0.25 * height, 0.0);
    (
        shot(Vec3::new(0.0, 1.2, 0.0), 6.0),
        shot(limb, 9.0),
        shot(top, 0.75 * height),
    )
}
