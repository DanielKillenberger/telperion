//! One engine-neutral mesh per parameter family. Renderers consume this and nothing else.
use crate::{
    foliage,
    math::Vec3,
    pipeline,
    presets::Family,
    surface::{Bounds, SurfaceMesh},
    tree::Tree,
    Error, Result,
};

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
        self.foliage.instances.len()
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
pub(crate) fn union(a: Option<Bounds>, b: Option<Bounds>) -> Option<Bounds> {
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

/// Grows this family's skeleton, ready to draw: the pipeline's skeleton stage.
pub fn grow(family: &Family) -> Result<Tree> {
    Ok(pipeline::skeleton(pipeline::GrowInput::of(family))?.tree)
}

/// Grows the skeleton, plaits the wood surface and places the culled foliage.
pub fn build(family: &Family) -> Result<TreeMesh> {
    assembled(pipeline::build(family, pipeline::Request::mesh())?.outputs)
}

/// Plaits the wood surface and places the culled foliage on a grown skeleton.
pub fn assemble(tree: &Tree, family: &Family) -> Result<TreeMesh> {
    let inputs = pipeline::Inputs::of(family);
    assembled(pipeline::outputs(tree, &inputs, pipeline::Request::mesh())?)
}

/// The pipeline's last stage: wood and leaves under their union bounds.
pub(crate) fn assembled(outputs: pipeline::Outputs) -> Result<TreeMesh> {
    let missing = Error::InvalidInput("mesh needs wood and leaves");
    let (Some(wood), Some(leaves), Some(element)) = (outputs.wood, outputs.leaves, outputs.element)
    else {
        return Err(missing);
    };
    let bounds = union(wood.bounds, leaves.bounds.map(Bounds::from))
        .ok_or(Error::InvalidInput("mesh has no geometry"))?;
    Ok(TreeMesh {
        wood,
        foliage: Foliage {
            element,
            instances: leaves.instances,
        },
        bounds,
    })
}
