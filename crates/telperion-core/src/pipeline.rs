//! The mature build, written once. A request names the outputs it wants;
//! the pipeline runs the stages those outputs need, each once, in a fixed
//! dependency order:
//!
//! 1. Family: the parameter table the caller hands in.
//! 2. Skeleton: the solved tree, apical twigs cleared under a frond crown
//!    and shed leaf bases hung on the stems.
//! 3. Plan: what is prepared before any triangle or leaf exists - the leaf
//!    element, the leaf plan a field reads, the quantisation box, and, where
//!    leaves are seated on the wood, the one ring sweep wood and leaves share.
//! 4. Outputs: the wood surface, the leaves, the field and the structure.
//! 5. Tree mesh: wood and leaves with their union bounds (`mesh::build`).
//!
//! Wood and leaves read only the skeleton and the plan, so where the target
//! has threads they run concurrently; a planned field runs beside them too.
//! Every stage keeps its own random stream and results join in stage order,
//! so the bytes never depend on the schedule, and when stages fail the
//! earliest one's error is returned.
use crate::{
    branching,
    field::Field,
    foliage::{self, plan, Element, Instances},
    presets::Family,
    surface::SurfaceMesh,
    tree::Tree,
    Result,
};

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

/// What each stage cost and how often the shared work ran. A stage that did
/// not run reads zero; under concurrency each time is that stage's own wall
/// time and `total_ms` the whole request.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Stages {
    pub skeleton_ms: f64,
    /// The leaf plan a field reads.
    pub plan_ms: f64,
    /// The ring sweep leaf contacts, and the wood beside them, read.
    pub rings_ms: f64,
    pub wood_ms: f64,
    pub placement_ms: f64,
    /// The cull and the retained leaves' bounds.
    pub cull_ms: f64,
    pub field_ms: f64,
    pub total_ms: f64,
    /// Ring sweeps run: at most one a request.
    pub sweeps: u32,
    /// Leaf elements built: at most one a request.
    pub elements: u32,
    /// Whether the wood ran on a thread of its own beside the leaves.
    pub concurrent: bool,
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
pub fn skeleton(family: &Family) -> Result<Skeleton> {
    let report = branching::generate(&family.skeleton, family.radii)?;
    let mut tree = report.tree;
    if family.canopy.rosette_fronds > 0 {
        branching::clear_apical_twigs(&mut tree)?;
    }
    branching::clothe_leaf_bases(&mut tree, &family.canopy)?;
    Ok(Skeleton {
        tree,
        shed: report.shed,
    })
}

/// Runs every stage the request needs, from the family on.
pub fn build(family: &Family, request: Request) -> Result<Built> {
    let started = (request.clock)();
    let skeleton = skeleton(family)?;
    let grown = (request.clock)();
    let mut outputs = outputs(&skeleton.tree, family, request)?;
    outputs.stages.skeleton_ms = grown - started;
    outputs.stages.total_ms = (request.clock)() - started;
    Ok(Built { skeleton, outputs })
}

/// Runs stages 3 and 4 on a skeleton already grown.
pub fn outputs(tree: &Tree, family: &Family, request: Request) -> Result<Outputs> {
    let clock = request.clock;
    let started = clock();
    let concurrent = request.schedule == Schedule::Concurrent;
    let mut stages = Stages::default();
    let twig = stage::twig(family)?;
    // A field the plan cannot describe reads placed leaves instead.
    let places =
        request.leaves || (request.field.is_some() && !plan::supports(family.canopy, Some(twig)));
    // Leaves seated on the wood read the rings of one sweep. Side by side
    // with the wood, the sweep runs first as the one Plan artifact the wood
    // reads; in turn, the wood sweeps and the leaves read its vertices.
    let seats = places && family.canopy.surface_contact > 0.0;
    let beside = concurrent && THREADS && request.wood && (places || request.field.is_some());
    let slot = stage::RingSlot::default();
    let mut given = None;
    if beside && seats {
        let rings = stage::rings(tree, family, request, &mut stages)?;
        given = Some(rings.clone());
        *stage::lock(&slot) = Some(rings);
    }
    let wood_job = || stage::wood(tree, family, request, given);
    let mut leafy_job = |wood: Option<&SurfaceMesh>| -> Result<_> {
        let prep = stage::prepare(tree, family, request, twig, places, &mut stages)?;
        let rings = (&slot, wood);
        let leaves_job = || stage::leaves(tree, family, request, twig, rings, &prep);
        let field_job = || stage::planned_field(tree, request, &prep);
        let side_by_side = concurrent && places && prep.leaf_plan.is_some();
        let (leaves, field, _) = both(side_by_side, leaves_job, field_job);
        Ok((prep, leaves, field))
    };
    let (wood, leafy, split) = if beside {
        both(true, wood_job, || leafy_job(None))
    } else {
        let wood = wood_job();
        let mesh = wood.as_ref().ok().and_then(|w| w.as_ref()).map(|w| &w.0);
        let leafy = leafy_job(mesh);
        (wood, leafy, false)
    };
    // Results join in stage order, so the earliest failure is the answer.
    let (prep, leaves, field) = leafy?;
    let wood = wood?;
    let mut out = None;
    if let Some(placed) = leaves? {
        let [rings_ms, placement_ms, cull_ms] = placed.ms;
        stages.rings_ms += rings_ms;
        (stages.placement_ms, stages.cull_ms) = (placement_ms, cull_ms);
        stages.sweeps += u32::from(placed.swept);
        out = Some(placed.leaves);
    }
    let mut field = field?;
    stages.concurrent = split;
    if let Some((_, ms, swept)) = wood {
        stages.wood_ms = ms;
        stages.sweeps += u32::from(swept);
    }
    if request.field.is_some() && field.is_none() {
        let start = clock();
        let placed = out
            .as_ref()
            .map(|l| &l.instances)
            .zip(prep.element.as_ref());
        field = Some((Field::new(tree, placed)?, clock() - start));
    }
    if let Some((_, ms)) = field {
        stages.field_ms = ms;
    }
    if !request.leaves {
        for leaves in out.iter_mut() {
            leaves.instances = Instances::new(leaves.instances.reference);
        }
    }
    let structure = if request.structure {
        Some(stage::structure(tree)?)
    } else {
        None
    };
    stages.total_ms = clock() - started;
    Ok(Outputs {
        element: prep.element,
        plan: prep.leaf_plan,
        wood: wood.map(|(w, _, _)| w),
        leaves: out,
        field: field.map(|(f, _)| f),
        structure,
        stages,
    })
}

/// Runs `a` and `b`, `a` on a thread of its own where `concurrent` asks and
/// the target has threads, else `a` first. The pair comes back in that order
/// either way, with whether `a` had its own thread.
fn both<A: Send, B>(
    concurrent: bool,
    a: impl FnOnce() -> A + Send,
    b: impl FnOnce() -> B,
) -> (A, B, bool) {
    #[cfg(not(target_arch = "wasm32"))]
    if concurrent {
        // The stage waits in a slot the thread takes it from, so a refused
        // spawn leaves it here to run in turn.
        let slot = std::sync::Mutex::new(Some(a));
        let take = || slot.lock().ok().and_then(|mut s| s.take());
        return std::thread::scope(|scope| {
            let spawned = std::thread::Builder::new()
                .stack_size(STACK)
                .spawn_scoped(scope, || take().map(|a| a()));
            let Ok(handle) = spawned else {
                let a = take().expect("a refused spawn leaves its stage")();
                return (a, b(), false);
            };
            let b = b();
            let a = handle
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                .expect("the spawned thread ran its stage");
            (a, b, true)
        });
    }
    #[cfg(target_arch = "wasm32")]
    let _ = concurrent;
    let a = a();
    (a, b(), false)
}
#[cfg(not(target_arch = "wasm32"))]
const STACK: usize = 8 << 20;
/// Whether this target runs a stage on a thread of its own.
const THREADS: bool = cfg!(not(target_arch = "wasm32"));

mod stage;
#[cfg(test)]
mod tests;
