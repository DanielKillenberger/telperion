//! Light from above over the full height (Beer–Lambert): every living tip
//! carries one unit of leaf area in its voxel; leaf area is carried down
//! each column once per cycle, spreading one voxel sideways per layer (a
//! 45° sky cone, by a 3 × 3 box average), and a cell's light is
//! exp(−k × leaf area above it).
use telperion_core::math::Vec3;

pub struct Light {
    n: [usize; 3],
    lo: Vec3,
    h: f64,
    leaf: Vec<f32>,
    above: Vec<f32>,
    k: f64,
    /// Cells visited by the downward pass, for the time split.
    pub updates: u64,
}

impl Light {
    pub fn new(lo: Vec3, hi: Vec3, h: f64, k: f64) -> Self {
        let cells = |l: f64, u: f64| ((u - l) / h).ceil() as usize + 1;
        let n = [cells(lo.x, hi.x), cells(lo.y, hi.y), cells(lo.z, hi.z)];
        let len = n[0] * n[1] * n[2];
        Self { n, lo, h, leaf: vec![0.0; len], above: vec![0.0; len], k, updates: 0 }
    }

    fn idx(&self, i: usize, j: usize, k: usize) -> usize {
        (j * self.n[2] + k) * self.n[0] + i
    }

    fn cell(&self, p: Vec3) -> [usize; 3] {
        let c = |v: f64, o: f64, n: usize| (((v - o) / self.h).floor().max(0.0) as usize).min(n - 1);
        [c(p.x, self.lo.x, self.n[0]), c(p.y, self.lo.y, self.n[1]), c(p.z, self.lo.z, self.n[2])]
    }

    /// Rebuilds the field from this cycle's leaf-bearing points, over the
    /// columns their box covers (plus the cone's spread below them).
    pub fn rebuild(&mut self, leaves: &[Vec3]) {
        self.leaf.iter_mut().for_each(|x| *x = 0.0);
        self.above.iter_mut().for_each(|x| *x = 0.0);
        if leaves.is_empty() {
            return;
        }
        let (mut a, mut b) = ([usize::MAX; 3], [0usize; 3]);
        for &p in leaves {
            let c = self.cell(p);
            let i = self.idx(c[0], c[1], c[2]);
            self.leaf[i] += 1.0;
            for d in 0..3 {
                a[d] = a[d].min(c[d]);
                b[d] = b[d].max(c[d]);
            }
        }
        let top = b[1];
        let spread = top;
        let (i0, i1) = (a[0].saturating_sub(spread), (b[0] + spread).min(self.n[0] - 1));
        let (k0, k1) = (a[2].saturating_sub(spread), (b[2] + spread).min(self.n[2] - 1));
        for j in (0..top).rev() {
            for k in k0..=k1 {
                for i in i0..=i1 {
                    let (mut s, mut c) = (0.0f32, 0.0f32);
                    for dk in k.saturating_sub(1)..=(k + 1).min(self.n[2] - 1) {
                        for di in i.saturating_sub(1)..=(i + 1).min(self.n[0] - 1) {
                            let up = self.idx(di, j + 1, dk);
                            s += self.above[up] + self.leaf[up];
                            c += 1.0;
                        }
                    }
                    let here = self.idx(i, j, k);
                    self.above[here] = s / c;
                }
            }
            self.updates += ((i1 - i0 + 1) * (k1 - k0 + 1)) as u64;
        }
    }

    fn at(&self, i: usize, j: usize, k: usize) -> f64 {
        let x = self.idx(i, j, k);
        (-self.k * f64::from(self.above[x] + 0.5 * self.leaf[x])).exp()
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
