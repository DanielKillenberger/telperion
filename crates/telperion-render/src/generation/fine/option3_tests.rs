//! fn-212 task 7, host option 3 (2026-10-07): a bearer's kind counts every
//! fine root it bears as a child, and its fork reads each root's base
//! radius from phase 2. Two probes, reported, not held (`cargo test
//! --profile ci -p telperion-render option3 -- --ignored --nocapture`,
//! under the GPU lock, `STILLS=<dir>`):
//! - **the extra leaves at 30:** the real path's part of the tree with its
//!   kinds read as before option 3 (children it draws alone) stations
//!   exactly the leaves the parity found on runs the CPU path lacks, every
//!   one of them on a node whose kind option 3 moved;
//! - **the beech's moved runs:** the CPU path's tree at 30 under the rule
//!   before option 3 against its tree under option 3, the runs whose curve
//!   moved counted and drawn bare at a close camera, both rules.
use super::super::Generator;
use super::resident::{build, Tools};
use super::roots::kernels;
use crate::{Camera, Renderer, View};
use std::collections::HashSet;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::presets::Family;
use telperion_core::tree::{NodeKind, Tree};

fn family(id: &str, (age, seed): (u32, u32)) -> Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    (family.age, family.skeleton.seed) = (f64::from(age), seed);
    family
}

/// `tree` under the rule before option 3: a thin node is a twig where it
/// draws no child, and a bearer forks where a seam it bears is codominant.
/// Also the nodes whose kind that moves.
fn before(tree: &Tree) -> (Tree, Vec<usize>) {
    let mut t = tree.clone();
    let mut children = vec![0u32; t.nodes.len()];
    for n in &t.nodes[1..] {
        children[n.parent.unwrap() as usize] += 1;
    }
    let mut moved = Vec::new();
    for (i, n) in t.nodes.iter_mut().enumerate().skip(1) {
        if n.kind == NodeKind::Branch && children[i] == 0 {
            n.kind = NodeKind::Twig;
            moved.push(i);
        }
    }
    for a in &mut t.attachments {
        a.fork = (a.seams.iter()).any(|&s| tree.nodes[s as usize].codominant.is_some());
    }
    (t, moved)
}

/// The leaves `x` stations, and those on segments ending at `at`.
fn stations(x: &Expansion, at: &HashSet<[u64; 3]>) -> (u32, u32) {
    let (_, shared) = x.curve_with_contacts().unwrap();
    let s = x.compact_stations(&shared).unwrap().unwrap();
    let key = |p: Vec3| [p.x, p.y, p.z].map(f64::to_bits);
    let on = (s.segments.iter())
        .filter(|g| g.endpoints.iter().any(|&p| at.contains(&key(p))))
        .map(|g| g.count)
        .sum();
    (s.count, on)
}

#[test]
#[ignore]
fn option3_extra_leaves_at_30() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let k = kernels(&gpu);
    for id in ["european-beech", "norway-spruce"] {
        let family = family(id, (30, 1));
        let built = pollster::block_on(build(&gpu, &gen, &k, &Tools::new(&gpu), &family)).unwrap();
        let part = built.expansion.tree().clone();
        let (old, moved) = before(&part);
        let key = |p: Vec3| [p.x, p.y, p.z].map(f64::to_bits);
        let at: HashSet<[u64; 3]> = moved.iter().map(|&i| key(part.nodes[i].position)).collect();
        let now = stations(&executor::expand(part, &family).unwrap(), &at);
        let then = stations(&executor::expand(old, &family).unwrap(), &at);
        println!(
            "{id} (30, 1): kinds moved {}; the CPU block's leaves before {} and now {}, \
             before on a moved node {} and now {}",
            moved.len(),
            then.0,
            now.0,
            then.1,
            now.1
        );
    }
}

/// The runs of `a` whose points differ from `b`'s (the same tree, so the
/// same runs), and the most a radius moved (relative).
fn moved_runs(
    a: &telperion_core::surface::Curve,
    b: &telperion_core::surface::Curve,
) -> (Vec<usize>, f64) {
    let mut out = Vec::new();
    let mut most = 0.0f64;
    for (k, (r, s)) in a.runs.iter().zip(&b.runs).enumerate() {
        let (p, q) = (a.run_points(r), b.run_points(s));
        let off = (p.iter().zip(q))
            .map(|(x, y)| (x.radius - y.radius).abs() / y.radius.max(1e-12))
            .fold(0.0, f64::max);
        if off > 1e-9 || p.len() != q.len() {
            out.push(k);
            most = most.max(off);
        }
    }
    (out, most)
}

/// A camera 0.25 m out from `at`, looking in.
fn close(at: Vec3) -> Camera {
    let out = Vec3::new(at.x, 0.0, at.z).normalized();
    Camera {
        position: at + (out + Vec3::new(0.0, 0.4, 0.0)).normalized() * 0.25,
        target: at,
        field_of_view: 35.0,
        near: 0.01,
        far: 40.0,
    }
}

fn shoot(renderer: &mut Renderer, camera: &Camera, path: std::path::PathBuf) {
    renderer.set_view(View::Bare);
    let still = crate::render(renderer, camera, 1024, 768).unwrap();
    crate::write_png(&path, &still).unwrap();
    println!("{}", path.display());
}

/// The beech at 30: the CPU path's runs option 3 moves (its tree under
/// both rules), and the real path's part's (its bearers forked by no fine
/// root, as the part read them before option 3, against option 3); the
/// part's most moved run drawn bare before and after, and on the real
/// path and the CPU path.
#[test]
#[ignore]
fn option3_moved_runs_on_the_beech() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let dir = std::path::PathBuf::from(std::env::var("STILLS").expect("STILLS"));
    std::fs::create_dir_all(&dir).unwrap();
    let family = family("european-beech", (30, 1));
    let now = executor::grow(&family).unwrap().expansion().unwrap();
    let (old, moved) = before(now.tree());
    let (a, b) = (
        now.curve().unwrap(),
        executor::expand(old, &family).unwrap().curve().unwrap(),
    );
    let (runs, most) = moved_runs(&a, &b);
    println!(
        "CPU path: kinds moved {}, runs moved {} (radius up to {most:.3})",
        moved.len(),
        runs.len()
    );
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let built = pollster::block_on(build(
        &gpu,
        &gen,
        &kernels(&gpu),
        &Tools::new(&gpu),
        &family,
    ))
    .unwrap();
    let part = built.expansion.tree().clone();
    let mut bare = part.clone();
    bare.attachments.iter_mut().for_each(|a| a.fork = false);
    let after = executor::expand(part, &family).unwrap();
    let before = executor::expand(bare, &family).unwrap();
    let (c, d) = (after.curve().unwrap(), before.curve().unwrap());
    let (runs, most) = moved_runs(&c, &d);
    println!(
        "real path's part: runs moved {} (radius up to {most:.3})",
        runs.len()
    );
    let worst = (runs.iter().copied())
        .max_by(|&x, &y| {
            let off = |k: usize| {
                let (p, q) = (c.run_points(&c.runs[k]), d.run_points(&d.runs[k]));
                (p.iter().zip(q))
                    .map(|(u, v)| (u.radius - v.radius).abs() / v.radius)
                    .fold(0.0, f64::max)
            };
            off(x).total_cmp(&off(y))
        })
        .expect("a moved run");
    let points = c.run_points(&c.runs[worst]);
    let camera = close(points[0].centre);
    println!("its most moved run {worst} from {:?}", points[0].centre);
    for (x, side) in [(&before, "part-before"), (&after, "part-option3")] {
        let mut r = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        r.submit(&x.mesh().unwrap()).unwrap();
        shoot(&mut r, &camera, dir.join(format!("beech30-{side}.png")));
    }
    renderer.submit_prepared(built.prepared).unwrap();
    shoot(&mut renderer, &camera, dir.join("beech30-real-path.png"));
    let resident = crate::generation::Delivery::Resident;
    let prepared = pollster::block_on(gen.prepare_whole(&family, resident)).unwrap();
    renderer.submit_prepared(prepared).unwrap();
    shoot(&mut renderer, &camera, dir.join("beech30-cpu-path.png"));
}
