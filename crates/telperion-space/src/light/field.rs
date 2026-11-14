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
use crate::error::{refuse, Result};
use crate::structure::Vec3;

/// The position domain: every coordinate below 2^12 m.
const REACH: f32 = 4096.0;
/// The nodes a box holds: the platform's limit, which the product passes
/// wherever a lattice is spanned (`Grid::spanning`).
pub(crate) const NODES: u64 = 1 << 28;
/// The nodes the refusal's tests hold a small tree's lattice to: a box of
/// 3 by 3 by 3 nodes, 1.5 m on a side, which the oak outgrows in its
/// first years.
#[cfg(test)]
pub(crate) const SMALL: u64 = 3 * 3 * 3;
/// The quantised deposit's unit of weighted leaf area: 2^-20 m².
pub(crate) const QUANTUM: f64 = 1.0 / 1_048_576.0;

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
    let n = crate::contract::floor(s);
    (n as i32, s - n)
}

impl Grid {
    /// The box holding every point's eight corners: from the least node to
    /// the greatest plus one on each axis; one node at the origin for none.
    /// A box past `most` nodes (the product's `NODES`, 2^28) is refused by
    /// name, before anything the size of it is allocated.
    pub(super) fn spanning(points: impl Iterator<Item = Vec3>, most: u64) -> Result<Self> {
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
        if nodes > most {
            return refuse("light.lattice", "a lattice spans at most 2^28 nodes");
        }
        Ok(Self { low, size })
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
    pub(super) fn corners(&self, at: Vec3) -> [(Option<usize>, f32); 8] {
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

/// The leaves a part of the quantised deposit holds: enough to pay for
/// its own lattice of quanta and their merge.
const PART: usize = 1 << 12;

/// `x.round()` as a count of quanta; none past a u32, or for NaN. A count
/// rounds to a u32 exactly where it is under `u32::MAX + 0.5`; below
/// that its whole part is exact, what is left of it is exact, and it
/// rounds up, half away from none, where that is a half or more. An `x`
/// under none counts none, as the cast of its `round` does. No call
/// into the platform's libm, which `round` is without SSE4.1.
pub(super) fn rounded(x: f64) -> Option<u32> {
    if !(x < 4_294_967_295.5) {
        return None;
    }
    let whole = x as u32;
    Some(whole + u32::from(x - f64::from(whole) >= 0.5))
}

/// A node past its u32 of quanta, refused by name.
fn overfull<T>() -> Result<T> {
    refuse(
        "light.deposit",
        "a lattice node holds at most 2^32 quanta of 2^-20 m² of leaf area",
    )
}

impl Grid {
    /// Each node's quanta of `leaves`, deposited in turn. A node past its
    /// u32 is refused by name, never wrapped.
    fn quanta_in_turn(&self, leaves: &[(Vec3, f64)]) -> Result<Vec<u32>> {
        let mut quanta = vec![0u32; self.nodes()];
        for &(at, area) in leaves {
            let area = area as f32;
            assert!(area >= 0.0, "a leaf's area inside the contract's bound");
            for (i, w) in self.inside(at) {
                let q = rounded(f64::from(area) * f64::from(w) / QUANTUM);
                let Some(sum) = q.and_then(|q| quanta[i].checked_add(q)) else {
                    return overfull();
                };
                quanta[i] = sum;
            }
        }
        Ok(quanta)
    }

    /// `corners` about a leaf, every one inside the box: each corner's
    /// index and weight as `corners` gives them, the box tested once a
    /// leaf, at its lowest and highest corner, which hold the rest between
    /// them. A weight is its axes' shares multiplied in the axes' order,
    /// as `corners` folds them from one.
    pub(super) fn inside(&self, at: Vec3) -> [(usize, f32); 8] {
        let split = [at.x, at.y, at.z].map(|x| node(x as f32));
        let low = split.map(|s| s.0);
        let within = |n: [i32; 3]| self.index(n).expect("a leaf inside its box");
        let (base, _highest) = (within(low), within(low.map(|n| n.wrapping_add(1))));
        let [sx, sy, _] = self.size.map(|s| s as usize);
        let share = split.map(|s| [1.0 - s.1, s.1]);
        std::array::from_fn(|c| {
            let up = |a: usize| (c >> a) & 1;
            let i = base + up(0) + up(1) * sx + up(2) * sx * sy;
            (i, share[0][up(0)] * share[1][up(1)] * share[2][up(2)])
        })
    }

    /// Each node's quanta of `leaves`, on up to `threads` cores: the
    /// leaves cut into parts, each deposited on its own lattice, and the
    /// lattices added. A quantum is a whole number of at least none, so
    /// the sum is the same in any parts, and a node passes its u32 in
    /// them exactly where it does in turn: where one quantum is past it,
    /// or the node's whole sum is. A part's lattice costs its nodes, so
    /// the leaves are cut only where each part holds more of them than
    /// the box has nodes.
    fn quanta(&self, leaves: &[(Vec3, f64)], threads: usize) -> Result<Vec<u32>> {
        let parts = threads.min(leaves.len() / PART.max(self.nodes()));
        if parts <= 1 {
            return self.quanta_in_turn(leaves);
        }
        let cut: Vec<&[(Vec3, f64)]> = leaves.chunks(leaves.len().div_ceil(parts)).collect();
        let each = crate::cores::map(cut, 1, threads, &|_, part| self.quanta_in_turn(part));
        let mut quanta = vec![0u32; self.nodes()];
        for part in each {
            for (sum, q) in quanta.iter_mut().zip(part?) {
                let Some(more) = sum.checked_add(q) else {
                    return overfull();
                };
                *sum = more;
            }
        }
        Ok(quanta)
    }
}

impl Field {
    /// The field of `leaves` (each a point and its leaf area in square
    /// metres) under `light`, over a box that holds every leaf and every
    /// point in `reads`: test-only, the unquantised lattice.
    #[cfg(test)]
    pub fn new(light: &super::Light, leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Self {
        Self::on(light, leaves, reads, 1)
    }

    /// The unquantised field `new` makes, its directions swept on up to
    /// `threads` cores: the whole-tree grower's light (`grow::run`),
    /// test-only.
    #[cfg(test)]
    pub fn on(
        light: &super::Light,
        leaves: &[(Vec3, f64)],
        reads: &[Vec3],
        threads: usize,
    ) -> Self {
        let points = leaves.iter().map(|l| l.0).chain(reads.iter().copied());
        let mut field = Self::spanning(points, NODES).expect("a test lattice inside its nodes");
        field.deposit_all(leaves);
        field.sweep(light.extinction, &light.directions(), threads);
        field
    }

    /// `on`, its leaves deposited as integer quanta of `QUANTUM` m² of
    /// weighted area, a u32 a node (COUNCIL.md's early test 2): the sum is
    /// the same in any order. A node past its u32 is refused by name,
    /// never wrapped; a box past `most` nodes too (`Grid::spanning`).
    pub fn quantised(
        light: &super::Light,
        leaves: &[(Vec3, f64)],
        reads: &[Vec3],
        (threads, most): (usize, u64),
    ) -> Result<Self> {
        Self::quantised_with(light, leaves, reads, (threads, most)).map(|q| q.0)
    }

    /// `quantised`, with each node's quanta (fn-256's busy-cycle capture).
    pub(crate) fn quantised_with(
        light: &super::Light,
        leaves: &[(Vec3, f64)],
        reads: &[Vec3],
        (threads, most): (usize, u64),
    ) -> Result<(Self, Vec<u32>)> {
        let points = leaves.iter().map(|l| l.0).chain(reads.iter().copied());
        let mut field = Self::spanning(points, most)?;
        let quanta = field.grid.quanta(leaves, threads)?;
        for (d, &q) in field.density.iter_mut().zip(&quanta) {
            *d = (8.0 * f64::from(q) * QUANTUM) as f32;
        }
        field.sweep(light.extinction, &light.directions(), threads);
        Ok((field, quanta))
    }

    /// Its box, densities and light (fn-256's busy-cycle capture).
    pub(crate) fn parts(&self) -> (Grid, &[f32], &[f32]) {
        (self.grid, &self.density, &self.light)
    }

    /// The nodes it holds.
    pub fn nodes(&self) -> usize {
        self.grid.nodes()
    }

    /// Its box's nodes on each axis.
    pub fn size(&self) -> [u32; 3] {
        self.grid.size
    }

    /// The nodes any leaf area reached.
    pub fn occupied(&self) -> usize {
        self.density.iter().filter(|&&d| d > 0.0).count()
    }

    /// The light at `at`: its corners inside the box by their nodes' light,
    /// those outside by the open sky's one. Each corner's weight is taken
    /// as it stands, never as one less the rest, and the f32 weights may
    /// sum a place over one, so the light is held to nothing to one
    /// (fn-254 review).
    pub fn at(&self, at: Vec3) -> f64 {
        let sum: f32 = self
            .grid
            .corners(at)
            .into_iter()
            .map(|(i, w)| w * i.map_or(1.0, |i| self.light[i]))
            .sum();
        f64::from(sum.clamp(0.0, 1.0))
    }

    pub(super) fn spanning(points: impl Iterator<Item = Vec3>, most: u64) -> Result<Self> {
        let grid = Grid::spanning(points, most)?;
        Ok(Self {
            grid,
            density: vec![0.0; grid.nodes()],
            light: vec![1.0; grid.nodes()],
        })
    }

    /// Every leaf's area deposited at its corners, in the leaves' order.
    #[cfg(test)]
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
        self.light = light
            .into_iter()
            .map(|l| l.clamp(0.0, 1.0) as f32)
            .collect();
    }
}
