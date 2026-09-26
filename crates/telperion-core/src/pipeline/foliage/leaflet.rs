//! A pinnate frond's leaflets before any draw: where each leaves the rachis,
//! the way it points, the rachis's own direction there and the share of its
//! size it is drawn at, with the frame a leaf is turned in about its axis.
//! Placement draws each leaflet from these, and the leaf plan bounds them, so
//! the planned and the placed frond stand in the same place.
use super::{rosette::frame, CanopyParams};
use crate::{
    math::{Transcendental, Vec3},
    rng::Rng,
};
use std::f64::consts::{PI, TAU};

/// One leaflet on its rachis, before its draws turn and scale it.
#[derive(Debug, Clone, Copy)]
pub(super) struct Leaflet {
    pub(super) at: Vec3,
    pub(super) axis: Vec3,
    /// The rachis's direction where the leaflet leaves it.
    pub(super) run: Vec3,
    /// The share of its size a basal leaflet borne as a spine is drawn at.
    pub(super) share: Option<f64>,
}

/// The point `t` of the way along the rachis that leaves `point` on
/// `heading`, bent out of that line by its arch toward `lift`.
pub(super) fn rachis(point: Vec3, heading: Vec3, lift: Vec3, p: &CanopyParams, t: f64) -> Vec3 {
    let length = p.rachis_length;
    point + heading * (length * t) + lift * (p.rachis_arch * length * t * t)
}

/// The `count` leaflets of the rachis that leaves `point` on `heading`.
///
/// The rachis runs a `rachis_length` from the station, bending out of that
/// straight line by its arch as it goes, and each leaflet leaves it by
/// `leaflet_pitch` to alternating sides. The last of them closes the rachis's
/// end as `terminal_leaflet` blends its pitch back toward the rachis itself.
pub(super) fn leaflets(
    point: Vec3,
    heading: Vec3,
    p: CanopyParams,
    count: usize,
) -> impl Iterator<Item = Leaflet> {
    // The frond's own plane: the rachis runs along `heading`, the leaflets
    // leave it to either side, and the arch lifts it out of the line between.
    let (lift, _) = frame(heading);
    let pitch = p.leaflet_pitch.to_radians();
    (0..count).map(move |leaf| {
        let t = (leaf + 1) as f64 / count as f64;
        let at = rachis(point, heading, lift, &p, t);
        let run = (heading + lift * (2. * p.rachis_arch * t)).normalized();
        let closing = if leaf + 1 == count {
            1. - p.terminal_leaflet
        } else {
            1.
        };
        let hand = if leaf % 2 == 0 { 1. } else { -1. };
        // A basal leaflet borne as a spine leaves the rachis at its own pitch
        // and is drawn at its own share of the size the leaflet would have
        // had. No draw moves: the spine is the same instance, scaled.
        let borne = spine(leaf, &p);
        let pitch = borne.map_or(pitch, |(_, steeper)| steeper);
        Leaflet {
            at,
            axis: run.rotate(lift, hand * pitch * closing),
            run,
            share: borne.map(|(share, _)| share),
        }
    })
}

/// The share of its own size a basal leaflet is drawn at and the radians it
/// leaves the rachis on, where the rows bear it as a spine. None everywhere
/// else, so a frond that bears none is scaled by nothing at all.
fn spine(index: usize, p: &CanopyParams) -> Option<(f64, f64)> {
    let borne = (index as u64) < u64::from(p.acanthophylls) && p.acanthophyll_length > 0.;
    borne.then(|| (p.acanthophyll_length, p.acanthophyll_pitch.to_radians()))
}

/// A leaf's frame after its draws, side, axis and face, and its scale: the
/// scatter turns the whole frame by up to `scatter` degrees about a random
/// axis, and the size varies by up to `size_variation` either way. Four
/// draws where the leaf scatters, one where it does not.
pub(super) fn drawn(
    axis: Vec3,
    tangent: Vec3,
    normal: Vec3,
    p: &CanopyParams,
    rng: &mut Rng,
) -> ([Vec3; 3], f64) {
    let (mut face, mut side) = leaf_frame(axis, tangent, normal);
    let mut axis = axis;
    if p.scatter > 0. {
        let z = rng.range(-1., 1.);
        let phi = rng.range(0., TAU);
        let ring = (1. - z * z).max(0.).sqrt();
        let jitter = Vec3::new(ring * phi.cos_fixed(), z, ring * phi.sin_fixed());
        let angle = p.scatter * PI / 180. * rng.next_f64();
        let sin_cos = angle.sin_cos_fixed();
        axis = axis.rotate_sin_cos(jitter, sin_cos);
        face = face.rotate_sin_cos(jitter, sin_cos);
        side = side.rotate_sin_cos(jitter, sin_cos);
    }
    let scale = p.size * (1. + p.size_variation * rng.range(-1., 1.));
    ([side, axis, face], scale)
}

/// The face and the side a leaf pointing along `axis` is turned to before
/// its scatter: the face toward the sky where the axis leaves any, else
/// toward `tangent`, else `normal`, and the side square to both.
pub(super) fn leaf_frame(axis: Vec3, tangent: Vec3, normal: Vec3) -> (Vec3, Vec3) {
    let mut face = Vec3::Y - axis * axis.y;
    if face.length_squared() <= 1e-12 {
        face = tangent - axis * tangent.dot(axis);
    }
    if face.length_squared() <= 1e-12 {
        face = normal - axis * normal.dot(axis);
    }
    let face = face.normalized();
    (face, axis.cross(face).normalized())
}
