//! The engine's one clock, for the timings its probes report: seconds from
//! a fixed origin natively, and zero on wasm32, where `Instant::now()`
//! traps (fn-198 host decision 8). A timing there reads zero; nothing the
//! engine grows reads a clock.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn seconds() -> f64 {
    static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    ORIGIN
        .get_or_init(std::time::Instant::now)
        .elapsed()
        .as_secs_f64()
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn seconds() -> f64 {
    0.0
}
