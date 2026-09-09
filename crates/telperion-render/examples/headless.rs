//! The native still: one tree from an id and a seed, rendered offscreen at the
//! hero pose and written as a PNG. The only place in the renderer that resolves
//! a named family to parameters.
use std::path::PathBuf;

use telperion_core::{
    mesh::{self, Detail},
    presets::Preset,
};
use telperion_render::{
    attachment, hero_pose, measure, measure_orbit, render, write_png, Gpu, Level, Renderer, View,
    DEPTH_FORMAT, GROUND_REACH, STILL_FORMAT,
};

const USAGE: &str = "usage: headless --preset <id> --seed <n> --out <png> [--size WxH] \
                     [--view whole|bare|leaf] [--level <n>] [--timing <json>] [--orbit]";

struct Arguments {
    preset: String,
    seed: u32,
    out: PathBuf,
    size: (u32, u32),
    view: View,
    /// The level every leaf the frame shows is held at, judged against the
    /// element once there is an element to judge it against.
    level: Option<u32>,
    timing: Option<PathBuf>,
    /// Whether the timing session turns the camera once around the hero pose
    /// instead of holding it still. The still beside it is always the hero
    /// pose: the orbit is what is measured, not what is judged.
    orbit: bool,
}

fn parse() -> Result<Arguments, String> {
    let (mut preset, mut seed, mut out, mut size) = (None, None, None, (1024u32, 1024u32));
    let (mut view, mut level, mut timing) = (View::default(), None, None);
    let mut orbit = false;
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
            "--timing" => timing = Some(PathBuf::from(value()?)),
            "--orbit" => orbit = true,
            "--size" => {
                let raw = value()?;
                size = parse_size(&raw)?;
            }
            "--view" => {
                let raw = value()?;
                view = View::from_id(&raw).ok_or_else(|| {
                    format!("unknown view \"{raw}\"; one of {}", View::NAMES.join(", "))
                })?;
            }
            "--level" => {
                let raw = value()?;
                level = Some(
                    raw.parse::<u32>()
                        .map_err(|_| format!("--level wants a level number, not \"{raw}\""))?,
                );
            }
            other => return Err(format!("unknown argument \"{other}\"\n{USAGE}")),
        }
    }
    Ok(Arguments {
        preset: preset.ok_or(format!("--preset is required\n{USAGE}"))?,
        seed: seed.ok_or(format!("--seed is required\n{USAGE}"))?,
        out: out.ok_or(format!("--out is required\n{USAGE}"))?,
        size,
        view,
        level,
        timing,
        orbit,
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
    let level = level_of(arguments.level, tree.foliage.element.levels.len())?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|error| error.to_string())?;
    let adapter = gpu.adapter.name.clone();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer
        .submit_at(&tree, level)
        .map_err(|error| error.to_string())?;
    renderer.set_view(arguments.view);

    let (width, height) = arguments.size;
    // The leaf view frames the element, the others the whole tree; the
    // renderer knows which, so the pose is solved on whatever is on stage.
    let bounds = renderer.bounds().ok_or("nothing was submitted to frame")?;
    let camera = hero_pose(bounds, f64::from(width) / f64::from(height), GROUND_REACH);
    let still = render(&mut renderer, &camera, width, height).map_err(|error| error.to_string())?;
    write_png(&arguments.out, &still).map_err(|error| error.to_string())?;
    if let Some(path) = &arguments.timing {
        let report = time(&mut renderer, &camera, arguments.size, arguments.orbit)?;
        write(path, &report.to_json())?;
        println!(
            "{}: {} - {}",
            path.display(),
            report.verdict().name(),
            detail(&report)
        );
    }

    println!(
        "{} {}x{} on {adapter}: {} wood vertices, {} wood triangles, \
         {} foliage instances; {} triangles and {} instances drawn in {} calls",
        arguments.out.display(),
        width,
        height,
        submitted.wood_vertices,
        submitted.wood_triangles,
        submitted.foliage_instances,
        still.stats.triangles,
        still.stats.instances,
        still.stats.draw_calls,
    );
    Ok(())
}

/// The level the crown is held at, or the choice made per frame. A number the
/// element has no level for is refused by name, with the range it has.
fn level_of(asked: Option<u32>, levels: usize) -> Result<Level, String> {
    match asked {
        None => Ok(Level::Chosen),
        Some(level) if (level as usize) < levels => Ok(Level::Forced(level)),
        Some(level) if levels == 0 => Err(format!(
            "--level {level}: this tree's element carries no levels"
        )),
        Some(level) => Err(format!(
            "--level {level} is out of range: 0 (coarsest) to {} (finest)",
            levels - 1
        )),
    }
}

/// Measures the selection and vegetation passes of the tree already on stage,
/// into targets of the still's own size so the number belongs to the picture
/// beside it. An orbit session turns the camera one full revolution around the
/// hero pose while it measures; a plain one holds it still.
fn time(
    renderer: &mut Renderer,
    camera: &telperion_render::Camera,
    size: (u32, u32),
    orbit: bool,
) -> Result<telperion_render::Report, String> {
    let colour = attachment(renderer.gpu(), "timing", STILL_FORMAT, size);
    let depth = attachment(renderer.gpu(), "timing depth", DEPTH_FORMAT, size);
    let (colour, depth) = (
        colour.create_view(&Default::default()),
        depth.create_view(&Default::default()),
    );
    let session = if orbit { measure_orbit } else { measure };
    session(renderer, camera, size, &colour, &depth).map_err(|error| error.to_string())
}

/// The one line a run says about its session: what the passes cost, or why
/// there is no number to say.
fn detail(report: &telperion_render::Report) -> String {
    let (Some(median), Some(tail)) = (report.p50_ms(), report.p95_ms()) else {
        return report
            .verdict()
            .reason()
            .unwrap_or("no reason given")
            .to_owned();
    };
    let mut line = format!("vegetation p50 {median:.3} ms, p95 {tail:.3} ms");
    if let (Some(select), Some(total)) = (report.selection_p50_ms(), report.total_p50_ms()) {
        line += &format!("; selection p50 {select:.3} ms, together p50 {total:.3} ms");
    }
    if let (Some(wall), Some(worst)) = (report.wall_p50_ms(), report.wall_max_ms()) {
        line += &format!("; wall p50 {wall:.2} ms, worst {worst:.2} ms");
    }
    line
}

/// Writes a small text record where it was asked for, naming the path when it
/// cannot; the still's own writer is for pixels.
fn write(path: &std::path::Path, text: &str) -> Result<(), String> {
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    }
    std::fs::write(path, text).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

fn main() {
    if let Err(reason) = run() {
        eprintln!("{reason}");
        std::process::exit(1);
    }
}
