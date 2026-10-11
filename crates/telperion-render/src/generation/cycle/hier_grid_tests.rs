//! The busy cycles timed: Telperion's and the oak's last build step and
//! their sums, held to the CPU's at the default partition, and the grid
//! of block caps and sweep widths the default is chosen from.
use super::columns::Cap;
use super::compare::Agreement;
use super::hier_build::{CAP, Hierarchy, Kernels};
use super::hier_columns::{Cpu, Hier};
use super::hier_compare::{Out, blocks, children, compare, words};
use super::hier_device::Sums;
use super::hier_plan::Plan;
use super::hier_scan::zeroed;
use super::hier_tests::{captured, device, summed, tree};
use crate::Gpu;
use crate::buffer::Owned;
use telperion_core::presets::Preset;
use telperion_space::Capture;

/// The busy captures: Telperion 82, seed 42, and the oak 80, seed 1.
const BUSY: [(Preset, &str, u32, u64); 2] = [
    (Preset::Telperion, "telperion", 82, 42),
    (Preset::OregonWhiteOak, "oregon-white-oak", 80, 1),
];

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

/// Each entry's median ms and its longest dispatch's, in the order first
/// run; the first to last; and the longest dispatch of any run.
type Times = (Vec<(String, f64, f64)>, f64, (String, f64));

/// Nine runs, each pass stamped.
fn timed(gpu: &Gpu, run: &dyn Fn(&mut wgpu::CommandEncoder)) -> Times {
    let mut each: Vec<(String, Vec<f64>, Vec<f64>)> = Vec::new();
    let (mut whole, mut longest) = (Vec::new(), (String::new(), 0.0));
    for _ in 0..9 {
        assert!(super::super::profile::start(gpu), "timestamps");
        let mut e = gpu.encoder(&Default::default());
        run(&mut e);
        gpu.submit([e.finish()]);
        let stamps = super::super::profile::finish(gpu);
        let mut sums: Vec<(String, f64, f64)> = Vec::new();
        for (entry, begin, end) in &stamps {
            let ms = (end - begin) * 1e-6;
            if ms > longest.1 {
                longest = (entry.clone(), ms);
            }
            match sums.iter_mut().find(|p| &p.0 == entry) {
                Some(p) => (p.1, p.2) = (p.1 + ms, p.2.max(ms)),
                None => sums.push((entry.clone(), ms, ms)),
            }
        }
        whole.push((stamps[stamps.len() - 1].2 - stamps[0].1) * 1e-6);
        for (entry, ms, most) in sums {
            match each.iter_mut().find(|p| p.0 == entry) {
                Some(p) => {
                    p.1.push(ms);
                    p.2.push(most);
                }
                None => each.push((entry, vec![ms], vec![most])),
            }
        }
    }
    let entries = each.into_iter().map(|(p, v, m)| (p, median(v), median(m)));
    (entries.collect(), median(whole), longest)
}

/// A busy cycle at a partition: the hierarchy after its last step, the
/// sums read, and the last step and the sums timed together.
struct Cycle {
    h: Hierarchy,
    out: Out,
    times: Times,
    dispatches: usize,
    step: usize,
    gens: usize,
    scratch: u64,
}

/// The hierarchy at `cap` built to the step before the last, kept; the
/// last step and the sums (`k`'s widths) run, read, then timed from the
/// kept state.
fn cycle(gpu: &Gpu, k: &Kernels, hier: &Hier, tree: &Owned, cap: u32) -> Cycle {
    let mut h = Hierarchy::new(gpu, hier.stride, cap).expect("buffers");
    let (last, earlier) = hier.laid.split_last().expect("cycles");
    for &n in earlier {
        h.grow(gpu, k, tree, n).expect("a step");
    }
    let before = (h.parity, h.built, h.blocks, h.levels);
    let start = saved(gpu, &[&h.shape, &h.counts, &h.lists[h.parity], &h.tally]);
    h.grow(gpu, k, tree, *last).expect("the last step");
    let (sums, out): (Sums, Out) = summed(gpu, k, hier, &h, tree);
    let after = (h.parity, h.built, h.blocks, h.levels);
    (h.parity, h.built, h.blocks, h.levels) = before;
    let passes = h.passes.last().expect("a step").clone();
    let plan = Plan::new(gpu, k, &h, tree, (*last, passes.len() as u32)).expect("a plan");
    (h.parity, h.built, h.blocks, h.levels) = after;
    let targets = [&h.shape, &h.counts, &h.lists[before.0], &h.tally];
    let run = |e: &mut wgpu::CommandEncoder| {
        for (from, to) in start.iter().zip(targets) {
            e.copy_buffer_to_buffer(from, 0, to, 0, from.size());
        }
        plan.record(&h, k, e, &passes);
        sums.record(k, tree, e);
    };
    let times = timed(gpu, &run);
    let step = plan.dispatches(&passes);
    let scratch = h.scratch() + sums.scratch();
    let dispatches = step + sums.dispatches();
    Cycle {
        h,
        out,
        times,
        dispatches,
        step,
        gens: passes.len(),
        scratch,
    }
}

/// The busy cycle at the default partition held to the CPU's, its
/// dispatches and times printed.
fn busy(gpu: &Gpu, k: &Kernels, name: &str, c: &Capture) -> bool {
    let cap = Cap(c);
    let (hier, cpu) = (Hier::new(&cap), Cpu::new(&cap));
    let tree = tree(gpu, &hier);
    let cy = cycle(gpu, k, &hier, &tree, CAP);
    let shape = cy.h.read(gpu).expect("a readback");
    let mut agreement = Agreement::default();
    children(&mut agreement, &shape, &hier, &cpu);
    let broken = blocks(&cy.h, &shape, &hier.tree, hier.n as usize);
    agreement.eq("blocks' rules broken", &broken, &[]);
    let (ints, reals) = compare(&hier, &cpu, &cy.out);
    println!(
        "{name}:\n{}{}{}",
        agreement.report(),
        ints.report(),
        reals.report()
    );
    let (times, whole, longest) = &cy.times;
    println!(
        "{name}: {} dispatches ({} the step's, {} generations), {} block levels, {} blocks, {} living apexes; longest {} at {:.3} ms; scratch {:.1} MB",
        cy.dispatches,
        cy.step,
        cy.gens,
        cy.h.levels,
        cy.h.blocks,
        hier.live(),
        longest.0,
        longest.1,
        cy.scratch as f64 / 1e6
    );
    for (entry, ms, most) in times {
        println!("{name}: {entry:>18} {ms:8.3} ms, longest {most:6.3} ms");
    }
    println!("{name}: {:>18} {whole:8.3} ms", "first to last");
    agreement.exact() && ints.exact() && reals.exact()
}

#[test]
#[ignore = "a measurement; red on the driver's division and contraction until the host rules (HIERARCHY.md)"]
fn busy_hierarchies_agree_and_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, k, compile)) = device(super::Arith::Plain) else {
        return;
    };
    let mut differ = Vec::new();
    for (preset, name, age, seed) in BUSY {
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

/// The sweeps' time: totals, walk, sub and split.
fn sweeps(times: &Times) -> f64 {
    let sweep = ["totals_main", "walk_main", "sub_main", "split_main"];
    (times.0.iter())
        .filter(|t| sweep.contains(&t.0.as_str()))
        .map(|t| t.1)
        .sum()
}

/// Caps 64 to 1,024 crossed with sweep widths 64 and 256, each a busy
/// cycle timed; every partition's words held to the first's.
#[test]
#[ignore = "a measurement"]
fn busy_partitions_are_timed() {
    let _held = super::super::whole_tree();
    let Some((gpu, _, _)) = device(super::Arith::Plain) else {
        return;
    };
    let widths = [64, 256].map(|w| (w, Kernels::with_width(&gpu, w, super::Arith::Plain)));
    let mut differ = Vec::new();
    for (preset, name, age, seed) in BUSY {
        let capture = captured(preset, age, seed, age);
        let hier = Hier::new(&Cap(&capture));
        drop(capture);
        let tree = tree(&gpu, &hier);
        let mut reference: Option<Vec<u32>> = None;
        for cap in [64, 128, 256, 1024] {
            for (width, k) in &widths {
                let cy = cycle(&gpu, k, &hier, &tree, cap);
                let all = words(&cy.out);
                if *reference.get_or_insert_with(|| all.clone()) != all {
                    differ.push(format!("{name} cap {cap} width {width}"));
                }
                let (_, whole, longest) = &cy.times;
                println!(
                    "grid {name} {age}: cap {cap:>5} width {width:>3}: {:>6} blocks {:>3} levels {:>4} dispatches; sweeps {:7.3} ms; first to last {whole:7.3} ms; longest {} {:.3} ms",
                    cy.h.blocks,
                    cy.h.levels,
                    cy.dispatches,
                    sweeps(&cy.times),
                    longest.0,
                    longest.1
                );
            }
        }
    }
    assert!(differ.is_empty(), "a partition changed a word: {differ:?}");
}
