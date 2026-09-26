//! The mature build, written once. A request names the outputs it wants;
//! the pipeline runs the stages those outputs need, each once, in a fixed
//! dependency order:
//!
//! 1. Family: the parameter table the caller hands in.
//! 2. Skeleton: the solved tree, apical twigs cleared under a frond crown
//!    and shed leaf bases hung on the stems.
//! 3. Plan: what is prepared before any triangle or leaf exists - the leaf
//!    element, the leaf plan a field reads, the quantisation box and the
//!    wood's rings, swept once for the wood and for leaves seated on it.
//! 4. Outputs: the wood surface, the leaves, the field and the structure.
//! 5. Tree mesh: wood and leaves with their union bounds (`mesh::build`).
//!
//! Wood and leaves read only the skeleton and the plan, so where the target
//! has threads they run concurrently, and a planned field beside the leaves.
//! The rings are swept once, by whichever of the wood and seated leaves
//! reaches them first; the other waits and reads them in place. Where no
//! wood is drawn, seated leaves sweep bare rings of their own. Every stage
//! keeps its own random stream and results join in a fixed order, so neither
//! the bytes nor the error depend on the schedule: where stages fail, the
//! answer is the error the build met first run in turn - the wood's, then
//! the Plan's, the leaves', the field's.
//! The rings' error is the wood's where the wood is drawn, else the leaves'.
use crate::{
    branching,
    field::Field,
    foliage::{self, plan, Element, Instances},
    presets::Family,
    surface::SurfaceMesh,
    tree::Tree,
    Result,
};
pub(crate) use input::{GrowInput, Inputs, PlanInput};

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
    pub leaves: bool,
    /// A field at the family's limb order (`Some(None)`) or at a stated one.
    pub field: Option<Option<u32>>,
    pub structure: bool,
    pub schedule: Schedule,
    /// Milliseconds from any fixed origin; stage timings are its differences.
    pub clock: fn() -> f64,
}
impl Default for Request {
    fn default() -> Self {
        Self {
            wood: false,
            leaves: false,
            field: None,
            structure: false,
            schedule: Schedule::default(),
            clock,
        }
    }
}
impl Request {
    /// Wood and leaves: what a tree mesh is made of.
    pub fn mesh() -> Self {
        Self {
            wood: true,
            leaves: true,
            ..Self::default()
        }
    }
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

/// Stage 2's artifact: the solved tree and how many nodes the shell shed.
#[derive(Debug)]
pub struct Skeleton {
    pub tree: Tree,
    pub shed: usize,
}

/// The placed leaves after the cull. A field that read them for a request
/// that did not ask for leaves keeps only the counts and the box.
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
    pub leaves: Option<Leaves>,
    pub field: Option<Field>,
    pub structure: Option<Structure>,
    pub stages: Stages,
}

pub struct Built {
    pub skeleton: Skeleton,
    pub outputs: Outputs,
}

/// Stage 2: grows the family's skeleton, ready to draw. An apex that bears a
/// rosette bears no twig: where the canopy stands a frond crown, the twig
/// wood above every stem apex is dropped, so the fronds stand on bare wood.
/// Where the canopy keeps the bases of its shed fronds, they are hung on
/// every stem as wood of their own, after the radius solve so no base
/// thickens the trunk.
pub(crate) fn skeleton(input: GrowInput) -> Result<Skeleton> {
    let report = branching::generate(input.skeleton, input.radii)?;
    let mut tree = report.tree;
    if input.canopy.rosette_fronds > 0 {
        branching::clear_apical_twigs(&mut tree)?;
    }
    branching::clothe_leaf_bases(&mut tree, input.canopy)?;
    Ok(Skeleton {
        tree,
        shed: report.shed,
    })
}

/// Runs every stage the request needs, from the family on. The family is
/// read once, into each stage's own input.
pub fn build(family: &Family, request: Request) -> Result<Built> {
    let started = (request.clock)();
    let skeleton = skeleton(GrowInput::of(family))?;
    let grown = (request.clock)();
    let inputs = Inputs::of(family);
    let mut outputs = outputs(&skeleton.tree, &inputs, request)?;
    outputs.stages.skeleton_ms = grown - started;
    outputs.stages.total_ms = (request.clock)() - started;
    Ok(Built { skeleton, outputs })
}

/// Runs stages 3 and 4 on a skeleton already grown. Without the `geometry`
/// feature only the plan's outputs are built: the field from the leaf plan
/// and the structure. Wood, leaves and a field a family's plan cannot
/// describe are refused there, since they need placement.
pub(crate) fn outputs(tree: &Tree, inputs: &Inputs, request: Request) -> Result<Outputs> {
    #[cfg(feature = "geometry")]
    return drawn::outputs(tree, inputs, request);
    #[cfg(not(feature = "geometry"))]
    planned::outputs(tree, inputs, request)
}

#[cfg(feature = "geometry")]
mod drawn;
#[cfg(feature = "geometry")]
pub mod executor;
mod input;
#[cfg(not(feature = "geometry"))]
mod planned;
mod stage;
#[cfg(test)]
mod tests;
