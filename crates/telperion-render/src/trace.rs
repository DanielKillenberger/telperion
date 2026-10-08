//! fn-235 R1: each GPU step's identity on stderr before it is submitted,
//! and each wait's end after it returns, so a run killed on a hang names
//! the step that never completed. Test builds only, and only where
//! `TELPERION_TRACE` is set.
use std::sync::Mutex;
use std::time::Instant;

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
    with(|log| log.pending.push(format!("{entry}x{groups}")));
}

/// A submission about to go to the queue, with every dispatch recorded
/// since the last.
pub(crate) fn submit() {
    use std::io::Write;
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
    with(|log| {
        let at = ms(log);
        eprintln!(
            "[trace {at:.0} ms] {what} done after submit {}",
            log.submits
        );
    });
}
