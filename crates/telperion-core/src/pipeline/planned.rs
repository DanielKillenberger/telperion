//! Stages 3 and 4 with the geometry compiled out: the leaf plan, the field
//! read from it and the structure. Nothing is placed, so wood, leaves and a
//! family the plan cannot describe are refused by name.
use super::{stage, Outputs, Request, Stages};
use crate::{presets::Family, tree::Tree, Error, Result};

pub(super) fn outputs(tree: &Tree, family: &Family, request: Request) -> Result<Outputs> {
    if request.wood || request.leaves {
        return Err(Error::InvalidInput(
            "wood and leaves need the geometry feature",
        ));
    }
    let twig = stage::twig(family);
    let prepared = stage::prepare(tree, family, request, &twig, false)?;
    let field = stage::planned_field(tree, request, &prepared)?;
    if request.field.is_some() && field.is_none() {
        return Err(Error::InvalidInput(
            "family without a leaf plan; its field needs the geometry feature",
        ));
    }
    let structure = request
        .structure
        .then(|| stage::structure(tree))
        .transpose()?;
    let stages = Stages {
        plan_ms: prepared.ms,
        field_ms: field.as_ref().map_or(0.0, |(_, ms)| *ms),
        ..Stages::default()
    };
    Ok(Outputs {
        element: prepared.element,
        plan: prepared.leaf_plan,
        wood: None,
        leaves: None,
        field: field.map(|(f, _)| f),
        structure,
        stages,
    })
}
