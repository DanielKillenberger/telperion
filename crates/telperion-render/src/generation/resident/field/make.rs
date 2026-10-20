//! A field read out: the tree grown, drawn, folded and stationed as a
//! harness build makes it (`build::folded`), its drawn records read as
//! the dress wrote them and no curve swept, each node's limb system
//! found, the box grown about its records, and every record written into
//! the grid, which alone is read back.
use super::super::super::cycle::tree::{self, Kernels};
use super::super::super::Generator;
use super::super::build::{folded, placeable, Folded, Timed, Tools};
use super::super::stations::{station_rows, Laid, Stationed};
use super::SPLAT_LIMB;
use super::{read_words, storage, Cfg, Rec};
use super::{BOUND_LEAVES, BOX_LEAVES, BOX_WOOD, CARRIER, CLEAR, GRID_CLEAR, INHERIT, OPENS};
use super::{ORDER, OWN, SPLAT_BEST, SPLAT_BEST_LOW, SPLAT_FINISH, SPLAT_LEAFLETS, SPLAT_LEAVES};
use super::{SPLAT_WOOD, TABLE_CLEAR, WIDEST};
use crate::{Gpu, RenderError, Result};
use telperion_core::field::{Frame, Grid, Occupancy};
use telperion_core::foliage::{Bounds, Element};
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::presets::Family;
use telperion_core::Error;

/// A node's work words (`field.wgsl`'s `WORDS`); the box's six and the
/// table's bound's two follow every node's.
const WORDS: u64 = 7;

/// The grid a field is asked for: `n` cells a side over the device's own
/// box, or a frame named outright, the CPU's, so a test compares the two
/// executors cell by cell.
#[derive(Debug, Clone, Copy)]
pub(in super::super) enum Ask {
    Over(u32),
    #[cfg(test)]
    In(Frame),
}

impl Ask {
    /// The grid's cells a side.
    pub(in super::super) fn n(self) -> u32 {
        match self {
            Ask::Over(n) => n,
            #[cfg(test)]
            Ask::In(f) => f.n,
        }
    }
}

/// `family` grown on the device and its field read out to the grid `ask`
/// names, its limb systems opened by laterals of `limb_order` or less
/// (the family's `clumpSystemOrder` where none is given). A side outside
/// 3 to 256, or a grid past the device's buffer limits, is refused by
/// name before any work.
pub(in super::super) async fn grow(
    gpu: &Gpu,
    (gen, kernels, tools): (&Generator, &Kernels, &Tools),
    family: &Family,
    limb_order: Option<u32>,
    ask: Ask,
) -> Result<Grid> {
    let cells = u64::from(Frame::side(ask.n())?).pow(3);
    let grid_words = 1 + 7 * cells;
    let answer_words = 4 * (cells + 1);
    within(gpu, "field grid", grid_words)?;
    within(gpu, "field answers", answer_words)?;
    let _capped = gpu.ledger.capped(gen.device_budget());
    let growth = executor::growth(family)?;
    placeable(family)?;
    let bases = super::super::boots::most(family)?;
    let drawn = tree::drawn(gpu, kernels, (&growth, bases)).await?;
    let year = growth.year(1.0);
    let Folded { x, dressed, hung } =
        folded(gpu, tools, (family, year), drawn, &mut Timed::default()).await?;
    let ids = dressed.ids;
    // The field reads the drawn records, never a curve swept from them:
    // no node has a contact.
    let table = storage(gpu, "field no sweep", 4, &[])?;
    let laid = Laid {
        records: &dressed.records,
        table: &table,
        contacts: 0,
        fronds: hung.fronds.as_deref(),
    };
    let rows = station_rows(&x, x.section()?.segments, None, year);
    let device = tools.stationer.stations(gpu, laid, ids, rows).await?;
    let mut cfg = cfg(&x, (ids, device.segments), limb_order);
    let read = Read {
        gpu,
        nodes: &dressed.records,
        device: &device,
    };
    read.grid(tools, &mut cfg, ask, (grid_words, answer_words))
        .await
}

/// Whether a buffer of `words` fits the device's limits; refused by name
/// where it would not.
fn within(gpu: &Gpu, buffer: &'static str, words: u64) -> Result<()> {
    let l = gpu.device.limits();
    let limit = l.max_buffer_size.min(l.max_storage_buffer_binding_size);
    let bytes = words * 4;
    match bytes <= limit {
        true => Ok(()),
        false => Err(RenderError::Oversize {
            buffer,
            bytes,
            limit,
        }),
    }
}

/// What the read-out reads: the drawn records and the station segments.
struct Read<'a> {
    gpu: &'a Gpu,
    nodes: &'a wgpu::Buffer,
    device: &'a Stationed,
}

impl Read<'_> {
    /// The limb systems and the box derived, the grid placed, the table
    /// sized from its bound and every record written into the grid.
    async fn grid(
        &self,
        tools: &Tools,
        cfg: &mut Cfg,
        ask: Ask,
        (grid_words, answer_words): (u64, u64),
    ) -> Result<Grid> {
        let (gpu, ids) = (self.gpu, cfg.nodes);
        let mut r = Rec {
            gpu,
            pass: &tools.fielder.0,
            held: Vec::new(),
        };
        let mut lows = vec![0u32; (WORDS * u64::from(ids)) as usize];
        lows.extend([u32::MAX; 3]);
        lows.extend([0u32; 5]);
        let work = storage(gpu, "field work", lows.len() as u64, &lows)?;
        let none = [(); 3].map(|_| storage(gpu, "field unbound", 4, &[]));
        let [a, b, c] = none;
        let (a, b, c) = (a?, b?, c?);
        let mut e = gpu.encoder(&Default::default());
        let segs: &wgpu::Buffer = &self.device.buffer;
        let derive: [&wgpu::Buffer; 6] = [self.nodes, segs, &work, &a, &b, &c];
        systems(&mut r, &mut e, cfg, &derive)?;
        let frame = match ask {
            Ask::Over(n) => {
                r.run(&mut e, (BOX_WOOD, ids, &*cfg), &derive)?;
                r.run(&mut e, (BOX_LEAVES, cfg.segs, &*cfg), &derive)?;
                gpu.submit([e.finish()]);
                e = gpu.encoder(&Default::default());
                let corners = read_words(gpu, &work, WORDS * u64::from(ids), 6).await?;
                let b: [f32; 6] = std::array::from_fn(|k| unord(corners[k]));
                let bounds = Bounds {
                    min: Vec3::new(b[0].into(), b[1].into(), b[2].into()),
                    max: Vec3::new(b[3].into(), b[4].into(), b[5].into()),
                };
                if !(b[0] <= b[3] && b.iter().all(|v| v.is_finite())) {
                    return Err(Error::InvalidInput("the device's field is empty").into());
                }
                Frame::over(bounds, n)?
            }
            #[cfg(test)]
            Ask::In(frame) => frame,
        };
        let cells = frame.n.pow(3);
        cfg.dims = [frame.n, frame.n, frame.n, cells];
        let o = frame.origin;
        cfg.lo = [o.x, o.y, o.z, frame.cell].map(|v| v as f32);
        let grid = storage(gpu, "field grid", grid_words, &[])?;
        let answers = storage(gpu, "field answers", answer_words, &[])?;
        let sized: [&wgpu::Buffer; 6] = [self.nodes, segs, &work, &grid, &b, &answers];
        r.run(&mut e, (GRID_CLEAR, grid_words as u32, &*cfg), &sized)?;
        r.run(&mut e, (BOUND_LEAVES, cfg.segs, &*cfg), &sized)?;
        gpu.submit([e.finish()]);
        let bound = read_words(gpu, &work, WORDS * u64::from(ids) + 6, 2).await?;
        let pairs = u64::from(bound[0]) | (u64::from(bound[1]) << 32);
        let slots = (2 * pairs).max(1).next_power_of_two();
        within(gpu, "field table", 4 * slots)?;
        let past = || Error::ResourceLimit("field table");
        cfg.slots = u32::try_from(4 * slots).map_err(|_| past())? / 4;
        let table = storage(gpu, "field table", 4 * slots, &[])?;
        let bufs: [&wgpu::Buffer; 6] = [self.nodes, segs, &work, &grid, &table, &answers];
        let mut e = gpu.encoder(&Default::default());
        r.run(&mut e, (TABLE_CLEAR, 4 * cfg.slots, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_WOOD, ids, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_LEAVES, cfg.segs, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_LEAFLETS, cfg.segs, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_BEST, cfg.slots, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_BEST_LOW, cfg.slots, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_LIMB, cfg.slots, &*cfg), &bufs)?;
        r.run(&mut e, (SPLAT_FINISH, cells, &*cfg), &bufs)?;
        gpu.submit([e.finish()]);
        let raw = read_words(gpu, &answers, 0, answer_words).await?;
        let (body, tail) = raw.split_at(4 * cells as usize);
        if tail[0] != 0 {
            return Err(Error::ResourceLimit("field table").into());
        }
        let cells = body.chunks(4).map(answer).collect();
        Ok(Grid { frame, cells })
    }
}

/// Each node's limb system (`clumping::systems`, by pointer jumps), named
/// by the node that opens it.
fn systems(
    r: &mut Rec,
    e: &mut wgpu::CommandEncoder,
    cfg: &mut Cfg,
    bufs: &[&wgpu::Buffer],
) -> Result<()> {
    let ids = cfg.nodes;
    for entry in [CLEAR, WIDEST, CARRIER, OPENS] {
        r.run(e, (entry, ids, &*cfg), bufs)?;
    }
    let jumps = 32 - ids.max(1).leading_zeros() + 1;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(e, (ORDER, ids, &*cfg), bufs)?;
    }
    cfg.flip = jumps % 2;
    r.run(e, (OWN, ids, &*cfg), bufs)?;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(e, (INHERIT, ids, &*cfg), bufs)?;
    }
    cfg.flip = jumps % 2;
    Ok(())
}

/// One cell's answer from its four words.
fn answer(w: &[u32]) -> Occupancy {
    Occupancy {
        wood: w[0] & 1 != 0,
        wood_radius: f32::from_bits(w[1]).into(),
        foliage: w[0] & 2 != 0,
        leaves: f32::from_bits(w[2]).into(),
        limb: (w[3] != u32::MAX).then_some(w[3]),
    }
}

/// The build's rows: the nodes and segments, the limbs' order, a leaf's
/// reach and draws, a frond leaflet's box.
fn cfg(x: &Expansion, (nodes, segs): (u32, u32), limb_order: Option<u32>) -> Cfg {
    let [seat, rachis, blade] = x.leaf_reach();
    let canopy = x.leaves().canopy;
    let (leaf_centre, leaf_half) = leaf_box(x.element(), canopy.size);
    Cfg {
        nodes,
        root: 0,
        most: limb_order.unwrap_or(canopy.clump_system_order),
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

/// A float from its ordered bits (`field.wgsl`'s `ord`).
fn unord(w: u32) -> f32 {
    f32::from_bits(if w & 0x8000_0000 != 0 {
        w & 0x7fff_ffff
    } else {
        !w
    })
}

/// A frond leaflet's box in its own frame (`fronds.rs`'s `Shape`): its
/// centre with the leaf's size, and its half extents.
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
