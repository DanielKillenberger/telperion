//! Renderer-independent tree generation. Coordinates and lengths are metres, Y is up.
// Without the geometry feature the specimen keeps the growth path's aged-read
// helpers, which only the placement-backed reads call.
#![cfg_attr(not(feature = "geometry"), allow(dead_code))]
pub mod catalogue;
mod family;
pub use family::Family;

pub mod blend;
pub mod capability;
pub mod envelope;
pub mod growth;
pub mod material;
pub mod math;
#[cfg(feature = "geometry")]
pub mod mesh;
pub mod noise;
#[cfg(feature = "json")]
pub mod params;
pub mod pipeline;
// The stages' data contracts, at the paths consumers name them by.
#[cfg(all(test, feature = "geometry"))]
pub use pipeline::contract::footprint;
pub use pipeline::contract::{
    bias, branching, colonization, field, foliage, radius, surface, twigs,
};
// The crate's own tests name it as its consumers do.
#[cfg(test)]
extern crate self as telperion_core;
pub mod presets;
pub mod ranges;
pub mod rng;
pub mod tree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(&'static str),
    InvalidValue { field: &'static str, value: String },
    ResourceLimit(&'static str),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue { field, value } => write!(f, "invalid input: {field} = {value}"),
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::ResourceLimit(message) => write!(f, "resource limit: {message}"),
        }
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

pub mod specimen;
#[cfg(all(test, feature = "geometry", feature = "json"))]
mod suite;
