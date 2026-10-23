//! The film's one continuous shot: the eye turns about the trunk at the
//! declared rate, and stands back as far and looks as high as the keys
//! say at that moment, eased between them (a monotone cubic on the log of
//! the distance, so a pull back neither overshoots nor stops at a key).
use super::path::{Camera as Declared, Key};
use telperion_core::math::Vec3;
use telperion_render::{Camera, GROUND_REACH};

/// The eye's bearing from the trunk at `seconds` into the film, in degrees.
pub fn azimuth(c: &Declared, seconds: f64) -> f64 {
    c.azimuth + c.orbit * seconds
}

/// The pose at `seconds` into the film.
pub fn pose(c: &Declared, seconds: f64) -> Camera {
    let distance = monotone(&c.keys, seconds, |k| k.distance.ln()).exp();
    let height = monotone(&c.keys, seconds, |k| k.height);
    let azimuth = azimuth(c, seconds).to_radians();
    let (rise, run) = c.elevation.to_radians().sin_cos();
    let target = Vec3::new(0.0, height, 0.0);
    let back = Vec3::new(run * azimuth.sin(), rise, run * azimuth.cos());
    let mut position = target + back * distance;
    // Never below the ground.
    position.y = position.y.max(0.05);
    Camera {
        position,
        target,
        field_of_view: c.fov,
        near: (distance * 0.01).clamp(0.01, 0.1),
        far: distance + GROUND_REACH * 2.0,
    }
}

/// A key's value at `at` by Fritsch-Carlson monotone cubic interpolation,
/// held flat before the first key and after the last.
fn monotone(keys: &[Key], at: f64, of: impl Fn(&Key) -> f64) -> f64 {
    let (x, y): (Vec<f64>, Vec<f64>) = keys.iter().map(|k| (k.at, of(k))).unzip();
    let n = x.len();
    if n == 1 || at <= x[0] {
        return y[0];
    }
    if at >= x[n - 1] {
        return y[n - 1];
    }
    let slope: Vec<f64> = (0..n - 1)
        .map(|i| (y[i + 1] - y[i]) / (x[i + 1] - x[i]))
        .collect();
    let tangent = |i: usize| -> f64 {
        if i == 0 || i == n - 1 {
            return 0.0;
        }
        let (a, b) = (slope[i - 1], slope[i]);
        if a * b <= 0.0 {
            0.0
        } else {
            // The harmonic mean keeps the curve within its keys.
            2.0 * a * b / (a + b)
        }
    };
    let i = (0..n - 1).find(|&i| at < x[i + 1]).unwrap_or(n - 2);
    let h = x[i + 1] - x[i];
    let s = (at - x[i]) / h;
    let (s2, s3) = (s * s, s * s * s);
    (2.0 * s3 - 3.0 * s2 + 1.0) * y[i]
        + (s3 - 2.0 * s2 + s) * h * tangent(i)
        + (-2.0 * s3 + 3.0 * s2) * y[i + 1]
        + (s3 - s2) * h * tangent(i + 1)
}
