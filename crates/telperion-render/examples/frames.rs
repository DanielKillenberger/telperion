//! fn-246's frame probe, extended for fn-259's scorecard: what one shipped
//! tree costs to start, to rebuild and to draw on the harness's GPU route,
//! natively at 1920x1080 with the renderer's multisampling, and what it
//! holds afterwards. Prints one JSON line.
//!
//!   cargo run --profile ci -p telperion-render --example frames -- <preset> [full|relief-off|parallax-off|no-rachis] [view...]
//!
//! - `fresh_ms`: from `Renderer::new` (the adapter and device already
//!   requested) through the programs, the first build and its first frame
//!   at the hero pose to the device's fence; shader compilation included.
//! - `warm_ms`: a second build of the same tree in the same process
//!   (`Generator::prepare` and its submission) to a fenced hero frame.
//! - `views`: frame time per view, then `peak_rss_mb` (the process's
//!   `VmHWM`) and `device_memory` with its source, read last with the tree
//!   and the generator alive, as on a page.
//!
//! Views are any of whole, mid (half the distance), close, bare-whole and
//! bare-close; all but mid by default.
//! `relief-off` draws the bark with no relief (ridge scale and depth zero),
//! `parallax-off` with its relief and no depth: the wood's shaded pass
//! measured against itself (fn-246 R1). `no-rachis` grows the flowers with
//! no rachis, the florets on their tips: what the rachises cost (fn-253).
//! `FLOWERING=<share>` sets a trial share for the short shoots, `SEED` a
//! seed in place of 1, `AGE` an age in place of the preset's.
//! `ONE_BUILD=1` builds once and draws four hero frames before reading
//! memory, for rows that only need memory: the timing keys are null.
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_render::{
    generation::Generator, hero_pose, measure, Camera, Frame, Gpu, Renderer, View, GROUND_REACH,
    STILL_FORMAT,
};

const SIZE: (u32, u32) = (1920, 1080);
const PIPELINED: u32 = 120;
/// Hero frames drawn before a one-build row reads memory (fn-258 BYTES.md).
const MEMORY_FRAMES: u32 = 4;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, rest @ ..] = &args[..] else {
        return Err(
            "usage: frames <preset> [full|relief-off|parallax-off|no-rachis] [view...]".into(),
        );
    };
    let variant = rest.first().map_or("full", String::as_str);
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.skeleton.seed = 1;
    // A trial seed, `SEED=<seed>` (fn-253).
    if let Ok(seed) = std::env::var("SEED") {
        family.skeleton.seed = seed.parse()?;
    }
    // An age in place of the preset's, `AGE=<years>` (fn-259).
    if let Ok(age) = std::env::var("AGE") {
        family.age = age.parse()?;
    }
    if variant == "no-rachis" {
        family.supernatural.bloom.rachis = Some(0.0);
    }
    // A trial flowering share for the short shoots (fn-253).
    if let Ok(share) = std::env::var("FLOWERING") {
        family.supernatural.flowering.age14 = Some(share.parse()?);
    }
    let one_build = std::env::var("ONE_BUILD").is_ok_and(|v| v == "1");
    let mut material = family.drawn_material();
    match variant {
        "full" | "no-rachis" => {}
        "relief-off" => (material.ridge_scale, material.depth_strength) = (0.0, 0.0),
        "parallax-off" => material.depth_strength = 0.0,
        other => return Err(format!("no variant {other}").into()),
    }
    let gpu = pollster::block_on(Gpu::request(None))?;
    let started = Instant::now();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let generator = Generator::new(&renderer)?;
    pollster::block_on(generator.prepare_programs())?;
    let frame = Frame::new(&renderer, "frames", SIZE);
    renderer.submit_prepared(generator.prepare(&family)?)?;
    renderer.set_family(&family);
    renderer.set_material(material);
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
    renderer.draw(&hero, SIZE, frame.target());
    wait(&renderer);
    let fresh_ms = ms(started);
    let (fresh, warm, views) = if one_build {
        for _ in 1..MEMORY_FRAMES {
            renderer.draw(&hero, SIZE, frame.target());
        }
        wait(&renderer);
        ("null".to_owned(), "null".to_owned(), "null".to_owned())
    } else {
        let rebuilt = Instant::now();
        renderer.submit_prepared(generator.prepare(&family)?)?;
        renderer.draw(&hero, SIZE, frame.target());
        wait(&renderer);
        let warm_ms = ms(rebuilt);
        let views = views(&mut renderer, &frame, hero, rest)?;
        (
            format!("{fresh_ms:.1}"),
            format!("{warm_ms:.1}"),
            format!("[{views}]"),
        )
    };
    let device = device_memory(&renderer)?;
    println!(
        "{{\"preset\":\"{id}\",\"variant\":\"{variant}\",\"age\":{},\"seed\":{},\"one_build\":{one_build},\"fresh_ms\":{fresh},\"warm_ms\":{warm},\"views\":{views},\"peak_rss_mb\":{},\"device_memory\":{device},\"load\":\"{}\"}}",
        family.age,
        family.skeleton.seed,
        peak_mb()?,
        load(),
    );
    Ok(())
}

/// Frame time at each view asked, as fn-246 R1 measured it: 20 frames to
/// settle, then 120 pipelined frames to a fence, then the renderer's own
/// timestamp protocol.
fn views(
    renderer: &mut Renderer,
    frame: &Frame,
    hero: Camera,
    rest: &[String],
) -> Result<String, Box<dyn std::error::Error>> {
    let toward = |share: f64| Camera {
        position: hero.target + (hero.position - hero.target) * share,
        ..hero
    };
    let (mid, close) = (toward(0.5), toward(0.25));
    let mut views = Vec::new();
    for (name, view, camera) in [
        ("whole", View::Whole, hero),
        ("mid", View::Whole, mid),
        ("close", View::Whole, close),
        ("bare-whole", View::Bare, hero),
        ("bare-close", View::Bare, close),
    ] {
        let asked = |v: &String| v == name;
        // The mid view is drawn only when named, so a default run is the
        // four views R1 measured.
        if (rest.len() > 1 && !rest[1..].iter().any(asked)) || (rest.len() <= 1 && name == "mid") {
            continue;
        }
        renderer.set_view(view);
        for _ in 0..20 {
            renderer.draw(&camera, SIZE, frame.target());
        }
        wait(renderer);
        let started = Instant::now();
        for _ in 0..PIPELINED {
            renderer.draw(&camera, SIZE, frame.target());
        }
        wait(renderer);
        let frame_ms = ms(started) / f64::from(PIPELINED);
        let timing = measure(renderer, &camera, SIZE, frame.target())?.to_json();
        views.push(format!(
            "{{\"view\":\"{name}\",\"frame_ms\":{frame_ms:.3},\"fps\":{:.1},\"timing\":{}}}",
            1e3 / frame_ms,
            timing.replace('\n', "")
        ));
    }
    Ok(views.join(","))
}

fn ms(since: Instant) -> f64 {
    since.elapsed().as_secs_f64() * 1e3
}

fn wait(renderer: &Renderer) {
    let _ = renderer
        .gpu()
        .device
        .poll(wgpu::PollType::wait_indefinitely());
}

/// The device memory this process holds, with its source: the allocator's
/// report where the backend gives one, else `nvidia-smi`'s figure for this
/// pid. `wgpu_internal_mib` is the part of `allocated_mib` wgpu holds for
/// itself; its indirect-draw validation buffers grow with the timing
/// loop's pipelined frames, not with the tree (measured, fn-259 task 3).
fn device_memory(renderer: &Renderer) -> Result<String, Box<dyn std::error::Error>> {
    const MIB: f64 = 1024.0 * 1024.0;
    if let Some(report) = renderer.gpu().device.generate_allocator_report() {
        let internal: u64 = report
            .allocations
            .iter()
            .filter(|a| a.name.starts_with("(wgpu internal)"))
            .map(|a| a.size)
            .sum();
        return Ok(format!(
            "{{\"source\":\"wgpu allocator report\",\"allocated_mib\":{:.1},\"wgpu_internal_mib\":{:.1},\"reserved_mib\":{:.1}}}",
            report.total_allocated_bytes as f64 / MIB,
            internal as f64 / MIB,
            report.total_reserved_bytes as f64 / MIB
        ));
    }
    let out = std::process::Command::new("nvidia-smi")
        .args([
            "--query-compute-apps=pid,used_memory",
            "--format=csv,noheader,nounits",
        ])
        .output()?;
    let pid = std::process::id().to_string();
    let used = String::from_utf8(out.stdout)?
        .lines()
        .filter_map(|l| l.split_once(", "))
        .find(|(p, _)| *p == pid)
        .map(|(_, mib)| mib.trim().to_owned())
        .ok_or("no device memory: no allocator report and no nvidia-smi row for this pid")?;
    Ok(format!(
        "{{\"source\":\"nvidia-smi compute-apps\",\"used_mib\":{used}}}"
    ))
}

/// The process's peak resident memory in MB, from `/proc/self/status`.
fn peak_mb() -> Result<u64, Box<dyn std::error::Error>> {
    let status = std::fs::read_to_string("/proc/self/status")?;
    let line = status
        .lines()
        .find(|l| l.starts_with("VmHWM:"))
        .ok_or("no VmHWM")?;
    let kb: u64 = line.split_whitespace().nth(1).ok_or("no VmHWM")?.parse()?;
    Ok(kb / 1024)
}

/// The one-minute load average when the views were measured.
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_default()
}
