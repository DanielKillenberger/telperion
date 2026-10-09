//! The chain grower against the CPU reference executor (fn-209 F1, task
//! 2, R1): every chain of a tree, grown alone on the GPU from its bud and
//! the species point's tables, has the CPU's chain's axes (lineage, PA,
//! birth, how and where each stands, where its apex stopped), phytomers
//! (cycle and key) and woken buds, in the same order; in any order of the
//! chains and any size of batch.
use super::chains::{packed, Caps, Grower};
use super::hostrun::Grown;
use crate::Gpu;
use std::time::Instant;
use telperion_space::{
    chain_cases, grow_roots, ChainAxis, ChainBud, ChainTables, GeometryTables, Light, Plan,
    Request, Species,
};

/// A species, made.
type Make = fn() -> Species;

/// A region's first bounds; a chain past them is grown again in one
/// eight times larger.
const CAPS: Caps = Caps {
    axes: 256,
    phyts: 1024,
    live: 32,
    sleepers: 128,
    woken: 64,
    units: 1024,
};

/// What growing a tree's chains on the GPU took: milliseconds growing
/// (from upload to the device's finishing), packing and reading back, and
/// the chains grown again in larger regions.
#[derive(Debug, Default, Clone, Copy)]
struct Cost {
    grow: f64,
    read: f64,
    again: usize,
}

/// Grows `buds` in `order`, `batch` at a time: each chain's axes by its
/// index among `buds`.
fn grow_all(
    gpu: &Gpu,
    grower: &Grower,
    (buds, order, batch): (&[ChainBud], &[usize], usize),
    (tables, geo): (&ChainTables, &GeometryTables),
) -> (Vec<Vec<ChainAxis>>, Cost) {
    let packed = packed(tables, geo);
    // Presences decide nothing: topology reads none of them.
    let made = vec![[1.0f32; 2]; buds.len()];
    let at = (tables.span, tables.age);
    let mut grown: Grown = vec![None; buds.len()];
    let mut cost = Cost::default();
    let mut pending: Vec<usize> = order.to_vec();
    let (mut caps, mut size) = (CAPS, batch);
    while !pending.is_empty() {
        for part in pending.chunks(size) {
            let these: Vec<ChainBud> = part.iter().map(|&c| buds[c]).collect();
            let (got, g, r) = grower.run(gpu, (&these, &made[..these.len()]), &packed, at, caps);
            (cost.grow, cost.read) = (cost.grow + g, cost.read + r);
            for (&c, axes) in part.iter().zip(got) {
                grown[c] = axes;
            }
        }
        pending.retain(|&c| grown[c].is_none());
        cost.again += pending.len();
        (caps, size) = (caps.times(8), 64);
        assert!(caps.axes < 1 << 16, "a chain no region holds");
    }
    let axes = grown.into_iter().map(|g| g.expect("grown").axes).collect();
    (axes, cost)
}

/// The tree's chains as the CPU grows them, and the counts compared.
/// The grower keeps every axis (realized mechanics); P2's committed
/// shedding, which the CPU's chains carry, is not compared.
fn hold(cases: &[(ChainBud, Vec<ChainAxis>)], grown: &[Vec<ChainAxis>]) -> (usize, usize) {
    let (mut axes, mut phytomers) = (0, 0);
    let grown_whole = |a: &ChainAxis| ChainAxis {
        kept: true,
        context: None,
        ..a.clone()
    };
    for (c, ((bud, want), got)) in cases.iter().zip(grown).enumerate() {
        assert_eq!(got.len(), want.len(), "chain {c} ({bud:?}): axes");
        for (k, (g, w)) in got.iter().zip(want).enumerate() {
            assert_eq!(grown_whole(g), grown_whole(w), "chain {c} axis {k}");
        }
        axes += want.len();
        phytomers += want.iter().map(|a| a.phytomers.len()).sum::<usize>();
    }
    (axes, phytomers)
}

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The species with every setting a chain reads while no bud reads light
/// (FINE-CONTRACT.md's adversarial cases): the beech; a beech whose
/// lifespans run to fractions, which grows ages within a cycle
/// (`carry_on`) and partial units; the spruce, whose sleeping buds wake;
/// and the oak with its light switched off, whose sleeping buds wake into
/// later stages, also with its lifespans run to fractions; a beech whose
/// bud counts run to fractions, each last place drawn; and a beech whose
/// unlived ages live a tenth of a tick, each a unit cut to nothing
/// (fn-231).
fn adversarial() -> Vec<(&'static str, Species)> {
    let sub_tick = |mut s: Species| {
        for state in s.states.iter_mut().filter(|s| s.lifespan == 0.0) {
            state.lifespan = 0.1 / f64::from(1u32 << 20);
        }
        s
    };
    let fractional_buds = |mut s: Species| {
        for zone in s.states.iter_mut().flat_map(|s| &mut s.zones) {
            zone.buds = zone.buds * 1.3 + 0.2;
        }
        s
    };
    let fractional = |mut s: Species| {
        for state in s.states.iter_mut().filter(|s| s.lifespan > 0.0) {
            state.lifespan = state.lifespan * 1.37 + 0.29;
        }
        s
    };
    let mut oak = telperion_space::oak();
    for state in &mut oak.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    vec![
        ("beech", telperion_space::beech()),
        ("beech, fractional", fractional(telperion_space::beech())),
        ("beech, fractional buds", fractional_buds(telperion_space::beech())),
        ("beech, sub-tick ages", sub_tick(telperion_space::beech().canonical())),
        ("spruce", telperion_space::spruce()),
        ("oak, light off", oak.clone()),
        ("oak, light off, fractional", fractional(oak)),
    ]
}

#[test]
fn every_chain_grows_on_the_gpu_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let grower = Grower::new(&gpu);
    for (name, species) in adversarial() {
        for seed in [1, 7] {
            let cases = chain_cases(&species, request(25, seed)).expect("chains");
            let tables = ChainTables::new(&species, 25).expect("tables");
            let geo = GeometryTables::new(&species, 25).expect("tables");
            let buds: Vec<ChainBud> = cases.iter().map(|c| c.0).collect();
            let forward: Vec<usize> = (0..buds.len()).collect();
            let (grown, _) = grow_all(&gpu, &grower, (&buds, &forward, 4096), (&tables, &geo));
            let (axes, phytomers) = hold(&cases, &grown);
            // Reversed, a few hundred at a time: the same chains.
            let reverse: Vec<usize> = forward.iter().rev().copied().collect();
            let (again, _) = grow_all(&gpu, &grower, (&buds, &reverse, 333), (&tables, &geo));
            assert!(again == grown, "{name} {seed}: the same in any order");
            println!(
                "{name} 25 {seed}: {} chains, {axes} axes, {phytomers} phytomers",
                buds.len()
            );
        }
    }
}

/// The beech and spruce at 40 and 80, seeds 1 and 7, held against the CPU,
/// in two orders and two batch sizes, with their costs:
/// `cargo test --profile ci -p telperion-render chains_at_40_and_80 --
/// --ignored --nocapture`.
#[test]
#[ignore = "device limits: a measurement over the old trees"]
fn chains_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let t = Instant::now();
    let grower = Grower::new(&gpu);
    println!(
        "pipelines built in {:.1} ms",
        t.elapsed().as_secs_f64() * 1e3
    );
    println!("| Tree | Chains | Axes | Phytomers | Tables ms | GPU grow cold / warm ms | Read ms | Again | µs a chain (warm) | CPU chains 1 core ms | CPU fine, all cores ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let species: [(&str, Make); 2] = [
        ("beech", telperion_space::beech),
        ("spruce", telperion_space::spruce),
    ];
    for (name, make) in species {
        let species = make();
        for age in [40, 80] {
            for seed in [1, 7] {
                let t = Instant::now();
                let cases = chain_cases(&species, request(age, seed)).expect("chains");
                let cpu = t.elapsed().as_secs_f64() * 1e3;
                let t = Instant::now();
                grow_roots(&species, request(age, seed), Plan::default()).expect("grows");
                let fine = t.elapsed().as_secs_f64() * 1e3;
                let t = Instant::now();
                let tables = ChainTables::new(&species, age).expect("tables");
                let geo = GeometryTables::new(&species, age).expect("tables");
                let tables_ms = t.elapsed().as_secs_f64() * 1e3;
                let buds: Vec<ChainBud> = cases.iter().map(|c| c.0).collect();
                let forward: Vec<usize> = (0..buds.len()).collect();
                let (grown, cold) =
                    grow_all(&gpu, &grower, (&buds, &forward, 4096), (&tables, &geo));
                let (axes, phytomers) = hold(&cases, &grown);
                let reverse: Vec<usize> = forward.iter().rev().copied().collect();
                let (again, warm) =
                    grow_all(&gpu, &grower, (&buds, &reverse, 1000), (&tables, &geo));
                assert!(again == grown, "{name} {age} {seed}: the same in any order");
                println!(
                    "| {name} {age} {seed} | {} | {axes} | {phytomers} | {tables_ms:.0} | {:.0} / {:.0} | {:.0} | {} | {:.2} | {cpu:.0} | {fine:.0} |",
                    buds.len(),
                    cold.grow,
                    warm.grow,
                    warm.read,
                    warm.again,
                    warm.grow * 1e3 / buds.len() as f64,
                );
            }
        }
    }
}
