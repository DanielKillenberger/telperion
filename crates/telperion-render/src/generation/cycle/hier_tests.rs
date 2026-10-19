//! The device's hierarchy, balance and allotment held to the CPU's
//! cycle: young trees in seconds, and Telperion's and the oak's busy
//! cycles, measured.
use super::super::io;
use super::columns::Cap;
use super::compare::Agreement;
use super::hier_build::{Hierarchy, Kernels, CAP};
use super::hier_columns::{Cpu, Hier};
use super::hier_compare::{blocks, children, compare, words};
use super::hier_device::Sums;
use crate::buffer::Owned;
use crate::Gpu;
use telperion_core::presets::Preset;
use telperion_space::{Capture, Request};

/// `preset` grown to `age` at `seed` through the one cycle and its
/// `at`th cycle's balance and allotment captured; light as the preset
/// sets it.
pub(super) fn captured(preset: Preset, age: u32, seed: u64, at: u32) -> Capture {
    let family = preset.parameters();
    let species = family.space.species.expect("an engine species");
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    telperion_space::grow_capture_hierarchy_at(&species, request, at).expect("a capture")
}

pub(super) fn tree(gpu: &Gpu, hier: &Hier) -> Owned {
    let usage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
    let bytes = hier.tree.len() as u64 * 4;
    io::buffer(
        gpu,
        "hierarchy tree",
        bytes,
        usage,
        bytemuck::cast_slice(&hier.tree),
    )
    .expect("a tree")
}

/// The hierarchy at `cap`, built a cycle's laid axes at a time, or all
/// at once.
fn built(gpu: &Gpu, k: &Kernels, hier: &Hier, tree: &Owned, cap: u32, at_once: bool) -> Hierarchy {
    let mut h = Hierarchy::new(gpu, hier.stride, cap).expect("buffers");
    let steps = if at_once {
        vec![hier.n]
    } else {
        hier.laid.clone()
    };
    for n in steps {
        h.grow(gpu, k, tree, n).expect("a step");
    }
    h
}

/// The balance and allotment over `h`: the sums run and read.
pub(super) fn summed(
    gpu: &Gpu,
    k: &Kernels,
    hier: &Hier,
    h: &Hierarchy,
    tree: &Owned,
) -> (Sums, super::hier_compare::Out) {
    let sums = Sums::new(gpu, k, hier, h, tree).expect("buffers");
    let mut e = gpu.encoder(&Default::default());
    sums.record(k, tree, &mut e);
    gpu.submit([e.finish()]);
    let out = sums.read(gpu, hier).expect("a readback");
    (sums, out)
}

/// What a capture's replay found: the hierarchy's agreement (its
/// children, its blocks' rules, built at once and at every cap), the
/// integers' and the reals'.
struct Replay {
    structure: Agreement,
    ints: Agreement,
    reals: Agreement,
}

fn replay(gpu: &Gpu, k: &Kernels, name: &str, c: &Capture) -> Replay {
    let cap = Cap(c);
    let (hier, cpu) = (Hier::new(&cap), Cpu::new(&cap));
    let tree = tree(gpu, &hier);
    let mut structure = Agreement::default();
    let h = built(gpu, k, &hier, &tree, CAP, false);
    let shape = h.read(gpu).expect("a readback");
    children(&mut structure, &shape, &hier, &cpu);
    let broken = blocks(&h, &shape, &hier.tree, hier.n as usize);
    structure.eq("blocks' rules broken", &broken, &[]);
    let (_, out) = summed(gpu, k, &hier, &h, &tree);
    let (ints, reals) = compare(&hier, &cpu, &out);
    let reference = words(&out);
    let once = built(gpu, k, &hier, &tree, CAP, true);
    let once_shape = once.read(gpu).expect("a readback");
    let n = hier.n as usize;
    structure.eq(
        "children built at once",
        &once_shape.children(n).1,
        &shape.children(n).1,
    );
    structure.eq(
        "blocks' rules at once",
        &blocks(&once, &once_shape, &hier.tree, n),
        &[],
    );
    structure.eq(
        "sums built at once",
        &words(&summed(gpu, k, &hier, &once, &tree).1),
        &reference,
    );
    for cap in [64, 128, 256, 1024, 4096].into_iter().filter(|&c| c != CAP) {
        let other = built(gpu, k, &hier, &tree, cap, false);
        let at = format!("sums at cap {cap}");
        structure.eq(
            &at,
            &words(&summed(gpu, k, &hier, &other, &tree).1),
            &reference,
        );
    }
    let width = if super::hier_build::WIDTH == 64 {
        256
    } else {
        64
    };
    let wide = Kernels::with_width(gpu, width);
    let at = format!("sums at width {width}");
    structure.eq(
        &at,
        &words(&summed(gpu, &wide, &hier, &h, &tree).1),
        &reference,
    );
    let grown = out.live_n;
    println!(
        "{name}: cycle {}, {} axes, {} open, {} living to {grown}; {} blocks on {} levels (at once {} on {}), {} steps",
        hier.cycle,
        hier.n,
        cpu.open.len(),
        hier.live(),
        h.blocks,
        h.levels,
        once.blocks,
        once.levels,
        h.passes.len()
    );
    println!("{}{}{}", structure.report(), ints.report(), reals.report());
    Replay {
        structure,
        ints,
        reals,
    }
}

pub(super) fn device() -> Option<(Gpu, Kernels, f64)> {
    let gpu = super::super::test_gpu()?;
    let started = std::time::Instant::now();
    let k = Kernels::new(&gpu);
    gpu.device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("compiled");
    let compile = started.elapsed().as_secs_f64();
    println!("hierarchy kernels compiled in {compile:.2} s");
    Some((gpu, k, compile))
}

/// The young captures: the oak (shed by its balance, sized by its
/// light), Telperion, the ordinary (continuations), the spruce (woken
/// buds), the beech and the birch.
const YOUNG: [(Preset, &str, u32, u32); 7] = [
    (Preset::OregonWhiteOak, "oregon-white-oak", 16, 12),
    (Preset::OregonWhiteOak, "oregon-white-oak", 30, 30),
    (Preset::Telperion, "telperion", 14, 10),
    (Preset::Ordinary, "ordinary", 16, 12),
    (Preset::NorwaySpruce, "norway-spruce", 24, 16),
    (Preset::EuropeanBeech, "european-beech", 10, 8),
    (Preset::SilverBirch, "silver-birch", 16, 12),
];

fn young(check: &dyn Fn(&Replay) -> bool) -> Vec<String> {
    let _held = super::super::whole_tree();
    let Some((gpu, k, _)) = device() else {
        return Vec::new();
    };
    let mut differ = Vec::new();
    for (preset, name, age, at) in YOUNG {
        let name = format!("{name} {age} at {at}");
        if !check(&replay(&gpu, &k, &name, &captured(preset, age, 42, at))) {
            differ.push(name);
        }
    }
    differ
}

#[test]
fn young_hierarchies_and_their_integers_agree_with_the_cpu() {
    let differ = young(&|r| r.structure.exact() && r.ints.exact());
    assert!(
        differ.is_empty(),
        "the device's hierarchy is not the CPU's: {differ:?}"
    );
}

/// The sums' reals within the ruled tolerance (`ruled.rs`): the driver's
/// division and contraction move incoming, bud, memory and size in their
/// last places (HIERARCHY.md).
#[test]
fn young_sums_agree_with_the_cpu_within_tolerance() {
    let differ = young(&|r| r.structure.exact() && r.ints.exact() && r.reals.exact());
    assert!(
        differ.is_empty(),
        "the device's sums are not the CPU's: {differ:?}"
    );
}
