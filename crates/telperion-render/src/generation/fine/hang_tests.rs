//! fn-235 R1: the browser's build route run natively, under the page's
//! device limits (TASK10.md: `maxBufferSize` 4,294,967,292,
//! `maxStorageBufferBindingSize` 2,147,483,644; every other limit
//! WebGPU's default), on one generator and renderer as the page holds
//! them. One tree a process, under a wall-clock `timeout` that kills it,
//! with `TELPERION_TRACE` set so the last step logged is the one that
//! never completed: `TREE=silver-birch:20 TELPERION_TRACE=1 timeout 120
//! cargo test --profile ci -p telperion-render browser_route -- --ignored`.
use super::super::{Delivery, Generator};
use crate::{Gpu, Renderer};

/// The page's device: its two raised limits, WebGPU's defaults beside.
pub(super) fn browser_gpu() -> Option<Gpu> {
    let limits = |available: &wgpu::Limits| wgpu::Limits {
        max_buffer_size: available.max_buffer_size.min(4_294_967_292),
        max_storage_buffer_binding_size: available
            .max_storage_buffer_binding_size
            .min(2_147_483_644),
        ..wgpu::Limits::default()
    };
    match pollster::block_on(Gpu::request_within(None, limits)) {
        Ok(gpu) => Some(gpu),
        Err(error) => {
            println!("skipped: {error}");
            None
        }
    }
}

#[test]
#[ignore = "a native repro, one tree a process"]
fn browser_route() {
    let tree = std::env::var("TREE").expect("TREE=<species>:<age>");
    let (id, age) = tree.split_once(':').expect("TREE=<species>:<age>");
    let Some(gpu) = browser_gpu() else {
        return;
    };
    // The page's own parameters where `FAMILY` names their JSON.
    let family = match std::env::var("FAMILY") {
        Ok(path) => {
            let text = std::fs::read_to_string(path).unwrap();
            telperion_core::params::parse(&serde_json::from_str(&text).unwrap()).unwrap()
        }
        Err(_) => {
            let mut family = telperion_core::presets::by_identity(id).unwrap();
            family.age = age.parse().unwrap();
            family
        }
    };
    // From the generator's making, as a page's first tree pays it.
    let t = std::time::Instant::now();
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = pollster::block_on(Generator::new_async(&renderer)).unwrap();
    let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
    let built = t.elapsed().as_secs_f64();
    renderer.submit_prepared(prepared).unwrap();
    let camera = crate::hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, crate::GROUND_REACH);
    crate::render(&mut renderer, &camera, 320, 240).unwrap();
    println!("{tree}: built in {built:.2} s and drawn");
}

/// The longest a pipeline may take to make on a cold driver cache, and
/// the longest a page's first tree may take to build on one, every
/// pipeline made (host decisions 2, 2026-10-09): well under Chrome's GPU
/// watchdog, which kills a GPU process that makes one for longer; 2.5 s
/// gives the 2 s target its noise on a loaded machine.
const COLD_PIPELINE_MS: f64 = 2_500.0;
const COLD_FIRST_TREE_S: f64 = 5.0;

/// fn-235 R1: a page's first tree, the beech at 20 on the fine path,
/// built in a process of its own on a cold driver cache. The chain
/// grower's `grow` took 145 s on the RTX 3080 (2026-10-09), long past
/// the watchdog: the harness sat at "building..." until Chrome killed its
/// GPU process.
#[test]
fn every_pipeline_within_2_5_s_and_the_first_tree_within_5_s_cold() {
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "--nocapture"])
        .arg("generation::fine::hang_tests::browser_route")
        .env("TREE", "european-beech:20")
        .env("TELPERION_TRACE", "1")
        // The NVIDIA driver's own disk cache, off: a cold page's compile.
        .env("__GL_SHADER_DISK_CACHE", "0")
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .spawn()
        .unwrap();
    let read = |pipe: Box<dyn std::io::Read + Send>| {
        std::thread::spawn(move || {
            use std::io::BufRead;
            std::io::BufReader::new(pipe)
                .lines()
                .map_while(Result::ok)
                .collect::<Vec<_>>()
        })
    };
    let stderr = read(Box::new(child.stderr.take().unwrap()));
    let stdout = read(Box::new(child.stdout.take().unwrap()));
    // Well past the bound, so a slow pipeline is named rather than cut off.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(120);
    let mut killed = false;
    while child.try_wait().unwrap().is_none() {
        if std::time::Instant::now() > deadline {
            child.kill().unwrap();
            killed = true;
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let status = child.wait().unwrap();
    let made: Vec<(String, f64)> = stderr
        .join()
        .unwrap()
        .iter()
        .filter_map(|l| {
            let (_, rest) = l.split_once("] pipeline ")?;
            let (entry, ms) = rest.split_once(" made in ")?;
            Some((entry.into(), ms.strip_suffix(" ms")?.parse().ok()?))
        })
        .collect();
    let last = made.last().map(|(entry, _)| entry.as_str());
    assert!(
        !killed,
        "a pipeline was still being made at 120 s, after {last:?}"
    );
    assert!(status.success(), "the build failed: {status}");
    let slow: Vec<_> = made
        .iter()
        .filter(|(_, ms)| *ms > COLD_PIPELINE_MS)
        .collect();
    assert!(!made.is_empty(), "no pipeline was made");
    assert!(
        slow.is_empty(),
        "made cold over {COLD_PIPELINE_MS} ms: {slow:?}"
    );
    let built: f64 = (stdout.join().unwrap().iter())
        .find_map(|l| {
            l.split_once("built in ")?
                .1
                .split_once(" s")?
                .0
                .parse()
                .ok()
        })
        .expect("the build's time");
    assert!(
        built <= COLD_FIRST_TREE_S,
        "the first tree built cold in {built} s"
    );
}

/// fn-235 host decision 1: the chain grower's `grow` alone, made from
/// `GROW_WGSL` where it names a file (a bisected variant), else from its
/// source, which `GROW_DUMP` writes out. Run with the cache off, one a
/// process; the trace prints the make time.
#[test]
#[ignore = "a bisection probe"]
fn make_grow() {
    let Some(gpu) = browser_gpu() else {
        return;
    };
    let source = match std::env::var("GROW_WGSL") {
        Ok(path) => std::fs::read_to_string(path).unwrap(),
        Err(_) => super::chains::source(),
    };
    if let Ok(path) = std::env::var("GROW_DUMP") {
        std::fs::write(path, &source).unwrap();
    }
    let read = [true, true, true, false, false, false, false, false];
    let _ = super::super::io::Pass::new(&gpu, source, &read, &["grow"]);
}
