use bytemuck::{Pod, Zeroable};
use telperion_core::{
    foliage::{prepared::PreparedStations, Element, Reference, TwigPlacement},
    math::Vec3,
    Family,
};
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
    envelope: [f32; 4],
    curve: [f32; 4],
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
}
pub(super) fn vector(p: Vec3, w: f64) -> [f32; 4] {
    [p.x as f32, p.y as f32, p.z as f32, w as f32]
}
pub(super) fn config(
    f: &Family,
    t: TwigPlacement,
    p: &PreparedStations,
    e: &Element,
    r: Reference,
) -> Config {
    let c = f.canopy;
    let env = f.skeleton.envelope;
    Config {
        counts: [
            p.count,
            p.segments.len() as u32,
            e.positions.len() as u32,
            p.ring_size,
        ],
        random: [
            f.skeleton.seed,
            t.stations_per_internode,
            p.count.div_ceil(256),
            129,
        ],
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
        size: [
            c.size as f32,
            c.size_variation as f32,
            (f.shell_depth * env.max_radius()) as f32,
            0.0,
        ],
        envelope: [
            env.height as f32,
            env.crown_base as f32,
            env.max_radius() as f32,
            env.fullness as f32,
        ],
        curve: [env.shoulder as f32, 0.0, 0.0, 0.0],
    }
}
pub(super) fn segments(p: &PreparedStations) -> Vec<Segment> {
    p.segments
        .iter()
        .map(|s| Segment {
            a: vector(s.endpoints[0], s.radii[0]),
            b: vector(s.endpoints[1], s.radii[1]),
            tangent: vector(s.frame[0], 0.0),
            normal: vector(s.frame[1], 0.0),
            binormal: vector(s.frame[2], 0.0),
            distance: [
                s.along as f32,
                s.span as f32,
                s.phase[0] as f32,
                s.phase[1] as f32,
            ],
            range: [s.first, s.count, s.run_station, 0],
            edge: s.contact.unwrap_or([0; 4]),
        })
        .collect()
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
