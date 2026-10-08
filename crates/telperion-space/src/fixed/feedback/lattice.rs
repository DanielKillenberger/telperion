//! The light lattice in integers (CONTRACT-A.md 2.6; Astra's review,
//! point 5): a leaf's Q32 tip and area deposited at its eight corners in W,
//! the shared CPU sweep's light rounded once to Q32, and a bud's light
//! sampled from it in integers.
//!
//! - **Nodes** are 0.5 m apart, a power of two, so a coordinate's node is
//!   its Q32 shifted right 31 (the floor, negatives included) and its
//!   fraction the low 31 bits.
//! - **Corner weights** are Q62: seven the floored products of their three
//!   Q31 factors, the all-upper corner what is left of one, so they sum to
//!   one exactly.
//! - **Deposits** conserve area exactly: seven floored, the eighth the
//!   rest of the leaf's `8 area` in W.
//! - **Every bound is checked before accumulation**, pointwise: a
//!   position inside `REACH`, an area in `[0, AREA)`, fewer than `LEAVES`
//!   leaves, so a node's sum stays below 2^123 in any order, and a box of
//!   at most `NODES` nodes.
use super::Fault;
use crate::fixed::{ONE32, P_ONE};
use crate::light::Light;

/// A node's spacing, 0.5 m, as a shift of Q32.
pub const SHIFT: u32 = 31;
/// The position domain: every coordinate below 2^12 m, Q32.
pub const REACH: i64 = 1 << 44;
/// A leaf's area below 2^16 square metres, Q32.
pub const AREA: i64 = 1 << 48;
/// The leaves a lattice takes: a u64, so the bound is the same on a
/// 32-bit target (Wasm), where a `usize` cannot hold it.
pub const LEAVES: u64 = 1 << 40;
/// The nodes a box holds.
pub const NODES: u64 = 1 << 28;

/// A node's light, rounded once to the nearest Q32, ties to even, within
/// [0, 1].
pub fn rounded(l: f64) -> i64 {
    (l * ONE32 as f64)
        .round_ties_even()
        .clamp(0.0, ONE32 as f64) as i64
}

/// A coordinate's node and its Q31 fraction.
pub fn node(x: i64) -> (i64, u64) {
    (x >> SHIFT, (x & ((1 << SHIFT) - 1)) as u64)
}

/// The eight corners' weights, Q62, from the three fractions: corner `c`
/// is upper on axis `a` where bit `a` of `c` is set (`light::corners`).
pub fn weights(t: [u64; 3]) -> [u64; 8] {
    let factor = |c: usize, a: usize| {
        u128::from(if (c >> a) & 1 == 1 {
            t[a]
        } else {
            (1 << SHIFT) - t[a]
        })
    };
    let mut w = [0u64; 8];
    let mut rest = P_ONE;
    for (c, weight) in w.iter_mut().enumerate().take(7) {
        *weight = ((factor(c, 0) * factor(c, 1) * factor(c, 2)) >> 31) as u64;
        rest -= *weight;
    }
    w[7] = rest;
    w
}

/// A leaf's eight deposits in W, `8 area w` (density over a 1/8 m^3 cell):
/// seven floored, the eighth the rest of `8 area`.
pub fn deposits(area: i64, w: &[u64; 8]) -> [u128; 8] {
    let a = area as u128;
    let mut d = [0u128; 8];
    let mut rest = a << 35;
    for c in 0..7 {
        d[c] = (a * u128::from(w[c])) >> 27;
        rest -= d[c];
    }
    d[7] = rest;
    d
}

/// The box of nodes a lattice spans.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid {
    pub low: [i64; 3],
    pub size: [u64; 3],
}

impl Grid {
    /// The box holding every point's eight corners: from the least node to
    /// the greatest plus one on each axis; one node at the origin for none.
    pub fn spanning(points: impl Iterator<Item = [i64; 3]>) -> Result<Self, Fault> {
        let (mut low, mut high) = ([i64::MAX; 3], [i64::MIN; 3]);
        for p in points {
            for a in 0..3 {
                if p[a].unsigned_abs() >= REACH as u64 {
                    return Err(Fault::Range);
                }
                let n = p[a] >> SHIFT;
                low[a] = low[a].min(n);
                high[a] = high[a].max(n + 1);
            }
        }
        if low[0] > high[0] {
            (low, high) = ([0; 3], [0; 3]);
        }
        let size = [0, 1, 2].map(|a| (high[a] - low[a] + 1) as u64);
        if size.iter().product::<u64>() > NODES {
            return Err(Fault::Range);
        }
        Ok(Self { low, size })
    }

    pub fn nodes(&self) -> usize {
        self.size.iter().product::<u64>() as usize
    }

    /// A node's index, x fastest; none outside the box.
    pub fn index(&self, n: [i64; 3]) -> Option<usize> {
        let mut at = [0u64; 3];
        for a in 0..3 {
            let k = n[a] - self.low[a];
            if k < 0 || k as u64 >= self.size[a] {
                return None;
            }
            at[a] = k as u64;
        }
        Some(((at[2] * self.size[1] + at[1]) * self.size[0] + at[0]) as usize)
    }

    /// The corners about `at`: each node's index (none outside) and weight.
    pub fn corners(&self, at: [i64; 3]) -> [(Option<usize>, u64); 8] {
        let (n, t): (Vec<i64>, Vec<u64>) = at.iter().map(|&x| node(x)).unzip();
        let w = weights([t[0], t[1], t[2]]);
        std::array::from_fn(|c| {
            let up = |a: usize| n[a] + ((c >> a) & 1) as i64;
            (self.index([up(0), up(1), up(2)]), w[c])
        })
    }

    /// One leaf deposited into `lattice`, wrapping (exact under the
    /// bounds); `Range` for an area outside `[0, AREA)`.
    pub fn deposit(&self, lattice: &mut [u128], at: [i64; 3], area: i64) -> Result<(), Fault> {
        if !(0..AREA).contains(&area) {
            return Err(Fault::Range);
        }
        let corners = self.corners(at);
        let w = corners.map(|c| c.1);
        for (c, d) in deposits(area, &w).into_iter().enumerate() {
            let i = corners[c].0.ok_or(Fault::Range)?;
            lattice[i] = lattice[i].wrapping_add(d);
        }
        Ok(())
    }

    /// Every leaf's deposits: the same lattice for any order or grouping.
    pub fn deposit_all(&self, leaves: &[([i64; 3], i64)]) -> Result<Vec<u128>, Fault> {
        if leaves.len() as u64 >= LEAVES {
            return Err(Fault::Range);
        }
        let mut lattice = vec![0u128; self.nodes()];
        for &(at, area) in leaves {
            self.deposit(&mut lattice, at, area)?;
        }
        Ok(lattice)
    }

    /// The light at each node, Q32: the one sweep (`light::swept_w`) over
    /// the lattice's densities, each node's light rounded once (`rounded`).
    /// One wherever nothing shades.
    pub fn swept(&self, light: &Light, lattice: &[u128]) -> Vec<i64> {
        if !light.shades() {
            return vec![ONE32; lattice.len()];
        }
        let size = self.size.map(|s| s as usize);
        crate::light::swept_w(light.extinction, &light.directions(), size, lattice, 1)
            .into_iter()
            .map(rounded)
            .collect()
    }

    /// A bud's light, Q32, floored: the corners inside the box by their
    /// nodes' light, the rest of the weight by the open sky's one.
    pub fn sample(&self, light: &[i64], at: [i64; 3]) -> i64 {
        let (mut sum, mut inside) = (0u128, 0u64);
        for (i, w) in self.corners(at) {
            if let Some(i) = i {
                sum += u128::from(w) * light[i] as u128;
                inside += w;
            }
        }
        sum += u128::from(P_ONE - inside) * ONE32 as u128;
        (sum >> 62) as i64
    }
}
