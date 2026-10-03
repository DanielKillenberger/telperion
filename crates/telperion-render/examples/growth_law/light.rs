//! Light from the sky (Beer–Lambert): every unit of leaf area sits in its
//! voxel; once per cycle leaf area is carried down along each sky direction
//! (the zenith, and four at `elevation` above the horizon), and a cell's
//! light is the mean over directions of exp(−k × leaf area along it).
//! An open-grown crown's sides then see the low sky, its interior does not.
use telperion_core::math::Vec3;

pub struct Light {
    n: [usize; 3],
    lo: Vec3,
    h: f64,
    leaf: Vec<f32>,
    /// Each cell's light, cached once per rebuild.
    val: Vec<f32>,
    /// Leaf area above each cell, one array per sky direction.
    above: Vec<Vec<f32>>,
    /// Horizontal voxel offset per layer of each direction.
    shift: Vec<(f64, f64)>,
    k: f64,
    pub updates: u64,
}

impl Light {
    pub fn new(lo: Vec3, hi: Vec3, h: f64, k: f64, elevation: f64) -> Self {
        let cells = |l: f64, u: f64| ((u - l) / h).ceil() as usize + 1;
        let n = [cells(lo.x, hi.x), cells(lo.y, hi.y), cells(lo.z, hi.z)];
        let len = n[0] * n[1] * n[2];
        let c = 1.0 / elevation.to_radians().tan();
        let shift = vec![(0.0, 0.0), (c, 0.0), (-c, 0.0), (0.0, c), (0.0, -c)];
        Self { n, lo, h, leaf: vec![0.0; len], val: vec![1.0; len], above: vec![vec![0.0; len]; shift.len()], shift, k, updates: 0 }
    }

    fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        (j * self.n[2] + k) * self.n[0] + i
    }

    fn cell(&self, p: Vec3) -> [usize; 3] {
        let c = |v: f64, o: f64, n: usize| (((v - o) / self.h).floor().max(0.0) as usize).min(n - 1);
        [c(p.x, self.lo.x, self.n[0]), c(p.y, self.lo.y, self.n[1]), c(p.z, self.lo.z, self.n[2])]
    }

    /// Rebuilds the field from this cycle's leaf area (position, amount).
    pub fn rebuild(&mut self, leaves: &[(Vec3, f64)]) {
        self.leaf.iter_mut().for_each(|x| *x = 0.0);
        let (mut top, mut a0, mut a1) = (0, [usize::MAX; 2], [0usize; 2]);
        for &(p, a) in leaves {
            let c = self.cell(p);
            let i = self.idx(c[0], c[1], c[2]);
            self.leaf[i] += a as f32;
            top = top.max(c[1]);
            a0 = [a0[0].min(c[0]), a0[1].min(c[2])];
            a1 = [a1[0].max(c[0]), a1[1].max(c[2])];
        }
        if leaves.is_empty() {
            return;
        }
        // Only cells whose sky rays can meet a leaf: the leaves' box widened
        // by the lowest ray's horizontal run.
        let reach = (self.shift.iter().map(|s| s.0.abs()).fold(0.0, f64::max) * top as f64).ceil() as usize + 1;
        let (nx, nz) = (self.n[0], self.n[2]);
        let (i0, i1) = (a0[0].saturating_sub(reach), (a1[0] + reach).min(nx - 1));
        let (k0, k1) = (a0[1].saturating_sub(reach), (a1[1] + reach).min(nz - 1));
        for d in 0..self.shift.len() {
            let (sx, sz) = self.shift[d];
            let mut above = std::mem::take(&mut self.above[d]);
            above.iter_mut().for_each(|x| *x = 0.0);
            for j in (0..top.min(self.n[1] - 1)).rev() {
                // A ray from layer j reaches (top - j) layers of run sideways.
                let r = ((sx.abs().max(sz.abs())) * (top - j) as f64).ceil() as usize + 1;
                let (li0, li1) = (a0[0].saturating_sub(r).max(i0), (a1[0] + r).min(i1));
                let (lk0, lk1) = (a0[1].saturating_sub(r).max(k0), (a1[1] + r).min(k1));
                for k in lk0..=lk1 {
                    for i in li0..=li1 {
                        // The ray from this cell toward the sky meets the
                        // layer above at (i + sx, k + sz): bilinear there.
                        let (x, z) = (i as f64 + sx, k as f64 + sz);
                        if x < 0.0 || z < 0.0 || x > (nx - 1) as f64 || z > (nz - 1) as f64 {
                            continue;
                        }
                        let (x0, z0) = (x.floor() as usize, z.floor() as usize);
                        let (x1, z1) = ((x0 + 1).min(nx - 1), (z0 + 1).min(nz - 1));
                        let (fx, fz) = ((x - x0 as f64) as f32, (z - z0 as f64) as f32);
                        let at = |a: usize, b: usize| {
                            let u = self.idx(a, j + 1, b);
                            above[u] + self.leaf[u]
                        };
                        let v = (at(x0, z0) * (1.0 - fx) + at(x1, z0) * fx) * (1.0 - fz) + (at(x0, z1) * (1.0 - fx) + at(x1, z1) * fx) * fz;
                        let here = self.idx(i, j, k);
                        above[here] = v;
                    }
                }
                self.updates += ((li1 - li0 + 1) * (lk1 - lk0 + 1)) as u64;
            }
            self.above[d] = above;
        }
        self.val.iter_mut().for_each(|x| *x = 1.0);
        let nd = self.above.len() as f64;
        for j in 0..=top.min(self.n[1] - 1) {
            for k in k0..=k1 {
                for i in i0..=i1 {
                    let x = self.idx(i, j, k);
                    let own = 0.5 * self.leaf[x];
                    self.val[x] = (self.above.iter().map(|a| (-self.k * f64::from(a[x] + own)).exp()).sum::<f64>() / nd) as f32;
                }
            }
        }
    }

    fn at(&self, i: usize, j: usize, k: usize) -> f64 {
        f64::from(self.val[self.idx(i, j, k)])
    }

    /// The light at `p`, 0 to 1.
    pub fn light(&self, p: Vec3) -> f64 {
        let [i, j, k] = self.cell(p);
        self.at(i, j, k)
    }

    /// The normalised gradient of light at `p`: toward light.
    pub fn toward(&self, p: Vec3) -> Vec3 {
        let [i, j, k] = self.cell(p);
        let d = |v: usize, n: usize| (v.saturating_sub(1), (v + 1).min(n - 1));
        let ((i0, i1), (j0, j1), (k0, k1)) = (d(i, self.n[0]), d(j, self.n[1]), d(k, self.n[2]));
        let g = Vec3::new(self.at(i1, j, k) - self.at(i0, j, k), self.at(i, j1, k) - self.at(i, j0, k), self.at(i, j, k1) - self.at(i, j, k0));
        if g.length() > 1e-9 { g.normalized() } else { Vec3::ZERO }
    }
}
