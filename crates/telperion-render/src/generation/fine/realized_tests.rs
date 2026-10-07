//! Fine roots under realized mechanics on the GPU executor against the
//! realized CPU executor (fn-209 F1 task 6; REALIZED.md, "Host decision
//! 8", steps 1 to 4): every root's subtree grown whole and shed,
//! identical by lineage, then settled with the structure above the cut,
//! within fn-125's screen, in any order of the roots and any batch size.
use super::device::Device;
use super::laid::{AL, PL};
use super::laid_tests::kernels;
use super::readback;
use super::realized::{grow_roots, RootBud};
use super::roots::{Cost, Kernels, Tables};
use super::shedding;
use crate::Gpu;
use telperion_space::{
    grow_fine, landed, realized_cases, root_axes, settle, ChainAxis, ChainTables, Expected,
    GeometryTables, Light, Plan, Presences, RealizedCase, Request, RootAxes, Species,
};

pub(super) fn request(age: u32, seed: u64) -> Request {
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

/// A root's axes in canonical order, by depth and then identity (its
/// bearer's identity and how it stands there), parents renumbered; and
/// the order, by the axes' places.
fn canon(axes: &[ChainAxis]) -> (Vec<ChainAxis>, Vec<usize>) {
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
    let canonical = order
        .iter()
        .map(|&i| ChainAxis {
            parent: axes[i].parent.map(|p| at[p]),
            ..axes[i].clone()
        })
        .collect();
    (canonical, order)
}

/// `of` in `order`, each axis's units summed, as the GPU's are read.
fn summed(of: &[Presences], order: &[usize]) -> Vec<Presences> {
    order
        .iter()
        .map(|&i| Presences {
            units: vec![of[i].units.iter().sum()],
            ..of[i].clone()
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
        .map(|c| (c.bud, [c.made.0 as f32, c.made.1 as f32], c.birth))
        .collect();
    (cases, tables, roots)
}

fn trips(cost: &Cost) {
    assert_eq!(cost.trips, cost.runs + cost.waves, "{cost:?}");
}

/// Steps 1 to 3: every root's axes as grown, before shedding, by lineage,
/// and their presences, Q32, to the bit; each root's living as reported
/// up; and its axes kept by its own subtree's shedding.
fn grown_as_on_the_cpu(gpu: &Gpu, k: &Kernels, name: &str, species: &Species, at: (u32, u64)) {
    let expected = Expected::new(species, at.0, 0);
    let chain = ChainTables::new(species, at.0, &expected).expect("tables");
    let (cases, tables, roots) = setup(gpu, species, at, &chain);
    let forward: Vec<usize> = (0..cases.len()).collect();
    let reverse: Vec<usize> = forward.iter().rev().copied().collect();
    let mut axes = 0;
    let fade_table = shedding::table(gpu, &chain);
    let all = vec![1u32; cases.len()];
    let usage = wgpu::BufferUsages::STORAGE;
    let all = crate::generation::io::buffer(
        gpu,
        "roots kept",
        (all.len().max(4) * 4) as u64,
        usage,
        bytemuck::cast_slice(&all),
    )
    .unwrap();
    for (order, batch) in [(&forward, 4096), (&reverse, 777), (&forward, 64)] {
        let (waves, mut cost) = grow_roots((gpu, k), (&roots, order, batch), &tables);
        trips(&cost);
        let got = readback::read(gpu, &waves.grown, cases.len(), true);
        for (r, (c, have)) in cases.iter().zip(&got.roots).enumerate() {
            let ((gpu, by), (cpu, want)) = (canon(have), canon(&c.grown));
            assert_eq!(gpu, cpu, "{name} {at:?} root {r}");
            let (gpu, cpu) = (summed(&got.presences[r], &by), summed(&c.presences, &want));
            for (j, (g, c)) in gpu.iter().zip(&cpu).enumerate() {
                assert_eq!(g, c, "{name} {at:?} root {r} axis {j}: presences");
            }
        }
        let living = k
            .fades
            .living(gpu, (&waves.grown, &waves.chains), (&fade_table, at.0));
        let report = shedding::report(gpu, &living[0], cases.len(), &mut cost);
        for (wi, &r) in order.iter().enumerate() {
            assert_eq!(
                report[wi], cases[r].living,
                "{name} {at:?} root {r}: living"
            );
        }
        k.fades.shed(gpu, (&waves.grown, &waves.own), &all);
        let shed = readback::read(gpu, &waves.grown, cases.len(), true);
        for (r, (c, have)) in cases.iter().zip(&shed.roots).enumerate() {
            assert_eq!(
                canon(have).0,
                canon(&c.kept).0,
                "{name} {at:?} root {r}: kept"
            );
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
    let t = std::time::Instant::now();
    let k = kernels(&gpu);
    println!("pipelines {:.0} ms", t.elapsed().as_secs_f64() * 1e3);
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

/// fn-125's screen, in metres.
const MOST: f64 = 50e-6;
const RMS: f64 = 10e-6;

/// How far the device's roots lie from the CPU executor's, the chains'
/// statuses, what the device's settle cost and its wall milliseconds.
#[derive(Debug, Default, Clone, Copy)]
pub(super) struct Apart {
    pub phytomers: usize,
    pub most: f64,
    pub squares: f64,
    pub radius: f64,
    pub digest: u64,
    pub statuses: u32,
    pub cost: Cost,
    pub wall: f64,
    /// The farthest phytomer: its root, wave, place along its axis,
    /// distance from its root's origin and radius.
    pub worst: (usize, usize, usize, f64, f64),
    /// The roots laid on the CPU, and the farthest of their tips from the
    /// CPU executor's.
    pub grounded: usize,
    pub landed_most: f64,
}

impl Apart {
    pub(super) fn rms(&self) -> f64 {
        (self.squares / self.phytomers.max(1) as f64).sqrt()
    }
}

/// Step 4: the tree settled with its roots on the device, in `order` and
/// batches of `batch`: each root's kept axes by lineage as the CPU
/// executor's (`grow_fine`), and how far its laid tips and radii lie.
pub(super) fn settled_against(
    gpu: &Gpu,
    k: &Kernels,
    (species, at, plan): (&Species, (u32, u64), Plan),
    want: &[Option<RootAxes>],
    (order, batch): (&[usize], usize),
) -> Apart {
    let expected = Expected::new(species, at.0, 0);
    let chain = ChainTables::new(species, at.0, &expected).expect("tables");
    let geo = GeometryTables::new(species, at.0, &expected).expect("tables");
    let t = std::time::Instant::now();
    let mut tables = Tables::new(gpu, &chain, &geo);
    tables.keep = true;
    let mut dev = Device::new(gpu, k, &tables, batch).ordered(order.to_vec());
    let (settled, _) =
        settle(species, request(at.0, at.1), &expected, plan, &mut dev).expect("settles");
    let wall = t.elapsed().as_secs_f64() * 1e3;
    // A root that can reach the ground is the CPU's (host decision 3).
    let landed = landed(&settled);
    let c = dev.cost;
    let reads = 2 + usize::from(tables.sags);
    assert_eq!(c.trips, c.runs + c.waves + reads, "{c:?}");
    let (waves, laid) = dev.waves();
    let got = readback::read(gpu, &waves.grown, want.len(), true);
    let floats: Vec<Vec<f32>> = laid.iter().map(|w| w.read(gpu)).collect();
    let mut apart = Apart {
        cost: c,
        wall,
        grounded: landed.len(),
        ..Apart::default()
    };
    for (r, axes) in &landed {
        let laid = cpu_apart(axes, want[*r].as_ref().expect("a live root"), *r);
        apart.landed_most = apart.landed_most.max(laid);
    }
    for (r, w) in want.iter().enumerate() {
        let Some((axes, origin)) = w else {
            assert!(got.roots[r].is_empty(), "root {r} was shed");
            continue;
        };
        if landed.iter().any(|(k, _)| *k == r) {
            continue;
        }
        apart.statuses |= got.root_statuses[r];
        let cpu: Vec<ChainAxis> = axes.iter().map(|a| a.0.clone()).collect();
        let ((gpu_axes, by), (cpu_axes, want_by)) = (canon(&got.roots[r]), canon(&cpu));
        assert_eq!(gpu_axes, cpu_axes, "root {r}: kept axes");
        for (&g, &c) in by.iter().zip(&want_by) {
            let (w, i) = got.laid[r][g];
            let wave = &floats[w];
            let phyts = laid[w].axes as usize * AL;
            let first = wave[i as usize * AL + 7].to_bits() as usize;
            for (n, &(tip, radius)) in axes[c].1.iter().enumerate() {
                let p = &wave[phyts + (first + n) * PL..][..PL];
                for v in p {
                    apart.digest =
                        (apart.digest ^ u64::from(v.to_bits())).wrapping_mul(0x100_0000_01b3);
                }
                let want = tip - *origin;
                let d = [
                    p[0] as f64 - want.x,
                    p[1] as f64 - want.y,
                    p[2] as f64 - want.z,
                ];
                let e = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                assert!(e.is_finite(), "root {r}: {p:?}");
                if e > apart.most {
                    apart.worst = (r, w, n, want.length(), radius);
                }
                if std::env::var("FINE_ROOT").ok() == Some(r.to_string()) {
                    let a = &axes[c].0;
                    eprintln!("axis {c} kind {} pa {} birth {} n {n} cycle {} dist {:.4} err {:.3e} cpu {:.7} {:.7} {:.7} gpu {:.7} {:.7} {:.7}", a.kind, a.pa, a.birth, a.phytomers[n].0, want.length(), e, want.x, want.y, want.z, p[0], p[1], p[2]);
                }
                apart.most = apart.most.max(e);
                apart.squares += e * e;
                apart.phytomers += 1;
                let dr = (f64::from(p[18]) - radius).abs() / radius.max(1e-9);
                apart.radius = apart.radius.max(dr);
            }
        }
    }
    apart
}

/// How far a root laid on the CPU (`have`) lies from the CPU executor's
/// (`want`), by lineage: its farthest tip from its origin.
fn cpu_apart((have, from): &RootAxes, want: &RootAxes, r: usize) -> f64 {
    let (have_axes, want_axes): (Vec<ChainAxis>, Vec<ChainAxis>) = (
        have.iter().map(|a| a.0.clone()).collect(),
        want.0.iter().map(|a| a.0.clone()).collect(),
    );
    let ((h, by), (w, want_by)) = (canon(&have_axes), canon(&want_axes));
    assert_eq!(h, w, "root {r} laid on the CPU: kept axes");
    let mut most = 0.0f64;
    for (&g, &c) in by.iter().zip(&want_by) {
        for (p, q) in have[g].1.iter().zip(&want.0[c].1) {
            most = most.max(((p.0 - *from) - (q.0 - want.1)).length());
        }
    }
    most
}

/// One tree settled in three orders and batch sizes, each against the
/// CPU executor, the same geometry in each.
fn settles_as_on_the_cpu(
    gpu: &Gpu,
    k: &Kernels,
    name: &str,
    (species, at, cut): (&Species, (u32, u64), f64),
) -> Apart {
    let expected = Expected::new(species, at.0, 0);
    let plan = Plan {
        cut,
        ..Plan::default()
    };
    let cpu = grow_fine(species, request(at.0, at.1), &expected, plan).expect("grows");
    let want = root_axes(&cpu);
    let forward: Vec<usize> = (0..want.len()).collect();
    let reverse: Vec<usize> = forward.iter().rev().copied().collect();
    let a = settled_against(gpu, k, (species, at, plan), &want, (&forward, 4096));
    assert_eq!(a.statuses, 0, "{name} {at:?}: a chain's status");
    for (order, batch) in [(&reverse, 777), (&forward, 64)] {
        let b = settled_against(gpu, k, (species, at, plan), &want, (order, batch));
        assert_eq!(b.statuses, 0, "{name} {at:?}: a chain's status");
        assert_eq!(
            a.digest, b.digest,
            "{name} {at:?}: the same geometry in any order and batch"
        );
    }
    a
}

#[test]
fn every_root_settles_on_the_gpu_as_on_the_cpu() {
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
        for (seed, cut) in [(1, 256.0), (1, 1024.0), (7, 1024.0), (7, 4096.0)] {
            let a = settles_as_on_the_cpu(&gpu, &k, name, (species, (25, seed), cut));
            let rms = a.rms();
            println!(
                "{name} 25 {seed} at {cut}: {} phytomers, tips {:.2e} most {rms:.2e} RMS, radii {:.2e} rel; {} roots on the CPU, their tips {:.2e}",
                a.phytomers, a.most, a.radius, a.grounded, a.landed_most
            );
            assert!(a.landed_most <= MOST, "{name} 25 {seed}: roots on the CPU");
            assert!(a.most <= MOST && rms <= RMS, "{name} 25 {seed}: {a:?}");
        }
    }
}
