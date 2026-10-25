//! The engine's own code size in wasm32: every species grown once.
use telperion_space::{grow, Light, Request};
#[no_mangle]
pub extern "C" fn run(which: u32, age: u32) -> u32 {
    let s = match which { 0 => telperion_space::palm(), 1 => telperion_space::beech(), 2 => telperion_space::oak(), _ => telperion_space::spruce() };
    let r = Request { age, seed: 1, budget: 20_000_000, light: Light { extinction: 0.5, sky: 0.5 } };
    grow(&s, r).map_or(0, |t| t.phytomer_count() as u32)
}
