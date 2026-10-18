//! A field made: the tree grown, drawn, swept and stationed as a harness
//! build makes it (`build::readied`), each node's limb system found and
//! named densely, the primitives written, and the grid that lists them.
use super::super::super::cycle::tree::{self, Kernels};
use super::super::super::Generator;
use super::super::build::{placeable, readied, Readied, Timed, Tools};
use super::super::stations::{station_rows, Stationed};
use super::{read_words, storage, unbound, Cfg, DeviceField, Rec};
use super::{CARRIER, CLEAR, INHERIT, LEAFLETS, LEAVES, MARKS, OPENS, ORDER, OWN, WIDEST, WOOD};
use crate::buffer::Owned;
use crate::{Gpu, Result};
use telperion_core::foliage::Element;
use telperion_core::math::Vec3;
use telperion_core::pipeline::executor::{self, Expansion};
use telperion_core::presets::Family;
use telperion_core::Error;

/// A node's work words (`field_build.wgsl`'s `WORDS`); the box's six
/// follow every node's.
const WORDS: u64 = 7;

/// `family` grown on the device and its field made there, its limb
/// systems opened by laterals of `limb_order` or less (the family's
/// `clumpSystemOrder` where none is given).
pub(in super::super) async fn grow(
    gpu: &Gpu,
    (gen, kernels, tools): (&Generator, &Kernels, &Tools),
    family: &Family,
    limb_order: Option<u32>,
) -> Result<DeviceField> {
    let _capped = gpu.ledger.capped(gen.device_budget());
    let growth = executor::growth(family)?;
    placeable(family)?;
    let clock = super::super::super::Clock::now();
    let mut at = Vec::new();
    let drawn = tree::drawn(gpu, kernels, &growth).await?;
    at.push(clock.elapsed_ms());
    let Readied { x, swept, ids, .. } =
        readied(gpu, tools, family, drawn, &mut Timed::default()).await?;
    at.push(clock.elapsed_ms());
    let rows = station_rows(&x, x.section()?.segments, None, growth.year(1.0));
    let device = tools.stationer.stations(gpu, &swept, ids, rows).await?;
    at.push(clock.elapsed_ms());
    // A frond's leaflets stand as primitives of their own.
    let fronds = rows.rosette[0][0] > 0.0;
    let mut cfg = cfg(&x, (ids, device.segments), limb_order);
    let leaflets = if fronds {
        u64::from(device.stations)
    } else {
        0
    };
    let prims = u64::from(ids) + u64::from(device.segments) + leaflets;
    let made = primitives(gpu, tools, (&swept.nodes, &device, fronds), &mut cfg, prims).await?;
    at.push(clock.elapsed_ms());
    drop((swept, device));
    let b = made.bounds;
    cfg.extent = [b[3] - b[0], b[4] - b[1], b[5] - b[2], 0.0];
    super::index::grid(&mut cfg, &b, prims);
    let index = super::index::index(gpu, tools, &cfg, (&made.prims, &made.owner)).await?;
    at.push(clock.elapsed_ms());
    if std::env::var("CYCLE_TRACE").is_ok() {
        eprintln!("TRACE field: drawn, readied, stationed, made, indexed at {at:.1?} ms");
    }
    Ok(DeviceField {
        prims: made.prims,
        owner: made.owner,
        index,
        cfg,
        systems: made.systems,
        bounds: made.bounds,
        wood: ids,
        leaves: (prims - u64::from(ids)) as u32,
    })
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

/// What the primitives' pass leaves: the primitives, their systems, the
/// systems named, and the field's box.
struct Made {
    prims: Owned,
    owner: Owned,
    systems: u32,
    bounds: [f32; 6],
}

/// Each node's limb system (`clumping::systems`, by pointer jumps), the
/// systems named densely, the box grown about each node's wood and each
/// segment's (or frond leaflet's) leaves, and each written as a
/// primitive within it.
async fn primitives(
    gpu: &Gpu,
    tools: &Tools,
    (nodes, device, fronds): (&wgpu::Buffer, &Stationed, bool),
    cfg: &mut Cfg,
    prims: u64,
) -> Result<Made> {
    let ids = cfg.nodes;
    let f = &tools.fielder.build;
    let mut r = Rec {
        gpu,
        held: Vec::new(),
    };
    let mut lows = vec![0u32; (WORDS * u64::from(ids)) as usize];
    lows.extend([u32::MAX; 3]);
    lows.extend([0u32; 3]);
    let work = storage(gpu, "field work", lows.len() as u64, &lows)?;
    let prim = storage(gpu, "field primitives", prims * 4, &[])?;
    let owner = storage(gpu, "field systems", prims - u64::from(ids), &[])?;
    let dense = storage(gpu, "field system names", u64::from(ids) + 1, &[])?;
    let none = unbound(gpu)?;
    let bufs = [
        nodes,
        &device.buffer,
        &work,
        &prim,
        &owner,
        &none[0],
        &dense,
    ];
    let mut e = gpu.encoder(&Default::default());
    for entry in [CLEAR, WIDEST, CARRIER, OPENS] {
        r.run(&mut e, (f, entry, ids, cfg), &bufs)?;
    }
    let jumps = 32 - ids.max(1).leading_zeros() + 1;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(&mut e, (f, ORDER, ids, cfg), &bufs)?;
    }
    cfg.flip = jumps % 2;
    r.run(&mut e, (f, OWN, ids, cfg), &bufs)?;
    for j in 0..jumps {
        cfg.flip = j % 2;
        r.run(&mut e, (f, INHERIT, ids, cfg), &bufs)?;
    }
    cfg.flip = jumps % 2;
    r.run(&mut e, (f, MARKS, ids, cfg), &bufs)?;
    let total = storage(gpu, "field systems total", 4, &[])?;
    tools
        .stationer
        .scan(gpu, &mut e, (&dense, ids, 1, 1), &total)?;
    for write in [0.0, 1.0] {
        cfg.extent[3] = write;
        r.run(&mut e, (f, WOOD, ids, cfg), &bufs)?;
        r.run(&mut e, (f, LEAVES, cfg.segs, cfg), &bufs)?;
        if fronds {
            r.run(&mut e, (f, LEAFLETS, cfg.segs, cfg), &bufs)?;
        }
    }
    gpu.submit([e.finish()]);
    let systems = read_words(gpu, &total, 0, 1).await?[0];
    let corners = read_words(gpu, &work, WORDS * u64::from(ids), 6).await?;
    let bounds: [f32; 6] = std::array::from_fn(|k| unord(corners[k]));
    if !(bounds[0] <= bounds[3] && bounds.iter().all(|v| v.is_finite())) {
        return Err(Error::InvalidInput("the device's field is empty").into());
    }
    Ok(Made {
        prims: prim,
        owner,
        systems,
        bounds,
    })
}

/// A float from its ordered bits (`field_build.wgsl`'s `ord`).
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
