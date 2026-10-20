//! The tree's leaf stations on the device (fn-212 task 6;
//! CONTRACT.md's probe, step 6): their bearing runs, station counts and
//! the station segments placement reads, `prepare_compact_stations`'s
//! layout (`data.rs`), from the drawn records and the contacts
//! `sweep.wgsl` left. Only totals are read back: the runs, then the
//! stations and segments.
use super::super::io;
use super::sort::Sorter;
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::foliage::{CanopyParams, Crown};
use telperion_core::pipeline::executor::Expansion;

const STATIONS: &str = include_str!("stations.wgsl");
/// The counts of organs the stations and the fronds read (`counts.wgsl`).
pub(super) const COUNTS: &str = include_str!("counts.wgsl");
pub(super) const SCAN: &str = include_str!("scan.wgsl");
const BLOOM_KERNELS: &str = include_str!("blooms.wgsl");
const SW: u64 = 4;
pub(super) const RW: u64 = 6;
/// A segment's words (`data.rs`'s `Segment`).
pub(super) const GW: usize = 40;
const KIDS: usize = 0;
const BEAR: usize = 1;
const STARTS: usize = 2;
const MEASURE: usize = 3;
const BASES: usize = 4;
const EMIT: usize = 5;
const BLOCKS: usize = 6;
const SPREAD: usize = 7;
const KEYS: usize = 8;
const FLORETS: usize = 9;
const BLOOMS: usize = 10;
/// A node's owned words, scanned together (`stations.wgsl`'s `OW`): its
/// florets and its bloom's record (fn-229).
const OW: u32 = 2;
/// Runs a workgroup of the bases' scan takes.
const LANES: u32 = 256;

pub(super) fn source() -> String {
    format!(
        "{}\n{}\n{COUNTS}\n{STATIONS}\n{BLOOM_KERNELS}",
        super::super::cycle::contract(),
        super::super::sources::bloom()
    )
}

/// The rows the stations read: the stations per internode, the ring's
/// segments, `STRUCTURAL` of the root's radius, the slender bound, the
/// twig layer's internode, the divergence (radians), whether leaves are
/// seated, and whether there are leaves at all.
#[derive(Debug, Clone, Copy)]
pub(super) struct Rows {
    pub spi: u32,
    pub segments: u32,
    pub structural: f32,
    /// The slender bound in whole micrometres (`plan::micrometres`).
    pub slender: u32,
    /// The twig layer's internode in whole micrometres, as the CPU
    /// executor rounds it (`plan::micrometres`), which the stations and
    /// placement count by (fn-248).
    pub unit: u32,
    pub divergence: f32,
    pub contact: bool,
    /// Whether the canopy bears leaves at all: none of zero size, as the
    /// pipeline prepares none (`prepared.rs`; fn-212 task 11).
    pub leaves: bool,
    /// Whether any of its leaf-bearing wood and short shoots bear them:
    /// its cauline share above none (fn-277).
    pub wood: bool,
    /// The blooms' rows where an age flowers (`data::flower`; fn-229).
    pub flower: Option<[[f32; 4]; 3]>,
    /// Whether some wood sheds its leaves by age (fn-248).
    pub retains: bool,
    /// Its counts of organs (`organ_rows`, fn-277).
    pub organs: [u32; 12],
}

/// What the stations read of a drawn tree: its records (`Dressed`), the
/// sweep's table with where its nodes' contacts start, none (0) where no
/// curve was swept, and its crowns' fronds (`Hung::fronds`), none where
/// it stands no rosette.
pub(super) struct Laid<'a> {
    pub records: &'a wgpu::Buffer,
    pub table: &'a wgpu::Buffer,
    pub contacts: u32,
    pub fronds: Option<&'a wgpu::Buffer>,
}

pub(super) struct Stationed {
    pub stations: u32,
    pub segments: u32,
    pub buffer: Owned,
    /// The florets, and the blooms' records placement reads (fn-229):
    /// none where no bloom keeps a floret.
    pub florets: u32,
    pub blooms: u32,
    pub records: Option<Owned>,
}

pub(super) struct Stationer {
    pass: io::Pass,
    sorter: Sorter,
    scan: io::Pass,
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

impl Stationer {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [true, true, false, false, false, false, false, true];
        let entries = [
            "kids", "bear", "starts", "measure", "bases", "emit", "blocks", "spread", "keys",
            "florets", "blooms",
        ];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
            sorter: Sorter::new(gpu),
            scan: io::Pass::new(
                gpu,
                SCAN.into(),
                &[false; 3],
                &["blocks", "spine", "spread"],
            ),
        }
    }

    /// An exclusive scan of the first `fields` words of `n` records
    /// `stride` words apart in `data`, its totals into `totals`
    /// (`scan.wgsl`).
    pub(super) fn scan(
        &self,
        gpu: &Gpu,
        e: &mut wgpu::CommandEncoder,
        (data, n, stride, fields): (&wgpu::Buffer, u32, u32, u32),
        totals: &wgpu::Buffer,
    ) -> Result<()> {
        let blocks = n.div_ceil(256);
        let groups = n.div_ceil(64);
        let row = groups.min(gpu.device.limits().max_compute_workgroups_per_dimension) * 64;
        let words = [n, stride, fields, blocks, row, 0, 0, 0];
        let usage = wgpu::BufferUsages::UNIFORM;
        let cfg = io::buffer(gpu, "scan cfg", 32, usage, bytemuck::cast_slice(&words))?;
        let sums = storage(gpu, "scan sums", u64::from(blocks.max(1)) * 4)?;
        let bind = self.scan.bind(gpu, &[&cfg, data, &sums, totals]);
        self.scan.run(e, &bind, 0, blocks.div_ceil(64));
        self.scan.run(e, &bind, 1, 1);
        self.scan.run(e, &bind, 2, groups);
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn run(
        &self,
        (gpu, e): (&Gpu, &mut wgpu::CommandEncoder),
        (entry, n, groups): (usize, u32, Option<u32>),
        (rows, contacts, fronds): (Rows, u32, &wgpu::Buffer),
        bufs: [&wgpu::Buffer; 7],
    ) -> Result<()> {
        let groups = groups.unwrap_or(n.div_ceil(64)).max(1);
        let most = gpu.device.limits().max_compute_workgroups_per_dimension;
        let mut words = vec![
            n,
            groups.min(most) * 64,
            rows.spi,
            rows.segments,
            rows.structural.to_bits(),
            rows.slender,
            rows.unit,
            rows.divergence.to_bits(),
            contacts,
            u32::from(rows.leaves) | u32::from(rows.wood) << 1,
            u32::from(rows.retains),
            // The flower's rows start on the uniform's 16-byte boundary.
            0,
        ];
        let flower = rows.flower.unwrap_or_default();
        words.extend(flower.as_flattened().iter().map(|v| v.to_bits()));
        words.extend(rows.organs);
        let usage = wgpu::BufferUsages::UNIFORM;
        let bytes = bytemuck::cast_slice(&words);
        let cfg = io::buffer(gpu, "stations cfg", bytes.len() as u64, usage, bytes)?;
        let mut all: Vec<&wgpu::Buffer> = vec![&cfg];
        all.extend(bufs);
        all.push(fronds);
        let bind = self.pass.bind(gpu, &all);
        self.pass.run(e, &bind, entry, groups);
        Ok(())
    }

    /// The counts: the runs, stations and segments, then the florets and
    /// the blooms' records (fn-229).
    async fn counts(gpu: &Gpu, e: &mut crate::Encoder, counts: &wgpu::Buffer) -> Result<[u32; 8]> {
        let fresh = gpu.encoder(&Default::default());
        gpu.submit([std::mem::replace(e, fresh).finish()]);
        let raw = io::read_async(gpu, counts, 32).await?;
        let w: &[u32] = bytemuck::cast_slice(&raw);
        Ok(std::array::from_fn(|k| w[k]))
    }

    /// The stations of the `ids` nodes `laid` holds.
    pub(super) async fn stations(
        &self,
        gpu: &Gpu,
        laid: Laid<'_>,
        ids: u32,
        rows: Rows,
    ) -> Result<Stationed> {
        Ok(self.stations_and_runs(gpu, laid, ids, rows).await?.0)
    }

    /// The stations, and the runs' words (`RW` a run: first node,
    /// stations, segments, nodes, first station and first segment), which
    /// only a test reads: `stations` lets them go (fn-212 task 11).
    pub(super) async fn stations_and_runs(
        &self,
        gpu: &Gpu,
        laid: Laid<'_>,
        ids: u32,
        rows: Rows,
    ) -> Result<(Stationed, Owned)> {
        if !rows.leaves && rows.flower.is_none() {
            let stationed = Stationed {
                stations: 0,
                segments: 0,
                buffer: storage(gpu, "station segments", 0)?,
                florets: 0,
                blooms: 0,
                records: None,
            };
            return Ok((stationed, storage(gpu, "station runs", 0)?));
        }
        // Contacts are read where leaves are seated (`Rows::contact`).
        let no_fronds = storage(gpu, "stations no fronds", 0)?;
        let fronds = laid.fronds.unwrap_or(&no_fronds);
        let at = (rows, if rows.contact { laid.contacts } else { 0 }, fronds);
        let st = storage(gpu, "station words", u64::from(ids) * SW)?;
        let counts = storage(gpu, "station counts", 8)?;
        let none = storage(gpu, "stations none", 4)?;
        let mut e = gpu.encoder(&Default::default());
        let none_runs = storage(gpu, "station runs none", 4)?;
        let owners = storage(gpu, "station owners", u64::from(ids) * u64::from(OW))?;
        let early = |runs| [laid.records, laid.table, &st, runs, &counts, &none, &owners];
        let flowers = rows.flower.is_some();
        let early_entries: &[usize] = match flowers {
            true => &[KIDS, BEAR, FLORETS],
            false => &[KIDS, BEAR],
        };
        for &entry in early_entries {
            self.run((gpu, &mut e), (entry, ids, None), at, early(&none_runs))?;
        }
        // Each node's first floret and record, scanned in node order; their
        // totals read back with the runs' count (fn-229).
        let totals = storage(gpu, "station owned totals", 4)?;
        self.scan(gpu, &mut e, (&owners, ids, OW, OW), &totals)?;
        e.copy_buffer_to_buffer(&totals, 0, &counts, 16, 8);
        // Each run's key, a pair a run: room for their count rounded up to
        // a power of two, as the sort takes them.
        let keys = storage(gpu, "station keys", 2 * u64::from(ids.next_power_of_two()))?;
        self.run((gpu, &mut e), (KEYS, ids, None), at, early(&keys))?;
        let [n, _, _, _, florets, blooms, ..] = Self::counts(gpu, &mut e, &counts).await?;
        // The runs numbered by their keys (fn-261): those past the count
        // sort last.
        let pad = vec![u32::MAX; 2 * (n.next_power_of_two() - n) as usize];
        gpu.queue
            .write_buffer(&keys, u64::from(n) * 8, bytemuck::cast_slice(&pad));
        self.sorter.sort(gpu, &mut e, &keys, n)?;
        let runs = storage(gpu, "station runs", u64::from(ids) * RW)?;
        let sorted = [
            laid.records,
            laid.table,
            &st,
            &runs,
            &counts,
            &keys,
            &owners,
        ];
        self.run((gpu, &mut e), (STARTS, n, None), at, sorted)?;
        self.run(
            (gpu, &mut e),
            (MEASURE, n, None),
            at,
            [
                laid.records,
                laid.table,
                &st,
                &runs,
                &counts,
                &none,
                &owners,
            ],
        )?;
        // The runs' first stations and segments, scanned in blocks.
        let blocks = n.div_ceil(LANES).max(1);
        let totals = storage(gpu, "station blocks", u64::from(blocks) * 2)?;
        for (entry, groups) in [(BASES, blocks), (BLOCKS, 1), (SPREAD, blocks)] {
            self.run(
                (gpu, &mut e),
                (entry, n, Some(groups)),
                at,
                [
                    laid.records,
                    laid.table,
                    &st,
                    &runs,
                    &counts,
                    &totals,
                    &owners,
                ],
            )?;
        }
        let [_, stations, segments, ..] = Self::counts(gpu, &mut e, &counts).await?;
        // Each segment's words, then the node each stands on (`owns`).
        let words = u64::from(segments) * (GW as u64 + 1);
        let buffer = storage(gpu, "station segments", words)?;
        let late: [&wgpu::Buffer; 7] = [
            laid.records,
            laid.table,
            &st,
            &runs,
            &counts,
            &buffer,
            &owners,
        ];
        self.run((gpu, &mut e), (EMIT, n, None), at, late)?;
        let mut records = None;
        if blooms > 0 {
            let words = u64::from(blooms) * GW as u64;
            let records = &*records.insert(storage(gpu, "bloom records", words)?);
            let late: [&wgpu::Buffer; 7] = [
                laid.records,
                laid.table,
                &st,
                &runs,
                &counts,
                records,
                &owners,
            ];
            self.run((gpu, &mut e), (BLOOMS, ids, None), at, late)?;
        }
        gpu.submit([e.finish()]);
        io::complete(gpu).await?;
        let stationed = Stationed {
            stations,
            segments,
            buffer,
            florets,
            blooms,
            records,
        };
        Ok((stationed, runs))
    }
}

/// The rows the device's stations read from the pipeline's expansion, its
/// rings `segments` vertices round; the blooms' `flower` rows where an age
/// flowers (`Organ::rows`, checked before they are read); the `year` the
/// tree is drawn in (`Growth::year`).
pub(super) fn station_rows(
    x: &Expansion,
    segments: u32,
    flower: Option<[[f32; 4]; 3]>,
    year: Option<telperion_core::tree::Year>,
) -> Rows {
    let [structural, slender] = x.bearing_bounds();
    let (twig, canopy) = (x.twig(), x.leaves().canopy);
    Rows {
        spi: twig.stations_per_internode,
        segments,
        structural: structural as f32,
        // `plan::micrometres`: the bound as the CPU executor compares it.
        slender: (slender * 1e6 + 0.5).floor() as u32,
        unit: (twig.internode_length * 1e6 + 0.5).floor() as u32,
        // Reduced to a turn in f64 before it is narrowed, as the CPU
        // block's placement takes it (`data::config`; fn-212 task 11).
        divergence: (canopy.divergence * std::f64::consts::PI / 180.0)
            .rem_euclid(std::f64::consts::TAU) as f32,
        contact: x.seats(),
        leaves: canopy.size != 0.0,
        wood: canopy.size != 0.0 && canopy.cauline > 0.0,
        flower,
        retains: x.retains(),
        organs: organ_rows(&canopy, year),
    }
}

/// The counts of organs `stations.wgsl` and `fronds.wgsl` draw (fn-277):
/// the cauline share, the living fronds, the dead fronds (none where they
/// are drawn at no length), the leaflets and the spines as counts' words
/// (`telperion_space::word`); the rachis's length; the newest frond's
/// serial at the lesser and the greater whole living count an apex draws;
/// and the fronds a crown draws (`Crown::slots`) at each pair of whole
/// living and dead counts, lesser first.
pub(super) fn organ_rows(p: &CanopyParams, year: Option<telperion_core::tree::Year>) -> [u32; 12] {
    let word = telperion_space::word;
    let skirt = if p.skirt_length > 0.0 {
        p.skirt_fronds
    } else {
        0.0
    };
    let crown = |fronds: f64, skirt_fronds: f64| {
        Crown::new(
            year,
            &CanopyParams {
                rosette_fronds: fronds,
                skirt_fronds,
                ..*p
            },
        )
    };
    let (f, k) = (p.rosette_fronds.floor(), skirt.floor());
    let newest = |fronds: f64| (crown(fronds, k).made().ceil() as i64 - 1) as i32 as u32;
    let slots = |fronds: f64, skirt: f64| crown(fronds, skirt).slots() as u32;
    [
        word(p.cauline),
        word(p.rosette_fronds),
        word(skirt),
        word(p.leaflet_count),
        word(p.acanthophylls),
        (p.rachis_length as f32).to_bits(),
        newest(f),
        newest(f + 1.0),
        slots(f, k),
        slots(f, k + 1.0),
        slots(f + 1.0, k),
        slots(f + 1.0, k + 1.0),
    ]
}
