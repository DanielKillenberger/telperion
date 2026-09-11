//! Fitting the sun's map to what is on stage: where the sun stands from two
//! angles, and the orthographic volume that holds the subject and the shadow it
//! throws across the ground. Nothing here touches a device, so the fit can be
//! read against the corners it must hold without one.
use telperion_core::{math::Vec3, surface::Bounds};

use crate::SceneRow;

/// Metres of slack around the fitted volume, so a caster sitting exactly on a
/// face of it still writes its depth.
const MARGIN: f64 = 1.0;

/// The lowest sun the map is fitted to. A sun on the horizon throws a shadow
/// with no end, and no map holds one; below this the volume is fitted as if the
/// sun stood here, and the far end of the shadow falls outside the map and
/// reads as open ground.
const LOWEST_SUN: f64 = 5.0;

/// Where the sun stands, and the volume its map covers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    /// World space to the sun's clip space, column-major, depth in 0..1.
    pub view_projection: [f32; 16],
    /// The direction from the scene towards the sun, unit length, with a
    /// fourth slot the uniform block's alignment wants.
    pub direction: [f32; 4],
}

/// The direction from the scene towards the sun. Azimuth is degrees clockwise
/// from +Z seen from above, elevation degrees above the horizon, so the default
/// row's midday sun stands high and behind the camera's right shoulder.
fn sun(row: &SceneRow) -> Vec3 {
    towards(row.sun_azimuth, row.sun_elevation)
}

fn towards(azimuth: f64, elevation: f64) -> Vec3 {
    let (azimuth, elevation) = (azimuth.to_radians(), elevation.to_radians());
    Vec3::new(
        elevation.cos() * azimuth.sin(),
        elevation.sin(),
        elevation.cos() * azimuth.cos(),
    )
}

/// The sun's view of these bounds: an orthographic volume holding the subject
/// and the shadow it throws across the ground, and nothing else. Nothing is on
/// stage until something is submitted, so a scene with no bounds is given a
/// metre at the origin and a map of it that nothing draws into.
pub fn light(row: &SceneRow, bounds: Option<Bounds>) -> Light {
    let to_sun = sun(row);
    let bounds = bounds.unwrap_or(Bounds {
        min: Vec3::ZERO,
        max: Vec3::new(1.0, 1.0, 1.0),
    });
    // The map's own axes: the sun looks along -back, exactly as a camera does.
    // A sun straight overhead has no bearing of its own, so the reference the
    // frame is built from steps aside rather than collapsing.
    let back = to_sun;
    let reference = if back.y.abs() > 0.999 {
        Vec3::Z
    } else {
        Vec3::Y
    };
    let right = reference.cross(back).normalized();
    let up = back.cross(right);

    // What the volume must hold: the subject itself, and where the sun throws
    // each of its corners onto the ground.
    let thrown = towards(row.sun_azimuth, row.sun_elevation.max(LOWEST_SUN));
    let mut low = [f64::MAX; 3];
    let mut high = [f64::MIN; 3];
    for corner in corners(bounds) {
        for point in [corner, shadow_of(corner, thrown)] {
            for (axis, extent) in [right, up, back].iter().enumerate() {
                low[axis] = low[axis].min(extent.dot(point));
                high[axis] = high[axis].max(extent.dot(point));
            }
        }
    }

    // Each axis maps its own extent onto the clip range: x and y across the map
    // at -1..1, and depth at 0..1 running away from the sun.
    let across = |axis: usize| {
        let (low, high) = (low[axis] - MARGIN, high[axis] + MARGIN);
        (2.0 / (high - low), -(high + low) / (high - low))
    };
    let (near, far) = (-high[2] - MARGIN, -low[2] + MARGIN);
    let depth = (1.0 / (near - far), near / (near - far));
    let axes = [(right, across(0)), (up, across(1)), (back, depth)];

    let mut view_projection = [0.0f32; 16];
    for (slot, (axis, (scale, offset))) in axes.iter().enumerate() {
        view_projection[slot] = (axis.x * scale) as f32;
        view_projection[4 + slot] = (axis.y * scale) as f32;
        view_projection[8 + slot] = (axis.z * scale) as f32;
        view_projection[12 + slot] = *offset as f32;
    }
    view_projection[15] = 1.0;
    Light {
        view_projection,
        direction: [to_sun.x as f32, to_sun.y as f32, to_sun.z as f32, 0.0],
    }
}

/// Where the sun throws this point onto the ground. A point already at or below
/// the ground throws no shadow past itself.
fn shadow_of(point: Vec3, to_sun: Vec3) -> Vec3 {
    let height = point.y.max(0.0);
    point - to_sun * (height / to_sun.y)
}

fn corners(bounds: Bounds) -> [Vec3; 8] {
    let (a, b) = (bounds.min, bounds.max);
    [
        Vec3::new(a.x, a.y, a.z),
        Vec3::new(b.x, a.y, a.z),
        Vec3::new(a.x, b.y, a.z),
        Vec3::new(b.x, b.y, a.z),
        Vec3::new(a.x, a.y, b.z),
        Vec3::new(b.x, a.y, b.z),
        Vec3::new(a.x, b.y, b.z),
        Vec3::new(b.x, b.y, b.z),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Clip-space x, y and depth of a world point through the sun's matrix.
    fn project(light: &Light, point: Vec3) -> (f64, f64, f64) {
        let v = [point.x as f32, point.y as f32, point.z as f32, 1.0];
        let mut clip = [0.0f32; 4];
        for (row, slot) in clip.iter_mut().enumerate() {
            *slot = (0..4)
                .map(|k| light.view_projection[k * 4 + row] * v[k])
                .sum();
        }
        let w = f64::from(clip[3]);
        (
            f64::from(clip[0]) / w,
            f64::from(clip[1]) / w,
            f64::from(clip[2]) / w,
        )
    }

    fn oak() -> Bounds {
        Bounds {
            min: Vec3::new(-9.4, 0.0, -8.1),
            max: Vec3::new(8.8, 22.6, 9.7),
        }
    }

    fn row(azimuth: f64, elevation: f64) -> SceneRow {
        SceneRow {
            sun_azimuth: azimuth,
            sun_elevation: elevation,
            ..Default::default()
        }
    }

    #[test]
    fn the_sun_stands_where_its_two_angles_put_it() {
        for (azimuth, elevation) in [(0.0, 0.0), (135.0, 55.0), (270.0, 30.0), (40.0, 90.0)] {
            let to_sun = sun(&row(azimuth, elevation));
            assert!(
                (to_sun.length() - 1.0).abs() < 1e-9,
                "{to_sun:?} is not a direction"
            );
            assert!(
                (to_sun.y - elevation.to_radians().sin()).abs() < 1e-9,
                "the sun at {elevation} degrees stands at {}",
                to_sun.y
            );
        }
        // Azimuth turns clockwise seen from above: from +Z towards +X.
        let quarter = sun(&row(90.0, 0.0));
        assert!(quarter.x > 0.999 && quarter.z.abs() < 1e-9, "{quarter:?}");
    }

    #[test]
    fn the_map_holds_the_subject_and_the_shadow_it_throws() {
        for (azimuth, elevation) in [(135.0, 55.0), (0.0, 80.0), (215.0, 12.0), (90.0, 90.0)] {
            let row = row(azimuth, elevation);
            let light = light(&row, Some(oak()));
            let to_sun = sun(&row);
            for corner in corners(oak()) {
                for point in [corner, shadow_of(corner, to_sun)] {
                    let (x, y, depth) = project(&light, point);
                    assert!(
                        x.abs() <= 1.0 && y.abs() <= 1.0,
                        "{point:?} fell outside the map at {azimuth}/{elevation}: {x}, {y}"
                    );
                    assert!(
                        (0.0..=1.0).contains(&depth),
                        "{point:?} fell outside the map's depth at \
                         {azimuth}/{elevation}: {depth}"
                    );
                }
            }
        }
    }

    #[test]
    fn a_lower_sun_is_given_a_longer_map_and_a_sun_on_the_horizon_is_not() {
        // A shadow lengthens away from the sun's bearing, which is the map's
        // own second axis, so the lower the sun the more metres that axis must
        // hold. Each axis of the matrix is a unit direction times its scale, so
        // its length is that scale: two clip units over the metres it covers.
        let metres_along = |elevation: f64| {
            let light = light(&row(135.0, elevation), Some(oak()));
            let scale: f64 = [1, 5, 9]
                .iter()
                .map(|slot| f64::from(light.view_projection[*slot]).powi(2))
                .sum::<f64>()
                .sqrt();
            2.0 / scale
        };
        let (high, low) = (metres_along(80.0), metres_along(20.0));
        assert!(
            low > high,
            "a low sun asked for {low} m, a high one {high} m"
        );
        // A sun on the horizon throws a shadow with no end and no map holds
        // one. The volume is fitted as if the sun stood at the lowest it is
        // fitted for, so it stays finite and still holds the subject casting
        // it; the far end of that shadow falls outside and reads as open.
        let horizon = light(&row(135.0, 0.0), Some(oak()));
        assert!(
            horizon.view_projection.iter().all(|slot| slot.is_finite()),
            "a sun on the horizon fitted a map with no end: {:?}",
            horizon.view_projection
        );
        for corner in corners(oak()) {
            let (x, y, depth) = project(&horizon, corner);
            assert!(
                x.abs() <= 1.0 && y.abs() <= 1.0 && (0.0..=1.0).contains(&depth),
                "{corner:?} left a horizon sun's map: {x}, {y}, {depth}"
            );
        }
    }
}
