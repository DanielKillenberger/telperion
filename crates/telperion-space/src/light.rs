//! Light (fn-197): Beer–Lambert from the sky through a coarse lattice of
//! leaf area. Leaves are deposited at the lattice's nodes by trilinear
//! weights; each sky direction is swept down the lattice a layer at a time,
//! carrying the optical depth along its ray, read bilinearly in the layer
//! above; a node's light is the sky's directions' transmittances, each
//! weighted by its share of the sky's light on a level surface. The lattice
//! is anchored at the world's origin, so its extent changes no value, and
//! every deposit and read moves by degree as its point does.
//!
//! The law is reproduced against closed forms (`light/tests.rs`): a leaf
//! slab's exp(-k L) (Monsi and Saeki 1953), a sphere of even leaf density
//! along any ray, and GreenLab's production Sp (1 - exp(-k S / Sp)) on an
//! even crown under light from overhead (Letort et al., eq. 1).
use crate::error::{refuse, Result};
use crate::structure::{Structure, Vec3};

/// The leaf area a node bears in square metres: one leaf of an oak's or a
/// beech's size. A stand-in until the species name theirs (fn-197 step 3).
pub(crate) const NODE_LEAF: f64 = 0.005;
/// The lattice's spacing in metres.
pub(crate) const CELL: f64 = 0.5;
/// The largest extinction coefficient the engine draws.
const MAX_EXTINCTION: f64 = 10.0;
/// The sky's directions: overhead, a ring at 60 degrees' elevation and one
/// at 30, four azimuths each, the lower ring turned by 45 degrees.
const RINGS: [(f64, f64); 2] = [(60.0, 0.0), (30.0, 45.0)];
/// The elevations, in degrees, between which each direction gathers the
/// sky: overhead above 75, the high ring down to 45, the low ring to the
/// horizon.
const BANDS: [f64; 4] = [90.0, 75.0, 45.0, 0.0];

/// The light model: a site's settings, not a species' (host, 2026-10-05).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Light {
    /// Beer–Lambert's extinction coefficient k: the optical depth a square
    /// metre of leaf in a cubic metre lends a metre of ray (0.5 for leaves
    /// at every angle). Neutral 0: no leaf shades.
    pub extinction: f64,
    /// Where the sky's light comes from: 0 all from overhead, 0.5 the
    /// standard overcast sky (radiance as 1 + 2 sin of the elevation), 1 a
    /// uniform overcast sky; between, the two nearest blended. Dormant
    /// without extinction.
    pub sky: f64,
}

impl Default for Light {
    fn default() -> Self {
        Self::NEUTRAL
    }
}

impl Light {
    /// No shade, under the standard overcast sky.
    pub const NEUTRAL: Self = Self {
        extinction: 0.0,
        sky: 0.5,
    };

    pub(crate) fn validate(&self) -> Result<()> {
        if !(0.0..=MAX_EXTINCTION).contains(&self.extinction) {
            return refuse("light.extinction", "an extinction lies in 0 to 10");
        }
        if !(0.0..=1.0).contains(&self.sky) {
            return refuse("light.sky", "a share lies in 0 to 1");
        }
        Ok(())
    }

    /// Whether any leaf shades: none, and light is 1 everywhere.
    pub(crate) fn shades(&self) -> bool {
        self.extinction > 0.0
    }

    /// The sky's directions, unit vectors towards it, and each one's share
    /// of the light on a level surface; the shares sum to 1.
    pub(crate) fn directions(&self) -> Vec<(Vec3, f64)> {
        let s = self.sky;
        let (zenith, standard, uniform) = (overhead(), bands(standard), bands(uniform));
        let blend = |i: usize| {
            if s <= 0.5 {
                (1.0 - 2.0 * s) * zenith[i] + 2.0 * s * standard[i]
            } else {
                (2.0 - 2.0 * s) * standard[i] + (2.0 * s - 1.0) * uniform[i]
            }
        };
        let mut out = vec![(Vec3::new(0.0, 0.0, 1.0), blend(0))];
        for (ring, &(elevation, turn)) in RINGS.iter().enumerate() {
            let (e, share) = (elevation.to_radians(), blend(ring + 1) / 4.0);
            for k in 0..4 {
                let a = (turn + 90.0 * f64::from(k)).to_radians();
                let toward = Vec3::new(e.cos() * a.cos(), e.cos() * a.sin(), e.sin());
                out.push((toward, share));
            }
        }
        out
    }
}

/// The light at each living apex's tip of `structure`, by its axis's
/// index, from the leaves its last cycle grew, one leaf a node sized by
/// its phytomer's scale: what a bud there would read before the next
/// cycle. The measure of a rough layout against the final one.
pub fn bud_light(structure: &Structure, light: &Light) -> Vec<(usize, f64)> {
    let leaves: Vec<(Vec3, f64)> = structure
        .axes
        .iter()
        .flat_map(|a| &a.phytomers)
        .filter(|p| p.cycle == structure.age)
        .map(|p| (p.tip, p.scale * NODE_LEAF))
        .collect();
    let tips: Vec<(usize, Vec3)> = structure
        .axes
        .iter()
        .enumerate()
        .filter(|(_, a)| a.apex_end.is_none())
        .map(|(i, a)| (i, a.phytomers.last().map_or(a.base, |p| p.tip)))
        .collect();
    let reads: Vec<Vec3> = tips.iter().map(|t| t.1).collect();
    let field = Field::new(light, &leaves, &reads);
    tips.iter().map(|&(i, at)| (i, field.at(at))).collect()
}

/// All the light from overhead.
fn overhead() -> [f64; 3] {
    [1.0, 0.0, 0.0]
}

/// Each band's share of a sky's light on a level surface, from the light
/// gathered below an elevation (`gathered`, 0 at the horizon, 1 overhead).
fn bands(gathered: fn(f64) -> f64) -> [f64; 3] {
    let g = |degrees: f64| gathered(degrees.to_radians());
    [
        g(BANDS[0]) - g(BANDS[1]),
        g(BANDS[1]) - g(BANDS[2]),
        g(BANDS[2]) - g(BANDS[3]),
    ]
}

/// A uniform sky's light on a level surface from below elevation `e`:
/// the integral of sin e cos e, normalised.
fn uniform(e: f64) -> f64 {
    e.sin().powi(2)
}

/// The standard overcast sky's (radiance as 1 + 2 sin e): the integral of
/// (1 + 2 sin e) sin e cos e, sin^2 e / 2 + 2 sin^3 e / 3, normalised by
/// its value overhead, 7 / 6.
fn standard(e: f64) -> f64 {
    let s = e.sin();
    (s * s / 2.0 + 2.0 * s.powi(3) / 3.0) / (7.0 / 6.0)
}

/// A lattice of light: leaf area deposited, swept from the sky, read.
pub(crate) struct Field {
    /// The node index of the box's least corner, and its extent in nodes.
    low: [i64; 3],
    size: [usize; 3],
    /// Leaf area density at each node, then the light there.
    density: Vec<f64>,
    light: Vec<f64>,
}

impl Field {
    /// The field of `leaves` (each a point and its leaf area in square
    /// metres) under `light`, over a box that holds every leaf and every
    /// point in `reads`.
    pub fn new(light: &Light, leaves: &[(Vec3, f64)], reads: &[Vec3]) -> Self {
        let mut field = Self::spanning(leaves.iter().map(|l| l.0).chain(reads.iter().copied()));
        for &(at, area) in leaves {
            field.deposit(at, area);
        }
        field.sweep(light.extinction, &light.directions());
        field
    }

    /// The light at `at`, trilinear between the nodes about it; 1 beyond
    /// the box, where nothing stands between it and the sky.
    pub fn at(&self, at: Vec3) -> f64 {
        let mut sum = 0.0;
        let mut weight = 0.0;
        for (node, w) in corners(at) {
            if let Some(i) = self.index(node) {
                sum += w * self.light[i];
                weight += w;
            }
        }
        sum + (1.0 - weight)
    }

    fn spanning(points: impl Iterator<Item = Vec3>) -> Self {
        let (mut low, mut high) = ([i64::MAX; 3], [i64::MIN; 3]);
        for p in points {
            for (axis, c) in [p.x, p.y, p.z].into_iter().enumerate() {
                let n = (c / CELL).floor() as i64;
                low[axis] = low[axis].min(n);
                high[axis] = high[axis].max(n + 1);
            }
        }
        if low[0] > high[0] {
            (low, high) = ([0; 3], [0; 3]);
        }
        let size = [0, 1, 2].map(|a| (high[a] - low[a] + 1) as usize);
        let cells = size[0] * size[1] * size[2];
        Self {
            low,
            size,
            density: vec![0.0; cells],
            light: vec![1.0; cells],
        }
    }

    fn index(&self, node: [i64; 3]) -> Option<usize> {
        let mut at = [0usize; 3];
        for a in 0..3 {
            let n = node[a] - self.low[a];
            if n < 0 || n as usize >= self.size[a] {
                return None;
            }
            at[a] = n as usize;
        }
        Some((at[2] * self.size[1] + at[1]) * self.size[0] + at[0])
    }

    fn deposit(&mut self, at: Vec3, area: f64) {
        let density = area / CELL.powi(3);
        for (node, w) in corners(at) {
            let i = self.index(node).expect("the box holds every leaf");
            self.density[i] += w * density;
        }
    }

    /// Each direction's optical depth carried down a layer at a time: a
    /// node's is the depth at the point its ray meets the layer above, read
    /// bilinearly, plus the trapezoid of the density between them along a
    /// layer's length of ray.
    fn sweep(&mut self, extinction: f64, directions: &[(Vec3, f64)]) {
        if extinction <= 0.0 {
            return;
        }
        let [nx, ny, nz] = self.size;
        let layer = nx * ny;
        self.light.iter_mut().for_each(|l| *l = 0.0);
        let mut above_depth = vec![0.0; layer];
        let mut depth = vec![0.0; layer];
        for &(toward, share) in directions.iter().filter(|d| d.1 > 0.0) {
            let (ox, oy) = (toward.x / toward.z, toward.y / toward.z);
            let path = CELL / toward.z;
            above_depth.iter_mut().for_each(|d| *d = 0.0);
            for k in (0..nz).rev() {
                let here = &self.density[k * layer..(k + 1) * layer];
                let above = (k + 1 < nz).then(|| &self.density[(k + 1) * layer..(k + 2) * layer]);
                for j in 0..ny {
                    for i in 0..nx {
                        let (x, y) = (i as f64 + ox, j as f64 + oy);
                        let (d0, rho0) = match above {
                            Some(a) => (
                                bilinear(&above_depth, nx, ny, x, y),
                                bilinear(a, nx, ny, x, y),
                            ),
                            None => (0.0, 0.0),
                        };
                        let rho = here[j * nx + i];
                        depth[j * nx + i] = d0 + extinction * path * (rho + rho0) / 2.0;
                    }
                }
                let lit = &mut self.light[k * layer..(k + 1) * layer];
                for (l, &d) in lit.iter_mut().zip(&depth) {
                    *l += share * (-d).exp();
                }
                std::mem::swap(&mut above_depth, &mut depth);
            }
        }
    }
}

/// The value at fractional node (x, y) of a layer, bilinear; 0 beyond it.
fn bilinear(layer: &[f64], nx: usize, ny: usize, x: f64, y: f64) -> f64 {
    let (fx, fy) = (x.floor(), y.floor());
    let (tx, ty) = (x - fx, y - fy);
    let mut sum = 0.0;
    for (dx, wx) in [(0, 1.0 - tx), (1, tx)] {
        for (dy, wy) in [(0, 1.0 - ty), (1, ty)] {
            let (i, j) = (fx as i64 + dx, fy as i64 + dy);
            if i >= 0 && j >= 0 && (i as usize) < nx && (j as usize) < ny {
                sum += wx * wy * layer[j as usize * nx + i as usize];
            }
        }
    }
    sum
}

/// The eight nodes about `at` and their trilinear weights.
fn corners(at: Vec3) -> impl Iterator<Item = ([i64; 3], f64)> {
    let f = [at.x / CELL, at.y / CELL, at.z / CELL];
    let n = f.map(|c| c.floor());
    let t = [f[0] - n[0], f[1] - n[1], f[2] - n[2]];
    (0..8).map(move |c: usize| {
        let mut node = [0i64; 3];
        let mut w = 1.0;
        for a in 0..3 {
            let up = (c >> a) & 1;
            node[a] = n[a] as i64 + up as i64;
            w *= if up == 1 { t[a] } else { 1.0 - t[a] };
        }
        (node, w)
    })
}

#[cfg(test)]
mod tests;
