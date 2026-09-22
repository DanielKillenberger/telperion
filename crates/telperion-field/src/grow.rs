//! The slim chain: a served family at a seed, the direct build's skeleton,
//! the leaf plan and the field. No surface is built and no leaf is placed;
//! a family the plan cannot describe is refused rather than placed.
use telperion_core::{
    branching,
    field::Field,
    foliage::{build_element, plan, TwigPlacement},
    presets, Error, Result,
};

pub fn field(species: &str, seed: u32, limb_order: Option<u32>) -> Result<Field> {
    let mut family = presets::by_identity(species)?;
    family.skeleton.seed = seed;
    let tree = branching::generate(&family.skeleton, family.radii)?.tree;
    let element = build_element(family.element)?;
    let plan = plan::plan(
        &tree,
        family.skeleton.envelope,
        family.canopy,
        Some(TwigPlacement::of(&family)?),
        &family.surface,
        &element,
        limb_order,
    )?
    .ok_or(Error::InvalidInput("family without a leaf plan"))?;
    Field::planned(&tree, &plan)
}
