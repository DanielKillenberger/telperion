//! The harness's own build of every shipped preset (`Generator::prepare`,
//! resident delivery; a refused tree is named and skipped), at its default
//! age and seed, at the whole-tree, mid and close views:
//! `<preset>-<view>.png` into the out directory, with each build's
//! milliseconds and leaves printed (fn-256 task 3's stills).
//!
//!   cargo run --profile ci -p telperion-render --example resident_stills -- <out dir> [preset,...]
use std::time::Instant;
use telperion_core::{math::Vec3, presets::by_identity, presets::CATALOGUE, surface::Bounds};
use telperion_render::{
    generation::{Delivery, Generator},
    hero_pose, render, write_png, Camera, Gpu, Renderer, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);

/// The hero camera's horizontal heading, from the tree to the camera.
fn toward(hero: &Camera) -> Vec3 {
    Vec3::new(hero.position.x, 0.0, hero.position.z).normalized()
}

/// The crown's edge on the hero camera's side, a little over half-way up,
/// from `share` of the tree's height out.
fn nearer(bounds: &Bounds, hero: &Camera, share: f64) -> Camera {
    let t = toward(hero);
    let height = bounds.max.y - bounds.min.y;
    let reach = 0.5 * (bounds.max.x - bounds.min.x).max(bounds.max.z - bounds.min.z);
    let side = Vec3::new(t.z, 0.0, -t.x);
    let target = Vec3::Y * (0.55 * height) + t * (0.5 * reach) + side * (0.2 * reach);
    let distance = share * height;
    Camera {
        position: target + t * distance + Vec3::Y * (0.1 * distance),
        target,
        field_of_view: 38.0,
        near: 0.005,
        far: 4.0 * height,
    }
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let out = args
        .first()
        .ok_or("usage: resident_stills <out dir> [preset,...]")?;
    std::fs::create_dir_all(out).map_err(|e| e.to_string())?;
    let only: Option<Vec<&str>> = args.get(1).map(|a| a.split(',').collect());
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let gen = Generator::new(&renderer).map_err(|e| e.to_string())?;
    for id in CATALOGUE
        .iter()
        .map(|c| c.1)
        .filter(|id| only.as_ref().is_none_or(|o| o.contains(id)))
    {
        let family = by_identity(id).map_err(|e| e.to_string())?;
        // The drawn tree let go first, as a page short of room does.
        renderer.release_tree();
        let t = Instant::now();
        // A refusal is reported and the preset skipped, never rebuilt on
        // the CPU (fn-256 R1).
        let prepared = match gen.prepare(&family, Delivery::Resident) {
            Ok(p) => p,
            Err(e) => {
                println!("{id}: refused ({e})");
                continue;
            }
        };
        let ms = t.elapsed().as_secs_f64() * 1e3;
        let leaves = prepared.count();
        renderer
            .submit_prepared(prepared)
            .map_err(|e| e.to_string())?;
        renderer.set_family(&family);
        let bounds = renderer.bounds().ok_or("nothing to frame")?;
        let hero = hero_pose(bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
        for (view, camera) in [
            ("whole", hero),
            ("mid", nearer(&bounds, &hero, 0.4)),
            ("close", nearer(&bounds, &hero, 0.12)),
        ] {
            let still =
                render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
            let path = std::path::Path::new(out).join(format!("{id}-{view}.png"));
            write_png(&path, &still).map_err(|e| e.to_string())?;
        }
        println!("{id} {}: built in {ms:.0} ms, {leaves} leaves", family.age);
    }
    Ok(())
}
