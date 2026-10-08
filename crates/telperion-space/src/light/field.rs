//! The light lattice on contract A (fn-211 task 9, CONTRACT-A.md 2.6):
//! every leaf's Q32 tip and area deposited in W at its eight corners
//! (`fixed::feedback::lattice`), the one f64 sweep run on the lattice and
//! its light rounded once to Q32, and every read sampled in integers. A
//! point or an area enters as the Q32 it holds exactly (or the nearest,
//! for a value not yet on the grid); a read returns the exact real of its
//! Q32 light.
use super::CELL;
use crate::fixed::feedback::lattice::{rounded, Grid};
use crate::fixed::{q32, ONE32};
use crate::structure::Vec3;

/// A lattice of light: leaf area deposited, swept from the sky, read.
pub(crate) struct Field {
    grid: Grid,
    /// Leaf area density at each node, W, then the light there, Q32.
    density: Vec<u128>,
    light: Vec<i64>,
}

/// A point as Q32.
pub(crate) fn q(p: Vec3) -> [i64; 3] {
    [q32(p.x), q32(p.y), q32(p.z)]
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
        let mut field = Self::spanning(
            leaves.iter().map(|l| l.0).chain(reads.iter().copied()),
            CELL,
        );
        field.deposit_all(leaves);
        field.sweep(light.extinction, &light.directions(), threads);
        field
    }

    /// The lattice `leaves` deposit in the box spanning them and `reads`,
    /// before any sweep (probe (a): any grouping of the leaves gives it).
    pub(crate) fn deposited(leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Vec<u128> {
        let points = leaves.iter().map(|l| l.0).chain(reads.iter().copied());
        let mut field = Self::spanning(points, CELL);
        field.deposit_all(leaves);
        field.density
    }

    /// The field of a lattice already deposited on `grid` (fn-211 task
    /// 10: its leaves deposited by more than one executor), swept.
    pub(crate) fn of_lattice(
        light: &super::Light,
        grid: Grid,
        density: Vec<u128>,
        threads: usize,
    ) -> Self {
        let mut field = Self {
            grid,
            density,
            light: vec![ONE32; grid.nodes()],
        };
        field.sweep(light.extinction, &light.directions(), threads);
        field
    }

    /// Its box, its W densities and its Q32 light.
    pub(crate) fn parts(&self) -> (&Grid, &[u128], &[i64]) {
        (&self.grid, &self.density, &self.light)
    }

    /// The nodes it holds.
    pub fn nodes(&self) -> usize {
        self.grid.nodes()
    }

    /// The light at `at`: its corners' trilinear weights in Q62, the
    /// weight outside the box under the open sky's one, floored to Q32.
    pub fn at(&self, at: Vec3) -> f64 {
        self.grid.sample(&self.light, q(at)) as f64 / ONE32 as f64
    }

    /// The box at the contract's 0.5 m. The layout keeps every position
    /// inside the contract's domain (CONTRACT-A.md 2.2).
    pub(super) fn spanning(points: impl Iterator<Item = Vec3>, cell: f64) -> Self {
        assert_eq!(cell, CELL, "the lattice's nodes are 0.5 m apart");
        let grid = Grid::spanning(points.map(q)).expect("positions inside the contract's domain");
        Self {
            grid,
            density: vec![0; grid.nodes()],
            light: vec![ONE32; grid.nodes()],
        }
    }

    /// Every leaf's area deposited in integers: any order or grouping of
    /// the leaves gives the same lattice.
    pub(super) fn deposit_all(&mut self, leaves: &[(Vec3, f64)]) {
        for &(at, area) in leaves {
            self.grid
                .deposit(&mut self.density, q(at), q32(area))
                .expect("a leaf's area inside the contract's bound");
        }
    }

    /// The one f64 sweep (`super::swept_w`), its light rounded once to Q32.
    pub(super) fn sweep(&mut self, extinction: f64, directions: &[(Vec3, f64)], threads: usize) {
        if extinction <= 0.0 {
            return;
        }
        let size = self.grid.size.map(|s| s as usize);
        let light = super::swept_w(extinction, directions, size, &self.density, threads);
        self.light = light.into_iter().map(rounded).collect();
    }
}
