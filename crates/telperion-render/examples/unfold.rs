//! A preset grown to an age and drawn at fractions of its last year (fn-270
//! R5): one still a fraction, every one framed by the camera of the whole
//! year, so this year's shoots and leaves are seen unfolding in place.
//!
//!   cargo run --profile ci -p telperion-render --example unfold --
//!     <out dir> <preset> <age> <seed> <fraction>,... [--crown]
//!
//! `--crown` frames the top of the crown from a quarter of its height off,
//! where the year's shoots stand, in place of the whole tree.
use telperion_core::{math::Vec3, pipeline::growth, presets::Preset};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);
/// The stills' sun, as the space stills light theirs.
const SUN: (f64, f64) = (115.0, 60.0);

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, preset, age, seed, fractions, rest @ ..] = &args[..] else {
        return Err("usage: unfold <out dir> <preset> <age> <seed> <fraction>,... [--crown]".into());
    };
    let crown = rest.iter().any(|a| a == "--crown");
    let family = Preset::from_id(preset).ok_or(format!("unknown tree {preset}"))?.parameters();
    let age: u32 = age.parse().map_err(|e| format!("age: {e}"))?;
    let seed: u32 = seed.parse().map_err(|e| format!("seed: {e}"))?;
    let parse = |f: &str| f.parse().map_err(|e| format!("{f}: {e}"));
    let fractions: Vec<f64> = fractions.split(',').map(parse).collect::<Result<_, _>>()?;
    let params = growth::Params::of(&family).map_err(|e| e.to_string())?;
    let mut state = growth::begin(&params, seed, f64::from(age)).map_err(|e| e.to_string())?;
    growth::grow_to(&mut state, f64::from(age)).map_err(|e| e.to_string())?;
    let at = |f: f64| f64::from(age) - 1.0 + f;
    let started = std::time::Instant::now();
    let whole = growth::mesh(&state, at(1.0), &family).map_err(|e| e.to_string())?;
    println!("the whole year drawn in {} ms", started.elapsed().as_millis());
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(whole.bounds, aspect, GROUND_REACH);
    let camera = match crown {
        true => top(&whole.bounds, &hero),
        false => hero,
    };
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for f in fractions {
        let started = std::time::Instant::now();
        let mesh = growth::mesh(&state, at(f), &family).map_err(|e| e.to_string())?;
        let drawn = started.elapsed().as_millis();
        renderer.submit_at(&mesh, Level::Chosen).map_err(|e| e.to_string())?;
        renderer.set_family(&family);
        renderer.set_colouring(growth::colouring(&state, at(f)).map_err(|e| e.to_string())?);
        renderer.set_view(View::Whole);
        renderer.set_scene(SceneRow { sun_azimuth: SUN.0, sun_elevation: SUN.1, ..SceneRow::default() });
        let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
        let shot = if crown { "crown" } else { "whole" };
        let file = format!("{out}/{preset}-{age}-{seed}-{f}-{shot}.png");
        write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
        println!("{file}: drawn in {drawn} ms, {} leaves, {} wood vertices", mesh.foliage_instances(), mesh.wood_vertices());
    }
    Ok(())
}

/// The top of the crown, a quarter of the tree's height below its top,
/// seen from the hero camera's side three quarters of the height off.
fn top(bounds: &telperion_core::surface::Bounds, hero: &Camera) -> Camera {
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length().max(1e-9));
    let height = bounds.max.y - bounds.min.y;
    let target = Vec3::new(0.0, bounds.max.y - 0.25 * height, 0.0);
    Camera { position: target + toward * (0.75 * height), target, field_of_view: 38.0, ..*hero }
}
