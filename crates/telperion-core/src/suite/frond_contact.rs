//! fn-301 R2: a frond rests on the ground. The frond curve
//! (`leaflet::rachis`) meets the ground where its bark would pass below
//! it and lies along it from there, keeping its length; a frond that
//! never reaches the ground is the parabola, bit for bit.
use telperion_core::{
    foliage::{self, CanopyParams},
    math::Vec3,
    Error,
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
        .unwrap()
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
    assert_eq!(f.at(0.0).0, f.point, "the base stays on the stem");
    for t in shares(100).skip(1) {
        let (at, run) = f.at(t);
        assert_eq!(at.y, f.radius(t), "at {t}");
        assert_eq!(run, level(f.heading).normalized(), "at {t}");
    }
}

/// A resting frond is as long as its parabola: drawn at the finest wood,
/// its centreline's length matches the closed-form arc to 1e-9.
#[test]
fn a_resting_frond_keeps_its_length() {
    let Frond { point, heading, .. } = descending();
    let f = Frond::new(point, heading, (7.0, -0.6), 0.0);
    let c = f.first_contact().unwrap();
    let mut t: Vec<f64> = shares(200_000).collect();
    t.push(c);
    t.sort_by(f64::total_cmp);
    let drawn: f64 = t
        .windows(2)
        .map(|w| f.at(w[0]).0.distance(f.at(w[1]).0))
        .sum();
    // The parabola's speed is length·√(1 + (2·arch·t)²).
    let (l, a) = (7.0, -0.6f64);
    let x = 2.0 * a;
    let arc = l / (4.0 * a) * (x * (1.0 + x * x).sqrt() + x.asinh());
    assert!((drawn - arc).abs() < 1e-9, "drawn {drawn}, arc {arc}");
}

#[test]
fn a_frond_straight_down_on_a_vertical_heading_is_refused() {
    let f = Frond::new(
        Vec3::new(0.0, 2.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        (5.0, 0.0),
        0.1,
    );
    let refused = foliage::rachis(f.point, f.heading, f.lift(), &f.p, f.girth, 1.0);
    match refused {
        Err(Error::InvalidInput(m)) => assert!(m.contains("straight down"), "{m}"),
        other => panic!("not refused: {other:?}"),
    }
    // The control: leaning, it lies the way it leans.
    let leaning = Frond::new(f.point, Vec3::new(0.1, -1.0, 0.0), (5.0, 0.0), 0.1);
    assert!(leaning.at(1.0).1.x > 0.0);
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
