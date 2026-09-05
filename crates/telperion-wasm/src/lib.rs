//! One owned result slot per Wasm instance. Copy output before any mutating call.
use std::cell::RefCell;
use telperion_core::{envelope::Envelope, rng::Rng};
thread_local! {static OUTPUT:RefCell<Vec<f64>>=const {RefCell::new(Vec::new())};}

/// Foundation ABI: 0 success, 1 invalid input, 2 resource limit. Failure clears prior output.
#[no_mangle]
pub extern "C" fn sample_envelope(
    seed: u32,
    count: u32,
    height: f64,
    crown_base: f64,
    spread: f64,
    fullness: f64,
    shoulder: f64,
) -> u32 {
    OUTPUT.with(|out| {
        let mut out = out.borrow_mut();
        out.clear();
        let envelope = Envelope {
            height,
            crown_base,
            spread,
            fullness,
            shoulder,
        };
        match envelope.sample(count as usize, &mut Rng::new(seed)) {
            Ok(points) => {
                out.extend(points.into_iter().flat_map(|p| [p.x, p.y, p.z]));
                0
            }
            Err(telperion_core::Error::InvalidInput(_)) => 1,
            Err(telperion_core::Error::ResourceLimit(_)) => 2,
        }
    })
}
#[no_mangle]
pub extern "C" fn output_ptr() -> *const f64 {
    OUTPUT.with(|out| out.borrow().as_ptr())
}
#[no_mangle]
pub extern "C" fn output_len() -> usize {
    OUTPUT.with(|out| out.borrow().len())
}
#[no_mangle]
pub extern "C" fn release() {
    OUTPUT.with(|out| *out.borrow_mut() = Vec::new());
}
