//! One engine-neutral mesh per parameter family. Renderers consume this and nothing else.
use crate::{
    foliage,
    math::Vec3,
    pipeline,
    presets::Family,
    surface::{Bounds, SurfaceMesh},
    Error, Result,
};

/// One element mesh drawn once per instance matrix.
#[derive(Debug)]
pub struct Foliage {
    pub element: foliage::Element,
    pub instances: foliage::Instances,
}

/// Everything a renderer needs for one tree: the wood as curves, which the
/// renderer surfaces at the screen's error (fn-208), foliage as one element
/// plus its instances, and the union bounds. `wood` is the same wood as the
/// pipeline's CPU reference build, an indexed surface, for consumers without
/// a GPU: built by `build`, and none from `drawn`, the renderer's tree, which
/// surfaces the curve on the device instead (fn-351). The bounds are the
/// curve's box joined with the leaves' and the flowers', the same either way.
#[derive(Debug)]
pub struct TreeMesh {
    pub wood: Option<SurfaceMesh>,
    #[cfg(feature = "geometry")]
    pub curve: crate::surface::Curve,
    pub foliage: Foliage,
    /// The flowers, drawn as a second foliage with colours of their own;
    /// none where the tree flowers nowhere (fn-216).
    #[cfg(feature = "geometry")]
    pub organ: Option<Foliage>,
    pub bounds: Bounds,
}
impl TreeMesh {
    /// The CPU surface's vertices; none where it was not built (`drawn`).
    pub fn wood_vertices(&self) -> Option<usize> {
        self.wood.as_ref().map(SurfaceMesh::vertices)
    }
    /// The CPU surface's triangles; none where it was not built (`drawn`).
    pub fn wood_triangles(&self) -> Option<usize> {
        self.wood.as_ref().map(SurfaceMesh::triangles)
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

/// Grows this family's skeleton with the scaffold's grower, clothed as the
/// pipeline clothes a tree: for the crate's own tests of today's stages
/// until it is removed (fn-198 R0), under the family's own envelope.
#[cfg(test)]
pub(crate) fn grow(family: &Family) -> Result<crate::tree::Tree> {
    let mut tree = pipeline::scaffold(family)?;
    pipeline::clothe(
        &mut tree,
        &pipeline::Inputs::of(family, family.skeleton.envelope),
    )?;
    Ok(tree)
}

/// Grows the skeleton, plaits the wood surface and places the culled foliage.
pub fn build(family: &Family) -> Result<TreeMesh> {
    assembled(pipeline::build(family, pipeline::Request::mesh())?.outputs)
}

/// The tree a renderer draws: `build`'s curve, leaves, flowers and bounds,
/// with no CPU wood surface, which the renderer never reads (fn-351).
pub fn drawn(family: &Family) -> Result<TreeMesh> {
    assembled(pipeline::build(family, pipeline::Request::drawn())?.outputs)
}

/// Plaits the wood surface and places the culled foliage on a grown
/// skeleton, for the crate's own tests.
#[cfg(test)]
pub(crate) fn assemble(tree: &crate::tree::Tree, family: &Family) -> Result<TreeMesh> {
    let inputs = pipeline::Inputs::of(family, family.skeleton.envelope);
    assembled(pipeline::outputs(tree, &inputs, pipeline::Request::mesh())?)
}

/// The pipeline's last stage: the curve and leaves under their union
/// bounds, with the wood surface where the request built one.
pub(crate) fn assembled(outputs: pipeline::Outputs) -> Result<TreeMesh> {
    let missing = Error::InvalidInput("mesh needs leaves");
    let (Some(leaves), Some(element)) = (outputs.leaves, outputs.element) else {
        return Err(missing);
    };
    let wood = outputs.wood;
    #[cfg(feature = "geometry")]
    let curve = outputs
        .curve
        .ok_or(Error::InvalidInput("mesh needs the wood's curve"))?;
    let bounds = union(curve.bounds(), leaves.bounds.map(Bounds::from));
    #[cfg(feature = "geometry")]
    let bounds = match &outputs.organ {
        Some(o) => union(bounds, o.instances.bounds(&o.element)?.map(Bounds::from)),
        None => bounds,
    };
    let bounds = bounds.ok_or(Error::InvalidInput("mesh has no geometry"))?;
    Ok(TreeMesh {
        wood,
        #[cfg(feature = "geometry")]
        curve,
        foliage: Foliage {
            element,
            instances: leaves.instances,
        },
        #[cfg(feature = "geometry")]
        organ: outputs.organ,
        bounds,
    })
}
