//! fn-257 task 4's probe (tests only): with `DISPATCH_PROBE` set, every
//! dispatch of the hierarchy's balance and allotment is followed, in its
//! compute pass, by an empty one of the same workgroups bound to a scratch
//! word of its own, so the added time is what that many dispatches cost
//! to launch and order: the most folding them could save. The empty
//! kernel writes nothing, so the tree is unchanged. With `PROBE_FOLD` set
//! the pair is instead the hierarchy's passes apart and folded
//! (`plan_hier.rs`), the same tree both ways.
use super::super::io;
use crate::buffer::Owned;
use crate::Gpu;

const NOOP: &str = "
struct C { never: u32, a: u32, b: u32, c: u32 }
@group(0) @binding(0) var<uniform> c: C;
@group(0) @binding(1) var<storage, read_write> scratch: array<u32>;
@compute @workgroup_size(64)
fn noop(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x == c.never {
        scratch[0] = id.y;
    }
}
";

pub(in crate::generation) struct Noop {
    pub pass: io::Pass,
    pub bind: wgpu::BindGroup,
    _held: [Owned; 2],
}

impl Noop {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let pass = io::Pass::new(gpu, NOOP.into(), &[false], &["noop"]);
        let words = [u32::MAX, 0, 0, 0];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "probe cfg", 16, usage, bytemuck::cast_slice(&words)).unwrap();
        let usage = wgpu::BufferUsages::STORAGE;
        let scratch = io::buffer(gpu, "probe scratch", 16, usage, &[]).unwrap();
        let bind = pass.bind(gpu, &[&cfg, &scratch]);
        Self {
            pass,
            bind,
            _held: [cfg, scratch],
        }
    }

    /// Whether `id`'s dispatch in `phase` is probed: the balance and
    /// allotment's, or with `PROBE_STEP` set the hierarchy's step alone
    /// (its build kernels and the scans between them).
    pub(super) fn probes(id: super::plan::PassId, phase: &str) -> bool {
        use super::plan::PassId::{HierBuild, HierScan};
        match std::env::var("PROBE_STEP").is_ok() {
            true => matches!(id, HierBuild | HierScan) && phase == "balance and allotment",
            false => phase == "balance and allotment",
        }
    }

    /// Whether the probe is on for this build (`ON`, set by the probe).
    pub(super) fn on() -> bool {
        ON.with(|on| on.get())
    }
}

thread_local! {
    static ON: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    static APART: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Whether this build's passes all run apart, none folded.
pub(super) fn apart() -> bool {
    APART.with(|a| a.get())
}

/// The probe's toggle: the empty dispatches, or the passes apart.
fn toggle(on: bool) {
    match std::env::var("PROBE_FOLD").is_ok() {
        true => APART.with(|a| a.set(on)),
        false => ON.with(|o| o.set(on)),
    }
}

/// Ten alternated pairs of the oak's warm build at 40 (seed 1), the base
/// and the probe, in one process on a quiet card: each pair's
/// milliseconds, the means and spreads, and the tree's records equal in
/// both. `PROBE_PRESET`, `PROBE_PAIRS`.
#[test]
#[ignore = "a measurement"]
fn the_balance_dispatches_cost_this_much() {
    use super::tree::{self, Kernels};
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    let id = std::env::var("PROBE_PRESET").unwrap_or_else(|_| "oregon-white-oak".into());
    let pairs: usize = std::env::var("PROBE_PAIRS").ok().and_then(|p| p.parse().ok()).unwrap_or(10);
    let family = telperion_core::presets::by_identity(&id).expect("a preset");
    let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
    let build = |on: bool| {
        toggle(on);
        let t = super::super::Clock::now();
        let (dressed, _) = pollster::block_on(tree::drawn(&gpu, &k, &growth)).expect("drawn");
        let ms = t.elapsed_ms();
        let nodes = super::super::resident::agree_tests::drawn_nodes(&gpu, &dressed);
        toggle(false);
        (ms, nodes)
    };
    // Warm, and the records the same with and without the probe.
    let (_, base) = build(false);
    let (_, probed) = build(true);
    assert!(format!("{base:?}") == format!("{probed:?}"), "the probe changed the tree");
    let (mut off, mut on) = (Vec::new(), Vec::new());
    for p in 0..pairs {
        // Alternate which runs first.
        let order = if p % 2 == 0 { [false, true] } else { [true, false] };
        for o in order {
            let (ms, _) = build(o);
            if o { on.push(ms) } else { off.push(ms) }
        }
        println!("pair {p}: base {:.1} ms, probe {:.1} ms", off[p], on[p]);
    }
    let stat = |v: &[f64]| {
        let mean = v.iter().sum::<f64>() / v.len() as f64;
        let spread = (v.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / v.len() as f64).sqrt();
        (mean, spread)
    };
    let ((a, sa), (b, sb)) = (stat(&off), stat(&on));
    let diffs: Vec<f64> = on.iter().zip(&off).map(|(n, f)| n - f).collect();
    let (d, sd) = stat(&diffs);
    println!("{id}: base {a:.1} ms (sd {sa:.1}), probe {b:.1} ms (sd {sb:.1}); paired difference {d:.1} ms (sd {sd:.1})");
}

/// Every shipped preset's tree the same with its passes folded and apart.
#[test]
#[ignore = "a measurement"]
fn the_fold_leaves_every_preset_s_tree_the_same() {
    use super::tree::{self, Kernels};
    let _held = super::super::whole_tree();
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    for id in telperion_core::presets::CATALOGUE.iter().map(|c| c.1) {
        let family = telperion_core::presets::by_identity(id).expect("a preset");
        let growth = telperion_core::pipeline::executor::growth(&family).expect("a growth");
        let build = |apart: bool| {
            APART.with(|a| a.set(apart));
            let (dressed, _) = pollster::block_on(tree::drawn(&gpu, &k, &growth)).expect("drawn");
            APART.with(|a| a.set(false));
            format!("{:?}", super::super::resident::agree_tests::drawn_nodes(&gpu, &dressed))
        };
        let same = build(false) == build(true);
        println!("{id}: {}", if same { "same" } else { "DIFFERENT" });
        assert!(same, "{id} changed with its passes folded");
    }
}
