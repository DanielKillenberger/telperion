//! fn-212 task 10, host decision 2: `Generator::prepare` settles a tree's
//! fine roots on the device wherever the realized settle takes it apart,
//! and grows the rest whole on the CPU executor, by the gap
//! `executor::apart` names. One generator builds every tree in turn, a
//! large one before small ones, as a page's dials do.
use super::super::{Delivery, Generator};
use crate::Renderer;
use telperion_core::pipeline::executor;

fn family(id: &str, age: u32) -> telperion_core::presets::Family {
    let mut family = telperion_core::presets::by_identity(id).unwrap();
    family.age = f64::from(age);
    family
}

#[test]
fn prepare_settles_fine_roots_on_the_device_where_the_tree_comes_apart() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let trees = [
        ("norway-spruce", 30, true),
        ("european-beech", 30, true),
        ("european-beech", 3, true),
        ("norway-spruce", 1, true),
        ("oregon-white-oak", 20, false),
        ("date-palm", 20, false),
        ("european-beech", 12, true),
    ];
    for (id, age, device) in trees {
        let family = family(id, age);
        assert_eq!(executor::apart(&family).is_ok(), device, "{id} at {age}");
        let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
        assert_eq!(prepared.wood.is_some(), device, "{id} at {age}");
        let whole = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident)).unwrap();
        assert_eq!(prepared.count(), whole.count(), "{id} at {age}: leaves");
        renderer.submit_prepared(prepared).unwrap();
        // Each tree draws: a frame's device error fails the still.
        if let Some(bounds) = renderer.bounds() {
            let camera = crate::hero_pose(bounds, 4.0 / 3.0, crate::GROUND_REACH);
            crate::render(&mut renderer, &camera, 320, 240).unwrap();
        }
    }
}

#[test]
fn a_cpu_delivery_grows_the_tree_whole() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let prepared = gen
        .prepare(&family("european-beech", 12), Delivery::Cpu)
        .unwrap();
    assert!(prepared.wood.is_none() && prepared.cpu_mesh().is_some());
}

/// R4 natively: one generator and renderer build the spruce at 80, then
/// smaller trees and the 80 again, each submitted, its leaves counted
/// against a fresh build's (`cargo test --profile ci -p telperion-render
/// large_to_small -- --ignored --nocapture`, under the GPU lock).
#[test]
#[ignore]
fn large_to_small() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for (age, seed) in [
        (80, 1),
        (80, 7),
        (80, 42),
        (40, 1),
        (12, 1),
        (80, 1),
        (1, 1),
        (80, 1),
    ] {
        let mut family = family("norway-spruce", age);
        family.skeleton.seed = seed;
        let t = std::time::Instant::now();
        let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
        let ms = t.elapsed().as_secs_f64() * 1e3;
        let leaves = prepared.count();
        renderer.submit_prepared(prepared).unwrap();
        let (pool, regions) = gen.fine_pool_bytes();
        let tree = Generator::tree_buffer_bytes(&renderer);
        println!(
            "spruce {age}: {} leaves in {ms:.0} ms; kept: pool {} MB, regions {} MB, tree {} MB",
            leaves,
            pool >> 20,
            regions >> 20,
            tree >> 20
        );
    }
}

/// fn-212 task 10, host decision 2: a build that fails on a device error
/// is reported by name, and the tree before it keeps drawing.
#[test]
fn a_failed_build_leaves_the_drawn_tree_drawing() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let first = family("european-beech", 12);
    renderer
        .submit_prepared(gen.prepare(&first, Delivery::Resident).unwrap())
        .unwrap();
    let aspect = 4.0 / 3.0;
    let camera = crate::hero_pose(renderer.bounds().unwrap(), aspect, crate::GROUND_REACH);
    let before = crate::render(&mut renderer, &camera, 320, 240).unwrap();
    gen.fail_next.set(true);
    let error = gen
        .prepare(&family("european-beech", 30), Delivery::Resident)
        .err()
        .expect("the build fails");
    assert!(error.to_string().contains("Validation"), "{error}");
    let after = crate::render(&mut renderer, &camera, 320, 240).unwrap();
    assert!(after.has_subject());
    assert_eq!(after.stats.triangles, before.stats.triangles);
    assert_eq!(after.stats.instances, before.stats.instances);
    // The next build draws again.
    let next = gen.prepare(&first, Delivery::Resident).unwrap();
    renderer.submit_prepared(next).unwrap();
    assert!(crate::render(&mut renderer, &camera, 320, 240)
        .unwrap()
        .has_subject());
}

/// fn-212 task 10, host decision 3: the device bytes a build peaks at, from
/// a fresh generator and renderer, against the predictor's expected kept
/// phytomers, and what stays held once its tree is drawn
/// (`cargo test --profile ci -p telperion-render budget_probe -- --ignored
/// --nocapture`, under the GPU lock).
#[test]
#[ignore]
fn budget_probe() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    for (id, age) in [
        ("european-beech", 40),
        ("european-beech", 80),
        ("norway-spruce", 1),
        ("norway-spruce", 12),
        ("norway-spruce", 20),
        ("norway-spruce", 40),
        ("norway-spruce", 60),
        ("norway-spruce", 80),
    ] {
        let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
        let gen = Generator::new(&renderer).unwrap();
        let family = family(id, age);
        let species = family.space.species.clone().unwrap();
        let kept = telperion_space::expected_kept(&species, age)
            .unwrap()
            .unwrap_or(0.0);
        let ledger = gpu.ledger.clone();
        ledger.take_peak();
        let base = ledger.live();
        let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
        renderer.submit_prepared(prepared).unwrap();
        if let Some(bounds) = renderer.bounds() {
            let camera = crate::hero_pose(bounds, 1280.0 / 900.0, crate::GROUND_REACH);
            crate::render(&mut renderer, &camera, 1280, 900).unwrap();
        }
        let peak = ledger.take_peak() - base;
        let held = ledger.live() - base;
        println!(
            "{id} {age}: expected kept {kept:.0}, peak {} MB ({:.0} B a phytomer), held {} MB",
            peak >> 20,
            peak as f64 / kept,
            held >> 20
        );
    }
}
