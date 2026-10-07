//! Host decision 3 under realized mechanics (`landing.rs`): a fine root
//! that can reach the ground's landing zone is laid on the CPU, and no
//! other root's wood enters that zone, so the ground never acts on what
//! another executor lays.
use super::request;
use crate::bounded::Expected;
use crate::closed_form::predictor::Predictor;
use crate::fine::expansion::settle_with;
use crate::fine::realized::Cpu;
use crate::fine::Plan;
use crate::geometry::ground_zone;
use crate::grow::alone::rules;
use crate::species::Species;
use crate::{beech, spruce};

/// The live roots, those laid on the CPU, and those whose wood came into
/// the landing zone though they were not.
fn routed(species: &Species, age: u32, seed: u64) -> (usize, usize, usize) {
    let request = request(age, seed);
    let expected = Expected::new(species, age, 0);
    let (rules, rows) = (
        rules(species, age, request.budget).expect("rules"),
        expected.rows(),
    );
    let predictor = Predictor::new(species, &rules.windows, age);
    let plan = Plan::default();
    let mut cpu = Cpu {
        species,
        rules: &rules,
        plan,
        threads: crate::cores::available(),
        roots: Vec::new(),
    };
    let (settled, _) = settle_with(
        species,
        (request, &predictor, &rules),
        plan,
        &mut cpu,
        Some(&rows),
    )
    .expect("settles");
    let near = settled
        .live
        .iter()
        .filter(|k| !settled.grounded.contains(k))
        .filter(|&&k| {
            cpu.roots[k].axes()[1..].iter().any(|axis| {
                let mut base = axis.base;
                axis.phytomers.iter().any(|p| {
                    let length = (p.tip - base).length();
                    let low = base.z.min(p.tip.z);
                    base = p.tip;
                    low < ground_zone(p.radius, length)
                })
            })
        })
        .count();
    (settled.live.len(), settled.grounded.len(), near)
}

#[test]
fn a_root_that_can_reach_the_ground_is_laid_on_the_cpu() {
    for (name, species) in [("beech", beech()), ("spruce", spruce())] {
        for seed in [1, 7] {
            let (live, grounded, near) = routed(&species, 30, seed);
            println!("{name} 30 {seed}: {live} live roots, {grounded} on the CPU");
            assert!(grounded > 0, "{name} {seed}: no root routed");
            assert_eq!(
                near, 0,
                "{name} {seed}: a root not routed came near the ground"
            );
        }
    }
}
