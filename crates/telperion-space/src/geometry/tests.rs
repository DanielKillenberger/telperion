//! The tropism's turn near straight down: a direction swept through the
//! vertical turns by degree, never flipping from one side to the other.
use super::lay::toward_elevation;
use super::q::{amount, q3, v3};
use crate::structure::Vec3;
use crate::transcendental::Transcendental;

#[test]
fn a_direction_swept_through_straight_down_turns_by_degree() {
    // The axis's carried side is level and square to the sweep, as a
    // lateral's frame is.
    let side = Vec3::new(1.0, 0.0, 0.0);
    let turned = |angle: f64| {
        let direction = Vec3::new(0.0, angle.sin_fixed(), -angle.cos_fixed());
        v3(toward_elevation(q3(direction), q3(side), amount(0.4), 0.3))
    };
    let steps = 2_000;
    let span = 0.2;
    let step = 2.0 * span / f64::from(steps);
    let mut worst = 0.0f64;
    for i in 0..steps {
        let a = -span + step * f64::from(i);
        worst = worst.max((turned(a + step) - turned(a)).length());
    }
    // Continuous: a step of 2e-4 rad moves the turn by a small amount (a
    // flip moves it by about the turn's length, near 1).
    assert!(worst < 0.01, "one step moves the turn by {worst}");
}

/// The layout's f32 primitives (fn-254, after CONTRACT-A.md 2.3 and 2.4)
/// against their f64 forms on random unit vectors and angles: each within
/// a few units of f32's last place.
#[test]
fn the_layouts_primitives_are_their_reals_to_f32() {
    use super::q::*;
    fn rot(v: Vec3, p: Vec3, a: f64) -> Vec3 {
        let (s, c) = a.sin_cos();
        v * c + p.cross(v) * s + p * (p.dot(v) * (1.0 - c))
    }
    let mut seed = 7u64;
    let mut r = || {
        seed = seed
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((seed >> 11) as f64 / (1u64 << 53) as f64) * 2.0 - 1.0
    };
    let mut worst = [0f64; 4];
    for _ in 0..20000 {
        let v = Vec3::new(r(), r(), r()).unit().unwrap();
        let p = Vec3::new(r(), r(), r()).unit().unwrap();
        let a = r() * 3.0;
        let want = rot(v, p, a);
        let got = v3(rotated(q3(v), q3(p), amount(a)));
        worst[0] = worst[0].max((got - want).length());
        let u = unit(q3(v)).unwrap();
        worst[1] = worst[1].max((v3(u) - v).length());
        let d = f64::from(dot(q3(v), q3(p)));
        worst[2] = worst[2].max((d - v.dot(p)).abs());
        let c = v3(cross(q3(v), q3(p)));
        worst[3] = worst[3].max((c - v.cross(p)).length());
    }
    println!("rotated, unit, dot, cross: {worst:?}");
    assert!(
        worst.iter().all(|&w| w < F32),
        "rotated, unit, dot, cross: {worst:?}"
    );
    // turned and toward_elevation against today's f64 forms.
    let up = Vec3::new(0.0, 0.0, 1.0);
    let turned_f = |(along, side): (Vec3, Vec3), az: f64, angle: f64, plane: f64, epi: f64| {
        let mut toward = side * az.cos() + along.cross(side) * az.sin();
        let level = up - along * up.dot(along);
        if let Some(upper) = level.unit() {
            let facing = toward.dot(upper);
            let turn = along.dot(toward.cross(upper)).atan2(facing);
            let sided = (level.length() / 0.2).min(1.0);
            toward = rot(toward, along, epi * sided * turn * (1.0 + facing) / 2.0);
        }
        let heading = along * angle.cos() + toward * angle.sin();
        let side = toward * angle.cos() - along * angle.sin();
        (
            heading,
            side * plane.cos() + heading.cross(side) * plane.sin(),
        )
    };
    let toward_f = |d: Vec3, side: Vec3, el: f64, share: f64| {
        let level = |v: Vec3| Vec3::new(v.x, v.y, 0.0).unit();
        let lean = Vec3::new(d.x, d.y, 0.0).length();
        let cone = 0.05f64.sin();
        let share = if d.z < 0.0 && lean < cone {
            share * lean / cone
        } else {
            share
        };
        let out = level(d).or_else(|| level(side)).unwrap();
        let now = d.z.clamp(-1.0, 1.0).asin();
        let to = now + (el - now) * share;
        out * to.cos() + up * to.sin()
    };
    let (mut wt, mut we) = (0f64, 0f64);
    for _ in 0..20000 {
        let along = Vec3::new(r(), r(), r()).unit().unwrap();
        let s0 = Vec3::new(r(), r(), r());
        let side = (s0 - along * s0.dot(along)).unit().unwrap();
        let (az, an, pl, ep) = (r() * 3.0, r(), r(), r().abs());
        let (h, s) = turned_f((along, side), az, an, pl, ep);
        let (hq, sq) = super::turned(
            (q3(along), q3(side)),
            amount(az),
            amount(an),
            amount(pl),
            ep as f32,
        );
        wt = wt.max((v3(hq) - h).length()).max((v3(sq) - s).length());
        let (el, sh) = (r(), r().abs());
        let e = toward_f(along, side, el, sh);
        let eq = v3(super::lay::toward_elevation(
            q3(along),
            q3(side),
            amount(el),
            sh as f32,
        ));
        we = we.max((eq - e).length());
    }
    println!("turned {wt:e}, toward_elevation {we:e}");
    assert!(
        wt < F32 && we < F32,
        "turned {wt:e}, toward_elevation {we:e}"
    );
}

/// What the f32 primitives may differ from their f64 forms by over unit
/// vectors and angles of a few radians: a few units in f32's last place.
const F32: f64 = 1e-5;

/// The scales whose last span's share at their summed tip divides to
/// 0.9999994 in f32: (0.644714 − 0.6) / 0.044714 (fn-303, oak to palm
/// frame 36's relay at its parent's tip). The device's `span_share` is
/// held to the same values (`settle_tests.rs`).
const SPANS: [f32; 3] = [0.3, 0.3, 0.044714];

/// fn-303: a length at a span's summed end is the span's end, share 1,
/// by the comparison that chose the span; a length short of it keeps its
/// share.
#[test]
fn a_length_at_a_span_end_is_the_whole_span() {
    use super::stand::{node_x, span_share, total};
    let mut axis = crate::structure::Axis {
        lineage: 0,
        pa: 0,
        birth: 0,
        origin: crate::structure::Origin::Seed,
        vigour: 1.0,
        blend: 1.0,
        apex_end: None,
        base: Vec3::default(),
        heading: Vec3::default(),
        side: Vec3::default(),
        phytomers: Vec::new(),
        units: Vec::new(),
        alive: 1.0,
        rank: 0.0,
        sleep: 0.0,
        phase: 0.0,
        joined: None,
    };
    for &scale in &SPANS {
        axis.phytomers.push(crate::structure::Phytomer {
            cycle: 1,
            tip: Vec3::default(),
            heading: Vec3::default(),
            side: Vec3::default(),
            radius: 0.0,
            scale: f64::from(scale),
            key: 0,
            rank: 0.0,
            size: 1.0,
            light: 1.0,
            grown: 1.0,
        });
    }
    let tip = total(&axis);
    let before = SPANS[0] + SPANS[1];
    // The division alone reads the tip short of its span's end.
    assert!(
        (tip - before) / SPANS[2] < 1.0,
        "the spans must round below 1"
    );
    assert_eq!(node_x(&axis, tip), (2, 1.0));
    // An interior node's end is that span's whole.
    assert_eq!(node_x(&axis, before), (1, 1.0));
    // One ulp short of the tip keeps the division's share.
    let short = f32::from_bits(tip.to_bits() - 1);
    let (k, t) = node_x(&axis, short);
    assert_eq!(k, 2);
    assert!(t < 1.0, "a length short of the end keeps its share: {t}");
    assert_eq!(t, span_share(short, before, SPANS[2]));
    // Past the tip, no span reaches it.
    assert_eq!(node_x(&axis, f32::from_bits(tip.to_bits() + 1)), (3, 0.0));
}
