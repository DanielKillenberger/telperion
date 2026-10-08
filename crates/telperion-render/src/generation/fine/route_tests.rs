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
    for age in [80, 40, 12, 80, 1, 80] {
        let family = family("norway-spruce", age);
        let t = std::time::Instant::now();
        let prepared = gen.prepare(&family, Delivery::Resident).unwrap();
        let ms = t.elapsed().as_secs_f64() * 1e3;
        println!("spruce {age}: {} leaves in {ms:.0} ms", prepared.count());
        renderer.submit_prepared(prepared).unwrap();
    }
}
