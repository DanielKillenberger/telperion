//! fn-276 R1: trees the CPU builds that the device refused (fn-272's
//! film, FILM.md defect 1), each built by the CPU and on the device in
//! one test: walk frames near the beech on the way to the Norway spruce
//! at 40 ("UnitRounds"), and the Laurelin at 20 ("Given").
use super::Generator;
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
        let cpu = telperion_core::mesh::build(family)
            .unwrap_or_else(|e| panic!("{name}: the CPU refused: {e}"));
        assert!(
            cpu.foliage_instances() > 0 || family.age < 2.0,
            "{name}: the CPU built a bare tree"
        );
        match gen.prepare(family) {
            Ok(p) => {
                println!(
                    "{name}: device {} instances, CPU {}",
                    p.count(),
                    cpu.foliage_instances()
                );
            }
            Err(e) => out.push(format!("{name}: {e}")),
        }
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
    }
    out
}

/// FILM.md defect 1: beech to Norway spruce at 40, near the beech, at
/// seeds 7 and 1, refused "UnitRounds (0x1)".
#[test]
fn walk_frames_near_the_beech_build_on_the_device() {
    let mut frames = Vec::new();
    for seed in [7, 1] {
        let (a, b) = (
            at("european-beech", 40.0, seed),
            at("norway-spruce", 40.0, seed),
        );
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

/// The film's walks (fn-272's `release.json`): each adjacent pair at its
/// age, and the shares R4 builds them at.
const WALKS: [(&str, &str, f64); 6] = [
    ("european-beech", "norway-spruce", 40.0),
    ("norway-spruce", "silver-birch", 40.0),
    ("silver-birch", "oregon-white-oak", 40.0),
    ("oregon-white-oak", "date-palm", 40.0),
    ("date-palm", "laurelin", 1.0),
    ("laurelin", "telperion", 80.0),
];
const T: [f64; 6] = [0.0, 0.01, 0.1, 0.5, 0.9, 1.0];

/// R4's cases: every shipped preset at every whole age to its own and to
/// 80, and the walk frames, at seeds 1, 7 and 42. `SWEEP_PRESETS` names a
/// comma list, `SWEEP_AGES` a range ("a-b") and `SWEEP_WALKS` 0 or 1 to
/// leave the walks out or take them alone; `SWEEP_ONLY` keeps the cases
/// whose name holds it.
fn sweep_cases() -> Vec<(String, Family)> {
    let only = std::env::var("SWEEP_PRESETS").ok();
    let range = std::env::var("SWEEP_AGES").ok();
    let range = range
        .as_deref()
        .and_then(|r| r.split_once('-'))
        .and_then(|(a, b)| Some((a.parse().ok()?, b.parse().ok()?)));
    let walks = std::env::var("SWEEP_WALKS").ok();
    let mut out = Vec::new();
    if walks.as_deref() != Some("1") {
        let ids = telperion_core::presets::CATALOGUE.iter().map(|c| c.1);
        for id in ids.filter(|id| only.as_ref().is_none_or(|o| o.split(',').any(|x| x == *id))) {
            let last = (by_identity(id).expect("a shipped preset").age.round() as u32).max(80);
            let (from, to) = range.unwrap_or((1, last));
            for age in from..=to.min(last) {
                out.extend(
                    [1, 7, 42]
                        .map(|seed| (format!("{id} {age} {seed}"), at(id, f64::from(age), seed))),
                );
            }
        }
    }
    if walks.as_deref() != Some("0") {
        for (a, b, age) in WALKS {
            for seed in [1, 7, 42] {
                let (x, y) = (at(a, age, seed), at(b, age, seed));
                for t in T {
                    let family = telperion_core::blend::families(&x, &y, t).expect("a walk frame");
                    out.push((format!("{a} to {b} at {age} t {t} {seed}"), family));
                }
            }
        }
    }
    let only = std::env::var("SWEEP_ONLY").ok();
    out.retain(|c| only.as_ref().is_none_or(|o| c.0.contains(o.as_str())));
    out
}

/// fn-276 R4: every case built on the device, none refused but by the
/// budget, by name.
#[test]
#[ignore = "R4's sweep: every preset at every age and the walk frames, on the device"]
fn every_tree_builds_on_the_device() {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    let mut refused = Vec::new();
    for (name, family) in sweep_cases() {
        let clock = super::Clock::now();
        // The cap this build is held to: the stated budget, or less where
        // other processes fill the card.
        let cap = super::resident::budget::native(&gpu, gpu.ledger.live()) >> 20;
        match gen.prepare(&family) {
            Ok(p) => println!(
                "{name}: {} instances, {:.0} ms, cap {cap} MiB",
                p.count(),
                clock.elapsed_ms()
            ),
            Err(e) if super::io::over_budget(&e) || e.to_string().contains("engine's budget") => {
                println!("{name}: refused by the budget (cap {cap} MiB): {e}")
            }
            Err(e) => {
                println!("{name}: REFUSED (cap {cap} MiB): {e}");
                refused.push(format!("{name}: {e}"));
            }
        }
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
    }
    assert!(refused.is_empty(), "{refused:#?}");
}

/// fn-276 task 2: the Norway spruce at 62, seed 1, started at its tables'
/// full margin, which the native 4 GiB held, and was refused at its first
/// units move (4,200 MiB); its start now leaves that move room. Under the
/// native budget, so it needs a card with 4 GiB free for it.
#[test]
#[ignore = "the spruce at 62 on the device under the native 4 GiB budget"]
fn the_spruce_at_62_starts_with_room_to_move() {
    let refused = refused_on_device(&[(
        "norway-spruce at 62, seed 1".into(),
        at("norway-spruce", 62.0, 1),
    )]);
    assert!(refused.is_empty(), "{refused:#?}");
}

/// Each family built on the device alone; the refusals, by family.
fn refused_on_device(families: &[(String, Family)]) -> Vec<String> {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return Vec::new();
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    let mut out = Vec::new();
    for (name, family) in families {
        let cap = super::resident::budget::native(&gpu, gpu.ledger.live()) >> 20;
        match gen.prepare(family) {
            Ok(p) => println!("{name}: {} instances, cap {cap} MiB", p.count()),
            Err(e) => out.push(format!("{name} (cap {cap} MiB): {e}")),
        }
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
    }
    out
}
