//! Work spread over the machine's cores (fn-210, lever 2). The engine's
//! parallel stages hand out numbered pieces of work; what each piece
//! makes depends on its number alone, so the tree is the same however
//! many cores took part. The helpers are rayon's global pool, kept for
//! the process (fn-211 task 3): it holds one thread a core, so trees
//! grown at once share the cores rather than each taking every one.
//! Where threads cannot be made (wasm), `available` is one and every
//! piece runs on the calling thread.
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

/// The cores the machine has; one where it cannot say (wasm).
pub(crate) fn available() -> usize {
    std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get)
}

/// Runs `work(k)` for every `k` below `count`, on this thread and on up
/// to `threads - 1` of the pool's threads.
pub(crate) fn spread(count: usize, threads: usize, work: &(dyn Fn(usize) + Sync)) {
    let helpers = threads.min(count).min(rayon::current_num_threads());
    if helpers <= 1 {
        (0..count).for_each(work);
        return;
    }
    let ticket = AtomicUsize::new(0);
    let run = || loop {
        let k = ticket.fetch_add(1, Ordering::Relaxed);
        if k >= count {
            break;
        }
        work(k);
    };
    rayon::in_place_scope(|scope| {
        for _ in 1..helpers {
            scope.spawn(|_| run());
        }
        run();
    });
}

/// The stack each `deep` piece runs on: its recursion goes as deep as the
/// tree's age, about a kilobyte a cycle (an age of 1,500 overflows the 2
/// MB a test thread has). Reserved, not committed, until used.
const DEEP_STACK: usize = 256 << 20;

/// `work(k)` for every `k` below `count`, each on a thread of its own
/// with `DEEP_STACK` of stack: for work that recurses as deep as the
/// tree's age (the closed form's rows, `closed_form/boundary.rs`). In
/// turn on this thread where threads cannot be made (wasm).
pub(crate) fn deep(count: usize, work: &(dyn Fn(usize) + Sync)) {
    if cfg!(target_family = "wasm") {
        (0..count).for_each(work);
        return;
    }
    std::thread::scope(|scope| {
        for k in 0..count {
            std::thread::Builder::new()
                .stack_size(DEEP_STACK)
                .spawn_scoped(scope, move || work(k))
                .expect("a thread for deep work");
        }
    });
}

#[cfg(not(target_family = "wasm"))]
/// `side()` on a thread of its own while `here()` runs on this one, both
/// results; in turn on this thread where threads cannot be made (wasm,
/// fn-212 decision D6). Neither reads the other's work.
pub(crate) fn beside<A: Send, B>(
    side: impl FnOnce() -> A + Send,
    here: impl FnOnce() -> B,
) -> (A, B) {
    std::thread::scope(|scope| {
        let side = scope.spawn(side);
        let here = here();
        (side.join().expect("the work beside"), here)
    })
}

/// The items handed to one piece of work by `map`: enough to pay for
/// the hand-out.
pub(crate) const PIECE: usize = 64;

/// `work(k, item)` for every `item` at its place `k`, in pieces of
/// `piece` on up to `threads` cores: the results in the items' order.
pub(crate) fn map<I: Send, R: Send>(
    items: Vec<I>,
    piece: usize,
    threads: usize,
    work: &(dyn Fn(usize, I) -> R + Sync),
) -> Vec<R> {
    if threads <= 1 || items.len() <= piece {
        return items
            .into_iter()
            .enumerate()
            .map(|(k, i)| work(k, i))
            .collect();
    }
    let mut items = items.into_iter();
    let pieces: Vec<Mutex<Vec<I>>> = (0..items.len().div_ceil(piece))
        .map(|_| Mutex::new(items.by_ref().take(piece).collect()))
        .collect();
    let done: Vec<Mutex<Vec<R>>> = (0..pieces.len()).map(|_| Mutex::new(Vec::new())).collect();
    spread(pieces.len(), threads, &|k| {
        let piece_items = std::mem::take(&mut *pieces[k].lock().unwrap());
        *done[k].lock().unwrap() = piece_items
            .into_iter()
            .enumerate()
            .map(|(j, item)| work(k * piece + j, item))
            .collect();
    });
    done.into_iter()
        .flat_map(|d| d.into_inner().unwrap())
        .collect()
}

/// `map`, each piece always on the same thread of the pool (piece `p` on
/// thread `p mod n`): work on items kept across calls (the stepped
/// executor's roots, fn-211 task 12) grows and frees their memory on the
/// thread that made it, so no core waits on another's allocator.
pub(crate) fn pinned<I: Send, R: Send>(
    items: Vec<I>,
    piece: usize,
    threads: usize,
    work: &(dyn Fn(usize, I) -> R + Sync),
) -> Vec<R> {
    let n = threads.min(rayon::current_num_threads());
    if n <= 1 || items.len() <= piece {
        return map(items, piece, 1, work);
    }
    let mut items = items.into_iter();
    let pieces: Vec<Mutex<Vec<I>>> = (0..items.len().div_ceil(piece))
        .map(|_| Mutex::new(items.by_ref().take(piece).collect()))
        .collect();
    let done: Vec<Mutex<Vec<R>>> = (0..pieces.len()).map(|_| Mutex::new(Vec::new())).collect();
    rayon::broadcast(|ctx| {
        for k in (ctx.index()..pieces.len())
            .step_by(n)
            .take_while(|_| ctx.index() < n)
        {
            let piece_items = std::mem::take(&mut *pieces[k].lock().unwrap());
            *done[k].lock().unwrap() = piece_items
                .into_iter()
                .enumerate()
                .map(|(j, item)| work(k * piece + j, item))
                .collect();
        }
    });
    done.into_iter()
        .flat_map(|d| d.into_inner().unwrap())
        .collect()
}

/// `out[k] = work(k)` for every place in `out`, in pieces of `piece` on
/// up to `threads` cores.
pub(crate) fn fill<T: Send>(
    out: &mut [T],
    piece: usize,
    threads: usize,
    work: &(dyn Fn(usize) -> T + Sync),
) {
    if threads <= 1 || out.len() <= piece {
        out.iter_mut().enumerate().for_each(|(k, o)| *o = work(k));
        return;
    }
    let pieces: Vec<Mutex<Option<&mut [T]>>> =
        out.chunks_mut(piece).map(|c| Mutex::new(Some(c))).collect();
    spread(pieces.len(), threads, &|k| {
        let chunk = pieces[k].lock().unwrap().take().expect("a piece once");
        for (j, o) in chunk.iter_mut().enumerate() {
            *o = work(k * piece + j);
        }
    });
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
