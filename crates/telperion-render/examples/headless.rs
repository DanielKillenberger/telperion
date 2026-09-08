//! The native still: one tree from an id and a seed, rendered offscreen at the
//! hero pose and written as a PNG. The only place in the renderer that resolves
//! a named family to parameters.
use std::path::PathBuf;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{hero_pose, render, write_png, Gpu, Renderer, GROUND_REACH, STILL_FORMAT};

const USAGE: &str = "usage: headless --preset <id> --seed <n> --out <png> [--size WxH]";

struct Arguments {
    preset: String,
    seed: u32,
    out: PathBuf,
    size: (u32, u32),
}

fn parse() -> Result<Arguments, String> {
    let (mut preset, mut seed, mut out, mut size) = (None, None, None, (1024u32, 1024u32));
    let mut args = std::env::args().skip(1);
    while let Some(flag) = args.next() {
        let mut value = || args.next().ok_or(format!("{flag} needs a value\n{USAGE}"));
        match flag.as_str() {
            "--preset" => preset = Some(value()?),
            "--seed" => {
                let raw = value()?;
                seed = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("--seed wants a whole number, not \"{raw}\""))?,
                );
            }
            "--out" => out = Some(PathBuf::from(value()?)),
            "--size" => {
                let raw = value()?;
                size = parse_size(&raw)?;
            }
            other => return Err(format!("unknown argument \"{other}\"\n{USAGE}")),
        }
    }
    Ok(Arguments {
        preset: preset.ok_or(format!("--preset is required\n{USAGE}"))?,
        seed: seed.ok_or(format!("--seed is required\n{USAGE}"))?,
        out: out.ok_or(format!("--out is required\n{USAGE}"))?,
        size,
    })
}

fn parse_size(raw: &str) -> Result<(u32, u32), String> {
    let bad = || format!("--size wants WxH in pixels, not \"{raw}\"");
    let (width, height) = raw.split_once(['x', 'X']).ok_or_else(bad)?;
    let (width, height) = (
        width.parse::<u32>().map_err(|_| bad())?,
        height.parse::<u32>().map_err(|_| bad())?,
    );
    if width == 0 || height == 0 {
        return Err(bad());
    }
    Ok((width, height))
}

fn run() -> Result<(), String> {
    let arguments = parse()?;
    let preset = Preset::from_id(&arguments.preset)
        .ok_or_else(|| format!("unknown tree \"{}\"", arguments.preset))?;
    let mut family = preset.parameters();
    family.skeleton.seed = arguments.seed;

    let tree = mesh::build(&family, Detail::Full).map_err(|error| error.to_string())?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|error| error.to_string())?;
    let adapter = gpu.adapter.name.clone();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer.submit(&tree);

    let (width, height) = arguments.size;
    let camera = hero_pose(
        tree.bounds,
        f64::from(width) / f64::from(height),
        GROUND_REACH,
    );
    let still = render(&mut renderer, &camera, width, height).map_err(|error| error.to_string())?;
    write_png(&arguments.out, &still).map_err(|error| error.to_string())?;

    println!(
        "{} {}x{} on {adapter}: {} wood vertices, {} wood triangles, {} drawn in {} calls",
        arguments.out.display(),
        width,
        height,
        submitted.wood_vertices,
        submitted.wood_triangles,
        still.stats.triangles,
        still.stats.draw_calls,
    );
    Ok(())
}

fn main() {
    if let Err(reason) = run() {
        eprintln!("{reason}");
        std::process::exit(1);
    }
}
