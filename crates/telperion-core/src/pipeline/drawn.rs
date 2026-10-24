//! Stages 3 and 4 with the geometry compiled in: the rings, the wood, the
//! leaves placed and culled, and the field read out from the drawn tree.
use super::{
    input::{LeafInput, SurfaceInput},
    stage,
    stage::Prepared,
    Inputs, Leaves, Outputs, Request, Schedule, Stages,
};
use crate::{
    pipeline::field::Grid,
    pipeline::foliage::{self, Instances, TwigPlacement},
    pipeline::surface::{self, AttachmentSurface, Curve, Faces, Rings, Sweep},
    tree::Tree,
    Result,
};
use std::sync::OnceLock;

/// Stages 3 and 4 with the wood and the leaves compiled in.
pub(super) fn outputs(tree: &Tree, inputs: &Inputs, request: Request) -> Result<Outputs> {
    let (surface, leaf) = (&inputs.surface, &inputs.leaves);
    let twig = inputs.plan.twig.as_ref().ok().copied();
    // Leaves are placed for their own sake alone; placed with surface
    // contact, they sit on the rings.
    let places = request.leaves;
    let seats = places && leaf.canopy.surface_contact > 0.0;
    let rings = OnceLock::new();
    let sweep = || self::rings(tree, surface, request, seats);
    let shared = || rings.get_or_init(sweep).as_ref().map_err(Clone::clone);
    let (wood, planned) = both(
        request.schedule == Schedule::Concurrent && request.wood && places,
        || {
            let wood = |r: &(_, _, _)| self::wood(tree, surface, request, &r.0);
            request.wood.then(|| shared().and_then(wood)).transpose()
        },
        || {
            let prepared = stage::prepare(tree, &inputs.plan, request, places)?;
            let seat = match (seats, request.wood) {
                (false, _) => Ok(Seat::Free),
                (true, false) => Ok(Seat::Own),
                (true, true) => shared().map(|r| Seat::Shared(&r.0)),
            };
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
    let (mut leaves, field) = (leaves?, field?);
    let rings = rings.into_inner().transpose()?;
    let mut stages = Stages {
        plan_ms: prepared.ms,
        ..Stages::default()
    };
    if let Some((_, _, ms)) = rings {
        stages.rings_ms = ms;
    }
    if let Some((_, ms)) = wood {
        stages.wood_ms = ms;
    }
    if let Some((_, [own, placement, cull])) = leaves {
        stages.rings_ms += own;
        [stages.placement_ms, stages.cull_ms] = [placement, cull];
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
    let (rings, drawn_from) = match rings {
        Some((rings, curve, _)) => (Some(rings), curve),
        None => (None, None),
    };
    let traces = request.curve && drawn_from.is_none();
    #[cfg(test)]
    super::tests::count(|t| {
        let own = seats && !request.wood && leaves.is_some();
        t.sweeps += u32::from(rings.is_some()) + u32::from(own) + u32::from(traces);
        t.elements += u32::from(prepared.element.is_some());
    });
    let wood = wood
        .zip(rings)
        .map(|((faces, _), rings)| rings.into_mesh(faces));
    let organ = organ(tree, inputs, request, prepared.reference)?;
    let curve = match drawn_from {
        Some(curve) => Some(curve),
        None => traces
            .then(|| surface::curve(tree, surface.height, &surface.params))
            .transpose()?,
    };
    Ok(Outputs {
        element: prepared.element,
        plan: prepared.leaf_plan,
        wood,
        curve,
        leaves: leaves.map(|(l, _)| l),
        organ,
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
/// drawn, and their contacts where leaves are seated on them. A wood drawn
/// beside its curve is traced once and drawn from the curve, which comes
/// back with the rings.
pub(super) fn rings(
    tree: &Tree,
    input: &SurfaceInput,
    request: Request,
    seats: bool,
) -> Result<(Rings, Option<Curve>, f64)> {
    let start = (request.clock)();
    let (height, params) = (input.height, &input.params);
    let (rings, curve) = if request.wood && request.curve {
        let traced = surface::traced(tree, height, params)?;
        let rings = surface::drawn_from(&traced, tree, height, params, seats)?;
        (rings, Some(traced.curve))
    } else {
        let sweep = Sweep {
            drawn: request.wood,
            edges: seats,
            distances: false,
        };
        (surface::rings(tree, height, params, sweep)?, None)
    };
    Ok((rings, curve, (request.clock)() - start))
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

/// The leaves and the field, side by side where the schedule allows;
/// neither reads the other.
pub(super) fn leaves_and_field(
    tree: &Tree,
    inputs: &Inputs,
    request: Request,
    prepared: &Prepared,
    twig: Option<TwigPlacement>,
    seat: Seat,
) -> (Result<TimedLeaves>, Result<Option<(Grid, f64)>>) {
    let both_run = prepared.reference.is_some() && prepared.leaf_plan.is_some();
    both(
        request.schedule == Schedule::Concurrent && both_run,
        || leaves(tree, inputs, request, prepared, twig, seat),
        || stage::field(tree, request, prepared),
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
        element,
    )?;
    drop(contacts);
    drop(own);
    let placed_at = clock();
    let placed_count = placed.len();
    // The row is still checked where it was, so a request meets the same
    // error; nothing past the check reads it (fn-212 R2).
    foliage::range(input.shell_depth, 0., 1., "shell depth")?;
    foliage::admit(&placed, element)?;
    let instances = placed;
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

/// The flowers (fn-216): where leaves are asked for and the tree's ages
/// flower, the organ element and its florets, packed against the leaves'
/// box as the device packs its own (`foliage::bloom_box`), or their own
/// where the tree has no leaf box; none where nothing flowers.
fn organ(
    tree: &Tree,
    inputs: &Inputs,
    request: Request,
    leaf: Option<foliage::Reference>,
) -> Result<Option<crate::mesh::Foliage>> {
    if !request.leaves || tree.blooms.is_empty() {
        return Ok(None);
    }
    let element = foliage::bloom_element(&inputs.bloom)?;
    let p = &inputs.bloom;
    let instances = match leaf {
        Some(leaf) => {
            foliage::place_blooms_in(tree, p, request.season, foliage::bloom_box(leaf, p))?
        }
        None => foliage::place_blooms(tree, p, request.season)?,
    };
    Ok(Some(crate::mesh::Foliage { element, instances }))
}
