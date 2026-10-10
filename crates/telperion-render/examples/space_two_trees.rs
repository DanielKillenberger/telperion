//! The Two Trees' sheets (fn-218 R2, fn-219 R2): the laurelin or telperion
//! preset through the one pipeline and the headless renderer, with the
//! supernatural group off (an old beech) and on (the legendary tree), each
//! at the whole-tree, mid, close and near views. Writes
//! `<tree>-<age>-<seed>-<off|on>-<view>.png` and prints each tree's build
//! cost and size.
//!
//!   cargo run --profile ci -p telperion-render --example space_two_trees -- <tree> <out dir> <age> [seed]
use std::time::Instant;
use telperion_core::{
    math::Vec3, mesh::TreeMesh, pipeline::executor, presets::Preset, surface::Bounds,
};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};

/// The stills' size: 960x720, or `STILL_SIZE=<w>x<h>` (fn-248's mid view
/// at the harness's 1920x1080).
fn size() -> Result<(u32, u32), String> {
    let Ok(text) = std::env::var("STILL_SIZE") else {
        return Ok((960, 720));
    };
    let (w, h) = text.split_once('x').ok_or("STILL_SIZE is <w>x<h>")?;
    let n = |v: &str| v.parse::<u32>().map_err(|e| format!("STILL_SIZE: {e}"));
    Ok((n(w)?, n(h)?))
}
/// The still runner's sun (`space/still.rs`): behind the camera and high.
const SUN: (f64, f64) = (115.0, 60.0);

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [tree, out, age, rest @ ..] = &args[..] else {
        return Err("usage: space_two_trees <laurelin|telperion> <out dir> <age> [seed]".into());
    };
    let preset = match tree.as_str() {
        "laurelin" => Preset::Laurelin,
        "telperion" => Preset::Telperion,
        _ => return Err(format!("{tree} is not one of the Two Trees")),
    };
    let age: f64 = age.parse().map_err(|e| format!("age {age}: {e}"))?;
    let seed: u32 = rest
        .first()
        .map_or(Ok(1), |s| s.parse())
        .map_err(|e| format!("seed: {e}"))?;
    let size = size()?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for on in [false, true] {
        let mut family = preset.parameters();
        (family.age, family.skeleton.seed) = (age, seed);
        family.supernatural.enabled = on;
        // A trial flowering share for the short shoots, `FLOWERING=<share>`
        // (fn-253), in place of the preset's.
        if let Ok(share) = std::env::var("FLOWERING") {
            let share = share.parse().map_err(|e| format!("FLOWERING: {e}"))?;
            family.supernatural.flowering.age14 = Some(share);
        }
        let started = Instant::now();
        let grown = executor::grow(&family).map_err(|e| e.to_string())?;
        let x = grown.expansion().map_err(|e| e.to_string())?;
        let mesh = x.mesh().map_err(|e| e.to_string())?;
        let ms = started.elapsed().as_secs_f64() * 1e3;
        let b = mesh.bounds.max - mesh.bounds.min;
        let florets = mesh.organ.as_ref().map_or(0, |o| o.instances.len());
        let group = if on { "on" } else { "off" };
        println!(
            "{group}: built in {ms:.0} ms; {} nodes, {} leaves, {florets} florets; {:.1} m tall, {:.1} x {:.1} m",
            x.tree().nodes.len(),
            mesh.foliage_instances(),
            b.y,
            b.x,
            b.z
        );
        let hero = hero_pose(
            mesh.bounds,
            f64::from(size.0) / f64::from(size.1),
            GROUND_REACH,
        );
        let views = [
            ("whole", hero.clone()),
            ("mid", mid(&mesh.bounds, &hero)),
            ("close", close(&mesh, &hero, 0.7)?),
            // A raceme's stalks at hand's length (fn-253).
            ("near", close(&mesh, &hero, 0.25)?),
        ];
        for (view, camera) in &views {
            renderer
                .submit_at(&mesh, Level::Chosen)
                .map_err(|e| e.to_string())?;
            renderer.set_family(&family);
            renderer.set_view(View::Whole);
            renderer.set_scene(SceneRow {
                sun_azimuth: SUN.0,
                sun_elevation: SUN.1,
                ..SceneRow::default()
            });
            let still = render(&mut renderer, camera, size.0, size.1).map_err(|e| e.to_string())?;
            let file = format!("{out}/{tree}-{age}-{seed}-{group}-{view}.png");
            write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
            // The share of the frame the florets cover (fn-217 M0's organ
            // mask, as `tests/organ_coverage.rs` reads it), with `COVERAGE`
            // set (fn-248).
            if on && std::env::var_os("COVERAGE").is_some() {
                renderer.set_organ_mask(true);
                let mask = render(&mut renderer, camera, size.0, size.1);
                renderer.set_organ_mask(false);
                let mask = mask.map_err(|e| e.to_string())?;
                println!(
                    "{group} {view}: florets cover {:.4} of the frame",
                    coverage(&mask)
                );
            }
        }
    }
    Ok(())
}

/// The share of the frame a mask covers: the mean of its resolved green,
/// decoded to linear (`tests/organ_coverage.rs`).
fn coverage(still: &telperion_render::Still) -> f64 {
    let linear = |v: u8| {
        let c = f64::from(v) / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    };
    let pixels = still.rgba.chunks(4);
    pixels.clone().map(|p| linear(p[1])).sum::<f64>() / pixels.len() as f64
}

/// The hero camera's horizontal heading, from the tree to the camera.
fn toward(hero: &Camera) -> Vec3 {
    Vec3::new(hero.position.x, 0.0, hero.position.z).normalized()
}

/// A limb's worth of crown: the crown's edge on the hero camera's side,
/// a little over half-way up, from 0.4 of the height out.
fn mid(bounds: &Bounds, hero: &Camera) -> Camera {
    let t = toward(hero);
    let height = bounds.max.y - bounds.min.y;
    let reach = 0.5 * (bounds.max.x - bounds.min.x).max(bounds.max.z - bounds.min.z);
    let side = Vec3::new(t.z, 0.0, -t.x);
    let target = Vec3::Y * (0.55 * height) + t * (0.5 * reach) + side * (0.2 * reach);
    let distance = 0.4 * height;
    Camera {
        position: target + t * distance + Vec3::Y * (0.1 * distance),
        target,
        field_of_view: 38.0,
        near: 0.05,
        far: 4.0 * height,
    }
}

/// One spray close up: the outermost whole floret half-way up the crown on
/// the camera's side, or, on a tree that flowers nowhere, the outermost
/// leaf there, from `distance` metres.
fn close(mesh: &TreeMesh, hero: &Camera, distance: f64) -> Result<Camera, String> {
    let t = toward(hero);
    let height = mesh.bounds.max.y - mesh.bounds.min.y;
    let instances = &mesh.organ.as_ref().unwrap_or(&mesh.foliage).instances;
    let r = &instances.reference;
    let target = (instances.leaves.iter())
        .filter(|l| r.scale(**l) > 0.6)
        .map(|l| r.position(*l))
        .filter(|p| (0.45..0.8).contains(&(p.y / height)))
        .max_by(|a, b| a.dot(t).total_cmp(&b.dot(t)))
        .ok_or("no whole floret or leaf half-way up the crown")?;
    Ok(Camera {
        position: target + t * distance + Vec3::Y * (0.2 * distance),
        target,
        field_of_view: 38.0,
        near: 0.005,
        far: 4.0 * height,
    })
}
