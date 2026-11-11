//! The pipeline's stages as tools and tests reach them: a family's grown
//! tree and its expansion, and the CPU steps that make its artifacts,
//! never a stage to chain. `docs/pipeline.md` has the rationale.
use super::{grown_envelope, GrowInput, Inputs, Request};
use crate::{
    envelope::Envelope,
    math::Vec3,
    mesh::{self, TreeMesh},
    pipeline::foliage::{
        self,
        prepared::{self, PreparedStations},
        CanopyParams, Element, ElementParams, Reference, TwigPlacement,
    },
    pipeline::surface::{self, Curve, SurfaceMesh},
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
    let inputs = Inputs::of(family, grown_envelope(&tree));
    Ok(Grown { tree, inputs })
}

/// What a family's tree grows by: the engine's species and the request.
pub struct Growth {
    pub species: telperion_space::Species,
    pub request: telperion_space::Request,
}

impl Growth {
    /// The year the tree is drawn in, `fraction` into its last, by its
    /// seed's PA, the first (`Tree::year`; fn-270 R5b).
    pub fn year(&self, fraction: f64) -> Option<crate::tree::Year> {
        let s = telperion_space::unfold::Season::of(self.species.states.first()?);
        let age = f64::from(self.request.age) - 1.0 + fraction.min(1.0);
        Some(crate::tree::Year {
            age,
            budburst: s.budburst,
            elongation: s.elongation,
            expansion: s.expansion,
        })
    }
}

/// The family's growth (`Growth`). A family with no engine species is
/// refused by name.
pub fn growth(family: &Family) -> Result<Growth> {
    let (species, request) = super::space::growth(GrowInput::of(family))?;
    Ok(Growth { species, request })
}

/// Stage 3's tree-free checks, in the order their errors answer: the
/// element and its own check, the twig placement, the shell depth and the
/// canopy's rows.
fn planned(inputs: &Inputs) -> Result<(Element, TwigPlacement)> {
    let plan = &inputs.plan;
    let element = foliage::build_element(plan.element)?;
    element.validate()?;
    let twig = plan.twig.clone()?;
    check_shell(inputs)?;
    foliage::canopy_rows(plan.canopy, Some(twig))?;
    Ok((element, twig))
}

/// The shell depth, refused unless a share.
fn check_shell(inputs: &Inputs) -> Result<()> {
    let shell = inputs.leaves.shell_depth;
    if !shell.is_finite() || !(0.0..=1.0).contains(&shell) {
        return Err(Error::InvalidInput("shell depth"));
    }
    Ok(())
}

/// A solved tree the caller already holds, read with this family's inputs
/// and the envelope it grew to: the same expansion `grow` leads to, for a
/// tree the caller was handed (its tests' hand-built trees). Growing a
/// tree stays inside the pipeline: nothing outside it converts the engine's
/// structure (fn-198).
pub fn expand(tree: Tree, family: &Family) -> Result<Expansion> {
    let inputs = Inputs::of(family, grown_envelope(&tree));
    Grown { tree, inputs }.expansion()
}

/// The leaf element built from its rows, as the plan stage builds it.
pub fn element(params: ElementParams) -> Result<Element> {
    foliage::build_element(params)
}

impl Grown {
    /// Stage 3's descriptors, in the order their errors answer: the leaf
    /// bases hung on the stems, then `planned`'s, then the reach the box
    /// leaves are quantised against is grown by.
    pub fn expansion(self) -> Result<Expansion> {
        let (mut tree, inputs) = (self.tree, self.inputs);
        super::clothe(&mut tree, &inputs)?;
        let (element, twig) = planned(&inputs)?;
        let reference = Reference::grown(&tree, inputs.plan.reach.clone()?);
        Ok(Expansion {
            tree,
            inputs,
            element,
            twig,
            reference,
        })
    }
}

/// A grown tree prepared for expansion.
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
    pub fn reference(&self) -> Reference {
        self.reference
    }
    fn envelope(&self) -> Envelope {
        self.inputs.leaves.envelope
    }
    fn canopy(&self) -> CanopyParams {
        self.inputs.leaves.canopy
    }

    /// The corners of the leaf box of the nodes `keep` keeps, and the
    /// root's position: combined by their corners, exactly the box of the
    /// whole (`reference`).
    pub fn wood_box(&self, keep: impl Fn(usize) -> bool) -> Result<(Vec3, Vec3)> {
        Ok(self.inputs.plan.reach.clone()?.wood_box(&self.tree, keep))
    }

    /// Stations with contact rings of their own; `None` where the family has
    /// no regular stations.
    pub fn stations(&self) -> Result<Option<PreparedStations>> {
        let (envelope, canopy) = (self.envelope(), self.canopy());
        let params = &self.inputs.surface.params;
        prepared::prepare_stations(&self.tree, envelope, canopy, Some(self.twig), params)
    }
    /// The plan's leaf-bearing runs and the twig layer they are walked
    /// under: the tests that count a run's stations by its wood (fn-354
    /// R5). `None` where the family has no regular stations.
    #[cfg(test)]
    pub(crate) fn runs(&self) -> Result<Option<(Vec<foliage::plan::Run>, TwigPlacement)>> {
        let runs =
            foliage::plan::runs(&self.tree, self.envelope(), self.canopy(), Some(self.twig))?;
        Ok(runs.map(|r| (r, self.twig)))
    }
    /// The wood as curves (fn-208): every run's ring points and frames, the
    /// run table and the clusters a view culls by, which the wood is surfaced
    /// from.
    pub fn curve(&self) -> Result<Curve> {
        let s = &self.inputs.surface;
        surface::curve(&self.tree, s.height, &s.params)
    }
    /// The wood the CPU builder sweeps.
    pub fn wood(&self) -> Result<SurfaceMesh> {
        let s = &self.inputs.surface;
        surface::build(&self.tree, s.height, &s.params)
    }
    /// The pipeline's own build of this tree.
    pub fn mesh(&self) -> Result<TreeMesh> {
        mesh::assembled(super::outputs(&self.tree, &self.inputs, Request::mesh())?)
    }
}

#[cfg(test)]
mod tests;
