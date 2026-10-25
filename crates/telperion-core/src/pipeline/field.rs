//! The field (fn-257 R8): a read-out of the drawn tree to a grid, a
//! consumer of the drawn stage as the renderer is, with no form of its own
//! between the records and the cells. Metres, Y up.
//!
//! The wood is each solved node's linearly tapered sphere sweep from its
//! parent (rounded ends, no bark displacement). The leaves are the leaf
//! plan's leaf-bearing segments, each with a conservative reach about it,
//! its station count and its limb system; a frond's leaflet is an oriented
//! box about its midline. A family the plan cannot describe has no field.
//!
//! Every record writes the cells it reaches by the cell rule: wood where
//! the cell's circumsphere meets its sweep, leaves where the circumsphere
//! (a box: the cube itself) meets their record, counted as the share of
//! the record's midline inside the closed cube. The circumsphere
//! overestimates by at most a cell's half diagonal; finer grids shrink it.
//!
//! The flowers (fn-316) are the florets the drawn tree hangs, at the
//! window's middle: each one counted in the cell its seat lies in.
#[cfg(feature = "geometry")]
mod splat;
use crate::{
    math::Vec3,
    pipeline::foliage::{withered, Bounds, Instances},
    Error, Result,
};
#[cfg(feature = "geometry")]
use crate::{pipeline::foliage::plan::Plan, tree::Tree};

/// One cell's answer. `wood_radius` is the larger end radius of the thickest
/// wood sweep reaching the cell, in metres, zero where no wood does.
/// `leaves` estimates the stations in the cell before the crown-shell cull:
/// over the grid the estimates sum to the plan's total. `limb` is the limb
/// system with the largest estimate in the cell, the lower id on a tie, and
/// `None` where no foliage reaches; each package entry names a system by
/// its densest cell (`Grid::name_limbs`). `flowers` counts the florets whose
/// seat lies in the cell: over the grid they sum to the tree's florets.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Occupancy {
    pub wood: bool,
    pub wood_radius: f64,
    pub foliage: bool,
    pub leaves: f64,
    pub limb: Option<u32>,
    pub flowers: f64,
}

/// The grid a field is read to (fn-257, "The grid"): `n` cells a side,
/// cubes of `cell` metres, `n - 2` of them spanning the box's longest axis,
/// centred on the box in x and z with one empty cell below its lowest
/// point. Cell `i + n * (j + n * k)` is centred at `origin + (i + 0.5,
/// j + 0.5, k + 0.5) * cell`. The CPU, the device and the site's voxelizer
/// read this one definition.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Frame {
    pub n: u32,
    pub cell: f64,
    pub origin: Vec3,
}

impl Frame {
    /// `n` where it is a side a grid may have: a whole number from 3 to 256.
    pub fn side(n: u32) -> Result<u32> {
        match n {
            3..=256 => Ok(n),
            _ => Err(Error::InvalidValue {
                field: "field grid side",
                value: n.to_string(),
            }),
        }
    }

    /// The grid of `n` cells a side over `b`.
    pub fn over(b: Bounds, n: u32) -> Result<Self> {
        let n = Self::side(n)?;
        let size = b.max - b.min;
        let cell = size.x.max(size.y).max(size.z) / f64::from(n - 2);
        if !(cell.is_finite() && cell > 0.0 && b.min.is_finite()) {
            return Err(Error::InvalidInput("field bounds"));
        }
        let side = f64::from(n) * cell;
        let centre = (b.min + b.max) * 0.5;
        Ok(Self {
            n,
            cell,
            origin: Vec3::new(centre.x - side * 0.5, b.min.y - cell, centre.z - side * 0.5),
        })
    }

    /// The grid's cells.
    pub fn cells(&self) -> usize {
        (self.n as usize).pow(3)
    }

    /// The centre of cell `index`.
    pub fn centre(&self, index: usize) -> Vec3 {
        let n = self.n as usize;
        let at = |i: usize| (i as f64 + 0.5) * self.cell;
        self.origin + Vec3::new(at(index % n), at(index / n % n), at(index / (n * n)))
    }
}

/// A field read out: its frame and one answer a cell, in cell order.
#[derive(Debug, Clone, PartialEq)]
pub struct Grid {
    pub frame: Frame,
    pub cells: Vec<Occupancy>,
}

impl Grid {
    /// Each limb system renamed by the cell where it holds the most leaves,
    /// the lowest on a tie (fn-317 R4c): each executor names a system by
    /// its own node, so the same partition read on the CPU and on the
    /// device carries the same names only once both rename it here. A
    /// system's densest cell lies inside it, where the executors agree,
    /// not on the edge it shares with a neighbour. Both package entries
    /// call it on the grid they hand on; inside, a system keeps its node's
    /// name, the same on every grid of one executor. Names stay opaque.
    pub fn name_limbs(&mut self) {
        let mut densest = std::collections::HashMap::<u32, (f64, u32)>::new();
        for (i, cell) in self.cells.iter().enumerate() {
            if let Some(limb) = cell.limb {
                let best = densest.entry(limb).or_insert((cell.leaves, i as u32));
                if cell.leaves > best.0 {
                    *best = (cell.leaves, i as u32);
                }
            }
        }
        for cell in &mut self.cells {
            if let Some(limb) = &mut cell.limb {
                *limb = densest[limb].1;
            }
        }
    }
}

#[cfg(feature = "geometry")]
/// The drawn tree's field over its own box at `n` cells a side, its
/// `organ`'s florets counted where it flowers.
pub(crate) fn field(tree: &Tree, plan: &Plan, organ: Option<&Instances>, n: u32) -> Result<Grid> {
    let mut b = bounds(tree, plan)?;
    if let Some(s) = organ.and_then(seat_bounds) {
        b = Some(union(b, s));
    }
    let b = b.ok_or(Error::InvalidInput("field of an empty tree"))?;
    let mut grid = read(tree, plan, Frame::over(b, n)?)?;
    if let Some(organ) = organ {
        florets_into(&mut grid, organ)?;
    }
    Ok(grid)
}

/// Where each of `organ`'s florets hangs from: its instance's stored
/// position, its rachis instances left out.
pub fn seats(organ: &Instances) -> impl Iterator<Item = Vec3> + '_ {
    let r = organ.reference;
    (organ.leaves.iter())
        .filter(|l| !withered(**l))
        .map(move |l| r.position(*l))
}

/// The box `organ`'s seats stand in, none where it has no floret.
pub fn seat_bounds(organ: &Instances) -> Option<Bounds> {
    let point = |p: Vec3| Bounds { min: p, max: p };
    seats(organ).fold(None, |b, p| {
        Some(match b {
            None => point(p),
            Some(b) => union(Some(b), point(p)),
        })
    })
}

/// `organ`'s florets counted into `grid`, each in the cell its seat lies
/// in (`floor` of its offset from the origin over the cell). A seat
/// outside the grid is refused.
pub fn florets_into(grid: &mut Grid, organ: &Instances) -> Result<()> {
    let f = grid.frame;
    let n = i64::from(f.n);
    for p in seats(organ) {
        let at = |v: f64, o: f64| ((v - o) / f.cell).floor() as i64;
        let [i, j, k] = [
            at(p.x, f.origin.x),
            at(p.y, f.origin.y),
            at(p.z, f.origin.z),
        ];
        if ![i, j, k].iter().all(|c| (0..n).contains(c)) {
            return Err(Error::InvalidInput("a floret outside the field"));
        }
        grid.cells[(i + n * (j + n * k)) as usize].flowers += 1.0;
    }
    Ok(())
}

#[cfg(feature = "geometry")]
/// The box every wood sweep and leaf record of the tree stands in, none
/// where it has neither.
pub fn bounds(tree: &Tree, plan: &Plan) -> Result<Option<Bounds>> {
    tree.validate_solved()?;
    let mut out: Option<Bounds> = None;
    for w in splat::wood(tree) {
        out = Some(union(out, w.bounds()?));
    }
    for d in &plan.descriptors {
        out = Some(union(out, splat::Leaf::of(plan, d)?.bounds()?));
    }
    Ok(out)
}

#[cfg(feature = "geometry")]
/// The drawn tree's field read to `frame`: every wood sweep and every leaf
/// record written into the cells it reaches.
pub fn read(tree: &Tree, plan: &Plan, frame: Frame) -> Result<Grid> {
    tree.validate_solved()?;
    let mut cells = Vec::new();
    cells
        .try_reserve_exact(frame.cells())
        .map_err(|_| Error::ResourceLimit("field allocation"))?;
    cells.resize(frame.cells(), Occupancy::default());
    let mut grid = Grid { frame, cells };
    for w in splat::wood(tree) {
        splat::wood_into(&mut grid, &w)?;
    }
    let mut tally = splat::Tally::default();
    for d in &plan.descriptors {
        splat::leaf_into(&mut grid, &splat::Leaf::of(plan, d)?, &mut tally)?;
    }
    tally.owners(&mut grid.cells);
    Ok(grid)
}

/// `a` grown to hold `b`.
fn union(a: Option<Bounds>, b: Bounds) -> Bounds {
    let Some(a) = a else { return b };
    let (lo, hi) = (a.min, a.max);
    Bounds {
        min: Vec3::new(lo.x.min(b.min.x), lo.y.min(b.min.y), lo.z.min(b.min.z)),
        max: Vec3::new(hi.x.max(b.max.x), hi.y.max(b.max.y), hi.z.max(b.max.z)),
    }
}

#[cfg(feature = "geometry")]
/// `b` where its arithmetic stays representable: squared distances and dot
/// products of its coordinates too.
fn checked(b: Bounds) -> Result<Bounds> {
    let limit = (f64::MAX / 64.).sqrt();
    let all = [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z];
    match all.iter().all(|v| v.is_finite() && v.abs() <= limit) {
        true => Ok(b),
        false => Err(Error::ResourceLimit("field coordinate overflow")),
    }
}
