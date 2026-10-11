//! fn-253: how much of a tree's frame its leaves show, beside its florets.
//! Builds a preset at its own age and seed (or `AGE`, `SEED`, and a trial
//! short-shoot flowering share `FLOWERING`), prints its leaves and florets,
//! and for the hero, mid and close views of `space_two_trees` the share of
//! the frame the crown adds over the bare wood, the share the florets cover
//! (the organ mask, in the clay room), and the share of the crown's pixels
//! no floret covers: the leaves that show.
//!
//!   cargo run --profile ci -p telperion-render --example leaf_share -- <preset>
use telperion_core::{math::Vec3, mesh::TreeMesh, pipeline::executor, presets::Preset};
use telperion_render::{
    hero_pose, render, Camera, Gpu, Level, Renderer, SceneRow, Still, View, GROUND_REACH,
    STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);

fn main() -> Result<(), String> {
    let id = std::env::args().nth(1).ok_or("usage: leaf_share <preset>")?;
    let preset = Preset::from_id(&id).ok_or("no preset")?;
    let mut family = preset.parameters();
    let var = |k: &str| std::env::var(k).ok();
    if let Some(a) = var("AGE") {
        family.age = a.parse().map_err(|e| format!("AGE: {e}"))?;
    }
    if let Some(s) = var("SEED") {
        family.skeleton.seed = s.parse().map_err(|e| format!("SEED: {e}"))?;
    }
    if let Some(s) = var("FLOWERING") {
        family.supernatural.flowering.age14 = Some(s.parse().map_err(|e| format!("FLOWERING: {e}"))?);
    }
    let x = executor::grow(&family).map_err(|e| e.to_string())?;
    let x = x.expansion().map_err(|e| e.to_string())?;
    let mesh = x.mesh().map_err(|e| e.to_string())?;
    let florets = mesh.organ.as_ref().map_or(0, |o| o.instances.len());
    println!(
        "{id} {} seed {}: {} leaves, {florets} organ instances, {} clusters",
        family.age,
        family.skeleton.seed,
        mesh.foliage_instances(),
        x.tree().clusters.len()
    );
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.submit_at(&mesh, Level::Chosen).map_err(|e| e.to_string())?;
    renderer.set_family(&family);
    renderer.set_scene(SceneRow {
        sun_azimuth: 115.0,
        sun_elevation: 60.0,
        ..SceneRow::default()
    });
    let hero = hero_pose(mesh.bounds, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
    let views = [
        ("whole", hero.clone()),
        ("mid", mid(&mesh, &hero)),
        ("close", close(&mesh, &hero)?),
    ];
    for (view, camera) in &views {
        let mut shot = |v: View, mask: bool| {
            renderer.set_view(v);
            renderer.set_organ_mask(mask);
            render(&mut renderer, camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())
        };
        let whole = shot(View::Whole, false)?;
        let bare = shot(View::Bare, false)?;
        let mask = shot(View::Clay, true)?;
        renderer.set_organ_mask(false);
        let (crown, organ, leaves) = shares(&whole, &bare, &mask);
        println!("  {view}: crown {crown:.4}, florets {organ:.4}, leaves showing {leaves:.4} of the frame");
    }
    Ok(())
}

/// The share of the frame the crown changes over the bare wood, the
/// florets' share under the mask, and the crown's share no floret covers.
fn shares(whole: &Still, bare: &Still, mask: &Still) -> (f64, f64, f64) {
    let n = (whole.rgba.len() / 4) as f64;
    let (mut crown, mut organ, mut leaves) = (0.0, 0.0, 0.0);
    for k in 0..whole.rgba.len() / 4 {
        let changed = (0..3).any(|c| whole.rgba[4 * k + c].abs_diff(bare.rgba[4 * k + c]) > 8);
        let flower = mask.rgba[4 * k + 1] >= 128;
        crown += f64::from(u8::from(changed));
        organ += f64::from(u8::from(flower));
        leaves += f64::from(u8::from(changed && !flower));
    }
    (crown / n, organ / n, leaves / n)
}

fn toward(hero: &Camera) -> Vec3 {
    Vec3::new(hero.position.x, 0.0, hero.position.z).normalized()
}

/// `space_two_trees`'s mid view: the crown's edge on the camera's side.
fn mid(mesh: &TreeMesh, hero: &Camera) -> Camera {
    let (b, t) = (&mesh.bounds, toward(hero));
    let height = b.max.y - b.min.y;
    let reach = 0.5 * (b.max.x - b.min.x).max(b.max.z - b.min.z);
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

/// `space_two_trees`'s close view: the outermost whole floret half-way up
/// the crown on the camera's side, from 0.7 m.
fn close(mesh: &TreeMesh, hero: &Camera) -> Result<Camera, String> {
    let t = toward(hero);
    let height = mesh.bounds.max.y - mesh.bounds.min.y;
    let instances = &mesh.organ.as_ref().unwrap_or(&mesh.foliage).instances;
    let r = &instances.reference;
    let target = (instances.leaves.iter())
        .filter(|l| r.scale(**l) > 0.6)
        .map(|l| r.position(*l))
        .filter(|p| (0.45..0.8).contains(&(p.y / height)))
        .max_by(|a, b| a.dot(t).total_cmp(&b.dot(t)))
        .ok_or("no whole floret half-way up the crown")?;
    let distance = 0.7;
    Ok(Camera {
        position: target + t * distance + Vec3::Y * (0.2 * distance),
        target,
        field_of_view: 38.0,
        near: 0.005,
        far: 4.0 * height,
    })
}
