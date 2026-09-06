//! Mesh-free occupancy in metres, Y up. Wood is the union of linearly tapered
//! sphere sweeps along solved edges (rounded ends, no bark displacement).
//! Foliage uses world AABBs of transformed local leaf bounds, conservatively
//! enclosing blades, including flat cards. It does not voxelize triangles.
//!
//! Queries describe closed cubes: touching counts, zero half extent is a point.
//! Wood uses the cube's circumsphere, overestimating by at most its half diagonal;
//! foliage uses box overlap. Smaller cells reduce this resolution-dependent wood
//! inflation. Leaf AABB overestimation remains independent of resolution.
use crate::{
    foliage::{transform_point, Bounds, Element, Instances},
    math::Vec3,
    tree::Tree,
    Error, Result,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Occupancy {
    pub wood: bool,
    pub foliage: bool,
}

/// Owned representation; build once, query at any caller-selected resolution.
/// Allocation failure and unrepresentable arithmetic return ResourceLimit.
/// The borrowed solved tree and retained foliage are never modified.
pub struct Field {
    wood: Vec<Segment>,
    wood_index: Index,
    leaves: Index,
}
/// Optional owned f64 experiment input. No snapshot storage is retained by Field.
/// Wood records are [ax, ay, az, bx, by, bz, start_radius, end_radius].
pub struct FieldSnapshot {
    pub wood: Vec<f64>,
    pub wood_index: IndexSnapshot,
    pub leaves: IndexSnapshot,
}
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
struct Segment {
    a: Vec3,
    b: Vec3,
    start: f64,
    end: f64,
}
impl Segment {
    fn contains(&self, p: Vec3, inflation: f64) -> bool {
        let d = self.b - self.a;
        let q = p - self.a;
        let r = self.start + inflation;
        let dr = self.end - self.start;
        let a = d.dot(d) - dr * dr;
        let b = q.dot(d) + r * dr;
        let t = if a > 0. {
            (b / a).clamp(0., 1.)
        } else if b > a * 0.5 {
            1.
        } else {
            0.
        };
        let distance = q - d * t;
        distance.dot(distance) <= (r + dr * t).powi(2)
    }
}
impl Field {
    pub fn new(tree: &Tree, foliage: Option<(&Instances, &Element)>) -> Result<Self> {
        tree.validate_solved()?;
        let mut wood = reserved(tree.nodes.len())?;
        let mut wood_items = reserved(tree.nodes.len())?;
        for n in &tree.nodes {
            let a = n
                .parent
                .map_or(n.position, |p| tree.nodes[p as usize].position);
            let start = if n.parent.is_some() {
                n.start_radius
            } else {
                n.radius
            };
            let bounds = union(cube(a, start)?, cube(n.position, n.radius)?);
            wood_items.push(Item {
                bounds,
                id: wood.len(),
            });
            wood.push(Segment {
                a,
                b: n.position,
                start,
                end: n.radius,
            });
        }
        let mut leaf_items = Vec::new();
        if let Some((instances, element)) = foliage {
            instances.validate()?;
            element.validate()?;
            if let Some(local) = bounds_of(element.positions.iter().copied()) {
                leaf_items = reserved(instances.matrices.len())?;
                for m in &instances.matrices {
                    let corners = [local.min.x, local.max.x].into_iter().flat_map(|x| {
                        [local.min.y, local.max.y].into_iter().flat_map(move |y| {
                            [local.min.z, local.max.z]
                                .into_iter()
                                .map(move |z| transform_point(m, Vec3::new(x, y, z)))
                        })
                    });
                    let bounds = bounds_of(corners).unwrap();
                    checked(bounds)?;
                    leaf_items.push(Item { bounds, id: 0 });
                }
            }
        }
        Ok(Self {
            wood,
            wood_index: Index::new(wood_items)?,
            leaves: Index::new(leaf_items)?,
        })
    }
    pub fn bounds(&self) -> Option<Bounds> {
        match (self.wood_index.bounds(), self.leaves.bounds()) {
            (Some(a), Some(b)) => Some(union(a, b)),
            (a, b) => a.or(b),
        }
    }
    /// No allocation per query. Empty and outside regions return both flags false.
    /// Nonfinite centres/extents and negative extents are invalid requests.
    pub fn query(&self, center: Vec3, half_extent: f64) -> Result<Occupancy> {
        if !center.is_finite() || !half_extent.is_finite() || half_extent < 0. {
            return Err(Error::InvalidInput("field query"));
        }
        let cell = cube(center, half_extent)?;
        let inflation = half_extent * 3_f64.sqrt();
        let wood_cell = cube(center, inflation)?;
        Ok(Occupancy {
            wood: self
                .wood_index
                .any(wood_cell, |id| self.wood[id].contains(center, inflation)),
            foliage: self.leaves.any(cell, |_| true),
        })
    }
    /// Copies only on explicit request; allocation failure returns ResourceLimit.
    pub fn snapshot(&self) -> Result<FieldSnapshot> {
        let mut wood = reserved(
            self.wood
                .len()
                .checked_mul(8)
                .ok_or(Error::ResourceLimit("snapshot size"))?,
        )?;
        for s in &self.wood {
            wood.extend([s.a.x, s.a.y, s.a.z, s.b.x, s.b.y, s.b.z, s.start, s.end]);
        }
        Ok(FieldSnapshot {
            wood,
            wood_index: self.wood_index.snapshot()?,
            leaves: self.leaves.snapshot()?,
        })
    }
    /// Heap capacity owned by this field, excluding allocator bookkeeping.
    pub fn storage_bytes(&self) -> usize {
        self.wood.capacity() * std::mem::size_of::<Segment>()
            + self.wood_index.storage_bytes()
            + self.leaves.storage_bytes()
    }
}
fn reserved<T>(count: usize) -> Result<Vec<T>> {
    let mut out = Vec::new();
    out.try_reserve_exact(count)
        .map_err(|_| Error::ResourceLimit("field allocation"))?;
    Ok(out)
}
fn cube(p: Vec3, radius: f64) -> Result<Bounds> {
    let r = Vec3::new(radius, radius, radius);
    let b = Bounds {
        min: p - r,
        max: p + r,
    };
    checked(b)?;
    Ok(b)
}
fn checked(b: Bounds) -> Result<()> {
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
fn union(a: Bounds, b: Bounds) -> Bounds {
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
fn bounds_of(points: impl Iterator<Item = Vec3>) -> Option<Bounds> {
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
struct Item {
    bounds: Bounds,
    id: usize,
}
struct Node {
    bounds: Bounds,
    start: usize,
    end: usize,
    children: Option<(usize, usize)>,
}
#[derive(Default)]
struct Index {
    items: Vec<Item>,
    nodes: Vec<Node>,
}
impl Index {
    fn snapshot(&self) -> Result<IndexSnapshot> {
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
    fn new(mut items: Vec<Item>) -> Result<Self> {
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
    fn bounds(&self) -> Option<Bounds> {
        self.nodes.first().map(|n| n.bounds)
    }
    fn any(&self, bounds: Bounds, predicate: impl Fn(usize) -> bool) -> bool {
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
    fn storage_bytes(&self) -> usize {
        self.items.capacity() * std::mem::size_of::<Item>()
            + self.nodes.capacity() * std::mem::size_of::<Node>()
    }
}
