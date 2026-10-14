//! fn-256 task 5 (COUNCIL.md round 4): builds on a card other processes
//! fill end in a build or a refusal by name, never a reset. `ballast`
//! holds `BALLAST_MIB` of the card for `BALLAST_SECS` in a process of its
//! own; `ten_builds_on_a_full_card` builds the Two Trees, the oak at 80
//! and the birch at 40 in turn, ten in all, each under a wall-clock bound
//! (`full-card.sh` in the evidence runs both).
use super::super::{Delivery, Generator};
use crate::Renderer;

/// The host's rulings of 2026-10-12 and fn-270 R5c: a build whose least
/// tables the budget cannot hold is refused by name before anything is
/// submitted; one whose least tables fit starts from them and, past the
/// budget as it grows, is refused by name mid-build, its state spent and
/// the tree drawn before it untouched.
#[test]
fn a_build_past_the_budget_is_refused_by_name_and_leaves_the_drawn_tree() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    pollster::block_on(gen.prepare_programs()).unwrap();
    let family = |id: &str, age: f64| {
        let mut f = telperion_core::presets::by_identity(id).unwrap();
        f.age = age;
        f
    };
    let drawn = gen.prepare(&family("silver-birch", 8.0), Delivery::Resident).expect("a small tree draws");
    let words = gen.read_instances(&drawn).unwrap().leaves;
    let beech = family("european-beech", 40.0);
    // Below the least tables: refused before any submit.
    gen.budget.set(Some(gpu.ledger.live() + (1 << 20)));
    let before = gpu.ledger.submits();
    let refused = gen.prepare(&beech, Delivery::Resident).err().expect("refused");
    assert!(refused.to_string().contains("device budget"), "{refused}");
    assert_eq!(gpu.ledger.submits(), before, "a build its least tables cannot start submitted work");
    // The least tables fit, the growth does not: refused by name mid-build.
    gen.budget.set(Some(gpu.ledger.live() + (64 << 20)));
    let before = gpu.ledger.submits();
    let refused = gen.prepare(&beech, Delivery::Resident).err().expect("refused");
    assert!(refused.to_string().contains("device budget"), "{refused}");
    assert!(gpu.ledger.submits() > before, "the least tables fit, so the build started");
    gen.budget.set(None);
    assert_eq!(gen.read_instances(&drawn).unwrap().leaves, words, "the drawn tree moved");
}

#[test]
#[ignore = "device limits: a ballast for the full-card runs"]
fn ballast() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mib: u64 = std::env::var("BALLAST_MIB").expect("BALLAST_MIB").parse().unwrap();
    let secs: u64 = std::env::var("BALLAST_SECS").map_or(600, |s| s.parse().unwrap());
    let usage = wgpu::BufferUsages::STORAGE;
    let held: Vec<_> = (0..mib.div_ceil(256)).map(|_| super::super::io::buffer(&gpu, "ballast", 256 << 20, usage, &[]).unwrap()).collect();
    let mut e = gpu.encoder(&Default::default());
    held.iter().for_each(|b| e.clear_buffer(b, 0, None));
    gpu.submit([e.finish()]);
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).unwrap();
    println!("ballast holds {} MiB", held.len() * 256);
    std::thread::sleep(std::time::Duration::from_secs(secs));
}

#[test]
#[ignore = "device limits: ten builds on a card other processes fill"]
fn ten_builds_on_a_full_card() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let trees = [("telperion", 82.0), ("oregon-white-oak", 80.0), ("laurelin", 80.0), ("silver-birch", 40.0), ("ordinary", 40.0)];
    let mut ended = Vec::new();
    for k in 0..10 {
        let (id, age) = trees[k % trees.len()];
        let mut family = telperion_core::presets::by_identity(id).unwrap();
        family.age = age;
        let free = gpu.free_device_memory().map_or(0, |f| f >> 20);
        let t = std::time::Instant::now();
        let end = match gen.prepare(&family, Delivery::Resident) {
            Ok(p) => format!("built, {} instances", p.count()),
            Err(e) => format!("refused: {e}"),
        };
        let line = format!("{k} {id} {age}: {end} in {:.1} s ({free} MiB free before)", t.elapsed().as_secs_f64());
        println!("{line}");
        ended.push(line);
    }
    assert!(gpu.lost().is_none(), "the device was lost: {:?}", gpu.lost());
    let unnamed: Vec<_> = ended.iter().filter(|l| l.contains("refused") && !l.contains("device budget") && !l.contains("device refused")).collect();
    assert!(unnamed.is_empty(), "refusals not by the budget: {unnamed:#?}");
}
