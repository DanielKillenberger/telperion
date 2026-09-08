//! The judging pose. One rule places the camera from the subject's bounds, so a
//! still and a browser frame of the same tree are the same picture.
use telperion_core::{math::Vec3, surface::Bounds};

/// Vertical field of view, degrees. Narrow enough that the perspective does not
/// editorialise about the trunk, wide enough to stand at a believable distance.
pub const FIELD_OF_VIEW: f64 = 38.0;
/// Air around the subject at the hero pose: the tightest corner sits this far
/// out from the picture's centre, as a fraction of the half-frame, so the tree
/// fills the frame and the wheel is how a hand pulls back to orbit it. The
/// owner judged the old orbit stand-off too far away (2026-09-09).
pub const FRAME_MARGIN: f64 = 1.15;
/// Where the camera stands as a direction from the subject's centre: a
/// three-quarter view from the front right, a little above the middle. Only the
/// angle is authored; the distance is solved.
const FRAME_DIRECTION: Vec3 = Vec3::new(0.62, 0.28, 1.0);
/// The near plane for a subject the size of a tree. Anything small enough that
/// a tenth of a metre would clip it draws its own near plane from its reach.
const NEAR: f64 = 0.1;

/// A pose the renderer can draw from. Metres, Y up, right-handed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    /// Vertical field of view, degrees.
    pub field_of_view: f64,
    pub near: f64,
    pub far: f64,
}

impl Camera {
    /// View-projection for a viewport of this aspect, column-major, WebGPU
    /// clip space (depth in 0..1).
    pub fn view_projection(&self, aspect: f64) -> [f32; 16] {
        multiply(
            &perspective(self.field_of_view, aspect, self.near, self.far),
            &look_at(self.position, self.target),
        )
    }
}

/// The eight corners of a box, the points a frame has to hold.
fn corners(bounds: Bounds) -> [Vec3; 8] {
    let (lo, hi) = (bounds.min, bounds.max);
    [
        Vec3::new(lo.x, lo.y, lo.z),
        Vec3::new(hi.x, lo.y, lo.z),
        Vec3::new(lo.x, hi.y, lo.z),
        Vec3::new(hi.x, hi.y, lo.z),
        Vec3::new(lo.x, lo.y, hi.z),
        Vec3::new(hi.x, lo.y, hi.z),
        Vec3::new(lo.x, hi.y, hi.z),
        Vec3::new(hi.x, hi.y, hi.z),
    ]
}

/// The pose that judges a tree of these bounds at this aspect: every corner of
/// the bounds inside the frame with the margin and no more, solved corner by
/// corner so a near corner that subtends more is what sets the distance, and
/// never underground however low the subject's centre sits.
pub fn hero_pose(bounds: Bounds, aspect: f64, ground_reach: f64) -> Camera {
    let size = bounds.max - bounds.min;
    let centre = (bounds.min + bounds.max) * 0.5;

    // The picture's axes at the hero direction, the same frame `look_at` builds.
    let back = FRAME_DIRECTION.normalized();
    let right = Vec3::Y.cross(back).normalized();
    let up = back.cross(right);
    // The vertical half-angle is the camera's own; the horizontal one is that
    // times the aspect, so a wide viewport pulls in and a tall one pulls back.
    let tangent = (FIELD_OF_VIEW.to_radians() / 2.0).tan();
    let solved = corners(bounds)
        .iter()
        .map(|corner| {
            let offset = *corner - centre;
            let need = (offset.dot(up).abs() / tangent)
                .max(offset.dot(right).abs() / (tangent * aspect.max(0.1)));
            offset.dot(back) + need * FRAME_MARGIN
        })
        .fold(0.0, f64::max);
    // A subject with no extent at all leaves nothing to solve, and the eye
    // would land on the target with no direction to look along.
    let reach = if solved > 0.0 { solved } else { NEAR * 2.0 };
    let mut position = centre + back * reach;
    // The eye stands on the floor at least, and at a person's eye height for
    // anything taller than a person. A leaf is not looked at from 1.8 m, so
    // for a subject shorter than the figure the direction alone places it.
    position.y = position.y.max(if size.y > crate::scene::FIGURE_HEIGHT {
        crate::scene::FIGURE_HEIGHT
    } else {
        0.0
    });

    Camera {
        position,
        target: centre,
        field_of_view: FIELD_OF_VIEW,
        // A leaf is looked at from centimetres away; a near plane fixed at a
        // tenth of a metre would clip the whole subject out of the frame.
        near: NEAR.min(reach / 2.0),
        // The far plane clears the whole ground disc from wherever the camera
        // stands: the disc reaches `ground_reach` from the origin and the
        // camera is `reach` out from a target inside it.
        far: reach + ground_reach * 2.0,
    }
}

/// Column-major, element `column * 4 + row`.
fn perspective(field_of_view: f64, aspect: f64, near: f64, far: f64) -> [f64; 16] {
    let f = 1.0 / (field_of_view.to_radians() / 2.0).tan();
    let mut m = [0.0; 16];
    m[0] = f / aspect;
    m[5] = f;
    m[10] = far / (near - far);
    m[11] = -1.0;
    m[14] = near * far / (near - far);
    m
}

fn look_at(eye: Vec3, target: Vec3) -> [f64; 16] {
    let back = (eye - target).normalized();
    let right = Vec3::Y.cross(back).normalized();
    let up = back.cross(right);
    [
        right.x,
        up.x,
        back.x,
        0.0,
        right.y,
        up.y,
        back.y,
        0.0,
        right.z,
        up.z,
        back.z,
        0.0,
        -right.dot(eye),
        -up.dot(eye),
        -back.dot(eye),
        1.0,
    ]
}

fn multiply(a: &[f64; 16], b: &[f64; 16]) -> [f32; 16] {
    let mut out = [0.0f32; 16];
    for column in 0..4 {
        for row in 0..4 {
            let sum: f64 = (0..4).map(|k| a[k * 4 + row] * b[column * 4 + k]).sum();
            out[column * 4 + row] = sum as f32;
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Clip-space x, y and depth of a world point through this view-projection.
    fn project(view_projection: &[f32; 16], p: Vec3) -> (f64, f64, f64) {
        let v = [p.x as f32, p.y as f32, p.z as f32, 1.0];
        let mut clip = [0.0f32; 4];
        for (row, slot) in clip.iter_mut().enumerate() {
            *slot = (0..4).map(|k| view_projection[k * 4 + row] * v[k]).sum();
        }
        let w = clip[3] as f64;
        (clip[0] as f64 / w, clip[1] as f64 / w, clip[2] as f64 / w)
    }

    fn oak_bounds() -> Bounds {
        Bounds {
            min: Vec3::new(-9.4, 0.0, -8.1),
            max: Vec3::new(8.8, 22.6, 9.7),
        }
    }

    #[test]
    fn every_corner_sits_inside_the_frame() {
        for aspect in [0.6, 1.0, 16.0 / 9.0, 3.0] {
            let camera = hero_pose(oak_bounds(), aspect, crate::GROUND_REACH);
            let view_projection = camera.view_projection(aspect);
            for corner in corners(oak_bounds()) {
                let (x, y, depth) = project(&view_projection, corner);
                assert!(
                    x.abs() <= 1.0 && y.abs() <= 1.0,
                    "corner {corner:?} left the frame at aspect {aspect}: {x}, {y}"
                );
                assert!(
                    (0.0..=1.0).contains(&depth),
                    "corner {corner:?} left the depth range at aspect {aspect}: {depth}"
                );
            }
        }
    }

    #[test]
    fn the_margin_is_the_only_air_around_the_subject() {
        // The tightest corner sits exactly on the margin line: any further
        // back and the tree is smaller than the rule says, any closer and a
        // corner leaves the frame.
        for aspect in [0.6, 1.0, 16.0 / 9.0, 3.0] {
            let camera = hero_pose(oak_bounds(), aspect, crate::GROUND_REACH);
            let view_projection = camera.view_projection(aspect);
            let tightest = corners(oak_bounds())
                .iter()
                .map(|&corner| {
                    let (x, y, _) = project(&view_projection, corner);
                    x.abs().max(y.abs())
                })
                .fold(0.0, f64::max);
            assert!(
                (tightest - 1.0 / FRAME_MARGIN).abs() < 2e-3,
                "the tightest corner sits at {tightest} of the half-frame at aspect \
                 {aspect}; the margin says {}",
                1.0 / FRAME_MARGIN
            );
        }
    }

    #[test]
    fn the_camera_never_stands_underground() {
        // A wide, flat subject centred near the ground would otherwise put the
        // eye below it.
        let flat = Bounds {
            min: Vec3::new(-6.0, 0.0, -6.0),
            max: Vec3::new(6.0, 0.4, 6.0),
        };
        let camera = hero_pose(flat, 1.0, crate::GROUND_REACH);
        assert!(camera.position.y >= crate::scene::FIGURE_HEIGHT);
    }

    #[test]
    fn a_subject_smaller_than_a_person_still_fills_its_frame() {
        // The leaf view frames one element: a hand-sized blade, or a spruce
        // needle two centimetres long. Held to a person's eye height, or to a
        // tree's near plane, either is a speck in the middle of the frame.
        for (name, element) in [
            (
                "blade",
                Bounds {
                    min: Vec3::new(-0.03, 0.0, -0.01),
                    max: Vec3::new(0.03, 0.13, 0.01),
                },
            ),
            (
                "needle",
                Bounds {
                    min: Vec3::new(-0.0005, 0.0, -0.0005),
                    max: Vec3::new(0.0005, 0.02, 0.0005),
                },
            ),
        ] {
            let camera = hero_pose(element, 1.0, crate::GROUND_REACH);
            let view_projection = camera.view_projection(1.0);
            let (mut low, mut high) = (f64::MAX, f64::MIN);
            for corner in corners(element) {
                let (x, y, depth) = project(&view_projection, corner);
                assert!(
                    x.abs() <= 1.0 && y.abs() <= 1.0,
                    "a {name} corner {corner:?} left the frame: {x}, {y}"
                );
                assert!(
                    (0.0..=1.0).contains(&depth),
                    "a {name} corner {corner:?} left the depth range: {depth}"
                );
                (low, high) = (low.min(y), high.max(y));
            }
            assert!(
                high - low > 1.0,
                "the {name} covers {:.0}% of the frame height: it is a speck",
                (high - low) * 50.0
            );
        }
    }

    #[test]
    fn the_far_plane_clears_the_ground_disc() {
        let camera = hero_pose(oak_bounds(), 1.0, crate::GROUND_REACH);
        let stand = camera.position.distance(camera.target);
        assert!(camera.far > stand + crate::GROUND_REACH);
    }
}
