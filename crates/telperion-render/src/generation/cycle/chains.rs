//! fn-256 task 3 (DESIGN.md 12.2): the settled tree's chains planned on
//! the device (`chains.wgsl`), for the dress to draw a wave at a time.
//! Two submissions: each axis's chain and wave and the counts, read back
//! (a fault, each wave's chains, axes and phytomers); then each wave's
//! chains listed in their heads' order, every chain's axes and phytomers
//! placed by scans, and each chain's axes listed in the table's order.
use super::super::io;
use super::hier_scan::{sums_words, uniform, widest, words, zeroed, Scan};
use super::settle_run::View;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const ENTRIES: [&str; 5] = [
    "heads_main",
    "counts_main",
    "flag_main",
    "list_main",
    "walk_main",
];
const HEADS: usize = 0;
const COUNTS: usize = 1;
const FLAG: usize = 2;
const LIST: usize = 3;
const WALK: usize = 4;
/// The plan's columns (`chains.wgsl`).
const COLS: u32 = 11;
const CH_AX: u32 = 8;
const RANK: u32 = 3;
const CH_PH: u32 = 9;
/// `atom`'s header: the waves a tree holds at most, then the columns.
const WAVES: usize = 64;
const COLUMNS: u64 = 256;

pub(super) struct Kernels {
    pass: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let source = include_str!("chains.wgsl").to_string();
        Self {
            pass: io::Pass::new(gpu, source, &[true, false, false], &ENTRIES),
        }
    }
}

/// A wave of chains: its first chain and chains, and where its axes and
/// phytomers start among every wave's, and how many it holds.
#[derive(Debug, Clone, Copy, Default)]
pub(super) struct Wave {
    pub first: u32,
    pub chains: u32,
    pub axes_at: u32,
    pub axes: u32,
    pub phyts_at: u32,
    pub phyts: u32,
}

/// The plan: its columns, a stride apart, and its waves.
pub(super) struct Chains {
    pub plan: Owned,
    pub stride: u32,
    pub waves: Vec<Wave>,
}

/// One planning's recording state.
struct Rec<'a> {
    gpu: &'a Gpu,
    k: &'a Kernels,
    nt: &'a wgpu::Buffer,
    plan: &'a Owned,
    atom: &'a Owned,
    cfg: [u32; 8],
    held: Vec<Owned>,
}

impl Rec<'_> {
    fn run(
        &mut self,
        e: &mut wgpu::CommandEncoder,
        entry: usize,
        words: &[(usize, u32)],
        items: u32,
    ) -> Result<()> {
        let mut cfg = self.cfg;
        for &(k, v) in words {
            cfg[k] = v;
        }
        let buf = uniform(self.gpu, &cfg)?;
        let bind = self
            .k
            .pass
            .bind(self.gpu, &[&buf, self.nt, self.plan, self.atom]);
        self.k.pass.run(e, &bind, entry, items.div_ceil(64).max(1));
        self.held.push(buf);
        Ok(())
    }
}

/// The uniform's words (`chains.wgsl`'s `Cfg`).
const BASE: usize = 1;
const COUNT: usize = 2;
const WAVE: usize = 6;
const FIRST: usize = 7;

/// The chains of the laid tree `s`, planned with `scan`.
pub(super) async fn plan(
    gpu: &Gpu,
    (k, scan): (&Kernels, &io::Pass),
    s: View<'_>,
) -> Result<Chains> {
    let kept = s.table.kept;
    let stride = (kept + 1).next_multiple_of(64);
    let plan = zeroed(gpu, "chain plan", u64::from(COLS) * u64::from(stride))?;
    let atom = zeroed(gpu, "chain counts", COLUMNS + 4 * u64::from(stride))?;
    let cfg = [widest(gpu).0, 0, 0, kept, s.table.sk, stride, 0, 0];
    let mut r = Rec {
        gpu,
        k,
        nt: s.nt,
        plan: &plan,
        atom: &atom,
        cfg,
        held: Vec::new(),
    };
    let mut e = gpu.encoder(&Default::default());
    for w in s.starts.windows(2).filter(|w| w[1] > w[0]) {
        r.run(
            &mut e,
            HEADS,
            &[(BASE, w[0]), (COUNT, w[1] - w[0])],
            w[1] - w[0],
        )?;
    }
    r.run(&mut e, COUNTS, &[], kept)?;
    gpu.submit([e.finish()]);
    let head = words(gpu, &atom, 0, COLUMNS).await?;
    if head[0] != 0 {
        return Err(io::error(format!(
            "a tree of more than {WAVES} lateral orders"
        )));
    }
    let deepest = head[1] as usize;
    let mut waves = Vec::with_capacity(deepest + 1);
    let mut at = Wave::default();
    for w in 0..=deepest {
        let (chains, axes, phyts) = (head[WAVES + w], head[2 * WAVES + w], head[3 * WAVES + w]);
        let wave = Wave {
            first: at.first + at.chains,
            chains,
            axes_at: at.axes_at + at.axes,
            axes,
            phyts_at: at.phyts_at + at.phyts,
            phyts,
        };
        waves.push(wave);
        at = wave;
    }
    let chains = at.first + at.chains;
    let sums = zeroed(gpu, "chain scan sums", sums_words(kept.max(chains), 1))?;
    let tally = zeroed(gpu, "chain scan tally", 64)?;
    let column = |c: u32| u64::from(c) * u64::from(stride);
    let mut scans = Vec::new();
    let mut e = gpu.encoder(&Default::default());
    for (w, wave) in waves.iter().enumerate() {
        let words = [(WAVE, w as u32), (FIRST, wave.first)];
        r.run(&mut e, FLAG, &words, kept)?;
        let s = Scan::new(gpu, scan, (&plan, column(RANK), kept), &sums, (&tally, 0))?;
        s.record(scan, &mut e);
        scans.push(s);
        r.run(&mut e, LIST, &words, kept)?;
    }
    for c in [CH_AX, CH_PH] {
        let s = Scan::new(gpu, scan, (&plan, column(c), chains), &sums, (&tally, 0))?;
        s.record(scan, &mut e);
        scans.push(s);
    }
    r.run(&mut e, WALK, &[(COUNT, chains)], chains)?;
    gpu.submit([e.finish()]);
    Ok(Chains {
        plan,
        stride,
        waves,
    })
}
