//! Devices asked for at the same instant. libtest runs a binary's tests as
//! threads of one process, so every device-backed binary does this by accident;
//! here it is done on purpose, and the process has to live through it.
use std::sync::{Arc, Barrier};
use std::thread;

use telperion_render::{Gpu, RenderError};

const THREADS: usize = 8;
const ROUNDS: usize = 8;

/// One request, reduced to what the round needs to know: a refusal about the
/// hardware is a skip reason, any other refusal fails the test.
fn request() -> Option<String> {
    match pollster::block_on(Gpu::request(None)) {
        Ok(_) => None,
        Err(error @ (RenderError::WebGpuUnavailable(_) | RenderError::FallbackOnly { .. })) => {
            Some(error.to_string())
        }
        Err(error) => panic!("the device was there and still refused: {error}"),
    }
}

/// Every thread leaves the barrier together and asks for a device; the skip
/// reasons, if any, come back.
fn round() -> Vec<String> {
    let barrier = Arc::new(Barrier::new(THREADS));
    let asking: Vec<_> = (0..THREADS)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                request()
            })
        })
        .collect();
    asking
        .into_iter()
        .filter_map(|thread| thread.join().expect("a requesting thread panicked"))
        .collect()
}

#[test]
fn devices_asked_for_at_once_all_arrive() {
    for _ in 0..ROUNDS {
        if let Some(reason) = round().first() {
            println!("skipped: {reason}");
            return;
        }
    }
}
