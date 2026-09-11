//! Renderer-independent tree generation. Coordinates and lengths are metres, Y is up.
pub mod bias;
pub mod blend;
pub mod branching;
pub mod colonization;
pub mod envelope;
pub mod field;
pub mod foliage;
pub mod material;
pub mod math;
pub mod mesh;
pub mod noise;
#[cfg(feature = "json")]
pub mod params;
pub mod presets;
pub mod radius;
pub mod rng;
pub mod surface;
pub mod tree;
pub mod twigs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    InvalidInput(&'static str),
    ResourceLimit(&'static str),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidInput(message) => write!(f, "invalid input: {message}"),
            Self::ResourceLimit(message) => write!(f, "resource limit: {message}"),
        }
    }
}
impl std::error::Error for Error {}
pub type Result<T> = std::result::Result<T, Error>;
