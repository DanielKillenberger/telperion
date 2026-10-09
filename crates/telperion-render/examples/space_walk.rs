//! A walk between two tree-space species (fn-206 R3): the species a share
//! `t` of the way from one to the other, every setting between theirs on
//! the shared reference axis, grown and drawn bare on one dressing (the
//! beech's rows, so only the structure walks), from one fixed camera so
//! the strip's frames compare. One tree per process, through the one
//! pipeline (fn-198).
//!
//!   cargo run --profile ci -p telperion-render --example space_walk -- <out dir> <from> <to> <t> <seed> <age>
use telperion_core::{blend, math::Vec3, mesh, presets::Preset, surface::Bounds};
use telperion_render::{
    hero_pose, render, write_png, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);
/// The stills' sun, behind the camera (`space/still.rs`).
const SUN: (f64, f64) = (115.0, 60.0);
/// One frame for every tree of every walk: 30 m wide, 26 m tall.
const FRAME: (f64, f64) = (15.0, 26.0);

/// A species' preset by its short name.
fn preset(name: &str) -> Result<Preset, String> {
    Ok(match name {
        "beech" => Preset::EuropeanBeech,
        "spruce" => Preset::NorwaySpruce,
        "oak" => Preset::OregonWhiteOak,
        "palm" => Preset::DatePalm,
        "ordinary" => Preset::Ordinary,
        _ => return Err(format!("no species {name}")),
    })
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [out, from, to, t, seed, age] = &args[..] else {
        return Err("usage: space_walk <out dir> <from> <to> <t> <seed> <age>".into());
    };
    let number = |s: &str| s.parse::<f64>().map_err(|e| format!("{s}: {e}"));
    let (t, seed, age) = (number(t)?, number(seed)? as u32, number(age)?);
    // The species walk; the dressing is the beech's at both ends, so only
    // the structure walks. The stem is either end's trunk-level ages.
    // Both ends at the walk's age, so the blend is sized at the age it is
    // grown at (fn-230, host 2026-10-08).
    let (mut a, mut b) = (preset(from)?.parameters(), preset(to)?.parameters());
    (a.age, b.age) = (age, age);
    let mut family = Preset::EuropeanBeech.parameters();
    family.space = blend::families(&a, &b, t)
        .map_err(|e| format!("t {t}: {e}"))?
        .space;
    family.space.light = telperion_space::Light::NEUTRAL;
    family.age = age;
    family.skeleton.seed = seed;
    let mesh = mesh::build(&family).map_err(|e| format!("t {t}: {e}"))?;
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
    renderer.set_family(&family);
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
