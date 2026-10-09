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

/// The longest a pipeline may take to make on a cold driver cache, and
/// the longest a page's first tree may take to build on one, every
/// pipeline made (host decisions 2, 2026-10-09): well under Chrome's GPU
/// watchdog, which kills a GPU process that makes one for longer; 2.5 s
/// gives the 2 s target its noise on a loaded machine.
const COLD_PIPELINE_MS: f64 = 2_500.0;
const COLD_FIRST_TREE_S: f64 = 5.0;

/// The named exception (host decisions, third, 2026-10-09): the chain
/// grower's `grow`, at most 20 s cold natively, its make time outside the
/// first tree's 5 s. Its cost is the one body the driver inlines the apex
/// loop into, with hundreds of inlined copies of the 64-bit arithmetic
/// under contract A (528 of `mul32`, 380 of `shr64`); slimmed to one call
/// site each for `run`, `grow_unit` and `abortion_after` it fell from 92 to
/// 145 s to 13 to 17 s, and splitting `grow_unit` and `move_on` into their
/// own pipelines would add 11 to 13 % to the beech's build before keeping
/// any state (REPRO.md). The owner's cold-profile browser check decides
/// whether Chrome's own translator stays under its watchdog.
const GROW: &str = "grow";
const COLD_GROW_MS: f64 = 20_000.0;

/// fn-235 R1: a page's first tree, the beech at 20 on the fine path,
/// built in a process of its own on a cold driver cache. The chain
/// grower's `grow` took 145 s on the RTX 3080 (2026-10-09), long past
/// the watchdog: the harness sat at "building..." until Chrome killed its
/// GPU process.
#[test]
fn every_pipeline_within_2_5_s_but_grow_within_20_s_and_the_first_tree_within_5_s_cold() {
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
    let bound = |entry: &str| match entry {
        GROW => COLD_GROW_MS,
        _ => COLD_PIPELINE_MS,
    };
    let slow: Vec<_> = made.iter().filter(|(e, ms)| *ms > bound(e)).collect();
    assert!(!made.is_empty(), "no pipeline was made");
    assert!(
        slow.is_empty(),
        "made cold over its bound ({COLD_PIPELINE_MS} ms, {GROW} {COLD_GROW_MS} ms): {slow:?}"
    );
    let grow: f64 = made.iter().filter(|(e, _)| e == GROW).map(|m| m.1).sum();
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
    let besides = built - grow / 1e3;
    assert!(
        besides <= COLD_FIRST_TREE_S,
        "the first tree built cold in {built} s, {besides} s besides {GROW}"
    );
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
        .arg("generation::fine::hang_tests::browser_route")
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

/// The longest one submission may hold the device (R2): well under
/// Chrome's GPU watchdog.
const SUBMISSION_MS: f64 = 1_000.0;

/// fn-235 R2: every shipped species on the fine or whole path at 80, each
/// submission waited for alone (`TELPERION_TRACE_WAIT`), none longer than
/// a second. Measured 2026-10-09 on the RTX 3080: the beech's worst 152 ms
/// (girths), the oak's 96, the spruce's 62, the date palm's 1.
#[test]
#[ignore = "device limits: each shipped species at 80 on the device"]
fn every_submission_at_80_within_a_second() {
    let env = [("TELPERION_TRACE", "1"), ("TELPERION_TRACE_WAIT", "1")];
    for tree in [
        "european-beech:80",
        "norway-spruce:80",
        "oregon-white-oak:80",
        "date-palm:80",
    ] {
        let run = built(tree, &env);
        let said: Vec<_> = run.stdout.iter().rev().take(4).collect();
        assert!(
            !run.killed && run.status.success(),
            "{tree}: {} {said:?}",
            run.status
        );
        let worst = (run.stderr.iter())
            .filter_map(|l| l.split_once(" ran ")?.1.strip_suffix(" ms")?.parse().ok())
            .fold(0.0f64, f64::max);
        println!("{tree}: worst submission {worst:.1} ms");
        assert!(worst > 0.0, "{tree}: no submission timed");
        assert!(
            worst <= SUBMISSION_MS,
            "{tree}: a submission ran {worst} ms"
        );
    }
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
    assert!(marked.len() >= 10, "the marked probes: {marked:?}");
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
