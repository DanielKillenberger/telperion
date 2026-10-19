//! The stages past the skeleton, each run only where the request asks.
use super::{PlanInput, Request, Structure};
#[cfg(feature = "geometry")]
use crate::pipeline::field::{self, Grid};
use crate::{
    pipeline::foliage::{self, plan, Element, Reference},
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

/// The element and the leaf plan, each only where read, and the box where
/// leaves are placed. The twig's error answers after the element's, as it
/// always did.
pub(super) fn prepare(
    tree: &Tree,
    input: &PlanInput,
    request: Request,
    places: bool,
) -> Result<Prepared> {
    let element = if request.leaves || request.field.is_some() {
        Some(foliage::build_element(input.element)?)
    } else {
        None
    };
    let twig = input.twig.clone()?;
    let (mut leaf_plan, mut ms) = (None, 0.0);
    if let (Some(f), Some(element)) = (request.field, element.as_ref()) {
        let start = (request.clock)();
        leaf_plan = plan::plan(
            tree,
            input.envelope,
            input.canopy,
            Some(twig),
            &input.surface,
            element,
            f.limb_order,
            input.seed,
        )?;
        ms = (request.clock)() - start;
    }
    // Leaves are placed only where the geometry is compiled in.
    #[cfg(feature = "geometry")]
    let reference = places
        .then(|| {
            input
                .reach
                .clone()
                .map(|reach| Reference::grown(tree, reach))
        })
        .transpose()?;
    #[cfg(not(feature = "geometry"))]
    let reference = places.then_some(Reference::default());
    Ok(Prepared {
        element,
        leaf_plan,
        reference,
        ms,
    })
}

/// The field read out from the drawn tree (fn-257 R8): its wood and its
/// leaf plan's records, to the request's grid. A family the plan cannot
/// describe is refused by name; no leaf is placed for a field.
#[cfg(feature = "geometry")]
pub(super) fn field(
    tree: &Tree,
    request: Request,
    prepared: &Prepared,
) -> Result<Option<(Grid, f64)>> {
    let Some(f) = request.field else {
        return Ok(None);
    };
    let plan = prepared.leaf_plan.as_ref().ok_or(Error::InvalidInput(
        "a field reads the leaf plan, which does not describe this family",
    ))?;
    let start = (request.clock)();
    let grid = field::field(tree, plan, f.n)?;
    Ok(Some((grid, (request.clock)() - start)))
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
