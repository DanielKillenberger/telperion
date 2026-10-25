//! fn-301 R2: a frond rests on the ground. The frond curve
//! (`leaflet::rachis`) meets the ground where its bark would pass below
//! it and lies along it from there, keeping its length; a frond that
//! never reaches the ground is the parabola, bit for bit.
use telperion_core::{
    foliage::{self, CanopyParams},
    math::Vec3,
};

/// The thinnest wood the engine draws (`space.rs`).
const FINEST: f64 = 2e-4;

/// A frond `length` long, arched by `arch`.
fn table(length: f64, arch: f64) -> CanopyParams {
    CanopyParams {
        rachis_length: length,
        rachis_arch: arch,
        ..CanopyParams::default()
    }
}

/// A frond leaving `point` on `heading`, its girth and its rows.
struct Frond {
    point: Vec3,
    heading: Vec3,
    p: CanopyParams,
    girth: f64,
}

impl Frond {
    fn new(point: Vec3, heading: Vec3, (length, arch): (f64, f64), girth: f64) -> Self {
        let (heading, p) = (heading.normalized(), table(length, arch));
        Self {
            point,
            heading,
            p,
            girth,
        }
    }
    fn lift(&self) -> Vec3 {
        foliage::frame(self.heading).0
    }
    /// The curve's point and direction at `t`.
    fn at(&self, t: f64) -> (Vec3, Vec3) {
        foliage::rachis(
            self.point,
            self.heading,
            self.lift(),
            &self.p,
            self.girth,
            t,
        )
    }
    /// The parabola as it was drawn before contact (fn-283).
    fn parabola(&self, t: f64) -> Vec3 {
        let (l, a) = (self.p.rachis_length, self.p.rachis_arch);
        self.point + self.heading * (l * t) + self.lift() * (a * l * t * t)
    }
    fn radius(&self, t: f64) -> f64 {
        (self.girth * (1.0 - t)).max(FINEST)
    }
    /// The first share at which the parabola comes down to its radius, by
    /// bisection on a grid, independent of the curve's closed form.
    fn first_contact(&self) -> Option<f64> {
        let below = |t: f64| self.parabola(t).y <= self.radius(t);
        let n = 100_000;
        let k = (0..=n).find(|&k| below(k as f64 / n as f64))?;
        if k == 0 {
            return Some(0.0);
        }
        let (mut lo, mut hi) = ((k - 1) as f64 / n as f64, k as f64 / n as f64);
        for _ in 0..80 {
            let mid = 0.5 * (lo + hi);
            match below(mid) {
                true => hi = mid,
                false => lo = mid,
            }
        }
        Some(hi)
    }
}

fn shares(n: usize) -> impl Iterator<Item = f64> {
    (0..=n).map(move |k| k as f64 / n as f64)
}

fn level(v: Vec3) -> Vec3 {
    Vec3::new(v.x, 0.0, v.z)
}

/// The young palm's frond: leaving a stem 3 m up 27.5 degrees below
/// horizontal, 7 m long, arched down by 0.6, and 0.125 m thick.
fn descending() -> Frond {
    let (sin, cos) = 27.5f64.to_radians().sin_cos();
    Frond::new(
        Vec3::new(0.0, 3.0, 0.0),
        Vec3::new(cos, -sin, 0.0),
        (7.0, -0.6),
        0.125,
    )
}

#[test]
fn a_descending_frond_lies_at_its_radius_past_its_first_contact() {
    let f = descending();
    let c = f
        .first_contact()
        .expect("the palm's frond reaches the ground");
    assert!(c > 0.0 && c < 1.0, "contact at {c}");
    let toward = level(f.heading + f.lift() * (2.0 * f.p.rachis_arch * c)).normalized();
    for t in shares(1000) {
        let (at, run) = f.at(t);
        assert!(
            at.y >= f.radius(t) - 1e-9,
            "at {t}: {} under {}",
            at.y,
            f.radius(t)
        );
        if t + 1e-9 < c {
            assert_eq!(at, f.parabola(t), "before contact at {t}");
        } else if t > c + 1e-9 {
            assert_eq!(at.y, f.radius(t), "flat at its radius at {t}");
            assert!(
                run.distance(toward) < 1e-9,
                "at {t}: runs {run:?}, not {toward:?}"
            );
        }
    }
}

/// An arch that bends up again: the first root wins, and the frond stays
/// down past the second, where the parabola would rise off the ground.
#[test]
fn an_upward_arch_rests_from_its_first_root() {
    let f = Frond::new(
        Vec3::new(0.0, 0.5, 0.0),
        Vec3::new(1.0, -0.6, 0.0),
        (5.0, 0.8),
        0.1,
    );
    let c = f.first_contact().expect("the frond reaches the ground");
    assert!(f.parabola(1.0).y > 1.0, "the parabola rises again");
    assert!(c < 0.375, "the first root: {c}");
    let (end, _) = f.at(1.0);
    assert_eq!(end.y, FINEST, "the tip stays down");
}

#[test]
fn a_base_below_its_radius_rests_from_its_base() {
    let f = Frond::new(
        Vec3::new(0.0, 0.05, 0.0),
        Vec3::new(1.0, 0.2, 0.0),
        (5.0, -0.2),
        0.1,
    );
    assert_eq!(f.first_contact(), Some(0.0));
    let base = Vec3::new(f.point.x, f.radius(0.0), f.point.z);
    assert_eq!(f.at(0.0).0, base, "the base rests at its radius");
    assert!(f.at(1e-9).0.distance(base) < 1e-8, "and runs on from it");
    for t in shares(100) {
        let (at, run) = f.at(t);
        assert_eq!(at.y, f.radius(t), "at {t}");
        assert_eq!(run, level(f.heading).normalized(), "at {t}");
    }
}

/// A resting frond is as long as its parabola: its centreline, falling as
/// its radius tapers and then lying at the finest wood, matches the
/// closed-form arc to 1e-9, arched or straight.
#[test]
fn a_resting_frond_keeps_its_length() {
    let straight = Frond::new(
        Vec3::new(0.0, 0.2, 0.0),
        Vec3::new(0.8, -0.6, 0.0),
        (1.0, 0.0),
        0.1,
    );
    // A frond falling faster than it runs at its base, not once it lies.
    let (sin, cos) = 45f64.to_radians().sin_cos();
    let steep = Frond::new(
        Vec3::new(0.0, 1.15, 0.0),
        Vec3::new(cos, -sin, 0.0),
        (1.0, -1.0),
        1.1,
    );
    for f in [descending(), straight, steep] {
        let c = f.first_contact().unwrap();
        // The kinks: where it meets the ground and where its taper ends.
        let mut t: Vec<f64> = shares(200_000).collect();
        t.extend([c, 1.0 - FINEST / f.girth]);
        t.sort_by(f64::total_cmp);
        let drawn: f64 = t
            .windows(2)
            .map(|w| f.at(w[0]).0.distance(f.at(w[1]).0))
            .sum();
        let arc = arc(f.p.rachis_length, f.p.rachis_arch, 1.0);
        assert!((drawn - arc).abs() < 1e-9, "drawn {drawn}, arc {arc}");
    }
}

/// A frond meeting the ground straight down rests along its frame's
/// lift (host, 2026-10-21): +lift with no arch, the arch's way with one
/// (here a base under its radius, met on its vertical heading), as far
/// along the ground as its parabola is long past contact. The device is
/// held to the same cases (`rachis_tests`).
#[test]
fn a_frond_straight_down_rests_along_its_lift() {
    let down = Vec3::new(0.0, -1.0, 0.0);
    // Its base height, its arch, the way it lies along the lift, and
    // where it meets the ground.
    let cases = [
        (2.0, 0.0, 1.0, (2.0 - FINEST) / 5.0),
        (1e-4, -0.5, -1.0, 0.0),
        (1e-4, 0.5, 1.0, 0.0),
    ];
    for (base, arch, way, c) in cases {
        let f = Frond::new(Vec3::new(0.0, base, 0.0), down, (5.0, arch), 0.0);
        let toward = level(f.lift()).normalized() * way;
        let (at, run) = f.at(1.0);
        assert_eq!(run, toward, "arch {arch}");
        assert_eq!(at.y, FINEST, "arch {arch}: on the ground");
        let lies = (at - f.point).dot(toward);
        let expected = arc(5.0, arch, 1.0) - arc(5.0, arch, c);
        assert!(
            (lies - expected).abs() < 1e-12,
            "arch {arch}: {lies} against {expected}"
        );
    }
}

/// The parabola's length to share `t`: its speed is
/// length·√(1 + (2·arch·t)²).
fn arc(l: f64, a: f64, t: f64) -> f64 {
    let x = 2.0 * a * t;
    match a == 0.0 {
        true => l * t,
        false => l / (4.0 * a) * (x * (1.0 + x * x).sqrt() + x.asinh()),
    }
}

/// A frond whose bark never reaches the ground takes today's operations in
/// today's order (R3).
#[test]
fn a_clear_frond_is_the_parabola_bit_for_bit() {
    let Frond { heading, .. } = descending();
    let f = Frond::new(Vec3::new(0.0, 10.0, 0.0), heading, (7.0, -0.6), 0.125);
    assert_eq!(f.first_contact(), None);
    let (l, a, lift) = (f.p.rachis_length, f.p.rachis_arch, f.lift());
    for t in shares(113) {
        let (at, run) = f.at(t);
        assert_eq!(at, f.point + f.heading * (l * t) + lift * (a * l * t * t));
        assert_eq!(run, (f.heading + lift * (2. * a * t)).normalized());
    }
}
