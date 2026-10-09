//! fn-213 round 11: one leaf close-up of the birch per age and seed, in
//! leaf, the camera 0.9 m off the outermost leaf toward the hero camera at
//! mid-crown height, under the still runner's sun. Copied into
//! `crates/telperion-render/examples/` to run, never committed there.
//!
//!   cargo run --profile ci -p telperion-render --example leaf_closeup -- <out dir> <age>... --seeds 1,7
use telperion_core::{math::Vec3, pipeline::executor, presets::Preset};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};

const SIZE: (u32, u32) = (960, 720);
const SUN: (f64, f64) = (115.0, 60.0);

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (out, rest) = args
        .split_first()
        .ok_or("usage: leaf_closeup <out> <age>... --seeds 1,7")?;
    let (mut ages, mut seeds) = (Vec::new(), vec![1u32]);
    let mut words = rest.iter();
    while let Some(w) = words.next() {
        if w == "--seeds" {
            let list = words.next().ok_or("--seeds needs a list")?;
            seeds = list
                .split(',')
                .map(|s| s.parse().map_err(|e| format!("{e}")))
                .collect::<Result<_, _>>()?;
        } else {
            ages.push(w.parse::<u32>().map_err(|e| format!("{e}"))?);
        }
    }
    let family = Preset::from_id("silver-birch")
        .ok_or("no preset")?
        .parameters();
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for &age in &ages {
        for &seed in &seeds {
            let mut family = family.clone();
            family.age = f64::from(age);
            family.skeleton.seed = seed;
            let grown = executor::grow(&family).map_err(|e| e.to_string())?;
            let x = grown.expansion().map_err(|e| e.to_string())?;
            let mesh = x.mesh().map_err(|e| e.to_string())?;
            let b = mesh.bounds;
            let hero = hero_pose(b, f64::from(SIZE.0) / f64::from(SIZE.1), GROUND_REACH);
            let toward = Vec3::new(hero.position.x, 0.0, hero.position.z);
            let toward = toward * (1.0 / toward.length().max(1e-9));
            let (lo, hi) = (
                b.min.y + 0.4 * (b.max.y - b.min.y),
                b.min.y + 0.6 * (b.max.y - b.min.y),
            );
            let leaves = &mesh.foliage.instances;
            let mut best = (f64::MIN, Vec3::new(0.0, 0.0, 0.0));
            for k in 0..leaves.len() {
                let p = leaves.position(k);
                let reach = p.x * toward.x + p.z * toward.z;
                if p.y > lo && p.y < hi && reach > best.0 {
                    best = (reach, p);
                }
            }
            let target = best.1;
            let camera = Camera {
                position: target + toward * 0.9 + Vec3::new(0.0, 0.1, 0.0),
                target,
                field_of_view: 38.0,
                near: 0.02,
                far: 500.0,
            };
            renderer
                .submit_at(&mesh, Level::Chosen)
                .map_err(|e| e.to_string())?;
            renderer.set_material(family.material);
            renderer.set_view(View::Whole);
            renderer.set_scene(SceneRow {
                sun_azimuth: SUN.0,
                sun_elevation: SUN.1,
                ..SceneRow::default()
            });
            let still =
                render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
            let file = format!("{out}/birch-{age}-{seed}-leaf.png");
            write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
            println!(
                "{file}: target {:.2} {:.2} {:.2}",
                target.x, target.y, target.z
            );
        }
    }
    Ok(())
}
