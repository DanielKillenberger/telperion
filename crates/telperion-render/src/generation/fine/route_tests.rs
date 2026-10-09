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
#[ignore = "device limits: a tree at 80 on the device"]
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

/// fn-235 R3: a build over the device budget is refused by name before
/// it starts, and the tree drawn keeps drawing.
#[test]
fn a_build_over_the_budget_is_refused_and_the_drawn_tree_kept() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let first = family("european-beech", 12);
    renderer
        .submit_prepared(gen.prepare(&first, Delivery::Resident).unwrap())
        .unwrap();
    let aspect = 4.0 / 3.0;
    let camera = crate::hero_pose(renderer.bounds().unwrap(), aspect, crate::GROUND_REACH);
    let before = crate::render(&mut renderer, &camera, 320, 240).unwrap();
    // A budget the drawn tree all but fills.
    gen.budget.set(Some(gpu.ledger.live() + (1 << 20)));
    let error = gen
        .prepare(&family("european-beech", 30), Delivery::Resident)
        .err()
        .expect("the build is refused");
    assert!(error.to_string().contains("device budget"), "{error}");
    let after = crate::render(&mut renderer, &camera, 320, 240).unwrap();
    assert!(after.has_subject());
    assert_eq!(after.stats.triangles, before.stats.triangles);
    assert_eq!(after.stats.instances, before.stats.instances);
}

/// fn-235 R3: a device lost under a drawn tree is named by the next frame
/// and the next build, and the same parameters build again, once, on a
/// fresh adapter and device, as the page's "retry build" asks.
#[test]
fn a_lost_device_is_named_and_a_fresh_one_builds_the_tree_again() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let tree = family("european-beech", 12);
    let mut renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    renderer
        .submit_prepared(gen.prepare(&tree, Delivery::Resident).unwrap())
        .unwrap();
    let camera = crate::hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, crate::GROUND_REACH);
    gpu.device.destroy();
    let lost = |error: Option<crate::RenderError>| match error {
        Some(e @ crate::RenderError::DeviceLost { .. }) => e.to_string(),
        other => panic!("not named as a loss: {other:?}"),
    };
    let drawn = lost(crate::render(&mut renderer, &camera, 320, 240).err());
    assert!(drawn.contains("the GPU device was lost"), "{drawn}");
    lost(gen.prepare(&tree, Delivery::Resident).err());
    // The retry: the lost device let go, a fresh adapter and device, the
    // same parameters.
    drop((gen, renderer, gpu));
    let fresh = crate::generation::test_gpu().unwrap();
    let mut renderer = Renderer::new(fresh, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    renderer
        .submit_prepared(gen.prepare(&tree, Delivery::Resident).unwrap())
        .unwrap();
    let still = crate::render(&mut renderer, &camera, 320, 240).unwrap();
    assert!(still.has_subject());
}

/// fn-212 task 10, host decision 3: the device bytes a build peaks at, from
/// a fresh generator and renderer, against the predictor's expected kept
/// phytomers, and what stays held once its tree is drawn
/// (`cargo test --profile ci -p telperion-render budget_probe -- --ignored
/// --nocapture`, under the GPU lock).
#[test]
#[ignore = "device limits: a tree at 80 on the device"]
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

/// fn-212 task 11, Astra's finding 2: a device held to WebGPU's default
/// limits cannot bind the grower's regions; the build is refused by
/// name, never a panic.
#[test]
fn a_buffer_over_the_devices_limit_is_refused_by_name() {
    let held = crate::Gpu::request_within(None, |_| wgpu::Limits::default());
    let Some(gpu) = pollster::block_on(held).ok() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let built = gen.prepare(&family("norway-spruce", 30), Delivery::Resident);
    match built {
        Err(crate::RenderError::Oversize { .. }) => {}
        other => panic!("not refused by name: {:?}", other.err()),
    }
}

/// fn-212 task 11, Astra's finding 4: the authored foliage budget holds
/// the whole tree's leaves, the device's block with the CPU's, and is
/// refused in the CPU reference's words.
#[test]
fn the_foliage_budget_holds_the_whole_trees_leaves() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let mut family = family("european-beech", 30);
    let count = gen.prepare(&family, Delivery::Resident).unwrap().count();
    family.canopy.max_instances = count as usize - 1;
    let whole = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident));
    let whole = whole.err().expect("the CPU reference refuses").to_string();
    assert!(whole.contains("foliage instance budget"), "{whole}");
    let device = gen.prepare(&family, Delivery::Resident).err();
    assert_eq!(device.map(|e| e.to_string()), Some(whole));
    // A budget of exactly the tree's leaves holds it.
    family.canopy.max_instances = count as usize;
    assert_eq!(
        gen.prepare(&family, Delivery::Resident).unwrap().count(),
        count
    );
}

/// fn-212 task 11, Astra's finding 7: a canopy of zero leaf size places
/// no leaves on the device, as the CPU reference places none.
#[test]
fn a_zero_leaf_size_places_no_leaves() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let mut renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let mut family = family("european-beech", 30);
    family.canopy.size = 0.0;
    let whole = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident)).unwrap();
    assert_eq!(whole.count(), 0);
    let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
    assert!(prepared.wood.is_some(), "not the device path");
    assert_eq!(prepared.count(), 0);
    // The leafless tree draws.
    renderer.submit_prepared(prepared).unwrap();
    let camera = crate::hero_pose(renderer.bounds().unwrap(), 4.0 / 3.0, crate::GROUND_REACH);
    assert!(crate::render(&mut renderer, &camera, 320, 240)
        .unwrap()
        .has_subject());
}

/// fn-212 task 11, Astra's finding 8: what the CPU path draws, the
/// device path draws: leaves that seat on no wood, round or lobed, and
/// leaves seated on lobed wood (host, 2026-10-08: lobed ring vertices on
/// the device), each the whole path's leaves.
#[test]
fn the_device_path_draws_what_the_cpu_path_draws() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu, crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    for (contact, lobes) in [(0.0, 0), (0.0, 2), (1.0, 2)] {
        let mut family = family("european-beech", 30);
        family.canopy.surface_contact = contact;
        family.surface.lobes = lobes;
        family.surface.lobe_depth = 0.1;
        let at = format!("contact {contact}, lobes {lobes}");
        let whole = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident));
        let whole = whole.unwrap_or_else(|e| panic!("{at}: whole: {e}"));
        let prepared = gen.prepare(&family, Delivery::Resident);
        let prepared = prepared.unwrap_or_else(|e| panic!("{at}: device: {e}"));
        assert!(prepared.wood.is_some(), "{at}: not the device path");
        assert_eq!(prepared.count(), whole.count(), "{at}: leaves");
    }
}

/// fn-229: a flowering tree comes apart, and the device draws its own
/// roots' florets, the CPU block's beside them in one organ buffer: the
/// resident, whole and CPU deliveries draw the same number of florets (at
/// fn-216's merge the device path drew 1,900 of the beech's 2,643).
#[test]
fn a_flowering_tree_draws_every_floret_on_either_path() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).unwrap();
    let mut family = family("european-beech", 12);
    for s in &mut family.space.species.as_mut().unwrap().states {
        s.flowering = 0.6;
    }
    assert!(
        executor::apart(&family).is_ok(),
        "a flowering beech comes apart"
    );
    let florets = |p: &super::super::Prepared| {
        let held = p.mesh.organ.as_ref().map_or(0, |o| o.instances.len());
        held + p.organ.as_ref().map_or(0, |o| o.count as usize)
    };
    let resident = gen.prepare(&family, Delivery::Resident).unwrap();
    let whole = pollster::block_on(gen.prepare_whole(&family, Delivery::Resident)).unwrap();
    let cpu = gen.prepare(&family, Delivery::Cpu).unwrap();
    assert!(florets(&whole) > 0, "the beech flowers");
    assert!(
        resident.organ.is_some(),
        "the device path holds its florets"
    );
    assert_eq!(florets(&resident), florets(&whole));
    assert_eq!(florets(&cpu), florets(&whole));
}
