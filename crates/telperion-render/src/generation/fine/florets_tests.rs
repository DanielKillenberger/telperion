//! fn-229 R2: the florets of a tree built on the real path against the CPU
//! whole tree's, packed against the same box. The comparison reads the one
//! organ buffer the submit takes: the CPU block, then the device's. A
//! floret has no identity on the wire, so the two sets are matched by
//! their words. The classes are:
//! - **exact:** the same three words;
//! - **within:** each position code within one of the other's (fn-125's
//!   screen, 50 um, lies under one code), the frame within `TURN` and the
//!   scale within a half float's step;
//! - **turned:** the same floret (its size within the screen) off by its
//!   stalk's turn about the axis, under twice the stalk's length: its
//!   axis's tangent at `Vec3::perpendicular`'s switch, two components equal
//!   in magnitude, where the executors take different reference axes and
//!   the golden spiral starts from another normal (FLORETS.md);
//! - **least:** a floret one side keeps and the other drops, its size at
//!   `bloom.rs`'s least size (`LEAST`, 1e-3) within a half float's step:
//!   the f32-against-f64 class of that threshold;
//! - **unmatched:** the device's, and the whole tree's, in none of these.
use super::resident::{build, Tools};
use super::roots::kernels;
use crate::generation::{io, Generator};
use crate::Gpu;
use std::collections::HashMap;
use telperion_core::foliage::{BloomParams, Leaf, Reference};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;

/// fn-125's screen on a frame, in radians.
const TURN: f64 = 0.01;
/// The stalk's length the probe draws (`flowering`).
const STALK: f64 = 0.003;

/// The florets by class.
#[allow(dead_code)]
#[derive(Debug, Default)]
struct Florets {
    /// The device path's (its CPU block's, its device block's) and the
    /// whole tree's.
    counts: (usize, usize, usize),
    exact: usize,
    within: usize,
    turned: usize,
    /// The device's, and the whole tree's, at the least size.
    least: (usize, usize),
    unmatched: (usize, usize),
    /// The most apart within: position (m), frame (rad), scale (relative).
    position: f64,
    turn: f64,
    scale: f64,
}

/// `id` at (age, seed), every age flowering at 0.6 under the oak's catkin
/// rows (fn-216's `overlays/oak-catkins.json`), each stalk a few
/// millimetres long so the pedicel is drawn too.
pub(super) fn flowering(id: &str, (age, seed): (u32, u32)) -> Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    (family.age, family.skeleton.seed) = (f64::from(age), seed);
    for s in &mut family.space.species.as_mut().unwrap().states {
        s.flowering = 0.6;
    }
    family.bloom = BloomParams {
        florets: 28.0,
        rachis: 0.07,
        droop: 1.0,
        pedicel: STALK,
        spread: 50.0,
        taper: 0.3,
        petals: 4.0,
        open: 70.0,
        petal_length: 0.004,
        petal_width: 0.004,
        petal_widest: 0.5,
        petal_cup: 0.6,
        scatter: 0.15,
        opens: 0.3,
        lasts: 0.08,
        stagger: 0.3,
        ..BloomParams::default()
    };
    family
}

fn codes(f: &Leaf) -> [i32; 3] {
    [f[1] & 0xffff, f[1] >> 16, f[2] & 0xffff].map(|c| c as i32)
}

/// Two florets' scales apart, relative to the second's.
fn relative(r: Reference, f: &Leaf, g: &Leaf) -> f64 {
    (r.scale(*f) - r.scale(*g)).abs() / r.scale(*g).max(1e-12)
}

fn column(m: &[f32; 16], c: usize) -> Vec3 {
    Vec3::new(m[c * 4] as f64, m[c * 4 + 1] as f64, m[c * 4 + 2] as f64).normalized()
}

/// The device path's florets against the whole tree's, both against `r`.
fn compare(mine: &[Leaf], theirs: &[Leaf], r: Reference) -> Florets {
    let mut out = Florets::default();
    let key = |f: &Leaf| (f[1], f[2], f[0]);
    let mut a: Vec<Leaf> = mine.to_vec();
    let mut b: Vec<Leaf> = theirs.to_vec();
    a.sort_unstable_by_key(key);
    b.sort_unstable_by_key(key);
    let (mut i, mut j) = (0, 0);
    let (mut left, mut right) = (Vec::new(), Vec::new());
    while i < a.len() && j < b.len() {
        match key(&a[i]).cmp(&key(&b[j])) {
            std::cmp::Ordering::Equal => (out.exact, i, j) = (out.exact + 1, i + 1, j + 1),
            std::cmp::Ordering::Less => (left.push(a[i]), i += 1).1,
            std::cmp::Ordering::Greater => (right.push(b[j]), j += 1).1,
        }
    }
    left.extend_from_slice(&a[i..]);
    right.extend_from_slice(&b[j..]);
    let mut by: HashMap<[i32; 3], Vec<usize>> = HashMap::new();
    for (k, f) in right.iter().enumerate() {
        by.entry(codes(f)).or_default().push(k);
    }
    let mut used = vec![false; right.len()];
    let mut lost = Vec::new();
    for f in &left {
        let c = codes(f);
        let m = r.unpack(*f);
        let off = |k: usize| {
            let n = r.unpack(right[k]);
            (0..3)
                .map(|q| column(&m, q).dot(column(&n, q)).clamp(-1.0, 1.0).acos())
                .fold(0.0, f64::max)
        };
        let near = (-1..=1).flat_map(|x| {
            (-1..=1).flat_map(move |y| (-1..=1).map(move |z| [c[0] + x, c[1] + y, c[2] + z]))
        });
        // A partner within the screen only: a catkin's florets stand a few
        // millimetres apart, so a nearer floret off the screen is another.
        let best = near
            .flat_map(|n| by.get(&n).into_iter().flatten().copied())
            .filter(|&k| !used[k] && off(k) <= TURN && relative(r, f, &right[k]) <= 1e-3)
            .min_by(|&p, &q| off(p).total_cmp(&off(q)));
        let Some(k) = best else {
            lost.push(*f);
            continue;
        };
        used[k] = true;
        let (turn, scale) = (off(k), relative(r, f, &right[k]));
        let position = r.position(*f).distance(r.position(right[k]));
        out.within += 1;
        (out.position, out.turn) = (out.position.max(position), out.turn.max(turn));
        out.scale = out.scale.max(scale);
    }
    let mut rest: Vec<Leaf> = (right.iter().zip(&used))
        .filter(|(_, &u)| !u)
        .map(|(f, _)| *f)
        .collect();
    // The leaves' sizes at the least size, within a half float's step.
    let least = |f: &Leaf| (r.scale(*f) - 1e-3).abs() < 2e-6;
    for f in &lost {
        let at = r.position(*f);
        let apart = |g: &Leaf| r.position(*g).distance(at);
        let twin = (0..rest.len())
            .filter(|&k| relative(r, f, &rest[k]) <= 1e-3 && apart(&rest[k]) < 2.0 * STALK)
            .min_by(|&p, &q| apart(&rest[p]).total_cmp(&apart(&rest[q])));
        match twin {
            Some(k) => (out.turned, _) = (out.turned + 1, rest.swap_remove(k)),
            None if least(f) => out.least.0 += 1,
            None => {
                out.unmatched.0 += 1;
                if std::env::var("TRACE").is_ok() {
                    println!("device floret at {at:?} scale {}", r.scale(*f));
                }
            }
        }
    }
    for g in &rest {
        if least(g) {
            out.least.1 += 1;
            continue;
        }
        out.unmatched.1 += 1;
        if std::env::var("TRACE").is_ok() {
            println!(
                "whole-tree floret at {:?} scale {}",
                r.position(*g),
                r.scale(*g)
            );
        }
    }
    out
}

fn generator(gpu: &Gpu) -> Generator {
    let make = Generator::create(gpu.clone(), std::sync::Arc::new(()), true);
    pollster::block_on(make).unwrap()
}

/// `family`'s florets on the real path against the CPU whole tree's.
fn parity(gpu: &Gpu, family: &Family) -> Florets {
    let (k, gen) = (kernels(gpu), generator(gpu));
    let built = pollster::block_on(build(gpu, &gen, &k, &Tools::new(gpu), family)).unwrap();
    let organ = built.prepared.organ.as_ref().expect("the tree flowers");
    let r = built
        .prepared
        .mesh
        .organ
        .as_ref()
        .unwrap()
        .instances
        .reference;
    let cpu = built.expansion.organ_in(r).unwrap().instances.len();
    let mine = io::read_leaves_async(gpu, organ.leaves.buffer(), organ.count);
    let mine = pollster::block_on(mine).unwrap();
    drop(built);
    let x = executor::grow(family).unwrap().expansion().unwrap();
    let theirs = x.organ_in(r).unwrap().instances.leaves;
    let mut out = compare(&mine, &theirs, r);
    out.counts = (cpu, mine.len() - cpu, theirs.len());
    out
}

/// The beech and the spruce at 30, flowering, on the real path: every
/// floret the whole tree draws, each within the screen.
#[test]
fn florets_on_the_device_match_the_whole_tree() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for id in ["european-beech", "norway-spruce"] {
        let p = parity(&gpu, &flowering(id, (30, 1)));
        println!("{id} (30, 1): {p:?}");
        assert!(p.counts.1 > 0, "{id}: the device draws florets");
        assert_eq!(p.unmatched, (0, 0), "{id}: every floret classed");
        let (mine, theirs) = (p.counts.0 + p.counts.1, p.counts.2);
        assert_eq!(
            mine + p.least.1,
            theirs + p.least.0,
            "{id}: florets per tree"
        );
    }
}

/// The parity probe at 40 and 80 (`cargo test --profile ci -p
/// telperion-render florets_at_80 -- --ignored --nocapture`, under the GPU
/// lock, `PROBE=norway-spruce:80:1`, one tree a process). Reported, not held.
#[test]
#[ignore = "device limits: a tree at 80 on the device"]
fn florets_at_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "norway-spruce:80:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let at = (parts[1].parse().unwrap(), parts[2].parse().unwrap());
    let p = parity(&gpu, &flowering(parts[0], at));
    println!("{only}: {p:?}");
}

/// Review finding: the bloom rows are checked before the device counts
/// florets by them, so a count past the rows' bounds is refused by name
/// instead of looping a billion times a node.
#[test]
fn a_bloom_out_of_bounds_is_refused_before_the_device_counts() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut family = flowering("european-beech", (12, 1));
    family.bloom.florets = 1e9;
    let (k, gen) = (kernels(&gpu), generator(&gpu));
    let built = pollster::block_on(build(&gpu, &gen, &k, &Tools::new(&gpu), &family));
    let error = built.err().expect("the rows are refused").to_string();
    assert!(error.contains("bloom florets"), "{error}");
}

/// Review finding: a window a billionth of a year long flowers on the
/// device as on the CPU; the time since it opened is taken in f64 before
/// it is narrowed.
#[test]
fn a_window_of_a_billionth_of_a_year_flowers_on_the_device() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut family = flowering("european-beech", (12, 1));
    (family.bloom.opens, family.bloom.lasts, family.bloom.stagger) = (0.3, 1e-9, 0.0);
    let p = parity(&gpu, &family);
    assert!(p.counts.1 > 0, "the device draws florets: {p:?}");
    assert_eq!(p.counts.0 + p.counts.1, p.counts.2, "{p:?}");
}

/// fn-253 R1: a shipped preset's own florets on the real path against the
/// CPU whole tree's (`cargo test --profile ci -p telperion-render
/// preset_florets -- --ignored --nocapture`, under the GPU lock,
/// `PROBE=laurelin:30:1`). Reported, not held.
#[test]
#[ignore = "a probe: one shipped preset on the device"]
fn preset_florets() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let only = std::env::var("PROBE").unwrap_or_else(|_| "laurelin:30:1".into());
    let parts: Vec<&str> = only.split(':').collect();
    let mut family = telperion_core::presets::by_identity(parts[0]).unwrap();
    family.age = parts[1].parse().unwrap();
    family.skeleton.seed = parts[2].parse().unwrap();
    let p = parity(&gpu, &family);
    println!("{only}: {p:?}");
}
