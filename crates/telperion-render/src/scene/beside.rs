//! Where the person stands beside a subject: never nearer the camera than
//! the trunk, outside the crown and out of the shadow it throws (fn-310 D7).
//! A still stands it square to the view on the sun's side (`stand`); a film
//! that turns about the subject states its own placement, built from
//! `reach` and `lit`.
use telperion_core::{math::Vec3, surface::Bounds};

use super::{person::person, room::GROUND_REACH};
use crate::{camera::hero_pose, shot::Shot};

/// How far outside the crown's reach the foot stands, in metres.
const MARGIN: f64 = 0.5;
/// The hero frame the person is stood whole in: the default shot's, 16:9.
const ASPECT: f64 = 16.0 / 9.0;
/// How far out from the picture's centre the person may reach, as a share of
/// the half-frame: a small margin inside every edge.
const FIT: f64 = 0.98;

/// Where the person stands and the azimuth it faces (radians, clockwise from
/// +Z seen from above). The renderer draws the placement it is given; a plain
/// submit gives it the tree's own (`hero`).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Placement {
    pub foot: Vec3,
    pub facing: f64,
}

impl Placement {
    /// The person `distance` metres from the trunk on the azimuth `bearing`
    /// (degrees), facing a camera on the azimuth `view`.
    pub fn at(bearing: f64, distance: f64, view: f64) -> Placement {
        let (x, z) = bearing.to_radians().sin_cos();
        Placement {
            foot: Vec3::new(x * distance, 0.0, z * distance),
            facing: view.to_radians(),
        }
    }

    /// Every point of the person's body standing here.
    pub fn body(self) -> Vec<Vec3> {
        let (body, _) = person(self.foot, self.facing, [0.0; 4], 0);
        let body = body.iter().map(|v| {
            let [x, y, z] = v.position.map(f64::from);
            Vec3::new(x, y, z)
        });
        body.collect()
    }
}

/// How far from the trunk the person stands on the azimuth `bearing`
/// (degrees): the reach that way of the crown the bounds inscribe (the one
/// the hero pose frames, `camera::reach_of`) and `MARGIN`.
pub fn reach(bounds: Bounds, bearing: f64) -> f64 {
    let (x, z) = bearing.to_radians().sin_cos();
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    centre.x * x + centre.z * z + (half.x * x).hypot(half.z * z) + MARGIN
}

/// Whether no point of the person at `placement` lies in the shadow the
/// crown the bounds inscribe throws under a sun at `azimuth` and `elevation`
/// (degrees): the ray from each point towards the sun misses the crown.
pub fn lit(bounds: Bounds, placement: Placement, (azimuth, elevation): (f64, f64)) -> bool {
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    let scale = |v: Vec3| Vec3::new(v.x / half.x, v.y / half.y, v.z / half.z);
    let (a, e) = (azimuth.to_radians(), elevation.to_radians());
    let d = scale(Vec3::new(e.cos() * a.sin(), e.sin(), e.cos() * a.cos()));
    placement.body().into_iter().all(|p| {
        let o = scale(p - centre);
        let (a, b, c) = (d.dot(d), 2.0 * o.dot(d), o.dot(o) - 1.0);
        let root = b * b - 4.0 * a * c;
        root < 0.0 || (-b + root.sqrt()) / (2.0 * a) <= 0.0
    })
}

/// Where the person stands beside a tree of these bounds, seen from the
/// azimuth `view` (degrees, the camera's bearing from the trunk) under a sun
/// at azimuth `sun`, and whether the whole person, head to feet, stands
/// inside the hero frame there (fn-310 D7, D6).
///
/// The foot stands square to one side of the view, 90 degrees round from
/// it, so at exactly the trunk's depth along the view, at the crown's
/// `reach` that way. The side is the sun's: the one whose bearing is within
/// 90 degrees of the sun's azimuth, `view + 90` when the sun stands exactly
/// on the view's axis. A crown's shadow keeps its width across the sun's
/// bearing and only stretches away from the sun, so a foot beyond the
/// crown's reach on the sun's side is outside the shadow at any elevation,
/// which is why the sun's elevation does not enter. A still has one view;
/// a film turning across the sun's bearing places the person itself.
///
/// The person faces the view. Out along the side the person keeps its depth
/// and only moves towards the frame's edge, so where the crown's distance
/// cuts it no distance further out holds it whole: beside a subject much
/// shorter than the person it shows cut by the frame (host ruling, D6).
pub fn stand(bounds: Bounds, view: f64, sun: f64) -> (Placement, bool) {
    let placement = at(bounds, view, sun, 0.0);
    let view_projection = hero_pose(bounds, ASPECT, GROUND_REACH).view_projection(ASPECT);
    (placement, whole(&view_projection, placement))
}

/// Where the hero shot of `bounds` stands the person under the row's sun:
/// what a plain submit draws.
pub fn hero(bounds: Bounds, sun: f64) -> (Placement, bool) {
    stand(bounds, Shot::default().azimuth, sun)
}

/// The bearing the person stands at from the trunk, in degrees: square to
/// the view on the sun's side.
fn bearing(view: f64, sun: f64) -> f64 {
    let side = if (sun - view).to_radians().sin() < 0.0 {
        -1.0
    } else {
        1.0
    };
    view + 90.0 * side
}

/// The person `out` metres beyond the crown's reach on its bearing.
fn at(bounds: Bounds, view: f64, sun: f64, out: f64) -> Placement {
    let bearing = bearing(view, sun);
    Placement::at(bearing, reach(bounds, bearing) + out, view)
}

/// Whether the person at a placement stands head to feet inside the frame
/// at `FIT` and in front of the near plane.
fn whole(view_projection: &[f32; 16], placement: Placement) -> bool {
    placement.body().iter().all(|p| {
        let v = [p.x as f32, p.y as f32, p.z as f32, 1.0];
        let clip: [f64; 4] = std::array::from_fn(|row| {
            (0..4)
                .map(|k| view_projection[k * 4 + row] * v[k])
                .sum::<f32>()
                .into()
        });
        let (x, y, depth) = (clip[0] / clip[3], clip[1] / clip[3], clip[2] / clip[3]);
        clip[3] > 0.0 && x.abs() <= FIT && y.abs() <= FIT && (0.0..=1.0).contains(&depth)
    })
}

/// Where the hero shot stands the person beside a tree of these bounds
/// under a sun at azimuth `sun`, whether it is whole in the hero frame
/// there, and every point of its body: what a test reads to find it in a
/// frame or a shadow.
pub fn points(bounds: Bounds, sun: f64) -> (Placement, bool, Vec<Vec3>) {
    let (placement, whole) = hero(bounds, sun);
    (placement, whole, placement.body())
}

#[cfg(test)]
mod tests;
