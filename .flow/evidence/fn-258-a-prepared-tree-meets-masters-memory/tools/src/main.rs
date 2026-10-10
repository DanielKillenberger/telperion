//! fn-258 probe: one preset, one seed, on the harness's GPU route
//! (`Generator::prepare`, resident delivery, `submit_prepared`, first draw),
//! with the device allocator's bytes grouped by buffer label and the GPU
//! time of each pass at whole, mid and close. Built against master (no
//! feature) and against the fn-258 worktree (`f4`).
//!
//!   probe <preset> <seed> [--age A] [--stills DIR]
use std::collections::BTreeMap;
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_render::{
    generation::{Delivery, Generator},
    hero_pose, measure, measure_orbit, render, write_png, Camera, Frame, Gpu, Renderer, View, GROUND_REACH,
    STILL_FORMAT,
};

const SIZE: (u32, u32) = (1920, 1080);
const PIPELINED: u32 = 120;
const ROUNDS: usize = 3;

#[cfg(feature = "f4")]
const VERSION: &str = "new";
#[cfg(not(feature = "f4"))]
const VERSION: &str = "old";

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let id = args.first().ok_or("preset")?;
    let seed: u32 = args.get(1).ok_or("seed")?.parse()?;
    let flag = |name: &str| {
        args.iter()
            .position(|a| a == name)
            .and_then(|i| args.get(i + 1).cloned())
    };
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.skeleton.seed = seed;
    if let Some(age) = flag("--age") {
        family.age = age.parse()?;
    }

    let gpu = pollster::block_on(Gpu::request(None))?;
    let stage_device = gpu_process_mb();
    let mut renderer = Renderer::new(gpu, STILL_FORMAT);
    let stage_renderer = gpu_process_mb();
    let generator = Generator::new(&renderer)?;
    let stage_generator = gpu_process_mb();
    #[cfg(feature = "f4")]
    pollster::block_on(generator.prepare_programs())?;
    let stage_programs = gpu_process_mb();
    let frame = Frame::new(&renderer, "probe", SIZE);
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let base = allocations(&renderer);

    let build = |renderer: &mut Renderer| -> Result<f64, Box<dyn std::error::Error>> {
        let t = Instant::now();
        let prepared = generator.prepare(&family, Delivery::Resident)?;
        renderer.submit_prepared(prepared)?;
        #[cfg(feature = "f4")]
        renderer.set_family(&family);
        #[cfg(not(feature = "f4"))]
        renderer.set_material(family.material);
        renderer.set_view(View::Whole);
        let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
        renderer.draw(&hero, SIZE, frame.target());
        wait(renderer);
        Ok(ms(t))
    };

    let first = build(&mut renderer)?;
    let after_first = allocations(&renderer);
    let first_process_mb = gpu_process_mb();
    if let Some(dir) = flag("--stills") {
        let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
        for (name, camera) in views(hero) {
            let still = render(&mut renderer, &camera, SIZE.0, SIZE.1)?;
            let path = std::path::PathBuf::from(&dir)
                .join(format!("{id}-{seed}-{}-{name}-{VERSION}.png", family.age));
            write_png(&path, &still)?;
        }
        println!("{{\"version\":\"{VERSION}\",\"preset\":\"{id}\",\"seed\":{seed},\"age\":{}}}", family.age);
        return Ok(());
    }
    if args.iter().any(|a| a == "--orbit") {
        let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
        let mut rows = Vec::new();
        for (name, camera) in views(hero) {
            let r = measure_orbit(&mut renderer, &camera, SIZE, frame.target())?;
            #[cfg(feature = "f4")]
            let surfacing = r.surfacing_p50_ms();
            #[cfg(not(feature = "f4"))]
            let surfacing: Option<f64> = None;
            rows.push(format!(
                "\"{name}\":{{\"verdict\":\"{}\",\"vegetation\":{},\"selection\":{},\"shadow\":{},\"surfacing\":{},\"total\":{},\"wall\":{}}}",
                r.verdict().name(),
                opt(r.p50_ms()),
                opt(r.selection_p50_ms()),
                opt(r.shadow_p50_ms()),
                opt(surfacing),
                opt(r.total_p50_ms()),
                opt(r.wall_p50_ms()),
            ));
        }
        println!(
            "{{\"version\":\"{VERSION}\",\"preset\":\"{id}\",\"seed\":{seed},\"age\":{},\"orbit\":{{{}}}}}",
            family.age,
            rows.join(",")
        );
        return Ok(());
    }
    let mut builds = Vec::new();
    for _ in 0..ROUNDS {
        builds.push(build(&mut renderer)?);
    }
    let resident = allocations(&renderer);
    let rss_before_trim = status_mb("VmRSS:");
    unsafe { malloc_trim(0) };
    let rss_trimmed = status_mb("VmRSS:");
    let tree_bytes = Generator::tree_buffer_bytes(&renderer);
    let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
    renderer.set_view(View::Whole);
    let mut fps = Vec::new();
    let mut passes = Vec::new();
    let mut curves: Vec<String> = Vec::new();
    for (name, camera) in views(hero) {
        let mut best = f64::INFINITY;
        for _ in 0..ROUNDS {
            for _ in 0..20 {
                renderer.draw(&camera, SIZE, frame.target());
            }
            wait(&renderer);
            let t = Instant::now();
            for _ in 0..PIPELINED {
                renderer.draw(&camera, SIZE, frame.target());
            }
            wait(&renderer);
            best = best.min(ms(t) / f64::from(PIPELINED));
        }
        fps.push(format!("\"{name}\":{best:.3}"));
        let report = measure(&mut renderer, &camera, SIZE, frame.target())?;
        #[cfg(feature = "f4")]
        {
            renderer.draw(&camera, SIZE, frame.target());
            wait(&renderer);
            curves.push(format!(
                "\"{name}\":{{\"camera\":{},\"sun\":{}}}",
                curve(renderer.curve_report()),
                curve(renderer.curve_sun_report())
            ));
        }
        #[cfg(feature = "f4")]
        let surfacing = report.surfacing_p50_ms();
        #[cfg(not(feature = "f4"))]
        let surfacing: Option<f64> = None;
        passes.push(format!(
            "\"{name}\":{{\"verdict\":\"{}\",\"vegetation\":{},\"selection\":{},\"shadow\":{},\"surfacing\":{},\"total\":{}}}",
            report.verdict().name(),
            opt(report.p50_ms()),
            opt(report.selection_p50_ms()),
            opt(report.shadow_p50_ms()),
            opt(surfacing),
            opt(report.total_p50_ms()),
        ));
    }
    println!(
        "{{\"version\":\"{VERSION}\",\"preset\":\"{id}\",\"seed\":{seed},\"age\":{},\
         \"first_ms\":{first:.1},\"builds_ms\":[{}],\"tree_buffer_bytes\":{tree_bytes},\
         \"gpu_process_mb_first\":{first_process_mb},\"gpu_process_mb\":{},\"rss_mb\":{},\"peak_rss_mb\":{},\
         \"frame_ms\":{{{}}},\"passes\":{{{}}},\"alloc_base\":{base},\"alloc_first\":{after_first},\
         \"alloc_resident\":{resident},\"curves\":{{{}}},\"rss_built\":[{rss_before_trim},{rss_trimmed}],\"stages_mb\":[{stage_device},{stage_renderer},{stage_generator},{stage_programs}],\"load\":\"{}\"}}",
        family.age,
        builds.iter().map(|b| format!("{b:.1}")).collect::<Vec<_>>().join(","),
        gpu_process_mb(),
        status_mb("VmRSS:"),
        status_mb("VmHWM:"),
        fps.join(","),
        passes.join(","),
        curves.join(","),
        load()
    );
    Ok(())
}

#[cfg(feature = "f4")]
fn curve(r: Option<telperion_render::CurveReport>) -> String {
    r.map_or("null".into(), |r| {
        format!(
            "{{\"scale\":{},\"overrun\":{},\"vertices\":{},\"tube_triangles\":{},\"ribbon_triangles\":{},\"demand\":{:?},\"budget\":{:?}}}",
            r.scale, r.overrun, r.vertices, r.tube_triangles, r.ribbon_triangles, r.demand, r.budget
        )
    })
}

extern "C" {
    fn malloc_trim(pad: usize) -> i32;
}

fn opt(v: Option<f64>) -> String {
    v.map_or("null".into(), |v| format!("{v:.4}"))
}

/// The device allocator's live bytes grouped by label, and its totals.
fn allocations(renderer: &Renderer) -> String {
    let Some(report) = renderer.gpu().device.generate_allocator_report() else {
        return "null".into();
    };
    let mut by: BTreeMap<String, (u64, u32)> = BTreeMap::new();
    for a in &report.allocations {
        let e = by.entry(a.name.clone()).or_default();
        e.0 += a.size;
        e.1 += 1;
    }
    let rows: Vec<String> = by
        .iter()
        .map(|(k, (b, n))| format!("{:?}:[{b},{n}]", k))
        .collect();
    format!(
        "{{\"allocated\":{},\"reserved\":{},\"blocks\":{},\"by\":{{{}}}}}",
        report.total_allocated_bytes,
        report.total_reserved_bytes,
        report.blocks.len(),
        rows.join(",")
    )
}

fn views(hero: Camera) -> [(&'static str, Camera); 3] {
    let toward = |share: f64| Camera {
        position: hero.target + (hero.position - hero.target) * share,
        ..hero
    };
    [("whole", hero), ("mid", toward(0.5)), ("close", toward(0.25))]
}

fn ms(t: Instant) -> f64 {
    t.elapsed().as_secs_f64() * 1e3
}

fn wait(renderer: &Renderer) {
    let _ = renderer
        .gpu()
        .device
        .poll(wgpu::PollType::wait_indefinitely());
}

fn gpu_process_mb() -> i64 {
    let me = std::process::id().to_string();
    let out = std::process::Command::new("nvidia-smi")
        .args(["--query-compute-apps=pid,used_memory", "--format=csv,noheader,nounits"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).into_owned())
        .unwrap_or_default();
    out.lines()
        .filter_map(|l| {
            let mut f = l.split(',').map(str::trim);
            (f.next()? == me).then(|| f.next()?.parse().ok())?
        })
        .next()
        .unwrap_or(-1)
}

fn status_mb(key: &str) -> u64 {
    std::fs::read_to_string("/proc/self/status")
        .ok()
        .and_then(|s| {
            let line = s.lines().find(|l| l.starts_with(key))?.to_owned();
            line.split_whitespace().nth(1)?.parse::<u64>().ok()
        })
        .map_or(0, |kb| kb / 1024)
}

fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_default()
}
