//! The feedback slice on the GPU against the CPU's (fn-211 task 8; Astra's
//! review of CONTRACT-A.md, section 4): leaves deposited by the device in
//! three orders and three chunkings, once into a buffer too small so the
//! executor retries from a zeroed lattice; the lattice read back is the
//! CPU's to the word; the CPU's one sweep of it; the device's samples and
//! D16 decisions are the CPU's to the word.
use super::super::io;
use crate::Gpu;
use telperion_space::fixed::feedback::lattice::Grid;
use telperion_space::fixed::feedback::slice::{self, Leaf, Read};
use telperion_space::Light;

const SLICE: &str = include_str!("feedback_slice.wgsl");

pub(super) fn source() -> String {
    format!("{}\n{SLICE}", super::feedback())
}

fn words(x: i64) -> [u32; 2] {
    [x as u32, (x as u64 >> 32) as u32]
}

fn leaf_words(leaves: &[Leaf]) -> Vec<u32> {
    let row = |l: &Leaf| [l.at[0], l.at[1], l.at[2], l.scale, l.size, l.area];
    leaves.iter().flat_map(row).flat_map(words).collect()
}

fn read_words(reads: &[Read]) -> Vec<u32> {
    let row = |r: &Read| {
        let b = &r.bud;
        let (draw, keep) = (b.draw as i64, b.keep as i64);
        [
            r.at[0],
            r.at[1],
            r.at[2],
            r.presence,
            b.c,
            b.memory,
            b.hazard,
            b.tolerance,
            draw,
            b.window,
            keep,
        ]
    };
    reads.iter().flat_map(row).flat_map(words).collect()
}

struct Device<'a> {
    gpu: &'a Gpu,
    pass: io::Pass,
    leaves: wgpu::Buffer,
    reads: wgpu::Buffer,
    grid: Grid,
}

const STORAGE: wgpu::BufferUsages = wgpu::BufferUsages::STORAGE.union(wgpu::BufferUsages::COPY_SRC);

impl Device<'_> {
    /// A buffer bound where a pass reads nothing, one a binding.
    fn unused(&self) -> wgpu::Buffer {
        self.buffer("slice unused", &[0; 16])
    }

    fn buffer(&self, label: &'static str, bytes: &[u8]) -> wgpu::Buffer {
        io::buffer(self.gpu, label, bytes.len() as u64, STORAGE, bytes).expect("a buffer")
    }

    fn cfg(&self, n: u32, start: u32, capacity: u32) -> wgpu::Buffer {
        let groups = n.div_ceil(64);
        let row = groups.min(
            self.gpu
                .device
                .limits()
                .max_compute_workgroups_per_dimension,
        ) * 64;
        let low = self.grid.low.map(|l| l as i32 as u32);
        let size = self.grid.size.map(|s| s as u32);
        let cfg = [
            n, start, capacity, row, low[0], low[1], low[2], 0, size[0], size[1], size[2], 0,
        ];
        let bytes: &[u8] = bytemuck::cast_slice(&cfg);
        io::buffer(
            self.gpu,
            "slice cfg",
            48,
            wgpu::BufferUsages::UNIFORM,
            bytes,
        )
        .expect("a buffer")
    }

    /// One pass of `entry` over `n` items from `start`.
    fn run(&self, entry: usize, cfg: &wgpu::Buffer, n: u32, bound: [&wgpu::Buffer; 5]) {
        let [order, light, lattice, flag, out] = bound;
        let all = [
            cfg,
            &self.leaves,
            order,
            &self.reads,
            light,
            lattice,
            flag,
            out,
        ];
        let bind = self.pass.bind(self.gpu, &all);
        let mut encoder = self.gpu.device.create_command_encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, entry, n.div_ceil(64));
        self.gpu.queue.submit([encoder.finish()]);
    }

    /// The leaves deposited in `order`, `chunk` a dispatch, into a lattice
    /// of `capacity` nodes; where one overflows, again from a zeroed
    /// lattice of the box's nodes. The lattice, and the attempts taken.
    fn deposit(&self, order: &[u32], chunk: usize, capacity: usize) -> (Vec<u128>, u32) {
        let (light, out) = (self.unused(), self.unused());
        let order_buffer = self.buffer("slice order", bytemuck::cast_slice(order));
        let mut capacity = capacity;
        for attempt in 1.. {
            let lattice = self.buffer("slice lattice", &vec![0u8; capacity * 16]);
            let flag = self.buffer("slice flag", &[0; 4]);
            for (k, c) in order.chunks(chunk).enumerate() {
                let cfg = self.cfg(c.len() as u32, (k * chunk) as u32, capacity as u32);
                self.run(
                    0,
                    &cfg,
                    c.len() as u32,
                    [&order_buffer, &light, &lattice, &flag, &out],
                );
            }
            let flag = io::read(self.gpu, &flag, 4).expect("the flag");
            let flag = u32::from_le_bytes(flag[..4].try_into().expect("a word"));
            assert_eq!(flag & !1, 0, "an area or corner out of range");
            if flag & 1 != 0 {
                capacity = self.grid.nodes();
                continue;
            }
            let bytes = io::read(self.gpu, &lattice, capacity as u64 * 16).expect("the lattice");
            let nodes = bytemuck::cast_slice::<u8, u128>(&bytes)[..self.grid.nodes()].to_vec();
            return (nodes, attempt);
        }
        unreachable!()
    }

    /// Each read's light, memory, threshold, shed, kept and status.
    fn sample(&self, light: &[i64], n: usize) -> Vec<[u64; 6]> {
        let (order, lattice, flag) = (self.unused(), self.unused(), self.unused());
        let light = self.buffer("slice light", bytemuck::cast_slice(light));
        let out = self.buffer("slice out", &vec![0u8; n * 48]);
        let cfg = self.cfg(n as u32, 0, 0);
        self.run(1, &cfg, n as u32, [&order, &light, &lattice, &flag, &out]);
        let bytes = io::read(self.gpu, &out, n as u64 * 48).expect("the reads");
        bytemuck::cast_slice::<u8, u64>(&bytes)
            .as_chunks::<6>()
            .0
            .to_vec()
    }
}

#[test]
fn the_feedback_slice_is_the_cpus_in_any_order_chunking_and_retry() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let (leaves, reads) = slice::crown(20_000, 3_000, 11);
    let n = leaves.len();
    let forward: Vec<usize> = (0..n).collect();
    let cpu = slice::run(&light, &leaves, &reads, &forward, n).expect("the CPU's slice");
    let device = Device {
        gpu: &gpu,
        pass: io::Pass::new(
            &gpu,
            source(),
            &[true, true, true, true, false, false, false],
            &["deposit", "sample"],
        ),
        leaves: io::buffer(
            &gpu,
            "slice leaves",
            n as u64 * 48,
            STORAGE,
            bytemuck::cast_slice(&leaf_words(&leaves)),
        )
        .expect("a buffer"),
        reads: io::buffer(
            &gpu,
            "slice reads",
            reads.len() as u64 * 88,
            STORAGE,
            bytemuck::cast_slice(&read_words(&reads)),
        )
        .expect("a buffer"),
        grid: cpu.grid,
    };
    let orders: [Vec<u32>; 3] = [
        (0..n as u32).collect(),
        (0..n as u32).rev().collect(),
        (0..n).map(|k| ((k * 2_654_435_761) % n) as u32).collect(),
    ];
    let nodes = cpu.grid.nodes();
    let mut retried = false;
    for (o, order) in orders.iter().enumerate() {
        for (chunk, capacity) in [(n, nodes), (4_096, nodes), (777, nodes / 2)] {
            let (lattice, attempts) = device.deposit(order, chunk, capacity);
            assert_eq!(
                attempts,
                if capacity < nodes { 2 } else { 1 },
                "order {o}, chunk {chunk}"
            );
            retried |= attempts == 2;
            assert!(
                lattice == cpu.lattice,
                "order {o}, chunk {chunk}: the lattice differs"
            );
        }
    }
    assert!(retried);
    // The device's lattice through the one CPU sweep, then its reads.
    let got = device.sample(&cpu.light, reads.len());
    let mut wrong = 0;
    for (k, (g, (l, b))) in got.iter().zip(&cpu.reads).enumerate() {
        let want = [
            *l as u64,
            b.memory as u64,
            b.threshold,
            u64::from(b.shed),
            b.kept,
            0,
        ];
        if *g != want {
            if wrong < 3 {
                eprintln!("read {k}: cpu {want:x?} gpu {g:x?}");
            }
            wrong += 1;
        }
    }
    let shed = cpu.reads.iter().filter(|r| r.1.shed).count();
    let shaded = cpu
        .light
        .iter()
        .filter(|&&l| l < telperion_space::fixed::ONE32)
        .count();
    println!(
        "{n} leaves, {} reads, {nodes} nodes ({shaded} shaded), {shed} shed; {wrong} reads differ",
        reads.len()
    );
    assert_eq!(wrong, 0);
}
