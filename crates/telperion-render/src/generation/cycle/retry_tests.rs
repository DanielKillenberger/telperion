//! fn-302 (host, 2026-10-21): a build past only the limits that step with
//! the tree begins again once, from its first cycle, with them at their
//! bound, and hands the renderer what a build recorded at that bound from
//! its first cycle does; past them again, or past any other limit, it is
//! refused by name.
use super::super::cpu_builds_tests::at;
use super::super::io;
use super::super::resident::digest_tests::line;
use super::super::{test_gpu, whole_tree, Generator};
use super::build_grow::{retries, stop, Stop};
use super::census::Flag;
use super::forced::{with_pinned, with_short};
use super::gate::Pinned;
use super::growth::GrowthState;
use super::tree::Kernels;
use crate::Renderer;
use pollster::block_on;
use telperion_core::pipeline::executor;

/// A small tree whose hierarchy generations pass 1 in its first cycles.
fn beech() -> telperion_core::Family {
    at("european-beech", 12.0, 1)
}

/// The R6 proof: the hierarchy's rounds forced short begin the build
/// again once, and it is the build recorded at their bound throughout.
#[test]
fn a_build_past_its_stepped_limits_is_the_build_at_their_bound() {
    let _held = whole_tree();
    let Some(gpu) = test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    retries();
    let short = (Pinned::ROUNDS, false);
    let retried = with_short(short, || gen.prepare(&beech())).expect("built again");
    assert_eq!(retries(), 1, "begun again once");
    let retried = line(&gen, &retried);
    let bound = with_pinned(Pinned::ROUNDS, || gen.prepare(&beech())).expect("built");
    assert_eq!(retries(), 0, "at the bound from its first cycle");
    assert_eq!(retried, line(&gen, &bound));
}

/// R6's error case: limits held short through the retry are refused by
/// name after exactly one retry.
#[test]
fn a_build_past_its_stepped_limits_twice_is_refused_by_name() {
    let _held = whole_tree();
    let Some(gpu) = test_gpu() else {
        return;
    };
    let renderer = Renderer::new(gpu.clone(), crate::STILL_FORMAT);
    let gen = Generator::new(&renderer).expect("a generator");
    retries();
    let held = (Pinned::ROUNDS, true);
    let Err(refused) = with_short(held, || gen.prepare(&beech())) else {
        panic!("built past the limits held short");
    };
    assert_eq!(retries(), 1, "begun again once");
    assert!(refused.to_string().contains("HierGens"), "{refused}");
}

/// R5 on the growth path: a paused growth that began again once never
/// begins again, however many calls grow it on.
#[test]
fn a_growth_begins_again_once_in_its_life() {
    let _held = whole_tree();
    let Some(gpu) = test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let growth = executor::growth(&beech()).expect("a growth");
    let begun = GrowthState::begin(&gpu, &k, &growth, (growth.request.seed, 12), 4 << 30);
    let mut s = block_on(begun).expect("begun");
    retries();
    let short = (Pinned::ROUNDS, false);
    with_short(short, || block_on(s.grow_to(&gpu, &k, 6.0))).expect("grown again");
    assert_eq!(retries(), 1, "begun again once");
    // Every stepped limit held at 1 to the horizon.
    let every = [
        Pinned::UNITS,
        Pinned::DRAWS,
        Pinned::DEPOSITS,
        Pinned::SWEEPS,
        Pinned::PASSES,
        Pinned::UNIT_ROUNDS,
    ];
    let held = (every.into_iter().fold(Pinned::ROUNDS, Pinned::with), true);
    let refused = with_short(held, || block_on(s.grow_to(&gpu, &k, 12.0))).expect_err("refused");
    assert_eq!(retries(), 0, "never begun again");
    // A stepped limit (`HierPasses` at cycle 8): a first one would begin again.
    assert!(refused.to_string().contains("HierPasses"), "{refused}");
}

/// Only flags that each name a stepped limit begin a build again: a
/// table, the budget, a fault, the plan's cap or the workgroups given
/// refuse it, alone or beside them.
#[test]
fn only_stepped_limits_begin_a_build_again() {
    let said = || io::error("refused".into());
    let both = [(Flag::HierGens, 1), (Flag::UnitRounds, 1)];
    let p = Pinned::ROUNDS.with(Pinned::UNIT_ROUNDS);
    assert!(matches!(stop(&both, said()), Stop::Stepped(q, _) if q == p));
    for other in [
        Flag::Phyts,
        Flag::Budget,
        Flag::HierFault,
        Flag::LayGenerations,
        Flag::Given,
        Flag::Compaction,
    ] {
        assert!(Pinned::of(other).is_none(), "{other:?}");
        let mixed = [(Flag::HierGens, 1), (other, 1)];
        assert!(
            matches!(stop(&mixed, said()), Stop::Refused(_)),
            "{other:?}"
        );
    }
}
