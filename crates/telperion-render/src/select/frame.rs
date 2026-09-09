//! What one frame decides against: the frustum a leaf has to fall inside, the
//! eye it is measured from, and how many pixels a metre spans there. All of it
//! is solved on the host once per frame and handed to the pass as one uniform.
use bytemuck::{Pod, Zeroable};
use telperion_core::{foliage::Element, math::Vec3};

/// The frame as `select.wgsl` reads it. Every vector is padded to four
/// components because a uniform is, so the fourth carries the scalar that
/// belongs with it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
pub struct Uniforms {
    pub planes: [[f32; 4]; 6],
    /// The eye, and the pixels one metre spans at one metre of depth.
    pub eye: [f32; 4],
    /// The direction the eye looks, and the near plane.
    pub forward: [f32; 4],
    /// The element's bounding sphere at the origin: centre, then radius.
    pub sphere: [f32; 4],
    pub instances: u32,
    pub levels: u32,
    pub stride: u32,
    pub forced: i32,
}

/// A vector and the scalar that travels beside it.
pub fn point(p: Vec3, w: f64) -> [f32; 4] {
    [p.x as f32, p.y as f32, p.z as f32, w as f32]
}

/// How many pixels a metre spans at a metre of depth: the vertical half-frame
/// in pixels over the tangent of the half angle it covers. A perspective
/// projection shrinks by depth alone, so the pass divides this by depth and
/// has its answer.
pub fn pixels_per_metre(height: f64, field_of_view: f64) -> f64 {
    (height / 2.0) / (field_of_view.to_radians() / 2.0).tan()
}

/// The element's bounding sphere at the origin: the centre of its extent, and
/// the furthest vertex from that centre.
pub fn sphere(element: &Element) -> [f32; 4] {
    let Some((min, max)) = element
        .positions
        .iter()
        .fold(None, |bounds, &p| match bounds {
            None => Some((p, p)),
            Some((min, max)) => Some((
                Vec3::new(min.x.min(p.x), min.y.min(p.y), min.z.min(p.z)),
                Vec3::new(max.x.max(p.x), max.y.max(p.y), max.z.max(p.z)),
            )),
        })
    else {
        return [0.0; 4];
    };
    let centre = (min + max) * 0.5;
    let radius = element
        .positions
        .iter()
        .fold(0.0_f64, |far, p| far.max(p.distance_squared(centre)))
        .sqrt();
    point(centre, radius)
}

/// The six planes of this view-projection's frustum, each pointing inward and
/// normalized, so `dot(n, p) + d` is a signed distance in metres and a sphere
/// is outside when that distance is under minus its radius. Clip space here
/// keeps depth in zero to w, so the near plane is the third row alone.
pub fn planes(view_projection: &[f32; 16]) -> [[f32; 4]; 6] {
    let row = |i: usize| {
        [
            view_projection[i],
            view_projection[4 + i],
            view_projection[8 + i],
            view_projection[12 + i],
        ]
    };
    let (x, y, z, w) = (row(0), row(1), row(2), row(3));
    let sum = |a: [f32; 4], b: [f32; 4]| std::array::from_fn(|i| a[i] + b[i]);
    let difference = |a: [f32; 4], b: [f32; 4]| std::array::from_fn(|i| a[i] - b[i]);
    [
        sum(w, x),
        difference(w, x),
        sum(w, y),
        difference(w, y),
        z,
        difference(w, z),
    ]
    .map(|plane| {
        let length = (plane[0] * plane[0] + plane[1] * plane[1] + plane[2] * plane[2]).sqrt();
        if length > 0.0 {
            plane.map(|term| term / length)
        } else {
            plane
        }
    })
}

#[cfg(test)]
mod tests {
    use telperion_core::foliage::{build_element, ElementParams};
    use telperion_core::surface::Bounds;

    use super::*;
    use crate::{hero_pose, GROUND_REACH};

    fn distance(plane: &[f32; 4], p: Vec3) -> f64 {
        f64::from(plane[0]) * p.x
            + f64::from(plane[1]) * p.y
            + f64::from(plane[2]) * p.z
            + f64::from(plane[3])
    }

    fn subject(height: f64) -> Bounds {
        Bounds {
            min: Vec3::new(-2.0, 0.0, -2.0),
            max: Vec3::new(2.0, height, 2.0),
        }
    }

    #[test]
    fn the_frustum_holds_what_the_camera_frames_and_nothing_behind_it() {
        let bounds = subject(8.0);
        let camera = hero_pose(bounds, 1.6, GROUND_REACH);
        let planes = planes(&camera.view_projection(1.6));
        let centre = (bounds.min + bounds.max) * 0.5;
        for plane in &planes {
            assert!(
                distance(plane, centre) > 0.0,
                "the subject the pose was solved for is outside {plane:?}"
            );
        }

        // Behind the eye, and far out to the side: each is outside at least
        // one plane, which is what puts a leaf in the unseen bucket.
        let back = camera.position + (camera.position - camera.target);
        let aside = centre
            + (camera.position - camera.target)
                .cross(Vec3::Y)
                .normalized()
                * 500.0;
        for outside in [back, aside] {
            assert!(
                planes.iter().any(|plane| distance(plane, outside) < 0.0),
                "{outside:?} is inside every plane of the frustum"
            );
        }
    }

    #[test]
    fn the_planes_measure_in_metres() {
        let camera = hero_pose(subject(4.0), 1.0, GROUND_REACH);
        // The near plane's own distance is what a normalized plane promises: a
        // point one metre in front of the eye is one metre inside it.
        let forward = (camera.target - camera.position).normalized();
        let near = planes(&camera.view_projection(1.0))[4];
        let at = camera.position + forward * (camera.near + 1.0);
        assert!(
            (distance(&near, at) - 1.0).abs() < 1e-3,
            "a metre past the near plane measured {}",
            distance(&near, at)
        );
    }

    #[test]
    fn a_metre_spans_the_frame_it_fills() {
        // A subject exactly as tall as the frame at one metre of depth spans
        // every pixel of it, and one twice as far spans half.
        let pixels = pixels_per_metre(1_000.0, 90.0);
        assert!((pixels - 500.0).abs() < 1e-9, "{pixels}");
        assert!((pixels / 2.0 - 250.0).abs() < 1e-9);
    }

    #[test]
    fn a_leaf_stands_in_a_sphere_that_holds_all_of_it() {
        let element = build_element(ElementParams::default()).expect("the core built a leaf");
        let bounding = sphere(&element);
        let centre = Vec3::new(
            f64::from(bounding[0]),
            f64::from(bounding[1]),
            f64::from(bounding[2]),
        );
        assert!(bounding[3] > 0.0, "a leaf with no radius");
        for p in &element.positions {
            assert!(
                p.distance(centre) <= f64::from(bounding[3]) * (1.0 + 1e-6),
                "{p:?} is outside the element's own sphere"
            );
        }
        assert_eq!(
            sphere(&Element::default()),
            [0.0; 4],
            "nothing bounds itself"
        );
    }
}
