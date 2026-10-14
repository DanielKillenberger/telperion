//! fn-276 R1: trees the CPU builds that the device refused (fn-272's
//! film, FILM.md defect 1), each built by the CPU and on the device in
//! one test: walk frames near the beech on the way to the Norway spruce
//! at 40 ("UnitRounds"), and the Laurelin at 20 ("Given").
use super::{Backend, Delivery, Generator};
use crate::Renderer;
use telperion_core::presets::{by_identity, Family};

/// A shipped preset at `age` and `seed`, as the film's walks take them.
fn at(id: &str, age: f64, seed: u32) -> Family {
    let mut f = by_identity(id).expect("a shipped preset");
    (f.age, f.skeleton.seed) = (age, seed);
    f
}

/// Each family built by the CPU, then on the device; the device's
/// refusals, by family.
fn refused(families: &[(String, Family)]) -> Vec<String> {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return Vec::new();
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    let mut out = Vec::new();
    for (name, family) in families {
        let cpu = telperion_core::mesh::build(family).unwrap_or_else(|e| panic!("{name}: the CPU refused: {e}"));
        assert!(cpu.foliage_instances() > 0 || family.age < 2.0, "{name}: the CPU built a bare tree");
        match gen.prepare(family, Delivery::Resident) {
            Ok(p) => {
                assert_eq!(p.backend, Backend::Gpu, "{name}: built off the device");
                println!("{name}: device {} instances, CPU {}", p.count(), cpu.foliage_instances());
            }
            Err(e) => out.push(format!("{name}: {e}")),
        }
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("freed");
    }
    out
}

/// FILM.md defect 1: beech to Norway spruce at 40, near the beech, at
/// seeds 7 and 1, refused "UnitRounds (0x1)".
#[test]
fn walk_frames_near_the_beech_build_on_the_device() {
    let mut frames = Vec::new();
    for seed in [7, 1] {
        let (a, b) = (at("european-beech", 40.0, seed), at("norway-spruce", 40.0, seed));
        for t in [0.00037, 0.01, 0.1] {
            let family = telperion_core::blend::families(&a, &b, t).expect("a walk frame");
            frames.push((format!("beech to spruce at 40, seed {seed}, t {t}"), family));
        }
    }
    let refused = refused(&frames);
    assert!(refused.is_empty(), "{refused:#?}");
}

/// FILM.md defect 1: the Laurelin at 20, alone, refused "Given (0x1)".
#[test]
fn the_laurelin_at_20_builds_on_the_device() {
    let refused = refused(&[("laurelin at 20, seed 7".into(), at("laurelin", 20.0, 7))]);
    assert!(refused.is_empty(), "{refused:#?}");
}
