//! The field's bounding-volume index: an exact CPU median BVH over boxed
//! items, and the checked box arithmetic the field shares with it.
use crate::{foliage::Bounds, math::Vec3, Error, Result};

/// Exact CPU median BVH order. Bounds are six f64 values (min xyz, max xyz)
/// per node, then per item. Topology contains four u32 values per node
/// (item start, exclusive end, left, right), then one primitive ID per item.
/// u32::MAX children denote a leaf; an empty index has zero nodes and items.
/// Root is node zero; leaf primitive IDs are unused. All ranges are index-local.
pub struct IndexSnapshot {
    pub bounds: Vec<f64>,
    pub topology: Vec<u32>,
    pub node_count: u32,
}
pub(super) fn reserved<T>(count: usize) -> Result<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit("field allocation"))?;
    Ok(out)
}
pub(super) fn cube(p: Vec3, radius: f64) -> Result<Bounds> {
    let r = Vec3::new(radius, radius, radius);
    let b = Bounds {
        min: p - r,
        max: p + r,
    };
    checked(b)?;
    Ok(b)
}
pub(super) fn checked(b: Bounds) -> Result<()> {
    // Squared distances and dot products must remain representable too.
    let limit = (f64::MAX / 64.).sqrt();
    if [b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]
        .iter()
        .any(|v| !v.is_finite() || v.abs() > limit)
    {
        Err(Error::ResourceLimit("field coordinate overflow"))
    } else {
        Ok(())
    }
}
pub(super) fn union(a: Bounds, b: Bounds) -> Bounds {
    Bounds {
        min: Vec3::new(
            a.min.x.min(b.min.x),
            a.min.y.min(b.min.y),
            a.min.z.min(b.min.z),
        ),
        max: Vec3::new(
            a.max.x.max(b.max.x),
            a.max.y.max(b.max.y),
            a.max.z.max(b.max.z),
        ),
    }
}
pub(super) fn bounds_of(points: impl Iterator<Item = Vec3>) -> Option<Bounds> {
    points.map(|p| Bounds { min: p, max: p }).reduce(union)
}
fn overlaps(a: Bounds, b: Bounds) -> bool {
    a.min.x <= b.max.x
        && a.max.x >= b.min.x
        && a.min.y <= b.max.y
        && a.max.y >= b.min.y
        && a.min.z <= b.max.z
        && a.max.z >= b.min.z
}
pub(super) struct Item {
    pub bounds: Bounds,
    pub id: usize,
}
struct Node {
    bounds: Bounds,
    start: usize,
    end: usize,
    children: Option<(usize, usize)>,
}
#[derive(Default)]
pub(super) struct Index {
    items: Vec<Item>,
    nodes: Vec<Node>,
}
impl Index {
    pub fn snapshot(&self) -> Result<IndexSnapshot> {
        let u32_index =
            |n: usize| u32::try_from(n).map_err(|_| Error::ResourceLimit("snapshot index"));
        let node_count = u32_index(self.nodes.len())?;
        u32_index(self.items.len())?;
        let bounds_len = self
            .nodes
            .len()
            .checked_add(self.items.len())
            .and_then(|n| n.checked_mul(6))
            .ok_or(Error::ResourceLimit("snapshot size"))?;
        let topology_len = self
            .nodes
            .len()
            .checked_mul(4)
            .and_then(|n| n.checked_add(self.items.len()))
            .ok_or(Error::ResourceLimit("snapshot size"))?;
        let mut bounds = reserved(bounds_len)?;
        let mut topology = reserved(topology_len)?;
        for b in self
            .nodes
            .iter()
            .map(|n| n.bounds)
            .chain(self.items.iter().map(|i| i.bounds))
        {
            bounds.extend([b.min.x, b.min.y, b.min.z, b.max.x, b.max.y, b.max.z]);
        }
        for n in &self.nodes {
            let (left, right) = match n.children {
                Some((l, r)) => (u32_index(l)?, u32_index(r)?),
                None => (u32::MAX, u32::MAX),
            };
            topology.extend([u32_index(n.start)?, u32_index(n.end)?, left, right]);
        }
        for i in &self.items {
            topology.push(u32_index(i.id)?);
        }
        Ok(IndexSnapshot {
            bounds,
            topology,
            node_count,
        })
    }
    pub fn new(mut items: Vec<Item>) -> Result<Self> {
        // Median partition gives bounded depth and O(n log n) construction,
        // without duplicating large primitives across grid cells.
        let count = items.len();
        let mut nodes = reserved(
            count
                .checked_mul(2)
                .ok_or(Error::ResourceLimit("field index"))?
                / 4,
        )?;
        if count > 0 {
            Self::split(&mut items, &mut nodes, 0)?;
        }
        Ok(Self { items, nodes })
    }
    fn split(items: &mut [Item], nodes: &mut Vec<Node>, start: usize) -> Result<usize> {
        let bounds = items.iter().map(|i| i.bounds).reduce(union).unwrap();
        nodes
            .try_reserve(1)
            .map_err(|_| Error::ResourceLimit("field index allocation"))?;
        let id = nodes.len();
        nodes.push(Node {
            bounds,
            start,
            end: start + items.len(),
            children: None,
        });
        if items.len() > 8 {
            let size = bounds.max - bounds.min;
            let axis = if size.x >= size.y && size.x >= size.z {
                0
            } else if size.y >= size.z {
                1
            } else {
                2
            };
            let coordinate = |b: Bounds| match axis {
                0 => b.min.x + b.max.x,
                1 => b.min.y + b.max.y,
                _ => b.min.z + b.max.z,
            };
            let mid = items.len() / 2;
            items.select_nth_unstable_by(mid, |a, b| {
                coordinate(a.bounds).total_cmp(&coordinate(b.bounds))
            });
            let (left, right) = items.split_at_mut(mid);
            let l = Self::split(left, nodes, start)?;
            let r = Self::split(right, nodes, start + mid)?;
            nodes[id].children = Some((l, r));
        }
        Ok(id)
    }
    pub fn bounds(&self) -> Option<Bounds> {
        self.nodes.first().map(|n| n.bounds)
    }
    /// Whether any item overlapping `bounds` satisfies the predicate.
    pub fn any(&self, bounds: Bounds, predicate: impl Fn(usize) -> bool) -> bool {
        !self.nodes.is_empty() && self.visit(0, bounds, &predicate)
    }
    fn visit(&self, id: usize, bounds: Bounds, predicate: &impl Fn(usize) -> bool) -> bool {
        let node = &self.nodes[id];
        if !overlaps(node.bounds, bounds) {
            return false;
        }
        match node.children {
            Some((l, r)) => self.visit(l, bounds, predicate) || self.visit(r, bounds, predicate),
            None => self.items[node.start..node.end]
                .iter()
                .any(|i| overlaps(i.bounds, bounds) && predicate(i.id)),
        }
    }
    /// Calls `each` on every item overlapping `bounds`, in index order.
    pub fn each(&self, bounds: Bounds, each: &mut impl FnMut(usize)) {
        if !self.nodes.is_empty() {
            self.walk(0, bounds, each);
        }
    }
    fn walk(&self, id: usize, bounds: Bounds, each: &mut impl FnMut(usize)) {
        let node = &self.nodes[id];
        if !overlaps(node.bounds, bounds) {
            return;
        }
        match node.children {
            Some((l, r)) => {
                self.walk(l, bounds, each);
                self.walk(r, bounds, each);
            }
            None => {
                for i in &self.items[node.start..node.end] {
                    if overlaps(i.bounds, bounds) {
                        each(i.id);
                    }
                }
            }
        }
    }
    pub fn storage_bytes(&self) -> usize {
        self.items.capacity() * std::mem::size_of::<Item>()
            + self.nodes.capacity() * std::mem::size_of::<Node>()
    }
}
