use bytemuck::{Pod, Zeroable};
use telperion_core::{
    foliage::{prepared::PreparedStations, BloomParams, Element, Reference, TwigPlacement},
    math::Vec3,
    pipeline::executor::LeafInput,
};
use telperion_space::Transcendental;
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct Config {
    counts: [u32; 4],
    random: [u32; 4],
    box_min: [f32; 4],
    box_extent: [f32; 4],
    shape: [f32; 4],
    lean: [f32; 4],
    size: [f32; 4],
}
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct Segment {
    a: [f32; 4],
    b: [f32; 4],
    tangent: [f32; 4],
    normal: [f32; 4],
    binormal: [f32; 4],
    distance: [f32; 4],
    range: [u32; 4],
    edge: [u32; 4],
    /// Its leaves' share of their whole size (fn-270 R5), then spare.
    unfolded: [f32; 4],
}
pub(super) fn vector(p: Vec3, w: f64) -> [f32; 4] {
    [p.x as f32, p.y as f32, p.z as f32, w as f32]
}
/// The placement's rows for `count` stations on `segments` segments, their
/// contacts on rings of `ring_size` vertices; the leaves written from the
/// leaf buffer's start (`at` places them further on).
pub(super) fn config(
    l: &LeafInput,
    t: TwigPlacement,
    (count, segments, ring_size): (u32, u32, u32),
    e: &Element,
    r: Reference,
) -> Config {
    let c = l.canopy;
    Config {
        counts: [count, segments, e.positions.len() as u32, ring_size],
        random: [l.seed, t.stations_per_internode, count.div_ceil(256), 0],
        box_min: vector(r.min, 0.0),
        box_extent: vector(r.extent, 0.0),
        shape: [
            t.internode_length as f32,
            (c.divergence * std::f64::consts::PI / 180.0).rem_euclid(std::f64::consts::TAU) as f32,
            c.surface_contact as f32,
            c.scatter.to_radians() as f32,
        ],
        lean: [
            c.forward_lean as f32,
            c.lean_rise as f32,
            c.outward as f32,
            c.upward as f32,
        ],
        // The internode in whole micrometres as the CPU rounds it
        // (`plan::micrometres`), which a segment's retention reads (fn-248).
        size: [
            c.size as f32,
            c.size_variation as f32,
            f32::from_bits((t.internode_length * 1e6 + 0.5).floor() as u32),
            0.0,
        ],
    }
}
/// A bloom's rows as `bloom.wgsl`'s `Flower` reads them (fn-229), drawn at
/// `now`: the florets, taper, scatter and the time since the window opened
/// (wrapped to a year in f64, so a window of any length survives the
/// narrowing); the window's length and stagger, and the droop; the rachis,
/// the pedicel and the stalk's spread as its sine and cosine, taken as
/// `bloom.rs` takes them.
pub(super) fn flower(p: &BloomParams, now: f64) -> [[f32; 4]; 3] {
    let (sin, cos) = p.spread.to_radians().sin_cos_fixed();
    let since = (now - p.opens).rem_euclid(1.0);
    [
        [p.florets, p.taper, p.scatter, since],
        [p.lasts, p.stagger, p.droop, 0.0],
        [p.rachis, p.pedicel, sin, cos],
    ]
    .map(|row| row.map(|v| v as f32))
}
/// The florets' rows: `count` instances of `records` blooms, each bloom's
/// florets then its rachis (fn-253), the organ element `e`, packed against
/// `r` (fn-229).
pub(super) fn florets(
    rows: [[f32; 4]; 3],
    (count, records): (u32, u32),
    e: &Element,
    r: Reference,
) -> Config {
    Config {
        counts: [count, records, e.positions.len() as u32, 0],
        random: [0, 0, count.div_ceil(256), 0],
        box_min: vector(r.min, 0.0),
        box_extent: vector(r.extent, 0.0),
        shape: rows[0],
        lean: rows[1],
        size: rows[2],
    }
}
pub(super) fn segments<R>(p: &PreparedStations<R>) -> Vec<Segment> {
    p.segments
        .iter()
        .map(|s| {
            let retention = s.retention.unwrap_or((0, u32::MAX, u32::MAX, 1.0));
            Segment {
                a: vector(s.endpoints[0], s.radii[0]),
                b: vector(s.endpoints[1], s.radii[1]),
                // Where some of its stations shed their leaves (fn-248): the
                // boundary in whole micrometres, the retentions before and from
                // it a year before (fn-270 R5a), and the stream their draws
                // take; every station keeps its leaf where they are all ones.
                tangent: worded(s.frame[0], retention.0),
                normal: worded(s.frame[1], retention.1),
                binormal: worded(s.frame[2], retain(s.key)),
                // A cluster (fn-223) is marked by a negative span, its spread
                // where a run's phase stands.
                distance: match s.cluster {
                    Some(spread) => [0.0, -1.0, spread as f32, 0.0],
                    None => [
                        s.along as f32,
                        s.span as f32,
                        s.phase[0] as f32,
                        s.phase[1] as f32,
                    ],
                },
                range: [s.first, s.count, s.run_station, s.key],
                edge: s.contact.unwrap_or([0; 4]),
                // Its leaves' share of their whole size (fn-270 R5), the
                // retentions now and the share a leaf kept only a year
                // before is drawn at (R5a).
                unfolded: [s.unfolding as f32, f32::from_bits(retention.2), retention.3, 0.0],
            }
        })
        .collect()
}

/// A direction with a word in its fourth lane, its bits as they are.
fn worded(v: Vec3, word: u32) -> [f32; 4] {
    [v.x as f32, v.y as f32, v.z as f32, f32::from_bits(word)]
}

/// The stream a run's retention draws take (`tree::keeps`): its first
/// node's key under the registry's `RETAIN`.
fn retain(key: u32) -> u32 {
    telperion_core::tree::mix32(key ^ telperion_space::lineage_keys::RETAIN as u32)
}
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
pub(super) struct Mass {
    pub count: u32,
    pub reach: u32,
    pub pad: [u32; 2],
    pub minimum: [f32; 4],
    pub extent: [f32; 4],
    pub grid_min: [f32; 4],
    pub dims: [u32; 4],
}
