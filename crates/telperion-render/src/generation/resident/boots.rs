//! fn-266 (fn-256 DESIGN.md 12.8): the retained leaf bases hung on the
//! device between the dress and the folds (`boots.wgsl`): two drawn
//! records a base in a wave of their own after every drawn wave, in the
//! room the dress left for them in its records (`most`), and, where the
//! bases pack, a cell each in the renderer's section layout. One
//! readback: the clothed stems, which size the rest. A family that hangs
//! no base adds no wave.
use super::super::io;
use super::fold::{record, Drawn, Dressed, CA_WORDS};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::foliage::CanopyParams;
use telperion_core::pipeline::executor;
use telperion_core::presets::Family;
use telperion_core::surface::SECTION_FLOATS;

const BOOTS: &str = include_str!("boots.wgsl");
const KIDS: usize = 0;
const APICES: usize = 1;
const PLACE: usize = 2;
const LATTICE: usize = 3;
const HANG: usize = 4;
/// A clothed stem's words and a base's (`boots.wgsl`).
pub(super) const AW: u64 = 12;
const BW: u64 = 16;
/// `leaf_bases.rs`' `UPRIGHT`: the nearest a packed base leans to its axis.
const UPRIGHT: f64 = 15.0;

pub(super) fn source() -> String {
    format!(
        "{}\n{}\n{BOOTS}",
        super::super::cycle::KEYS,
        super::super::cycle::ORGANS
    )
}

pub(super) struct Hanger {
    pass: io::Pass,
}

impl Hanger {
    pub(super) fn new(gpu: &Gpu) -> Self {
        let read = [false, false, false, false, false];
        let entries = ["kids", "apices", "place", "lattice", "hang"];
        Self {
            pass: io::Pass::new(gpu, source(), &read, &entries),
        }
    }
}

/// The bases hung: how many, the clothed stems' slots (a test reads
/// their apices) and the cells, where the bases pack.
#[derive(Default)]
pub(super) struct Hung {
    #[cfg(test)]
    pub bases: u32,
    #[cfg(test)]
    pub stems: Option<Owned>,
    pub cells: Option<Owned>,
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

/// `degrees` as radians within half a turn either way, reduced in f64 as
/// `lattice::cell` reduces it, so the f32 the device reads is the step
/// itself and not a turn and a step that cancel there (fn-297 R6).
fn half_turn(degrees: f64) -> f64 {
    use std::f64::consts::{PI, TAU};
    (degrees.to_radians() + PI).rem_euclid(TAU) - PI
}

/// The rows `boots.wgsl` reads for `n` invocations: the seed wave's, the
/// boots' first id, where they go (`room`) and the canopy's.
fn cfg(
    gpu: &Gpu,
    n: u32,
    (seed, out_base): (&Drawn, u32),
    out: [u32; 3],
    p: &CanopyParams,
) -> Result<Owned> {
    let most = gpu.device.limits().max_compute_workgroups_per_dimension;
    let packed = p.leaf_base_width > 0.0;
    let pitch = match packed {
        true => p.leaf_base_pitch.clamp(UPRIGHT, 180.0 - UPRIGHT),
        false => p.leaf_base_pitch,
    };
    let f = |v: f64| (v as f32).to_bits();
    let words = [
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
    let usage = wgpu::BufferUsages::UNIFORM;
    io::buffer(gpu, "boots cfg", 80, usage, bytemuck::cast_slice(&words))
}

impl Hanger {
    /// The bases `p` hangs on `dressed`'s stems, appended as a wave of
    /// their own in its room; none where `p` is none or no stem is
    /// clothed, refused by name where the room cannot hold them.
    pub(super) async fn hang(
        &self,
        gpu: &Gpu,
        dressed: &mut Dressed,
        p: Option<&CanopyParams>,
    ) -> Result<Hung> {
        let (Some(p), Some(seed)) = (p, dressed.waves.get(1)) else {
            return Ok(Hung::default());
        };
        let carried = storage(gpu, "boots carried", u64::from(seed.phyts))?;
        let stems = storage(gpu, "boots stems", 2 + u64::from(seed.phyts) * AW)?;
        let none = [
            storage(gpu, "boots none", 4)?,
            storage(gpu, "boots none", 4)?,
        ];
        let mut e = gpu.encoder(&Default::default());
        let out = room(dressed);
        for (entry, n) in [(KIDS, seed.phyts), (APICES, 1)] {
            let c = cfg(gpu, n, (seed, 0), out, p)?;
            let bind = self.pass.bind(
                gpu,
                &[&c, &dressed.records, &carried, &stems, &none[0], &none[1]],
            );
            self.pass.run(&mut e, &bind, entry, n.div_ceil(64).max(1));
        }
        gpu.submit([e.finish()]);
        let raw = io::read_async(gpu, &stems, 8).await?;
        let [clothed, bases] = [0, 1].map(|w| bytemuck::cast_slice::<u8, u32>(&raw)[w]);
        if clothed == 0 {
            return Ok(Hung::default());
        }
        if 2 * u64::from(bases) > u64::from(dressed.room) {
            return Err(telperion_core::Error::ResourceLimit("leaf base nodes").into());
        }
        let table = storage(gpu, "boots table", u64::from(bases) * BW)?;
        let packed = p.leaf_base_width > 0.0;
        let cells = match packed {
            true => Some(storage(
                gpu,
                "boot cells",
                u64::from(bases) * SECTION_FLOATS as u64,
            )?),
            false => None,
        };
        let out_base = dressed.ids;
        let mut e = gpu.encoder(&Default::default());
        for (entry, n) in [(PLACE, bases), (LATTICE, clothed), (HANG, bases)] {
            let c = cfg(gpu, n, (seed, out_base), out, p)?;
            let at = cells.as_ref().unwrap_or(&none[1]);
            let bind = self
                .pass
                .bind(gpu, &[&c, &dressed.records, &carried, &stems, &table, at]);
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
        })
    }
}
