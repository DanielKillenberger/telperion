//! fn-246's bark ground truth: every bark test view drawn at its size and
//! at four times it, the wood surfaced for the smaller size in both so only
//! the shading differs, written as raw RGBA. A second call compares one
//! shader's draws against another's four-times draw, box-reduced, over the
//! wood the clay room paints, as mean and p95 in code values.
//!
//!   cargo run --profile ci -p telperion-render --example bark_truth -- render <dir>
//!   cargo run --profile ci -p telperion-render --example bark_truth -- compare <truth dir> <dir>
use std::path::Path;
use telperion_core::{math::Vec3, mesh, presets::Preset, Family};
use telperion_render::{
    hero_pose, render, shot_pose, Camera, Gpu, Renderer, Shot, Still, View, GROUND_REACH,
    STILL_FORMAT,
};

const SCALE: u32 = 4;

/// One test view: its family, its camera and the size it is judged at.
struct Fixture {
    name: &'static str,
    family: Family,
    camera: fn(&Renderer, (u32, u32)) -> Camera,
    size: (u32, u32),
}

fn seeded(preset: Preset, seed: u32, age: Option<f64>) -> Family {
    let mut family = preset.parameters();
    family.skeleton.seed = seed;
    if let Some(age) = age {
        family.age = age;
    }
    if family.space.species.is_none() {
        family.space = Preset::EuropeanBeech.parameters().space;
    }
    family
}

fn trunk(target: Vec3, at: f64, fov: f64) -> Camera {
    let side = Vec3::new(1.7307636095778745, 0.2967023330704928, -1.7307636095778745);
    Camera {
        position: Vec3::new(0.0, target.y, 0.0) + side * at,
        target,
        field_of_view: fov,
        near: 0.01,
        far: 1000.0,
    }
}

fn shot(renderer: &Renderer, size: (u32, u32), text: &str) -> Camera {
    let aspect = f64::from(size.0) / f64::from(size.1);
    shot_pose(
        renderer.bounds().unwrap(),
        aspect,
        GROUND_REACH,
        &Shot::parse(text).unwrap(),
    )
}

/// The frame probe's close pose: the hero pose at a quarter of its distance.
fn close(renderer: &Renderer, size: (u32, u32)) -> Camera {
    let aspect = f64::from(size.0) / f64::from(size.1);
    let hero = hero_pose(renderer.bounds().unwrap(), aspect, GROUND_REACH);
    Camera {
        position: hero.target + (hero.position - hero.target) * 0.25,
        ..hero
    }
}

fn fixtures() -> Vec<Fixture> {
    let oak = || seeded(Preset::OregonWhiteOak, 7, None);
    let wide = (800, 500);
    let harness = (480, 270);
    vec![
        Fixture {
            name: "oak-trunk",
            family: oak(),
            size: wide,
            camera: |_, _| trunk(Vec3::new(0.0, 2.0, 0.0), 1.0, 38.0),
        },
        Fixture {
            name: "oak-grazing",
            family: oak(),
            size: wide,
            camera: |_, _| trunk(Vec3::new(-0.24, 2.0, -0.24), 1.0, 20.0),
        },
        Fixture {
            name: "spruce-grazing",
            family: seeded(Preset::NorwaySpruce, 7, None),
            size: wide,
            camera: |_, _| trunk(Vec3::new(-0.24, 0.9, -0.24), 1.0, 20.0),
        },
        Fixture {
            name: "oak-2x",
            family: oak(),
            size: wide,
            camera: |_, _| trunk(Vec3::new(0.0, 2.0, 0.0), 2.0, 38.0),
        },
        Fixture {
            name: "oak-4x",
            family: oak(),
            size: wide,
            camera: |_, _| trunk(Vec3::new(0.0, 2.0, 0.0), 4.0, 38.0),
        },
        Fixture {
            name: "beech-base",
            family: seeded(Preset::EuropeanBeech, 1, Some(80.0)),
            size: wide,
            camera: |r, s| {
                shot(
                    r,
                    s,
                    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.04,"fov":45,"distance":1.2}"#,
                )
            },
        },
        Fixture {
            name: "birch-bark",
            family: seeded(Preset::SilverBirch, 1, Some(80.0)),
            size: wide,
            camera: |r, s| {
                shot(
                    r,
                    s,
                    r#"{"azimuth":30,"elevation":0,"fill":1,"targetHeight":0.06,"fov":45,"distance":1.5}"#,
                )
            },
        },
        Fixture {
            name: "oak-close",
            family: seeded(Preset::OregonWhiteOak, 1, None),
            size: harness,
            camera: close,
        },
        Fixture {
            name: "spruce-close",
            family: seeded(Preset::NorwaySpruce, 1, None),
            size: harness,
            camera: close,
        },
        Fixture {
            name: "birch-close",
            family: seeded(Preset::SilverBirch, 1, None),
            size: harness,
            camera: close,
        },
        Fixture {
            name: "telperion-close",
            family: seeded(Preset::Telperion, 1, None),
            size: harness,
            camera: close,
        },
    ]
}

fn write(dir: &Path, name: &str, still: &Still) -> std::io::Result<()> {
    std::fs::write(dir.join(name), &still.rgba)
}

fn draw(dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(dir)?;
    let mut renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    for f in fixtures() {
        let tree = mesh::build(&f.family)?;
        renderer.submit(&tree)?;
        renderer.set_family(&f.family);
        renderer.set_figure(false);
        let camera = (f.camera)(&renderer, f.size);
        renderer.pin_curve_viewport(Some(f.size));
        renderer.set_view(View::Clay);
        write(
            dir,
            &format!("{}-clay", f.name),
            &render(&mut renderer, &camera, f.size.0, f.size.1)?,
        )?;
        renderer.set_view(View::Bare);
        write(
            dir,
            &format!("{}-1", f.name),
            &render(&mut renderer, &camera, f.size.0, f.size.1)?,
        )?;
        let (w, h) = (f.size.0 * SCALE, f.size.1 * SCALE);
        write(
            dir,
            &format!("{}-{SCALE}", f.name),
            &render(&mut renderer, &camera, w, h)?,
        )?;
        renderer.pin_curve_viewport(None);
        eprintln!("drew {}", f.name);
    }
    Ok(())
}

/// Pixels the clay room paints as wood, clear of every silhouette by four
/// pixels: red over blue (`tests/common/resolution.rs`).
fn mask(clay: &[u8], (w, h): (u32, u32)) -> Vec<(usize, usize)> {
    let (w, h) = (w as usize, h as usize);
    let warm = |x: usize, y: usize| clay[(y * w + x) * 4] > clay[(y * w + x) * 4 + 2];
    (4..h - 4)
        .flat_map(|y| (4..w - 4).map(move |x| (x, y)))
        .filter(|&(x, y)| (y - 4..=y + 4).all(|sy| (x - 4..=x + 4).all(|sx| warm(sx, sy))))
        .collect()
}

/// Mean and p95 code-value distance of `low` from `high` box-reduced, and
/// the signed mean (`low` over the truth): what an expected term can move.
fn agreement(high: &[u8], low: &[u8], width: u32, wood: &[(usize, usize)]) -> (f64, f64, f64) {
    let (lw, hw, s) = (width as usize, (width * SCALE) as usize, SCALE as usize);
    let mut errors = Vec::with_capacity(wood.len() * 3);
    for &(x, y) in wood {
        for c in 0..3 {
            let sum: u32 = (0..s)
                .flat_map(|dy| (0..s).map(move |dx| (dx, dy)))
                .map(|(dx, dy)| u32::from(high[((y * s + dy) * hw + x * s + dx) * 4 + c]))
                .sum();
            let truth = f64::from(sum) / (s * s) as f64;
            errors.push(f64::from(low[(y * lw + x) * 4 + c]) - truth);
        }
    }
    let n = errors.len().max(1) as f64;
    let bias = errors.iter().sum::<f64>() / n;
    let mut errors: Vec<f64> = errors.iter().map(|e| e.abs()).collect();
    let mean = errors.iter().sum::<f64>() / n;
    errors.sort_by(f64::total_cmp);
    (
        mean,
        errors.get(errors.len() * 95 / 100).copied().unwrap_or(0.0),
        bias,
    )
}

fn compare(truth: &Path, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    for f in fixtures() {
        let read = |d: &Path, s: &str| std::fs::read(d.join(format!("{}-{s}", f.name)));
        let wood = mask(&read(truth, "clay")?, f.size);
        let high = read(truth, &SCALE.to_string())?;
        let (mean, p95, bias) = agreement(&high, &read(dir, "1")?, f.size.0, &wood);
        let own = agreement(
            &read(dir, &SCALE.to_string())?,
            &read(dir, "1")?,
            f.size.0,
            &wood,
        );
        println!(
            "{{\"fixture\":\"{}\",\"pixels\":{},\"mean\":{mean:.3},\"p95\":{p95:.2},\"bias\":{bias:.3},\"own_mean\":{:.3},\"own_p95\":{:.2}}}",
            f.name,
            wood.len(),
            own.0,
            own.1
        );
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match &args[..] {
        [mode, dir] if mode == "render" => draw(Path::new(dir)),
        [mode, truth, dir] if mode == "compare" => compare(Path::new(truth), Path::new(dir)),
        _ => Err("usage: bark_truth render <dir> | compare <truth dir> <dir>".into()),
    }
}
