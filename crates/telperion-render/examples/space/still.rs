//! A tree-space species through the pipeline's own expansion and the
//! headless renderer: bare and whole stills per age and seed, the camera
//! fitted to each tree, and its measures. The structure comes from
//! `telperion-space`; the wood, leaves and material from a preset's rows.
use crate::tree;
use std::time::Instant;
use telperion_core::{math::Vec3, params, pipeline::executor, presets::Preset, surface::Bounds};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, View, GROUND_REACH, STILL_FORMAT,
};
use telperion_space::{grow, Request, Species};

const BUDGET: u32 = 20_000_000;
const SIZE: (u32, u32) = (960, 720);

/// Grows `species` at every age and seed the arguments name
/// (`<out dir> <age>... [--seeds 1,7]`), dresses it in the `preset`'s rows
/// overlaid by `rows`, and writes `<name>-<age>-<seed>-<view>.png`.
pub fn run(name: &str, species: fn() -> Species, preset: &str, rows: &str) -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let (out, rest) = args.split_first().ok_or(format!(
        "usage: space_{name} <out dir> <age>... [--seeds 1,7]"
    ))?;
    let (mut ages, mut seeds) = (Vec::new(), vec![1u64, 7]);
    let mut words = rest.iter();
    while let Some(word) = words.next() {
        if word == "--seeds" {
            let list = words.next().ok_or("--seeds needs a list")?;
            seeds = list
                .split(',')
                .map(|s| s.parse().map_err(|e| format!("{e}")))
                .collect::<Result<_, _>>()?;
        } else {
            ages.push(
                word.parse::<u32>()
                    .map_err(|e| format!("age {word}: {e}"))?,
            );
        }
    }
    let preset = Preset::from_id(preset)
        .ok_or(format!("no preset {preset}"))?
        .parameters();
    let rows: serde_json::Value = serde_json::from_str(rows).map_err(|e| e.to_string())?;
    let family = params::overlay(&preset, &rows).map_err(|e| format!("{e:?}"))?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let species = species();
    for &age in &ages {
        for &seed in &seeds {
            let started = Instant::now();
            let structure = grow(
                &species,
                Request {
                    age,
                    seed,
                    budget: BUDGET,
                },
            )
            .map_err(|e| format!("age {age} seed {seed}: {e:?}"))?;
            let grown = started.elapsed().as_secs_f64() * 1e3;
            let pipeline_tree = tree::convert(&structure);
            let nodes = pipeline_tree.nodes.len();
            let (fine, wood) = wood_km(&pipeline_tree);
            let bounds = bounds(&pipeline_tree);
            let dressed = Instant::now();
            let mesh = executor::expand(pipeline_tree, &family)
                .and_then(|x| x.mesh())
                .map_err(|e| e.to_string())?;
            let dress = dressed.elapsed().as_secs_f64() * 1e3;
            let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
            let camera = hero_pose(bounds, aspect, GROUND_REACH);
            let (base, limb) = close_ups(&bounds, &camera);
            let shots = [
                (View::Bare, "bare", &camera),
                (View::Whole, "whole", &camera),
                (View::Bare, "base", &base),
                (View::Bare, "limb", &limb),
            ];
            for (view, shot, camera) in shots {
                renderer
                    .submit_at(&mesh, Level::Chosen)
                    .map_err(|e| e.to_string())?;
                renderer.set_material(family.material);
                renderer.set_view(view);
                let still =
                    render(&mut renderer, camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
                let path = format!("{out}/{name}-{age}-{seed}-{shot}.png");
                write_png(std::path::Path::new(&path), &still).map_err(|e| e.to_string())?;
            }
            let (b, l) = (bounds.max - bounds.min, mesh.foliage.instances.len());
            println!(
                "age {age} seed {seed}: grown in {grown:.1} ms, dressed in {dress:.0} ms; {nodes} nodes, {l} leaves; {:.1} m tall, {:.1} x {:.1} m; wood {wood:.2} km, fine {fine:.2} km",
                b.y, b.x, b.z
            );
        }
    }
    Ok(())
}

fn bounds(tree: &telperion_core::tree::Tree) -> Bounds {
    let mut b = Bounds {
        min: tree.nodes[0].position,
        max: tree.nodes[0].position,
    };
    for n in &tree.nodes {
        let p = n.position;
        b.min = Vec3::new(b.min.x.min(p.x), b.min.y.min(p.y), b.min.z.min(p.z));
        b.max = Vec3::new(b.max.x.max(p.x), b.max.y.max(p.y), b.max.z.max(p.z));
    }
    b
}

/// Wood length in km: on nodes finer than `STRUCTURAL` of the root's
/// radius (the baseline's fine wood), and all of it.
fn wood_km(tree: &telperion_core::tree::Tree) -> (f64, f64) {
    let root = tree.nodes[0].radius;
    let (mut fine, mut all) = (0.0, 0.0);
    for n in &tree.nodes[1..] {
        let length = (n.position - tree.nodes[n.parent.unwrap() as usize].position).length();
        all += length;
        if n.radius < tree::STRUCTURAL * root {
            fine += length;
        }
    }
    (fine / 1e3, all / 1e3)
}

/// Close-ups from the hero camera's side: the trunk base from 6 m, and the
/// crown a third of its width off the stem at half its height from 9 m.
fn close_ups(bounds: &Bounds, hero: &Camera) -> (Camera, Camera) {
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
    (shot(Vec3::new(0.0, 1.2, 0.0), 6.0), shot(limb, 9.0))
}
