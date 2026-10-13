//! fn-256's spike, COUNCIL.md early test 3, native only: one busy cycle
//! of the one cycle replayed on the device in throwaway kernels, wired
//! into nothing.
//!
//! - `capture <preset> <age> <seed> <out>`: grows the preset through the
//!   one cycle (quantised deposit) and writes its busy cycle's phases.
//! - `replay <capture> [block cap] [reps]`: the phases on the device, both
//!   hierarchy schedules and both sweeps: agreement with the CPU, then
//!   compile, encoding, per-phase GPU time, the longest dispatch and
//!   scratch.
//! - `chain [reps]`: an empty indirect-dispatch chain of 80 × 8.
//!
//!   cargo run --profile ci -p telperion-render --example gpucycle -- replay raw/oak-80-1.cap
mod bufs;
mod cap;
mod chain;
mod check;
mod cycle;
mod gpu;
mod tree;

use bufs::Kits;
use cycle::{Ctx, Hier, Sweep};
use std::collections::BTreeMap;
use std::time::Instant;
use telperion_core::presets::Preset;

const PRESETS: [(Preset, &str); 8] = [
    (Preset::EuropeanBeech, "european-beech"),
    (Preset::OregonWhiteOak, "oregon-white-oak"),
    (Preset::NorwaySpruce, "norway-spruce"),
    (Preset::DatePalm, "date-palm"),
    (Preset::SilverBirch, "silver-birch"),
    (Preset::Ordinary, "ordinary"),
    (Preset::Laurelin, "laurelin"),
    (Preset::Telperion, "telperion"),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args[0].as_str() {
        "capture" => capture(
            &args[1],
            args[2].parse().expect("an age"),
            args[3].parse().expect("a seed"),
            &args[4],
        ),
        "replay" => replay(
            &args[1],
            args.get(2)
                .map_or(1024, |a| a.parse().expect("a block cap")),
            args.get(3).map_or(5, |a| a.parse().expect("reps")),
        ),
        "chain" => {
            let gpu = gpu::gpu();
            println!("adapter: {}", gpu.name);
            chain::chain(&gpu, args.get(1).map_or(5, |a| a.parse().expect("reps")));
        }
        _ => panic!("capture, replay or chain"),
    }
}

fn capture(name: &str, age: u32, seed: u64, out: &str) {
    let (p, _) = PRESETS
        .iter()
        .find(|p| p.1 == name)
        .expect("a shipped preset");
    let family = p.parameters();
    let species = family.space.species.expect("an engine species");
    let request = telperion_space::Request {
        age,
        seed,
        budget: 20_000_000,
        light: family.space.light,
    };
    let started = Instant::now();
    let c = telperion_space::grow_capture(&species, request).expect("a capture");
    let bytes = c.bytes();
    std::fs::write(out, &bytes).expect("written");
    println!(
        "{name} age {age} seed {seed}: captured in {:.1} s, {} MB",
        started.elapsed().as_secs_f64(),
        bytes.len() >> 20
    );
}

fn shader(body: &str) -> String {
    format!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        include_str!("../../src/generation/cycle/u64.wgsl"),
        include_str!("../../src/generation/cycle/keys.wgsl"),
        include_str!("../../src/generation/cycle/decide.wgsl"),
        include_str!("../../src/generation/cycle/arith.wgsl"),
        include_str!("common.wgsl"),
        body
    )
}

fn replay(path: &str, block_cap: u32, reps: usize) {
    let gpu = gpu::gpu();
    println!("adapter: {}", gpu.name);
    let cap = cap::Cap::read(path);
    let started = Instant::now();
    let (parent, node) = (cap.u32s("parent"), cap.u32s("node"));
    let shape = tree::shape(&parent, &node);
    let shaped = started.elapsed().as_secs_f64();
    let blocks = tree::blocks(&parent, block_cap);
    let blocked = started.elapsed().as_secs_f64() - shaped;
    println!(
        "host preparation: children {:.1} ms, blocks {:.1} ms ({} blocks of at most {block_cap}, largest {}, {} levels, deepest inside one {})",
        1e3 * shaped,
        1e3 * blocked,
        blocks.count,
        blocks.largest,
        blocks.levels,
        blocks.tallest
    );
    let kits_started = Instant::now();
    let kits = Kits {
        sum: gpu.kit(
            &shader(include_str!("sum.wgsl")),
            &["walk", "unpack", "block"],
            15,
        ),
        shed: gpu.kit(
            &shader(include_str!("shed.wgsl")),
            &["local", "walk", "block"],
            18,
        ),
        flat: gpu.kit(
            &shader(include_str!("flat.wgsl")),
            &["own", "lit_of", "group_sums", "size_of"],
            17,
        ),
        split: gpu.kit(
            &shader(include_str!("split.wgsl")),
            &["factor", "walk", "block"],
            15,
        ),
        light: gpu.kit(
            &shader(include_str!("light.wgsl")),
            &[
                "span",
                "deposit",
                "densities",
                "sweep_wg",
                "sweep_layer",
                "combine",
                "read",
            ],
            12,
        ),
        fate: gpu.kit(
            &shader(include_str!("fate.wgsl")),
            &["fate", "scan_blocks", "scan_sums", "scan_add"],
            11,
        ),
    };
    let all = kits_started.elapsed().as_secs_f64();
    println!(
        "compile (module and pipelines, ms): sum {:.1}, shed {:.1}, flat {:.1}, split {:.1}, light {:.1}, fate {:.1}; all {:.1}",
        1e3 * kits.sum.compile,
        1e3 * kits.shed.compile,
        1e3 * kits.flat.compile,
        1e3 * kits.split.compile,
        1e3 * kits.light.compile,
        1e3 * kits.fate.compile,
        1e3 * all
    );
    let up = Instant::now();
    let (bufs, sizes) = bufs::upload(&gpu, &cap, &shape, &blocks);
    let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
    let input_bytes: u64 = bufs.map.values().map(|b| b.size()).sum();
    println!(
        "upload: {:.1} ms for {} MB of buffers; axes {}, edges {}, deepest {} ancestors, groups {}, leaves {}, buds read {}, lattice {} ({}×{} layers of {}), directions {}, extinction {}, apexes {}",
        1e3 * up.elapsed().as_secs_f64(),
        input_bytes >> 20,
        sizes.n,
        sizes.edges,
        shape.max_depth,
        sizes.groups,
        sizes.leaves,
        sizes.tips,
        sizes.nodes,
        sizes.nz,
        sizes.dirs,
        sizes.layer,
        sizes.dirs,
        sizes.extinction,
        sizes.apexes
    );
    let (axes, live) = (cap.u32s("per_cycle_axes"), cap.u32s("per_cycle_live"));
    let last = f64::from(*axes.last().expect("cycles"));
    let held: f64 = axes.iter().map(|&a| f64::from(a)).sum();
    let living: f64 = live.iter().map(|&a| f64::from(a)).sum();
    println!(
        "cycles {}; axes held summed over the cycles {:.2} times the busy cycle's, living apexes {:.2} times",
        axes.len(),
        held / last,
        living / f64::from(*live.last().expect("cycles"))
    );
    let uniform = gpu.uniform(8192);
    let chained = bufs::binds(&gpu, &kits, &bufs, &uniform, true);
    let isolated = bufs::binds(&gpu, &kits, &bufs, &uniform, false);
    let light_wg = bufs::light_with(&gpu, &kits, &bufs, &uniform, "depth_wg");
    let light_layer = bufs::light_with(&gpu, &kits, &bufs, &uniform, "depth_layer");
    let c = Ctx {
        gpu: &gpu,
        kits: &kits,
        bufs: &bufs,
        sizes: &sizes,
        blocks: &blocks,
        light_wg: &light_wg,
        light_layer: &light_layer,
    };
    // Agreement: each phase on the CPU's inputs.
    for (hier, sweep) in [(Hier::A, Sweep::Wg), (Hier::B, Sweep::Layer)] {
        println!("\n== agreement, schedule {hier:?}, sweep {sweep:?} ==");
        let mut r = gpu::Recording::new(&gpu, &uniform);
        cycle::reset(&c, &mut r);
        cycle::light(&c, &mut r, &isolated, sweep, true);
        cycle::hierarchy(&c, &mut r, &isolated, hier, false);
        cycle::fate(&c, &mut r, &isolated);
        r.submit(&gpu, Instant::now());
        check::light(&gpu, &bufs, &cap, &sizes);
        check::hierarchy(&gpu, &bufs, &cap, &sizes, hier);
        check::normalise(&gpu, &bufs, &cap, &sizes);
        check::fate(&gpu, &bufs, &cap, &sizes);
    }
    // Timing: the cycle chained, one submission.
    println!("\n== timing, the cycle chained in one submission (median of {reps}) ==");
    for hier in [Hier::A, Hier::B] {
        for sweep in [Sweep::Wg, Sweep::Layer] {
            timing(&c, &chained, &uniform, (hier, sweep), reps);
        }
    }
    let scratch = |names: &[&str]| names.iter().map(|n| bufs.scratch[n]).sum::<u64>();
    println!(
        "\nscratch (bytes): A {} (integer sums, local decisions, incoming); B {} (blocks) + rest {}; sweep by workgroup {}, by layer {}",
        scratch(&["acc_bal", "liv_acc", "acc_alloc", "loc", "rest"]),
        scratch(&["blist", "up_meta", "up_offs", "up_nodes", "down_meta", "down_offs", "down_nodes"]),
        4 * sizes.edges,
        scratch(&["dlight", "depth_wg"]),
        scratch(&["dlight", "depth_layer"]),
    );
}

fn timing(
    c: &Ctx,
    b: &bufs::Binds,
    uniform: &wgpu::Buffer,
    (hier, sweep): (Hier, Sweep),
    reps: usize,
) {
    let mut runs = Vec::new();
    for rep in 0..=reps {
        let begun = Instant::now();
        let mut r = gpu::Recording::new(c.gpu, uniform);
        cycle::reset(c, &mut r);
        cycle::light(c, &mut r, b, sweep, false);
        cycle::hierarchy(c, &mut r, b, hier, true);
        cycle::fate(c, &mut r, b);
        let dispatches = r.dispatches;
        let (encoding, wall, timed) = r.submit(c.gpu, begun);
        if rep > 0 {
            runs.push((encoding, wall, timed, dispatches));
        }
    }
    runs.sort_by(|a, b| a.1.total_cmp(&b.1));
    let (encoding, wall, timed, dispatches) = &runs[runs.len() / 2];
    let mut phases: BTreeMap<String, (f64, usize)> = BTreeMap::new();
    for (label, s) in &timed.passes {
        let e = phases.entry(label.clone()).or_default();
        e.0 += s;
        e.1 += 1;
    }
    let busy: f64 = timed.passes.iter().map(|p| p.1).sum();
    let (longest, at) =
        timed.passes.iter().fold(
            (0.0, ""),
            |m, p| if p.1 > m.0 { (p.1, p.0.as_str()) } else { m },
        );
    println!(
        "{hier:?} + {sweep:?}: {dispatches} dispatches; encoding {:.2} ms; fenced wall {:.2} ms; GPU first-to-last stamp {:.2} ms, in dispatches {:.2} ms; longest dispatch {:.3} ms ({at})",
        1e3 * encoding,
        1e3 * wall,
        1e3 * timed.span,
        1e3 * busy,
        1e3 * longest
    );
    let line: Vec<String> = phases
        .iter()
        .map(|(l, (s, k))| format!("{l} {:.3} ms/{k}", 1e3 * s))
        .collect();
    println!("  phases: {}", line.join("; "));
}

#[cfg(test)]
mod tests {
    use super::shader;

    /// Each cycle module as the spike composes it parses and validates
    /// (the browser probe, `src/probe/kernels.ts`, joins the same files in
    /// the same order).
    #[test]
    fn the_spikes_modules_validate() {
        let bodies = [
            ("sum", include_str!("sum.wgsl")),
            ("shed", include_str!("shed.wgsl")),
            ("flat", include_str!("flat.wgsl")),
            ("split", include_str!("split.wgsl")),
            ("light", include_str!("light.wgsl")),
            ("fate", include_str!("fate.wgsl")),
        ];
        for (name, body) in bodies {
            let module = naga::front::wgsl::parse_str(&shader(body)).unwrap_or_else(|e| panic!("{name}: {e}"));
            naga::valid::Validator::new(naga::valid::ValidationFlags::all(), naga::valid::Capabilities::all())
                .validate(&module)
                .unwrap_or_else(|e| panic!("{name}: {e:?}"));
        }
    }
}
