//! The light lattice (fn-211 task 9, CONTRACT-A.md 2.6; in f32 since
//! fn-254): every leaf's tip and area deposited at its eight corners, the
//! one sweep run on the lattice and its light rounded once to f32, and
//! every read sampled from it.
//!
//! - **Nodes** are 0.5 m apart: a coordinate's node is the floor of twice
//!   it, and its fraction what is left.
//! - **Corner weights** are the trilinear products of the three fractions.
//! - **Deposits** are summed in the leaves' order, so a lattice is the same
//!   for the same leaves in the same order.
//! - **The domain** is every coordinate below 2^12 m and a box of at most
//!   2^28 nodes, refused by name past them (CONTRACT-A.md 2.2).
use super::CELL;
use crate::structure::Vec3;

/// The position domain: every coordinate below 2^12 m.
const REACH: f32 = 4096.0;
/// The nodes a box holds.
const NODES: u64 = 1 << 28;

/// A lattice of light: leaf area deposited, swept from the sky, read.
pub(crate) struct Field {
    grid: Grid,
    /// Leaf area density at each node (8 area w a deposit: density over a
    /// 1/8 m^3 cell), then the light there.
    density: Vec<f32>,
    light: Vec<f32>,
}

/// The box of nodes a lattice spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct Grid {
    pub low: [i32; 3],
    pub size: [u32; 3],
}

/// A coordinate's node and its fraction of the way to the next.
fn node(x: f32) -> (i32, f32) {
    let s = x / CELL as f32;
    let n = s.floor();
    (n as i32, s - n)
}

impl Grid {
    /// The box holding every point's eight corners: from the least node to
    /// the greatest plus one on each axis; one node at the origin for none.
    pub(super) fn spanning(points: impl Iterator<Item = Vec3>) -> Self {
        let (mut low, mut high) = ([i32::MAX; 3], [i32::MIN; 3]);
        for p in points {
            let p = [p.x as f32, p.y as f32, p.z as f32];
            for a in 0..3 {
                assert!(p[a].abs() < REACH, "positions inside the contract's domain");
                let n = node(p[a]).0;
                low[a] = low[a].min(n);
                high[a] = high[a].max(n + 1);
            }
        }
        if low[0] > high[0] {
            (low, high) = ([0; 3], [0; 3]);
        }
        let size = [0, 1, 2].map(|a| (high[a] - low[a] + 1) as u32);
        let nodes: u64 = size.iter().map(|&s| u64::from(s)).product();
        assert!(nodes <= NODES, "a lattice inside the contract's domain");
        Self { low, size }
    }

    fn nodes(&self) -> usize {
        self.size.iter().map(|&s| s as usize).product()
    }

    /// A node's index, x fastest; none outside the box.
    fn index(&self, n: [i32; 3]) -> Option<usize> {
        let mut at = [0usize; 3];
        for a in 0..3 {
            let k = n[a] - self.low[a];
            if k < 0 || k as u32 >= self.size[a] {
                return None;
            }
            at[a] = k as usize;
        }
        let [sx, sy, _] = self.size.map(|s| s as usize);
        Some((at[2] * sy + at[1]) * sx + at[0])
    }

    /// The corners about `at`: each node's index (none outside) and weight;
    /// corner `c` is upper on axis `a` where bit `a` of `c` is set.
    fn corners(&self, at: Vec3) -> [(Option<usize>, f32); 8] {
        let split = [at.x, at.y, at.z].map(|x| node(x as f32));
        std::array::from_fn(|c| {
            let upper = |a: usize| (c >> a) & 1 == 1;
            let n = std::array::from_fn(|a| split[a].0 + i32::from(upper(a)));
            let w = (0..3).fold(1.0, |w, a| {
                w * if upper(a) {
                    split[a].1
                } else {
                    1.0 - split[a].1
                }
            });
            (self.index(n), w)
        })
    }
}

impl Field {
    /// The field of `leaves` (each a point and its leaf area in square
    /// metres) under `light`, over a box that holds every leaf and every
    /// point in `reads`.
    pub fn new(light: &super::Light, leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Self {
        Self::on(light, leaves, reads, 1)
    }

    /// `new`, its directions swept on up to `threads` cores.
    pub fn on(
        light: &super::Light,
        leaves: &[(Vec3, f64)],
        reads: &[Vec3],
        threads: usize,
    ) -> Self {
        let mut field = Self::spanning(leaves.iter().map(|l| l.0).chain(reads.iter().copied()));
        field.deposit_all(leaves);
        field.sweep(light.extinction, &light.directions(), threads);
        field
    }

    /// The lattice `leaves` deposit in the box spanning them and `reads`,
    /// before any sweep.
    pub(crate) fn deposited(leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Vec<f32> {
        let points = leaves.iter().map(|l| l.0).chain(reads.iter().copied());
        let mut field = Self::spanning(points);
        field.deposit_all(leaves);
        field.density
    }

    /// The nodes it holds.
    pub fn nodes(&self) -> usize {
        self.grid.nodes()
    }

    /// The light at `at`: its corners inside the box by their nodes' light,
    /// the rest of the weight by the open sky's one.
    pub fn at(&self, at: Vec3) -> f64 {
        let (mut sum, mut inside) = (0.0f32, 0.0f32);
        for (i, w) in self.grid.corners(at) {
            if let Some(i) = i {
                sum += w * self.light[i];
                inside += w;
            }
        }
        f64::from(sum + (1.0 - inside))
    }

    pub(super) fn spanning(points: impl Iterator<Item = Vec3>) -> Self {
        let grid = Grid::spanning(points);
        Self {
            grid,
            density: vec![0.0; grid.nodes()],
            light: vec![1.0; grid.nodes()],
        }
    }

    /// Every leaf's area deposited at its corners, in the leaves' order.
    pub(super) fn deposit_all(&mut self, leaves: &[(Vec3, f64)]) {
        for &(at, area) in leaves {
            let area = area as f32;
            assert!(area >= 0.0, "a leaf's area inside the contract's bound");
            for (i, w) in self.grid.corners(at) {
                let i = i.expect("a leaf inside its box");
                self.density[i] += 8.0 * area * w;
            }
        }
    }

    /// The one sweep (`super::swept`), its light rounded once to f32.
    pub(super) fn sweep(&mut self, extinction: f64, directions: &[(Vec3, f64)], threads: usize) {
        if extinction <= 0.0 {
            return;
        }
        let size = self.grid.size.map(|s| s as usize);
        let density: Vec<f64> = self.density.iter().map(|&d| f64::from(d)).collect();
        let light = super::swept_all(extinction, directions, size, &density, threads);
        self.light = light.into_iter().map(|l| l.clamp(0.0, 1.0) as f32).collect();
    }
}
