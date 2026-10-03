//! The shadow-propagation light field of Pałubicki et al. 2009, section 4.1:
//! a voxel grid whose every bud casts a pyramid of shadow a·b^-q onto the
//! q-th layer below it, q = 0..qmax.
use telperion_core::math::Vec3;

pub struct Shadow {
    s: Vec<f32>,
    n: [usize; 3],
    origin: Vec3,
    h: f64,
    fall: Vec<f32>,
    pub updates: u64,
}

impl Shadow {
    pub fn new(lo: Vec3, hi: Vec3, h: f64, a: f64, b: f64, depth: usize) -> Self {
        let cells = |l: f64, u: f64| ((u - l) / h).ceil() as usize + 1;
        let n = [cells(lo.x, hi.x), cells(lo.y, hi.y), cells(lo.z, hi.z)];
        Self {
            s: vec![0.0; n[0] * n[1] * n[2]],
            n,
            origin: lo,
            h,
            fall: (0..=depth).map(|q| (a * b.powi(-(q as i32))) as f32).collect(),
            updates: 0,
        }
    }

    fn cell(&self, p: Vec3) -> Option<[usize; 3]> {
        let c = |v: f64, o: f64, n: usize| {
            let i = ((v - o) / self.h).floor();
            (i >= 0.0 && (i as usize) < n).then_some(i as usize)
        };
        Some([
            c(p.x, self.origin.x, self.n[0])?,
            c(p.y, self.origin.y, self.n[1])?,
            c(p.z, self.origin.z, self.n[2])?,
        ])
    }

    fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        (j * self.n[2] + k) * self.n[0] + i
    }

    /// Adds (sign 1) or removes (sign -1) one bud's pyramid.
    pub fn cast(&mut self, p: Vec3, sign: f32) {
        let Some([i0, j0, k0]) = self.cell(p) else { return };
        for (q, &d) in self.fall.iter().enumerate() {
            if q > j0 {
                break;
            }
            let j = j0 - q;
            let (ia, ib) = (i0.saturating_sub(q), (i0 + q).min(self.n[0] - 1));
            let (ka, kb) = (k0.saturating_sub(q), (k0 + q).min(self.n[2] - 1));
            for k in ka..=kb {
                let row = self.idx(0, j, k);
                for v in &mut self.s[row + ia..=row + ib] {
                    *v += sign * d;
                }
            }
            self.updates += ((ib - ia + 1) * (kb - ka + 1)) as u64;
        }
    }

    pub fn at(&self, p: Vec3) -> f64 {
        self.cell(p).map_or(0.0, |[i, j, k]| f64::from(self.s[self.idx(i, j, k)]))
    }

    /// The normalised negative gradient of the shadow: towards light.
    pub fn descent(&self, p: Vec3) -> Vec3 {
        let Some([i, j, k]) = self.cell(p) else { return Vec3::ZERO };
        let g = |a: usize, b: usize, ax: usize| {
            let (lo, hi) = (a.saturating_sub(1), (a + 1).min(self.n[ax] - 1));
            let pick = |v: usize| match ax {
                0 => self.s[self.idx(v, j, k)],
                1 => self.s[self.idx(i, v, k)],
                _ => self.s[self.idx(i, j, v)],
            };
            let _ = b;
            f64::from(pick(hi) - pick(lo))
        };
        let d = Vec3::new(-g(i, 0, 0), -g(j, 0, 1), -g(k, 0, 2));
        if d.length() > 1e-9 {
            d.normalized()
        } else {
            Vec3::ZERO
        }
    }
}
