//! fn-246's crown ground truth: each species' crown at the harness's whole
//! view and a mid view (half the distance), at 1920x1080 and at a multiple
//! of it, the wood surfaced for the smaller size in both, written as raw
//! RGBA beside the bare view. A second call compares a thinned 1x draw with
//! an unthinned draw at the multiple, box-reduced, over the pixels the crown
//! changes in that truth: mean and p95 code values, and the share of crown
//! pixels each covers.
//!
//!   cargo run --profile ci -p telperion-render --example crown_truth -- render <dir> <scale> [species...]
//!   cargo run --profile ci -p telperion-render --example crown_truth -- compare <truth dir> <scale> <dir> [species...]
//!
//! Every shipped species by default.
use std::path::Path;
use telperion_core::{mesh, presets::Preset};
use telperion_render::{
    hero_pose, render, Camera, Gpu, Renderer, View, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (1920, 1080);
const SPECIES: [&str; 8] = [
    "ordinary",
    "oregon-white-oak",
    "norway-spruce",
    "european-beech",
    "silver-birch",
    "date-palm",
    "telperion",
    "laurelin",
];

fn views(renderer: &Renderer) -> [(&'static str, Camera); 2] {
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(renderer.bounds().unwrap(), aspect, GROUND_REACH);
    let mid = Camera {
        position: hero.target + (hero.position - hero.target) * 0.5,
        ..hero
    };
    [("whole", hero), ("mid", mid)]
}

fn draw(dir: &Path, scale: u32, species: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dir)?;
    let mut renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    let (w, h) = (SIZE.0 * scale, SIZE.1 * scale);
    for &id in species {
        let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
        family.skeleton.seed = 1;
        renderer.submit(&mesh::build(&family)?)?;
        renderer.set_family(&family);
        renderer.pin_curve_viewport(Some(SIZE));
        for (name, camera) in views(&renderer) {
            for (view, label) in [(View::Whole, "whole"), (View::Bare, "bare")] {
                renderer.set_view(view);
                let still = render(&mut renderer, &camera, w, h)?;
                std::fs::write(dir.join(format!("{id}-{name}-{label}")), &still.rgba)?;
            }
        }
        renderer.pin_curve_viewport(None);
        eprintln!("drew {id}");
    }
    Ok(())
}

/// One frame's channel `c` at (x, y), box-averaged over `s` pixels a side.
fn at(rgba: &[u8], width: usize, s: usize, x: usize, y: usize, c: usize) -> f64 {
    let sum: u32 = (0..s)
        .flat_map(|dy| (0..s).map(move |dx| (dx, dy)))
        .map(|(dx, dy)| u32::from(rgba[((y * s + dy) * width * s + x * s + dx) * 4 + c]))
        .sum();
    f64::from(sum) / (s * s) as f64
}

fn compare(
    truth: &Path,
    scale: u32,
    dir: &Path,
    species: &[&str],
) -> Result<(), Box<dyn std::error::Error>> {
    let (w, h, s) = (SIZE.0 as usize, SIZE.1 as usize, scale as usize);
    for &id in species {
        for name in ["whole", "mid"] {
            let read =
                |d: &Path, label: &str| std::fs::read(d.join(format!("{id}-{name}-{label}")));
            let (whole, bare) = (read(truth, "whole")?, read(truth, "bare")?);
            let (thin, thin_bare) = (read(dir, "whole")?, read(dir, "bare")?);
            let (mut errors, mut covered, mut drawn) = (Vec::new(), 0usize, 0usize);
            for y in 0..h {
                for x in 0..w {
                    let gap = |c| (at(&whole, w, s, x, y, c) - at(&bare, w, s, x, y, c)).abs();
                    if (0..3).map(gap).fold(0.0, f64::max) <= 8.0 {
                        continue;
                    }
                    covered += 1;
                    let mine = |c| (at(&thin, w, 1, x, y, c) - at(&thin_bare, w, 1, x, y, c)).abs();
                    drawn += usize::from((0..3).map(mine).fold(0.0, f64::max) > 8.0);
                    for c in 0..3 {
                        errors.push((at(&thin, w, 1, x, y, c) - at(&whole, w, s, x, y, c)).abs());
                    }
                }
            }
            let mean = errors.iter().sum::<f64>() / errors.len().max(1) as f64;
            errors.sort_by(f64::total_cmp);
            let p95 = errors.get(errors.len() * 95 / 100).copied().unwrap_or(0.0);
            println!(
                "{{\"species\":\"{id}\",\"view\":\"{name}\",\"crown_pixels\":{covered},\"covered\":{:.4},\"mean\":{mean:.3},\"p95\":{p95:.2}}}",
                drawn as f64 / covered.max(1) as f64
            );
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();

    match &args[..] {
        [mode, dir, scale, rest @ ..] if mode == "render" => {
            draw(Path::new(dir), scale.parse()?, &pick(rest))
        }
        [mode, truth, scale, dir, rest @ ..] if mode == "compare" => compare(
            Path::new(truth),
            scale.parse()?,
            Path::new(dir),
            &pick(rest),
        ),
        _ => Err(
            "usage: crown_truth render <dir> <scale> | compare <truth dir> <scale> <dir>".into(),
        ),
    }
}

/// The species named, or every shipped one.
fn pick(rest: &[String]) -> Vec<&str> {
    if rest.is_empty() {
        SPECIES.to_vec()
    } else {
        rest.iter().map(String::as_str).collect()
    }
}
