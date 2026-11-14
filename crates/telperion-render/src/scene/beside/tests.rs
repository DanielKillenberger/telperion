use super::*;
use crate::scene::person::{person, HEIGHT};

fn bounds(min: [f64; 3], max: [f64; 3]) -> Bounds {
    Bounds {
        min: Vec3::new(min[0], min[1], min[2]),
        max: Vec3::new(max[0], max[1], max[2]),
    }
}

/// A shrub a few metres tall, and Telperion at 80 (seed 7), the widest
/// crown the catalogue grows.
fn subjects() -> [(&'static str, Bounds); 2] {
    [
        ("shrub", bounds([-1.2, 0.0, -1.0], [1.1, 2.5, 1.3])),
        (
            "telperion",
            bounds([-25.64, -0.18, -23.66], [30.64, 46.28, 27.53]),
        ),
    ]
}

fn towards(azimuth: f64, elevation: f64) -> Vec3 {
    let (azimuth, elevation) = (azimuth.to_radians(), elevation.to_radians());
    Vec3::new(
        elevation.cos() * azimuth.sin(),
        elevation.sin(),
        elevation.cos() * azimuth.cos(),
    )
}

/// A point in the frame of the crown the bounds inscribe, where the
/// crown is the unit ball.
fn scaled(bounds: Bounds, p: Vec3) -> Vec3 {
    let (centre, half) = (
        (bounds.min + bounds.max) * 0.5,
        (bounds.max - bounds.min) * 0.5,
    );
    let d = p - centre;
    Vec3::new(d.x / half.x, d.y / half.y, d.z / half.z)
}

/// Whether the ray from `from` towards the sun passes through the crown.
fn shaded(bounds: Bounds, from: Vec3, to_sun: Vec3) -> bool {
    let (o, half) = (scaled(bounds, from), (bounds.max - bounds.min) * 0.5);
    let d = Vec3::new(to_sun.x / half.x, to_sun.y / half.y, to_sun.z / half.z);
    let (a, b, c) = (d.dot(d), 2.0 * o.dot(d), o.dot(o) - 1.0);
    let root = b * b - 4.0 * a * c;
    root >= 0.0 && (-b + root.sqrt()) / (2.0 * a) > 0.0
}

/// From every view round the subject, under the harness's sun and the
/// film's: the foot at or behind the trunk's depth, square to the view,
/// the crown's reach and the margin out, and no point of the person
/// under the crown or in its shadow (fn-310 D7).
#[test]
fn the_person_stands_beside_the_subject_outside_the_crown_and_its_shadow() {
    let suns = [(135.0, 30.0), (115.0, 60.0)];
    for (name, tree) in subjects() {
        for ((sun, elevation), view) in suns
            .iter()
            .flat_map(|&s| (0..72).map(move |k| (s, 5.0 * f64::from(k))))
        {
            let (placement, _) = stand(tree, view, sun);
            let foot = placement.foot;
            let (x, z) = view.to_radians().sin_cos();
            let depth = -(foot.x * x + foot.z * z);
            assert!(depth.abs() < 1e-9, "{name} from {view}: depth {depth}");
            assert_eq!(placement.facing, view.to_radians());
            let toward = towards(sun, elevation);
            assert!(lit(tree, placement, (sun, elevation)), "{name} from {view}");
            let (body, _) = person(foot, placement.facing, [0.0; 4], 0);
            for vertex in &body {
                let [x, y, z] = vertex.position.map(f64::from);
                let footprint = scaled(tree, Vec3::new(x, (tree.min.y + tree.max.y) * 0.5, z));
                assert!(
                    footprint.length() > 1.0,
                    "{name} from {view}: the person is under the crown"
                );
                assert!(
                    !shaded(tree, Vec3::new(x, y, z), toward),
                    "{name} from {view}: in the crown's shadow under the sun at {sun}"
                );
            }
        }
        // The rule can fail: the trunk's foot is in the crown's shadow.
        assert!(shaded(tree, Vec3::ZERO, towards(135.0, 30.0)));
        assert!(!lit(tree, Placement::at(0.0, 0.0, 0.0), (135.0, 30.0)));
    }
}

/// The side is the sun's, and is decided the same way every time: square
/// to the view towards the sun, and `view + 90` with the sun on the axis.
#[test]
fn the_side_is_the_suns_and_a_sun_on_the_axis_takes_the_turning_side() {
    for (view, sun, expected) in [
        (115.0, 135.0, 205.0),
        (115.0, 95.0, 25.0),
        (20.0, 115.0, 110.0),
        (160.0, 115.0, 70.0),
        (115.0, 115.0, 205.0),
        (115.0, 295.0, 205.0),
    ] {
        assert_eq!(bearing(view, sun), expected, "{view}, {sun}");
    }
}

/// Where the crown's distance holds the whole person in the hero frame
/// it is whole; where it cuts the person, every distance further out
/// cuts it too (out along its bearing it keeps its depth and only nears
/// the frame's edge). Across subjects 0.25 to 20 m tall and 0.1 to 3
/// times as wide, every subject shorter than the person cuts it and
/// every one over 4 m holds it whole (fn-310 D6, D7).
#[test]
fn the_person_is_whole_at_the_crowns_distance_or_nowhere_out_from_it() {
    let shapes = (1..=80).flat_map(|k| [0.05, 0.15, 0.3, 0.6, 1.0, 1.5].map(|r| (k, r)));
    let view = Shot::default().azimuth;
    for (k, ratio) in shapes {
        let height = 0.25 * f64::from(k);
        let w = ratio * height;
        let tree = bounds([-w, 0.0, -w], [w, height, w]);
        let view_projection = hero_pose(tree, ASPECT, GROUND_REACH).view_projection(ASPECT);
        let (placement, is_whole) = hero(tree, 135.0);
        assert_eq!(placement, at(tree, view, 135.0, 0.0));
        let cut = (0..400).all(|step| {
            !whole(
                &view_projection,
                at(tree, view, 135.0, 0.01 * f64::from(step)),
            )
        });
        assert_eq!(is_whole, !cut, "{height} m, {ratio}");
        if height < HEIGHT {
            assert!(!is_whole, "a {height} m subject holds the whole person");
        }
        if height > 4.0 {
            assert!(
                is_whole,
                "a {height} m subject, {ratio} wide, cuts the person"
            );
        }
    }
}
