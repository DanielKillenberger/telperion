//! A leaf or a flower that would cross the ground lies on it (fn-301 R2).
//! An instance is turned about its attachment by the least rotation that
//! brings its bounds' eight corners onto the ground, the plane y = 0. It is
//! never culled, scaled or moved off its attachment. An instance clear of
//! the ground is returned as it came, so a tree that never reaches the
//! ground keeps its bytes (R3).
//!
//! The solve is one function of eight corners, their offsets `v` from the
//! attachment, and how far below the attachment each may reach, `t`: the
//! up direction `d` nearest world up with `v·d ≥ -t` for every corner. A
//! leaf's base corners, its stalk, which is its seat on the wood, may reach
//! as far down as its base depth; the rest clear the ground by the encoder's
//! margin (host, 2026-10-21). The instance is then turned about a
//! horizontal axis by the rotation that carries `d` onto up.
//! `generation/rest.wgsl` is the device's copy, in the same order.
use super::Element;
use crate::math::Vec3;

/// The 10-bit rotation's worst angle, radians: a half code `sqrt 2 / 2046`
/// on each of three components, amplified by up to `sqrt 2` through the
/// dropped one (`packed/tests.rs`).
pub(crate) const ANGLE: f64 = 3.0 / 1023.0;
/// The half-float scale's worst relative rounding.
pub(crate) const HALF: f64 = 1.0 / 2048.0;

/// What packing may move a vertex `reach` metres from its attachment, in a
/// box whose position code steps `step` metres up: half a step, the
/// scale's rounding and the rotation's. The pose clears by it.
pub(crate) fn margin(step: f64, reach: f64) -> f64 {
    0.5 * step + (HALF + ANGLE) * reach
}

/// An element's bounds as the pose reads them, in its own frame (side +X,
/// axis +Y, face +Z): a box whose bottom face tilts about its base edge.
/// `depth` is how far the base row's vertices, its stalk's section, fall
/// below the attachment plane (floored at 0); the slope is the least that
/// still bounds every other vertex.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Wedge {
    pub corners: [Vec3; 8],
    pub depth: f64,
    /// The farthest corner from the attachment, at a size of one.
    pub reach: f64,
}

impl Wedge {
    /// The wedge of an element, measured once where it is prepared. Corner
    /// `k` takes its side from bit 0, its end from bit 1 (base, tip) and its
    /// face from bit 2 (bottom, top).
    pub fn of(element: &Element) -> Self {
        let p = &element.positions;
        let Some(&first) = p.first() else {
            return Self {
                corners: [Vec3::ZERO; 8],
                depth: 0.0,
                reach: 0.0,
            };
        };
        let (mut lo, mut hi) = (first, first);
        for v in p {
            lo = Vec3::new(lo.x.min(v.x), lo.y.min(v.y), lo.z.min(v.z));
            hi = Vec3::new(hi.x.max(v.x), hi.y.max(v.y), hi.z.max(v.z));
        }
        let base = p.iter().filter(|v| v.y == lo.y);
        let depth = base.map(|v| -v.z).fold(0.0, f64::max);
        let slope = (p.iter().filter(|v| v.y > lo.y))
            .map(|v| -(v.z + depth) / (v.y - lo.y))
            .fold(f64::NEG_INFINITY, f64::max);
        let slope = if slope.is_finite() { slope } else { 0.0 };
        let tip = -depth - slope * (hi.y - lo.y);
        let corners = std::array::from_fn(|k| {
            let x = if k & 1 == 0 { lo.x } else { hi.x };
            let (y, bottom) = if k & 2 == 0 {
                (lo.y, -depth)
            } else {
                (hi.y, tip)
            };
            Vec3::new(x, y, if k & 4 == 0 { bottom } else { hi.z })
        });
        let reach = corners.iter().map(|c| c.length()).fold(0.0, f64::max);
        Self {
            corners,
            depth,
            reach,
        }
    }
}

/// One leaf's finished transform, rested: where its wedge's corners would
/// reach below the ground, turned about its attachment so they lie on it,
/// cleared by the encoder's margin, its stalk let down as far as its base
/// depth reaches below the attachment. `step` is the stored box's position
/// code in y.
pub(crate) fn leaf(m: [f32; 16], wedge: &Wedge, step: f64) -> [f32; 16] {
    // A leaf higher than its reach clears whatever its turn: screened
    // before the corners are read, it decides nothing the gate would not.
    let size = f64::from(m[4]).abs() + f64::from(m[5]).abs() + f64::from(m[6]).abs();
    let reach = wedge.reach * size * (1.0 + HALF);
    if f64::from(m[13]) >= reach + margin(step, reach) {
        return m;
    }
    let column = |c: usize| {
        Vec3::new(
            f64::from(m[c * 4]),
            f64::from(m[c * 4 + 1]),
            f64::from(m[c * 4 + 2]),
        )
    };
    let columns = [column(0), column(1), column(2)];
    let v = wedge
        .corners
        .map(|c| columns[0] * c.x + columns[1] * c.y + columns[2] * c.z);
    let scale = (columns[0].length() + columns[1].length() + columns[2].length()) / 3.0;
    let reach = v.iter().map(|v| v.length()).fold(0.0, f64::max);
    let h = f64::from(m[13]).max(0.0);
    let clear = h - margin(step, reach);
    let stalk = clear.max(wedge.depth * scale);
    // Corner `k`'s end is bit 1: its base is the stalk's.
    let t: [f64; 8] = std::array::from_fn(|k| if k & 2 == 0 { stalk } else { clear });
    if v.iter().zip(&t).all(|(v, t)| v.y >= -t) {
        return m;
    }
    let d = upright(&v, &t).unwrap_or_else(|| best(&v));
    let mut out = m;
    for (c, column) in columns.iter().enumerate() {
        let r = turned(*column, d);
        out[c * 4] = r.x as f32;
        out[c * 4 + 1] = r.y as f32;
        out[c * 4 + 2] = r.z as f32;
    }
    out
}

/// The up direction nearest world up for which every corner `v[k]` stands
/// at least `-t[k]` along it, or none where no direction does. The nearest is
/// up itself, or on one corner's bound, or where two corners' bounds meet:
/// each candidate is tried against every corner, and the highest kept.
pub(crate) fn upright(v: &[Vec3; 8], t: &[f64; 8]) -> Option<Vec3> {
    let reach = (v.iter().map(|v| v.length()))
        .chain(t.iter().map(|t| t.abs()))
        .fold(0.0, f64::max);
    let slack = 1e-9 * reach;
    let fits = |d: Vec3| d.is_finite() && v.iter().zip(t).all(|(v, t)| v.dot(d) >= -t - slack);
    let mut found: Option<Vec3> = None;
    let mut offer = |d: Vec3| {
        if fits(d) && found.is_none_or(|f| d.y > f.y) {
            found = Some(d);
        }
    };
    offer(Vec3::Y);
    for (a, t) in v.iter().zip(t) {
        if let Some(d) = one(*a, *t) {
            offer(d);
        }
    }
    for i in 0..8 {
        for j in i + 1..8 {
            for d in two((v[i], t[i]), (v[j], t[j])).into_iter().flatten() {
                offer(d);
            }
        }
    }
    found
}

/// The direction nearest up with `a·d = -t`.
fn one(a: Vec3, t: f64) -> Option<Vec3> {
    let l = a.length();
    let c = -t / l;
    if !(l > 0.0 && c.abs() <= 1.0) {
        return None;
    }
    let n = a / l;
    let w = Vec3::Y - n * n.y;
    let w = if w.length_squared() > 1e-24 {
        w.normalized()
    } else {
        Vec3::X
    };
    Some(n * c + w * (1.0 - c * c).sqrt())
}

/// The two directions with `a·d = -ta` and `b·d = -tb`, where they exist.
fn two((a, ta): (Vec3, f64), (b, tb): (Vec3, f64)) -> [Option<Vec3>; 2] {
    let (la, lb) = (a.length(), b.length());
    if !(la > 0.0 && lb > 0.0) {
        return [None; 2];
    }
    let (na, nb) = (a / la, b / lb);
    let (ca, cb) = (-ta / la, -tb / lb);
    let g = na.dot(nb);
    let det = 1.0 - g * g;
    if det <= 1e-12 {
        return [None; 2];
    }
    let (alpha, beta) = ((ca - g * cb) / det, (cb - g * ca) / det);
    let rest = 1.0 - (alpha * ca + beta * cb);
    if rest < 0.0 {
        return [None; 2];
    }
    let base = na * alpha + nb * beta;
    let across = na.cross(nb) * (rest / det).sqrt();
    [Some(base + across), Some(base - across)]
}

/// Where no direction clears: the one that lifts the corners' box's lowest
/// point highest. Outside the box, toward its nearest point; inside it, away
/// from its nearest face.
pub(crate) fn best(v: &[Vec3; 8]) -> Vec3 {
    let (mut lo, mut hi) = (v[0], v[0]);
    for p in v {
        lo = Vec3::new(lo.x.min(p.x), lo.y.min(p.y), lo.z.min(p.z));
        hi = Vec3::new(hi.x.max(p.x), hi.y.max(p.y), hi.z.max(p.z));
    }
    let near = Vec3::new(
        0f64.clamp(lo.x, hi.x),
        0f64.clamp(lo.y, hi.y),
        0f64.clamp(lo.z, hi.z),
    );
    if near.length_squared() > 0.0 {
        return near.normalized();
    }
    let faces = [
        (-lo.x, Vec3::X),
        (-lo.y, Vec3::Y),
        (-lo.z, Vec3::Z),
        (hi.x, -Vec3::X),
        (hi.y, -Vec3::Y),
        (hi.z, -Vec3::Z),
    ];
    let mut away = faces[0];
    for face in &faces[1..] {
        if face.0 < away.0 {
            away = *face;
        }
    }
    away.1
}

/// `u` turned by the rotation that carries `d` onto up, about the
/// horizontal axis `d × up`; half a turn about +X where `d` is down.
pub(crate) fn turned(u: Vec3, d: Vec3) -> Vec3 {
    let c = d.y;
    if c <= -1.0 + 1e-12 {
        return Vec3::new(u.x, -u.y, -u.z);
    }
    let a = Vec3::new(-d.z, 0.0, d.x);
    u * c + a.cross(u) + a * (a.dot(u) / (1.0 + c))
}

#[cfg(test)]
mod tests;
