//! A pinnate frond's leaflets before any draw: where each leaves the rachis,
//! the way it points, the rachis's own direction there and the share of its
//! size it is drawn at, with the frame a leaf is turned in about its axis.
//! Placement draws each leaflet from these, and the leaf plan bounds them, so
//! the planned and the placed frond stand in the same place.
use super::{crown::Split, rosette::frame, CanopyParams};
use crate::{
    math::{Transcendental, Vec3},
    pipeline::space::FINEST,
    rng::Rng,
    Error, Result,
};
use std::{
    cmp::Ordering,
    f64::consts::{PI, TAU},
};

/// One leaflet on its rachis, before its draws turn and scale it.
#[derive(Debug, Clone, Copy)]
pub(crate) struct Leaflet {
    pub(crate) at: Vec3,
    pub(crate) axis: Vec3,
    /// The rachis's direction where the leaflet leaves it.
    pub(super) run: Vec3,
    /// The share of its size a basal leaflet borne as a spine is drawn at.
    pub(crate) share: Option<f64>,
}

/// The point `t` of the way along the rachis that leaves `point` on
/// `heading`, bent out of that line by its arch toward `lift`, and the
/// rachis's direction there.
///
/// A rachis whose bark, `girth` where it leaves the stem and tapering by
/// its station (`radius`), would pass below the ground meets it (fn-301
/// R2). From its first contact it lies along the ground, its centreline
/// at its radius, in the horizontal direction it met the ground on,
/// keeping its length: past contact `t` maps to the parabola's own arc
/// length (`along`). One that meets it straight down lies along its
/// frame's `lift`, the way its arch bends it (+`lift` with no arch; host,
/// 2026-10-21). A base at or under its radius rests from the base. A
/// rachis that never reaches the ground is the parabola, by today's
/// operations.
pub(crate) fn rachis(
    point: Vec3,
    heading: Vec3,
    lift: Vec3,
    p: &CanopyParams,
    girth: f64,
    t: f64,
) -> Result<(Vec3, Vec3)> {
    let (length, arch) = (p.rachis_length, p.rachis_arch);
    let at = |t: f64| point + heading * (length * t) + lift * (arch * length * t * t);
    let run = |t: f64| heading + lift * (2. * arch * t);
    let rests = contact(point.y, heading.y * length, lift.y * arch * length, girth);
    let Some(c) = rests.filter(|c| t >= *c) else {
        return Ok((at(t), run(t).normalized()));
    };
    let level = |v: Vec3| Vec3::new(v.x, 0., v.z);
    let met = level(run(c));
    let bends = if arch < 0. { -1. } else { 1. };
    let toward = match met.length_squared() > 1e-12 {
        true => met,
        false => level(lift) * bends,
    }
    .normalized();
    let lies = at(c) + toward * along(length, arch, girth, (c, t))?;
    Ok((Vec3::new(lies.x, radius(girth, t), lies.z), toward))
}

/// How far a resting rachis runs along the ground from share `c` to `t`:
/// the parabola's arc length between them less what its centreline falls
/// as its radius tapers, by `girth` a share until the finest wood and not
/// past it, so the resting axis is as long as the parabola it replaces.
fn along(length: f64, arch: f64, girth: f64, (c, t): (f64, f64)) -> Result<f64> {
    let thin = thin(girth);
    let tapered = match c < thin {
        true => ground(length, arch, girth, t.min(thin))? - ground(length, arch, girth, c)?,
        false => 0.,
    };
    Ok(tapered + ground(length, arch, 0., t.max(thin))? - ground(length, arch, 0., c.max(thin))?)
}

/// The run along the ground to share `t` of a rachis whose centreline
/// falls by `fall` a share, in closed form: the parabola's speed less the
/// fall, `√(length² − fall² + (2·arch·length·t)²)` (its heading and lift
/// square and of unit length). Read only where it lies; one falling faster
/// than its parabola runs there has no way to lie, and is refused.
fn ground(length: f64, arch: f64, fall: f64, t: f64) -> Result<f64> {
    let square = length * length - fall * fall;
    let b = (2. * arch * length).abs();
    let speed = square + b * b * t * t;
    if speed < 0. {
        return Err(Error::InvalidInput(
            "a frond falls faster than it runs where it rests",
        ));
    }
    let speed = speed.sqrt();
    if b == 0. {
        return Ok(speed * t);
    }
    let tail = match square.total_cmp(&0.) {
        Ordering::Greater => asinh(b * t / square.sqrt()),
        Ordering::Less => acosh(b * t / (-square).sqrt()),
        Ordering::Equal => 0.,
    };
    Ok(t / 2. * speed + square / (2. * b) * tail)
}

fn asinh(x: f64) -> f64 {
    (x + x * x / (1. + (1. + x * x).sqrt())).ln_1p_fixed()
}

/// `acosh` at or past one, where the rachis lies.
fn acosh(x: f64) -> f64 {
    let over = x - 1.;
    (over + (over * (x + 1.)).max(0.).sqrt()).ln_1p_fixed()
}

/// The share where a rachis's taper meets the finest wood: over the taper
/// its radius falls by `girth` a share, past it it stands at `FINEST`.
fn thin(girth: f64) -> f64 {
    if girth > FINEST {
        1. - FINEST / girth
    } else {
        0.
    }
}

/// The rachis's radius at share `t`: its `girth` tapering to its tip, and
/// never below the finest wood.
pub(super) fn radius(girth: f64, t: f64) -> f64 {
    (girth * (1. - t)).max(FINEST)
}

/// The least share at which a centreline standing `base` high, rising by
/// `rise` a share and bending by `bend` a share squared, comes down to its
/// radius (`radius`); none where it never does.
fn contact(base: f64, rise: f64, bend: f64, girth: f64) -> Option<f64> {
    if base <= radius(girth, 0.) {
        return Some(0.);
    }
    let thin = thin(girth);
    root(bend, rise + girth, base - girth, 0., thin)
        .or_else(|| root(bend, rise, base - FINEST, thin, 1.))
}

/// The least root of `a t² + b t + c` within `[lo, hi]`.
fn root(a: f64, b: f64, c: f64, lo: f64, hi: f64) -> Option<f64> {
    let within = |t: f64| (lo..=hi).contains(&t).then_some(t);
    if a == 0. {
        return (b != 0.).then(|| -c / b).and_then(within);
    }
    let discriminant = b * b - 4. * a * c;
    if discriminant < 0. {
        return None;
    }
    let q = -0.5 * (b + discriminant.sqrt().copysign(b));
    let (x, y) = (q / a, if q != 0. { c / q } else { q / a });
    within(x.min(y)).or_else(|| within(x.max(y)))
}

/// Where leaflet `leaf` of `count` leaves its rachis, as a share of the
/// rachis, and its length as a share of a mature leaflet's: evenly to the
/// rachis's end on a mature frond, and where its strip begins on a young
/// one (`Split::segment`). The leaflets and the frond's axis both stand on
/// these stations.
pub(super) fn station(
    leaf: usize,
    count: usize,
    p: &CanopyParams,
    split: Option<Split>,
) -> (f64, f64) {
    match split {
        Some(s) => s.segment(leaf, count, p.rachis_length, p.size * s.unit),
        None => ((leaf + 1) as f64 / count as f64, 1.0),
    }
}

/// The `count` leaflets of the rachis that leaves `point` on `heading`,
/// `girth` its radius there (`rachis`).
///
/// The rachis runs a `rachis_length` from the station, bending out of that
/// straight line by its arch as it goes, and each leaflet leaves it by
/// `leaflet_pitch` to alternating sides. The last of them closes the rachis's
/// end as `terminal_leaflet` blends its pitch back toward the rachis itself.
/// A young frond's (`split`) are strips of one blade (`Split::segment`):
/// each stands where its strip begins and runs toward the blade's tip.
pub(crate) fn leaflets(
    point: Vec3,
    heading: Vec3,
    (p, girth): (CanopyParams, f64),
    count: usize,
    split: Option<Split>,
) -> impl Iterator<Item = Result<Leaflet>> {
    // The frond's own plane: the rachis runs along `heading`, the leaflets
    // leave it to either side, and the arch lifts it out of the line between.
    let (lift, _) = frame(heading);
    let pitch = p.leaflet_pitch.to_radians();
    (0..count).map(move |leaf| {
        let (t, length) = station(leaf, count, &p, split);
        let (at, run) = rachis(point, heading, lift, &p, girth, t)?;
        let closing = if leaf + 1 == count {
            1. - p.terminal_leaflet
        } else {
            1.
        };
        let hand = if leaf % 2 == 0 { 1. } else { -1. };
        // A basal leaflet borne as a spine leaves the rachis at its own pitch
        // and is drawn at its own share of the size the leaflet would have
        // had. No draw moves: the spine is the same instance, scaled.
        let borne = spine(leaf, &p);
        let pitch = borne.map_or(pitch, |(_, steeper)| steeper);
        Ok(Leaflet {
            at,
            axis: run.rotate(lift, hand * pitch * closing),
            run,
            share: match (borne, split) {
                (_, None) => borne.map(|(share, _)| share),
                (b, Some(_)) => Some(b.map_or(1.0, |(share, _)| share) * length),
            },
        })
    })
}

/// The share of its own size a basal leaflet is drawn at and the radians it
/// leaves the rachis on, where the rows bear it as a spine. None everywhere
/// else, so a frond that bears none is scaled by nothing at all.
fn spine(index: usize, p: &CanopyParams) -> Option<(f64, f64)> {
    let borne = (index as f64) < p.acanthophylls && p.acanthophyll_length > 0.;
    borne.then(|| (p.acanthophyll_length, p.acanthophyll_pitch.to_radians()))
}

/// A leaf's frame after its draws, side, axis and face, and its scale: the
/// scatter turns the whole frame by up to `scatter` degrees about a random
/// axis, and the size varies by up to `size_variation` either way. Four
/// draws where the leaf scatters, one where it does not.
pub(super) fn drawn(
    axis: Vec3,
    tangent: Vec3,
    normal: Vec3,
    p: &CanopyParams,
    rng: &mut Rng,
) -> ([Vec3; 3], f64) {
    let (mut face, mut side) = leaf_frame(axis, tangent, normal);
    let mut axis = axis;
    if p.scatter > 0. {
        let z = rng.range(-1., 1.);
        let phi = rng.range(0., TAU);
        let ring = (1. - z * z).max(0.).sqrt();
        let jitter = Vec3::new(ring * phi.cos_fixed(), z, ring * phi.sin_fixed());
        let angle = p.scatter * PI / 180. * rng.next_f64();
        let sin_cos = angle.sin_cos_fixed();
        axis = axis.rotate_sin_cos(jitter, sin_cos);
        face = face.rotate_sin_cos(jitter, sin_cos);
        side = side.rotate_sin_cos(jitter, sin_cos);
    }
    let scale = p.size * (1. + p.size_variation * rng.range(-1., 1.));
    ([side, axis, face], scale)
}

/// The face and the side a leaf pointing along `axis` is turned to before
/// its scatter: the face toward the sky where the axis leaves any, else
/// toward `tangent`, else `normal`, and the side square to both.
pub(super) fn leaf_frame(axis: Vec3, tangent: Vec3, normal: Vec3) -> (Vec3, Vec3) {
    let mut face = Vec3::Y - axis * axis.y;
    if face.length_squared() <= 1e-12 {
        face = tangent - axis * tangent.dot(axis);
    }
    if face.length_squared() <= 1e-12 {
        face = normal - axis * normal.dot(axis);
    }
    let face = face.normalized();
    (face, axis.cross(face).normalized())
}
