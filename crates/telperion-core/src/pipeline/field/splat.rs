//! The cell rule, record by record: each wood sweep and each leaf record of
//! the drawn tree written into the cells of a grid it reaches, and each
//! cell's limb system chosen from what every record left there.
use super::{checked, Grid, Occupancy};
use crate::{
    math::Vec3,
    pipeline::foliage::{
        plan::{Descriptor, Plan},
        Bounds,
    },
    tree::Tree,
    Error, Result,
};
use std::collections::HashMap;

/// A node's wood: a linearly tapered sphere sweep from its parent, a ball
/// at the root.
pub(super) struct Sweep {
    a: Vec3,
    b: Vec3,
    start: f64,
    end: f64,
}

/// Every node's wood, in node order.
pub(super) fn wood(tree: &Tree) -> impl Iterator<Item = Sweep> + '_ {
    tree.nodes.iter().map(|n| {
        let (a, start) = match n.parent {
            Some(p) => (tree.nodes[p as usize].position, n.start_radius),
            None => (n.position, n.radius),
        };
        Sweep {
            a,
            b: n.position,
            start,
            end: n.radius,
        }
    })
}

/// The box of balls of `r` about `a` and of `s` about `b`.
fn balls(a: Vec3, r: f64, b: Vec3, s: f64) -> Result<Bounds> {
    let (r, s) = (Vec3::new(r, r, r), Vec3::new(s, s, s));
    let (lo, hi) = (a - r, b - s);
    let (up, top) = (a + r, b + s);
    checked(Bounds {
        min: Vec3::new(lo.x.min(hi.x), lo.y.min(hi.y), lo.z.min(hi.z)),
        max: Vec3::new(up.x.max(top.x), up.y.max(top.y), up.z.max(top.z)),
    })
}

impl Sweep {
    pub(super) fn bounds(&self) -> Result<Bounds> {
        balls(self.a, self.start, self.b, self.end)
    }

    /// Whether the ball of `inflation` about `p` meets the sweep.
    fn contains(&self, p: Vec3, inflation: f64) -> bool {
        capsule(self.a, self.b, [self.start, self.end], p, inflation)
    }
}

/// Whether the ball of `inflation` about `p` meets the sweep from `a` to
/// `b` tapering from `radii[0]` to `radii[1]`.
fn capsule(a: Vec3, b: Vec3, radii: [f64; 2], p: Vec3, inflation: f64) -> bool {
    let d = b - a;
    let q = p - a;
    let r = radii[0] + inflation;
    let dr = radii[1] - radii[0];
    let aa = d.dot(d) - dr * dr;
    let bb = q.dot(d) + r * dr;
    let t = if aa > 0. {
        (bb / aa).clamp(0., 1.)
    } else if bb > aa * 0.5 {
        1.
    } else {
        0.
    };
    let distance = q - d * t;
    distance.dot(distance) <= (r + dr * t).powi(2)
}

/// A leaflet's box: three unit axes (along its midline, across it and its
/// normal), its extent along each, its centre and its world box.
struct Shape {
    axes: [Vec3; 3],
    extent: [f64; 3],
    centre: Vec3,
    bounds: Bounds,
}

impl Shape {
    /// The box about the midline `a`, `b`, `side` its half-width square to
    /// the midline and `thickness` its half-thickness square to both.
    fn new(a: Vec3, b: Vec3, side: Vec3, thickness: f64) -> Result<Self> {
        let breadth = side.length_squared();
        if breadth.is_nan() || breadth <= 0. {
            return Err(Error::InvalidInput("box width"));
        }
        let across = side.normalized();
        let run = (b - a) - across * (b - a).dot(across);
        let along = match run.length_squared() > 0. {
            true => run.normalized(),
            false => across.perpendicular(),
        };
        let axes = [along, across, along.cross(across)];
        let extent = [(b - a).dot(along) * 0.5, side.length(), thickness];
        let centre = (a + b) * 0.5;
        let reach = |u: Vec3| -> f64 { (0..3).map(|k| axes[k].dot(u).abs() * extent[k]).sum() };
        let r = Vec3::new(reach(Vec3::X), reach(Vec3::Y), reach(Vec3::Z));
        let bounds = checked(Bounds {
            min: centre - r,
            max: centre + r,
        })?;
        Ok(Self {
            axes,
            extent,
            centre,
            bounds,
        })
    }

    /// Whether the closed cube of `half` about `p` meets the box: the
    /// world's axes and the box's tried as separating axes. Where none
    /// separates the cell counts, so a cell only an edge pair would part
    /// is kept and the answer stays conservative.
    fn meets(&self, p: Vec3, half: f64) -> bool {
        let b = &self.bounds;
        let world = p.x + half >= b.min.x
            && p.x - half <= b.max.x
            && p.y + half >= b.min.y
            && p.y - half <= b.max.y
            && p.z + half >= b.min.z
            && p.z - half <= b.max.z;
        world
            && (0..3).all(|k| {
                let u = self.axes[k];
                let (c, r) = (p.dot(u), half * (u.x.abs() + u.y.abs() + u.z.abs()));
                let at = self.centre.dot(u);
                c + r >= at - self.extent[k] && c - r <= at + self.extent[k]
            })
    }
}

/// A leaf-bearing record of the plan: its midline, its reach about it, a
/// leaflet's box where it is one, its stations and its limb system.
pub(super) struct Leaf {
    a: Vec3,
    b: Vec3,
    reach: f64,
    shape: Option<Shape>,
    count: f64,
    system: u32,
}

impl Leaf {
    pub(super) fn of(plan: &Plan, d: &Descriptor) -> Result<Self> {
        let reach = plan.reach(d);
        if !reach.is_finite() || reach < 0. {
            return Err(Error::InvalidInput("foliage reach"));
        }
        let [a, b] = d.endpoints;
        let shape = (d.side != Vec3::ZERO)
            .then(|| Shape::new(a, b, d.side, reach))
            .transpose()?;
        Ok(Self {
            a,
            b,
            reach,
            shape,
            count: f64::from(d.count),
            system: d.system,
        })
    }

    pub(super) fn bounds(&self) -> Result<Bounds> {
        match &self.shape {
            Some(s) => Ok(s.bounds),
            None => balls(self.a, self.reach, self.b, self.reach),
        }
    }

    /// Whether the cell of `half` about `p` meets the record: a capsule by
    /// the cell's circumsphere, `inflation`; a box cube to box.
    fn meets(&self, p: Vec3, half: f64, inflation: f64) -> bool {
        match &self.shape {
            Some(s) => s.meets(p, half),
            None => capsule(self.a, self.b, [self.reach; 2], p, inflation),
        }
    }

    /// The share of the midline inside the closed cube of `half` about
    /// `c`; a point midline counts whole where it lies inside.
    fn share_inside(&self, c: Vec3, half: f64) -> f64 {
        let (mut t0, mut t1) = (0_f64, 1_f64);
        let (a, d) = (self.a, self.b - self.a);
        for (a, d, c) in [(a.x, d.x, c.x), (a.y, d.y, c.y), (a.z, d.z, c.z)] {
            if d == 0. {
                if (a - c).abs() > half {
                    return 0.;
                }
                continue;
            }
            let (ta, tb) = ((c - half - a) / d, (c + half - a) / d);
            t0 = t0.max(ta.min(tb));
            t1 = t1.min(ta.max(tb));
        }
        (t1 - t0).max(0.)
    }
}

/// The cells of `grid` whose centres may lie within `b` grown by `pad`.
fn reached(grid: &Grid, b: Bounds, pad: f64) -> impl Iterator<Item = usize> {
    let f = grid.frame;
    let n = f.n as usize;
    // A hair wider: the cell rule decides, never the range.
    let pad = pad + f.cell * 1e-3;
    let span = |lo: f64, hi: f64, o: f64| {
        let first = ((lo - pad - o) / f.cell - 0.5).ceil().max(0.0);
        let last = ((hi + pad - o) / f.cell - 0.5).floor().min(n as f64 - 1.0);
        first as usize..(last + 1.0).max(first) as usize
    };
    let (xs, ys, zs) = (
        span(b.min.x, b.max.x, f.origin.x),
        span(b.min.y, b.max.y, f.origin.y),
        span(b.min.z, b.max.z, f.origin.z),
    );
    zs.flat_map(move |k| {
        let xs = xs.clone();
        ys.clone()
            .flat_map(move |j| xs.clone().map(move |i| i + n * (j + n * k)))
    })
}

/// `w` written into every cell whose circumsphere meets it.
pub(super) fn wood_into(grid: &mut Grid, w: &Sweep) -> Result<()> {
    let half = grid.frame.cell * 0.5;
    let inflation = half * 3_f64.sqrt();
    for c in reached(grid, w.bounds()?, inflation) {
        if w.contains(grid.frame.centre(c), inflation) {
            let cell = &mut grid.cells[c];
            cell.wood = true;
            cell.wood_radius = cell.wood_radius.max(w.start.max(w.end));
        }
    }
    Ok(())
}

/// Each cell's estimate on each limb system it holds leaves of.
#[derive(Default)]
pub(super) struct Tally(HashMap<u64, f64>);

impl Tally {
    /// The limb system with the most leaves in each cell, the lower id on
    /// a tie, whatever order the records came in.
    pub(super) fn owners(self, cells: &mut [Occupancy]) {
        let mut all: Vec<(u64, f64)> = self.0.into_iter().collect();
        all.sort_unstable_by_key(|e| e.0);
        let mut best: Option<(usize, f64)> = None;
        for (key, sum) in all {
            let (c, system) = ((key >> 32) as usize, key as u32);
            if best.is_none_or(|(at, most)| at != c || sum > most) {
                cells[c].limb = Some(system);
                best = Some((c, sum));
            }
        }
    }
}

/// `leaf` written into every cell it meets: foliage, its share of the
/// record's stations, on its limb system.
pub(super) fn leaf_into(grid: &mut Grid, leaf: &Leaf, tally: &mut Tally) -> Result<()> {
    let half = grid.frame.cell * 0.5;
    let inflation = half * 3_f64.sqrt();
    for c in reached(grid, leaf.bounds()?, inflation) {
        let p = grid.frame.centre(c);
        if !leaf.meets(p, half, inflation) {
            continue;
        }
        let estimate = leaf.count * leaf.share_inside(p, half);
        let cell = &mut grid.cells[c];
        cell.foliage = true;
        cell.leaves += estimate;
        let t = &mut tally.0;
        t.try_reserve(1)
            .map_err(|_| Error::ResourceLimit("field allocation"))?;
        *t.entry((c as u64) << 32 | u64::from(leaf.system)).or_default() += estimate;
    }
    Ok(())
}
