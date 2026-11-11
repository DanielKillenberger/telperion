//! A preset grown in through the growth the page uses (fn-270 R6, R7;
//! fn-353 R4): begun for its age, grown on and drawn `steps` times a year
//! by `pipeline::growth`, each draw put up in place of the last and taken
//! as a still, all from the camera that frames the grown tree. The stills
//! are numbered for a video encoder.
//!
//!   cargo run --profile ci -p telperion-render --example grow_in --
//!     <out dir> <preset> <steps a year> [<first year>]
use std::sync::Arc;
use telperion_core::{mesh, pipeline::growth, presets::by_identity};
use telperion_render::{
    hero_pose, render, write_png, Gpu, Renderer, SceneRow, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);
/// The stills' sun, as the space stills light theirs.
const SUN: (f64, f64) = (115.0, 60.0);

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, preset, steps, rest @ ..] = &args[..] else {
        return Err("usage: grow_in <out dir> <preset> <steps a year> [<first year>]".into());
    };
    let steps: u32 = steps.parse().map_err(|e| format!("steps: {e}"))?;
    let first: u32 = rest
        .first()
        .map_or(Ok(1), |f| f.parse())
        .map_err(|e| format!("first year: {e}"))?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let family = by_identity(preset).map_err(|e| e.to_string())?;
    // The camera frames the tree at its age, built once and let go.
    let bounds = mesh::build(&family).map_err(|e| e.to_string())?.bounds;
    let camera = hero_pose(bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_scene(SceneRow {
        sun_azimuth: SUN.0,
        sun_elevation: SUN.1,
        ..SceneRow::default()
    });
    let params = Arc::new(growth::Params::of(&family).map_err(|e| e.to_string())?);
    let mut tree =
        growth::begin(&params, family.skeleton.seed, family.age).map_err(|e| e.to_string())?;
    let mut frame = 0;
    for year in first..=tree.horizon() {
        // The seedling stands only from its first budburst.
        for k in (if year == 1 { steps } else { 1 })..=steps {
            let at = f64::from(year) - 1.0 + f64::from(k) / f64::from(steps);
            let started = std::time::Instant::now();
            growth::grow_to(&mut tree, at).map_err(|e| e.to_string())?;
            let grown = started.elapsed().as_secs_f64() * 1e3;
            let started = std::time::Instant::now();
            let drawn = growth::mesh(&tree, at, &family).map_err(|e| e.to_string())?;
            let settled = started.elapsed().as_secs_f64() * 1e3;
            let leaves = drawn.foliage_instances();
            renderer.submit(&drawn).map_err(|e| e.to_string())?;
            renderer.set_family(&family);
            renderer.set_colouring(growth::colouring(&tree, at).map_err(|e| e.to_string())?);
            let still =
                render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
            let file = format!("{out}/{preset}-{frame:05}.png");
            write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
            println!("{file}: at {at:.3}, grown in {grown:.0} ms, drawn in {settled:.0} ms, {leaves} leaves");
            frame += 1;
        }
    }
    Ok(())
}
