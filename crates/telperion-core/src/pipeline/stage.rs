//! The stages past the skeleton, each run only where the request asks.
use super::{both, Leaves, Request, Schedule, Structure};
use crate::{
    field::Field,
    foliage::{self, plan, Element, Instances, Reference, TwigPlacement},
    presets::Family,
    surface::{self, AttachmentSurface, WoodWithContacts},
    tree::{NodeKind, Tree},
    Error, Result,
};

/// Stage 3's artifacts: what the leaves and the field read.
pub(super) struct Prepared {
    pub(super) element: Option<Element>,
    pub(super) leaf_plan: Option<plan::Plan>,
    /// The quantisation box, present exactly where leaves are placed.
    reference: Option<Reference>,
    /// The leaf plan's time.
    pub(super) ms: f64,
}

pub(super) fn twig(family: &Family) -> Result<TwigPlacement> {
    let twig = family.skeleton.twigs.resolved()?.twig;
    Ok(TwigPlacement {
        internode_length: twig.internode_length,
        stations_per_internode: twig.stations_per_internode,
    })
}

/// The element and the leaf plan, each only where read, and the box where
/// leaves are placed. The twig's error answers after the element's, as it
/// always did.
pub(super) fn prepare(
    tree: &Tree,
    family: &Family,
    request: Request,
    twig: &Result<TwigPlacement>,
    places: bool,
) -> Result<Prepared> {
    let element = if request.leaves || request.field.is_some() {
        Some(foliage::build_element(family.element)?)
    } else {
        None
    };
    let twig = twig.clone()?;
    let (mut leaf_plan, mut ms) = (None, 0.0);
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
        ms = (request.clock)() - start;
    }
    let reference = places.then(|| Reference::of(family)).transpose()?;
    Ok(Prepared {
        element,
        leaf_plan,
        reference,
        ms,
    })
}

/// The wood surface and its time, with its contacts where leaves are seated
/// on it.
pub(super) fn wood(
    tree: &Tree,
    family: &Family,
    request: Request,
    seated: bool,
) -> Result<Option<(WoodWithContacts, f64)>> {
    if !request.wood {
        return Ok(None);
    }
    let start = (request.clock)();
    let (height, params) = (family.skeleton.envelope.height, &family.surface);
    let wood = if seated {
        surface::build_contacts(tree, height, params)?
    } else {
        WoodWithContacts {
            mesh: surface::build(tree, height, params)?,
            edges: Vec::new(),
        }
    };
    Ok(Some((wood, (request.clock)() - start)))
}

/// The leaves with their rings, placement and cull times.
pub(super) type TimedLeaves = Option<(Leaves, [f64; 3])>;

/// The leaves and the field read from the plan, side by side where the
/// schedule allows; neither reads the other.
pub(super) fn leaves_and_field(
    tree: &Tree,
    family: &Family,
    request: Request,
    prepared: &Prepared,
    twig: Option<TwigPlacement>,
    seat: Option<&WoodWithContacts>,
) -> (Result<TimedLeaves>, Result<Option<(Field, f64)>>) {
    let both_run = prepared.reference.is_some() && prepared.leaf_plan.is_some();
    both(
        request.schedule == Schedule::Concurrent && both_run,
        || leaves(tree, family, request, prepared, twig, seat),
        || planned_field(tree, request, prepared),
    )
}

/// Placement and the cull, timed apart, and the retained leaves' bounds
/// where leaves were asked for. Leaves seated on a wood read its rings in
/// place, else sweep their own; either way they free them once placed.
fn leaves(
    tree: &Tree,
    family: &Family,
    request: Request,
    prepared: &Prepared,
    twig: Option<TwigPlacement>,
    seat: Option<&WoodWithContacts>,
) -> Result<TimedLeaves> {
    let (Some(reference), Some(element)) = (prepared.reference, prepared.element.as_ref()) else {
        return Ok(None);
    };
    let clock = request.clock;
    let envelope = family.skeleton.envelope;
    let start = clock();
    let contacts = match seat {
        Some(wood) => Some(AttachmentSurface::on_wood(wood, &family.surface)?),
        None if family.canopy.surface_contact > 0.0 => Some(AttachmentSurface::new(
            tree,
            envelope.height,
            &family.surface,
        )?),
        None => None,
    };
    let rung = clock();
    let placed = foliage::place_on(
        tree,
        envelope,
        family.skeleton.seed,
        family.canopy,
        twig,
        contacts.as_ref(),
        reference,
    )?;
    drop(contacts);
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
    let ms = [rung - start, placed_at - rung, clock() - placed_at];
    Ok(Some((leaves, ms)))
}

/// The field read from the leaf plan, where the plan describes the family.
fn planned_field(
    tree: &Tree,
    request: Request,
    prepared: &Prepared,
) -> Result<Option<(Field, f64)>> {
    match (request.field, prepared.leaf_plan.as_ref()) {
        (Some(_), Some(p)) => {
            let start = (request.clock)();
            Ok(Some((Field::planned(tree, p)?, (request.clock)() - start)))
        }
        _ => Ok(None),
    }
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
