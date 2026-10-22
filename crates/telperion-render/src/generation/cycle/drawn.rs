//! fn-256 task 3 (DESIGN.md 12.2 and 12.3): the settled tree drawn as
//! the pipeline's nodes on the device, a wave of chains a dispatch
//! (`dress.wgsl`), into the one buffer of records the folds, the bases,
//! the sweep, the stations and the field read (`Dressed`, fn-293); then
//! the trunk's measures folded over them (`stems.wgsl`) and read back.
use super::super::io;
use super::super::resident::fold::{record, Drawn, Dressed, CA_WORDS, CV};
use super::chains::{Chains, Wave};
use super::hier_scan::{uniform, widest, words, zeroed};
use super::settle_run::View;
use crate::buffer::Owned;
use crate::{Gpu, Result};

const DRESS: &str = include_str!("dress.wgsl");
/// The stem's establishment, appended to the dress (fn-275 R2a).
const ESTABLISH: &str = include_str!("establish.wgsl");
const STEMS: &str = include_str!("stems.wgsl");
const COUNT: usize = 0;
const NEAREST: usize = 1;
const FORK: usize = 2;
const WIDEST: usize = 3;
const ROOT: usize = 4;
/// The root's record's flags: taken and made (`dress.wgsl`).
const TAKEN_MADE: u32 = 3;
/// The prefix a record or branch of the wave before carries.
const PREV: u32 = 0x8000_0000;

pub(super) struct Kernels {
    dress: io::Pass,
    stems: io::Pass,
}

impl Kernels {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let source = format!(
            "{}\n{DRESS}\n{ESTABLISH}",
            super::contract_with(super::ARITH)
        );
        let entries = ["count", "nearest", "fork", "widest", "root"];
        Self {
            dress: io::Pass::new(gpu, source, &[true, true, false, true, true], &["dress"]),
            stems: io::Pass::new(gpu, STEMS.to_string(), &[false, false, false], &entries),
        }
    }
}

/// What the dress reads besides the tree: each PA's row (`dress.wgsl`'s
/// `rows`), the tree's age, the scale it is drawn at, how far into its
/// last year it is drawn (fn-270 R5a: 1, the year whole) and, drawn
/// within it, each phytomer's leaf share (`present.rs`).
pub(super) struct Rows<'a> {
    pub rows: &'a [f32],
    pub age: u32,
    pub scale: f32,
    pub fraction: f32,
    pub leaf: Option<&'a Owned>,
    /// The rows its rosette's stem establishes by (`Establishing::words`).
    pub establishing: [f32; 3],
    /// The most records a stem's clothing takes (fn-266, fn-283;
    /// `boots::most`): its retained leaf bases' and its frond axes'. The
    /// records hold this many for every axis of the seed's wave.
    pub clothing: u32,
}

/// The trunk's measures: the root's radius and the stem radius a canopy
/// measures its slender wood against (`Tree::stem_radius`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub(in crate::generation) struct Trunk {
    pub root: f64,
    pub stem: f64,
}

/// The root's axis and record, the only ones of the wave before the
/// seed: at the origin, taken, made and its own.
fn root() -> ([f32; CA_WORDS], [f32; CV]) {
    let mut r = [0.0f32; CV];
    r[6] = f32::from_bits(TAKEN_MADE);
    r[7] = f32::from_bits(PREV);
    r[16] = f32::from_bits(u32::MAX);
    r[23] = f32::from_bits(PREV);
    // The root keeps every leaf, whole: it bears no station.
    r[31] = f32::from_bits(u32::MAX);
    r[32] = 1.0;
    r[33] = f32::from_bits(u32::MAX);
    r[34] = 1.0;
    (r[35], r[36], r[37]) = (0.0, 0.0, 0.0);
    ([0.0; CA_WORDS], r)
}

fn storage(gpu: &Gpu, label: &'static str, words: u64, data: &[u8]) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, data)
}

/// The waves' places in the one buffer of records (fn-293): the root's
/// and each of `chains`' waves', each its first id and first axis slot;
/// the ids they take, the room for `clothing` records a stem of the seed's
/// wave, and the slot the clothing's axis takes. Every count is the chains' own, read
/// with the plan: no readback is added.
fn laid(chains: &Chains, clothing: u32) -> Result<(Vec<Drawn>, [u32; 3])> {
    let root = Drawn {
        phyts: 1,
        base: 0,
        slot: 0,
        root: true,
    };
    let (mut ids, mut slots, mut waves) = (1u64, 1u64, vec![root]);
    for w in &chains.waves {
        waves.push(Drawn {
            phyts: w.phyts + w.axes,
            base: word(ids)?,
            slot: word(slots)?,
            root: false,
        });
        ids += u64::from(w.phyts + w.axes);
        slots += u64::from(w.axes);
    }
    let seed = chains.waves.first().map_or(0, |w| w.axes);
    let room = u64::from(seed) * u64::from(clothing);
    // Every word of the buffer is addressed by a u32.
    word((ids + room) * CV as u64 + (slots + 1) * CA_WORDS as u64)?;
    Ok((waves, [word(ids)?, word(room)?, word(slots)?]))
}

/// A count or offset of the drawn records, refused by name past a u32.
fn word(v: u64) -> Result<u32> {
    u32::try_from(v).map_err(|_| telperion_core::Error::ResourceLimit("drawn records").into())
}

/// Every wave of `chains` drawn from the settled tree `s`, each from the
/// one before, the first from the root, into one buffer of records.
pub(super) fn dress(
    gpu: &Gpu,
    k: &Kernels,
    (s, chains): (View, &Chains),
    r: &Rows,
) -> Result<Dressed> {
    let rows = storage(
        gpu,
        "dress rows",
        r.rows.len() as u64,
        bytemuck::cast_slice(r.rows),
    )?;
    let whole = storage(gpu, "dress whole leaves", 1, &[])?;
    let leaf = r.leaf.unwrap_or(&whole);
    let (waves, [ids, room, room_slot]) = laid(chains, r.clothing)?;
    let slots = room_slot + u32::from(room > 0);
    let axes = record(ids + room);
    let words = u64::from(axes) + u64::from(slots) * CA_WORDS as u64;
    let records = storage(gpu, "drawn", words, &[])?;
    let (axis, root) = root();
    gpu.queue
        .write_buffer(&records, 0, bytemuck::cast_slice(&root));
    gpu.queue
        .write_buffer(&records, u64::from(axes) * 4, bytemuck::cast_slice(&axis));
    let t = s.table;
    let mut prev = Wave {
        axes: 1,
        ..Wave::default()
    };
    let mut held = Vec::new();
    let mut e = gpu.encoder(&Default::default());
    for (w, wave) in chains.waves.iter().enumerate() {
        let (out, before) = (&waves[w + 1], &waves[w]);
        let cfg = [
            wave.chains,
            widest(gpu).0,
            wave.first,
            wave.axes_at,
            wave.phyts_at,
            prev.axes_at,
            prev.phyts_at,
            record(out.base),
            record(before.base),
            out.base,
            before.base,
            wave.phyts,
            r.scale.to_bits(),
            r.age,
            t.sk,
            t.skp,
            t.at_np,
            chains.stride,
            u32::from(w == 0),
            r.fraction.to_bits(),
            u32::from(r.leaf.is_some()),
            axes,
            out.slot,
            before.slot,
        ];
        let cfg: Vec<u32> = cfg
            .into_iter()
            .chain(r.establishing.map(f32::to_bits))
            .collect();
        let cfg = uniform(gpu, &cfg)?;
        let bind = k
            .dress
            .bind(gpu, &[&cfg, s.nt, &chains.plan, &records, &rows, leaf]);
        k.dress
            .run(&mut e, &bind, 0, wave.chains.div_ceil(64).max(1));
        held.push(cfg);
        prev = *wave;
    }
    gpu.submit([e.finish()]);
    Dressed::new(gpu, (records, axes), (waves, ids), (room, room_slot))
}

/// The trunk's measures folded over `dressed`'s records, and its root's
/// record drawn at the root's radius.
pub(super) async fn trunk(gpu: &Gpu, k: &Kernels, dressed: &Dressed) -> Result<Trunk> {
    let parts = zeroed(gpu, "stem parts", u64::from(dressed.ids))?;
    let start = [0u32, u32::MAX, u32::MAX, 0];
    let sums = storage(gpu, "stem sums", 4, bytemuck::cast_slice(&start))?;
    let mut e = gpu.encoder(&Default::default());
    let mut held = Vec::new();
    for entry in [COUNT, NEAREST, FORK, WIDEST] {
        for w in dressed.waves.iter().filter(|w| !w.root && w.phyts > 0) {
            let cfg = uniform(gpu, &[w.phyts, widest(gpu).0, record(w.base), 0])?;
            let bind = k.stems.bind(gpu, &[&cfg, &dressed.records, &parts, &sums]);
            k.stems.run(&mut e, &bind, entry, w.phyts.div_ceil(64));
            held.push(cfg);
        }
    }
    let cfg = uniform(gpu, &[1, widest(gpu).0, record(0), 0])?;
    let bind = k.stems.bind(gpu, &[&cfg, &dressed.records, &parts, &sums]);
    k.stems.run(&mut e, &bind, ROOT, 1);
    gpu.submit([e.finish()]);
    let s = words(gpu, &sums, 0, 4).await?;
    let root = f64::from(f32::from_bits(s[0]));
    let stem = match s[2] {
        u32::MAX => root,
        _ => f64::from(f32::from_bits(s[3])),
    };
    drop(held);
    Ok(Trunk { root, stem })
}
