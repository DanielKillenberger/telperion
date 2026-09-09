//! One engine-neutral mesh per parameter family. Renderers consume this and nothing else.
use crate::{
    branching, foliage,
    math::Vec3,
    presets::Family,
    surface::{self, Bounds, SurfaceMesh},
    Error, Result,
};

/// Detail budget. Full detail is the only budget today; the argument is the
/// seam a coarse-first budget extends without changing the call.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Detail {
    #[default]
    Full,
}

/// One element mesh drawn once per instance matrix.
#[derive(Debug)]
pub struct Foliage {
    pub element: foliage::Element,
    pub instances: foliage::Instances,
}

/// Everything a renderer needs for one tree: wood as an indexed surface,
/// foliage as one element plus its instances, and the union bounds.
#[derive(Debug)]
pub struct TreeMesh {
    pub wood: SurfaceMesh,
    pub foliage: Foliage,
    pub bounds: Bounds,
}
impl TreeMesh {
    pub fn wood_vertices(&self) -> usize {
        self.wood.positions.len() / 3
    }
    pub fn wood_triangles(&self) -> usize {
        self.wood.indices.len() / 3
    }
    pub fn foliage_instances(&self) -> usize {
        self.foliage.instances.matrices.len()
    }
}

impl From<foliage::Bounds> for Bounds {
    fn from(b: foliage::Bounds) -> Self {
        Self {
            min: b.min,
            max: b.max,
        }
    }
}
fn union(a: Option<Bounds>, b: Option<Bounds>) -> Option<Bounds> {
    match (a, b) {
        (Some(a), Some(b)) => Some(Bounds {
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
        }),
        (a, b) => a.or(b),
    }
}

/// Grows the skeleton, plaits the wood surface and places the culled foliage.
pub fn build(family: &Family, detail: Detail) -> Result<TreeMesh> {
    let Detail::Full = detail;
    let tree = branching::generate(&family.skeleton, family.radii)?.tree;
    let wood = surface::build(&tree, family.skeleton.envelope.height, &family.surface)?;
    let element = foliage::build_element(family.element)?;
    let twig = family.skeleton.twigs.resolved()?.twig;
    let placed = foliage::place_on_surface(
        &tree,
        family.skeleton.envelope,
        family.skeleton.seed,
        family.canopy,
        Some(foliage::TwigPlacement {
            internode_length: twig.internode_length,
            stations_per_internode: twig.stations_per_internode,
        }),
        &family.surface,
    )?;
    let instances = foliage::cull(
        &placed,
        &element,
        family.skeleton.envelope,
        family.shell_depth,
    )?;
    drop(placed);
    let bounds = union(wood.bounds, instances.bounds(&element)?.map(Bounds::from))
        .ok_or(Error::InvalidInput("mesh has no geometry"))?;
    Ok(TreeMesh {
        wood,
        foliage: Foliage { element, instances },
        bounds,
    })
}
