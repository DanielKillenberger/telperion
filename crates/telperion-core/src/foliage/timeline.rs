//! Leaf stations belong to a shoot's identity, independently of compact storage.
use super::{
    station::{place_run, Run},
    *,
};
use crate::{
    growth::Age,
    presets::Family,
    rng::Rng,
    surface::{AttachmentSurface, SurfaceParams},
    tree::{NodeIdentity, NodeKind, Tree},
};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlacementIdentity {
    pub shoot: NodeIdentity,
    pub station: u32,
}
#[derive(Debug, Clone)]
pub struct Placement {
    pub identity: PlacementIdentity,
    pub transform: [f32; 16],
}

impl PartialEq for Placement {
    fn eq(&self, other: &Self) -> bool {
        self.identity == other.identity
            && self.transform.map(f32::to_bits) == other.transform.map(f32::to_bits)
    }
}

#[derive(Clone, PartialEq)]
struct Wood {
    from: Vec3,
    to: Vec3,
    radii: [f64; 2],
    // Exact f32 polygon vertices read by the contact query, including adjacent
    // segments. An extension can change the socket without changing this edge.
    contact: Vec<Vec3>,
}
#[derive(Clone)]
struct Cached {
    wood: Wood,
    placements: Vec<Placement>,
    birth: Age,
}
#[derive(Clone, Default)]
struct Cache {
    shoots: BTreeMap<NodeIdentity, Cached>,
    // Exact geometry, not a hash: no collision may hide a neighboring socket
    // change. Read reuse stores no mesh and does not alter per-shoot derivation.
    geometry: Option<(u64, Vec<[u64; 9]>)>,
    #[cfg(test)]
    derived: usize,
    #[cfg(test)]
    surfaces: usize,
}
#[derive(Clone)]
pub(crate) struct Foliage {
    canopy: CanopyParams,
    twig: TwigPlacement,
    surface: SurfaceParams,
    // Authored surface units stay fixed for this specimen. Annual crown
    // height is a growth budget, never a station-transform dependency.
    contact_height: f64,
    seed: u32,
    lifetime: Age,
    bearing_radius: f64,
    cache: RefCell<Cache>,
}
impl Foliage {
    pub(crate) fn forget(&self, removed: &[NodeIdentity]) {
        let mut cache = self.cache.borrow_mut();
        for id in removed {
            cache.shoots.remove(id);
        }
        cache.geometry = None;
    }

    pub fn new(family: &Family) -> Result<Self> {
        let twig = family.skeleton.twigs.resolved()?.twig;
        let twig = TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        };
        // Reuse the placement contract, including its empty-tree validation.
        place(
            &Tree::default(),
            family.skeleton.envelope,
            family.skeleton.seed,
            family.canopy,
            Some(twig),
        )?;
        if family.canopy.surface_contact > 0.0 {
            family.surface.validate()?;
        }
        Ok(Self {
            canopy: family.canopy,
            twig,
            surface: family.surface,
            contact_height: family.skeleton.envelope.height.max(1e-6),
            seed: family.skeleton.seed,
            lifetime: Age::from_years(family.growth.leaf_lifetime)?,
            bearing_radius: family.skeleton.twigs.resolved()?.twig.bearing_diameter / 2.0,
            cache: RefCell::new(Cache::default()),
        })
    }

    /// Historical reads own their cache, preserving frontier transforms for a
    /// later clock-only fill. Copy only immutable traits, never cached foliage.
    pub(crate) fn read_uncached(
        &self,
        tree: &Tree,
        envelope: Envelope,
        age: Age,
    ) -> Result<Vec<Placement>> {
        Self {
            canopy: self.canopy,
            twig: self.twig,
            surface: self.surface,
            contact_height: self.contact_height,
            seed: self.seed,
            lifetime: self.lifetime,
            bearing_radius: self.bearing_radius,
            cache: RefCell::new(Cache::default()),
        }
        .read(tree, envelope, age)
    }

    pub fn read(&self, tree: &Tree, envelope: Envelope, age: Age) -> Result<Vec<Placement>> {
        tree.validate_solved()?;
        let live = self.living(tree, age)?;
        let mut cache = self.cache.borrow_mut();
        let geometry = (!live.is_empty() && self.canopy.surface_contact > 0.0)
            .then(|| contact_geometry(tree, self.contact_height));
        let reuse = geometry.is_some()
            && cache.geometry == geometry
            && live
                .iter()
                .all(|&i| cache.shoots.contains_key(&tree.nodes[i].identity));
        // Rebuild contact polygons only after wood changes or an uncached shoot
        // becomes visible. Their exact signatures still select changed matrices.
        let contacts = if geometry.is_some() && !reuse {
            #[cfg(test)]
            {
                cache.surfaces += 1;
            }
            Some(AttachmentSurface::new(
                tree,
                self.contact_height,
                &self.surface,
            )?)
        } else {
            None
        };
        #[cfg(test)]
        {
            cache.derived = 0;
        }
        let mut kept = BTreeSet::new();
        let mut count = 0;
        for i in live {
            let n = &tree.nodes[i];
            let id = n.identity;
            if !reuse {
                let parent = n.parent.unwrap() as usize;
                let wood = Wood {
                    from: tree.nodes[parent].position,
                    to: n.position,
                    radii: [n.start_radius, n.radius],
                    contact: contacts.as_ref().map_or_else(Vec::new, |s| s.signature(i)),
                };
                if cache.shoots.get(&id).is_none_or(|old| old.wood != wood) {
                    let placements = self.place_shoot(tree, i, envelope, contacts.as_ref())?;
                    cache.shoots.insert(
                        id,
                        Cached {
                            wood,
                            placements,
                            birth: Age::from_years(n.shoot.birth_year)?,
                        },
                    );
                    #[cfg(test)]
                    {
                        cache.derived += 1;
                    }
                }
            }
            let entry = &cache.shoots[&id];
            count += self.visible(entry.birth, age, entry.placements.len());
            if count > self.canopy.max_instances {
                return Err(Error::ResourceLimit("foliage instance budget"));
            }
            kept.insert(id);
        }
        cache.shoots.retain(|id, _| kept.contains(id));
        cache.geometry = geometry;
        let mut out = Vec::new();
        out.try_reserve(count)
            .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
        for entry in cache.shoots.values() {
            let count = self.visible(entry.birth, age, entry.placements.len());
            out.extend_from_slice(&entry.placements[..count]);
        }
        Ok(out)
    }
    // Spread stations evenly over ceil(lifetime) annual cohorts. Offset zero
    // flushes at birth; a one-year lifetime therefore fills immediately. Use
    // integer ticks and products, even at the maximum supported lifetime.
    fn visible(&self, birth: Age, age: Age, stations: usize) -> usize {
        if self.lifetime.ticks() == 0 || age.ticks() < birth.ticks() {
            return 0;
        }
        let cohorts = self.lifetime.slice + u64::from(self.lifetime.remainder > 0);
        let years = (age.ticks() - birth.ticks())
            / Age {
                slice: 1,
                remainder: 0,
            }
            .ticks();
        ((u128::from((years + 1).min(cohorts)) * stations as u128).div_ceil(u128::from(cohorts)))
            as usize
    }
    fn living(&self, tree: &Tree, age: Age) -> Result<Vec<usize>> {
        let now = age.ticks();
        let slender = tree
            .nodes
            .first()
            .map_or(0.0, |n| n.radius * self.canopy.shoot_radius);
        let mut live = Vec::new();
        for (i, n) in tree.nodes.iter().enumerate() {
            if n.parent.is_some()
                && self.canopy.size > 0.0
                && self.lifetime.ticks() > 0
                && n.radius.max(n.start_radius) <= self.bearing_radius
                && n.shoot.death_year.is_none_or(|death| age.slice < death)
                && (n.kind == NodeKind::Twig
                    || (slender > 0.0 && n.radius.max(n.start_radius) <= slender))
            {
                let birth = Age::from_years(n.shoot.birth_year)?.ticks();
                if now >= birth {
                    live.push(i);
                }
            }
        }
        Ok(live)
    }

    fn place_shoot(
        &self,
        tree: &Tree,
        i: usize,
        envelope: Envelope,
        contacts: Option<&AttachmentSurface>,
    ) -> Result<Vec<Placement>> {
        let n = &tree.nodes[i];
        let parent = n.parent.unwrap() as usize;
        let id = n.identity;
        let mut instances = Instances::default();
        // Each node is the shoot born at that annual station. Extension
        // appends another shoot; it cannot renumber existing leaf sites.
        let nodes = [parent, i];
        let birth = id.birth_order();
        let seed = self.seed ^ birth as u32 ^ ((birth >> 32) as u32).rotate_left(13);
        place_run(
            &Run {
                tree,
                nodes: &nodes,
                envelope,
                // The station budget still bounds this shoot. The instance
                // budget applies to visible cohorts, checked by the reader.
                params: CanopyParams {
                    max_instances: usize::MAX,
                    ..self.canopy
                },
                twig: Some(self.twig),
                contacts,
            },
            &mut Rng::new(seed ^ 0x2c9e1a7f),
            &mut instances,
        )?;
        let placements = instances
            .matrices
            .into_iter()
            .enumerate()
            .map(|(k, transform)| Placement {
                identity: PlacementIdentity {
                    shoot: id,
                    station: k as u32,
                },
                transform,
            })
            .collect();
        Ok(placements)
    }

    #[cfg(test)]
    pub(crate) fn surfaces(&self) -> usize {
        self.cache.borrow().surfaces
    }

    #[cfg(test)]
    pub(crate) fn derived(&self) -> usize {
        self.cache.borrow().derived
    }
}

/// xyz and distal/proximal/base radii, packed topology, kind, birth identity.
/// Family surface traits are immutable throughout this Foliage object's life.
fn contact_geometry(tree: &Tree, height: f64) -> (u64, Vec<[u64; 9]>) {
    let wood = tree
        .nodes
        .iter()
        .map(|n| {
            [
                n.position.x.to_bits(),
                n.position.y.to_bits(),
                n.position.z.to_bits(),
                n.radius.to_bits(),
                n.start_radius.to_bits(),
                n.base_radius.to_bits(),
                u64::from(n.parent.unwrap_or(u32::MAX)) << 32 | u64::from(n.branch),
                match n.kind {
                    NodeKind::Structural => 0,
                    NodeKind::Branch => 1,
                    NodeKind::Twig => 2,
                },
                n.identity.birth_order(),
            ]
        })
        .collect();
    (height.to_bits(), wood)
}

mod interval;
