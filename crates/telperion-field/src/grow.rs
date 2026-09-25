//! The slim request: a served family at a seed, and the field alone from the
//! one build pipeline the main binding runs. The core is built without its
//! geometry, so the field is always the leaf plan's: no wood surface is built
//! and no leaf is placed, and a family the plan cannot describe is refused.
use telperion_core::{
    field::Field,
    pipeline::{self, Request},
    presets, Error, Result,
};

pub fn field(species: &str, seed: u32, limb_order: Option<u32>) -> Result<Field> {
    let mut family = presets::by_identity(species)?;
    family.skeleton.seed = seed;
    let request = Request {
        field: Some(limb_order),
        ..Request::default()
    };
    pipeline::build(&family, request)?
        .outputs
        .field
        .ok_or(Error::InvalidInput("no field built"))
}
