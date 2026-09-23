//! The stages past the skeleton, each a call of the functions that did this
//! work before the pipeline named it, and each run only where asked.
use super::{both, Leaves, Request, Schedule, Stages, Structure};
use crate::{
    field::Field,
    foliage::{self, plan, Element, Instances, Reference, TwigPlacement},
    presets::Family,
    surface::{self, AttachmentSurface, SurfaceMesh},
    tree::{NodeKind, Tree},
    Error, Result,
};

/// Stage 3's artifacts: what the leaves and the field read.
pub(super) struct Plan {
    twig: TwigPlacement,
    pub(super) element: Option<Element>,
    pub(super) leaf_plan: Option<plan::Plan>,
    /// The quantisation box, present exactly where leaves are placed.
    reference: Option<Reference>,
}
impl Plan {
    pub(super) fn places(&self) -> bool {
        self.reference.is_some()
    }
}

/// The element and the leaf plan, each only where read, and the box where
/// leaves are placed: for leaves, or for a field the plan cannot describe.
pub(super) fn plan(
    tree: &Tree,
    family: &Family,
    request: Request,
    stages: &mut Stages,
) -> Result<Plan> {
    let twig = family.skeleton.twigs.resolved()?.twig;
    let twig = TwigPlacement {
        internode_length: twig.internode_length,
        stations_per_internode: twig.stations_per_internode,
    };
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
    let places = request.leaves || (request.field.is_some() && leaf_plan.is_none());
    let reference = places.then(|| Reference::of(family)).transpose()?;
    Ok(Plan {
        twig,
        element,
        leaf_plan,
        reference,
    })
}

/// The wood surface and its time; where leaves are seated on it, the node
/// edges that read its vertices as their contact rings.
pub(super) struct Wood {
    pub(super) mesh: SurfaceMesh,
    edges: Vec<Option<[usize; 4]>>,
    pub(super) ms: f64,
}
impl Wood {
    /// The wood's vertices and the edges into them, for the leaves to seat on.
    pub(super) fn seat(&mut self) -> Seat<'_> {
        (&self.mesh, std::mem::take(&mut self.edges))
    }
}
pub(super) type Seat<'w> = (&'w SurfaceMesh, Vec<Option<[usize; 4]>>);

pub(super) fn wood(
    tree: &Tree,
    family: &Family,
    request: Request,
    seated: bool,
) -> Result<Option<Wood>> {
    if !request.wood {
        return Ok(None);
    }
    let start = (request.clock)();
    let (height, params) = (family.skeleton.envelope.height, &family.surface);
    let (mesh, edges) = if seated {
        surface::build_contacts(tree, height, params)?
    } else {
        (surface::build(tree, height, params)?, Vec::new())
    };
    let ms = (request.clock)() - start;
    Ok(Some(Wood { mesh, edges, ms }))
}

/// Placed leaves, then the rings, placement and cull times, and whether the
/// leaves swept rings of their own.
pub(super) struct Placed {
    pub(super) leaves: Leaves,
    pub(super) ms: [f64; 3],
    pub(super) swept: bool,
}

/// The leaves and the field read from the plan, side by side where the
/// schedule allows; neither reads the other.
pub(super) fn leafy(
    tree: &Tree,
    family: &Family,
    request: Request,
    plan: &Plan,
    seat: Option<Seat<'_>>,
) -> (Result<Option<Placed>>, Result<Option<(Field, f64)>>) {
    let both_run = plan.places() && plan.leaf_plan.is_some();
    let concurrent = request.schedule == Schedule::Concurrent && both_run;
    let leaves = || leaves(tree, family, request, plan, seat);
    let field = || planned_field(tree, request, plan);
    let (leaves, field, _) = both(concurrent, leaves, field);
    (leaves, field)
}

/// Placement and the cull, timed apart, and the retained leaves' bounds
/// where leaves were asked for. Leaves seated on the wood read its vertices
/// where it was built, else sweep the rings here; they free them once placed.
fn leaves(
    tree: &Tree,
    family: &Family,
    request: Request,
    plan: &Plan,
    seat: Option<Seat<'_>>,
) -> Result<Option<Placed>> {
    let (Some(reference), Some(element)) = (plan.reference, plan.element.as_ref()) else {
        return Ok(None);
    };
    let clock = request.clock;
    let envelope = family.skeleton.envelope;
    let start = clock();
    let swept = seat.is_none() && family.canopy.surface_contact > 0.0;
    let contacts = match seat {
        Some((mesh, edges)) => Some(AttachmentSurface::on_wood(mesh, edges, &family.surface)?),
        None if swept => Some(AttachmentSurface::new(
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
        Some(plan.twig),
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
    Ok(Some(Placed { leaves, ms, swept }))
}

/// The field read from the leaf plan, where the plan describes the family.
fn planned_field(tree: &Tree, request: Request, plan: &Plan) -> Result<Option<(Field, f64)>> {
    match (request.field, plan.leaf_plan.as_ref()) {
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
