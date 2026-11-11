//! fn-235 R1: each submission on stderr before it goes to the queue, so a
//! run killed on a hang names the submission that never completed. Test
//! builds only, and only where `TELPERION_TRACE` is set.
use std::sync::Mutex;
use std::time::Instant;

struct Log {
    start: Option<Instant>,
    submits: u64,
}

static LOG: Mutex<Log> = Mutex::new(Log {
    start: None,
    submits: 0,
});

fn with(f: impl FnOnce(&mut Log)) {
    if std::env::var_os("TELPERION_TRACE").is_some() {
        f(&mut LOG.lock().unwrap_or_else(|e| e.into_inner()));
    }
}

fn ms(log: &mut Log) -> f64 {
    log.start
        .get_or_insert_with(Instant::now)
        .elapsed()
        .as_secs_f64()
        * 1e3
}

/// A submission about to go to the queue.
pub(crate) fn submit() {
    use std::io::Write;
    with(|log| {
        log.submits += 1;
        let at = ms(log);
        eprintln!("[trace {at:.0} ms] submit {}", log.submits);
        let _ = std::io::stderr().flush();
    });
}

/// Where `TELPERION_TRACE_WAIT` is set too, each submission waited for
/// alone, the queue empty before it, and its wall time logged: what one
/// submission holds the device for (fn-235 R2).
pub(crate) fn ran(device: &wgpu::Device) {
    if std::env::var_os("TELPERION_TRACE_WAIT").is_none() {
        return;
    }
    let started = Instant::now();
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    let took = started.elapsed().as_secs_f64() * 1e3;
    with(|log| {
        let at = ms(log);
        eprintln!("[trace {at:.0} ms] submit {} ran {took:.1} ms", log.submits);
    });
}
