//! The interface a second executor builds through: the GPU executor in
//! telperion-render, and the growth path. It hands out prepared artifacts
//! and the CPU steps that make them, never a stage to chain; each step is
//! synchronous, so the executor keeps its own schedule and overlap around
//! them. `docs/pipeline.md` has the rationale.
pub use super::input::LeafInput;
use super::{GrowInput, Inputs, Request};
use crate::{
    envelope::Envelope,
    mesh::{self, TreeMesh},
    pipeline::foliage::{
        self,
        prepared::{self, PreparedStations},
        CanopyParams, Element, ElementParams, Instances, Leaf, Reference, TwigPlacement,
    },
    pipeline::surface::{
        self,
        compact::{CompactSurface, CompactWithContacts},
        prepared::{PreparedSurface, PreparedWithContacts},
        SurfaceMesh,
    },
    presets::Family,
    tree::Tree,
    Error, Result,
};

/// The skeleton stage's artifact and the inputs every later stage reads.
pub struct Grown {
    tree: Tree,
    inputs: Inputs,
}

/// Stage 2: the family read once into its stage inputs, and its skeleton.
pub fn grow(family: &Family) -> Result<Grown> {
    let tree = super::skeleton(GrowInput::of(family))?.tree;
    let inputs = Inputs::of(family);
    Ok(Grown { tree, inputs })
}

/// A solved tree the caller already holds, read with this family's inputs:
/// the same expansion `grow` leads to, for a tree the executor was handed
/// (its tests' hand-built trees). Building a tree stays inside the pipeline.
pub fn expand(tree: Tree, family: &Family) -> Result<Expansion> {
    let inputs = Inputs::of(family);
    Grown { tree, inputs }.expansion()
}

/// The leaf element built from its rows, as the plan stage builds it.
pub fn element(params: ElementParams) -> Result<Element> {
    foliage::build_element(params)
}

impl Grown {
    /// Stage 3's descriptors, in the order their errors answer: the element
    /// and its own check, the twig placement, the shell depth and the box
    /// leaves are quantised against.
    pub fn expansion(self) -> Result<Expansion> {
        let plan = &self.inputs.plan;
        let element = foliage::build_element(plan.element)?;
        element.validate()?;
        let twig = plan.twig.clone()?;
        let shell = self.inputs.leaves.shell_depth;
        if !shell.is_finite() || !(0.0..=1.0).contains(&shell) {
            return Err(Error::InvalidInput("shell depth"));
        }
        let reference = plan.reference.clone()?;
        Ok(Expansion {
            tree: self.tree,
            inputs: self.inputs,
            element,
            twig,
            reference,
        })
    }
}

/// A grown tree prepared for expansion by another executor.
pub struct Expansion {
    tree: Tree,
    inputs: Inputs,
    element: Element,
    twig: TwigPlacement,
    reference: Reference,
}

impl Expansion {
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
    pub fn element(&self) -> &Element {
        &self.element
    }
    pub fn into_element(self) -> Element {
        self.element
    }
    pub fn twig(&self) -> TwigPlacement {
        self.twig
    }
    pub fn reference(&self) -> Reference {
        self.reference
    }
    /// The rows leaves are placed and culled by.
    pub fn leaves(&self) -> LeafInput {
        self.inputs.leaves
    }
    fn envelope(&self) -> Envelope {
        self.inputs.leaves.envelope
    }
    fn canopy(&self) -> CanopyParams {
        self.inputs.leaves.canopy
    }

    /// Whether the family's leaves stand on regular twig stations.
    pub fn supports_stations(&self) -> bool {
        prepared::supports_stations(self.canopy(), Some(self.twig))
    }
    /// Whether the wood's section is round: no lobes, or lobes of no depth.
    pub fn round_section(&self) -> bool {
        let s = &self.inputs.surface.params;
        s.lobes == 0 || s.lobe_depth == 0.0
    }
    /// Whether leaves of some size are seated on the wood.
    pub fn seats(&self) -> bool {
        let c = self.canopy();
        c.surface_contact > 0.0 && c.size != 0.0
    }

    /// The wood's compact rings.
    pub fn compact(&self) -> Result<CompactSurface> {
        let s = &self.inputs.surface;
        surface::compact::prepare(&self.tree, s.height, &s.params)
    }
    /// The wood's compact rings with the node contacts seated leaves read.
    pub fn compact_with_contacts(&self) -> Result<CompactWithContacts<'_>> {
        let s = &self.inputs.surface;
        surface::compact::prepare_with_contacts(&self.tree, s.height, &s.params)
    }
    /// Stations tied to the compact rings' emitted vertices.
    pub fn compact_stations(
        &self,
        shared: &CompactWithContacts<'_>,
    ) -> Result<Option<PreparedStations<()>>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        prepared::prepare_compact_stations(shared, envelope, canopy, Some(self.twig))
    }
    /// Stations with contact rings of their own; `None` where the family has
    /// no regular stations.
    pub fn stations(&self) -> Result<Option<PreparedStations>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        let params = &self.inputs.surface.params;
        prepared::prepare_stations(&self.tree, envelope, canopy, Some(self.twig), params)
    }
    /// The canonical float32 wood with its node contacts; `None` where the
    /// CPU builder must sweep it.
    pub fn prepared_with_contacts(&self) -> Result<Option<PreparedWithContacts<'_>>> {
        let s = &self.inputs.surface;
        surface::prepared::prepare_with_contacts(&self.tree, s.height, &s.params)
    }
    /// Stations borrowing the canonical wood's contacts.
    pub fn shared_stations<'a>(
        &self,
        shared: &'a PreparedWithContacts<'_>,
    ) -> Result<Option<PreparedStations<&'a [f32]>>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        prepared::prepare_shared_stations(shared, envelope, canopy, Some(self.twig))
    }
    /// The canonical float32 wood; `None` where the CPU builder must sweep it.
    pub fn prepared_wood(&self) -> Result<Option<PreparedSurface>> {
        let s = &self.inputs.surface;
        surface::prepared::prepare(&self.tree, s.height, &s.params)
    }
    /// The wood the CPU builder sweeps.
    pub fn wood(&self) -> Result<SurfaceMesh> {
        let s = &self.inputs.surface;
        surface::build(&self.tree, s.height, &s.params)
    }
    /// The pipeline's own CPU build of this tree: the reference a GPU
    /// expansion falls back to.
    pub fn mesh(&self) -> Result<TreeMesh> {
        mesh::assembled(super::outputs(&self.tree, &self.inputs, Request::mesh())?)
    }
}

/// The growth path's presentation of one age: the wood on screen swept at
/// the specimen's height, the element, the leaves its record placed with the
/// short shoots and rosette that wood bears, culled against the envelope of
/// that age, under the union bounds. `envelope` and `owners` (each recorded
/// leaf's shoot, read only where limbs clump) answer in the order they did.
pub(crate) fn present(
    tree: &Tree,
    height: f64,
    inputs: &Inputs,
    leaves: Vec<Leaf>,
    envelope: impl FnOnce() -> Result<Envelope>,
    owners: impl FnOnce() -> Vec<u32>,
) -> Result<TreeMesh> {
    let wood = surface::build(tree, height, &inputs.surface.params)?;
    let element = foliage::build_element(inputs.plan.element)?;
    let mut instances = Instances::new(inputs.plan.reference.clone()?);
    instances.leaves = leaves;
    let envelope = envelope()?;
    let (seed, canopy) = (inputs.leaves.seed, inputs.leaves.canopy);
    if canopy.limb_clumping > 0.0 {
        let owners = owners();
        foliage::place_short_shoots_clumped(tree, envelope, seed, canopy, owners, &mut instances)?;
    } else {
        foliage::place_short_shoots(tree, envelope, seed, canopy, &mut instances)?;
    }
    foliage::place_rosette(tree, seed, canopy, &mut instances)?;
    let shell = inputs.leaves.shell_depth;
    let instances = foliage::cull(instances, &element, envelope, shell)?;
    let bounds = mesh::union(wood.bounds, instances.bounds(&element)?.map(Into::into)).unwrap_or(
        surface::Bounds {
            min: crate::math::Vec3::ZERO,
            max: crate::math::Vec3::Y * 0.01,
        },
    );
    Ok(TreeMesh {
        wood,
        foliage: mesh::Foliage { element, instances },
        bounds,
    })
}

#[cfg(test)]
mod tests;
