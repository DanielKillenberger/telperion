//! Fine roots expanded on the GPU, wave by wave (fn-209 F1, task 2, R1):
//! each root's chains grown on the device from its record, the buds a
//! wave's kept axes bear grown in the next, until none is left; the
//! roots' kept axes, compared by lineage with the CPU reference
//! executor's expansion of the same records before any geometry (host
//! decision 2), in any order of the roots and any batch size. The overflow
//! rule is the chain: an item is split along its chains, each grown from
//! its own bud's record, and a chain that outgrows its region is grown
//! again in a larger one.
use super::chains::{packed, Caps, Grew, Grower};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{
    fine_cases, grow_fine, ChainAxis, ChainBud, ChainTables, Expected, Light, Plan, Request,
    Species, Woke,
};

const AXW: usize = 32;
/// A chain's first region: most chains are a few axes, so a small region
/// and a large batch; one that outgrows it is grown again in a region
/// eight times larger, in batches an eighth the size.
const CAPS: Caps = Caps {
    axes: 32,
    phyts: 64,
    live: 8,
    sleepers: 16,
    woken: 8,
};

/// A chain to grow: its root's index, its bud, and its bearer among the
/// root's kept axes (none: the root's own bud, borne above the cut).
struct Item {
    root: usize,
    bud: ChainBud,
    bearer: Option<usize>,
}

/// What expanding a tree's roots took: waves and chains, milliseconds
/// growing (upload to the device's finishing) and reading back, and the
/// chains grown again in larger regions.
#[derive(Debug, Default, Clone, Copy)]
struct Cost {
    waves: usize,
    chains: usize,
    grow: f64,
    read: f64,
    again: usize,
}

/// A bud a chain bears, from its axis's words, as its own chain takes it.
fn bud(words: &[u32], i: usize, context: Option<i64>) -> ChainBud {
    let w = |f: usize| words[i * AXW + f];
    let wide = |f: usize| u64::from(w(f)) | u64::from(w(f + 1)) << 32;
    let kind = w(4);
    let birth = w(3);
    ChainBud {
        seed: false,
        lineage: wide(0),
        pa: w(2) as usize,
        birth,
        node: w(6) as usize,
        slot: (kind >> 8 & 0xff) as u8,
        root: wide(8),
        horizon: wide(10) as i64,
        woke: (kind & 0x80000 != 0).then(|| Woke {
            key: wide(8),
            pa: w(31) as usize,
            sleep: u64::from(w(19)),
            stage: w(20) as usize,
            spent: wide(12),
            cycle: birth + 1,
        }),
        context,
    }
}

/// Grows `items`, `batch` at a time, each outgrown chain again in a region
/// eight times larger.
fn grow(
    (gpu, grower): (&Gpu, &Grower),
    items: &[Item],
    batch: usize,
    tables: &ChainTables,
    cost: &mut Cost,
) -> Vec<Grew> {
    let packed = packed(tables);
    let at = (tables.span, tables.age);
    let buds: Vec<ChainBud> = items.iter().map(|i| i.bud).collect();
    let mut grown: Vec<Option<Grew>> = vec![None; buds.len()];
    let mut pending: Vec<usize> = (0..buds.len()).collect();
    let (mut caps, mut size) = (CAPS, batch);
    while !pending.is_empty() {
        for part in pending.chunks(size) {
            let these: Vec<ChainBud> = part.iter().map(|&c| buds[c]).collect();
            let (got, g, r) = grower.run(gpu, &these, &packed, at, caps);
            (cost.grow, cost.read) = (cost.grow + g, cost.read + r);
            for (&c, chain) in part.iter().zip(got) {
                grown[c] = chain;
            }
        }
        pending.retain(|&c| grown[c].is_none());
        cost.again += pending.len();
        (caps, size) = (caps.times(8), (size / 8).max(64));
        assert!(caps.axes < 1 << 16, "a chain no region holds");
    }
    grown.into_iter().map(|g| g.expect("grown")).collect()
}

/// Expands every root of `buds`, taken in `order`, `batch` chains at a
/// time: each root's kept axes, by its index.
fn expand(
    gpu: (&Gpu, &Grower),
    (buds, order, batch): (&[ChainBud], &[usize], usize),
    tables: &ChainTables,
) -> (Vec<Vec<ChainAxis>>, Cost) {
    let mut roots: Vec<Vec<ChainAxis>> = vec![Vec::new(); buds.len()];
    let mut cost = Cost::default();
    let mut wave: Vec<Item> = order
        .iter()
        .map(|&k| Item {
            root: k,
            bud: buds[k],
            bearer: None,
        })
        .collect();
    while !wave.is_empty() {
        cost.waves += 1;
        cost.chains += wave.len();
        let grown = grow(gpu, &wave, batch, tables, &mut cost);
        let mut next = Vec::new();
        for (item, chain) in wave.iter().zip(&grown) {
            let axes = &chain.axes;
            if !axes[0].kept {
                continue;
            }
            // The chain's kept own axes join its root's, each on its
            // parent's place there; the stub stands for the item's bearer.
            let mut at = vec![None; axes.len()];
            let kept = &mut roots[item.root];
            for (i, a) in axes.iter().enumerate() {
                let own = i == 0 || a.kind != 1;
                if !own || !a.kept {
                    continue;
                }
                let parent = a.parent.map_or(item.bearer, |p| at[p]);
                at[i] = Some(kept.len());
                kept.push(ChainAxis {
                    parent,
                    context: None,
                    ..a.clone()
                });
            }
            for (i, a) in axes.iter().enumerate().skip(1) {
                let bearer = a.parent.and_then(|p| at[p]);
                if a.kind == 1 && bearer.is_some() {
                    next.push(Item {
                        root: item.root,
                        bud: bud(&chain.words, i + 1, a.context),
                        bearer,
                    });
                }
            }
        }
        wave = next;
    }
    (roots, cost)
}

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The roots of `species` at `age` and `seed`, expanded on the GPU in two
/// orders and batch sizes and held against the CPU's: the roots, their
/// kept axes and phytomers, and the costs of the two runs.
fn hold(gpu: (&Gpu, &Grower), species: &Species, (age, seed): (u32, u64)) -> [usize; 3] {
    let (_, cases) = fine_cases(species, request(age, seed), Plan::default()).expect("grows");
    let tables = ChainTables::new(species, age, &Expected::new(species, age, 0)).expect("tables");
    let buds: Vec<ChainBud> = cases.iter().map(|c| c.0).collect();
    let forward: Vec<usize> = (0..buds.len()).collect();
    let (roots, _) = expand(gpu, (&buds, &forward, 4096), &tables);
    let reverse: Vec<usize> = forward.iter().rev().copied().collect();
    let (again, _) = expand(gpu, (&buds, &reverse, 777), &tables);
    assert!(again == roots, "the same roots in any order and batch");
    let (mut axes, mut phytomers) = (0, 0);
    for (k, ((_, want), got)) in cases.iter().zip(&roots).enumerate() {
        assert_eq!(got.len(), want.len(), "root {k}: kept axes");
        for (j, (g, w)) in got.iter().zip(want).enumerate() {
            assert_eq!(g, w, "root {k} axis {j}");
        }
        axes += want.len();
        phytomers += want.iter().map(|a| a.phytomers.len()).sum::<usize>();
    }
    [buds.len(), axes, phytomers]
}

#[test]
fn every_fine_root_expands_on_the_gpu_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let grower = Grower::new(&gpu);
    let mut oak = telperion_space::oak();
    for state in &mut oak.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    let species = [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
        ("oak, light off", oak),
    ];
    for (name, species) in species {
        for seed in [1, 7] {
            let [roots, axes, phytomers] = hold((&gpu, &grower), &species, (25, seed));
            println!("{name} 25 {seed}: {roots} roots, {axes} kept axes, {phytomers} phytomers");
        }
    }
}

/// The beech and spruce at 40 and 80, seeds 1 and 7, held against the CPU,
/// with the costs of both executors: `cargo test --profile ci -p
/// telperion-render fine_roots_at_40_and_80 -- --ignored --nocapture`.
#[test]
#[ignore = "a measurement over the old trees"]
fn fine_roots_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let grower = Grower::new(&gpu);
    println!("| Tree | Roots | Chains | Waves | Kept axes | Phytomers | Tables ms | GPU grow cold / warm ms | Read ms | Again | µs a root (warm) | CPU executor 1 core / all cores ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    let species: [(&str, Species); 2] = [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
    ];
    for (name, species) in &species {
        for age in [40, 80] {
            for seed in [1, 7] {
                let request = request(age, seed);
                let (_, cases) = fine_cases(species, request, Plan::default()).expect("grows");
                let expected = Expected::new(species, age, 0);
                let cpu = |threads| {
                    let plan = Plan {
                        threads,
                        ..Plan::default()
                    };
                    let fine = grow_fine(species, request, &expected, plan).expect("grows");
                    let micros: f64 = fine.roots.iter().map(|r| r.2.micros).sum();
                    (micros / 1e3, fine)
                };
                let (one, _) = cpu(1);
                let t = Instant::now();
                cpu(0);
                let all = t.elapsed().as_secs_f64() * 1e3;
                let t = Instant::now();
                let tables = ChainTables::new(species, age, &expected).expect("tables");
                let tables_ms = t.elapsed().as_secs_f64() * 1e3;
                let buds: Vec<ChainBud> = cases.iter().map(|c| c.0).collect();
                let forward: Vec<usize> = (0..buds.len()).collect();
                let (roots, cold) = expand((&gpu, &grower), (&buds, &forward, 1 << 16), &tables);
                let (again, warm) = expand((&gpu, &grower), (&buds, &forward, 1 << 16), &tables);
                assert!(again == roots);
                let want: Vec<&Vec<ChainAxis>> = cases.iter().map(|c| &c.1).collect();
                assert!(
                    roots.iter().zip(&want).all(|(g, w)| g == *w),
                    "{name} {age} {seed}"
                );
                let axes: usize = roots.iter().map(Vec::len).sum();
                let phyts: usize = roots.iter().flatten().map(|a| a.phytomers.len()).sum();
                println!(
                    "| {name} {age} {seed} | {} | {} | {} | {axes} | {phyts} | {tables_ms:.0} | {:.0} / {:.0} | {:.0} | {} | {:.2} | {one:.0} / {all:.0} |",
                    buds.len(),
                    warm.chains,
                    warm.waves,
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
