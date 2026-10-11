//! The device's hierarchy, balance and allotment held to the CPU's
//! cycle: young trees in seconds, and Telperion's and the oak's busy
//! cycles, measured.
use super::super::io;
use super::columns::Cap;
use super::compare::Agreement;
use super::hier_build::{Hierarchy, Kernels, Plan, zeroed};
use super::hier_columns::{Cpu, Hier};
use super::hier_compare::{blocks, children, compare, words};
use super::hier_device::Sums;
use crate::Gpu;
use crate::buffer::Owned;
use telperion_core::presets::Preset;
use telperion_space::{Capture, Request};

/// `preset` grown to `age` at `seed` through the one cycle and its
/// `at`th cycle's balance and allotment captured; light as the preset
/// sets it.
fn captured(preset: Preset, age: u32, seed: u64, at: u32) -> Capture {
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

fn tree(gpu: &Gpu, hier: &Hier) -> Owned {
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
fn summed(
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
    let h = built(gpu, k, &hier, &tree, 1024, false);
    let shape = h.read(gpu).expect("a readback");
    children(&mut structure, &shape, &hier, &cpu);
    let broken = blocks(&h, &shape, &hier.tree, hier.n as usize);
    structure.eq("blocks' rules broken", &broken, &[]);
    let (_, out) = summed(gpu, k, &hier, &h, &tree);
    let (ints, reals) = compare(&hier, &cpu, &out);
    let reference = words(&out);
    let once = built(gpu, k, &hier, &tree, 1024, true);
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
    for cap in [64, 256, 4096] {
        let other = built(gpu, k, &hier, &tree, cap, false);
        let at = format!("sums at cap {cap}");
        structure.eq(
            &at,
            &words(&summed(gpu, k, &hier, &other, &tree).1),
            &reference,
        );
    }
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
        h.gens.len()
    );
    println!("{}{}{}", structure.report(), ints.report(), reals.report());
    Replay {
        structure,
        ints,
        reals,
    }
}

fn device() -> Option<(Gpu, Kernels, f64)> {
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

#[test]
#[ignore = "red on the driver's division and contraction (hier_cycle.wgsl's `hm` and `hdiv` sites, fine/contract.wgsl) until the host rules (HIERARCHY.md)"]
fn young_sums_agree_with_the_cpu_in_every_word() {
    let differ = young(&|r| r.structure.exact() && r.ints.exact() && r.reals.exact());
    assert!(
        differ.is_empty(),
        "the device's sums are not the CPU's: {differ:?}"
    );
}

/// Copies of `bufs`, to start each timed run from.
fn saved(gpu: &Gpu, bufs: &[&Owned]) -> Vec<Owned> {
    let copies: Vec<Owned> = (bufs.iter())
        .map(|b| zeroed(gpu, "hierarchy saved", b.size() / 4).expect("a copy"))
        .collect();
    let mut e = gpu.encoder(&Default::default());
    for (b, c) in bufs.iter().zip(&copies) {
        e.copy_buffer_to_buffer(b, 0, c, 0, b.size());
    }
    gpu.submit([e.finish()]);
    copies
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// Each entry's median ms, the first to last, and the longest dispatch.
type Times = (Vec<(String, f64)>, f64, (String, f64));

/// Nine runs of the last step and the sums, each pass stamped: each
/// entry's median ms in the order first run, the first to last, and the
/// longest dispatch.
fn timed(gpu: &Gpu, run: &dyn Fn(&mut wgpu::CommandEncoder)) -> Times {
    let mut each: Vec<(String, Vec<f64>)> = Vec::new();
    let (mut whole, mut longest) = (Vec::new(), (String::new(), 0.0));
    for _ in 0..9 {
        assert!(super::super::profile::start(gpu), "timestamps");
        let mut e = gpu.encoder(&Default::default());
        run(&mut e);
        gpu.submit([e.finish()]);
        let stamps = super::super::profile::finish(gpu);
        let mut sums: Vec<(String, f64)> = Vec::new();
        for (entry, begin, end) in &stamps {
            let ms = (end - begin) * 1e-6;
            if ms > longest.1 {
                longest = (entry.clone(), ms);
            }
            match sums.iter_mut().find(|p| &p.0 == entry) {
                Some(p) => p.1 += ms,
                None => sums.push((entry.clone(), ms)),
            }
        }
        whole.push((stamps[stamps.len() - 1].2 - stamps[0].1) * 1e-6);
        for (entry, ms) in sums {
            match each.iter_mut().find(|p| p.0 == entry) {
                Some(p) => p.1.push(ms),
                None => each.push((entry, vec![ms])),
            }
        }
    }
    (
        each.into_iter().map(|(p, v)| (p, median(v))).collect(),
        median(whole),
        longest,
    )
}

/// A busy cycle replayed and held to the CPU's, then its last step and
/// its sums timed.
fn busy(gpu: &Gpu, k: &Kernels, name: &str, c: &Capture) -> bool {
    let cap = Cap(c);
    let (hier, cpu) = (Hier::new(&cap), Cpu::new(&cap));
    let tree = tree(gpu, &hier);
    let mut h = Hierarchy::new(gpu, hier.stride, 1024).expect("buffers");
    let (last, earlier) = hier.laid.split_last().expect("cycles");
    for &n in earlier {
        h.grow(gpu, k, &tree, n).expect("a step");
    }
    let before = (h.parity, h.built, h.blocks, h.levels);
    let held = [&h.shape, &h.counts, &h.lists[h.parity], &h.tally];
    let start = saved(gpu, &held);
    h.grow(gpu, k, &tree, *last).expect("the last step");
    let shape = h.read(gpu).expect("a readback");
    let mut agreement = Agreement::default();
    children(&mut agreement, &shape, &hier, &cpu);
    agreement.eq(
        "blocks' rules broken",
        &blocks(&h, &shape, &hier.tree, hier.n as usize),
        &[],
    );
    let (sums, out) = summed(gpu, k, &hier, &h, &tree);
    let (ints, reals) = compare(&hier, &cpu, &out);
    println!(
        "{name}:\n{}{}{}",
        agreement.report(),
        ints.report(),
        reals.report()
    );
    let after = (h.parity, h.built, h.blocks, h.levels);
    (h.parity, h.built, h.blocks, h.levels) = before;
    let gens = *h.gens.last().expect("a step");
    let plan = Plan::new(gpu, k, &h, &tree, (*last, gens)).expect("a plan");
    (h.parity, h.built, h.blocks, h.levels) = after;
    let targets = [&h.shape, &h.counts, &h.lists[before.0], &h.tally];
    let run = |e: &mut wgpu::CommandEncoder| {
        for (from, to) in start.iter().zip(targets) {
            e.copy_buffer_to_buffer(from, 0, to, 0, from.size());
        }
        plan.head(&h, k, e);
        plan.tail(&h, k, e);
        sums.record(k, &tree, e);
    };
    let (times, whole, longest) = timed(gpu, &run);
    let dispatches = plan.dispatches() + sums.dispatches();
    println!(
        "{name}: {dispatches} dispatches ({} the step's, {gens} generations, {} new axes), {} block levels, {} blocks, {} living apexes; longest {} at {:.3} ms; scratch {:.1} MB",
        plan.dispatches(),
        *last - plan.n_old(),
        h.levels,
        h.blocks,
        hier.live(),
        longest.0,
        longest.1,
        (h.scratch() + sums.scratch()) as f64 / 1e6
    );
    for (entry, ms) in times {
        println!("{name}: {entry:>18} {ms:8.3} ms");
    }
    println!("{name}: {:>18} {whole:8.3} ms", "first to last");
    agreement.exact() && ints.exact() && reals.exact()
}

#[test]
#[ignore = "a measurement; red on the driver's division and contraction until the host rules (HIERARCHY.md)"]
fn busy_hierarchies_agree_and_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, k, compile)) = device() else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, seed) in [
        (Preset::Telperion, "telperion", 82, 42),
        (Preset::OregonWhiteOak, "oregon-white-oak", 80, 1),
    ] {
        let started = std::time::Instant::now();
        let capture = captured(preset, age, seed, age);
        println!(
            "{name} {age}: captured in {:.1} s",
            started.elapsed().as_secs_f64()
        );
        if !busy(&gpu, &k, &format!("{name} {age}"), &capture) {
            differ.push(name);
        }
    }
    println!("compile: {compile:.2} s");
    assert!(
        differ.is_empty(),
        "the device's sums are not the CPU's: {differ:?}"
    );
}
