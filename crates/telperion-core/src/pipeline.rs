//! The mature build, written once. A request names the outputs it wants;
//! the pipeline runs the stages those outputs need, each once, in a fixed
//! dependency order:
//!
//! 1. Family: the parameter table the caller hands in.
//! 2. Skeleton: the solved tree, apical twigs cleared under a frond crown.
//! 3. Plan: the shed leaf bases hung on the stems of the solved tree, grown
//!    or handed in; then what is prepared before any triangle or leaf
//!    exists - the leaf element, the leaf plan a field reads, the
//!    quantisation box and the wood's rings, swept once for the wood and
//!    for leaves seated on it.
//! 4. Outputs: the wood surface, the leaves, the field and the structure.
//! 5. Tree mesh: wood and leaves with their union bounds (`mesh::build`).
//!
//! Wood and leaves read only the skeleton and the plan, so where the target
//! has threads they run concurrently, and the field beside the leaves.
//! The rings are swept once, by whichever of the wood and seated leaves
//! reaches them first; the other waits and reads them in place. Where no
//! wood is drawn, seated leaves sweep bare rings of their own. Every stage
//! keeps its own random stream and results join in a fixed order, so neither
//! the bytes nor the error depend on the schedule: where stages fail, the
//! answer is the error the build met first run in turn - the wood's, then
//! the Plan's, the leaves', the field's.
//! The rings' error is the wood's where the wood is drawn, else the leaves'.
use crate::{presets::Family, tree::Tree, Result};
#[cfg(feature = "geometry")]
use field::Grid;
use foliage::{plan, Element, Instances};
pub(crate) use input::{grown_envelope, GrowInput, Inputs, PlanInput};
use surface::SurfaceMesh;

/// How stages independent of each other are run. `Concurrent` falls back to
/// one stage at a time where the target has no threads.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Schedule {
    #[default]
    Concurrent,
    Serial,
}

/// The outputs a caller wants. A stage nobody needs, directly or through a
/// stage that reads it, never runs.
#[derive(Debug, Clone, Copy)]
pub struct Request {
    pub wood: bool,
    /// The wood as curves, which the renderer surfaces (fn-208).
    pub curve: bool,
    pub leaves: bool,
    /// The field read out to a grid (fn-257 R8).
    pub field: Option<FieldRequest>,
    pub structure: bool,
    pub schedule: Schedule,
    /// The time of year flowers are drawn at, as a share of the year; none
    /// is the flowering window's middle (fn-216).
    pub season: Option<f64>,
    /// Milliseconds from any fixed origin; stage timings are its differences.
    pub clock: fn() -> f64,
}
impl Default for Request {
    fn default() -> Self {
        Self {
            wood: false,
            curve: false,
            leaves: false,
            field: None,
            structure: false,
            schedule: Schedule::default(),
            season: None,
            clock,
        }
    }
}
impl Request {
    /// Wood, its curve and leaves: what a tree mesh is made of.
    pub fn mesh() -> Self {
        Self {
            wood: true,
            curve: true,
            leaves: true,
            ..Self::default()
        }
    }
}

/// A field request names its grid: `n` cells a side over the tree's box
/// (`field::Frame`), its limb systems opened by laterals of `limb_order` or
/// less (the family's `clumpSystemOrder` where none is given).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FieldRequest {
    pub n: u32,
    pub limb_order: Option<u32>,
}

/// The monotonic clock native stages are timed by; zero where the target
/// has none, and a caller there passes its own.
pub fn clock() -> f64 {
    #[cfg(not(target_arch = "wasm32"))]
    {
        static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
        ORIGIN
            .get_or_init(std::time::Instant::now)
            .elapsed()
            .as_secs_f64()
            * 1000.0
    }
    #[cfg(target_arch = "wasm32")]
    {
        0.0
    }
}

/// Stage 2's artifact: the grown tree.
#[derive(Debug)]
pub struct Skeleton {
    pub tree: Tree,
}

/// The placed leaves after the cull.
#[derive(Debug)]
pub struct Leaves {
    pub instances: Instances,
    /// Leaves placed before the cull.
    pub placed: usize,
    /// Leaves the cull kept.
    pub retained: usize,
    /// The union of every retained leaf, where leaves were asked for.
    pub bounds: Option<foliage::Bounds>,
}

/// Node records for a consumer that reads the skeleton itself: six floats a
/// node (position, radius, start radius, base radius) and three words (parent
/// or `u32::MAX`, branch, kind as structural 0, branch 1, twig 2).
#[derive(Debug, Default)]
pub struct Structure {
    pub nodes: Vec<f64>,
    pub topology: Vec<u32>,
}

/// What each stage cost. A stage that did not run reads zero; under
/// concurrency each time is that stage's own wall time.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Stages {
    pub skeleton_ms: f64,
    /// The leaf plan a field reads.
    pub plan_ms: f64,
    /// The rings the wood and seated leaves read, swept once.
    pub rings_ms: f64,
    /// The wood's mesh step around its rings.
    pub wood_ms: f64,
    /// Placement, with the leaves' contact bounds.
    pub placement_ms: f64,
    /// The cull and the retained leaves' bounds.
    pub cull_ms: f64,
    pub field_ms: f64,
    /// The whole request from the family on; zero from `outputs` alone.
    pub total_ms: f64,
}

/// Everything a request produced past the skeleton.
#[derive(Default)]
pub struct Outputs {
    pub element: Option<Element>,
    pub plan: Option<plan::Plan>,
    pub wood: Option<SurfaceMesh>,
    #[cfg(feature = "geometry")]
    pub curve: Option<surface::Curve>,
    pub leaves: Option<Leaves>,
    /// The flowers, where the tree's ages flower and leaves were asked
    /// for: their element and their florets (fn-216). None on every tree
    /// that flowers nowhere.
    #[cfg(feature = "geometry")]
    pub organ: Option<crate::mesh::Foliage>,
    /// The field read out to the request's grid (fn-257 R8).
    #[cfg(feature = "geometry")]
    pub field: Option<Grid>,
    pub structure: Option<Structure>,
    pub stages: Stages,
}

pub struct Built {
    pub skeleton: Skeleton,
    pub outputs: Outputs,
}

/// Stage 2: the tree-space engine grows the family's species, ready to draw
/// (`space.rs`). A family with no engine species is refused by name.
pub(crate) fn skeleton(input: GrowInput) -> Result<Skeleton> {
    Ok(Skeleton {
        tree: space::grow(input)?,
    })
}

/// The scaffold's grower, today's stage 2 before the engine, which no
/// production path reaches: the suite's tests of its stages grow their
/// trees with it until it is removed (fn-198 R0). An apex that bears a
/// rosette bears no twig under a frond crown.
#[cfg(test)]
pub(crate) fn scaffold(family: &Family) -> Result<Tree> {
    let crowned = family.canopy.rosette_fronds > 0.0;
    Ok(branching::crowned(&family.skeleton, family.radii, crowned)?.tree)
}

/// The first step of expansion, which every solved tree passes, grown here
/// or handed in: where the canopy keeps the bases of its shed fronds, they
/// are hung on every stem as wood of their own, after the radius solve so
/// no base thickens the trunk.
pub(crate) fn clothe(tree: &mut Tree, inputs: &Inputs) -> Result<()> {
    branching::clothe_leaf_bases(tree, &inputs.plan.canopy)
}

/// Runs every stage the request needs, from the family on. The family is
/// read once, into each stage's own input. A field's grid is checked
/// before any stage runs.
pub fn build(family: &Family, request: Request) -> Result<Built> {
    if let Some(f) = request.field {
        field::Frame::side(f.n)?;
    }
    let started = (request.clock)();
    let mut skeleton = skeleton(GrowInput::of(family))?;
    let inputs = Inputs::of(family, grown_envelope(&skeleton.tree));
    clothe(&mut skeleton.tree, &inputs)?;
    fitted(&mut skeleton.tree);
    // The skeleton's time keeps the clothing it always held.
    let grown = (request.clock)();
    let mut outputs = outputs(&skeleton.tree, &inputs, request)?;
    outputs.stages.skeleton_ms = grown - started;
    outputs.stages.total_ms = (request.clock)() - started;
    Ok(Built { skeleton, outputs })
}

/// Gives back what growth reserved past the tree's own length before the
/// wood reserves its buffers (fn-291 R2a): a node buffer doubled while it
/// grew keeps up to twice its nodes, and 32-bit Wasm holds the spare through
/// the build.
fn fitted(tree: &mut Tree) {
    tree.nodes.shrink_to_fit();
    tree.sections.shrink_to_fit();
    tree.frames.shrink_to_fit();
    tree.keys.shrink_to_fit();
    tree.grown.shrink_to_fit();
    tree.blooms.shrink_to_fit();
    tree.clusters.shrink_to_fit();
    tree.clustered.shrink_to_fit();
    tree.leafing.shrink_to_fit();
    tree.unfolding.shrink_to_fit();
}

/// Runs stages 3 and 4 on a skeleton already grown. Without the `geometry`
/// feature only the structure is built: wood, leaves and the field, which
/// reads the drawn tree, are refused there by name.
pub(crate) fn outputs(tree: &Tree, inputs: &Inputs, request: Request) -> Result<Outputs> {
    #[cfg(feature = "geometry")]
    return drawn::outputs(tree, inputs, request);
    #[cfg(not(feature = "geometry"))]
    planned::outputs(tree, inputs, request)
}

// The scaffold's parameter groups, its leaf bases and the skeleton's
// validation. Its grower (the scaffold, its twig layer and radius solve)
// compiles only into tests: no production path reaches it since the engine
// is stage 2 (fn-198), and the suite grows its tests' trees with it until
// fn-288 removes it.
mod bias;
mod branching;
mod colonization;
pub mod contract;
mod field;
mod foliage;
// Reached only by the suite, which holds a prediction to the specimen.
#[cfg(all(test, feature = "geometry"))]
mod footprint;
mod radius;
mod space;
/// Growth that continues from any age (fn-270 R2).
pub use space::growth;
mod surface;
mod twigs;
/// What the rest of the crate reads of the stages: the family's own checks.
pub(crate) use {
    branching::validate_skeleton,
    foliage::{build_element, canopy_rows},
    surface::height,
};

#[cfg(feature = "geometry")]
mod drawn;
#[cfg(feature = "geometry")]
pub mod executor;
mod input;
#[cfg(not(feature = "geometry"))]
mod planned;
mod stage;
#[cfg(all(test, feature = "geometry"))]
mod tests;
