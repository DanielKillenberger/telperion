//! Work spread over the machine's cores (fn-210, lever 2). The engine's
//! parallel stages hand out numbered pieces of work; what each piece
//! makes depends on its number alone, so the tree is the same however
//! many cores took part. Helper threads are drawn from what the process
//! has spare, so trees grown at once do not each take every core.
use std::sync::atomic::{AtomicUsize, Ordering};

/// The helper threads at work in this process, across every tree.
static HELPERS: AtomicUsize = AtomicUsize::new(0);

/// The cores the machine has; one where it cannot say (wasm).
pub(crate) fn available() -> usize {
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
}

/// Runs `work(k)` for every `k` below `count`, on this thread and on up
/// to `threads - 1` helpers.
pub(crate) fn spread(count: usize, threads: usize, work: &(dyn Fn(usize) + Sync)) {
    let ticket = AtomicUsize::new(0);
    let run = || loop {
        let k = ticket.fetch_add(1, Ordering::Relaxed);
        if k >= count {
            break;
        }
        work(k);
    };
    let helpers = Helpers::reserve(threads.min(count).saturating_sub(1), threads);
    std::thread::scope(|scope| {
        for _ in 0..helpers.0 {
            scope.spawn(run);
        }
        run();
    });
}

/// Helper threads held, given back when dropped.
struct Helpers(usize);

impl Helpers {
    /// Up to `wanted` helpers, as far as the process holds fewer than
    /// `cores` in all.
    fn reserve(wanted: usize, cores: usize) -> Self {
        let mut held = HELPERS.load(Ordering::Relaxed);
        loop {
            let take = wanted.min(cores.saturating_sub(held));
            let swapped = HELPERS.compare_exchange_weak(
                held,
                held + take,
                Ordering::Relaxed,
                Ordering::Relaxed,
            );
            match swapped {
                Ok(_) => return Self(take),
                Err(now) => held = now,
            }
        }
    }
}

impl Drop for Helpers {
    fn drop(&mut self) {
        HELPERS.fetch_sub(self.0, Ordering::Relaxed);
    }
}

/// Mutable references to `v`'s elements at the increasing indices
/// `at`, each its own.
pub(crate) fn picked<'v, T>(v: &'v mut [T], at: impl IntoIterator<Item = usize>) -> Vec<&'v mut T> {
    let (mut rest, mut from, mut out) = (v, 0, Vec::new());
    for i in at {
        let (x, tail) = std::mem::take(&mut rest)[i - from..]
            .split_first_mut()
            .expect("an index within, increasing");
        out.push(x);
        rest = tail;
        from = i + 1;
    }
    out
}
