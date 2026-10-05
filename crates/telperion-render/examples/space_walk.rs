//! A walk between two tree-space species (fn-206 R3): the species a share
//! `t` of the way from one to the other, every setting between theirs on
//! the shared reference axis, grown and drawn bare on one dressing (the
//! beech's rows, so only the structure walks), from one fixed camera so
//! the strip's frames compare. One tree per process.
//!
//!   cargo run --profile ci -p telperion-render --example space_walk -- <out dir> <from> <to> <t> <seed> <age>
#[path = "space/tree.rs"]
mod tree;

use telperion_core::{math::Vec3, params, pipeline::executor, presets::Preset, surface::Bounds};
use telperion_render::{
    hero_pose, render, write_png, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT,
};
use telperion_space::{beech, blend, grow, oak, palm, spruce, Light, Request, Species};

const BUDGET: u32 = 20_000_000;
const SIZE: (u32, u32) = (960, 720);
/// The stills' sun, behind the camera (`space/still.rs`).
const SUN: (f64, f64) = (115.0, 60.0);
/// One frame for every tree of every walk: 30 m wide, 26 m tall.
const FRAME: (f64, f64) = (15.0, 26.0);
/// The beech's rows (`space_beech.rs`): the walk's one dressing.
const ROWS: &str = r#"{"canopy": {"shortShootSpacing": 0, "shootRadius": 0.018, "limbClumping": 0}, "skeleton": {"twigs": {"twig": {"internodeLength": 0.006}}}}"#;

/// A species' trunk-level ages on the shared chain (`space/tree.rs`).
fn trunk(name: &str) -> &'static [usize] {
    match name {
        "beech" => &tree::BEECH_TRUNK,
        "spruce" => &tree::SPRUCE_TRUNK,
        "oak" => &tree::OAK_TRUNK,
        _ => &tree::PALM_TRUNK,
    }
}

fn species(name: &str) -> Result<Species, String> {
    Ok(match name {
        "beech" => beech(),
        "spruce" => spruce(),
        "oak" => oak(),
        "palm" => palm(),
        _ => return Err(format!("no species {name}")),
    })
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    // The budget is a safety bound; one look at a full tree may raise it.
    let (out, from, to, t, seed, age, budget) = match &args[..] {
        [out, from, to, t, seed, age] => (out, from, to, t, seed, age, BUDGET),
        [out, from, to, t, seed, age, budget] => {
            let budget = budget
                .parse::<u32>()
                .map_err(|e| format!("{budget}: {e}"))?;
            (out, from, to, t, seed, age, budget)
        }
        _ => {
            return Err("usage: space_walk <out dir> <from> <to> <t> <seed> <age> [budget]".into())
        }
    };
    let number = |s: &str| s.parse::<f64>().map_err(|e| format!("{s}: {e}"));
    let (t, seed, age) = (number(t)?, number(seed)? as u64, number(age)? as u32);
    let walked = blend(&species(from)?, &species(to)?, t).map_err(|e| format!("{e:?}"))?;
    let request = Request {
        age,
        seed,
        budget,
        light: Light::NEUTRAL,
    };
    let structure = grow(&walked, request).map_err(|e| format!("t {t}: {e:?}"))?;
    let preset = Preset::from_id("european-beech")
        .ok_or("no beech preset")?
        .parameters();
    let rows: serde_json::Value = serde_json::from_str(ROWS).map_err(|e| e.to_string())?;
    let mut family = params::overlay(&preset, &rows).map_err(|e| format!("{e:?}"))?;
    family.skeleton.seed = (seed ^ (seed >> 32)) as u32;
    // The stem: either end's trunk-level ages, which the walk mixes.
    let mut stem: Vec<usize> = trunk(from).iter().chain(trunk(to)).copied().collect();
    stem.sort_unstable();
    stem.dedup();
    let mesh = executor::expand(tree::convert(&structure, &stem), &family)
        .and_then(|x| x.mesh())
        .map_err(|e| e.to_string())?;
    let bounds = Bounds {
        min: Vec3::new(-FRAME.0, 0.0, -FRAME.0),
        max: Vec3::new(FRAME.0, FRAME.1, FRAME.0),
    };
    let camera = hero_pose(bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer
        .submit_at(&mesh, Level::Chosen)
        .map_err(|e| e.to_string())?;
    renderer.set_material(family.material);
    renderer.set_view(View::Bare);
    renderer.set_scene(SceneRow {
        sun_azimuth: SUN.0,
        sun_elevation: SUN.1,
        ..SceneRow::default()
    });
    let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
    let path = format!("{out}/walk-{from}-{to}-{t:.3}.png");
    write_png(std::path::Path::new(&path), &still).map_err(|e| e.to_string())?;
    let height = mesh.bounds.max.y - mesh.bounds.min.y;
    println!("{from} to {to} at {t:.3}: {height:.1} m tall");
    Ok(())
}
