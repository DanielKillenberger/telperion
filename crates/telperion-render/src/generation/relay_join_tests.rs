//! fn-303 (host, 2026-10-22): every relay join the CPU's structure holds,
//! over each shipped preset at its default age and at 12 and 40 where
//! younger, at seeds 1, 7 and 42, and every walk frame with
//! `RELAY_SWEEP=1`. It counts the joins whose t is exactly 1, those within
//! 1e-4 of it and those further inside, and the least t.
//!
//!   [RELAY_SWEEP=1] cargo test --profile ci -p telperion-render relay_joins -- --ignored --nocapture
use super::cpu_builds_tests::at;
use telperion_core::pipeline::executor;
use telperion_core::presets::{by_identity, Family, CATALOGUE};

/// Each shipped preset at its default age and at 12 and 40 where younger,
/// at seeds 1, 7 and 42.
fn presets() -> Vec<(String, Family)> {
    let mut out = Vec::new();
    for &(_, id, ..) in CATALOGUE {
        let own = by_identity(id).expect("a shipped preset").age;
        let younger = [12.0, 40.0].into_iter().filter(|&a| a < own);
        for age in std::iter::once(own).chain(younger) {
            for seed in [1, 7, 42] {
                out.push((format!("{id} at {age}, seed {seed}"), at(id, age, seed)));
            }
        }
    }
    out
}

#[test]
#[ignore = "a survey: about 2 minutes over the presets, an hour with the walk frames"]
fn relay_joins() {
    let mut families = presets();
    if std::env::var("RELAY_SWEEP").is_ok() {
        families.extend(super::film_walk_tests::sweep());
    }
    let (mut joins, mut whole, mut near, mut inside) = (0usize, 0usize, 0usize, 0usize);
    let mut least = (f32::INFINITY, String::new());
    for (name, family) in &families {
        let growth = executor::growth(family).expect("its growth");
        let s = telperion_space::grow(&growth.species, growth.request).expect("its structure");
        for (a, axis) in s.axes.iter().enumerate() {
            let telperion_space::Origin::Relay { .. } = axis.origin else {
                continue;
            };
            let Some(j) = axis.joined() else { continue };
            joins += 1;
            whole += usize::from(j.t == 1.0);
            near += usize::from(j.t < 1.0 && j.t >= 1.0 - 1e-4);
            inside += usize::from(j.t < 1.0 - 1e-4);
            if j.t < least.0 {
                least = (j.t, format!("{name}, axis {a}"));
            }
        }
    }
    println!(
        "{} families: {joins} relay joins; t == 1: {whole}; t in [1 - 1e-4, 1): {near}; \
         t < 1 - 1e-4: {inside}; least t {} ({})",
        families.len(),
        least.0,
        least.1
    );
}
