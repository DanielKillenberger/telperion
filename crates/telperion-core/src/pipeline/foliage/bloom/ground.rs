//! An inflorescence that would cross the ground lies on it (fn-301 R2,
//! host ruling D): its florets and pedicels are turned as one about its
//! seat, by the rest every leaf takes (`rest`), with the box of its points
//! as the eight corners: its axis's samples, each floret's element corners
//! and each pedicel's ends. The rachis the renderer rebuilds from the seat
//! is not turned; its vertices are kept off the ground where they are drawn
//! (`stalk.wgsl`). `place.wgsl`'s `rested` is the device's copy.
use super::super::rest::{best, margin, turned, upright};
use super::Matrix;
use crate::math::Vec3;

/// The box of the points one inflorescence draws, as offsets from its seat.
pub(super) struct Span {
    lo: Vec3,
    hi: Vec3,
}

impl Span {
    /// The box of the axis's samples about `seat`.
    pub(super) fn of(seat: Vec3, axis: &[Vec3]) -> Self {
        let mut span = Self {
            lo: Vec3::ZERO,
            hi: Vec3::ZERO,
        };
        for p in axis {
            span.take(*p - seat);
        }
        span
    }

    fn take(&mut self, v: Vec3) {
        self.lo = Vec3::new(self.lo.x.min(v.x), self.lo.y.min(v.y), self.lo.z.min(v.z));
        self.hi = Vec3::new(self.hi.x.max(v.x), self.hi.y.max(v.y), self.hi.z.max(v.z));
    }

    /// One floret: its pedicel from `at` on the axis to its matrix's origin,
    /// a tube `tube` thick about both ends (`stalk.wgsl`'s
    /// `pedicel_vertex`), and its element's corners `element` under its
    /// matrix.
    pub(super) fn floret(
        &mut self,
        seat: Vec3,
        (at, tube): (Vec3, f64),
        m: &[f32; 16],
        element: &[Vec3; 8],
    ) {
        let round = Vec3::new(tube, tube, tube);
        for end in [at, Vec3::new(m[12].into(), m[13].into(), m[14].into())] {
            self.take(end - seat - round);
            self.take(end - seat + round);
        }
        for c in element {
            self.take(super::super::transform_point(m, *c) - seat);
        }
    }

    fn corners(&self) -> [Vec3; 8] {
        std::array::from_fn(|k| {
            Vec3::new(
                if k & 1 == 0 { self.lo.x } else { self.hi.x },
                if k & 2 == 0 { self.lo.y } else { self.hi.y },
                if k & 4 == 0 { self.lo.z } else { self.hi.z },
            )
        })
    }
}

/// The florets `out` of the inflorescence seated at `seat`, rested: where
/// its span's corners would reach below the ground, every floret is turned
/// about the seat by the rotation that lifts them onto it, or lifts the
/// lowest highest where none does. `step` is the stored box's position code
/// in y.
pub(super) fn rest(seat: Vec3, span: &Span, out: &mut [Matrix], step: f64) {
    let v = span.corners();
    let reach = v.iter().map(|v| v.length()).fold(0.0, f64::max);
    let t = seat.y.max(0.0) - margin(step, reach);
    if v.iter().all(|v| v.y >= -t) {
        return;
    }
    let d = upright(&v, &[t; 8]).unwrap_or_else(|| best(&v));
    for (m, _) in out {
        for c in 0..3 {
            let column = Vec3::new(m[c * 4].into(), m[c * 4 + 1].into(), m[c * 4 + 2].into());
            let r = turned(column, d);
            (m[c * 4], m[c * 4 + 1], m[c * 4 + 2]) = (r.x as f32, r.y as f32, r.z as f32);
        }
        let at = Vec3::new(m[12].into(), m[13].into(), m[14].into());
        let r = seat + turned(at - seat, d);
        (m[12], m[13], m[14]) = (r.x as f32, r.y as f32, r.z as f32);
    }
}

#[cfg(test)]
mod tests;
