use super::*;

/// Directions over the whole sphere, the folds of the octahedron among them.
fn directions() -> impl Iterator<Item = Vec3> {
    let n = 200_000;
    let golden = std::f64::consts::PI * (3.0 - 5f64.sqrt());
    let spiral = (0..n).map(move |i| {
        let z = 1.0 - 2.0 * (i as f64 + 0.5) / n as f64;
        let r = (1.0 - z * z).sqrt();
        let a = golden * i as f64;
        Vec3::new(r * a.cos(), r * a.sin(), z)
    });
    let folds = (0..=64).flat_map(|k| {
        let t = k as f64 / 64.0;
        let s = 1.0 - t;
        [
            Vec3::new(t, s, 0.0),
            Vec3::new(-t, s, 0.0),
            Vec3::new(t, -s, 1e-9),
            Vec3::new(t, 0.0, -s),
            Vec3::new(0.0, -t, -s),
        ]
    });
    spiral.chain(folds).map(Vec3::normalized)
}

/// A decoded wood normal is within 1e-4 radians of the unit normal it
/// encodes, over the whole sphere and on the octahedron's folds: a
/// hundredth of a degree, far under what a shaded pixel resolves.
#[test]
fn a_decoded_normal_is_within_its_angle_of_the_normal() {
    const BOUND: f64 = 1e-4;
    let worst = directions()
        .map(|v| {
            let back = unnormal(normal(v));
            assert!((back.length() - 1.0).abs() < 1e-12, "{v:?}");
            back.dot(v).clamp(-1.0, 1.0).acos()
        })
        .fold(0.0, f64::max);
    assert!(worst <= BOUND, "worst {worst} rad");
}

/// A decoded bark coord is within half a code of the coord it encodes: the
/// distance along within its range over 131,070, a millimetre on a wood
/// 131 m long, and the angle within a turn over 131,070, under a tenth of a
/// millimetre round a trunk a metre across.
#[test]
fn a_decoded_coord_is_within_half_a_code() {
    let along = 87.3;
    let (d_bound, a_bound) = (along / (2.0 * TOP), TAU / (2.0 * TOP));
    let (mut d_worst, mut a_worst) = (0.0f64, 0.0f64);
    for i in 0..=100_000 {
        let t = i as f64 / 100_000.0;
        let (d, angle) = (t * along, (1.0 - t) * TAU * 0.999_999);
        let [dd, aa] = uncoord(coord(d, angle, along), along);
        d_worst = d_worst.max((dd - d).abs());
        a_worst = a_worst.max((aa - angle).abs());
    }
    assert!(d_worst <= d_bound * (1.0 + 1e-9), "along off {d_worst} m");
    assert!(a_worst <= a_bound * (1.0 + 1e-9), "angle off {a_worst} rad");
}

/// The codes the browser's decoder is tested on (`src/browser/wood.test.ts`)
/// are the ones this encoder writes.
#[test]
fn the_browser_decodes_the_codes_the_core_writes() {
    for (v, code) in [
        ([0.0, 0.0, 1.0], [0, 0]),
        ([0.0, 0.0, -1.0], [32767, 32767]),
        ([1.0, 0.0, 0.0], [32767, 0]),
        ([0.0, -1.0, 0.0], [0, -32767]),
        ([-0.663556801, -0.646286771, 0.376836544], [-12891, -12555]),
        ([-0.712254759, -0.044520411, -0.700507738], [-31766, -16752]),
        ([0.626027246, 0.115370593, 0.771219498], [13561, 2499]),
        ([-0.71698689, 0.566264184, -0.406539879], [-21786, 18864]),
    ] {
        assert_eq!(normal(Vec3::new(v[0], v[1], v[2])), code, "{v:?}");
    }
}
