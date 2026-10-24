//! fn-302 R7: each stepped limit's most rise and least headroom, and the
//! builds that began again, over the film's walk frames and every
//! shipped preset at its default age and at 12 and 40 where younger, at
//! seeds 1, 7 and 42 (`cycle::tree::Survey`). `SURVEY_DIGEST=1` prints
//! each build's digest line (`resident::digest_tests::line`), so two
//! generators' builds are compared frame by frame; `SURVEY_ONLY` keeps
//! the builds whose name holds it.
//!
//!   [SURVEY_DIGEST=1] [SURVEY_ONLY=<text>] cargo test --profile ci -p telperion-render
//!   stepped_survey -- --ignored --nocapture
use super::cpu_builds_tests::at;
use super::cycle::tree::Survey;
use super::resident::digest_tests::line;
use super::Generator;
use crate::Renderer;
use telperion_core::presets::{by_identity, Family, CATALOGUE};

/// Every shipped preset at its default age and at 12 and 40 where
/// younger, at seeds 1, 7 and 42.
fn presets() -> Vec<(String, Family)> {
    let mut out = Vec::new();
    for &(_, id, ..) in CATALOGUE {
        let own = by_identity(id).expect("a shipped preset").age;
        let younger = [12.0, 40.0].into_iter().filter(|&a| a < own);
        let ages = std::iter::once(own).chain(younger);
        for age in ages {
            for seed in [1, 7, 42] {
                out.push((format!("{id} at {age}, seed {seed}"), at(id, age, seed)));
            }
        }
    }
    out
}

#[test]
#[ignore = "a survey: about 1,600 device builds"]
fn stepped_survey() {
    let _held = super::whole_tree();
    let Some(gpu) = super::test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    let digest = std::env::var("SURVEY_DIGEST").is_ok();
    let mut survey = Survey::default();
    let mut refused = Vec::new();
    let only = std::env::var("SURVEY_ONLY").unwrap_or_default();
    let builds = presets().into_iter().chain(super::film_walk_tests::sweep());
    for (name, family) in builds.filter(|(n, _)| n.contains(&only)) {
        match gen.prepare(&family) {
            Ok(p) if digest => println!("DIGEST {name}: {}", line(&gen, &p)),
            Ok(_) => {}
            Err(e) => refused.push(format!("{name}: {e}")),
        }
        gpu.device
            .poll(wgpu::PollType::wait_indefinitely())
            .expect("freed");
        survey.take(&name);
    }
    println!("{}", survey.report());
    assert!(refused.is_empty(), "{refused:#?}");
}
