//! fn-256 task 8 (COUNCIL.md round 5, step 8): the two races ROOFLINE.md
//! found while reading, each driven through the real kernel's function by
//! a probe entry added to its module.
//! - **The layout's fault tally:** every fault bit any axis of a dispatch
//!   meets reaches the cycle's tally (`fault_out`).
//! - **The units' counted laterals:** a lateral counted (`node_laterals`,
//!   `what` 0) writes nothing, though the emitted round counts its zones
//!   again (`zones_main`).
use super::super::io;
use super::build::Tables;
use super::build_grow::{grow_now, Sizes};
use super::grown_tests::{device, grown};
use super::lay_device::T_FAULT;
use crate::buffer::Owned;
use crate::Gpu;
use telperion_core::presets::Preset;

/// The layout's faults, one bit an axis on the first seven of a
/// workgroup, through `fault_out`.
const FAULTS: &str = "
@compute @workgroup_size(64)
fn faults_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = invocation(id);
    if i >= cfg.n {
        return;
    }
    status = select(0u, 1u << (i % 64u), i % 64u < 7u);
    fault_out(i);
}
";

/// Each zone row's laterals at `cfg.n` nodes counted as the emitted
/// round's `zones_main` counts them, an invocation a node.
const COUNTED: &str = "
@compute @workgroup_size(64)
fn counted_main(@builtin(global_invocation_id) id: vec3<u32>) {
    let k = id.x + id.y * cfg.row;
    let rows = (cfg.rows - table[0]) / ZW;
    if k >= cfg.n * rows {
        return;
    }
    let zone = k % rows;
    emitting = true;
    base_axes = 0u;
    n_axes = 0u;
    let key = vec2<u32>(k * 0x9e3779b9u, k);
    let whorl = words[cfg.whorls + zone];
    counts[k] = node_laterals(table[0] + zone * ZW, key, vec2<u32>(0u, k), cfg.cycle, whorl, 0u);
}
";

fn words(gpu: &Gpu, b: &wgpu::Buffer) -> Vec<u32> {
    bytemuck::cast_slice(&io::read(gpu, b, b.size()).expect("a readback")).to_vec()
}

fn storage(gpu: &Gpu, label: &'static str, init: &[u32]) -> Owned {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    io::buffer(
        gpu,
        label,
        4 * init.len() as u64,
        usage,
        bytemuck::cast_slice(init),
    )
    .expect("a buffer")
}

fn uniform(gpu: &Gpu, words: &[u32]) -> Owned {
    io::buffer(
        gpu,
        "probe cfg",
        4 * words.len() as u64,
        wgpu::BufferUsages::UNIFORM,
        bytemuck::cast_slice(words),
    )
    .expect("a uniform")
}

fn run(gpu: &Gpu, pass: &io::Pass, buffers: &[&wgpu::Buffer], groups: u32) {
    let bind = pass.bind(gpu, buffers);
    let mut e = gpu.device.create_command_encoder(&Default::default());
    pass.run(&mut e, &bind, 0, groups);
    gpu.queue.submit([e.finish()]);
}

/// Seven axes of each workgroup fault at once, each its own bit: the
/// cycle's tally holds every bit, every time.
#[test]
fn every_fault_of_a_dispatch_reaches_the_tally() {
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let src = format!("{}\n{FAULTS}", super::layout_source());
    let mut reads = vec![true; 3];
    reads.extend([false; 5]);
    let pass = io::Pass::new(&gpu, src, &reads, &["faults_main"]);
    let (n, groups) = (64 * 64, 64);
    // `layout.wgsl`'s `Cfg`: the axes' stride is `n`.
    let cfg = uniform(
        &gpu,
        &[n, groups * 64, 1, 0, n, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    );
    let lost: Vec<u32> = (0..16)
        .map(|_| {
            let held = ["form", "units", "grown", "laid"].map(|l| storage(&gpu, l, &[0; 4]));
            let (axes, pencils) = (
                storage(&gpu, "axes", &vec![0; 64 * n as usize]),
                storage(&gpu, "pencils", &[0; 4]),
            );
            let (lists, counts) = (
                storage(&gpu, "lists", &[0; 64]),
                storage(&gpu, "counts", &[0; 4]),
            );
            let [form, units, grown, laid] = &held;
            run(
                &gpu,
                &pass,
                &[
                    &cfg, form, units, grown, laid, &axes, &pencils, &lists, &counts,
                ],
                groups,
            );
            words(&gpu, &lists)[T_FAULT as usize]
        })
        .filter(|&tally| tally != 0x7f)
        .collect();
    assert!(
        lost.is_empty(),
        "the tally lost a fault in {} of 16 dispatches: {lost:x?}",
        lost.len()
    );
}

/// A lateral counted while the round emits writes no axis: every word of
/// the axes stays as it was, and the probe counted laterals.
#[test]
fn a_counted_lateral_writes_no_axis() {
    let Some((gpu, kit)) = device() else {
        return;
    };
    let g = grown(Preset::EuropeanBeech, 8, 42);
    let t = Tables::new(&gpu, &g.tables, &g.chain).expect("tables");
    let (b, ..) = grow_now(&gpu, &kit, &t, &g.tables, (Sizes::least(), 3)).expect("grown to 3");
    let src = format!("{}\n{COUNTED}", super::source());
    let mut reads = vec![true; 3];
    reads.extend([false; 5]);
    let pass = io::Pass::new(&gpu, src, &reads, &["counted_main"]);
    let nodes = 256;
    let groups = (nodes * t.rows).div_ceil(64);
    let mut cfg = b.units_static(&t, (1, 0, 0));
    (cfg[0], cfg[1], cfg[2]) = (nodes, groups.min(super::plan::MAXX) * 64, 2);
    let cfg = uniform(&gpu, &cfg);
    // Every axis word one: a write leaves another.
    let fill = vec![1.0f32.to_bits(); (b.state.axes.size() / 4) as usize];
    let axes = storage(&gpu, "probe axes", &fill);
    let counts = storage(&gpu, "probe counts", &vec![0; (nodes * t.rows) as usize]);
    let s = &b.state;
    run(
        &gpu,
        &pass,
        &[
            &cfg, &t.table, &t.words, &b.inputs, &axes, &s.units, &s.grown, &s.lists, &counts,
        ],
        groups,
    );
    let laterals: u64 = words(&gpu, &counts).iter().map(|&n| u64::from(n)).sum();
    let written = words(&gpu, &axes)
        .iter()
        .zip(&fill)
        .filter(|(a, f)| a != f)
        .count();
    println!("{laterals} laterals counted, {written} axis words written");
    assert!(laterals > 0, "the probe counted no lateral");
    assert_eq!(written, 0, "a counted lateral wrote {written} axis words");
}
