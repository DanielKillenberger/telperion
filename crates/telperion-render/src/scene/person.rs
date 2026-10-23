//! The scale figure: a standing person 1.75 m tall, calm and low in detail
//! like an architect's scale figure. Every part is a stack of elliptical rings
//! lofted bottom to top and closed at both ends; the legs rise into the torso,
//! which is as wide at the hips as the legs are together, so the body has no
//! step or band where they meet. Nothing is copied from a mesh.
//!
//! Where it stands beside a subject is `beside`'s.
use telperion_core::math::Vec3;

use super::room::Vertex;

/// The person's height, from the soles to the crown of the head.
pub const HEIGHT: f64 = 1.75;
/// Sides on every ring: few enough to stay calm, enough to read round.
const SIDES: u32 = 10;

/// One ring: its centre (across, up, forward), its half width across the
/// shoulders and its half depth front to back, in metres.
type Ring = ([f64; 3], f64, f64);

/// A leg at `x`: shoe, ankle, calf, knee and thigh, up into the torso.
fn leg(x: f64) -> Vec<Ring> {
    vec![
        ([x, 0.0, 0.03], 0.0, 0.0),
        ([x, 0.0, 0.03], 0.05, 0.11),
        ([x, 0.08, 0.0], 0.045, 0.05),
        ([x, 0.42, 0.0], 0.06, 0.065),
        ([x, 0.52, 0.0], 0.055, 0.058),
        ([x, 0.75, 0.0], 0.085, 0.088),
        ([x, 0.84, 0.0], 0.09, 0.095),
        ([x, 0.98, 0.0], 0.06, 0.07),
        ([x, 1.0, 0.0], 0.0, 0.0),
    ]
}

/// An arm hanging at the side `side` (minus one left, one right): hand,
/// wrist, forearm, elbow, upper arm and shoulder.
fn arm(side: f64) -> Vec<Ring> {
    vec![
        ([side * 0.235, 0.70, 0.0], 0.0, 0.0),
        ([side * 0.235, 0.74, 0.0], 0.03, 0.045),
        ([side * 0.233, 0.82, 0.0], 0.032, 0.045),
        ([side * 0.23, 0.86, 0.0], 0.028, 0.035),
        ([side * 0.227, 1.02, 0.0], 0.038, 0.042),
        ([side * 0.225, 1.10, 0.0], 0.04, 0.045),
        ([side * 0.218, 1.28, 0.0], 0.048, 0.052),
        ([side * 0.21, 1.40, 0.0], 0.055, 0.06),
        ([side * 0.19, 1.47, 0.0], 0.0, 0.0),
    ]
}

/// Hips as wide as the two legs, a slight taper to the waist, the chest, the
/// shoulders and the neck. The torso closes inside the tops of the legs, so
/// its underside never shows as a crease at the hips.
fn torso() -> Vec<Ring> {
    vec![
        ([0.0, 0.76, 0.0], 0.0, 0.0),
        ([0.0, 0.80, 0.0], 0.16, 0.09),
        ([0.0, 0.92, 0.0], 0.18, 0.11),
        ([0.0, 1.05, 0.0], 0.165, 0.10),
        ([0.0, 1.25, 0.0], 0.18, 0.11),
        ([0.0, 1.40, 0.0], 0.205, 0.11),
        ([0.0, 1.46, 0.0], 0.17, 0.095),
        ([0.0, 1.50, 0.0], 0.07, 0.06),
        ([0.0, 1.56, 0.0], 0.055, 0.055),
        ([0.0, 1.58, 0.0], 0.0, 0.0),
    ]
}

/// A head without a face: an upright ellipsoid whose crown is the person's
/// height, at even latitudes.
fn head() -> Vec<Ring> {
    let (centre, rise) = (HEIGHT - 0.105, 0.105);
    (0..=8)
        .map(|k| {
            let latitude = std::f64::consts::PI * (f64::from(k) / 8.0 - 0.5);
            let (up, spread) = latitude.sin_cos();
            (
                [0.0, centre + rise * up, 0.01],
                0.075 * spread,
                0.095 * spread,
            )
        })
        .collect()
}

fn parts() -> [Vec<Ring>; 6] {
    [leg(-0.09), leg(0.09), arm(-1.0), arm(1.0), torso(), head()]
}

/// How many vertices and triangles the person has.
pub fn counts() -> (u32, u32) {
    let rings: Vec<u32> = parts().iter().map(|part| part.len() as u32).collect();
    let vertices = rings.iter().sum::<u32>() * (SIDES + 1);
    let triangles = rings.iter().map(|r| (r - 1) * SIDES * 2).sum();
    (vertices, triangles)
}

/// The person standing at `foot` and facing the azimuth `yaw` (radians,
/// clockwise from +Z seen from above): its vertices, and its indices counted
/// from `base`. Normals are the area-weighted mean of the faces round each
/// vertex, so the facets soften with no modelled normal.
pub fn person(foot: Vec3, yaw: f64, colour: [f32; 4], base: u32) -> (Vec<Vertex>, Vec<u32>) {
    let (sin, cos) = yaw.sin_cos();
    let stride = SIDES + 1;
    let mut points = Vec::new();
    let mut indices = Vec::new();
    for part in parts() {
        let first = points.len() as u32;
        for &([x, y, z], across, deep) in &part {
            for side in 0..=SIDES {
                let angle = std::f64::consts::TAU * f64::from(side) / f64::from(SIDES);
                let (px, pz) = (x + across * angle.cos(), z + deep * angle.sin());
                points.push(Vec3::new(px * cos + pz * sin, y, pz * cos - px * sin));
            }
        }
        for ring in 0..part.len() as u32 - 1 {
            for side in 0..SIDES {
                let a = first + ring * stride + side;
                let b = a + stride;
                indices.extend([a, b, a + 1, a + 1, b, b + 1]);
            }
        }
    }
    let normals = normals(&points, &indices);
    let vertices = points
        .iter()
        .zip(normals)
        .map(|(p, n)| Vertex {
            position: [
                (foot.x + p.x) as f32,
                (foot.y + p.y) as f32,
                (foot.z + p.z) as f32,
            ],
            normal: [n.x as f32, n.y as f32, n.z as f32],
            colour,
        })
        .collect();
    (vertices, indices.into_iter().map(|i| i + base).collect())
}

/// Smooth normals: each face's area-weighted normal summed at its corners,
/// then summed again across the copies of one point every ring and pole
/// holds - a ring's seam and a closed end - so neither shows.
fn normals(points: &[Vec3], indices: &[u32]) -> Vec<Vec3> {
    let mut sums = vec![Vec3::ZERO; points.len()];
    for face in indices.as_chunks::<3>().0 {
        let [a, b, c] = face.map(|i| i as usize);
        let n = (points[b] - points[a]).cross(points[c] - points[a]);
        for i in [a, b, c] {
            sums[i] += n;
        }
    }
    let stride = (SIDES + 1) as usize;
    for ring in (0..points.len()).step_by(stride) {
        let copies = &mut sums[ring..ring + stride];
        let pole = points[ring..ring + stride]
            .iter()
            .all(|p| (*p - points[ring]).length() < 1e-9);
        if pole {
            let sum = copies.iter().fold(Vec3::ZERO, |sum, n| sum + *n);
            copies.fill(sum);
        } else {
            let seam = copies[0] + copies[stride - 1];
            copies[0] = seam;
            copies[stride - 1] = seam;
        }
    }
    sums.into_iter().map(|n| n.normalized()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scene::{
        room::{linear, FIGURE, FIGURE_HEIGHT},
        SceneRow,
    };

    fn towards(azimuth: f64, elevation: f64) -> Vec3 {
        let (azimuth, elevation) = (azimuth.to_radians(), elevation.to_radians());
        Vec3::new(
            elevation.cos() * azimuth.sin(),
            elevation.sin(),
            elevation.cos() * azimuth.cos(),
        )
    }

    #[test]
    fn the_person_stands_on_the_ground_at_the_figure_height_within_budget() {
        let (vertices, indices) = person(Vec3::new(3.0, 0.0, 1.5), 0.4, [0.0; 4], 7);
        let ys = vertices.iter().map(|v| f64::from(v.position[1]));
        let low = ys.clone().fold(f64::MAX, f64::min);
        let high = ys.fold(f64::MIN, f64::max);
        assert!(low.abs() < 1e-6, "the person floats or sinks: {low}");
        assert!((high - 1.75).abs() < 1e-5, "the person is {high} m tall");
        assert!(
            (FIGURE_HEIGHT - high).abs() < 1e-5,
            "FIGURE_HEIGHT is not the person's"
        );
        let (count, triangles) = counts();
        assert_eq!(
            (vertices.len(), indices.len()),
            (count as usize, 3 * triangles as usize)
        );
        assert!(
            count <= 800 && triangles <= 1200,
            "{count} vertices, {triangles} triangles"
        );
        assert!(indices.iter().all(|&i| (7..7 + count).contains(&i)));
        assert!(vertices.iter().all(|v| {
            let n = v.normal;
            ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt() - 1.0).abs() < 1e-4
        }));
    }

    #[test]
    fn the_person_reads_cool_in_sun_and_in_shade() {
        let row = SceneRow::default();
        let albedo = linear(FIGURE).map(f64::from);
        let sun = [row.sun_red, row.sun_green, row.sun_blue];
        let sky = [
            0.5 * (row.sky_zenith_red + row.sky_horizon_red),
            0.5 * (row.sky_zenith_green + row.sky_horizon_green),
            0.5 * (row.sky_zenith_blue + row.sky_horizon_blue),
        ];
        let ground = [row.ground_red, row.ground_green, row.ground_blue];
        let to_sun = towards(row.sun_azimuth, row.sun_elevation);
        let away = Vec3::new(-to_sun.x, 0.0, -to_sun.z).normalized();
        for (name, n) in [("lit", to_sun), ("shade", away)] {
            let up = 0.5 + 0.5 * n.y;
            let key = n.dot(to_sun).max(0.0);
            let seen: Vec<f64> = (0..3)
                .map(|i| {
                    let ambient = ground[i] * sky[i] * (1.0 - up) + sky[i] * up;
                    albedo[i] * (ambient + sun[i] * key)
                })
                .collect();
            assert!(
                seen[2] >= seen[0],
                "{name}: the person reads warm: {seen:?}"
            );
        }
    }
}
