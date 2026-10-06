//! The chain grower against the CPU reference executor (fn-209 F1, task
//! 2, R1): every chain of a tree, grown alone on the GPU from its bud and
//! the species point's tables, has the CPU's chain's axes (lineage, PA,
//! birth, how and where each stands, where its apex stopped) and
//! phytomers (cycle and key), in the same order.
use super::super::io;
use super::units::pack;
use crate::Gpu;
use std::time::Instant;
use telperion_space::{
    chain_cases, ChainAxis, ChainBud, ChainTables, Light, Request, Species, Woke,
};

const CHAIN: &str = include_str!("chain.wgsl");
const AXW: usize = 28;
const PW: usize = 6;
const NONE: u32 = u32::MAX;

/// A region's bounds per chain: axes, phytomers, living apexes, sleeping
/// buds and the buds that woke.
#[derive(Debug, Clone, Copy)]
struct Caps {
    axes: u32,
    phyts: u32,
    live: u32,
    sleepers: u32,
    woken: u32,
}

impl Caps {
    fn times(self, k: u32) -> Self {
        Self {
            axes: self.axes * k,
            phyts: self.phyts * k,
            live: self.live * k,
            sleepers: self.sleepers * k,
            woken: self.woken * k,
        }
    }
}

/// A sleeping bud's and a woken bud's words (`chain.wgsl`).
const SW: u32 = 10;
const QW: u32 = 8;

/// The unit tables, then each PA's row (flags, lived, successor, words at)
/// and its words (`chain.wgsl`); the rows' offset in the table.
fn packed(t: &ChainTables) -> (Vec<u32>, Vec<u64>, u32) {
    let (mut table, mut words) = pack(&t.units);
    let rows = table.len() as u32;
    let mut woods = Vec::new();
    for (pa, r) in t.pas.iter().enumerate() {
        let at = words.len() as u32;
        let flags = u32::from(r.lives)
            | u32::from(r.abortion.is_some()) << 1
            | u32::from(r.abortion.is_some_and(|a| a.2.is_some())) << 2
            | u32::from(r.successor.is_some()) << 3
            | u32::from(r.delay.is_some()) << 4
            | u32::from(r.lasting.is_some()) << 5;
        let (a_thr, a_log, rise) = r.abortion.unwrap_or((0, 0, None));
        let (next, go, go62) = r
            .successor
            .map_or((NONE, 0, 0), |(n, g, q)| (n as u32, g, q));
        table.extend([flags, r.lived.map_or(NONE, |l| l as u32), next, at]);
        words.extend([
            r.life,
            r.viability.0,
            r.viability.1 as u64,
            a_thr,
            a_log as u64,
            rise.unwrap_or(0) as u64,
            r.relay[0],
            r.relay[1],
            r.relay[2],
            go,
            go62,
            r.moved.0 as u64,
            r.moved.1 as u64,
            r.delay.unwrap_or(0) as u64,
            r.lasting.unwrap_or(0) as u64,
            0, // the log woods' offset, set below
            r.relay_at,
        ]);
        woods.push((at as usize + 15, pa));
    }
    for (slot, pa) in woods {
        words[slot] = words.len() as u64;
        words.extend(t.log_woods[pa].iter().map(|&l| l as u64));
    }
    (table, words, rows)
}

fn bud_words(b: &ChainBud) -> [u64; 10] {
    let pair = |lo: u32, hi: u32| u64::from(lo) | u64::from(hi) << 32;
    let w = b.woke.unwrap_or(Woke {
        key: 0,
        pa: 0,
        sleep: 0,
        stage: 0,
        spent: 0,
        cycle: 0,
    });
    [
        b.lineage,
        pair(b.pa as u32, b.birth),
        pair(u32::from(b.seed), b.node as u32),
        b.root,
        b.horizon as u64,
        pair(u32::from(b.woke.is_some()), u32::from(b.slot)),
        w.key,
        pair(w.pa as u32, w.stage as u32),
        w.spent,
        pair(w.sleep as u32, w.cycle),
    ]
}

struct Grower {
    pass: io::Pass,
}

/// One dispatch's chains as grown: each chain's axes, or none where it
/// outgrew its region.
type Grown = Vec<Option<Vec<ChainAxis>>>;

impl Grower {
    fn new(gpu: &Gpu) -> Self {
        let shader = format!("{}\n{CHAIN}", super::zones());
        let read = [true, true, true, false, false, false, false, false];
        Self {
            pass: io::Pass::new(gpu, shader, &read, &["grow"]),
        }
    }

    fn run(
        &self,
        gpu: &Gpu,
        buds: &[ChainBud],
        t: &(Vec<u32>, Vec<u64>, u32),
        (span, age): (i64, u32),
        caps: Caps,
    ) -> Grown {
        let storage = wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC;
        let n = buds.len() as u32;
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let cfg: [u32; 12] = [
            n,
            row,
            caps.axes,
            caps.phyts,
            caps.live,
            t.2,
            age,
            span as u32,
            (span >> 32) as u32,
            caps.sleepers,
            caps.woken,
            0,
        ];
        let input: Vec<u64> = buds.iter().flat_map(bud_words).collect();
        let b =
            |label, bytes: u64, data: &[u8]| io::buffer(gpu, label, bytes, storage, data).unwrap();
        let uniform = io::buffer(
            gpu,
            "chain cfg",
            48,
            wgpu::BufferUsages::UNIFORM,
            bytemuck::cast_slice(&cfg),
        )
        .unwrap();
        let input = b(
            "chain buds",
            input.len() as u64 * 8,
            bytemuck::cast_slice(&input),
        );
        let table = b(
            "chain table",
            t.0.len() as u64 * 4,
            bytemuck::cast_slice(&t.0),
        );
        let words = b(
            "chain words",
            t.1.len() as u64 * 8,
            bytemuck::cast_slice(&t.1),
        );
        let axes_bytes = u64::from(n * caps.axes) * AXW as u64 * 4;
        let phyt_bytes = u64::from(n * caps.phyts) * PW as u64 * 4;
        let axes = b("chain axes", axes_bytes, &[]);
        let phyts = b("chain phytomers", phyt_bytes, &[]);
        let live = b("chain apexes", u64::from(n * caps.live) * 32, &[]);
        let status = b("chain status", u64::from(n) * 16, &[]);
        let sleep = u64::from(n * (caps.sleepers * SW + caps.woken * QW)) * 4;
        let sleepers = b("chain sleeping buds", sleep, &[]);
        let bind = self.pass.bind(
            gpu,
            &[
                &uniform, &input, &table, &words, &axes, &phyts, &live, &status, &sleepers,
            ],
        );
        let mut encoder = gpu.device.create_command_encoder(&Default::default());
        self.pass.run(&mut encoder, &bind, 0, groups);
        gpu.queue.submit([encoder.finish()]);
        let status: Vec<[u32; 4]> =
            bytemuck::cast_slice::<u8, u32>(&io::read(gpu, &status, u64::from(n) * 16).unwrap())
                .as_chunks::<4>()
                .0
                .to_vec();
        let axes = io::read(gpu, &axes, axes_bytes).unwrap();
        let axes: &[u32] = bytemuck::cast_slice(&axes);
        let phyts = io::read(gpu, &phyts, phyt_bytes).unwrap();
        let phyts: &[u32] = bytemuck::cast_slice(&phyts);
        buds.iter()
            .enumerate()
            .map(|(c, bud)| {
                let [count, grown, overflow, _] = status[c];
                (overflow == 0).then(|| {
                    let a = &axes[c * caps.axes as usize * AXW..];
                    let p = &phyts[c * caps.phyts as usize * PW..];
                    topology(bud, &a[..count as usize * AXW], &p[..grown as usize * PW])
                })
            })
            .collect()
    }
}

/// A chain's axes from its region, as `telperion_space::chain_cases` gives
/// the CPU's: the bearer's stub left out.
fn topology(bud: &ChainBud, axes: &[u32], phyts: &[u32]) -> Vec<ChainAxis> {
    let root = usize::from(!bud.seed);
    let word = |i: usize, f: usize| axes[i * AXW + f];
    let wide = |i: usize, f: usize| u64::from(word(i, f)) | u64::from(word(i, f + 1)) << 32;
    let count = axes.len() / AXW;
    let mut out: Vec<ChainAxis> = (root..count)
        .map(|i| {
            let kind = word(i, 4) & 0xff;
            let parent = word(i, 5);
            let lateral = kind == 1;
            ChainAxis {
                woken: word(i, 4) & 0x80000 != 0,
                parent: (parent != NONE && parent as usize >= root).then(|| parent as usize - root),
                kind: kind as u8,
                node: if lateral || kind == 3 {
                    word(i, 6) as usize
                } else {
                    0
                },
                slot: if lateral {
                    (word(i, 4) >> 8 & 0xff) as u8
                } else {
                    0
                },
                lineage: wide(i, 0),
                pa: word(i, 2) as usize,
                birth: word(i, 3),
                apex_end: word(i, 7).checked_sub(1),
                phytomers: Vec::new(),
            }
        })
        .collect();
    for p in phyts.as_chunks::<PW>().0 {
        let axis = p[3] as usize;
        if axis >= root {
            let key = u64::from(p[0]) | u64::from(p[1]) << 32;
            out[axis - root].phytomers.push((p[2], key));
        }
    }
    out
}

/// Grows every chain of `species` at `age` and `seed` on the GPU, in
/// batches, each outgrown chain again in a region eight times larger, and
/// holds each against the CPU's. The chains, axes and phytomers compared,
/// and the GPU's milliseconds from upload to readback.
fn check(
    gpu: &Gpu,
    grower: &Grower,
    species: &Species,
    (age, seed): (u32, u64),
) -> (usize, usize, usize, f64) {
    let request = Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    };
    let cases = chain_cases(species, request).expect("chains");
    let tables = ChainTables::new(species, age).expect("tables");
    let packed = packed(&tables);
    let buds: Vec<ChainBud> = cases.iter().map(|c| c.0).collect();
    let start = Instant::now();
    let mut grown: Grown = Vec::with_capacity(buds.len());
    let caps = Caps {
        axes: 256,
        phyts: 1024,
        live: 32,
        sleepers: 128,
        woken: 64,
    };
    for batch in buds.chunks(4096) {
        grown.extend(grower.run(gpu, batch, &packed, (tables.span, age), caps));
    }
    let mut caps = caps;
    while grown.iter().any(Option::is_none) {
        caps = caps.times(8);
        let over: Vec<usize> = (0..grown.len()).filter(|&c| grown[c].is_none()).collect();
        for batch in over.chunks(64) {
            let these: Vec<ChainBud> = batch.iter().map(|&c| buds[c]).collect();
            for (&c, g) in
                batch
                    .iter()
                    .zip(grower.run(gpu, &these, &packed, (tables.span, age), caps))
            {
                grown[c] = g;
            }
        }
        assert!(caps.axes < 1 << 16, "a chain no region holds");
    }
    let ms = start.elapsed().as_secs_f64() * 1e3;
    let (mut axes, mut phytomers) = (0, 0);
    for (c, ((bud, want), got)) in cases.iter().zip(&grown).enumerate() {
        let got = got.as_ref().expect("grown");
        assert_eq!(got.len(), want.len(), "chain {c} ({bud:?}): axes");
        for (k, (g, w)) in got.iter().zip(want).enumerate() {
            assert_eq!(g, w, "chain {c} axis {k}");
        }
        axes += want.len();
        phytomers += want.iter().map(|a| a.phytomers.len()).sum::<usize>();
    }
    (cases.len(), axes, phytomers, ms)
}

/// The species with every setting a chain reads while no bud reads light
/// (FINE-CONTRACT.md's adversarial cases): the beech; a beech whose
/// lifespans run to fractions, which grows ages within a cycle
/// (`carry_on`) and partial units; the spruce, whose sleeping buds wake;
/// and the oak with its light switched off, whose sleeping buds wake into
/// later stages and whose lifespans run to fractions too.
fn adversarial() -> Vec<(&'static str, Species)> {
    let fractional = |mut s: Species| {
        for state in s.states.iter_mut().filter(|s| s.lifespan > 0.0) {
            state.lifespan = state.lifespan * 1.37 + 0.29;
        }
        s
    };
    let mut oak = telperion_space::oak();
    for state in &mut oak.states {
        state.shade_hazard = 0.0;
        state.shade_size = 0.0;
        state.balance_hazard = 0.0;
        state.leaf_girth = 0.0;
    }
    vec![
        ("beech", telperion_space::beech()),
        (
            "beech, fractional lifespans",
            fractional(telperion_space::beech()),
        ),
        ("spruce", telperion_space::spruce()),
        ("oak, light off", oak.clone()),
        ("oak, light off, fractional lifespans", fractional(oak)),
    ]
}

#[test]
fn every_chain_grows_on_the_gpu_as_on_the_cpu() {
    let Some(gpu) = crate::generation::test_gpu() else {
        return;
    };
    let grower = Grower::new(&gpu);
    for (name, species) in adversarial() {
        for seed in [1, 7] {
            let (chains, axes, phytomers, ms) = check(&gpu, &grower, &species, (30, seed));
            println!(
                "{name} 30 {seed}: {chains} chains, {axes} axes, {phytomers} phytomers, {ms:.1} ms"
            );
        }
    }
}
