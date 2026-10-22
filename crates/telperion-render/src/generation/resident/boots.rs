//! fn-266 (fn-256 DESIGN.md 12.8): the retained leaf bases hung on the
//! device between the dress and the folds (`boots.wgsl`): two drawn
//! records a base in a wave of their own after every drawn wave, in the
//! room the dress left for them in its records (`most`), and, where the
//! bases pack, a cell each in the renderer's section layout. Where the
//! canopy stands a rosette, each crown's fronds are drawn here once, into
//! the table the stations read them from (`fronds.wgsl`, fn-283 R4), and
//! each frond's axis is hung from that table as wood in the same wave, a
//! record a node past the bases' (fn-283 R1). One readback: the clothed
//! stems and the crowns, which size the rest. A family that hangs no base
//! and draws no axis adds no wave.
use super::super::io;
use super::fold::{record, Drawn, Dressed, CA_WORDS};
use super::stations::{organ_rows, COUNTS};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::foliage::{blade, CanopyParams, Crown, MAX_FRONDS};
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::surface::SECTION_FLOATS;
use telperion_core::tree::Year;

const BOOTS: &str = include_str!("boots.wgsl");
const FRONDS: &str = include_str!("fronds.wgsl");
const KIDS: usize = 0;
const APICES: usize = 1;
const PLACE: usize = 2;
const LATTICE: usize = 3;
const HANG: usize = 4;
const CROWNS: usize = 5;
const CROWN_FRONDS: usize = 6;
const RACHISES: usize = 7;
/// A clothed stem's words and a base's (`boots.wgsl`).
pub(super) const AW: u64 = 12;
const BW: u64 = 16;
/// A crown's words and a frond's in the fronds' table (`fronds.wgsl`).
const CW: u64 = 4;
const FW: u64 = 32;
/// `leaf_bases.rs`' `UPRIGHT`: the nearest a packed base leans to its axis.
const UPRIGHT: f64 = 15.0;

pub(super) fn source() -> String {
    format!(
        "{}\n{}\n{COUNTS}\n{BOOTS}\n{}\n{FRONDS}",
        super::super::cycle::KEYS,
        super::super::cycle::ORGANS,
        super::super::sources::rachis()
    )
}

pub(super) struct Hanger {
    pass: io::Pass,
}

impl Hanger {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [false; 6];
        let entries = [
            "kids",
            "apices",
            "place",
            "lattice",
            "hang",
            "crowns",
            "crown_fronds",
            "rachises",
        ];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }
}

/// The bases hung: how many, the clothed stems' slots (a test reads
/// their apices) and the cells, where the bases pack; and the crowns'
/// fronds (`fronds.wgsl`), where any apex draws a living frond.
#[derive(Default)]
pub(super) struct Hung {
    #[cfg(test)]
    pub bases: u32,
    #[cfg(test)]
    pub stems: Option<Owned>,
    pub cells: Option<Owned>,
    pub fronds: Option<Owned>,
}

/// The rows `fronds.wgsl` draws a crown's fronds by: the rosette's
/// (`rosette_rows`), its divergence as a share of a turn (`rosette_turn`)
/// and its counts of organs (`organ_rows`); and their axes by: the
/// rachis's width and the records a frond's axis takes room for
/// (`axis_nodes`, none where no frond draws one).
#[derive(Debug, Clone, Copy)]
pub(super) struct FrondRows {
    rosette: [[f32; 4]; 7],
    turn: u32,
    organs: [u32; 12],
    width: f32,
    nodes: u32,
}

/// The rows `family`'s crowns are drawn by in `year`, as the
/// supernatural group applies them (`Family::applied`), the blade a young
/// frond's segments are measured against from its element rows; none
/// where its canopy stands no rosette or bears no leaves.
pub(super) fn frond_rows(family: &Family, year: Option<Year>) -> Result<Option<FrondRows>> {
    let family = family.applied();
    let p = &family.canopy;
    if p.rosette_fronds == 0.0 || p.size == 0.0 {
        return Ok(None);
    }
    let blade = blade(&executor::element(family.element)?);
    Ok(Some(FrondRows {
        rosette: rosette_rows(p, year, blade),
        turn: rosette_turn(p),
        organs: organ_rows(p, year),
        width: p.rachis_width as f32,
        nodes: axis_nodes(p),
    }))
}

/// A count's most whole organs: its floor, and one more where it has a
/// share (`telperion_space::drawn`).
fn whole(count: f64) -> u32 {
    let word = telperion_space::word(count);
    (word >> 16) + u32::from(word & 0xffff != 0)
}

/// The records one frond's axis takes room for (`rachis.rs`): its
/// insertion and the most leaflets a frond draws, from the rows' whole
/// counts; none where no frond draws an axis (`rachis::draws`, at a size).
fn axis_nodes(p: &CanopyParams) -> u32 {
    let draws = p.rachis_width > 0.0
        && p.rosette_fronds > 0.0
        && p.size != 0.0
        && p.leaflet_count > 1.0
        && p.rachis_length > 0.0;
    match draws {
        true => whole(p.leaflet_count) + 1,
        false => 0,
    }
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

/// The most records one of `family`'s stems' clothing takes: two for
/// each of the most bases it hangs, and an axis's room (`axis_nodes`) for
/// each of the most fronds its crown draws (`Crown::slots`' rail past its
/// whole living and skirt fronds); none where it hangs neither. The dress
/// holds room for this many on every stem.
pub(in crate::generation) fn most(family: &Family) -> Result<u32> {
    let bases = executor::bases(family)?.map_or(0, |p| whole(p.leaf_bases));
    let p = family.applied().canopy;
    let skirt = if p.skirt_length > 0.0 {
        p.skirt_fronds
    } else {
        0.0
    };
    let fronds = whole(p.rosette_fronds) + whole(skirt) + MAX_FRONDS;
    Ok(2 * bases + fronds * axis_nodes(&p))
}

/// Where the boots go in `dressed`'s records: their records' first word,
/// their axis's and its slot.
fn room(dressed: &Dressed) -> [u32; 3] {
    let axis = dressed.axes + dressed.room_slot * CA_WORDS as u32;
    [record(dressed.ids), axis, dressed.room_slot]
}

/// `degrees` as radians within half a turn either way, reduced in f64 as
/// `lattice::cell` reduces it, so the f32 the device reads is the step
/// itself and not a turn and a step that cancel there (fn-297 R6).
fn half_turn(degrees: f64) -> f64 {
    use std::f64::consts::{PI, TAU};
    (degrees.to_radians() + PI).rem_euclid(TAU) - PI
}

/// The rows `boots.wgsl` reads for `n` invocations: the seed wave's, the
/// boots' first id and their axes' first record past it, where they go
/// (`room`), the canopy's where it hangs bases and the crowns' where it
/// stands a rosette (none where not).
fn cfg(
    gpu: &Gpu,
    n: u32,
    (seed, out_base, axes): (&Drawn, u32, u32),
    out: [u32; 3],
    (p, crown): (Option<&CanopyParams>, Option<&FrondRows>),
) -> Result<Owned> {
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let p = p.copied().unwrap_or_default();
    let packed = p.leaf_base_width > 0.0;
    let pitch = match packed {
        true => p.leaf_base_pitch.clamp(UPRIGHT, 180.0 - UPRIGHT),
        false => p.leaf_base_pitch,
    };
    let f = |v: f64| (v as f32).to_bits();
    let mut words = vec![
        n,
        n.div_ceil(64).max(1).min(most) * 64,
        record(seed.base),
        seed.phyts,
        seed.base,
        telperion_space::word(p.leaf_bases),
        out_base,
        u32::from(packed),
        telperion_space::word(p.rosette_fronds),
        f(half_turn(p.rosette_divergence)),
        f(p.rosette_depth),
        f(pitch.to_radians()),
        f(p.leaf_base_length),
        f(p.leaf_base_radius),
        f(p.leaf_base_weathering),
        f(p.leaf_base_width),
        f(p.leaf_base_flatness),
        out[0],
        out[1],
        out[2],
    ];
    let c = crown.copied().unwrap_or(FrondRows {
        rosette: [[0.0; 4]; 7],
        turn: 0,
        organs: [0; 12],
        width: 0.0,
        nodes: 0,
    });
    // The rosette's rows start on the uniform's 16-byte boundary.
    words.extend([c.turn, 0, 0, 0]);
    words.extend(c.rosette.as_flattened().iter().map(|v| v.to_bits()));
    words.extend(c.organs);
    words.extend([axes, c.nodes, c.width.to_bits(), 0]);
    let bytes = bytemuck::cast_slice(&words);
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(gpu, "boots cfg", bytes.len() as u64, usage, bytes)
}

impl Hanger {
    /// The bases `p` hangs on `dressed`'s stems and the axes of its
    /// crowns' fronds, appended as a wave of their own in its room, and
    /// the fronds drawn by `crown`; none where `p` is none or no stem is
    /// clothed and no frond draws an axis, and no table where no apex
    /// draws a living frond. Refused by name where the room cannot hold
    /// the bases, or the axes past them.
    pub(super) async fn hang(
        &self,
        gpu: &Gpu,
        dressed: &mut Dressed,
        (p, crown): (Option<&CanopyParams>, Option<&FrondRows>),
    ) -> Result<Hung> {
        let Some(seed) = dressed.waves.get(1) else {
            return Ok(Hung::default());
        };
        if p.is_none() && crown.is_none() {
            return Ok(Hung::default());
        }
        let rows = (p, crown);
        let phyts = u64::from(seed.phyts);
        let b = Bound {
            carried: storage(gpu, "boots carried", phyts)?,
            stems: storage(gpu, "boots stems", p.map_or(0, |_| 2 + phyts * AW))?,
            table: storage(gpu, "boots none", 4)?,
            cells: storage(gpu, "boots none", 4)?,
            crowns: storage(gpu, "boots crowns", crown.map_or(0, |_| 2 + phyts * CW))?,
        };
        let totals = storage(gpu, "boots totals", 4)?;
        let mut e = gpu.encoder(&Default::default());
        let out = room(dressed);
        let entries = [
            Some((KIDS, seed.phyts)),
            p.map(|_| (APICES, 1)),
            crown.map(|_| (CROWNS, 1)),
        ];
        for (entry, n) in entries.into_iter().flatten() {
            let c = cfg(gpu, n, (seed, 0, 0), out, rows)?;
            self.dispatch(gpu, &mut e, (entry, n, &c), (&dressed.records, &b));
        }
        e.copy_buffer_to_buffer(&b.stems, 0, &totals, 0, 8);
        e.copy_buffer_to_buffer(&b.crowns, 0, &totals, 8, 8);
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &totals, 16).await?;
        let [clothed, bases, apices, fronds] =
            std::array::from_fn(|w| bytemuck::cast_slice::<u8, u32>(&raw)[w]);
        let axes = u64::from(fronds) * u64::from(crown.map_or(0, |c| c.nodes));
        let records = 2 * u64::from(bases);
        if records > u64::from(dressed.room) {
            return Err(telperion_core::Error::ResourceLimit("leaf base nodes").into());
        }
        if records + axes > u64::from(dressed.room) {
            return Err(telperion_core::Error::ResourceLimit("rachis nodes").into());
        }
        let (records, axes) = (records as u32, axes as u32);
        let mut e = gpu.encoder(&Default::default());
        // Each crown's fronds after the crowns' words, one invocation a
        // crown: the table the axes and the stations read.
        let b = match apices {
            0 => b,
            _ => {
                let listed = 2 + u64::from(apices) * CW;
                let table = storage(gpu, "frond table", listed + u64::from(fronds) * FW)?;
                e.copy_buffer_to_buffer(&b.crowns, 0, &table, 0, listed * 4);
                let c = cfg(gpu, apices, (seed, 0, 0), out, rows)?;
                let b = Bound { crowns: table, ..b };
                self.dispatch(
                    gpu,
                    &mut e,
                    (CROWN_FRONDS, apices, &c),
                    (&dressed.records, &b),
                );
                b
            }
        };
        if clothed == 0 && axes == 0 {
            gpu.submit([e.finish()]);
            return Ok(Hung {
                fronds: (apices > 0).then_some(b.crowns),
                ..Hung::default()
            });
        }
        let out_base = dressed.ids;
        let packed = p.is_some_and(|p| p.leaf_base_width > 0.0);
        let b = Bound {
            table: storage(gpu, "boots table", u64::from(bases) * BW)?,
            cells: match packed {
                true => storage(gpu, "boot cells", u64::from(bases) * SECTION_FLOATS as u64)?,
                false => b.cells,
            },
            ..b
        };
        let hanging = [(PLACE, bases), (LATTICE, clothed), (HANG, bases)];
        let hanging = hanging.into_iter().filter(|_| clothed > 0);
        for (entry, n) in hanging.chain((axes > 0).then_some((RACHISES, fronds))) {
            let c = cfg(gpu, n, (seed, out_base, records), out, rows)?;
            self.dispatch(gpu, &mut e, (entry, n, &c), (&dressed.records, &b));
        }
        gpu.submit([e.finish()]);
        dressed.waves.push(Drawn {
            phyts: records + axes,
            base: out_base,
            slot: dressed.room_slot,
            root: false,
        });
        dressed.ids += records + axes;
        Ok(Hung {
            #[cfg(test)]
            bases,
            #[cfg(test)]
            stems: Some(b.stems),
            cells: packed.then_some(b.cells),
            fronds: (apices > 0).then_some(b.crowns),
        })
    }

    /// `entry` run in `e` for `n` invocations under `c`, over the drawn
    /// `records` and the stage's own buffers (`b`).
    fn dispatch(
        &self,
        gpu: &Gpu,
        e: &mut crate::Encoder,
        (entry, n, c): (usize, u32, &Owned),
        (records, b): (&Owned, &Bound),
    ) {
        let bound: [&wgpu::Buffer; 7] = [
            c, records, &b.carried, &b.stems, &b.table, &b.cells, &b.crowns,
        ];
        let bind = self.pass.bind(gpu, &bound);
        self.pass.run(e, &bind, entry, n.div_ceil(64).max(1));
    }
}

/// The stage's own buffers, as `boots.wgsl` and `fronds.wgsl` bind them:
/// the stems carried on, the clothed stems, the bases' table, their cells
/// and the crowns (their list, then their fronds' table).
struct Bound {
    carried: Owned,
    stems: Owned,
    table: Owned,
    cells: Owned,
    crowns: Owned,
}

/// The rosette's rows as `fronds.wgsl` and `pack.wgsl` read them
/// (`rosette.rs`, `leaflet.rs`): its fronds and skirt, divergence and
/// depth; the living pitch and its spread, the skirt's pitch and length;
/// the leaflets a frond, its rachis length and arch, the leaflet pitch;
/// the terminal share, and the spines' count, share and pitch; the crown
/// drawn in `year` (`Crown::rows`), with the blade a young frond's
/// segments are measured against as a share of a mature leaflet's length
/// (`Crown::segments`'s strap; the element reaching `blade` metres), none
/// where it groups no leaflets.
fn rosette_rows(p: &CanopyParams, year: Option<Year>, blade: f64) -> [[f32; 4]; 7] {
    let crown = Crown::new(year, p).rows().map(|r| r.map(|v| v as f32));
    let skirt = if p.skirt_length > 0.0 {
        p.skirt_fronds
    } else {
        0.0
    };
    let leaflets = if p.leaflet_count > 1.0 && p.rachis_length > 0.0 {
        p.leaflet_count
    } else {
        1.0
    };
    let radians = f64::to_radians;
    let rows = [
        [
            p.rosette_fronds,
            skirt,
            radians(p.rosette_divergence),
            p.rosette_depth,
        ],
        [
            radians(p.rosette_pitch),
            radians(p.rosette_pitch_spread),
            radians(p.skirt_pitch),
            p.skirt_length,
        ],
        [
            leaflets,
            p.rachis_length,
            p.rachis_arch,
            radians(p.leaflet_pitch),
        ],
        [
            p.terminal_leaflet,
            p.acanthophylls,
            p.acanthophyll_length,
            radians(p.acanthophyll_pitch),
        ],
    ]
    .map(|r| r.map(|v| v as f32));
    let [a, b, c, d] = rows;
    let [made, clock, mut establishment] = crown;
    if leaflets > 1.0 && blade > 0.0 {
        establishment[2] = (p.rachis_length / (blade * p.size)) as f32;
    }
    [a, b, c, d, made, clock, establishment]
}

/// The rosette's divergence as a fixed-point share of a turn, which a
/// frond's serial multiplies exactly (`rosette::fronds`' turn).
fn rosette_turn(p: &CanopyParams) -> u32 {
    ((p.rosette_divergence / 360.0).rem_euclid(1.0) * 2f64.powi(32)).round() as u64 as u32
}
