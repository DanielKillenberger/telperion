//! The whole fine stage and the build from parameters on the GPU against
//! the CPU executor on 1 and all cores, cold and warm (fn-209 F1 task
//! 8, step 3): beech and spruce at 40 and 80, seeds 1 and 7. The GPU's
//! build is `profile_tests::build` (the species point's tables, the
//! structure above the cut, the four phases and the landing); the CPU's
//! is `grow_fine` at the default plan. The fine stage is the executor's
//! four phases. Cold is a tree's first build in the process, after one
//! build of another tree (the device's clocks up, its pools those of
//! another tree). `cargo test --profile ci -p telperion-render
//! fine_stage_and_build -- --ignored --nocapture`, `FINE_GPU_LOCK`
//! naming the GPU lock, taken only around the device's builds.
use super::profile_tests::build;
use super::realized_tests::request;
use super::roots::kernels;
use std::time::Instant;
use telperion_space::{grow_fine, Plan, Species};

/// The GPU lock named by `FINE_GPU_LOCK`, held while the guard lives.
fn locked() -> Option<std::fs::File> {
    let path = std::env::var("FINE_GPU_LOCK").ok()?;
    let file = std::fs::File::create(path).expect("the lock file");
    file.lock().expect("the GPU lock");
    Some(file)
}

/// `grow_fine` on `threads` cores: the first build's milliseconds, the
/// best of two more, and that build's fine stage.
fn cpu(species: &Species, at: (u32, u64), threads: usize) -> [f64; 3] {
    let plan = Plan {
        threads,
        ..Plan::default()
    };
    let mut runs = (0..3).map(|_| {
        let t = Instant::now();
        let fine = grow_fine(species, request(at.0, at.1), plan).expect("grows");
        [t.elapsed().as_secs_f64() * 1e3, fine.expand_seconds() * 1e3]
    });
    let cold = runs.next().expect("a build")[0];
    let warm = runs.fold([f64::INFINITY; 2], |a, b| if b[0] < a[0] { b } else { a });
    [cold, warm[0], warm[1]]
}

#[test]
#[ignore = "a measurement"]
fn fine_stage_and_build() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    // One build first, so the device's clocks are up before the first
    // tree's cold build is timed.
    let warming = locked();
    build(&gpu, &k, &telperion_space::beech(), (40, 1));
    drop(warming);
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the start: {}", load.trim());
    println!("| Tree | GPU cold / warm ms (fine stage) | CPU all cores cold / warm ms (fine stage) | CPU 1 core cold / warm ms (fine stage) |");
    println!("|---|--:|--:|--:|");
    for (name, species) in [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
    ] {
        for at in [(40, 1), (40, 7), (80, 1), (80, 7)] {
            let lock = locked();
            let cold = build(&gpu, &k, &species, at).ms;
            let warm = (0..2)
                .map(|_| build(&gpu, &k, &species, at))
                .min_by(|a, b| a.ms.total_cmp(&b.ms))
                .expect("a build");
            drop(lock);
            let [ac, aw, af] = cpu(&species, at, 0);
            let [oc, ow, of] = cpu(&species, at, 1);
            println!(
                "| {name} {} {} | {cold:.0} / {:.0} ({:.0}) | {ac:.0} / {aw:.0} ({af:.0}) | {oc:.0} / {ow:.0} ({of:.0}) |",
                at.0, at.1, warm.ms, warm.fine
            );
        }
    }
    let load = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    println!("load at the end: {}", load.trim());
}
