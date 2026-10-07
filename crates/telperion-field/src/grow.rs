//! The slim request: a served family at a seed, and the field alone from the
//! one build pipeline the main binding runs. The core is built without its
//! geometry, so the field is always the leaf plan's: no wood surface is built
//! and no leaf is placed, and a family the plan cannot describe is refused.
use telperion_core::{
    field::Field,
    pipeline::{self, Request},
    presets, Error, Result,
};

/// The field of `species` at `seed`, its limbs named at `limb_order`, every
/// fine root at or below `cut` expected kept phytomers standing as its
/// expected occupancy (fn-214; 0 grows every one).
pub fn field(species: &str, seed: u32, limb_order: Option<u32>, cut: u32) -> Result<Field> {
    let mut family = presets::by_identity(species)?;
    family.skeleton.seed = seed;
    let request = Request {
        field: Some(limb_order),
        cut,
        ..Request::default()
    };
    pipeline::build(&family, request)?
        .outputs
        .field
        .ok_or(Error::InvalidInput("no field built"))
}
