//! The unit kernel against the CPU's grown tree (fn-209 F1, task 2): every
//! growth unit of the beech's and spruce's trees, drawn on the GPU from
//! `telperion_space::Units`' tables, gives the tree's nodes in its order
//! (their keys), and each lateral the tree grew stands on the bud place
//! that drew its PA.
use super::super::io;
use super::units::{pack, NONE};
use crate::Gpu;
use std::time::Instant;
use telperion_space::{grow, Light, Origin, Request, Species, Structure, Units};

const UNIT: &str = include_str!("unit.wgsl");

/// A tree to check: species, age and seed.
type Case = (&'static str, fn() -> Species, u32, u64);

/// The unit kernel's WGSL.
pub(super) fn source() -> String {
    format!("{}\n{UNIT}", super::zones())
}

/// A growth unit of the grown tree: its axis's lineage, PA and cycle, and
/// its phytomers' range on the axis.
struct Grown {
    axis: usize,
    cycle: u32,
    from: usize,
    to: usize,
}

fn grown(tree: &Structure) -> Vec<Grown> {
    let mut out = Vec::new();
    for (axis, a) in tree.axes.iter().enumerate() {
        let mut from = 0;
        while from < a.phytomers.len() {
            let cycle = a.phytomers[from].cycle;
            let to = from
                + a.phytomers[from..]
                    .iter()
                    .take_while(|p| p.cycle == cycle)
                    .count();
            out.push(Grown {
                axis,
                cycle,
                from,
                to,
            });
            from = to;
        }
    }
    out
}

/// A unit's key: its axis's lineage's child by its cycle, or, for a failed
/// unit's relay's first unit (fn-199 R1), that key's child by
/// `lineage::FAILED`: the one its first node's key descends from, through
/// a zone (`lineage::ZONE` + z) and the node's draw index.
fn unit_key(lineage: u64, cycle: u32, node: u64) -> u64 {
    use telperion_space::fixed;
    use telperion_space::lineage_keys::{FAILED, ZONE};
    let unit = fixed::child(lineage, u64::from(cycle));
    let apart = fixed::child(unit, FAILED);
    let holds = |key: u64| {
        (0..4).any(|z| (0..256).any(|j| fixed::child(fixed::child(key, ZONE + z), j) == node))
    };
    if holds(unit) {
        unit
    } else {
        assert!(holds(apart), "a unit keyed neither way");
        apart
    }
}

struct Kernel {
    pass: io::Pass,
}

/// What a run of the kernel returns: each unit's nodes (key, presence,
/// bud places), and its timings in ms.
struct Run {
    nodes: Vec<[u64; 3]>,
    offsets: Vec<u32>,
    upload: f64,
    total: f64,
}

impl Kernel {
    fn new(gpu: &Gpu) -> Self {
        let pass = io::Pass::new(
            gpu,
            source(),
            &[true, true, true, false, false],
            &["count", "emit"],
        );
        Self { pass }
    }

    fn run(&self, gpu: &Gpu, units: &[[u32; 4]], tables: &(Vec<u32>, Vec<u64>)) -> Run {
        let start = Instant::now();
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = units.len() as u32;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg = [n, 0, row, 0];
        let uniform = io::buffer(
            gpu,
            "unit cfg",
            16,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&cfg),
        )
        .unwrap();
        let input = io::buffer(
            gpu,
            "units",
            n as u64 * 16,
            storage,
            bytemuck::cast_slice(units),
        )
        .unwrap();
        let table = io::buffer(
            gpu,
            "unit table",
            tables.0.len() as u64 * 4,
            storage,
            bytemuck::cast_slice(&tables.0),
        )
        .unwrap();
        let words = io::buffer(
            gpu,
            "unit words",
            tables.1.len() as u64 * 8,
            storage,
            bytemuck::cast_slice(&tables.1),
        )
        .unwrap();
        let counts = io::buffer(gpu, "unit counts", n as u64 * 4, storage, &[]).unwrap();
        let upload = start.elapsed().as_secs_f64() * 1e3;
        let empty = io::buffer(gpu, "unit nodes", 16, storage, &[]).unwrap();
        let bind = self
            .pass
            .bind(gpu, &[&uniform, &input, &table, &words, &counts, &empty]);
        let mut encoder = gpu.encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 0, groups);
        gpu.submit([encoder.finish()]);
        let sizes: Vec<u32> =
            bytemuck::cast_slice(&io::read(gpu, &counts, n as u64 * 4).unwrap()).to_vec();
        assert!(
            sizes.iter().all(|&c| c != NONE),
            "a zone past the kernel's bound"
        );
        // The offsets (a scan on the device once the kernel is the
        // executor's).
        let mut offsets = Vec::with_capacity(sizes.len() + 1);
        let mut total = 0u32;
        for &c in &sizes {
            offsets.push(total);
            total += c;
        }
        gpu.queue
            .write_buffer(&counts, 0, bytemuck::cast_slice(&offsets));
        offsets.push(total);
        let bytes = u64::from(total.max(1)) * 24;
        let out = io::buffer(gpu, "unit nodes", bytes, storage, &[]).unwrap();
        let bind = self
            .pass
            .bind(gpu, &[&uniform, &input, &table, &words, &counts, &out]);
        let mut encoder = gpu.encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 1, groups);
        gpu.submit([encoder.finish()]);
        let read = io::read(gpu, &out, bytes).unwrap();
        let nodes =
            bytemuck::cast_slice::<u8, u64>(&read).as_chunks::<3>().0[..total as usize].to_vec();
        Run {
            nodes,
            offsets,
            upload,
            total: start.elapsed().as_secs_f64() * 1e3,
        }
    }
}

/// The tree of `species` at `age` and `seed`, and the kernel's units of it.
fn tree(species: &Species, age: u32, seed: u64) -> (Structure, Vec<Grown>, Vec<[u32; 4]>, f64) {
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    };
    let t = Instant::now();
    let tree = grow(species, request).expect("grows");
    let ms = t.elapsed().as_secs_f64() * 1e3;
    let units = grown(&tree);
    let packed = units
        .iter()
        .map(|u| {
            let a = &tree.axes[u.axis];
            let key = unit_key(a.lineage, u.cycle, a.phytomers[u.from].key());
            [key as u32, (key >> 32) as u32, a.pa as u32, u.cycle]
        })
        .collect();
    (tree, units, packed, ms)
}

/// The kernel's nodes against the tree's: keys in order, and each lateral
/// on the bud place that drew its PA. The phytomers compared.
fn compare(species: &Species, tree: &Structure, units: &[Grown], run: &Run) -> usize {
    let mut place = vec![Vec::new(); tree.axes.len()];
    for (k, u) in units.iter().enumerate() {
        let from = run.offsets[k] as usize;
        let got = &run.nodes[from..run.offsets[k + 1] as usize];
        let want = &tree.axes[u.axis].phytomers[u.from..u.to];
        assert_eq!(
            got.len(),
            want.len(),
            "axis {} cycle {}: node count",
            u.axis,
            u.cycle
        );
        for (g, p) in got.iter().zip(want) {
            assert_eq!(
                g[0],
                p.key(),
                "axis {} cycle {}: node order",
                u.axis,
                u.cycle
            );
        }
        place[u.axis].extend((from..from + got.len()).map(|i| run.nodes[i][2]));
    }
    for a in &tree.axes {
        if let Origin::Lateral {
            parent,
            node,
            slot,
            woken: false,
            ..
        } = a.origin
        {
            let drawn = (place[parent][node] >> (8 * slot)) & 0xff;
            assert_ne!(drawn, 0xff, "a lateral on a bare place");
            let lived = species.lived(drawn as usize).map(|l| l.0);
            assert_eq!(lived, Some(a.pa), "a lateral's PA");
        }
    }
    run.nodes.len()
}

fn check(gpu: &Gpu, kernel: &Kernel, cases: &[Case]) {
    for &(name, species, age, seed) in cases {
        let species = species();
        let tables = pack(&Units::new(&species, age).expect("tables"));
        let (tree, units, packed, grow_ms) = tree(&species, age, seed);
        let cold = kernel.run(gpu, &packed, &tables);
        let warm = kernel.run(gpu, &packed, &tables);
        assert_eq!(cold.nodes, warm.nodes, "{name}: the same nodes again");
        let n = compare(&species, &tree, &units, &warm);
        println!(
            "| {name} {age} {seed} | {} | {n} | {:.2} | {:.1} / {:.1} | {:.1} | {grow_ms:.0} |",
            units.len(),
            warm.upload,
            cold.total,
            warm.total,
            warm.total * 1e6 / n as f64,
        );
    }
}

#[test]
fn every_unit_of_the_young_trees_draws_the_cpu_s_nodes() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let kernel = Kernel::new(&gpu);
    let cases: [Case; 4] = [
        ("beech", telperion_space::beech, 40, 1),
        ("beech", telperion_space::beech, 40, 7),
        ("spruce", telperion_space::spruce, 40, 1),
        ("spruce", telperion_space::spruce, 40, 7),
    ];
    check(&gpu, &kernel, &cases);
}

/// The same at 80, with the timings: `cargo test --profile ci -p
/// telperion-render unit_kernel_at_80 -- --ignored --nocapture`.
#[test]
#[ignore = "a measurement over the old trees"]
fn unit_kernel_at_80() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let t = Instant::now();
    let kernel = Kernel::new(&gpu);
    println!(
        "pipelines built in {:.1} ms",
        t.elapsed().as_secs_f64() * 1e3
    );
    println!("| Tree | Units | Nodes | Upload ms | GPU cold / warm ms | ns a node (warm) | CPU grow ms (all cores) |");
    println!("|---|--:|--:|--:|--:|--:|--:|");
    let cases: [Case; 8] = [
        ("beech", telperion_space::beech, 40, 1),
        ("beech", telperion_space::beech, 40, 7),
        ("beech", telperion_space::beech, 80, 1),
        ("beech", telperion_space::beech, 80, 7),
        ("spruce", telperion_space::spruce, 40, 1),
        ("spruce", telperion_space::spruce, 40, 7),
        ("spruce", telperion_space::spruce, 80, 1),
        ("spruce", telperion_space::spruce, 80, 7),
    ];
    check(&gpu, &kernel, &cases);
}
