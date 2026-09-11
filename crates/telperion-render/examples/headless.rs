//! The native still: one tree from an id and a seed, rendered offscreen at the
//! hero pose and written as a PNG. With a second preset it is the walk between
//! the two instead, one blended family per frame as a numbered sequence. The
//! only place in the renderer that resolves a named family to parameters.
// A sibling file rather than another example: `examples/*.rs` is a target
// apiece, so the walk lives one directory down and is named by path.
#[path = "headless/walk.rs"]
mod walk;

use std::path::{Path, PathBuf};

use telperion_core::{
    blend,
    mesh::{self, Detail},
    presets::{Family, Preset},
};
use telperion_render::{
    hero_pose, measure, measure_orbit, render, walk_pose, write_png, Camera, Frame, Gpu, Level,
    Renderer, GROUND_REACH, STILL_FORMAT,
};

use walk::{Arguments, Schedule, FPS};

fn run() -> Result<(), String> {
    let arguments = walk::parse(std::env::args().skip(1))?;
    let preset = Preset::from_id(&arguments.preset)
        .ok_or_else(|| format!("unknown tree \"{}\"", arguments.preset))?;
    let mut family = preset.parameters();
    family.skeleton.seed = arguments.seed;
    if let Some(id) = &arguments.to {
        let mut far = Preset::from_id(id)
            .ok_or_else(|| format!("unknown tree \"{id}\""))?
            .parameters();
        far.skeleton.seed = arguments.seed;
        return transition(&arguments, family, far);
    }

    let tree = mesh::build(&family, Detail::Full).map_err(|error| error.to_string())?;
    let level = level_of(arguments.level, tree.foliage.element.levels.len())?;
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|error| error.to_string())?;
    let adapter = gpu.adapter.name.clone();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let submitted = renderer
        .submit_at(&tree, level)
        .map_err(|error| error.to_string())?;
    renderer.set_material(family.material);
    renderer.set_view(arguments.view);
    renderer.set_scene(arguments.scene);

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
        "{} {}x{} on {adapter} at {} samples a pixel: {} wood vertices, {} wood triangles, \
         {} foliage instances; {} triangles and {} instances drawn in {} calls",
        arguments.out.display(),
        width,
        height,
        renderer.samples(),
        submitted.wood_vertices,
        submitted.wood_triangles,
        submitted.foliage_instances,
        still.stats.triangles,
        still.stats.instances,
        still.stats.draw_calls,
    );
    Ok(())
}

/// The walk between two families as a numbered PNG sequence. Every frame is one
/// point between the two rows at the one seed, built and uploaded on its own.
/// A frame count holds the camera at the hero pose of the first frame's bounds,
/// so what moves is the tree and not the shot. A walk in seconds eases the
/// camera between the two ends' own hero poses instead, drifting round the
/// subject as it goes, so neither tree is framed for the other one.
fn transition(arguments: &Arguments, from: Family, to: Family) -> Result<(), String> {
    let (directory, stem) = sequence(&arguments.out)?;
    let (width, height) = arguments.size;
    let aspect = f64::from(width) / f64::from(height);
    let gpu = pollster::block_on(Gpu::request(None)).map_err(|error| error.to_string())?;
    let adapter = gpu.adapter.name.clone();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    renderer.set_view(arguments.view);
    renderer.set_scene(arguments.scene);
    // A walk needs the pose of the end it has not reached yet, so both ends are
    // built and framed before the first frame is drawn.
    let ends = match arguments.schedule {
        Schedule::Walk(_) => Some((
            pose_of(&mut renderer, &from, arguments, aspect)?,
            pose_of(&mut renderer, &to, arguments, aspect)?,
        )),
        Schedule::Frames(_) => None,
    };
    let frames = arguments.schedule.frames();
    let mut fixed: Option<Camera> = None;
    for frame in 0..frames {
        let step = arguments.schedule.at(frame);
        let at = step.blend;
        let family = blend::families(&from, &to, at).map_err(|error| error.to_string())?;
        let tree = mesh::build(&family, Detail::Full)
            .map_err(|error| format!("frame {frame} at {at}: {error}"))?;
        let level = level_of(arguments.level, tree.foliage.element.levels.len())?;
        renderer
            .submit_at(&tree, level)
            .map_err(|error| format!("frame {frame}: {error}"))?;
        renderer.set_material(family.material);
        if ends.is_none() && fixed.is_none() {
            let bounds = renderer.bounds().ok_or("nothing was submitted to frame")?;
            fixed = Some(hero_pose(bounds, aspect, GROUND_REACH));
        }
        let pose = match &ends {
            Some((near, far)) => walk_pose(near, far, at, step.azimuth),
            None => *fixed.as_ref().expect("the pose the first frame fixed"),
        };
        let still =
            render(&mut renderer, &pose, width, height).map_err(|error| error.to_string())?;
        write_png(&frame_path(&directory, &stem, frame), &still)
            .map_err(|error| error.to_string())?;
    }
    let encoder = encode(&directory, &stem);
    write(
        &directory.join("transition.json"),
        &record(arguments, &encoder),
    )?;
    println!(
        "{} {width}x{height} on {adapter}: {} to {} at seed {}, {frames} frames at {FPS} fps \
         ({:.1} s); {encoder}",
        frame_path(&directory, &stem, 0).display(),
        arguments.preset,
        arguments.to.as_deref().unwrap_or("nowhere"),
        arguments.seed,
        arguments.schedule.seconds(),
    );
    Ok(())
}

/// The hero pose of one end of a walk: that family built and put on stage on
/// its own, so the bounds are the ones this view frames, and the pose solved
/// from them. What is on stage is replaced by the first frame straight after.
fn pose_of(
    renderer: &mut Renderer,
    family: &Family,
    arguments: &Arguments,
    aspect: f64,
) -> Result<Camera, String> {
    let tree = mesh::build(family, Detail::Full).map_err(|error| error.to_string())?;
    let level = level_of(arguments.level, tree.foliage.element.levels.len())?;
    renderer
        .submit_at(&tree, level)
        .map_err(|error| error.to_string())?;
    let bounds = renderer.bounds().ok_or("nothing was submitted to frame")?;
    Ok(hero_pose(bounds, aspect, GROUND_REACH))
}

/// The directory a sequence is written into and the name its frames carry:
/// `--out .../walk/frame.png` writes `.../walk/frame-0001.png` onward.
fn sequence(out: &Path) -> Result<(PathBuf, String), String> {
    let directory = out
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .to_path_buf();
    let stem = out
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| format!("--out {} names no frame", out.display()))?;
    Ok((directory, stem.to_owned()))
}

fn frame_path(directory: &Path, stem: &str, frame: u32) -> PathBuf {
    directory.join(format!("{stem}-{:04}.png", frame + 1))
}

/// The sequence assembled by the system encoder, when the machine has one. The
/// frames are the artefact and the video is the convenience, so a machine with
/// no `ffmpeg` is told once and keeps its sequence; nothing here fails the run.
fn encode(directory: &Path, stem: &str) -> String {
    let video = directory.join("transition.mp4");
    let rate = FPS.to_string();
    let assembled = std::process::Command::new("ffmpeg")
        .args(["-y", "-framerate", rate.as_str(), "-i"])
        .arg(directory.join(format!("{stem}-%04d.png")))
        .args(["-c:v", "libx264", "-pix_fmt", "yuv420p"])
        .arg(&video)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status();
    match assembled {
        Ok(status) if status.success() => video.display().to_string(),
        Ok(status) => format!("the encoder refused the sequence ({status}); the frames stand"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            "no ffmpeg on the path; the frames stand".to_owned()
        }
        Err(error) => format!("the encoder could not be run ({error}); the frames stand"),
    }
}

/// What the sequence is, beside it: both ends of the walk, the seed both were
/// read at, the frame size, how many frames and at what rate, what the encoder
/// did, and - when the sequence was walked in seconds rather than counted in
/// frames - the walk that shaped it.
fn record(arguments: &Arguments, encoder: &str) -> String {
    let quoted = |text: &str| text.replace('\\', "\\\\").replace('"', "\\\"");
    let walked = match arguments.schedule {
        Schedule::Frames(_) => String::new(),
        Schedule::Walk(walk) => format!(
            "  \"walk\": {},\n  \"hold\": {},\n  \"sweep\": {},\n  \"ease\": \"smoothstep\",\n",
            walk.seconds, walk.hold, walk.sweep,
        ),
    };
    format!(
        "{{\n  \"from\": \"{}\",\n  \"to\": \"{}\",\n  \"seed\": {},\n  \
         \"size\": [{}, {}],\n  \"frames\": {},\n  \"fps\": {FPS},\n{}  \
         \"encoder\": \"{}\"\n}}\n",
        quoted(&arguments.preset),
        quoted(arguments.to.as_deref().unwrap_or("nowhere")),
        arguments.seed,
        arguments.size.0,
        arguments.size.1,
        arguments.schedule.frames(),
        walked,
        quoted(encoder),
    )
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
    let frame = Frame::new(renderer, "timing", size);
    let session = if orbit { measure_orbit } else { measure };
    session(renderer, camera, size, frame.target()).map_err(|error| error.to_string())
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
    let mut line = format!(
        "{} samples a pixel; vegetation p50 {median:.3} ms, p95 {tail:.3} ms",
        report.multisample()
    );
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
