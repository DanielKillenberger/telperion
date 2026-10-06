//! Fine roots expanded on the GPU executor against the CPU reference
//! executor (fn-209 F1, tasks 2 and 4, R1 and R2): every root's kept axes
//! by lineage, identical; then every kept phytomer's laid tip and radius,
//! within fn-125's screen (50 µm most, 10 µm RMS), in any order of the
//! roots and any batch size; and the whole fine stage's cost.
use super::chains::Grower;
use super::laid::{Resolver, AL, PL};
use super::roots::{expand, Cost, Expanded, Tables};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{
    fine_cases, grow_fine, roots_geometry, ChainAxis, ChainBud, ChainTables, Expected, Fine,
    GeometryTables, Light, Plan, Request, RootGeometry, Species,
};

/// fn-125's screen, in metres.
const MOST: f64 = 50e-6;
const RMS: f64 = 10e-6;

fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}

/// The executors' kernels.
struct Kernels {
    grower: Grower,
    resolver: Resolver,
}

/// A tree as the CPU reference executor expands it: the tree, each root's
/// bud and kept axes, and its attachment.
struct Reference {
    fine: Fine,
    cases: Vec<(ChainBud, Vec<ChainAxis>)>,
    attached: Vec<RootGeometry>,
}

fn reference(species: &Species, (age, seed): (u32, u64)) -> Reference {
    let (fine, cases) = fine_cases(species, request(age, seed), Plan::default()).expect("grows");
    let attached = roots_geometry(&fine);
    Reference {
        fine,
        cases,
        attached,
    }
}

/// How far the GPU's laid geometry stands from the CPU's.
#[derive(Debug, Default, Clone, Copy)]
struct Apart {
    phytomers: usize,
    most: f64,
    squares: f64,
    radius: f64,
    radius_most: f64,
    worst: (usize, usize, usize, f64, f64, f64),
    far: (usize, usize, usize, usize, f64, usize, f64),
    statuses: u32,
}

impl Apart {
    fn rms(&self) -> f64 {
        (self.squares / self.phytomers.max(1) as f64).sqrt()
    }
}

/// The GPU's expansion held against the reference: topology identical by
/// lineage, and how far its geometry lies.
fn held(gpu: &Gpu, r: &Reference, got: &Expanded) -> Apart {
    for (k, ((_, want), have)) in r.cases.iter().zip(&got.roots).enumerate() {
        assert_eq!(have.len(), want.len(), "root {k}: kept axes");
        for (j, (g, w)) in have.iter().zip(want).enumerate() {
            assert_eq!(g, w, "root {k} axis {j}");
        }
    }
    let waves: Vec<Vec<f32>> = got.waves.iter().map(|w| w.read(gpu)).collect();
    let mut apart = Apart::default();
    for (buffer, at) in &got.statuses {
        let words: Vec<u32> =
            bytemuck::cast_slice(&super::super::io::read(gpu, buffer, buffer.size()).unwrap())
                .to_vec();
        apart.statuses |= at.iter().fold(0, |s, &a| s | words[a as usize]);
    }
    for (k, laid) in got.laid.iter().enumerate() {
        let origin = r.attached[k].bent.tip;
        let start = r.fine.starts[k];
        for (j, &(w, i)) in laid.iter().enumerate() {
            let wave = &waves[w];
            let phyts = got.waves[w].axes as usize * AL;
            let first = wave[i as usize * AL + 7].to_bits() as usize;
            let axis = &r.fine.structure.axes[start + j];
            for (n, p) in axis.phytomers.iter().enumerate() {
                let g = &wave[phyts + (first + n) * PL..][..PL];
                let want = p.tip - origin;
                let d = [
                    f64::from(g[0]) - want.x,
                    f64::from(g[1]) - want.y,
                    f64::from(g[2]) - want.z,
                ];
                let e = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                assert!(e.is_finite(), "root {k} axis {j} phytomer {n}: {g:?}");
                if e > apart.most {
                    let dist = want.length();
                    apart.far = (k, j, n, w, dist, axis.pa, p.radius);
                }
                apart.most = apart.most.max(e);
                apart.squares += e * e;
                apart.phytomers += 1;
                let dr = (f64::from(g[18]) - p.radius).abs() / p.radius.max(1e-9);
                if dr > apart.radius {
                    apart.worst = (k, j, n, p.radius, f64::from(g[18]), e);
                }
                apart.radius = apart.radius.max(dr);
                apart.radius_most = apart.radius_most.max((f64::from(g[18]) - p.radius).abs());
            }
        }
    }
    apart
}

fn run(
    gpu: &Gpu,
    k: &Kernels,
    r: &Reference,
    tables: &Tables,
    (order, batch): (&[usize], usize),
) -> (Expanded, Cost) {
    let buds: Vec<ChainBud> = r.cases.iter().map(|c| c.0).collect();
    expand(
        (gpu, &k.grower, &k.resolver),
        (&buds, &r.attached, order, batch),
        tables,
    )
}

/// One tree held in two orders and batch sizes: the roots, kept axes and
/// phytomers, and how far apart.
fn hold(gpu: &Gpu, k: &Kernels, species: &Species, at: (u32, u64)) -> (usize, Apart) {
    let r = reference(species, at);
    let expected = Expected::new(species, at.0, 0);
    let chain = ChainTables::new(species, at.0, &expected).expect("tables");
    let geo = GeometryTables::new(species, at.0, &expected).expect("tables");
    let tables = Tables::new(gpu, &chain, &geo);
    let forward: Vec<usize> = (0..r.cases.len()).collect();
    let (one, _) = run(gpu, k, &r, &tables, (&forward, 4096));
    let reverse: Vec<usize> = forward.iter().rev().copied().collect();
    let (two, _) = run(gpu, k, &r, &tables, (&reverse, 777));
    let a = held(gpu, &r, &one);
    let b = held(gpu, &r, &two);
    assert_eq!(a.most, b.most, "the same geometry in any order and batch");
    assert_eq!(
        a.squares, b.squares,
        "the same geometry in any order and batch"
    );
    (r.cases.len(), a)
}

fn kernels(gpu: &Gpu) -> Kernels {
    Kernels {
        grower: Grower::new(gpu),
        resolver: Resolver::new(gpu),
    }
}

fn oak_light_off() -> Species {
    let mut oak = telperion_space::oak();
    for state in &mut oak.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    oak
}

#[test]
fn every_fine_root_expands_and_lays_on_the_gpu_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    let species = [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
        ("oak, light off", oak_light_off()),
    ];
    for (name, species) in species {
        for seed in [1, 7] {
            let (roots, a) = hold(&gpu, &k, &species, (25, seed));
            println!(
                "{name} 25 {seed}: {roots} roots, {} phytomers, tips {:.2e} most {:.2e} RMS, radii {:.2e} rel {:.2e} m, status {}, worst {:?}, far {:?}",
                a.phytomers, a.most, a.rms(), a.radius, a.radius_most, a.statuses, a.worst, a.far
            );
            assert!(a.most <= MOST && a.rms() <= RMS, "{name} 25 {seed}: {a:?}");
        }
    }
}

/// The beech and spruce at 40 and 80, seeds 1 and 7: geometry held
/// against the CPU, and the whole fine stage timed on both executors:
/// `cargo test --profile ci -p telperion-render fine_geometry_at_40_and_80
/// -- --ignored --nocapture`.
#[test]
#[ignore = "a measurement over the old trees"]
fn fine_geometry_at_40_and_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let k = kernels(&gpu);
    println!("| Tree | Roots | Chains | Phytomers | Tips most / RMS µm | Radii most rel | GPU cold ms (grow / read / lay) | GPU warm ms (grow / read / lay) | Worst item: root, phytomers, GPU warm ms | CPU items 1 core ms | CPU expansion all cores ms |");
    println!("|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|");
    for (name, species) in [
        ("beech", telperion_space::beech()),
        ("spruce", telperion_space::spruce()),
    ] {
        for age in [40, 80] {
            for seed in [1, 7] {
                let r = reference(&species, (age, seed));
                let expected = Expected::new(&species, age, 0);
                let cpu = |threads| {
                    let plan = Plan {
                        threads,
                        ..Plan::default()
                    };
                    grow_fine(&species, request(age, seed), &expected, plan).expect("grows")
                };
                let one: f64 = cpu(1).roots.iter().map(|x| x.2.micros).sum::<f64>() / 1e3;
                let all = cpu(0).expand_seconds * 1e3;
                let t = Instant::now();
                let chain = ChainTables::new(&species, age, &expected).expect("tables");
                let geo = GeometryTables::new(&species, age, &expected).expect("tables");
                let tables = Tables::new(&gpu, &chain, &geo);
                let tables_ms = t.elapsed().as_secs_f64() * 1e3;
                let forward: Vec<usize> = (0..r.cases.len()).collect();
                let (cold_got, cold) = run(&gpu, &k, &r, &tables, (&forward, 1 << 16));
                let (warm_got, warm) = run(&gpu, &k, &r, &tables, (&forward, 1 << 16));
                let a = held(&gpu, &r, &warm_got);
                drop(cold_got);
                // The worst item: the root that grew the most kept
                // phytomers, expanded alone.
                let (worst, size) = r
                    .fine
                    .roots
                    .iter()
                    .enumerate()
                    .map(|(i, x)| (i, x.1))
                    .max_by_key(|x| x.1)
                    .expect("a root");
                run(&gpu, &k, &r, &tables, (&[worst], 1 << 16));
                let (_, alone) = run(&gpu, &k, &r, &tables, (&[worst], 1 << 16));
                let ms = |c: &Cost| {
                    format!(
                        "{:.0} ({:.0} / {:.0} / {:.0})",
                        c.grow + c.read + c.lay,
                        c.grow,
                        c.read,
                        c.lay
                    )
                };
                println!(
                    "| {name} {age} {seed} | {} | {} | {} | {:.1} / {:.2} | {:.1e} | {} | {} | {worst}, {size}, {:.1} | {one:.0} | {all:.0} |",
                    r.cases.len(),
                    warm.chains,
                    a.phytomers,
                    a.most * 1e6,
                    a.rms() * 1e6,
                    a.radius,
                    ms(&cold),
                    ms(&warm),
                    alone.grow + alone.read + alone.lay,
                );
                println!(
                    "  tables {tables_ms:.0} ms, status {}, again {}",
                    a.statuses, warm.again
                );
                assert!(
                    a.most <= MOST && a.rms() <= RMS,
                    "{name} {age} {seed}: {a:?}"
                );
            }
        }
    }
}
