//! fn-266 (fn-256 DESIGN.md 12.8): the retained leaf bases hung on the
//! device between the dress and the folds (`boots.wgsl`): two drawn
//! records a base in a wave of their own after every drawn wave, in the
//! room the dress left for them in its records (`most`), and, where the
//! bases pack, a cell each in the renderer's section layout. Where the
//! canopy stands a rosette, each crown's fronds are drawn here once, into
//! the table the stations read them from (`fronds.wgsl`, fn-283 R4). One
//! readback: the clothed stems and the crowns, which size the rest. A
//! family that hangs no base adds no wave.
use super::super::io;
use super::fold::{record, Drawn, Dressed, CA_WORDS};
use super::stations::{organ_rows, COUNTS};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::foliage::{blade, CanopyParams, Crown};
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
        "{}\n{}\n{COUNTS}\n{BOOTS}\n{FRONDS}",
        super::super::cycle::KEYS,
        super::super::cycle::ORGANS
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
/// and its counts of organs (`organ_rows`).
#[derive(Debug, Clone, Copy)]
pub(super) struct FrondRows {
    rosette: [[f32; 4]; 7],
    turn: u32,
    organs: [u32; 12],
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
    }))
}

fn storage(gpu: &Gpu, label: &'static str, words: u64) -> Result<Owned> {
    let usage =
        wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST;
    io::buffer(gpu, label, words.max(4) * 4, usage, &[])
}

/// The most bases one of `family`'s stems hangs: its count's floor, and
/// one more where it has a share (`telperion_space::drawn`); none where
/// it hangs none. The dress holds room for this many on every stem.
pub(in crate::generation) fn most(family: &Family) -> Result<u32> {
    Ok(executor::bases(family)?.map_or(0, |p| {
        let word = telperion_space::word(p.leaf_bases);
        (word >> 16) + u32::from(word & 0xffff != 0)
    }))
}

/// Where the boots go in `dressed`'s records: their records' first word,
/// their axis's and its slot.
fn room(dressed: &Dressed) -> [u32; 3] {
    let axis = dressed.axes + dressed.room_slot * CA_WORDS as u32;
    [record(dressed.ids), axis, dressed.room_slot]
}

/// The rows `boots.wgsl` reads for `n` invocations: the seed wave's, the
/// boots' first id, where they go (`room`), the canopy's where it hangs
/// bases and the crowns' where it stands a rosette (none where not).
fn cfg(
    gpu: &Gpu,
    n: u32,
    (seed, out_base): (&Drawn, u32),
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
        f(p.rosette_divergence.to_radians()),
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
    });
    // The rosette's rows start on the uniform's 16-byte boundary.
    words.extend([c.turn, 0, 0, 0]);
    words.extend(c.rosette.as_flattened().iter().map(|v| v.to_bits()));
    words.extend(c.organs);
    let bytes = bytemuck::cast_slice(&words);
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(gpu, "boots cfg", bytes.len() as u64, usage, bytes)
}

impl Hanger {
    /// The bases `p` hangs on `dressed`'s stems, appended as a wave of
    /// their own in its room, and the fronds of its crowns drawn by
    /// `crown`; none where `p` is none or no stem is clothed, and no
    /// table where no apex draws a living frond. Refused by name where the
    /// room cannot hold the bases.
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
        let carried = storage(gpu, "boots carried", phyts)?;
        let stems = storage(gpu, "boots stems", p.map_or(0, |_| 2 + phyts * AW))?;
        let crowns = storage(gpu, "boots crowns", crown.map_or(0, |_| 2 + phyts * CW))?;
        let totals = storage(gpu, "boots totals", 4)?;
        let none = [
            storage(gpu, "boots none", 4)?,
            storage(gpu, "boots none", 4)?,
        ];
        let mut e = gpu.encoder(&Default::default());
        let out = room(dressed);
        let entries = [
            Some((KIDS, seed.phyts)),
            p.map(|_| (APICES, 1)),
            crown.map(|_| (CROWNS, 1)),
        ];
        for (entry, n) in entries.into_iter().flatten() {
            let c = cfg(gpu, n, (seed, 0), out, rows)?;
            let bind = self.pass.bind(
                gpu,
                &[
                    &c,
                    &dressed.records,
                    &carried,
                    &stems,
                    &none[0],
                    &none[1],
                    &crowns,
                ],
            );
            self.pass.run(&mut e, &bind, entry, n.div_ceil(64).max(1));
        }
        e.copy_buffer_to_buffer(&stems, 0, &totals, 0, 8);
        e.copy_buffer_to_buffer(&crowns, 0, &totals, 8, 8);
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &totals, 16).await?;
        let [clothed, bases, apices, fronds] =
            std::array::from_fn(|w| bytemuck::cast_slice::<u8, u32>(&raw)[w]);
        if 2 * u64::from(bases) > u64::from(dressed.room) {
            return Err(telperion_core::Error::ResourceLimit("leaf base nodes").into());
        }
        let mut e = gpu.encoder(&Default::default());
        // Each crown's fronds after the crowns' words, one invocation a
        // crown.
        let table = match apices {
            0 => None,
            _ => {
                let listed = 2 + u64::from(apices) * CW;
                let table = storage(gpu, "frond table", listed + u64::from(fronds) * FW)?;
                e.copy_buffer_to_buffer(&crowns, 0, &table, 0, listed * 4);
                let c = cfg(gpu, apices, (seed, 0), out, rows)?;
                let bind = self.pass.bind(
                    gpu,
                    &[
                        &c,
                        &dressed.records,
                        &carried,
                        &stems,
                        &none[0],
                        &none[1],
                        &table,
                    ],
                );
                self.pass
                    .run(&mut e, &bind, CROWN_FRONDS, apices.div_ceil(64));
                Some(table)
            }
        };
        if clothed == 0 {
            gpu.submit([e.finish()]);
            return Ok(Hung {
                fronds: table,
                ..Hung::default()
            });
        }
        let base_table = storage(gpu, "boots table", u64::from(bases) * BW)?;
        let packed = p.is_some_and(|p| p.leaf_base_width > 0.0);
        let cells = match packed {
            true => Some(storage(
                gpu,
                "boot cells",
                u64::from(bases) * SECTION_FLOATS as u64,
            )?),
            false => None,
        };
        let out_base = dressed.ids;
        for (entry, n) in [(PLACE, bases), (LATTICE, clothed), (HANG, bases)] {
            let c = cfg(gpu, n, (seed, out_base), out, rows)?;
            let at = cells.as_ref().unwrap_or(&none[1]);
            let bind = self.pass.bind(
                gpu,
                &[
                    &c,
                    &dressed.records,
                    &carried,
                    &stems,
                    &base_table,
                    at,
                    &crowns,
                ],
            );
            self.pass.run(&mut e, &bind, entry, n.div_ceil(64));
        }
        gpu.submit([e.finish()]);
        dressed.waves.push(Drawn {
            phyts: 2 * bases,
            base: out_base,
            slot: dressed.room_slot,
            root: false,
        });
        dressed.ids += 2 * bases;
        Ok(Hung {
            #[cfg(test)]
            bases,
            #[cfg(test)]
            stems: Some(stems),
            cells,
            fronds: table,
        })
    }
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
