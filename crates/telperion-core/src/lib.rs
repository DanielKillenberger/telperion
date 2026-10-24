//! Renderer-independent tree generation. Coordinates and lengths are metres, Y is up.
// Without the geometry feature, helpers only leaf placement and the wood
// sweep call still compile, unused.
#![cfg_attr(not(feature = "geometry"), allow(dead_code))]
pub mod catalogue;
mod family;
pub use family::Family;

pub mod blend;
pub mod capability;
pub mod envelope;
pub mod material;
pub mod math;
pub mod memory;
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
pub mod supernatural;
pub mod tree;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(&'static str),
    InvalidValue {
        field: &'static str,
        value: String,
    },
    ResourceLimit(&'static str),
    /// A wood the memory left cannot hold, refused before any of it is
    /// allocated: the bytes it needs and the bytes left (fn-291 R2f).
    WoodMemory {
        needed: u64,
        available: u64,
    },
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidValue { field, value } => write!(f, "invalid input: {field} = {value}"),
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::ResourceLimit(message) => write!(f, "resource limit: {message}"),
            Self::WoodMemory { needed, available } => write!(
                f,
                "the wood needs {needed} bytes and {available} are left in 32-bit Wasm \
                 memory; build it through the device entry or with fewer radialSegments"
            ),
        }
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod one_tree;
#[cfg(all(test, feature = "geometry", feature = "json"))]
mod suite;
