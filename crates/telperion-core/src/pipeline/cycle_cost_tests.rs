//! fn-327: what the one cycle grows, to the bit, and what each of its passes
//! costs. A digest here moves only where a change moved a tree.
use super::executor;
use crate::presets::{by_identity, Family, CATALOGUE};
use telperion_space::{grow_census, grow_cycle, tree_digest};

const AGE: f64 = 16.0;
const SEEDS: [u32; 3] = [1, 1234, 5678];
/// Young enough for an ordinary test: a table of every preset at both ends of
/// the dial, in the catalogue's order, finest first. fn-328 moved the oak's
/// and the spruce's coarsest, with their rows; fn-349's balance, which
/// charges living tips by their path and dead wood nothing, moved the
/// oak's finest and the birch's coarsest. fn-354's wood, laid by the leaf
/// above it, and its sag, 4ρg/E, moved every tree; its pass's skipping of
/// closed wood moved their sapwood there, every section kept. fn-354's
/// leaf account moved the spruce, whose needles shade and lay wood for
/// their 5.5 years while their wood is open, and the Two Trees, whose
/// leaf is their drawn leaf.
const PINNED: [(u64, u64); 8] = [
    (1233340031879171559, 1233340031879171559),
    (17687829558383956639, 11667850423448962601),
    (12287197284966836912, 1773060326561079923),
    (15759162824176722754, 15759162824176722754),
    (3753336579002065071, 11296798367218290338),
    (4799502948138528708, 4799502948138528708),
    (12978362334831825737, 12978362334831825737),
    (1161467971842050269, 1161467971842050269),
];

fn family(id: &str, age: f64, seed: u32, fidelity: f64) -> Family {
    let mut f = by_identity(id).expect("a preset");
    f.age = age;
    f.skeleton.seed = seed;
    f.fidelity = fidelity;
    f
}

fn digest(f: &Family) -> u64 {
    let g = executor::growth(f).expect("its growth");
    tree_digest(&grow_cycle(&g.species, g.request).expect("a tree").structure)
}

#[test]
fn every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial() {
    let read: Vec<(u64, u64)> = CATALOGUE
        .iter()
        .map(|&(_, id, ..)| {
            (
                digest(&family(id, AGE, 7, 0.0)),
                digest(&family(id, AGE, 7, 1.0)),
            )
        })
        .collect();
    assert_eq!(read, PINNED, "{read:?}");
}

/// The heavier read of R2: each tree's digest at its preset's age.
#[test]
#[ignore = "a measurement: fn-327 R2, read before and after"]
fn digests_at_preset_age() {
    for id in ["oregon-white-oak", "norway-spruce", "silver-birch"] {
        let age = by_identity(id).expect("a preset").age;
        for seed in SEEDS {
            for fidelity in [0.0, 1.0] {
                let d = digest(&family(id, age, seed, fidelity));
                println!("DIGEST {id} {age} {seed} {fidelity} {d}");
            }
        }
    }
}

/// Each pass's seconds and the cycle's work: `COST_PRESET`, its age, seed 1.
#[test]
#[ignore = "a measurement: fn-327 R1, one thread"]
fn pass_costs() {
    for id in ["oregon-white-oak", "norway-spruce", "silver-birch"] {
        let age = by_identity(id).expect("a preset").age;
        for fidelity in [0.0, 1.0] {
            let f = family(id, age, 1, fidelity);
            let g = executor::growth(&f).expect("its growth");
            let _warm = grow_census(&g.species, g.request).expect("a tree");
            let c = grow_census(&g.species, g.request).expect("a tree");
            let census = c.census.expect("a census");
            println!(
                "COST {id} {fidelity} seconds {:.4} grown {} axes {}",
                c.seconds, c.grown, census.axes
            );
            let (mut apex, mut axes, mut leaves) = (0u64, 0u64, 0u64);
            for y in &census.cycles {
                (apex, axes, leaves) = (
                    apex + y.active as u64,
                    axes + y.axes as u64,
                    leaves + y.leaves as u64,
                );
            }
            println!(
                "WORK {id} {fidelity} apex-cycles {apex} axis-cycles {axes} leaf-cycles {leaves}"
            );
            let mut phases: Vec<_> = census.phases.iter().collect();
            phases.sort_by(|a, b| b.1.total_cmp(a.1));
            for (name, s) in phases.into_iter().take(14) {
                println!("PASS {id} {fidelity} {name} {:.1}", s * 1000.0);
            }
        }
    }
}

/// fn-331 R3: with the phytomer budget gone, the oak at 200, seed 1, which
/// the budget refused at 80, builds wood and leaves through the pipeline on
/// the CPU; its seconds, the skeleton's share and the process's peak memory.
#[test]
#[ignore = "a measurement: fn-331 R3, minutes and gigabytes"]
fn the_oak_at_200_builds_with_no_phytomer_budget() {
    let mut f = by_identity("oregon-white-oak").expect("a preset");
    (f.age, f.skeleton.seed) = (200.0, 1);
    let started = std::time::Instant::now();
    let built = super::build(&f, super::Request::mesh()).expect("the oak at 200");
    let seconds = started.elapsed().as_secs_f64();
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let peak = status
        .lines()
        .find(|l| l.starts_with("VmHWM"))
        .unwrap_or("VmHWM: unknown");
    let skeleton = built.outputs.stages.skeleton_ms / 1e3;
    let nodes = built.skeleton.tree.nodes.len();
    println!("OAK200 seconds {seconds:.1} skeleton {skeleton:.1} nodes {nodes} {peak}");
}
