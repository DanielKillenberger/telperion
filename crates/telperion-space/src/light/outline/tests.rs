//! The outline's light against closed forms (fn-226 R3): chords through
//! cone sections of known length, in each case the solver meets (the
//! quadratic both ways, the linear, the constant, a tangent, a miss, a
//! clip to the section's floor), and Beer–Lambert's transmission through
//! a stack of them; the volume of a known solid, the shoulder of a known
//! crown, and the bootstrap: no leaf, and a bud reads light 1.
use super::chord::{chord, Section};
use super::fold::Fold;
use super::*;
use crate::structure::Vec3;

fn q3(x: f64, y: f64, z: f64) -> [i64; 3] {
    [q32(x), q32(y), q32(z)]
}

fn real(q: i64) -> f64 {
    q as f64 / ONE32 as f64
}

fn section(z: [f64; 2], r: [f64; 2]) -> Section {
    Section {
        z: z.map(q32),
        r: r.map(q32),
    }
}

/// The chord in the ray's own units, against its closed form.
fn chord_is(s: &Section, p: [i64; 3], d: [i64; 3], want: f64) {
    let got = real(chord(s, p, d));
    assert!((got - want).abs() < 1e-6, "chord {got}, want {want}");
}

/// The constant case: a vertical ray in a cylinder is inside over the
/// whole span or nowhere; a ray from below the section enters at its
/// floor; a section wholly below the bud gives nothing.
#[test]
fn a_vertical_ray_in_a_cylinder_is_the_constant_case() {
    let cylinder = section([2.0, 6.0], [2.0, 2.0]);
    let up = q3(0.0, 0.0, 1.0);
    chord_is(&cylinder, q3(1.0, 0.0, 3.0), up, 3.0);
    chord_is(&cylinder, q3(1.0, 0.0, 0.0), up, 4.0);
    chord_is(&cylinder, q3(3.0, 0.0, 3.0), up, 0.0);
    chord_is(&cylinder, q3(0.0, 0.0, 7.0), up, 0.0);
}

/// The linear case: a ray along a cone's generator (r = z) is inside
/// over all of the section, from where it crosses the cone, or nowhere.
#[test]
fn a_ray_along_a_generator_is_the_linear_case() {
    let cone = section([0.0, 4.0], [0.0, 4.0]);
    chord_is(&cone, q3(1.0, 0.0, 2.0), q3(1.0, 0.0, 1.0), 2.0);
    chord_is(&cone, q3(3.0, 0.0, 2.0), q3(-1.0, 0.0, 1.0), 1.5);
    chord_is(&cone, q3(3.0, 0.0, 2.0), q3(1.0, 0.0, 1.0), 0.0);
    // An inverted cone r = 8 - z: from its axis at 45°, out at z = 4.
    let narrowing = section([0.0, 8.0], [8.0, 0.0]);
    let s = 0.5f64.sqrt();
    chord_is(
        &narrowing,
        q3(0.0, 0.0, 0.0),
        q3(s, 0.0, s),
        8.0 / (2.0 * s),
    );
}

/// The quadratic opening away from the ray: a slant ray from a
/// cylinder's axis leaves through its wall, 2 / cos 30° along it; one
/// that leaves through the top first is clipped there.
#[test]
fn a_slant_ray_crosses_a_cylinder_between_its_roots() {
    let cylinder = section([0.0, 4.0], [2.0, 2.0]);
    let e = 30f64.to_radians();
    let d = q3(e.cos(), 0.0, e.sin());
    chord_is(&cylinder, q3(0.0, 0.0, 1.0), d, 2.0 / e.cos());
    chord_is(&cylinder, q3(0.0, 0.0, 3.5), d, 0.5 / e.sin());
}

/// The quadratic closing on the ray (a cone steeper than it): inside
/// outside its roots, and with no root (a vertical ray on the axis)
/// inside everywhere.
#[test]
fn a_cone_steeper_than_the_ray_holds_it_outside_its_roots() {
    let steep = section([1.0, 3.0], [2.0, 6.0]);
    chord_is(&steep, q3(0.0, 0.0, 1.0), q3(1.0, 0.0, 1.0), 2.0);
    chord_is(&steep, q3(0.0, 0.0, 1.0), q3(0.0, 0.0, 1.0), 2.0);
}

/// A tangent ray and a missing one give no chord.
#[test]
fn a_tangent_or_missing_ray_has_no_chord() {
    let cylinder = section([0.0, 10.0], [2.0, 2.0]);
    chord_is(&cylinder, q3(2.0, -5.0, 0.0), q3(0.0, 1.0, 1.0), 0.0);
    chord_is(&cylinder, q3(5.0, 5.0, 0.0), q3(0.0, 0.0, 1.0), 0.0);
    chord_is(&cylinder, q3(3.0, -9.0, 0.0), q3(0.0, 1.0, 1.0), 0.0);
}

/// An outline whose sections, density and sky are set by hand.
fn stack(sections: [Section; SECTIONS], kd: f64, sky: Vec<([i64; 3], f64)>) -> Outline {
    let mut o = Outline::new([0; 5], 0, &Light::NEUTRAL);
    o.sections = sections;
    o.kd = q32(kd);
    o.sky = sky.into_iter().map(|(d, w)| (d, q32(w))).collect();
    o
}

/// Sixteen sections of a solid `r(z)` over `[0, top]`.
fn sliced(top: f64, r: impl Fn(f64) -> f64) -> [Section; SECTIONS] {
    std::array::from_fn(|k| {
        let z = [k, k + 1].map(|k| top * k as f64 / SECTIONS as f64);
        section(z, z.map(&r))
    })
}

/// Beer–Lambert through a stack of sections: exp(-k D L) along each
/// direction, `L` its chords summed over the stack, weighted by shares.
#[test]
fn transmission_through_a_stack_is_beer_lamberts() {
    let (kd, s) = (0.5 * 0.3, 0.5f64.sqrt());
    let up = q3(0.0, 0.0, 1.0);
    let e = 60f64.to_radians();
    let high = q3(e.cos(), 0.0, e.sin());
    let cylinder = stack(sliced(8.0, |_| 2.0), kd, vec![(up, 0.5), (high, 0.5)]);
    let want = 0.5 * (-kd * 8.0).exp() + 0.5 * (-kd * 2.0 / e.cos()).exp();
    let got = real(cylinder.at(q3(0.0, 0.0, 0.0)));
    assert!((got - want).abs() < 1e-6, "{got} against {want}");
    let cone = stack(sliced(8.0, |z| 8.0 - z), kd, vec![(q3(s, 0.0, s), 1.0)]);
    let want = (-kd * 4.0 / s).exp();
    let got = real(cone.at(q3(0.0, 0.0, 0.0)));
    assert!((got - want).abs() < 1e-6, "{got} against {want}");
}

/// A bud outside the stack, under the ray's whole sky, reads light 1;
/// the shares' sum is clamped to 1.
#[test]
fn transmission_is_clamped_to_one() {
    let up = q3(0.0, 0.0, 1.0);
    let over = stack(sliced(8.0, |_| 2.0), 0.15, vec![(up, 0.75), (up, 0.75)]);
    assert_eq!(over.at(q3(5.0, 0.0, 0.0)), ONE32);
}

/// The bootstrap: an outline with no leaf, or no volume, gives every
/// bud light 1 and means 1.
#[test]
fn no_leaf_or_no_volume_reads_light_one() {
    let light = Light {
        extinction: 0.5,
        sky: 0.5,
    };
    let shape = [0.0, 8.0, 3.0, 0.5, 2.0].map(q32);
    let bare = Outline::new(shape, 0, &light);
    assert_eq!(
        (bare.density, bare.at(q3(0.0, 0.0, 1.0)), bare.mean),
        (0, ONE32, ONE32)
    );
    let flat = Outline::new([4.0, 4.0, 3.0, 0.5, 2.0].map(q32), q32(10.0), &light);
    assert_eq!((flat.volume, flat.density), (0, 0));
    assert_eq!((flat.at(q3(0.0, 0.0, 4.0)), flat.mean), (ONE32, ONE32));
    let leafy = Outline::new(shape, q32(50.0), &light);
    let (inside, mean) = (real(leafy.at(q3(0.0, 0.0, 1.0))), real(leafy.mean));
    assert!(
        inside < 1.0 && mean < 1.0 && mean > inside,
        "{inside} {mean}"
    );
}

/// `quadrant` on contract A: the ends, a straight line at s = 1, a
/// quarter circle at s = 2.
#[test]
fn quadrant_is_the_super_ellipse() {
    let s2 = q32(2.0);
    assert_eq!((quadrant(0, s2), quadrant(ONE32, s2)), (ONE32, 0));
    for (p, s, want) in [(0.25, 1.0, 0.75), (0.6, 2.0, 0.8), (0.8, 2.0, 0.6)] {
        let got = real(quadrant(q32(p), q32(s)));
        assert!((got - want).abs() < 1e-7, "quadrant({p}, {s}) {got}");
    }
}

/// The midpoint rule's volume of an ellipsoid (s = 2, widest halfway):
/// 4/3 π R² (H - B) / 2.
#[test]
fn the_volume_is_the_solids() {
    let o = Outline::new(
        [1.0, 9.0, 3.0, 0.5, 2.0].map(q32),
        q32(1.0),
        &Light::NEUTRAL,
    );
    let want = 4.0 / 3.0 * std::f64::consts::PI * 9.0 * 4.0;
    let got = real(o.volume);
    assert!((got - want).abs() / want < 1e-3, "{got} against {want}");
}

/// The fold of an ellipsoidal crown's surface on a stem: its base and
/// height from the crown, its widest reach and fullness, and the least
/// shoulder that holds it, 2.
#[test]
fn the_fold_of_an_ellipsoid_is_its_shape() {
    let mut fold = Fold::default();
    fold.add(Vec3::new(0.0, 0.0, 0.5), false);
    for i in 1..200 {
        let t = f64::from(i) / 200.0;
        let (z, r) = (2.0 + 8.0 * t, 3.0 * (1.0 - (2.0 * t - 1.0).powi(2)).sqrt());
        let a = f64::from(i) * 0.7;
        fold.add(Vec3::new(r * a.cos(), r * a.sin(), z), true);
    }
    fold.add(Vec3::new(0.0, 0.0, 2.0), true);
    fold.add(Vec3::new(0.0, 0.0, 10.0), true);
    let [base, height, reach, full, s] = fold.shape().map(real);
    assert_eq!((base, height), (2.0, 10.0));
    assert!((reach - 3.0).abs() < 1e-6 && (full - 0.5).abs() < 1e-6);
    assert!((s - 2.0).abs() < 1e-3, "shoulder {s}");
}
