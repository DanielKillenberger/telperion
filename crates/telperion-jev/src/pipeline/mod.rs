//! Species template pipeline: literature to preset (fn-58).
//!
//! A fixed sequence of stages, each reading the admitted manifest and earlier
//! artifacts at fixed paths and writing one complete artifact atomically under
//! a versioned schema with a canonical serialization. Jev answers choices,
//! scores and nouls through the shared caller; code owns every number.

pub mod adapter;
pub mod canon;
pub mod consume;
pub mod cost;
pub mod curve;
pub mod decision;
pub mod gap;
pub mod judge;
pub mod known;
pub mod manifest;
pub mod render;
pub mod requirements;
pub mod routes;
pub mod sets;
pub mod stage;
pub mod stages;
pub mod swap;
