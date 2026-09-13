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
#[derive(Debug, Clone, PartialEq)]
pub struct Placement {
    pub identity: PlacementIdentity,
    pub transform: [f32; 16],
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
}
#[derive(Clone, Default)]
struct Cache {
    shoots: BTreeMap<NodeIdentity, Cached>,
    #[cfg(test)]
    derived: usize,
}
#[derive(Clone)]
pub(crate) struct Foliage {
    canopy: CanopyParams,
    twig: TwigPlacement,
    surface: SurfaceParams,
    seed: u32,
    lifetime: Age,
    cache: RefCell<Cache>,
}
impl Foliage {
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
            seed: family.skeleton.seed,
            lifetime: Age::from_years(family.growth.leaf_lifetime)?,
            cache: RefCell::new(Cache::default()),
        })
    }

    pub fn read(&self, tree: &Tree, envelope: Envelope, age: Age) -> Result<Vec<Placement>> {
        tree.validate_solved()?;
        let live = self.living(tree, age)?;
        // A read may rebuild the contact surface just as wood submission does;
        // only changed contact polygons trigger station matrix re-derivation.
        let contacts = if !live.is_empty() && self.canopy.surface_contact > 0.0 {
            Some(AttachmentSurface::new(
                tree,
                envelope.height.max(1e-6),
                &self.surface,
            )?)
        } else {
            None
        };
        let mut cache = self.cache.borrow_mut();
        #[cfg(test)]
        {
            cache.derived = 0;
        }
        let mut kept = BTreeSet::new();
        let mut count = 0;
        for i in live {
            let n = &tree.nodes[i];
            let parent = n.parent.unwrap() as usize;
            let wood = Wood {
                from: tree.nodes[parent].position,
                to: n.position,
                radii: [n.start_radius, n.radius],
                contact: contacts.as_ref().map_or_else(Vec::new, |s| s.signature(i)),
            };
            let id = n.identity;
            if cache.shoots.get(&id).is_none_or(|old| old.wood != wood) {
                let placements = self.place_shoot(tree, i, envelope, contacts.as_ref())?;
                cache.shoots.insert(id, Cached { wood, placements });
                #[cfg(test)]
                {
                    cache.derived += 1;
                }
            }
            count += cache.shoots[&id].placements.len();
            if count > self.canopy.max_instances {
                return Err(Error::ResourceLimit("foliage instance budget"));
            }
            kept.insert(id);
        }
        cache.shoots.retain(|id, _| kept.contains(id));
        let mut out = Vec::new();
        out.try_reserve(count)
            .map_err(|_| Error::ResourceLimit("foliage allocation"))?;
        for entry in cache.shoots.values() {
            out.extend_from_slice(&entry.placements);
        }
        Ok(out)
    }
    fn living(&self, tree: &Tree, age: Age) -> Result<Vec<usize>> {
        let now = age.ticks();
        let lifetime = self.lifetime.ticks();
        let slender = tree
            .nodes
            .first()
            .map_or(0.0, |n| n.radius * self.canopy.shoot_radius);
        let mut live = Vec::new();
        for (i, n) in tree.nodes.iter().enumerate() {
            if n.parent.is_some()
                && self.canopy.size > 0.0
                && (n.kind == NodeKind::Twig
                    || (slender > 0.0 && n.radius.max(n.start_radius) <= slender))
            {
                let birth = Age::from_years(n.shoot.birth_year)?.ticks();
                if now >= birth && now - birth < lifetime {
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
                params: self.canopy,
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
    pub(crate) fn derived(&self) -> usize {
        self.cache.borrow().derived
    }
}
