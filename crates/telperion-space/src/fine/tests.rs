//! The coarse tree (`coarse.rs`) and the decision contract's draws
//! (`draws.rs`) and ticks (`ticks.rs`).
use crate::grow::Request;
use crate::light::Light;

mod coarse;
mod draws;
mod ticks;

pub(super) fn request(age: u32, seed: u64) -> Request {
    Request {
        age,
        seed,
        budget: 20_000_000,
        light: Light::NEUTRAL,
    }
}
