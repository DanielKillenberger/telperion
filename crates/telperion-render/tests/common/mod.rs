//! What every device-backed test file needs: a device, or a printed reason and
//! a test that does not run. A machine without a GPU is not a failing build.
use telperion_render::{Gpu, RenderError};

/// A device, or `None` with the reason printed. Every device-backed assertion
/// hangs off this; a refusal that is not about the hardware still fails.
pub fn gpu() -> Option<Gpu> {
    match pollster::block_on(Gpu::request(None)) {
        Ok(gpu) => Some(gpu),
        Err(error @ (RenderError::WebGpuUnavailable(_) | RenderError::FallbackOnly { .. })) => {
            println!("skipped: {error}");
            None
        }
        Err(error) => panic!("the device was there and still refused: {error}"),
    }
}
