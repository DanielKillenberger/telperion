//! fn-235 R1: each GPU step's identity on stderr before it is submitted,
//! and each wait's end after it returns, so a run killed on a hang names
//! the step that never completed. Test builds only, and only where
//! `TELPERION_TRACE` is set.
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::sync::Mutex;
use std::time::Instant;

/// fn-257's growth profile: what a build did, counted whatever the
/// environment (test builds only): submissions, dispatches, the waits the
/// host blocked on by kind, and the buffers made and their bytes.
pub(crate) struct Counted {
    pub submits: AtomicU64,
    pub dispatches: AtomicU64,
    pub readbacks: AtomicU64,
    pub stages: AtomicU64,
    pub censuses: AtomicU64,
    pub buffers: AtomicU64,
    pub buffer_bytes: AtomicU64,
    pub writes: AtomicU64,
    pub write_bytes: AtomicU64,
    pub passes: AtomicU64,
}

pub(crate) static COUNTED: Counted = Counted {
    submits: AtomicU64::new(0),
    dispatches: AtomicU64::new(0),
    readbacks: AtomicU64::new(0),
    stages: AtomicU64::new(0),
    censuses: AtomicU64::new(0),
    buffers: AtomicU64::new(0),
    buffer_bytes: AtomicU64::new(0),
    writes: AtomicU64::new(0),
    write_bytes: AtomicU64::new(0),
    passes: AtomicU64::new(0),
};

impl Counted {
    /// Every count, in field order, read and reset.
    pub(crate) fn take(&self) -> [u64; 10] {
        [
            &self.submits,
            &self.dispatches,
            &self.readbacks,
            &self.stages,
            &self.censuses,
            &self.buffers,
            &self.buffer_bytes,
            &self.writes,
            &self.write_bytes,
            &self.passes,
        ]
        .map(|c| c.swap(0, Relaxed))
    }
}

/// A dispatch in a compute pass of its own.
pub(crate) fn own_pass() {
    COUNTED.passes.fetch_add(1, Relaxed);
}

/// A buffer made, of `bytes`, and `initial` bytes written to it.
pub(crate) fn made(bytes: u64, initial: u64) {
    COUNTED.buffers.fetch_add(1, Relaxed);
    COUNTED.buffer_bytes.fetch_add(bytes, Relaxed);
    if initial > 0 {
        COUNTED.writes.fetch_add(1, Relaxed);
        COUNTED.write_bytes.fetch_add(initial, Relaxed);
    }
}

struct Log {
    start: Option<Instant>,
    submits: u64,
    pending: Vec<String>,
}

static LOG: Mutex<Log> = Mutex::new(Log {
    start: None,
    submits: 0,
    pending: Vec::new(),
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

/// A dispatch recorded into an encoder not yet submitted.
pub(crate) fn dispatch(entry: &str, groups: u32) {
    COUNTED.dispatches.fetch_add(1, Relaxed);
    with(|log| log.pending.push(format!("{entry}x{groups}")));
}

/// A submission about to go to the queue, with every dispatch recorded
/// since the last.
pub(crate) fn submit() {
    use std::io::Write;
    COUNTED.submits.fetch_add(1, Relaxed);
    with(|log| {
        log.submits += 1;
        let at = ms(log);
        let steps = std::mem::take(&mut log.pending).join(" ");
        eprintln!("[trace {at:.0} ms] submit {}: {steps}", log.submits);
        let _ = std::io::stderr().flush();
    });
}

/// A wait on the device that returned.
pub(crate) fn waited(what: &str) {
    match what {
        "readback" => &COUNTED.readbacks,
        "census" => &COUNTED.censuses,
        _ => &COUNTED.stages,
    }
    .fetch_add(1, Relaxed);
    with(|log| {
        let at = ms(log);
        eprintln!(
            "[trace {at:.0} ms] {what} done after submit {}",
            log.submits
        );
    });
}

/// A compute pipeline made, and what making it took.
pub(crate) fn compiled(entry: &str, took: std::time::Duration) {
    with(|log| {
        let at = ms(log);
        let took = took.as_secs_f64() * 1e3;
        eprintln!("[trace {at:.0} ms] pipeline {entry} made in {took:.0} ms");
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
