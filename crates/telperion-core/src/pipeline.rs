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
use stage::Seat;
use std::sync::OnceLock;

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
    let twig = stage::twig(family);
    // Leaves are placed for their own sake, or for a field the plan cannot
    // describe; placed with surface contact, they sit on the rings.
    let places = request.leaves
        || (request.field.is_some() && !plan::supports(family.canopy, twig.clone().ok()));
    let seats = places && family.canopy.surface_contact > 0.0;
    let rings = OnceLock::new();
    let sweep = || stage::rings(tree, family, request, seats);
    let shared = || rings.get_or_init(sweep).as_ref().map_err(Clone::clone);
    let (wood, planned) = both(
        request.schedule == Schedule::Concurrent && request.wood && places,
        || {
            let wood = |r: &(_, _)| stage::wood(tree, family, request, &r.0);
            request.wood.then(|| shared().and_then(wood)).transpose()
        },
        || {
            let prepared = stage::prepare(tree, family, request, &twig, places)?;
            let seat = match (seats, request.wood) {
                (false, _) => Ok(Seat::Free),
                (true, false) => Ok(Seat::Own),
                (true, true) => shared().map(|r| Seat::Shared(&r.0)),
            };
            let twig = twig.clone().ok();
            let made = match seat {
                Ok(seat) => stage::leaves_and_field(tree, family, request, &prepared, twig, seat),
                Err(error) => (Err(error), Ok(None)),
            };
            Ok((prepared, made))
        },
    );
    // The wood's error answers first, as it did run in turn.
    let wood = wood?;
    let (prepared, (leaves, field)) = planned?;
    let (mut leaves, mut field) = (leaves?, field?);
    let rings = rings.into_inner().transpose()?;
    let mut stages = Stages {
        plan_ms: prepared.ms,
        ..Stages::default()
    };
    if let Some((_, ms)) = rings {
        stages.rings_ms = ms;
    }
    if let Some((_, ms)) = wood {
        stages.wood_ms = ms;
    }
    if let Some((_, [own, placement, cull])) = leaves {
        stages.rings_ms += own;
        [stages.placement_ms, stages.cull_ms] = [placement, cull];
    }
    if request.field.is_some() && field.is_none() {
        let placed = leaves.as_ref().map(|(l, _)| &l.instances);
        let placed = placed.zip(prepared.element.as_ref());
        field = Some(stage::placed_field(tree, request, placed)?);
    }
    if let Some((_, ms)) = field {
        stages.field_ms = ms;
    }
    if !request.leaves {
        for (l, _) in leaves.iter_mut() {
            l.instances = Instances::new(l.instances.reference);
        }
    }
    let structure = request
        .structure
        .then(|| stage::structure(tree))
        .transpose()?;
    #[cfg(test)]
    tests::count(|t| {
        let own = seats && !request.wood && leaves.is_some();
        t.sweeps += u32::from(rings.is_some()) + u32::from(own);
        t.elements += u32::from(prepared.element.is_some());
    });
    let wood = wood
        .zip(rings)
        .map(|((faces, _), (rings, _))| rings.into_mesh(faces));
    Ok(Outputs {
        element: prepared.element,
        plan: prepared.leaf_plan,
        wood,
        leaves: leaves.map(|(l, _)| l),
        field: field.map(|(f, _)| f),
        structure,
        stages,
    })
}

/// Runs `a` and `b`, `a` on a thread of its own where `concurrent` asks and
/// the target has threads, else `a` first. The pair comes back in that order
/// either way.
fn both<A: Send, B>(
    concurrent: bool,
    a: impl FnOnce() -> A + Send,
    b: impl FnOnce() -> B,
) -> (A, B) {
    #[cfg(not(target_arch = "wasm32"))]
    if concurrent {
        #[cfg(test)]
        tests::count(|t| t.split = true);
        // The stage waits in a slot the thread takes it from, so a thread
        // the system refuses leaves it here to run in turn.
        let slot = std::sync::Mutex::new(Some(a));
        let take = || slot.lock().ok().and_then(|mut a| a.take());
        return std::thread::scope(|scope| {
            let spawned = std::thread::Builder::new()
                .stack_size(STACK)
                .spawn_scoped(scope, || take().map(|a| a()));
            let Ok(handle) = spawned else {
                let a = take().expect("a refused thread leaves its stage")();
                return (a, b());
            };
            let b = b();
            let a = handle
                .join()
                .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
                .expect("the thread ran its stage");
            (a, b)
        });
    }
    #[cfg(target_arch = "wasm32")]
    let _ = concurrent;
    (a(), b())
}
#[cfg(not(target_arch = "wasm32"))]
const STACK: usize = 8 << 20;

mod stage;
#[cfg(test)]
mod tests;
