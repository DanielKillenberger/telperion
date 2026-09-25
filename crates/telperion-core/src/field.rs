//! Mesh-free occupancy in metres, Y up. Wood is the union of linearly tapered
//! sphere sweeps along solved edges (rounded ends, no bark displacement).
//!
//! Foliage comes from the leaf plan where the family has one: a sweep along
//! every leaf-bearing segment with a conservative reach as its radius, tested
//! against the query cube's circumsphere the way wood is, carrying the
//! segment's station count and its limb system. No leaf is placed. A family
//! the plan cannot describe keeps the placed path: world AABBs of transformed
//! local leaf bounds, conservatively enclosing blades, including flat cards.
//!
//! Queries describe closed cubes: touching counts, zero half extent is a point.
//! Sweeps use the cube's circumsphere, overestimating by at most its half
//! diagonal; placed leaves use box overlap. Smaller cells reduce the
//! resolution-dependent sweep inflation.
mod index;
mod ribbon;
use crate::{
    foliage::{plan::Plan, transform_point, Bounds, Element, Instances},
    math::Vec3,
    tree::Tree,
    Error, Result,
};
pub use index::IndexSnapshot;
use index::{bounds_of, checked, cube, reserved, union, Index, Item};
use ribbon::Ribbon;

/// One cell's answer. `wood_radius` is the larger end radius of the thickest
/// wood sweep reaching the cell, in metres, zero where no wood does.
/// `leaves` estimates the stations in the cell before the crown-shell cull:
/// over a grid of non-overlapping cells the estimates sum to the plan's
/// total. `limb` is the limb system with the largest estimate in the cell,
/// the lower id on a tie, and `None` where no foliage reaches. On the placed
/// path `leaves` counts the retained leaves whose box overlaps the cell and
/// `limb` is always `None`.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Occupancy {
    pub wood: bool,
    pub wood_radius: f64,
    pub foliage: bool,
    pub leaves: f64,
    pub limb: Option<u32>,
}

/// Owned representation; build once, query at any caller-selected resolution.
/// Allocation failure and unrepresentable arithmetic return ResourceLimit.
/// The borrowed solved tree, plan and retained foliage are never modified.
pub struct Field {
    wood: Vec<Segment>,
    wood_index: Index,
    foliage: Foliage,
}
enum Foliage {
    Placed(Index),
    Planned { sweeps: Vec<Sweep>, index: Index },
}
/// Optional owned f64 experiment input. No snapshot storage is retained by Field.
/// Wood records are [ax, ay, az, bx, by, bz, start_radius, end_radius]. Plan
/// records are [ax, ay, az, bx, by, bz, reach] with [count, system] beside
/// them; a planned field has an empty leaf index, a placed field an empty plan.
/// A plan holding a ribbon also carries three f64 a record in `plan_sides`,
/// the ribbon's half-width vector (zero for a capsule), its reach being its
/// thickness; a plan of capsules alone leaves `plan_sides` empty.
pub struct FieldSnapshot {
    pub wood: Vec<f64>,
    pub wood_index: IndexSnapshot,
    pub leaves: IndexSnapshot,
    pub plan: Vec<f64>,
    pub plan_stations: Vec<u32>,
    pub plan_index: IndexSnapshot,
    pub plan_sides: Vec<f64>,
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
    /// The share of the segment's length inside the closed cube; a point
    /// segment counts whole where it lies inside.
    fn share_inside(&self, center: Vec3, half: f64) -> f64 {
        let (mut t0, mut t1) = (0_f64, 1_f64);
        for axis in 0..3 {
            let (a, d, c) = match axis {
                0 => (self.a.x, self.b.x - self.a.x, center.x),
                1 => (self.a.y, self.b.y - self.a.y, center.y),
                _ => (self.a.z, self.b.z - self.a.z, center.z),
            };
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
/// A plan record: a capsule, the segment itself, or a ribbon along it. The
/// ribbon's midline is the segment, which the count estimates run along.
struct Sweep {
    segment: Segment,
    ribbon: Option<Ribbon>,
    count: f64,
    system: u32,
}
impl Sweep {
    /// Whether the cell of `half` extent about `p` meets the record: a
    /// capsule by the cell's circumsphere, `inflation`, a ribbon cell to box.
    fn meets(&self, p: Vec3, half: f64, inflation: f64) -> bool {
        match &self.ribbon {
            Some(r) => r.meets(p, half),
            None => self.segment.contains(p, inflation),
        }
    }
}
/// Per-system estimates of one query, on the stack for the systems a cell
/// ordinarily meets and spilling only past sixteen.
#[derive(Default)]
struct Tally {
    entries: [(u32, f64); 16],
    len: usize,
    spill: Vec<(u32, f64)>,
}
impl Tally {
    fn add(&mut self, system: u32, estimate: f64) {
        for e in self.entries[..self.len].iter_mut().chain(&mut self.spill) {
            if e.0 == system {
                e.1 += estimate;
                return;
            }
        }
        if self.len < self.entries.len() {
            self.entries[self.len] = (system, estimate);
            self.len += 1;
        } else {
            self.spill.push((system, estimate));
        }
    }
    fn total(&self) -> f64 {
        self.entries[..self.len]
            .iter()
            .chain(&self.spill)
            .map(|e| e.1)
            .sum()
    }
    fn best(&self) -> Option<u32> {
        self.entries[..self.len]
            .iter()
            .chain(&self.spill)
            .fold(
                None,
                |best: Option<(u32, f64)>, &(system, estimate)| match best {
                    Some((s, e)) if e > estimate || (e == estimate && s < system) => Some((s, e)),
                    _ => Some((system, estimate)),
                },
            )
            .map(|(system, _)| system)
    }
}
impl Field {
    /// The placed path: foliage from retained leaves.
    pub fn new(tree: &Tree, foliage: Option<(&Instances, &Element)>) -> Result<Self> {
        let (wood, wood_index) = Self::wood(tree)?;
        let mut leaf_items = Vec::new();
        if let Some((instances, element)) = foliage {
            instances.validate()?;
            element.validate()?;
            if let Some(local) = bounds_of(element.positions.iter().copied()) {
                leaf_items = reserved(instances.len())?;
                for m in instances.matrices() {
                    let corners = [local.min.x, local.max.x].into_iter().flat_map(|x| {
                        [local.min.y, local.max.y].into_iter().flat_map(move |y| {
                            [local.min.z, local.max.z]
                                .into_iter()
                                .map(move |z| transform_point(&m, Vec3::new(x, y, z)))
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
            wood_index,
            foliage: Foliage::Placed(Index::new(leaf_items)?),
        })
    }
    /// The planned path: foliage from the leaf plan, no leaf placed.
    pub fn planned(tree: &Tree, plan: &Plan) -> Result<Self> {
        let (wood, wood_index) = Self::wood(tree)?;
        let mut sweeps = reserved(plan.descriptors.len())?;
        let mut items = reserved(plan.descriptors.len())?;
        for d in &plan.descriptors {
            let reach = plan.reach(d);
            if !reach.is_finite() || reach < 0. {
                return Err(Error::InvalidInput("foliage reach"));
            }
            let [a, b] = d.endpoints;
            let ribbon = (d.side != Vec3::ZERO).then_some(Ribbon {
                a,
                b,
                side: d.side,
                thickness: reach,
            });
            let bounds = match &ribbon {
                Some(r) => r.bounds()?,
                None => union(cube(a, reach)?, cube(b, reach)?),
            };
            items.push(Item {
                bounds,
                id: sweeps.len(),
            });
            sweeps.push(Sweep {
                segment: Segment {
                    a,
                    b,
                    start: reach,
                    end: reach,
                },
                ribbon,
                count: f64::from(d.count),
                system: d.system,
            });
        }
        Ok(Self {
            wood,
            wood_index,
            foliage: Foliage::Planned {
                sweeps,
                index: Index::new(items)?,
            },
        })
    }
    /// Whether foliage answers come from the plan rather than placed leaves.
    pub fn is_planned(&self) -> bool {
        matches!(self.foliage, Foliage::Planned { .. })
    }
    fn wood(tree: &Tree) -> Result<(Vec<Segment>, Index)> {
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
        Ok((wood, Index::new(wood_items)?))
    }
    fn foliage_index(&self) -> &Index {
        match &self.foliage {
            Foliage::Placed(index) => index,
            Foliage::Planned { index, .. } => index,
        }
    }
    pub fn bounds(&self) -> Option<Bounds> {
        match (self.wood_index.bounds(), self.foliage_index().bounds()) {
            (Some(a), Some(b)) => Some(union(a, b)),
            (a, b) => a.or(b),
        }
    }
    /// No allocation per query below seventeen limb systems in one cell.
    /// Empty and outside regions return both flags false. Nonfinite
    /// centres/extents and negative extents are invalid requests.
    pub fn query(&self, center: Vec3, half_extent: f64) -> Result<Occupancy> {
        if !center.is_finite() || !half_extent.is_finite() || half_extent < 0. {
            return Err(Error::InvalidInput("field query"));
        }
        let cell = cube(center, half_extent)?;
        let inflation = half_extent * 3_f64.sqrt();
        let wood_cell = cube(center, inflation)?;
        let (mut wood, mut wood_radius) = (false, 0.);
        self.wood_index.each(wood_cell, &mut |id| {
            let s = &self.wood[id];
            if s.contains(center, inflation) {
                wood = true;
                wood_radius = s.start.max(s.end).max(wood_radius);
            }
        });
        Ok(match &self.foliage {
            Foliage::Placed(index) => {
                let mut leaves = 0.;
                index.each(cell, &mut |_| leaves += 1.);
                Occupancy {
                    wood,
                    wood_radius,
                    foliage: leaves > 0.,
                    leaves,
                    limb: None,
                }
            }
            Foliage::Planned { sweeps, index } => {
                let mut tally = Tally::default();
                let mut foliage = false;
                index.each(wood_cell, &mut |id| {
                    let s = &sweeps[id];
                    if s.meets(center, half_extent, inflation) {
                        foliage = true;
                        tally.add(
                            s.system,
                            s.count * s.segment.share_inside(center, half_extent),
                        );
                    }
                });
                Occupancy {
                    wood,
                    wood_radius,
                    foliage,
                    leaves: tally.total(),
                    limb: tally.best(),
                }
            }
        })
    }
    /// Copies only on explicit request; allocation failure returns ResourceLimit.
    pub fn snapshot(&self) -> Result<FieldSnapshot> {
        fn records<T>(count: usize, width: usize) -> Result<Vec<T>> {
            count
                .checked_mul(width)
                .ok_or(Error::ResourceLimit("snapshot size"))
                .and_then(reserved)
        }
        let mut wood = records(self.wood.len(), 8)?;
        for s in &self.wood {
            wood.extend([s.a.x, s.a.y, s.a.z, s.b.x, s.b.y, s.b.z, s.start, s.end]);
        }
        let mut plan_sides = Vec::new();
        let (leaves, plan, plan_stations, plan_index) = match &self.foliage {
            Foliage::Placed(index) => (
                index.snapshot()?,
                Vec::new(),
                Vec::new(),
                Index::default().snapshot()?,
            ),
            Foliage::Planned { sweeps, index } => {
                let mut plan = records(sweeps.len(), 7)?;
                let mut stations = records(sweeps.len(), 2)?;
                for s in sweeps {
                    let g = &s.segment;
                    plan.extend([g.a.x, g.a.y, g.a.z, g.b.x, g.b.y, g.b.z, g.start]);
                    stations.extend([s.count as u32, s.system]);
                }
                if sweeps.iter().any(|s| s.ribbon.is_some()) {
                    plan_sides = records(sweeps.len(), 3)?;
                    for s in sweeps {
                        let side = s.ribbon.map_or(Vec3::ZERO, |r| r.side);
                        plan_sides.extend([side.x, side.y, side.z]);
                    }
                }
                (
                    Index::default().snapshot()?,
                    plan,
                    stations,
                    index.snapshot()?,
                )
            }
        };
        Ok(FieldSnapshot {
            wood,
            wood_index: self.wood_index.snapshot()?,
            leaves,
            plan,
            plan_stations,
            plan_index,
            plan_sides,
        })
    }
    /// Heap capacity owned by this field, excluding allocator bookkeeping.
    pub fn storage_bytes(&self) -> usize {
        let foliage = match &self.foliage {
            Foliage::Placed(index) => index.storage_bytes(),
            Foliage::Planned { sweeps, index } => {
                sweeps.capacity() * std::mem::size_of::<Sweep>() + index.storage_bytes()
            }
        };
        self.wood.capacity() * std::mem::size_of::<Segment>()
            + self.wood_index.storage_bytes()
            + foliage
    }
}
