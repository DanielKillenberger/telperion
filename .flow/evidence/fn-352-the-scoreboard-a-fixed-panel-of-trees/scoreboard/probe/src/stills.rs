//! The stills, framed and lit as the space examples frame theirs
//! (`crates/telperion-render/examples/space/still.rs`): the hero pose fitted
//! to the dressed tree, the sun behind the camera and high, and its three
//! close-ups. Shots: `whole` and `bare` (hero, leafed and leafless),
//! `crown` (the upper crown from three quarters of the height away), `spray`
//! (a limb from 9 m, leafed) and `low` (the trunk base from 6 m, leafed);
//! fn-354: `joint` (the largest limb's junction with the trunk, leafless,
//! from eight trunk radii and no nearer than 3 m) and `foot` (the trunk
//! base from 6 m, leafless).
use telperion_core::{math::Vec3, mesh::TreeMesh, Family};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);
const SUN: (f64, f64) = (115.0, 60.0);

pub fn shoot(
    mesh: &TreeMesh,
    family: &Family,
    out: &str,
    shots: &[&str],
    joint: Option<(Vec3, f64)>,
) -> Result<(), String> {
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let hero = hero_pose(mesh.bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
    let (base, limb, crown) = close_ups(mesh, &hero);
    let at_joint = joint.map(|(at, r)| near(&hero, at, (8.0 * r).max(3.0)));
    for &shot in shots {
        let (view, camera) = match shot {
            "whole" => (View::Whole, &hero),
            "bare" => (View::Bare, &hero),
            "crown" => (View::Whole, &crown),
            "spray" => (View::Whole, &limb),
            "low" => (View::Whole, &base),
            "foot" => (View::Bare, &base),
            "joint" => match &at_joint {
                Some(c) => (View::Bare, c),
                None => return Err("no limb leaves the trunk".into()),
            },
            other => return Err(format!("no shot {other}")),
        };
        renderer
            .submit_at(mesh, Level::Chosen)
            .map_err(|e| e.to_string())?;
        renderer.set_family(family);
        renderer.set_view(view);
        renderer.set_scene(SceneRow {
            sun_azimuth: SUN.0,
            sun_elevation: SUN.1,
            ..SceneRow::default()
        });
        let still = render(&mut renderer, camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
        write_png(std::path::Path::new(&format!("{out}-{shot}.png")), &still)
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// A close-up of `target` from `distance` on the hero camera's side.
fn near(hero: &Camera, target: Vec3, distance: f64) -> Camera {
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length().max(1e-9));
    Camera {
        position: target + toward * distance + Vec3::new(0.0, 0.5, 0.0),
        target,
        field_of_view: 38.0,
        near: 0.05,
        far: 500.0,
    }
}

/// still.rs's close-ups from the hero camera's side: the trunk base from
/// 6 m, a limb a third of the width off the stem at half the height from
/// 9 m, and the crown top from three quarters of the height away.
fn close_ups(mesh: &TreeMesh, hero: &Camera) -> (Camera, Camera, Camera) {
    let b = &mesh.bounds;
    let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
    let toward = toward * (1.0 / toward.length().max(1e-9));
    let shot = |target: Vec3, distance: f64| Camera {
        position: target + toward * distance + Vec3::new(0.0, 0.5, 0.0),
        target,
        field_of_view: 38.0,
        near: 0.05,
        far: 500.0,
    };
    let (height, width) = (b.max.y - b.min.y, b.max.x - b.min.x);
    let side = Vec3::new(toward.z, 0.0, -toward.x);
    (
        shot(Vec3::new(0.0, 1.2, 0.0), 6.0),
        shot(Vec3::new(0.0, 0.5 * height, 0.0) + side * (width / 3.0), 9.0),
        shot(Vec3::new(0.0, b.max.y - 0.25 * height, 0.0), 0.75 * height),
    )
}
