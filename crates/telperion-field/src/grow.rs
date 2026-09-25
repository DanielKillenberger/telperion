//! The slim request: a served family at a seed, and the field alone from the
//! one build pipeline the main binding runs. No wood surface is built; leaves
//! are placed only for a family the leaf plan cannot describe, whose field
//! reads them.
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
