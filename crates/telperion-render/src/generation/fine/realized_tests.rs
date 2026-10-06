//! Fine roots under realized mechanics on the GPU executor against the
//! realized CPU executor (fn-209 F1 task 6; REALIZED.md, "Host decision
//! 8", steps 1 to 3): every root's subtree grown whole, identical by
//! lineage, in any order of the roots and any batch size.
use super::laid_tests::kernels;
use super::readback;
use super::realized::{grow_roots, RootBud};
use super::roots::{Cost, Kernels, Tables};
use crate::Gpu;
use telperion_space::{
    realized_cases, ChainAxis, ChainTables, Expected, GeometryTables, Light, Plan, RealizedCase,
    Request, Species,
};

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The oak with its light switched off and no shed pipe retained, as the
/// CPU executor's own tests take it.
fn unlit_oak() -> Species {
    let mut oak = telperion_space::oak();
    for s in &mut oak.states {
        (s.shade_hazard, s.shade_size) = (0.0, 0.0);
        (s.balance_hazard, s.leaf_girth, s.retained) = (0.0, 0.0, 0.0);
    }
    oak
}

/// A root's axes in canonical order: by depth, then identity (its
/// bearer's identity and how it stands there), parents renumbered.
fn canon(axes: &[ChainAxis]) -> Vec<ChainAxis> {
    let mix = |h: u64, v: u64| (h ^ v).wrapping_mul(0x0000_0100_0000_01b3);
    let (mut ids, mut depth) = (vec![0u64; axes.len()], vec![0usize; axes.len()]);
    for (i, a) in axes.iter().enumerate() {
        let (from, d) = a
            .parent
            .map_or((0xcbf2_9ce4_8422_2325, 0), |p| (ids[p], depth[p] + 1));
        let how = match a.kind {
            1 => [1, a.node as u64, u64::from(a.slot), u64::from(a.woken)],
            k => [u64::from(k), a.pa as u64, 0, 0],
        };
        ids[i] = how.iter().fold(from, |h, &v| mix(h, v));
        depth[i] = d;
    }
    let mut order: Vec<usize> = (0..axes.len()).collect();
    order.sort_by_key(|&i| (depth[i], ids[i]));
    let mut at = vec![0; axes.len()];
    for (k, &i) in order.iter().enumerate() {
        at[i] = k;
    }
    order
        .iter()
        .map(|&i| ChainAxis {
            parent: axes[i].parent.map(|p| at[p]),
            ..axes[i].clone()
        })
        .collect()
}

/// A tree's roots, its tables on the device keeping every axis, and the
/// roots as the device takes them.
fn setup<'a>(
    gpu: &Gpu,
    species: &Species,
    (age, seed): (u32, u64),
    chain: &'a ChainTables,
) -> (Vec<RealizedCase>, Tables<'a>, Vec<RootBud>) {
    let cases = realized_cases(species, request(age, seed), Plan::default()).expect("grows");
    let expected = Expected::new(species, age, 0);
    let geo = GeometryTables::new(species, age, &expected).expect("tables");
    let mut tables = Tables::new(gpu, chain, &geo);
    tables.keep = true;
    let roots = cases
        .iter()
        .map(|c| (c.bud, [c.made.0 as f32, c.made.1 as f32]))
        .collect();
    (cases, tables, roots)
}

fn trips(cost: &Cost) {
    assert_eq!(cost.trips, cost.runs + cost.waves, "{cost:?}");
}

/// Step 1: every root's axes as grown, before shedding, by lineage.
fn grown_as_on_the_cpu(gpu: &Gpu, k: &Kernels, name: &str, species: &Species, at: (u32, u64)) {
    let expected = Expected::new(species, at.0, 0);
    let chain = ChainTables::new(species, at.0, &expected).expect("tables");
    let (cases, tables, roots) = setup(gpu, species, at, &chain);
    let forward: Vec<usize> = (0..cases.len()).collect();
    let reverse: Vec<usize> = forward.iter().rev().copied().collect();
    let mut axes = 0;
    for (order, batch) in [(&forward, 4096), (&reverse, 777), (&forward, 64)] {
        let (grown, cost) = grow_roots((gpu, k), (&roots, order, batch), &tables);
        trips(&cost);
        let got = readback::read(gpu, &grown, cases.len());
        for (r, (c, have)) in cases.iter().zip(&got.roots).enumerate() {
            assert_eq!(canon(have), canon(&c.grown), "{name} {at:?} root {r}");
        }
        axes = got.roots.iter().map(Vec::len).sum::<usize>();
    }
    println!("{name} {at:?}: {} roots, {axes} axes grown", cases.len());
}

#[test]
fn every_root_grows_whole_on_the_gpu_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let species = [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
        ("oak, unlit", unlit_oak()),
    ];
    for (name, species) in &species {
        for seed in [1, 7] {
            grown_as_on_the_cpu(&gpu, &k, name, species, (25, seed));
        }
    }
}
