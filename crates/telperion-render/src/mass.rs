//! How deep a leaf stands inside its own leaf mass. The crown's placements
//! are counted into a grid over the box they fill, once when the tree goes
//! up, and every cell reads how full the cells straight over it are, up to a
//! clump's height and no further than the first empty one: a gap between two
//! limb systems ends the mass above it. A leaf on a clump's face has little
//! of its own mass over it and one hanging under the clump has all of it,
//! where the crown's ellipsoid reads one smooth depth for both. The vertex
//! stage looks its placement's cell up by position; nothing is recomputed a
//! frame.
use telperion_core::surface::Bounds;

/// Cells along the crown's longest side, at most: a crown too sparse to put
/// `FILL` leaves in a cell on the average gets coarser cells, so an empty cell
/// is a gap in the mass and not the space between two leaves.
pub const CELLS: usize = 64;
const FILL: f64 = 8.0;
/// The share of the crown's longest side a leaf looks up through: a clump's
/// height.
const REACH: f64 = 0.125;
/// Floats ahead of the cells: the grid's corner, its cell's edge and its
/// three counts, and one to keep the cells on a four-float boundary.
pub const HEADER: usize = 8;

/// A grid of no cells, which reads no depth anywhere: the leaf view's, and a
/// crown with no leaves.
pub fn empty() -> Vec<f32> {
    vec![0.0; HEADER]
}

/// The grid for a crown: the header, then each cell's depth in its own mass,
/// 0 with nothing over it to 1 under a full clump, x fastest, then y, then z.
pub fn grid(matrices: &[[f32; 16]], crown: Option<Bounds>) -> Vec<f32> {
    let Some(crown) = crown else { return empty() };
    let size = crown.max - crown.min;
    let cells = (matrices.len() as f64 / FILL)
        .cbrt()
        .clamp(1.0, CELLS as f64)
        .floor();
    let edge = size.x.max(size.y).max(size.z) / cells;
    if matrices.is_empty() || !edge.is_finite() || edge <= 0.0 {
        return empty();
    }
    let reach = ((cells * REACH).round() as usize).max(1);
    let count = |extent: f64| ((extent / edge).floor() as usize + 1).min(CELLS);
    let n = [count(size.x), count(size.y), count(size.z)];
    let cell = |m: &[f32; 16]| {
        let at = |k: usize, lo: f64| {
            (((f64::from(m[12 + k]) - lo) / edge).floor().max(0.0) as usize).min(n[k] - 1)
        };
        at(0, crown.min.x) + n[0] * (at(1, crown.min.y) + n[1] * at(2, crown.min.z))
    };
    let mut counts = vec![0u32; n[0] * n[1] * n[2]];
    for m in matrices {
        counts[cell(m)] += 1;
    }
    let occupied = counts.iter().filter(|&&c| c > 0).count().max(1);
    let mean = matrices.len() as f64 / occupied as f64;
    let mut out = Vec::with_capacity(HEADER + counts.len());
    out.extend([crown.min.x, crown.min.y, crown.min.z, edge].map(|v| v as f32));
    out.extend(n.map(|v| v as f32));
    out.push(0.0);
    for z in 0..n[2] {
        for y in 0..n[1] {
            for x in 0..n[0] {
                let at = |y: usize| counts[x + n[0] * (y + n[1] * z)];
                let mut over = 0.0;
                if at(y) > 0 {
                    for above in (y + 1..n[1]).take(reach) {
                        let c = at(above);
                        if c == 0 {
                            break;
                        }
                        over += (f64::from(c) / mean).min(1.0);
                    }
                }
                out.push((over / reach as f64) as f32);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use telperion_core::math::Vec3;

    fn at(x: f32, y: f32, z: f32) -> [f32; 16] {
        [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, x, y, z, 1.0,
        ]
    }

    fn depth(grid: &[f32], p: [f32; 3]) -> f32 {
        let n = [grid[4], grid[5], grid[6]].map(|v| v as usize);
        let c: Vec<usize> = (0..3)
            .map(|k| (((p[k] - grid[k]) / grid[3]).floor().max(0.0) as usize).min(n[k] - 1))
            .collect();
        grid[HEADER + c[0] + n[0] * (c[1] + n[1] * c[2])]
    }

    fn bounds(leaves: &[[f32; 16]]) -> Option<Bounds> {
        let p = |m: &[f32; 16]| Vec3::new(m[12].into(), m[13].into(), m[14].into());
        leaves.iter().map(p).fold(None, |b, q| {
            Some(match b {
                None => Bounds { min: q, max: q },
                Some(b) => Bounds {
                    min: Vec3::new(b.min.x.min(q.x), b.min.y.min(q.y), b.min.z.min(q.z)),
                    max: Vec3::new(b.max.x.max(q.x), b.max.y.max(q.y), b.max.z.max(q.z)),
                },
            })
        })
    }

    /// A column of leaves 512 to the unit of height over `from..to`.
    fn column(from: f32, to: f32) -> Vec<[f32; 16]> {
        let count = ((to - from) * 512.0) as usize;
        (0..count)
            .map(|k| at(0.0, from + (k as f32 + 0.5) / 512.0, 0.0))
            .collect()
    }

    #[test]
    fn a_leaf_under_its_own_lobe_is_deeper_than_one_on_its_face() {
        // Two lobes, one over the other with a gap between them wider than
        // any cell: the lower lobe's face reads nothing of the upper one, and
        // its underside reads a whole lobe's height of its own mass over it.
        let mut leaves = column(0.0, 16.0);
        leaves.extend(column(32.0, 64.0));
        let grid = grid(&leaves, bounds(&leaves));
        let edge = grid[3];
        let under = depth(&grid, [0.0, 0.5, 0.0]);
        let face = depth(&grid, [0.0, 15.9, 0.0]);
        let between = depth(&grid, [0.0, 15.9 - edge, 0.0]);
        assert!((under - 1.0).abs() < 1e-6, "under its lobe: {under}");
        assert_eq!(face, 0.0, "a gap ends the lobe over the face");
        assert!(
            face < between && between < under,
            "{face} {between} {under}"
        );
        // And the lower lobe's underside does not read the upper lobe.
        assert!(depth(&grid, [0.0, 40.0, 0.0]) > 0.9);
    }

    #[test]
    fn no_crown_and_no_leaves_read_no_depth() {
        assert_eq!(grid(&[], None), empty());
        assert_eq!(grid(&[], bounds(&[at(0.0, 0.0, 0.0)])), empty());
        // One leaf fills a box of no size, which has no cells to count into.
        let one = [at(1.0, 2.0, 3.0)];
        assert_eq!(grid(&one, bounds(&one)), empty());
    }
}
