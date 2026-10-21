//! fn-298 R3: the film's walks build on the device. Each walk segment of
//! fn-272's `release.json`, taken as `Path::frames` takes it: at 30 fps,
//! `n = round(seconds * 30)` frames at `t = ease((i + 1) / n)`, `ease` the
//! smoothstep. Frames are named by their 1-based number `i + 1`.
//!
//! The ordinary test builds a stated subset: the first and last frame of
//! each walk at each seed, the 9 frames whose hierarchy generations passed
//! their stepped `rounds` (R2.md, the survey), and the Laurelin at 20. R1's
//! other rows (beech to spruce at t 0.00037, 0.01 and 0.1, seeds 7 and 1,
//! and the Laurelin at 20 at seed 7) are `cpu_builds_tests`' own. The full
//! sweep is ignored and named in RESULT.md.
use super::cpu_builds_tests::{apart, at, refused_on_device};
use super::Generator;
use crate::Renderer;
use telperion_core::presets::Family;

/// The film's walk segments: from, to, age and seconds (`release.json`).
const SEGMENTS: [(&str, &str, f64, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0, 3.0),
    ("norway-spruce", "silver-birch", 40.0, 3.0),
    ("silver-birch", "oregon-white-oak", 40.0, 3.0),
    ("oregon-white-oak", "date-palm", 40.0, 4.0),
    ("date-palm", "laurelin", 1.0, 1.0),
    ("laurelin", "telperion", 80.0, 3.0),
];
const SEEDS: [u32; 3] = [1, 7, 42];

/// The frames whose `rounds` the survey saw passed (+1 each): segment,
/// seed and frame number.
const ROUNDS: [(usize, u32, u32); 9] = [
    (0, 1, 41),
    (0, 7, 27),
    (1, 1, 63),
    (1, 7, 63),
    (1, 42, 56),
    (1, 42, 59),
    (3, 1, 36),
    (3, 1, 37),
    (3, 1, 48),
];

/// The share frame `i` (1-based) of `n` stands at.
fn share(i: u32, n: u32) -> f64 {
    let s = f64::from(i) / f64::from(n);
    s * s * (3.0 - 2.0 * s)
}

/// Frame `i` (1-based) of segment `k` at `seed`, named.
fn frame(k: usize, seed: u32, i: u32) -> (String, Family) {
    let (a, b, age, _) = SEGMENTS[k];
    let (n, t) = (count(k), share(i, count(k)));
    let (x, y) = (at(a, age, seed), at(b, age, seed));
    let family = telperion_core::blend::families(&x, &y, t).expect("a walk frame");
    let name = format!("{a} to {b} at {age}, seed {seed}, frame {i} of {n}, t {t}");
    (name, family)
}

/// The frames in segment `k`.
fn count(k: usize) -> u32 {
    (SEGMENTS[k].3 * 30.0).round() as u32
}

/// The Laurelin at 20 at each seed.
fn laurelin() -> Vec<(String, Family)> {
    SEEDS
        .map(|seed| {
            (
                format!("laurelin at 20, seed {seed}"),
                at("laurelin", 20.0, seed),
            )
        })
        .into()
}

/// fn-299 owns these: oak to palm, where the walk's blended rows let one
/// apex enter 12 units a cycle, past `FRAMES` = 11 (frames 106 to 119,
/// t ≥ 0.962, every seed), and the two frames at seed 42 whose living
/// units rounds passed their projection (76 and 77).
fn fn299(k: usize, seed: u32, i: u32) -> bool {
    k == 3 && ((106..=119).contains(&i) || (seed == 42 && (76..=77).contains(&i)))
}

/// Every frame of every walk at every seed, but fn-299's.
fn sweep() -> Vec<(String, Family)> {
    let mut out = laurelin();
    for k in 0..SEGMENTS.len() {
        for seed in SEEDS {
            for i in (1..=count(k)).filter(|&i| !fn299(k, seed, i)) {
                out.push(frame(k, seed, i));
            }
        }
    }
    out
}

/// The ordinary test's subset: each walk's first and last frame at each
/// seed, the 9 `rounds` frames and the Laurelin at 20.
fn subset() -> Vec<(String, Family)> {
    let mut out = laurelin();
    for k in 0..SEGMENTS.len() {
        for seed in SEEDS {
            out.extend([frame(k, seed, 1), frame(k, seed, count(k))]);
        }
    }
    out.extend(ROUNDS.map(|(k, seed, i)| frame(k, seed, i)));
    out
}

/// fn-298 R3, the stated subset: every frame builds on the device.
#[test]
fn the_film_walks_build_on_the_device() {
    let refused = refused_on_device(&subset());
    assert!(refused.is_empty(), "{refused:#?}");
}

/// fn-298 R3, the full sweep: every frame of every walk at seeds 1, 7 and
/// 42, and the Laurelin at 20, builds on the device.
#[test]
#[ignore = "fn-298 R3's full sweep, about 1,500 device builds"]
fn every_film_walk_frame_builds_on_the_device() {
    let frames = sweep();
    let refused = refused_on_device(&frames);
    println!("{} frames, {} refused", frames.len(), refused.len());
    assert!(refused.is_empty(), "{refused:#?}");
}

/// The frames fn-299 owns, on the device.
#[test]
#[ignore = "fn-299: oak-to-palm walk frames whose blended rows pass FRAMES or the units rounds"]
fn fn299_oak_to_palm_frames_build_on_the_device() {
    let mut frames = Vec::new();
    for seed in SEEDS {
        for i in (1..=count(3)).filter(|&i| fn299(3, seed, i)) {
            frames.push(frame(3, seed, i));
        }
    }
    let refused = refused_on_device(&frames);
    assert!(refused.is_empty(), "{refused:#?}");
}

/// The `rounds` frame whose node count fn-300 owns (host, 2026-10-21):
/// spruce to birch at 40, seed 42, frame 56, one node more on the device,
/// with every stepped limit at its bound as well.
const FN300: (usize, u32, u32) = (1, 42, 56);

/// fn-298 R4 (narrowed): each `rounds` frame has its leaf instances within
/// 1 of the CPU's on the device, and the CPU's node count, but `FN300`'s.
/// Positions are fn-300's.
#[test]
fn the_rounds_frames_have_the_cpus_counts() {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    let mut off = Vec::new();
    for (k, seed, i) in ROUNDS {
        let (name, family) = frame(k, seed, i);
        let cpu = telperion_core::mesh::build(&family)
            .unwrap_or_else(|e| panic!("{name}: the CPU refused: {e}"))
            .foliage_instances();
        let device = gen
            .prepare(&family)
            .unwrap_or_else(|e| panic!("{name}: {e}"))
            .count();
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
        println!("{name}: device {device} instances, CPU {cpu}");
        if device.abs_diff(cpu) > 1 {
            off.push(format!("{name}: device {device} instances, CPU {cpu}"));
        }
        if (k, seed, i) != FN300 {
            if let Some(d) = nodes_off(&gpu, &name, family) {
                off.push(d);
            }
        }
    }
    assert!(off.is_empty(), "{off:#?}");
}

/// `FN300`'s node count on the device is the CPU's.
#[test]
#[ignore = "fn-300: spruce to birch at 40, seed 42, frame 56 has 323,513 device nodes against the CPU's 323,512"]
fn fn300_the_spruce_to_birch_frame_56_has_the_cpus_nodes() {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let (k, seed, i) = FN300;
    let (name, family) = frame(k, seed, i);
    let off = nodes_off(&gpu, &name, family);
    assert!(off.is_none(), "{off:?}");
}

/// Where the device's drawn node count is not the CPU's, the frame and
/// how they stood.
fn nodes_off(gpu: &crate::Gpu, name: &str, mut family: Family) -> Option<String> {
    // The dress draws the tree before its leaf bases are hung.
    family.canopy.leaf_bases = 0.0;
    let d = apart(gpu, &family);
    println!("{name}: {d:?}");
    (d.device != d.cpu).then(|| format!("{name}: {d:?}"))
}
