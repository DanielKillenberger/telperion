//! Stages 3 and 4 with the geometry compiled out: the structure alone.
//! Nothing is drawn, so wood, leaves and the field, which reads the drawn
//! tree, are refused by name.
use super::{stage, Inputs, Outputs, Request, Stages};
use crate::{tree::Tree, Error, Result};

pub(super) fn outputs(tree: &Tree, inputs: &Inputs, request: Request) -> Result<Outputs> {
    if request.wood || request.leaves || request.field.is_some() {
        return Err(Error::InvalidInput(
            "wood, leaves and the field need the geometry feature",
        ));
    }
    let prepared = stage::prepare(tree, &inputs.plan, request, false)?;
    let structure = request
        .structure
        .then(|| stage::structure(tree))
        .transpose()?;
    Ok(Outputs {
        element: prepared.element,
        plan: prepared.leaf_plan,
        wood: None,
        leaves: None,
        structure,
        stages: Stages::default(),
    })
}
