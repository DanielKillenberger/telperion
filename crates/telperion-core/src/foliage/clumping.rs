//! Limb systems clump their leaves. Every leaf belongs to the limb system of
//! the wood that bears it: the axis the scaffold grew on a first-order limb,
//! with everything that axis carries, or the limb's own run, or the trunk's.
//! The crown is parted into the rooms nearest each system's centre, and the
//! leaves are thinned toward the walls between rooms - the planes halfway
//! between two centres - so each system keeps a rounded leaf mass of its own
//! with a gap between it and the next. The row is how far from a wall its gap
//! reaches, as a share of the way to the centre; at zero nothing is touched.
use super::Instances;
use crate::{
    math::Vec3,
    tree::{NodeKind, Tree},
};

/// The deepest scaffold order that starts a limb system of its own: the
/// trunk is order zero, the limbs it bears one, and the axes those bear two.
/// An axis deeper than this belongs to the system of the order-two axis that
/// carries it, and so does every twig and branch the local layer grew on it.
const SYSTEM_ORDER: u32 = 2;
/// The neighbours a system's leaves are measured against. A boundary further
/// off than the twelfth nearest centre is shared with no leaf of this system.
const NEIGHBOURS: usize = 12;

/// The system every node belongs to, named by the node its system starts at.
/// At a scaffold fork the thickest child carries its parent's axis on and
/// every other child opens a lateral one, an order deeper; a lateral of order
/// two or less starts a system, and so does a stem leaving the root. Every
/// other node, and all the wood the local layer grew, is its parent's. The
/// rule reads only the solved radii, so a tree rebuilt from its record for
/// the growth view falls into the same systems as the one it was grown as.
pub(super) fn systems(tree: &Tree) -> Vec<u32> {
    let structural = |i: usize| tree.nodes[i].kind == NodeKind::Structural;
    let mut carrier = vec![u32::MAX; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let Some(parent) = n.parent.map(|p| p as usize) else {
            continue;
        };
        if !structural(i) {
            continue;
        }
        let best = carrier[parent];
        if best == u32::MAX || n.start_radius > tree.nodes[best as usize].start_radius {
            carrier[parent] = i as u32;
        }
    }
    let mut order = vec![0u32; tree.nodes.len()];
    let mut system = vec![0u32; tree.nodes.len()];
    for (i, n) in tree.nodes.iter().enumerate().skip(1) {
        let parent = n.parent.map_or(0, |p| p as usize);
        let stem = parent == 0;
        let opens = structural(i) && (stem || carrier[parent] != i as u32);
        order[i] = order[parent] + u32::from(opens && !stem);
        system[i] = if opens && order[i] <= SYSTEM_ORDER {
            i as u32
        } else {
            system[parent]
        };
    }
    system
}

/// Where a placement stands: the translation its matrix carries.
fn at(m: &[f32; 16]) -> Vec3 {
    Vec3::new(f64::from(m[12]), f64::from(m[13]), f64::from(m[14]))
}

/// One system's centre, and the neighbours whose boundaries its leaves meet:
/// each as the unit direction toward it and half the distance to it.
struct Cell {
    centre: Vec3,
    walls: Vec<(Vec3, f64, usize)>,
}

fn cells(positions: &[Vec3], owners: &[u32]) -> Vec<Option<Cell>> {
    let count = owners.iter().map(|&s| s as usize + 1).max().unwrap_or(0);
    let mut sums = vec![(Vec3::ZERO, 0usize); count];
    for (p, &s) in positions.iter().zip(owners) {
        sums[s as usize].0 += *p;
        sums[s as usize].1 += 1;
    }
    let centres: Vec<(usize, Vec3)> = sums
        .iter()
        .enumerate()
        .filter(|(_, (_, n))| *n > 0)
        .map(|(s, (sum, n))| (s, *sum / *n as f64))
        .collect();
    let mut out: Vec<Option<Cell>> = (0..count).map(|_| None).collect();
    for &(s, centre) in &centres {
        let mut near: Vec<(f64, Vec3, usize)> = centres
            .iter()
            .filter(|(o, _)| *o != s)
            .map(|(o, c)| (c.distance(centre), *c - centre, *o))
            .filter(|(d, _, _)| *d > 1e-9)
            .collect();
        near.sort_by(|a, b| a.0.total_cmp(&b.0));
        near.truncate(NEIGHBOURS);
        let walls = near
            .iter()
            .map(|(d, v, o)| (*v / *d, d / 2.0, *o))
            .collect();
        out[s] = Some(Cell { centre, walls });
    }
    out
}

/// Metres from a point to the nearest wall of the cell it stands in, and the
/// half-distance between the two centres that wall parts. The walk starts in
/// the cell of the system that bore the leaf and crosses into whichever
/// neighbour the point stands past, so a leaf that has grown into another
/// system's room is read in that room.
fn wall(cells: &[Option<Cell>], mut at: usize, p: Vec3) -> Option<(f64, f64)> {
    for _ in 0..NEIGHBOURS {
        let cell = cells[at].as_ref()?;
        let offset = p - cell.centre;
        let (gap, half, past) = cell
            .walls
            .iter()
            .map(|(n, half, o)| (half - offset.dot(*n), *half, *o))
            .fold(
                (f64::INFINITY, 0.0, at),
                |a, b| if b.0 < a.0 { b } else { a },
            );
        if gap >= 0.0 {
            return (half > 0.0).then_some((gap, half));
        }
        at = past;
    }
    None
}

/// A draw in [0, 1) keyed by the seed and the placement's place in the crown.
fn draw(seed: u32, index: usize) -> f64 {
    let mut z =
        (u64::from(seed ^ 0x1b87_3593) << 32) ^ (index as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    ((z ^ (z >> 31)) >> 11) as f64 / (1u64 << 53) as f64
}

/// Thins the placements toward the walls between limb systems. `owners`
/// names the node that bears each placement, in placement order.
pub(super) fn thin(tree: &Tree, owners: &[u32], seed: u32, reach: f64, out: &mut Instances) {
    if reach <= 0.0 || out.matrices.is_empty() {
        return;
    }
    debug_assert_eq!(owners.len(), out.matrices.len());
    let system = systems(tree);
    let systems: Vec<u32> = owners.iter().map(|&w| system[w as usize]).collect();
    let positions: Vec<Vec3> = out.matrices.iter().map(at).collect();
    let keep = kept(&positions, &systems, seed, reach);
    let mut k = 0;
    out.matrices.retain(|_| {
        k += 1;
        keep[k - 1]
    });
}

/// Which leaves stay, each standing at its position in its system. Within
/// `reach` of a wall, as a share of the way from it to either centre, a leaf
/// is kept with the chance its share of that reach gives it, squared, so the
/// gap is empty at the wall and fills in toward both systems' centres.
fn kept(positions: &[Vec3], systems: &[u32], seed: u32, reach: f64) -> Vec<bool> {
    let cells = cells(positions, systems);
    (0..positions.len())
        .map(|k| {
            let Some((gap, half)) = wall(&cells, systems[k] as usize, positions[k]) else {
                return true;
            };
            let share = gap / (reach * half);
            share >= 1.0 || draw(seed, k) < share.powi(2)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two systems either side of the origin along x, each a row of leaves
    /// from two metres out to the wall between them, so each centre stands a
    /// metre from the wall.
    fn two_rows() -> (Vec<Vec3>, Vec<u32>) {
        let mut positions = Vec::new();
        let mut systems = Vec::new();
        for (system, side) in [(1u32, -1.0), (2, 1.0)] {
            for k in 0..=400 {
                let x = side * (2.0 - f64::from(k) / 200.0);
                positions.push(Vec3::new(x, 0.0, 0.0));
                systems.push(system);
            }
        }
        (positions, systems)
    }

    #[test]
    fn the_gap_is_empty_at_the_wall_and_the_centres_are_whole() {
        let (positions, systems) = two_rows();
        let keep = kept(&positions, &systems, 7, 0.5);
        for (p, kept) in positions.iter().zip(&keep) {
            if p.x.abs() < 0.02 {
                assert!(!kept, "a leaf on the wall at {:.3} stayed", p.x);
            }
            if p.x.abs() >= 0.5 {
                assert!(kept, "a leaf past the reach at {:.3} went", p.x);
            }
        }
        // Thinner near the wall than halfway out to the reach.
        let share = |lo: f64, hi: f64| {
            let band: Vec<_> = positions
                .iter()
                .zip(&keep)
                .filter(|(p, _)| (lo..hi).contains(&p.x.abs()))
                .collect();
            band.iter().filter(|(_, k)| **k).count() as f64 / band.len() as f64
        };
        assert!(
            share(0.0, 0.2) < share(0.3, 0.5),
            "the gap does not fill in"
        );
    }

    #[test]
    fn one_system_has_no_walls_and_keeps_every_leaf() {
        let (positions, _) = two_rows();
        let one = vec![3u32; positions.len()];
        assert!(kept(&positions, &one, 7, 1.0).iter().all(|k| *k));
    }

    #[test]
    fn a_trunk_its_limbs_and_their_axes_are_systems_and_deeper_wood_is_not() {
        use crate::tree::Node;
        // A trunk of two nodes; at its top a limb (thinner than the leader
        // that carries on); on the limb an axis of order two, and on that one
        // of order three, which belongs to the order-two axis's system.
        let node = |parent: u32, x: f64, y: f64, r: f64| Node {
            parent: Some(parent),
            position: Vec3::new(x, y, 0.0),
            radius: r,
            start_radius: r,
            ..Node::root()
        };
        let nodes = vec![
            Node::root(),
            node(0, 0.0, 1.0, 1.0),
            node(1, 0.0, 2.0, 0.9),
            node(1, 1.0, 2.0, 0.5),
            node(3, 2.0, 3.0, 0.4),
            node(3, 2.0, 2.0, 0.2),
            node(5, 3.0, 2.0, 0.1),
            node(5, 3.0, 1.5, 0.05),
        ];
        let tree = Tree {
            crossover: nodes.len(),
            nodes,
            ..Tree::default()
        };
        let system = systems(&tree);
        // The stem opens a system at the root; the leader carries it on.
        assert_eq!(system[1], 1);
        assert_eq!(system[2], 1);
        // The limb opens one, and its thicker child carries it on.
        assert_eq!(system[3], 3);
        assert_eq!(system[4], 3);
        // The limb's lateral is order two and opens one; its own lateral is
        // order three and stays in it.
        assert_eq!(system[5], 5);
        assert_eq!(system[6], 5);
        assert_eq!(system[7], 5);
    }
}
