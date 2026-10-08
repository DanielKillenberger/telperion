//! Host decision 3 under realized mechanics (`landing.rs`): a fine root
//! that can reach the ground's landing zone is laid on the CPU, and no
//! other root's wood enters that zone, so the ground never acts on what
//! another executor lays.
use super::request;
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
    let rules = rules(species, age, request.budget).expect("rules");
    let predictor = Predictor::new(species, &rules.windows, age);
    let plan = Plan::default();
    let mut cpu = Cpu {
        species,
        rules: Some(&rules),
        plan,
        threads: crate::cores::available(),
        roots: Vec::new(),
    };
    let (settled, _) = crate::fine::step::now(settle_with(
        species,
        (request, &predictor, &rules),
        plan,
        &mut cpu,
        true,
    ))
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

/// Task 9: a root the spare cores grew ahead lands as the executor lays
/// its own, to the bit (`spare.rs`), on the beech, which lands its roots
/// before bending, and the spruce, which lands most once bent.
#[test]
fn a_root_grown_ahead_lands_as_the_executor_lays_it() {
    for (name, species) in [("beech", beech()), ("spruce", spruce())] {
        let request = request(30, 1);
        let rules = rules(&species, 30, request.budget).expect("rules");
        let predictor = Predictor::new(&species, &rules.windows, 30);
        let plan = Plan::default();
        let mut cpu = Cpu {
            species: &species,
            rules: Some(&rules),
            plan,
            threads: crate::cores::available(),
            roots: Vec::new(),
        };
        let at = (request, &predictor, &rules);
        let (settled, _) = crate::fine::step::now(settle_with(&species, at, plan, &mut cpu, true))
            .expect("settles");
        assert!(settled.ahead > 0, "{name}: none grown ahead");
        assert!(!settled.landed.is_empty(), "{name}: none landed");
        for (j, root) in &settled.landed {
            let k = settled.live[*j];
            assert_eq!(root.axes(), cpu.roots[k].axes(), "{name}: root {k}");
        }
        println!(
            "{name} 30 1: {} grown ahead, {} landed",
            settled.ahead,
            settled.landed.len()
        );
    }
}
