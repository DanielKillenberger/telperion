//! fn-246's frame probe: what one shipped tree costs a frame on the
//! harness's GPU route, natively at 1920x1080 with the renderer's
//! multisampling. One build (`Generator::prepare`, resident delivery), then
//! the hero pose ("whole") and the same pose at a quarter of the distance
//! ("close"), each in the whole and the bare view. Prints one JSON line.
//!
//!   cargo run --profile ci -p telperion-render --example frames -- <preset> [full|relief-off|parallax-off|no-rachis] [view...]
//!
//! Views are any of whole, mid (half the distance), close, bare-whole and
//! bare-close; all but mid by default.
//! `relief-off` draws the bark with no relief (ridge scale and depth zero),
//! `parallax-off` with its relief and no depth: the wood's shaded pass
//! measured against itself (fn-246 R1). `no-rachis` grows the flowers with
//! no rachis, the florets on their tips: what the rachises cost (fn-253).
//! `FLOWERING=<share>` sets a trial share for the short shoots.
use std::time::Instant;
use telperion_core::presets::Preset;
use telperion_render::{
    generation::{Delivery, Generator},
    hero_pose, measure, Camera, Frame, Gpu, Renderer, View, GROUND_REACH, STILL_FORMAT,
};

const SIZE: (u32, u32) = (1920, 1080);
const PIPELINED: u32 = 120;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [id, rest @ ..] = &args[..] else {
        return Err("usage: frames <preset> [full|relief-off|parallax-off|no-rachis] [view...]".into());
    };
    let variant = rest.first().map_or("full", String::as_str);
    let mut family = Preset::from_id(id).ok_or("no preset")?.parameters();
    family.skeleton.seed = 1;
    if variant == "no-rachis" {
        family.supernatural.bloom.rachis = Some(0.0);
    }
    // A trial flowering share for the short shoots (fn-253).
    if let Ok(share) = std::env::var("FLOWERING") {
        family.supernatural.flowering.age14 = Some(share.parse()?);
    }
    let mut renderer = Renderer::new(pollster::block_on(Gpu::request(None))?, STILL_FORMAT);
    let generator = Generator::new(&renderer)?;
    pollster::block_on(generator.prepare_programs())?;
    let frame = Frame::new(&renderer, "frames", SIZE);
    renderer.submit_prepared(generator.prepare(&family, Delivery::Resident)?)?;
    drop(generator);
    renderer.set_family(&family);
    let mut material = family.drawn_material();
    match variant {
        "full" | "no-rachis" => {}
        "relief-off" => (material.ridge_scale, material.depth_strength) = (0.0, 0.0),
        "parallax-off" => material.depth_strength = 0.0,
        other => return Err(format!("no variant {other}").into()),
    }
    renderer.set_material(material);
    let aspect = f64::from(SIZE.0) / f64::from(SIZE.1);
    let hero = hero_pose(renderer.bounds().ok_or("no bounds")?, aspect, GROUND_REACH);
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
        wait(&renderer);
        let started = Instant::now();
        for _ in 0..PIPELINED {
            renderer.draw(&camera, SIZE, frame.target());
        }
        wait(&renderer);
        let frame_ms = started.elapsed().as_secs_f64() * 1e3 / f64::from(PIPELINED);
        let timing = measure(&mut renderer, &camera, SIZE, frame.target())?.to_json();
        views.push(format!(
            "{{\"view\":\"{name}\",\"frame_ms\":{frame_ms:.3},\"fps\":{:.1},\"timing\":{}}}",
            1e3 / frame_ms,
            timing.replace('\n', "")
        ));
    }
    println!(
        "{{\"preset\":\"{id}\",\"variant\":\"{variant}\",\"load\":\"{}\",\"views\":[{}]}}",
        load(),
        views.join(",")
    );
    Ok(())
}

fn wait(renderer: &Renderer) {
    let _ = renderer
        .gpu()
        .device
        .poll(wgpu::PollType::wait_indefinitely());
}

/// The one-minute load average when the views were measured.
fn load() -> String {
    std::fs::read_to_string("/proc/loadavg")
        .ok()
        .and_then(|s| s.split_whitespace().next().map(str::to_owned))
        .unwrap_or_default()
}
