//! fn-236 probe, not product code: the oak from several azimuths, and its
//! projected foliage gap share. Copied into
//! `crates/telperion-render/examples/oak_views.rs` by `views.sh` for a run
//! and removed after.
//!
//!   oak_views <out dir> <ages> <seeds> [<path>=<value>...]
//!
//! The family is the preset's value file as it stands on disk, read at run
//! time with each `<path>=<value>` row replaced or added (as
//! `oak_measure.rs`), so a probe needs no rebuild.
//!
//! - Stills: the in-leaf hero shot with the tree turned by 0, 90, 180 and
//!   270 degrees (camera and sun turned together about the trunk, so the
//!   light is the hero's): `<out>/oak-<age>-<seed>-az<deg>.png`.
//! - `gaps`: in side views at 8 azimuths, each leaf drawn as a square of
//!   0.6 times its blade's length (the element's length times the canopy's
//!   size) on a 0.1 m grid; in each grid row the outline runs from the
//!   leftmost to the rightmost covered cell, and gaps is the share of
//!   outline cells no leaf covers, the mean of the 8 views.
//! - `wh`: the leaves' mean horizontal extent over the tree's height.
use std::time::Instant;
use telperion_core::{math::Vec3, pipeline::executor, presets::values, Family};
use telperion_render::{
    hero_pose, render, write_png, Camera, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH,
    STILL_FORMAT,
};

const FILE: &str = "crates/telperion-core/presets/oregon-white-oak.values";
const SIZE: (u32, u32) = (960, 720);
const SUN: (f64, f64) = (115.0, 60.0);

fn family(sets: &[String]) -> Family {
    let mut text = std::fs::read_to_string(FILE).expect("the preset's value file");
    for set in sets {
        let (path, value) = set.split_once('=').expect("<path>=<value>");
        let line = format!("{path} = {value}");
        let mut lines: Vec<String> = text.lines().map(String::from).collect();
        match lines.iter().position(|l| l.split('=').next().map(str::trim) == Some(path)) {
            Some(i) => lines[i] = line,
            None => lines.push(line),
        }
        text = lines.join("\n") + "\n";
    }
    values::read(&text).expect("the value file reads")
}

fn main() -> Result<(), String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let list = |i: usize| -> Vec<u32> { args[i].split(',').map(|v| v.parse().unwrap()).collect() };
    let (out, base) = (&args[0], family(&args[3..]));
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for &age in &list(1) {
        for &seed in &list(2) {
            let mut f = base.clone();
            f.age = f64::from(age);
            f.skeleton.seed = seed;
            let t = Instant::now();
            let grown = executor::grow(&f).map_err(|e| format!("{age} {seed}: {e}"))?;
            let x = grown.expansion().map_err(|e| e.to_string())?;
            let ms = t.elapsed().as_secs_f64() * 1e3;
            let nodes = x.tree().nodes.len();
            let mesh = x.mesh().map_err(|e| e.to_string())?;
            let leaves = &mesh.foliage.instances;
            let points: Vec<Vec3> = (0..leaves.len()).map(|i| leaves.position(i)).collect();
            let side = 0.6 * f.element.length * f.canopy.size;
            let gaps: f64 = (0..8).map(|k| gap_share(&points, k as f64 * 45.0, side)).sum::<f64>() / 8.0;
            let b = mesh.bounds;
            let (lo, hi) = points.iter().fold(
                (Vec3::new(f64::MAX, 0.0, f64::MAX), Vec3::new(f64::MIN, 0.0, f64::MIN)),
                |(l, h), p| (Vec3::new(l.x.min(p.x), 0.0, l.z.min(p.z)), Vec3::new(h.x.max(p.x), 0.0, h.z.max(p.z))),
            );
            let height = b.max.y - b.min.y;
            let width = 0.5 * ((hi.x - lo.x) + (hi.z - lo.z));
            println!(
                "{{\"age\":{age},\"seed\":{seed},\"ms\":{ms:.0},\"nodes\":{nodes},\"leaves\":{},\"height\":{height:.1},\"width\":{width:.1},\"wh\":{:.2},\"gaps\":{gaps:.3}}}",
                leaves.len(),
                width / height
            );
            let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
            let hero = hero_pose(mesh.bounds, aspect, GROUND_REACH);
            for turn in [0.0f64, 90.0, 180.0, 270.0] {
                let camera = turned(&hero, turn);
                renderer.submit_at(&mesh, Level::Chosen).map_err(|e| e.to_string())?;
                renderer.set_family(&f);
                renderer.set_view(View::Whole);
                renderer.set_scene(SceneRow {
                    sun_azimuth: SUN.0 + turn,
                    sun_elevation: SUN.1,
                    ..SceneRow::default()
                });
                let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
                let file = format!("{out}/oak-{age}-{seed}-az{turn:.0}.png");
                write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
            }
        }
    }
    Ok(())
}

/// The hero camera turned by `degrees` about the vertical through the trunk.
fn turned(hero: &Camera, degrees: f64) -> Camera {
    let (s, c) = degrees.to_radians().sin_cos();
    let spin = |p: Vec3| Vec3::new(p.x * c + p.z * s, p.y, -p.x * s + p.z * c);
    Camera {
        position: spin(hero.position),
        target: spin(hero.target),
        ..*hero
    }
}

/// The share of the foliage outline no leaf covers, seen from `azimuth`.
fn gap_share(points: &[Vec3], azimuth: f64, side: f64) -> f64 {
    use std::collections::{BTreeMap, HashSet};
    const CELL: f64 = 0.1;
    let (s, c) = azimuth.to_radians().sin_cos();
    let half = (side / 2.0 / CELL).ceil() as i64;
    let mut cells = HashSet::new();
    for p in points {
        let (u, v) = (((p.x * c - p.z * s) / CELL) as i64, (p.y / CELL) as i64);
        for du in -half..half.max(1) {
            for dv in -half..half.max(1) {
                cells.insert((u + du, v + dv));
            }
        }
    }
    let mut rows: BTreeMap<i64, (i64, i64)> = BTreeMap::new();
    for &(u, v) in &cells {
        let r = rows.entry(v).or_insert((u, u));
        *r = (r.0.min(u), r.1.max(u));
    }
    let outline: i64 = rows.values().map(|(a, b)| b - a + 1).sum();
    1.0 - cells.len() as f64 / outline as f64
}
