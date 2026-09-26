//! Stages 3 and 4 with the geometry compiled in: the rings, the wood, the
//! leaves placed and culled, and the field read from placed leaves where the
//! leaf plan cannot describe a family.
use super::{
    input::{LeafInput, SurfaceInput},
    stage,
    stage::Prepared,
    Inputs, Leaves, Outputs, Request, Schedule, Stages,
};
use crate::{
    field::Field,
    foliage::{self, plan, Element, Instances, TwigPlacement},
    surface::{self, AttachmentSurface, Faces, Rings, Sweep},
    tree::Tree,
    Result,
};
use std::sync::OnceLock;

/// Stages 3 and 4 with the wood and the leaves compiled in.
pub(super) fn outputs(tree: &Tree, inputs: &Inputs, request: Request) -> Result<Outputs> {
    let (surface, leaf) = (&inputs.surface, &inputs.leaves);
    let twig = inputs.plan.twig();
    // Leaves are placed for their own sake, or for a field the plan cannot
    // describe; placed with surface contact, they sit on the rings.
    let places = request.leaves
        || (request.field.is_some() && !plan::supports(leaf.canopy, twig.clone().ok()));
    let seats = places && leaf.canopy.surface_contact > 0.0;
    let rings = OnceLock::new();
    let sweep = || self::rings(tree, surface, request, seats);
    let shared = || rings.get_or_init(sweep).as_ref().map_err(Clone::clone);
    let (wood, planned) = both(
        request.schedule == Schedule::Concurrent && request.wood && places,
        || {
            let wood = |r: &(_, _)| self::wood(tree, surface, request, &r.0);
            request.wood.then(|| shared().and_then(wood)).transpose()
        },
        || {
            let prepared = stage::prepare(tree, &inputs.plan, request, &twig, places)?;
            let seat = match (seats, request.wood) {
                (false, _) => Ok(Seat::Free),
                (true, false) => Ok(Seat::Own),
                (true, true) => shared().map(|r| Seat::Shared(&r.0)),
            };
            let twig = twig.clone().ok();
            let made = match seat {
                Ok(seat) => leaves_and_field(tree, inputs, request, &prepared, twig, seat),
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
        field = Some(placed_field(tree, request, placed)?);
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
    super::tests::count(|t| {
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
pub(super) fn both<A: Send, B>(
    concurrent: bool,
    a: impl FnOnce() -> A + Send,
    b: impl FnOnce() -> B,
) -> (A, B) {
    #[cfg(not(target_arch = "wasm32"))]
    if concurrent {
        #[cfg(test)]
        super::tests::count(|t| t.split = true);
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

/// Stage 3's rings and their time: with their coords where the wood is
/// drawn, and their contacts where leaves are seated on them.
pub(super) fn rings(
    tree: &Tree,
    input: &SurfaceInput,
    request: Request,
    seats: bool,
) -> Result<(Rings, f64)> {
    let start = (request.clock)();
    let (height, params) = (input.height, &input.params);
    let sweep = Sweep {
        drawn: request.wood,
        edges: seats,
    };
    let rings = surface::rings(tree, height, params, sweep)?;
    Ok((rings, (request.clock)() - start))
}

/// The wood's mesh step around its rings, and its time.
pub(super) fn wood(
    tree: &Tree,
    input: &SurfaceInput,
    request: Request,
    rings: &Rings,
) -> Result<(Faces, f64)> {
    let start = (request.clock)();
    let (height, params) = (input.height, &input.params);
    let faces = surface::faces(rings, tree, height, params)?;
    Ok((faces, (request.clock)() - start))
}

/// Where the leaves sit: free of the wood, on rings the wood shares, or on
/// rings they sweep themselves where no wood is drawn.
#[derive(Clone, Copy)]
pub(super) enum Seat<'r> {
    Free,
    Shared(&'r Rings),
    Own,
}

/// The leaves with the time of their own rings, placement and the cull.
pub(super) type TimedLeaves = Option<(Leaves, [f64; 3])>;

/// The leaves and the field read from the plan, side by side where the
/// schedule allows; neither reads the other.
pub(super) fn leaves_and_field(
    tree: &Tree,
    inputs: &Inputs,
    request: Request,
    prepared: &Prepared,
    twig: Option<TwigPlacement>,
    seat: Seat,
) -> (Result<TimedLeaves>, Result<Option<(Field, f64)>>) {
    let both_run = prepared.reference.is_some() && prepared.leaf_plan.is_some();
    both(
        request.schedule == Schedule::Concurrent && both_run,
        || leaves(tree, inputs, request, prepared, twig, seat),
        || stage::planned_field(tree, request, prepared),
    )
}

/// Placement and the cull, timed apart, and the retained leaves' bounds
/// where leaves were asked for. Seated leaves read their rings in place and
/// free what they own of them once placed.
fn leaves(
    tree: &Tree,
    inputs: &Inputs,
    request: Request,
    prepared: &Prepared,
    twig: Option<TwigPlacement>,
    seat: Seat,
) -> Result<TimedLeaves> {
    let (Some(reference), Some(element)) = (prepared.reference, prepared.element.as_ref()) else {
        return Ok(None);
    };
    let clock = request.clock;
    let input: &LeafInput = &inputs.leaves;
    let envelope = input.envelope;
    let start = clock();
    let own = match seat {
        Seat::Own => Some(rings(tree, &inputs.surface, request, true)?.0),
        _ => None,
    };
    let rung = clock();
    let rings = match seat {
        Seat::Shared(rings) => Some(rings),
        _ => own.as_ref(),
    };
    let contacts = rings.map(AttachmentSurface::on_wood).transpose()?;
    let placed = foliage::place_on(
        tree,
        envelope,
        input.seed,
        input.canopy,
        twig,
        contacts.as_ref(),
        reference,
    )?;
    drop(contacts);
    drop(own);
    let placed_at = clock();
    let placed_count = placed.len();
    let instances = foliage::cull(placed, element, envelope, input.shell_depth)?;
    let bounds = if request.leaves {
        instances.bounds(element)?
    } else {
        None
    };
    let leaves = Leaves {
        retained: instances.len(),
        instances,
        placed: placed_count,
        bounds,
    };
    let ms = [rung - start, placed_at - rung, clock() - placed_at];
    Ok(Some((leaves, ms)))
}

/// The field read from placed leaves, where the plan cannot describe them.
pub(super) fn placed_field(
    tree: &Tree,
    request: Request,
    placed: Option<(&Instances, &Element)>,
) -> Result<(Field, f64)> {
    let start = (request.clock)();
    Ok((Field::new(tree, placed)?, (request.clock)() - start))
}
