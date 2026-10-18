//! The resident build's digest (fn-293): one line a shipped preset and
//! seed of what the renderer receives from the device, the placed leaves
//! and the wood's packed curve (its point, cluster and section words and
//! its demand), each hashed. Two builds of the same generator print the
//! same lines; a change that keeps the device's trees keeps every line.
//!
//!   DIGEST_SEEDS=1,7,42 [DIGEST_PRESETS=norway-spruce] [DIGEST_DUMP=leaves.bin] [DIGEST_STILLS=<dir>] cargo test --profile ci -p telperion-render digest -- --ignored --nocapture
use super::super::{io, Generator, Prepared};
use std::fmt::{Debug, Write};
use telperion_core::pipeline::executor;
use telperion_core::presets::{Preset, CATALOGUE};
use telperion_core::surface::{CLUSTER_WORDS, POINT_WORDS};

/// FNV-1a, as `generation_digest` hashes: over bytes, or over a value's
/// `Debug` text, which prints every float exactly.
struct Fnv(u64);
impl Write for Fnv {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.bytes(s.as_bytes());
        Ok(())
    }
}
impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn bytes(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0 ^ u64::from(b)).wrapping_mul(0x0100_0000_01b3);
        }
    }
}
fn hash(bytes: &[u8]) -> String {
    let mut h = Fnv::new();
    h.bytes(bytes);
    format!("{:016x}", h.0)
}
fn debug(v: &impl Debug) -> String {
    let mut h = Fnv::new();
    write!(h, "{v:?}").expect("hashing never fails");
    format!("{:016x}", h.0)
}

/// What `prepared` hands the renderer, as one line.
fn line(gen: &Generator, prepared: &Prepared) -> String {
    let gpu = &gen.gpu;
    let leaves = gen.read_instances(prepared).unwrap();
    // `DIGEST_DUMP` names a file for the leaves' matrices, so two builds'
    // leaves are compared value by value (fn-293 task 6).
    if let Ok(path) = std::env::var("DIGEST_DUMP") {
        let bytes: Vec<u8> = (0..leaves.len())
            .flat_map(|k| leaves.matrix(k))
            .flat_map(f32::to_le_bytes)
            .collect();
        std::fs::write(path, bytes).unwrap();
    }
    let wood = prepared.wood.as_ref().expect("a resident build's curve");
    let (points, clusters, p, k, demand) = wood.blocks();
    let read = |b: &wgpu::Buffer, bytes: u64| match bytes {
        0 => Vec::new(),
        n => io::read(gpu, b, n).unwrap(),
    };
    let points = read(points, p * POINT_WORDS as u64 * 4);
    let clusters = read(clusters, u64::from(k) * CLUSTER_WORDS as u64 * 4);
    // The section words have no count beside them: the buffer less its
    // zero tail, so its capacity is not what is hashed.
    let sections = read(wood.sections(), wood.sections().size());
    let used = sections.iter().rposition(|&b| b != 0).map_or(0, |i| i + 1);
    format!(
        "leaves {} points {p} {} clusters {k} {} sections {} demand {}",
        debug(&leaves),
        hash(&points),
        hash(&clusters),
        hash(&sections[..used]),
        debug(&demand),
    )
}

/// Every shipped preset the device draws, at each of `DIGEST_SEEDS`
/// (1, 7 and 42 by default), built resident through the generator.
#[test]
#[ignore = "a digest"]
fn digest() {
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    // The renderer a still is drawn by, whose generator builds every tree.
    let mut renderer = crate::Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let seeds: Vec<u32> = std::env::var("DIGEST_SEEDS")
        .unwrap_or_else(|_| "1,7,42".into())
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    // `DIGEST_PRESETS` names the presets to digest (every one by default).
    let named = std::env::var("DIGEST_PRESETS").ok();
    let wanted = |id: &str| {
        named
            .as_deref()
            .is_none_or(|n| n.split(',').any(|p| p == id))
    };
    for &(_, id, _, _) in CATALOGUE.iter().filter(|c| wanted(c.1)) {
        for &seed in &seeds {
            let mut family = Preset::from_id(id).unwrap().parameters();
            family.skeleton.seed = seed;
            if !executor::stations(&family) {
                println!("{id} {seed}: not drawn on the device");
                continue;
            }
            let prepared = gen.prepare(&family).unwrap();
            println!("{id} {seed}: {}", line(&gen, &prepared));
            if let Ok(dir) = std::env::var("DIGEST_STILLS") {
                let path = format!("{dir}/{id}-{seed}.png");
                still(&mut renderer, (&family, prepared), &path);
            }
        }
    }
}

/// `DIGEST_STILLS` names a directory for each tree's still at the hero
/// pose, so two builds' trees are compared by eye and pixel (fn-293 task 6).
fn still(
    renderer: &mut crate::Renderer,
    (family, prepared): (&telperion_core::Family, Prepared),
    path: &str,
) {
    renderer.submit_prepared(prepared).unwrap();
    renderer.set_family(family);
    let bounds = renderer.bounds().expect("a tree drawn");
    let camera = crate::hero_pose(bounds, 960.0 / 720.0, crate::GROUND_REACH);
    let shot = crate::render(renderer, &camera, 960, 720).unwrap();
    crate::write_png(std::path::Path::new(path), &shot).unwrap();
}
