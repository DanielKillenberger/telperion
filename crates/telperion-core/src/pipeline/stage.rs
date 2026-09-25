//! The stages past the skeleton, each run only where the request asks.
use super::{Request, Structure};
use crate::{
    field::Field,
    foliage::{self, plan, Element, Reference, TwigPlacement},
    presets::Family,
    tree::{NodeKind, Tree},
    Error, Result,
};

/// Stage 3's artifacts: what the leaves and the field read.
pub(super) struct Prepared {
    pub(super) element: Option<Element>,
    pub(super) leaf_plan: Option<plan::Plan>,
    /// The quantisation box, present exactly where leaves are placed.
    pub(super) reference: Option<Reference>,
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

/// The field read from the leaf plan, where the plan describes the family.
pub(super) fn planned_field(
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
