//! The stages past the skeleton, each a call of the functions that did this
//! work before the pipeline named it, and each run only where asked.
use super::{Leaves, Request, Stages, Structure};
use crate::{
    field::Field,
    foliage::{self, plan, Element, Reference, TwigPlacement},
    presets::Family,
    surface::{self, AttachmentSurface, SurfaceMesh},
    tree::{NodeKind, Tree},
    Error, Result,
};
use std::sync::{Arc, Mutex};

/// The Plan artifacts the leaves and the field read.
pub(super) struct Prep {
    pub(super) element: Option<Element>,
    pub(super) leaf_plan: Option<plan::Plan>,
    /// The quantisation box, where leaves are placed.
    pub(super) reference: Option<Reference>,
}

pub(super) fn twig(family: &Family) -> Result<TwigPlacement> {
    let twig = family.skeleton.twigs.resolved()?.twig;
    Ok(TwigPlacement {
        internode_length: twig.internode_length,
        stations_per_internode: twig.stations_per_internode,
    })
}

/// Where the rings wait for the leaves once a stage before them swept.
pub(super) type RingSlot = Mutex<Option<Arc<AttachmentSurface>>>;

/// The one ring sweep: the rings leaf contacts query and a wood surface
/// built beside them takes as its own vertices.
pub(super) fn rings(
    tree: &Tree,
    family: &Family,
    request: Request,
    stages: &mut Stages,
) -> Result<Arc<AttachmentSurface>> {
    let start = (request.clock)();
    let height = family.skeleton.envelope.height;
    let rings = AttachmentSurface::new(tree, height, &family.surface)?;
    stages.sweeps += 1;
    stages.rings_ms = (request.clock)() - start;
    Ok(Arc::new(rings))
}

/// The wood surface, its time, and whether it swept: it reads the rings
/// where a sweep ran before it, else sweeps them itself.
pub(super) fn wood(
    tree: &Tree,
    family: &Family,
    request: Request,
    given: Option<Arc<AttachmentSurface>>,
) -> Result<Option<(SurfaceMesh, f64, bool)>> {
    if !request.wood {
        return Ok(None);
    }
    let start = (request.clock)();
    let height = family.skeleton.envelope.height;
    let (wood, swept) = match given {
        Some(rings) => (
            surface::build_swept(tree, height, &family.surface, &rings)?,
            false,
        ),
        None => (surface::build(tree, height, &family.surface)?, true),
    };
    Ok(Some((wood, (request.clock)() - start, swept)))
}

pub(super) fn lock(slot: &RingSlot) -> std::sync::MutexGuard<'_, Option<Arc<AttachmentSurface>>> {
    slot.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}

/// The element, the leaf plan and the box, each only where read.
pub(super) fn prepare(
    tree: &Tree,
    family: &Family,
    request: Request,
    twig: TwigPlacement,
    places: bool,
    stages: &mut Stages,
) -> Result<Prep> {
    let element = if request.leaves || request.field.is_some() {
        stages.elements += 1;
        Some(foliage::build_element(family.element)?)
    } else {
        None
    };
    let mut leaf_plan = None;
    if let (Some(limb_order), Some(element)) = (request.field, element.as_ref()) {
        let start = (request.clock)();
        leaf_plan = plan::plan(
            tree,
            family.skeleton.envelope,
            family.canopy,
            Some(twig),
            &family.surface,
            element,
            limb_order,
        )?;
        stages.plan_ms = (request.clock)() - start;
    }
    let reference = if places {
        Some(Reference::of(family)?)
    } else {
        None
    };
    Ok(Prep {
        element,
        leaf_plan,
        reference,
    })
}

/// Placed leaves, the rings, placement and cull times, and whether this
/// stage swept the rings itself.
pub(super) struct Placed {
    pub(super) leaves: Leaves,
    pub(super) ms: [f64; 3],
    pub(super) swept: bool,
}

/// Placement and the cull, timed apart, and the retained leaves' bounds
/// where leaves were asked for. Leaves seated on the wood read the rings a
/// sweep before them left in `slot`, else the vertices of a `wood` already
/// swept, else sweep the rings here; they free them once placed.
pub(super) fn leaves(
    tree: &Tree,
    family: &Family,
    request: Request,
    twig: TwigPlacement,
    (slot, wood): (&RingSlot, Option<&SurfaceMesh>),
    prep: &Prep,
) -> Result<Option<Placed>> {
    let (Some(reference), Some(element)) = (prep.reference, prep.element.as_ref()) else {
        return Ok(None);
    };
    let clock = request.clock;
    let envelope = family.skeleton.envelope;
    let start = clock();
    let mut rings = lock(slot).take();
    let mut swept = false;
    if rings.is_none() && family.canopy.surface_contact > 0.0 {
        let (height, params) = (envelope.height, &family.surface);
        rings = Some(Arc::new(match wood {
            Some(wood) => AttachmentSurface::of_wood(tree, height, params, wood)?,
            None => {
                swept = true;
                AttachmentSurface::new(tree, height, params)?
            }
        }));
    }
    let swept_at = clock();
    let placed = foliage::place_on(
        tree,
        envelope,
        family.skeleton.seed,
        family.canopy,
        Some(twig),
        rings.as_deref(),
        reference,
    )?;
    drop(rings);
    let placed_at = clock();
    let placed_count = placed.len();
    let instances = foliage::cull(placed, element, envelope, family.shell_depth)?;
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
    let ms = [swept_at - start, placed_at - swept_at, clock() - placed_at];
    Ok(Some(Placed { leaves, ms, swept }))
}

/// The field read from the leaf plan, where the plan describes the family.
pub(super) fn planned_field(
    tree: &Tree,
    request: Request,
    prep: &Prep,
) -> Result<Option<(Field, f64)>> {
    match (request.field, prep.leaf_plan.as_ref()) {
        (Some(_), Some(p)) => {
            let start = (request.clock)();
            Ok(Some((Field::planned(tree, p)?, (request.clock)() - start)))
        }
        _ => Ok(None),
    }
}

/// The structure export: every node's record, in node order.
pub(super) fn structure(tree: &Tree) -> Result<Structure> {
    let mut out = Structure::default();
    out.nodes
        .try_reserve(tree.nodes.len() * 6)
        .map_err(|_| Error::ResourceLimit("structure transfer"))?;
    out.topology
        .try_reserve(tree.nodes.len() * 3)
        .map_err(|_| Error::ResourceLimit("topology transfer"))?;
    for n in &tree.nodes {
        out.nodes.extend([
            n.position.x,
            n.position.y,
            n.position.z,
            n.radius,
            n.start_radius,
            n.base_radius,
        ]);
        out.topology.extend([
            n.parent.unwrap_or(u32::MAX),
            n.branch,
            match n.kind {
                NodeKind::Structural => 0,
                NodeKind::Branch => 1,
                NodeKind::Twig => 2,
            },
        ]);
    }
    Ok(out)
}
