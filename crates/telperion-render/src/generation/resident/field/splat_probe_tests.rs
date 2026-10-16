//! fn-257 task 12, R8's probe (PROBE-R8.md): the field splatted to a grid
//! from what the drawn stage hands its consumers, the drawn nodes and the
//! station segments, measured against the CPU reference's per-cube query
//! at the same cells and timed against today's `grow_field` and 36³
//! `query_field`. Measurement only; nothing here ships.
//! `SPLAT_PRESETS`, `SPLAT_SEEDS`, `SPLAT_SIDES` and `SPLAT_PAIRS` narrow
//! it. `cargo test --profile ci -p telperion-render splat_ -- --ignored
//! --nocapture --test-threads 1`.
use super::super::super::cycle::tree;
use super::super::super::{io, profile, Clock, Generator};
use super::super::build::{placeable, readied, Readied, Timed};
use super::super::field_tests::{compare, within};
use super::super::field_time_tests::cells as box_cells;
use super::super::stations::station_rows;
use super::{read_words, storage, unbound, Answer, Cfg, Rec, BUILD, ENTRIES, FIELD};
use super::{CARRIER, CLEAR, INHERIT, LEAFLETS, LEAVES, MARKS, OPENS, ORDER, OWN, WIDEST, WOOD};
use crate::{Gpu, Result};
use telperion_core::foliage::Element;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::pipeline::{self, Request};
use telperion_core::presets::Family;

const SPLAT: &str = include_str!("splat_probe.wgsl");
const SPLATS: [&str; 7] = [
    "splat_clear",
    "splat_wood",
    "splat_leaves",
    "splat_leaflets",
    "splat_best",
    "splat_limb",
    "splat_finish",
];
const S_CLEAR: usize = ENTRIES.len();
const S_WOOD: usize = S_CLEAR + 1;
const S_LEAVES: usize = S_CLEAR + 2;
const S_LEAFLETS: usize = S_CLEAR + 3;
const S_BEST: usize = S_CLEAR + 4;
const S_LIMB: usize = S_CLEAR + 5;
const S_FINISH: usize = S_CLEAR + 6;
/// A node's work words (`field_build.wgsl`'s `WORDS`).
const WORDS: u64 = 7;
/// The table's slots a cell.
const SLOTS: u64 = 8;

/// The grid (spec, "The grid"): `n` cells a side of `cell` metres from
/// `origin`, cell `i + n * (j + n * k)` centred at `origin + (i + 0.5,
/// j + 0.5, k + 0.5) * cell`.
#[derive(Debug, Clone, Copy)]
struct Grid {
    n: u32,
    cell: f64,
    origin: [f64; 3],
}

impl Grid {
    /// `voxelize.ts`'s `grid` over a box (least, then most corner).
    fn over(b: [f64; 6], n: u32) -> Self {
        let cell = (b[3] - b[0]).max(b[4] - b[1]).max(b[5] - b[2]) / f64::from(n - 2);
        let side = f64::from(n) * cell;
        Self {
            n,
            cell,
            origin: [
                (b[0] + b[3]) / 2.0 - side / 2.0,
                b[1] - cell,
                (b[2] + b[5]) / 2.0 - side / 2.0,
            ],
        }
    }

    fn count(&self) -> u32 {
        self.n * self.n * self.n
    }

    fn centres(&self) -> Vec<Vec3> {
        let n = self.n;
        let at = |i: u32, o: f64| o + (f64::from(i) + 0.5) * self.cell;
        let mut out = Vec::with_capacity(self.count() as usize);
        for k in 0..n {
            for j in 0..n {
                for i in 0..n {
                    let [x, y, z] = self.origin;
                    out.push(Vec3::new(at(i, x), at(j, y), at(k, z)));
                }
            }
        }
        out
    }
}

/// A grid named outright (the CPU reference's), or `n` a side over the
/// device's own box.
#[derive(Clone, Copy)]
enum Ask {
    Fixed(Grid),
    Over(u32),
}

/// One grid's answers, and what its table held.
struct Splatted {
    grid: Grid,
    answers: Vec<Answer>,
    pairs: u32,
    lost: u32,
    /// The splat's own device bytes: the grid's words, the table and the
    /// answers.
    bytes: u64,
}

/// A splat's grids, the device's box and systems, and each stage's
/// milliseconds where they were waited on.
struct Run {
    grids: Vec<Splatted>,
    bounds: [f32; 6],
    systems: u32,
    nodes: u32,
    segments: u32,
    stages: Vec<(&'static str, f64)>,
    /// The derivations' work and names' bytes, held while the grids are
    /// splatted.
    work_bytes: u64,
}

/// The probe's source: the field build's, then the splat's (the shader
/// tests validate it with the rest).
pub(super) fn source() -> String {
    format!("{FIELD}\n{BUILD}\n{SPLAT}")
}

/// The probe's kernels: the field build's, then the splat's.
fn pass(gpu: &Gpu) -> io::Pass {
    let mut entries: Vec<&str> = ENTRIES.to_vec();
    entries.extend(SPLATS);
    io::Pass::new(
        gpu,
        source(),
        &[true, true, false, false, false, false, false],
        &entries,
    )
}

/// `make.rs`'s rows: the nodes and segments, a leaf's reach and draws, a
/// frond leaflet's box.
fn cfg(x: &Expansion, (nodes, segs): (u32, u32)) -> Cfg {
    let [seat, rachis, blade] = x.leaf_reach();
    let canopy = x.leaves().canopy;
    let (leaf_centre, leaf_half) = leaf_box(x.element(), canopy.size);
    Cfg {
        nodes,
        root: 0,
        most: canopy.clump_system_order,
        segs,
        reach: [seat, rachis, blade, canopy.rachis_length].map(|v| v as f32),
        bound: [
            x.bearing_bounds()[0] as f32,
            canopy.size_variation as f32,
            canopy.scatter.to_radians() as f32,
            f32::from_bits(x.leaves().seed),
        ],
        leaf_centre,
        leaf_half,
        ..Cfg::default()
    }
}

fn leaf_box(element: &Element, size: f64) -> ([f32; 4], [f32; 4]) {
    let p = &element.positions;
    let first = p.first().copied().unwrap_or(Vec3::ZERO);
    let (lo, hi) = p.iter().fold((first, first), |(lo, hi), v| {
        (
            Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z)),
            Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z)),
        )
    });
    let (c, h) = ((lo + hi) * 0.5, (hi - lo) * 0.5);
    (
        [c.x, c.y, c.z, size].map(|v| v as f32),
        [h.x, h.y, h.z, 0.0].map(|v| v as f32),
    )
}

fn unord(w: u32) -> f32 {
    f32::from_bits(if w & 0x8000_0000 != 0 {
        w & 0x7fff_ffff
    } else {
        !w
    })
}

/// Waits on the device and adds the time since `from` as `name` where
/// stages are timed.
async fn mark(
    gpu: &Gpu,
    staged: bool,
    t: &mut Vec<(&'static str, f64)>,
    name: &'static str,
    from: &mut Clock,
) -> Result<()> {
    if staged {
        io::complete(gpu).await?;
        t.push((name, from.elapsed_ms()));
        *from = Clock::now();
    }
    Ok(())
}

/// `family` grown and drawn as `grow_field` grows it (`make.rs`'s
/// `grow`, to the stations), its limb systems and box derived on the
/// device, and each of `asks` splatted from the drawn nodes and station
/// segments. With `staged`, each stage is waited on and timed; with
/// `profiled`, the stages after the stations are stamped apart.
async fn splat(
    gen: &Generator,
    family: &Family,
    asks: &[Ask],
    staged: bool,
    probe: &io::Pass,
) -> Result<Run> {
    let gpu = &gen.gpu;
    let kit = gen.kit();
    let _capped = gpu.ledger.capped(gen.device_budget());
    let growth = executor::growth(family)?;
    placeable(family)?;
    let mut stages = Vec::new();
    let mut clock = Clock::now();
    let drawn = tree::drawn(gpu, &kit.kernels, &growth).await?;
    mark(gpu, staged, &mut stages, "drawn", &mut clock).await?;
    let mut timed = Timed::default();
    let Readied { x, swept, ids, .. } = readied(gpu, &kit.tools, family, drawn, &mut timed).await?;
    if staged {
        stages.push(("hang", timed.bases));
        stages.push(("folds", timed.folds));
        stages.push(("sweep", timed.sweep));
        clock = Clock::now();
    }
    let rows = station_rows(&x, x.section()?.segments, None, growth.year(1.0));
    let device = kit.tools.stationer.stations(gpu, &swept, ids, rows).await?;
    mark(gpu, staged, &mut stages, "stations", &mut clock).await?;
    if profile::on() {
        report_profile("drawn to stations", profile::finish(gpu));
        profile::start(gpu);
    }
    let fronds = rows.rosette[0][0] > 0.0;
    let mut cfg = cfg(&x, (ids, device.segments));
    let mut r = Rec {
        gpu,
        held: Vec::new(),
    };
    // The limb systems and their dense names (`make.rs`'s `primitives`).
    let mut lows = vec![0u32; (WORDS * u64::from(ids)) as usize];
    lows.extend([u32::MAX; 3]);
    lows.extend([0u32; 3]);
    let work = storage(gpu, "splat work", lows.len() as u64, &lows)?;
    let dense = storage(gpu, "splat names", u64::from(ids) + 1, &[])?;
    let total = storage(gpu, "splat systems", 4, &[])?;
    let none = unbound(gpu)?;
    let derive = [
        &*swept.nodes,
        &*device.buffer,
        &*work,
        &*none[0],
        &*none[1],
        &*none[2],
        &*dense,
    ];
    let mut e = gpu.encoder(&Default::default());
    for entry in [CLEAR, WIDEST, CARRIER, OPENS] {
        r.run(&mut e, (probe, entry, ids, &cfg), &derive)?;
    }
    let jumps = 32 - ids.max(1).leading_zeros() + 1;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(&mut e, (probe, ORDER, ids, &cfg), &derive)?;
    }
    cfg.flip = jumps % 2;
    r.run(&mut e, (probe, OWN, ids, &cfg), &derive)?;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(&mut e, (probe, INHERIT, ids, &cfg), &derive)?;
    }
    cfg.flip = jumps % 2;
    r.run(&mut e, (probe, MARKS, ids, &cfg), &derive)?;
    kit.tools
        .stationer
        .scan(gpu, &mut e, (&dense, ids, 1, 1), &total)?;
    if staged {
        gpu.submit([e.finish()]);
        e = gpu.encoder(&Default::default());
    }
    mark(gpu, staged, &mut stages, "systems", &mut clock).await?;
    // The box (`make.rs`'s first round of the primitives' kernels).
    cfg.extent[3] = 0.0;
    r.run(&mut e, (probe, WOOD, ids, &cfg), &derive)?;
    r.run(&mut e, (probe, LEAVES, cfg.segs, &cfg), &derive)?;
    if fronds {
        r.run(&mut e, (probe, LEAFLETS, cfg.segs, &cfg), &derive)?;
    }
    let both = storage(gpu, "splat box", 8, &[])?;
    e.copy_buffer_to_buffer(&total, 0, &both, 0, 4);
    e.copy_buffer_to_buffer(&work, WORDS * u64::from(ids) * 4, &both, 4, 24);
    gpu.submit([e.finish()]);
    let words = read_words(gpu, &both, 0, 7).await?;
    let systems = words[0];
    let bounds: [f32; 6] = std::array::from_fn(|k| unord(words[1 + k]));
    mark(gpu, staged, &mut stages, "box", &mut clock).await?;
    let mut grids = Vec::new();
    for ask in asks {
        let grid = match *ask {
            Ask::Fixed(g) => g,
            Ask::Over(n) => Grid::over(bounds.map(f64::from), n),
        };
        let cells = grid.count();
        assert!(
            u64::from(cells) * u64::from(systems.max(1)) < u64::from(u32::MAX),
            "a cell and system key past 32 bits: {cells} cells, {systems} systems"
        );
        let cap = (u64::from(cells) * SLOTS).next_power_of_two();
        let words = 2 + 6 * u64::from(cells) + 2 * cap;
        let index = storage(gpu, "splat grid", words, &[])?;
        let answers = storage(gpu, "splat answers", 4 * (u64::from(cells) + 1), &[])?;
        cfg.dims = [grid.n, grid.n, grid.n, cells];
        cfg.lo = [grid.origin[0], grid.origin[1], grid.origin[2], grid.cell].map(|v| v as f32);
        cfg.buckets = cap as u32;
        let bufs = [
            &*swept.nodes,
            &*device.buffer,
            &*work,
            &*answers,
            &*none[1],
            &*index,
            &*dense,
        ];
        let mut e = gpu.encoder(&Default::default());
        r.run(&mut e, (probe, S_CLEAR, words as u32, &cfg), &bufs)?;
        r.run(&mut e, (probe, S_WOOD, ids, &cfg), &bufs)?;
        r.run(&mut e, (probe, S_LEAVES, cfg.segs, &cfg), &bufs)?;
        if fronds {
            r.run(&mut e, (probe, S_LEAFLETS, cfg.segs, &cfg), &bufs)?;
        }
        if staged {
            gpu.submit([e.finish()]);
            mark(gpu, staged, &mut stages, "splat", &mut clock).await?;
            e = gpu.encoder(&Default::default());
        }
        r.run(&mut e, (probe, S_BEST, cap as u32, &cfg), &bufs)?;
        r.run(&mut e, (probe, S_LIMB, cap as u32, &cfg), &bufs)?;
        r.run(&mut e, (probe, S_FINISH, cells, &cfg), &bufs)?;
        if staged {
            gpu.submit([e.finish()]);
            mark(gpu, staged, &mut stages, "owners", &mut clock).await?;
            e = gpu.encoder(&Default::default());
        }
        gpu.submit([e.finish()]);
        let raw = read_words(gpu, &answers, 0, 4 * (u64::from(cells) + 1)).await?;
        mark(gpu, staged, &mut stages, "readback", &mut clock).await?;
        let (body, tail) = raw.split_at(4 * cells as usize);
        grids.push(Splatted {
            grid,
            answers: body.chunks(4).map(answer).collect(),
            pairs: tail[0],
            lost: tail[1],
            bytes: index.size() + answers.size(),
        });
    }
    Ok(Run {
        grids,
        bounds,
        systems,
        nodes: ids,
        segments: device.segments,
        stages,
        work_bytes: work.size() + dense.size(),
    })
}

fn answer(w: &[u32]) -> Answer {
    Answer {
        wood: w[0] & 1 != 0,
        wood_radius: f32::from_bits(w[1]),
        foliage: w[0] & 2 != 0,
        leaves: f32::from_bits(w[2]),
        limb: (w[3] != u32::MAX).then_some(w[3]),
    }
}

/// A profile's passes summed by label, in first-seen order.
fn report_profile(part: &str, passes: Vec<(String, f64, f64)>) {
    let mut sums: Vec<(String, f64, u32)> = Vec::new();
    for (label, a, b) in passes {
        let ms = (b - a) / 1e6;
        match sums.iter_mut().find(|s| s.0 == label) {
            Some(s) => {
                s.1 += ms;
                s.2 += 1;
            }
            None => sums.push((label, ms, 1)),
        }
    }
    let all: f64 = sums.iter().map(|s| s.1).sum();
    println!("PROFILE {part}: {all:.2} ms of passes");
    for (label, ms, n) in sums {
        println!("PROFILE {part} | {label} | {n} | {ms:.3}");
    }
}

fn list<T: std::str::FromStr>(var: &str, or: &str) -> Vec<T> {
    std::env::var(var)
        .unwrap_or_else(|_| or.into())
        .split(',')
        .filter_map(|v| v.parse().ok())
        .collect()
}

fn family(id: &str, seed: u32) -> Family {
    let mut family = telperion_core::presets::by_identity(id).expect("a preset");
    family.skeleton.seed = seed;
    family
}

fn generator() -> Option<(Gpu, Generator)> {
    let gpu = super::super::super::test_gpu()?;
    let gen = pollster::block_on(Generator::for_fields_async(gpu.clone())).expect("a generator");
    Some((gpu, gen))
}

/// The splat against the CPU reference's per-cube query at the same
/// cells (the grid over the reference's box), and today's device field
/// asked the same cells beside it: one row a preset, seed and side.
#[test]
#[ignore = "a measurement (fn-257 task 12)"]
fn splat_agrees() {
    let _held = super::super::super::whole_tree();
    let Some((gpu, gen)) = generator() else {
        return;
    };
    let probe = pass(&gpu);
    let ids: Vec<String> = list("SPLAT_PRESETS", "norway-spruce,oregon-white-oak,date-palm");
    let seeds: Vec<u32> = list("SPLAT_SEEDS", "1");
    let sides: Vec<u32> = list("SPLAT_SIDES", "24,36,48");
    println!("| Preset | Seed | n | Path | wood J | radius off | foliage J | leaves err | total | limbs | within |");
    println!("|---|--:|--:|---|--:|--:|--:|--:|--:|--:|---|");
    for id in &ids {
        for &seed in &seeds {
            let family = family(id, seed);
            let fronds = family.canopy.rosette_fronds > 0.0;
            let request = Request {
                field: Some(None),
                ..Request::default()
            };
            let reference = pipeline::build(&family, request).expect("the reference");
            let reference = reference.outputs.field.expect("a field");
            let b = reference.bounds().expect("bounds");
            let cpu_box = [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z];
            let asks: Vec<Ask> = sides
                .iter()
                .map(|&n| Ask::Fixed(Grid::over(cpu_box, n)))
                .collect();
            let run =
                pollster::block_on(splat(&gen, &family, &asks, false, &probe)).expect("splatted");
            let today = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
            let off: Vec<f64> = (0..6)
                .map(|k| f64::from(run.bounds[k]) - cpu_box[k])
                .collect();
            println!(
                "BOX {id} {seed}: device {:?}, cpu {cpu_box:.4?}, device less cpu {off:.5?}; nodes {}, segments {}, systems {}, today's field {:.1} MB",
                run.bounds, run.nodes, run.segments, run.systems, today.bytes() as f64 / 1e6
            );
            for s in &run.grids {
                let g = s.grid;
                let cpu: Vec<_> = g
                    .centres()
                    .iter()
                    .map(|c| reference.query(*c, g.cell * 0.5).expect("answered"))
                    .collect();
                let words: Vec<f32> = g
                    .centres()
                    .iter()
                    .flat_map(|c| [c.x as f32, c.y as f32, c.z as f32, (g.cell * 0.5) as f32])
                    .collect();
                let old = pollster::block_on(gen.query_field(&today, &words)).expect("answered");
                for (path, dev) in [("splat", &s.answers), ("today", &old)] {
                    let m = compare(&cpu, dev);
                    let held = within(&m, fronds).map_or("no", |()| "yes");
                    println!(
                        "| {id} | {seed} | {} | {path} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {:.4} | {held} |",
                        g.n, m.wood, m.radius, m.foliage, m.leaves, m.total, m.limbs
                    );
                }
                println!(
                    "TABLE {id} {seed} {}: pairs {} ({:.2} a cell), lost {}, splat bytes {:.2} MB, grid read back {:.2} MB",
                    g.n,
                    s.pairs,
                    f64::from(s.pairs) / f64::from(g.count()),
                    s.lost,
                    s.bytes as f64 / 1e6,
                    f64::from(g.count()) * 16.0 / 1e6
                );
            }
        }
    }
}

/// GPU load now, for the timing rows: `nvidia-smi`'s utilisation.
fn load() -> String {
    std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=utilization.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "?".into())
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) * 0.5
    }
}

/// Warm, paired and interleaved (`SPLAT_PAIRS`, 16): today's `grow_field`
/// and 36³ `query_field` against the drawn stage, the splat to 36 cells a
/// side over the device's box and its readback; each path's peak and
/// held device bytes.
#[test]
#[ignore = "a measurement (fn-257 task 12)"]
fn splat_time() {
    let _held = super::super::super::whole_tree();
    let Some((gpu, gen)) = generator() else {
        return;
    };
    let probe = pass(&gpu);
    let pairs: u32 = list("SPLAT_PAIRS", "16")[0];
    for id in list::<String>("SPLAT_PRESETS", "norway-spruce") {
        let family = family(&id, 1);
        let today = || {
            let t = Clock::now();
            let field = pollster::block_on(gen.grow_field(&family, None)).expect("grown");
            let asked = box_cells(&field, 36);
            pollster::block_on(gen.query_field(&field, &asked)).expect("answered");
            let ms = t.elapsed_ms();
            (ms, field.bytes(), gpu.ledger.live())
        };
        let splatted = || {
            let t = Clock::now();
            let run = pollster::block_on(splat(&gen, &family, &[Ask::Over(36)], false, &probe));
            let run = run.expect("splatted");
            let ms = t.elapsed_ms();
            (ms, run.grids[0].bytes + run.work_bytes, gpu.ledger.live())
        };
        // Warm: each once first.
        today();
        splatted();
        let (mut a, mut b, mut d) = (Vec::new(), Vec::new(), Vec::new());
        println!("| {id} pair | load % | today ms | splat ms | splat - today |");
        println!("|--:|--:|--:|--:|--:|");
        for p in 0..pairs {
            let before = load();
            gpu.ledger.take_peak();
            let (x, y) = if p % 2 == 0 {
                let x = today();
                let pa = gpu.ledger.take_peak();
                let y = splatted();
                let pb = gpu.ledger.take_peak();
                ((x, pa), (y, pb))
            } else {
                let y = splatted();
                let pb = gpu.ledger.take_peak();
                let x = today();
                let pa = gpu.ledger.take_peak();
                ((x, pa), (y, pb))
            };
            let ((ta, held_a, _), peak_a) = x;
            let ((tb, held_b, _), peak_b) = y;
            if p == 0 {
                println!(
                    "MEMORY {id}: today holds {:.1} MB between grow and query, peak {:.1} MB; the splat holds nothing after its readback (the grid on the host, {:.2} MB), its own buffers {:.1} MB while it runs, peak {:.1} MB",
                    held_a as f64 / 1e6,
                    peak_a as f64 / 1e6,
                    36f64.powi(3) * 16.0 / 1e6,
                    held_b as f64 / 1e6,
                    peak_b as f64 / 1e6
                );
            }
            let after = load();
            println!(
                "| {p} | {before}/{after} | {ta:.1} | {tb:.1} | {:+.1} |",
                tb - ta
            );
            a.push(ta);
            b.push(tb);
            d.push(tb - ta);
        }
        let faster = d.iter().filter(|v| **v < 0.0).count();
        println!(
            "PAIRED {id}: today median {:.1} ms, splat median {:.1} ms, paired difference median {:+.1} ms, splat faster {faster} of {pairs}",
            median(a),
            median(b),
            median(d)
        );
    }
}

/// Each stage of the splat's path waited on and timed (`SPLAT_PAIRS`
/// runs, the median), then one run stamped by pass (`profile`).
#[test]
#[ignore = "a measurement (fn-257 task 12)"]
fn splat_stages() {
    let _held = super::super::super::whole_tree();
    let Some((gpu, gen)) = generator() else {
        return;
    };
    let probe = pass(&gpu);
    let runs: u32 = list("SPLAT_PAIRS", "8")[0];
    let sides: Vec<u32> = list("SPLAT_SIDES", "36");
    for id in list::<String>("SPLAT_PRESETS", "norway-spruce,oregon-white-oak,date-palm") {
        let family = family(&id, 1);
        let asks: Vec<Ask> = sides.iter().map(|&n| Ask::Over(n)).collect();
        pollster::block_on(splat(&gen, &family, &asks, true, &probe)).expect("warm");
        let mut each: Vec<(&'static str, Vec<f64>)> = Vec::new();
        for _ in 0..runs {
            let run =
                pollster::block_on(splat(&gen, &family, &asks, true, &probe)).expect("splatted");
            for (k, (name, ms)) in run.stages.into_iter().enumerate() {
                if each.len() <= k {
                    each.push((name, Vec::new()));
                }
                each[k].1.push(ms);
            }
        }
        let line: Vec<String> = each
            .into_iter()
            .map(|(name, v)| format!("{name} {:.2}", median(v)))
            .collect();
        println!("STAGES {id} {sides:?}: {}", line.join(", "));
        assert!(profile::start(&gpu), "timestamps");
        pollster::block_on(splat(&gen, &family, &asks, false, &probe)).expect("profiled");
        report_profile(
            &format!("{id} derivations and splat"),
            profile::finish(&gpu),
        );
    }
}
