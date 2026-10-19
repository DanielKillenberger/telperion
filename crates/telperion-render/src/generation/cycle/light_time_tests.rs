//! fn-256 task 13, measured (`#[ignore = "a measurement"]`): each shipped
//! preset at its default age, seed 42, grown as a build grows it (the
//! hinted arena, a move where the tree asks one), warm, then again with
//! every dispatch stamped: the light phase's GPU time by kernel, beside
//! the build's stamped total. `LIGHT_CASES` names other cases
//! (`id:age:seed,...`). `cargo test --profile ci -p telperion-render
//! the_light_phase_is_timed -- --ignored --nocapture`, under the GPU lock.
use super::build::Tables;
use super::build_arena::hint;
use super::build_grow::{grow_now as grow, Sizes};
use super::grown_tests::{device, grown_from};
use telperion_core::presets::{Preset, CATALOGUE};

/// The light phase's dispatches, by kernel: its ms and dispatches.
fn light(stamped: &[&'static str], stamps: &[(String, f64, f64)]) -> Vec<(String, f64, usize)> {
    let mut out: Vec<(String, f64, usize)> = Vec::new();
    for (_, (kernel, a, z)) in stamped.iter().zip(stamps).filter(|(p, _)| **p == "light") {
        let ms = (z - a) * 1e-6;
        match out.iter_mut().find(|k| k.0 == *kernel) {
            Some(k) => (k.1, k.2) = (k.1 + ms, k.2 + 1),
            None => out.push((kernel.clone(), ms, 1)),
        }
    }
    out
}

fn cases() -> Vec<(&'static str, u32, u64)> {
    let named = |id: &str| {
        CATALOGUE
            .iter()
            .map(|c| c.1)
            .find(|c| *c == id)
            .expect("a shipped preset")
    };
    match std::env::var("LIGHT_CASES") {
        Ok(v) => v
            .split(',')
            .map(|c| {
                let p: Vec<&str> = c.split(':').collect();
                (
                    named(p[0]),
                    p[1].parse().expect("an age"),
                    p[2].parse().expect("a seed"),
                )
            })
            .collect(),
        Err(_) => CATALOGUE
            .iter()
            .map(|c| {
                (
                    c.1,
                    Preset::from_id(c.1).expect("a preset").parameters().age as u32,
                    42,
                )
            })
            .collect(),
    }
}

#[test]
#[ignore = "a measurement"]
fn the_light_phase_is_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, kit)) = device() else {
        return;
    };
    for (id, age, seed) in cases() {
        let growth = telperion_core::pipeline::executor::growth(
            &Preset::from_id(id).expect("a preset").parameters(),
        )
        .expect("a growth");
        let g = grown_from(growth.species, growth.request.light, (age, seed));
        let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
        let hinted = Sizes::hinted(hint(&g.species, age).expect("a hint")).expect("sized");
        drop(grow(&gpu, &kit, &t, &g.tables, (hinted, age)).expect("a build"));
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
        assert!(super::super::profile::start(&gpu), "timestamps");
        let (b, _, grew) = grow(&gpu, &kit, &t, &g.tables, (hinted, age)).expect("a build");
        let stamps = super::super::profile::finish(&gpu);
        assert_eq!(stamps.len(), grew.stamped.len(), "a phase a stamp");
        let total: f64 = stamps.iter().map(|(_, a, z)| (z - a) * 1e-6).sum();
        let kernels = light(&grew.stamped, &stamps);
        let (ms, n) = kernels.iter().fold((0.0, 0), |s, k| (s.0 + k.1, s.1 + k.2));
        let by: Vec<String> = kernels
            .iter()
            .map(|k| format!("{} {:.1} ({})", k.0, k.1, k.2))
            .collect();
        println!("{id} {age} seed {seed}: light {ms:.1} ms in {n} dispatches of {total:.1} ms stamped, {} dispatches; dormant {}, moved {}; {}", stamps.len(), b.light.dormant, grew.moves, by.join(", "));
        drop(b);
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
    }
}
