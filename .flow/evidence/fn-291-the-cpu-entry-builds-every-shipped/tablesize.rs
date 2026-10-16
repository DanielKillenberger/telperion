//! fn-291 task 3: growth's tables by bytes, natively, for one case grown
//! skeleton-only through `pipeline::build`, with `tablesize-patch.py`
//! applied: each cycle's sample on stderr, the heap's peak on stdout.
//!   tablesize <id> <seed>
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use telperion_core::{params, pipeline};
use telperion_space::TABLE_HEAP as NOW;

struct Counting;
static PEAK: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let now = NOW.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(now, Relaxed);
        System.alloc(l)
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        NOW.fetch_sub(l.size(), Relaxed);
        System.dealloc(p, l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        if n > l.size() {
            let now = NOW.fetch_add(n - l.size(), Relaxed) + n - l.size();
            PEAK.fetch_max(now, Relaxed);
        } else {
            NOW.fetch_sub(l.size() - n, Relaxed);
        }
        System.realloc(p, l, n)
    }
}
#[global_allocator]
static A: Counting = Counting;

fn main() {
    let mut a = std::env::args().skip(1);
    let id = a.next().unwrap();
    let seed: u32 = a.next().unwrap().parse().unwrap();
    let mut f = params::by_identity(&id).unwrap();
    f.skeleton.seed = seed;
    let built = pipeline::build(&f, pipeline::Request::default()).unwrap();
    let mib = |b: usize| b as f64 / 1048576.0;
    println!(
        "{{\"id\":\"{id}\",\"seed\":{seed},\"nodes\":{},\"peakMiB\":{:.0},\"heldAfterMiB\":{:.0},\"sizes\":{}}}",
        built.skeleton.tree.nodes.len(), mib(PEAK.load(Relaxed)), mib(NOW.load(Relaxed)),
        telperion_space::table_sizes()
    );
}
