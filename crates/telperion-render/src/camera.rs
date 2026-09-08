//! The judging pose. One rule places the camera from the subject's bounds, so a
//! still and a browser frame of the same tree are the same picture.
use telperion_core::{math::Vec3, surface::Bounds};

/// Vertical field of view, degrees. Narrow enough that the perspective does not
/// editorialise about the trunk, wide enough to stand at a believable distance.
pub const FIELD_OF_VIEW: f64 = 38.0;
/// Air around the subject: a quarter again its own extent, so it stays clear of
/// every edge while it is orbited.
pub const FRAME_MARGIN: f64 = 1.3;
/// Where the camera stands as a direction from the subject's centre: a
/// three-quarter view from the front right, a little above the middle. Only the
/// angle is authored; the distance is solved.
const FRAME_DIRECTION: Vec3 = Vec3::new(0.62, 0.28, 1.0);
/// Nothing is visible closer than this and the room is what moves, so the near
/// plane is fixed at every subject size.
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

/// The pose that judges a tree of these bounds at this aspect: framed on what
/// is actually there, far enough back that it fits both ways with the margin,
/// and never underground however low the subject's centre sits.
pub fn hero_pose(bounds: Bounds, aspect: f64, ground_reach: f64) -> Camera {
    let size = bounds.max - bounds.min;
    let centre = (bounds.min + bounds.max) * 0.5;

    // The vertical half-angle is the camera's own; the horizontal one is that
    // times the aspect, so a wide viewport pulls in and a tall one pulls back.
    let half_angle = FIELD_OF_VIEW.to_radians() / 2.0;
    let across = size.x.max(size.z) / 2.0;
    let distance = (size.y / 2.0 / half_angle.tan())
        .max(across / (half_angle.tan() * aspect.max(0.1)))
        * FRAME_MARGIN;

    let reach = (distance + size.length() / 2.0).max(1.0);
    let mut position = centre + FRAME_DIRECTION.normalized() * reach;
    position.y = position.y.max(crate::scene::FIGURE_HEIGHT);

    Camera {
        position,
        target: centre,
        field_of_view: FIELD_OF_VIEW,
        near: NEAR,
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

    /// The same pose turned around the subject, which is what the margin is
    /// for: the tree is orbited, and it may not leave the frame on the way.
    fn orbited(camera: &Camera, angle: f64) -> Camera {
        Camera {
            position: camera.target + (camera.position - camera.target).rotate(Vec3::Y, angle),
            ..*camera
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
    fn the_margin_keeps_the_subject_clear_all_the_way_round() {
        // Solved for a fit at the subject's centre plane alone, the near
        // corners of a turned subject swing out past the edge; the margin is
        // the air that stops them.
        for aspect in [0.6, 1.0, 16.0 / 9.0, 3.0] {
            let hero = hero_pose(oak_bounds(), aspect, crate::GROUND_REACH);
            for step in 0..24 {
                let angle = std::f64::consts::TAU * f64::from(step) / 24.0;
                let camera = orbited(&hero, angle);
                let view_projection = camera.view_projection(aspect);
                for corner in corners(oak_bounds()) {
                    let (x, y, _) = project(&view_projection, corner);
                    assert!(
                        x.abs() <= 1.0 && y.abs() <= 1.0,
                        "corner {corner:?} left the frame at aspect {aspect}, \
                         {} degrees round: {x}, {y}",
                        angle.to_degrees()
                    );
                }
            }
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
    fn the_far_plane_clears_the_ground_disc() {
        let camera = hero_pose(oak_bounds(), 1.0, crate::GROUND_REACH);
        let stand = camera.position.distance(camera.target);
        assert!(camera.far > stand + crate::GROUND_REACH);
    }
}
