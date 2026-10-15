//! fn-286 probe, not product code: a preset at its own age from the front,
//! from its value file on disk with rows replaced or added. Copied into
//! `crates/telperion-render/examples/relay_views.rs` by `views.sh` for a
//! run and removed after.
//!
//!   relay_views <out dir> <tag> <value file> <seeds> [<path>=<value>...]
//!
//! Stills: `<out>/<tag>-<seed>.png`, the hero shot. One JSON line per
//! tree: its build time (warm after the first), nodes and leaves.
use std::time::Instant;
use telperion_core::{pipeline::executor, presets::values, Family};
use telperion_render::{hero_pose, render, write_png, Gpu, Level, Renderer, SceneRow, View, GROUND_REACH, STILL_FORMAT};

const SIZE: (u32, u32) = (960, 720);

fn family(file: &str, sets: &[String]) -> Family {
    let mut text = std::fs::read_to_string(file).expect("the value file");
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
    let (out, tag, base) = (&args[0], &args[1], family(&args[2], &args[4..]));
    let seeds: Vec<u32> = args[3].split(',').map(|v| v.parse().unwrap()).collect();
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|e| e.to_string())?;
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    for seed in seeds {
        let mut f = base.clone();
        f.skeleton.seed = seed;
        let t = Instant::now();
        let grown = executor::grow(&f).map_err(|e| format!("{seed}: {e}"))?;
        let x = grown.expansion().map_err(|e| e.to_string())?;
        let ms = t.elapsed().as_secs_f64() * 1e3;
        let nodes = x.tree().nodes.len();
        let mesh = x.mesh().map_err(|e| e.to_string())?;
        println!(
            "{{\"tag\":\"{tag}\",\"age\":{},\"seed\":{seed},\"ms\":{ms:.0},\"nodes\":{nodes},\"leaves\":{}}}",
            f.age,
            mesh.foliage.instances.len()
        );
        let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
        let camera = hero_pose(mesh.bounds, aspect, GROUND_REACH);
        renderer.submit_at(&mesh, Level::Chosen).map_err(|e| e.to_string())?;
        renderer.set_family(&f);
        renderer.set_view(View::Whole);
        renderer.set_scene(SceneRow::default());
        let still = render(&mut renderer, &camera, SIZE.0, SIZE.1).map_err(|e| e.to_string())?;
        let file = format!("{out}/{tag}-{seed}.png");
        write_png(std::path::Path::new(&file), &still).map_err(|e| e.to_string())?;
    }
    Ok(())
}
