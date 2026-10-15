//! fn-257's growth profile (`#[ignore = "a measurement"]`): where a
//! tree's device build (`tree::drawn`) spends its wall time. For each
//! case, warm (the second build): `tree::drawn` timed whole; the same
//! steps timed one by one, with each cycle's host recording, census wait
//! and readying, and each stage's submissions, dispatches, host waits and
//! buffers made (`trace::COUNTED`); then a third build with every pass
//! stamped, each stage's GPU busy time against its GPU span, and the
//! growth's stamps and censuses in `the_roofline_is_measured`'s files for
//! `roofline.py`'s byte estimates. The drawn records' used bytes beside
//! their buffers' sizes. Files go to `PROFILE_OUT`; `PROFILE_CASES` names
//! `id:age:seed,...`. `cargo test --profile ci -p telperion-render
//! growth_profile -- --ignored --nocapture`.
use super::super::resident::fold::{CA_WORDS, CV};
use super::super::Clock;
use super::build::Tables;
use super::build_grow::grow_hinted;
use super::drawn::{dress, trunk, Rows};
use super::settle_run::settle;
use super::tree::{self, rows, Kernels};
use super::{build_arena, chains};
use crate::trace::COUNTED;
use std::fmt::Write as _;
use std::path::PathBuf;
use telperion_core::pipeline::executor::{self, Establishing, Growth};
use telperion_space::{ChainTables, SettleTo};

const CASES: &str = "norway-spruce:40:1,norway-spruce:40:1234,silver-birch:40:1,silver-birch:40:1234,oregon-white-oak:40:1,oregon-white-oak:40:1234";
const COUNTS: [&str; 10] = [
    "submits",
    "dispatches",
    "readback waits",
    "stage waits",
    "census waits",
    "buffers made",
    "bytes made",
    "writes",
    "bytes written",
    "own passes",
];

#[test]
#[ignore = "a measurement"]
fn growth_profile() {
    let _held = super::super::whole_tree();
    let out = PathBuf::from(std::env::var("PROFILE_OUT").expect("PROFILE_OUT names a directory"));
    std::fs::create_dir_all(&out).expect("the output directory");
    let Some(gpu) = super::super::test_gpu() else {
        return;
    };
    let k = Kernels::new(&gpu);
    gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("compiled");
    let floor = floor(&gpu);
    std::fs::write(out.join("floor.md"), &floor).expect("written");
    println!("{floor}");
    let cases = std::env::var("PROFILE_CASES").unwrap_or_else(|_| CASES.into());
    for case in cases.split(',') {
        let mut part = case.split(':');
        let id = part.next().expect("an id");
        let age: f64 = part.next().expect("an age").parse().expect("an age");
        let seed: u32 = part.next().expect("a seed").parse().expect("a seed");
        let mut family = telperion_core::presets::by_identity(id).expect("a preset");
        (family.age, family.skeleton.seed) = (age, seed);
        let growth = executor::growth(&family).expect("a growth");
        let mut report = format!("# {id} age {age} seed {seed}\n\n");
        // Whole: warm, the second of two.
        let mut whole = 0.0;
        for _ in 0..2 {
            gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("idle");
            let t = Clock::now();
            let drawn = pollster::block_on(tree::drawn(&gpu, &k, &growth)).expect("drawn");
            whole = t.elapsed_ms();
            drop(drawn);
        }
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("idle");
        writeln!(report, "whole `tree::drawn`, warm: {whole:.1} ms\n").unwrap();
        // Staged, unstamped.
        COUNTED.take();
        gpu.ledger.take_peak_census();
        let staged = pollster::block_on(stages(&gpu, &k, &growth, false));
        report.push_str(&staged.table());
        report.push_str(&staged.cycles());
        // Stamped.
        gpu.device.poll(wgpu::PollType::wait_indefinitely()).expect("idle");
        let stamped = pollster::block_on(stages(&gpu, &k, &growth, true));
        report.push_str(&stamped.gpu(&out, id, seed));
        report.push_str(&staged.records);
        std::fs::write(out.join(format!("{id}-{seed}.md")), &report).expect("written");
        println!("{report}");
    }
}

/// One build's stages: each one's name, host milliseconds, counts and
/// peak bytes by label; with stamps on, each one's stamps.
#[derive(Default)]
struct Staged {
    stages: Vec<(&'static str, f64, [u64; 10], Vec<(&'static str, u64)>)>,
    stamps: Vec<(&'static str, Vec<(String, f64, f64)>)>,
    grew: Option<super::build_grow::Grew>,
    records: String,
}

async fn stages(gpu: &crate::Gpu, k: &Kernels, growth: &Growth, stamping: bool) -> Staged {
    let mut st = Staged::default();
    let mut clock = Clock::now();
    let mark = |st: &mut Staged, name: &'static str, clock: &mut Clock| {
        let ms = clock.elapsed_ms();
        let stamps = super::super::profile::finish(gpu);
        if stamping {
            st.stamps.push((name, stamps));
            assert!(super::super::profile::start(gpu), "timestamps");
        }
        st.stages.push((name, ms, COUNTED.take(), gpu.ledger.take_peak_census().1));
        *clock = Clock::now();
    };
    if stamping {
        assert!(super::super::profile::start(gpu), "timestamps");
    }
    let (species, request) = (&growth.species, growth.request);
    let tables = telperion_space::grow_capture_tables(species, request).expect("tables");
    let chain = ChainTables::cycle(species, request.age).expect("chain tables");
    let t = Tables::new(gpu, &tables, &chain).expect("uploaded");
    mark(&mut st, "tables", &mut clock);
    let hint = build_arena::hint(species, request.age).expect("a hint");
    let (b, _, grew) = grow_hinted(gpu, &k.build, &t, &tables, (hint, request.age))
        .await
        .expect("grown");
    mark(&mut st, "grow", &mut clock);
    let b = b.settling(gpu, (&t, &tables)).await.expect("settling");
    let s = settle(gpu, (&k.build, &k.settle), (&b, &t), (&t.settle_pa, SettleTo::Laid))
        .await
        .expect("settled");
    mark(&mut st, "settle", &mut clock);
    drop(b);
    let planned = chains::plan(gpu, (&k.chains, &k.build.hier.scan), s.view())
        .await
        .expect("planned");
    mark(&mut st, "plan", &mut clock);
    let rows = rows(&growth.rows);
    let at = Rows {
        rows: &rows,
        age: request.age,
        scale: growth.scale as f32,
        fraction: 1.0,
        leaf: None,
        establishing: Establishing::words(growth.establishing),
    };
    let dressed = dress(gpu, &k.drawn, (s.view(), &planned), &at).expect("dressed");
    mark(&mut st, "dress", &mut clock);
    let _trunk = trunk(gpu, &k.drawn, &dressed).await.expect("trunk");
    mark(&mut st, "trunk", &mut clock);
    if stamping {
        super::super::profile::finish(gpu);
    }
    // The drawn records: the words they use beside their buffers' sizes.
    let (mut used, mut size, mut axes, mut phyts) = (0u64, 0u64, 0u64, 0u64);
    for w in &dressed.waves {
        used += 4 * (u64::from(w.axes) * CA_WORDS as u64 + u64::from(w.phyts) * CV as u64);
        size += w.buffer.size();
        (axes, phyts) = (axes + u64::from(w.axes), phyts + u64::from(w.phyts));
    }
    st.records = format!(
        "\n## The drawn records\n\n{} waves; {axes} axis records of {} B and {phyts} phytomer records of {} B: {:.1} MB used in buffers of {:.1} MB ({:.0}% used).\n",
        dressed.waves.len(),
        CA_WORDS * 4,
        CV * 4,
        used as f64 / 1e6,
        size as f64 / 1e6,
        100.0 * used as f64 / size as f64
    );
    st.grew = Some(grew);
    st
}

impl Staged {
    fn table(&self) -> String {
        let mut s = String::from("## Stages, host wall (unstamped build)\n\n| Stage | ms |");
        COUNTS.iter().for_each(|c| write!(s, " {c} |").unwrap());
        s.push_str(" peak MB | largest at peak |\n|---|--:|");
        COUNTS.iter().for_each(|_| s.push_str("--:|"));
        s.push_str("--:|---|\n");
        let mut total = 0.0;
        for (name, ms, counts, peak) in &self.stages {
            total += ms;
            write!(s, "| {name} | {ms:.1} |").unwrap();
            for (i, c) in counts.iter().enumerate() {
                if i == 6 || i == 8 {
                    write!(s, " {:.1} MB |", *c as f64 / 1e6).unwrap();
                } else {
                    write!(s, " {c} |").unwrap();
                }
            }
            let mut p = peak.clone();
            p.sort_by_key(|x| std::cmp::Reverse(x.1));
            let sum: u64 = p.iter().map(|x| x.1).sum();
            let top: Vec<String> = p.iter().take(4).map(|(l, b)| format!("{l} {:.1}", *b as f64 / 1e6)).collect();
            writeln!(s, " {:.1} | {} |", sum as f64 / 1e6, top.join(", ")).unwrap();
        }
        writeln!(s, "| total | {total:.1} |").unwrap();
        s
    }

    /// Each cycle's host time: recording, waiting for the census before
    /// it, readying the arena; and the build's totals.
    fn cycles(&self) -> String {
        let g = self.grew.as_ref().expect("grown");
        let mut s = format!(
            "\n## Growth, host (unstamped build)\n\nmoves {}, compactions {}; recording {:.1} ms, waiting {:.1} ms, moving {:.1} ms; recorded dispatches {}; workgroups given {} for {} needed.\n\n| Cycle | recording ms | census wait ms | readying ms |\n|--:|--:|--:|--:|\n",
            g.moves,
            g.compactions,
            g.recording * 1e3,
            g.waiting * 1e3,
            g.moving * 1e3,
            g.stamped.len(),
            g.over.given,
            g.over.needed
        );
        for (c, h) in g.host.iter().enumerate() {
            writeln!(s, "| {} | {:.2} | {:.2} | {:.2} |", c + 1, h[0], h[1], h[2]).unwrap();
        }
        s
    }

    /// Each stage's stamped GPU time against its GPU span; the growth's
    /// per-cycle GPU busy time and span, and its files for `roofline.py`.
    fn gpu(&self, out: &std::path::Path, id: &str, seed: u32) -> String {
        let mut s = String::from("\n## GPU, stamped build\n\n| Stage | host ms (stamped) | passes | GPU busy ms | GPU span ms | idle in span ms | passes under 20 µs | their ms |\n|---|--:|--:|--:|--:|--:|--:|--:|\n");
        for ((name, stamps), (_, ms, ..)) in self.stamps.iter().zip(&self.stages) {
            let busy: f64 = stamps.iter().map(|(_, a, z)| z - a).sum::<f64>() / 1e6;
            let span = match (stamps.first(), stamps.iter().map(|x| x.2).reduce(f64::max)) {
                (Some(f), Some(l)) => (l - f.1) / 1e6,
                _ => 0.0,
            };
            let small: Vec<f64> = stamps.iter().map(|(_, a, z)| z - a).filter(|d| *d < 20e3).collect();
            writeln!(
                s,
                "| {name} | {ms:.1} | {} | {busy:.1} | {span:.1} | {:.1} | {} | {:.1} |",
                stamps.len(),
                span - busy,
                small.len(),
                small.iter().sum::<f64>() / 1e6
            )
            .unwrap();
        }
        let g = self.grew.as_ref().expect("grown");
        let grow = &self.stamps.iter().find(|x| x.0 == "grow").expect("grow stamped").1;
        if grow.len() != g.stamped.len() {
            writeln!(s, "\ngrowth stamps {} against {} recorded phases: no per-cycle split.", grow.len(), g.stamped.len()).unwrap();
            return s;
        }
        let cycle_of = |k: usize| g.ends.iter().position(|&e| k < e).map_or(g.ends.len(), |c| c + 1);
        let mut by = vec![(0.0f64, f64::MAX, 0.0f64, 0usize); g.ends.len() + 1];
        let mut csv = String::from("cycle,phase,kernel,ns\n");
        for (i, ((kernel, a, z), phase)) in grow.iter().zip(&g.stamped).enumerate() {
            let c = cycle_of(i);
            let c = if *phase == super::build_grow::COMPACTION { c - 1 } else { c };
            let e = &mut by[c.min(g.ends.len())];
            (e.0, e.1, e.2, e.3) = (e.0 + z - a, e.1.min(*a), e.2.max(*z), e.3 + 1);
            writeln!(csv, "{c},{phase},{kernel},{:.0}", z - a).unwrap();
        }
        std::fs::write(out.join(format!("{id}-{seed}-stamps.csv")), csv).expect("written");
        let mut census = String::from("cycle");
        (0..256).for_each(|w| write!(census, ",w{w}").unwrap());
        for (c, read) in g.censuses.iter().enumerate() {
            write!(census, "\n{}", c + 1).unwrap();
            read[..256].iter().for_each(|w| write!(census, ",{w}").unwrap());
        }
        std::fs::write(out.join(format!("{id}-{seed}-census.csv")), census).expect("written");
        s.push_str("\n| Cycle | passes | GPU busy ms | GPU span ms | gap before ms |\n|--:|--:|--:|--:|--:|\n");
        let mut last_end = None;
        for (c, (busy, first, end, n)) in by.iter().enumerate() {
            if *n == 0 {
                continue;
            }
            let gap = last_end.map_or(0.0, |l: f64| (first - l) / 1e6);
            writeln!(s, "| {c} | {n} | {:.2} | {:.2} | {gap:.2} |", busy / 1e6, (end - first) / 1e6).unwrap();
            last_end = Some(*end);
        }
        s
    }
}

/// The card's floor for a chain of small dependent dispatches, as the
/// cycle records them: 10,000 dispatches of one kernel that bumps one
/// word, each in a compute pass of its own or all in one pass, of 1 or
/// 1,024 workgroups (all but one exiting at once), submitted once; warm,
/// the best of five, waited for whole.
fn floor(gpu: &crate::Gpu) -> String {
    const N: u32 = 10_000;
    let d = &gpu.device;
    let module = d.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("floor"),
        source: wgpu::ShaderSource::Wgsl(
            "@group(0) @binding(0) var<storage, read_write> b: array<u32>;
@compute @workgroup_size(64) fn main(@builtin(global_invocation_id) g: vec3<u32>) { if (g.x == 0u) { b[0] = b[0] + 1u; } }"
                .into(),
        ),
    });
    let pipeline = d.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("floor"),
        layout: None,
        module: &module,
        entry_point: Some("main"),
        compilation_options: Default::default(),
        cache: None,
    });
    let buffer = d.create_buffer(&wgpu::BufferDescriptor {
        label: Some("floor"),
        size: 256,
        usage: wgpu::BufferUsages::STORAGE,
        mapped_at_creation: false,
    });
    let bind = d.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("floor"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry { binding: 0, resource: buffer.as_entire_binding() }],
    });
    let run = |own: bool, groups: u32, submits: u32| {
        let (mut best, mut recorded) = (f64::MAX, f64::MAX);
        for _ in 0..5 {
            d.poll(wgpu::PollType::wait_indefinitely()).expect("idle");
            let t = Clock::now();
            for _ in 0..submits {
                let mut e = d.create_command_encoder(&Default::default());
                let per = N / submits;
                if own {
                    for _ in 0..per {
                        let mut p = e.begin_compute_pass(&Default::default());
                        p.set_pipeline(&pipeline);
                        p.set_bind_group(0, &bind, &[]);
                        p.dispatch_workgroups(groups, 1, 1);
                    }
                } else {
                    let mut p = e.begin_compute_pass(&Default::default());
                    p.set_pipeline(&pipeline);
                    p.set_bind_group(0, &bind, &[]);
                    for _ in 0..per {
                        p.dispatch_workgroups(groups, 1, 1);
                    }
                }
                gpu.queue.submit([e.finish()]);
            }
            let r = t.elapsed_ms();
            d.poll(wgpu::PollType::wait_indefinitely()).expect("done");
            let all = t.elapsed_ms();
            if all < best {
                (best, recorded) = (all, r);
            }
        }
        (best, recorded)
    };
    let mut s = String::from("## The dispatch floor (10,000 dependent dispatches, warm, best of 5)\n\n| Passes | Workgroups | Submits | ms | of which recording and submitting ms | µs a dispatch |\n|---|--:|--:|--:|--:|--:|\n");
    for (own, groups, submits) in [(true, 1, 1), (false, 1, 1), (true, 1024, 1), (false, 1024, 1), (true, 1, 40), (false, 1, 40)] {
        let (ms, rec) = run(own, groups, submits);
        writeln!(
            s,
            "| {} | {groups} | {submits} | {ms:.1} | {rec:.1} | {:.2} |",
            if own { "one a dispatch" } else { "one for all" },
            ms * 1e3 / f64::from(N)
        )
        .unwrap();
    }
    s
}
