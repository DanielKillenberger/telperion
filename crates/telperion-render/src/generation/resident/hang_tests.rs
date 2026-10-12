//! fn-235 R1: the browser's build route run natively, under the page's
//! device limits (TASK10.md: `maxBufferSize` 4,294,967,292,
//! `maxStorageBufferBindingSize` 2,147,483,644; every other limit
//! WebGPU's default), on one generator and renderer as the page holds
//! them. One tree a process, under a wall-clock `timeout` that kills it,
//! with `TELPERION_TRACE` set so the last step logged is the one that
//! never completed: `TREE=silver-birch:20 TELPERION_TRACE=1
//! timeout 120
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
    // From the generator's making, as a page's first tree pays it: the
    // renderer is the page's, made before its "building...".
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let t = std::time::Instant::now();
    let gen = pollster::block_on(Generator::new_async(&renderer)).unwrap();
    let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
    let built = t.elapsed().as_secs_f64();
    renderer.submit_prepared(prepared).unwrap();
    let camera = crate::hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, crate::GROUND_REACH);
    crate::render(&mut renderer, &camera, 320, 240).unwrap();
    println!("{tree}: built in {built:.2} s and drawn");
}

/// fn-243 R4: the page prepares every program before its first tree, and
/// neither a build on the device nor a second preparation makes one
/// again.
#[test]
fn a_build_makes_no_program_the_preparation_did_not() {
    let Some(gpu) = browser_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let before = crate::programs::made();
    let gen = pollster::block_on(Generator::new_async(&renderer)).unwrap();
    pollster::block_on(gen.prepare_programs()).unwrap();
    let prepared = crate::programs::made();
    assert!(prepared > before, "the preparation made no program");
    let mut family = telperion_core::presets::by_identity("european-beech").unwrap();
    family.age = 5.0;
    gen.prepare(&family, Delivery::Resident).unwrap();
    pollster::block_on(gen.prepare_programs()).unwrap();
    assert_eq!(crate::programs::made(), prepared);
}

/// The longest a pipeline may take to make on a cold driver cache (host
/// decisions 2, 2026-10-09): well under Chrome's GPU watchdog, which kills
/// a GPU process that makes one for longer; 2.5 s gives the 2 s target its
/// noise on a loaded machine. The watchdog acts on one pipeline, never a
/// sum, so the first tree's cold total is reported, not bounded (fn-241
/// host decision).
const COLD_PIPELINE_MS: f64 = 2_500.0;

/// fn-235 R1: a page's first tree, the beech at 20 on the device, built
/// in a process of its own on a cold driver cache: no pipeline made past
/// the watchdog's margin (fn-256 task 2 made every cycle kernel small).
#[test]
fn every_pipeline_within_2_5_s_cold() {
    // The NVIDIA driver's own disk cache, off: a cold page's compile.
    let env = [("TELPERION_TRACE", "1"), ("__GL_SHADER_DISK_CACHE", "0")];
    let Built {
        status,
        killed,
        stderr,
        stdout,
    } = built("european-beech:20", &env);
    let made: Vec<(String, f64)> = stderr
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
    let slow: Vec<_> = made.iter().filter(|(_, ms)| *ms > COLD_PIPELINE_MS).collect();
    assert!(!made.is_empty(), "no pipeline was made");
    assert!(slow.is_empty(), "made cold over {COLD_PIPELINE_MS} ms: {slow:?}");
    let built: f64 = (stdout.iter())
        .find_map(|l| {
            l.split_once("built in ")?
                .1
                .split_once(" s")?
                .0
                .parse()
                .ok()
        })
        .expect("the build's time");
    println!("the first tree built cold in {built:.2} s");
}

/// What a tree's build on the browser route, in a process of its own,
/// left: how it ended, whether its 120 s ran out (well past every bound,
/// so a slow step is named rather than cut off), and what it printed.
struct Built {
    status: std::process::ExitStatus,
    killed: bool,
    stderr: Vec<String>,
    stdout: Vec<String>,
}

fn built(tree: &str, env: &[(&str, &str)]) -> Built {
    let mut child = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--ignored", "--exact", "--nocapture"])
        .arg("generation::resident::hang_tests::browser_route")
        .env("TREE", tree)
        .envs(env.iter().copied())
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
    Built {
        status: child.wait().unwrap(),
        killed,
        stderr: stderr.join().unwrap(),
        stdout: stdout.join().unwrap(),
    }
}

/// fn-235 R4: the tests that take the device toward its limits (a tree at
/// 80 built on it, out-of-memory probes) are marked `#[ignore = "device
/// limits: ..."]`, so a default run beside an open harness never takes
/// the driver's address space; they run only when asked (`-- --ignored`).
/// Every marked test in the crate's source is one libtest leaves out.
#[test]
fn device_limit_probes_run_only_when_asked() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut marked = Vec::new();
    let mut dirs = vec![src.clone()];
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            let module = path.strip_prefix(&src).unwrap().with_extension("");
            let module = module.to_str().unwrap().replace('/', "::");
            let text = std::fs::read_to_string(&path).unwrap();
            for part in text.split("#[ignore = \"device limits").skip(1) {
                let name = part.split_once("fn ").unwrap().1.split_once('(').unwrap().0;
                marked.push(format!("{module}::{name}: test"));
            }
        }
    }
    let listed = std::process::Command::new(std::env::current_exe().unwrap())
        .args(["--list", "--ignored", "--format", "terse"])
        .output()
        .unwrap();
    let ignored = String::from_utf8(listed.stdout).unwrap();
    assert!(marked.len() >= 2, "the marked probes: {marked:?}");
    for name in &marked {
        assert!(ignored.lines().any(|l| l == name), "{name} runs by default");
    }
}

/// fn-235 R2: every shader loop whose end the data decides (`loop`,
/// `while`) states its cap, derived from its inputs, on its own line or
/// the line before; a `for` states its bound in its header.
#[test]
fn every_data_bound_shader_loop_states_its_cap() {
    let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
    let (mut dirs, mut loops, mut uncapped) = (vec![src], 0, Vec::new());
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                dirs.push(path);
                continue;
            }
            if path.extension().is_none_or(|e| e != "wgsl") {
                continue;
            }
            let text = std::fs::read_to_string(&path).unwrap();
            let lines: Vec<&str> = text.lines().collect();
            for (k, line) in lines.iter().enumerate() {
                let code = line.split("//").next().unwrap().trim_start();
                if !(code.starts_with("loop {") || code.starts_with("while ")) {
                    continue;
                }
                loops += 1;
                let before = k.checked_sub(1).map_or("", |b| lines[b]);
                if !line.contains("cap") && !before.contains("cap") {
                    uncapped.push(format!("{}:{}", path.display(), k + 1));
                }
            }
        }
    }
    assert!(loops > 20, "{loops} loops found");
    assert!(
        uncapped.is_empty(),
        "loops with no cap stated: {uncapped:?}"
    );
}
