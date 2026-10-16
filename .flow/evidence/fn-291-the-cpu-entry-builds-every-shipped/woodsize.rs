//! fn-291 trace: the native heap's peak and the wood buffers for one case,
//! the same request the wasm entry makes for { surface, foliage }.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use telperion_core::{params, pipeline};

struct Counting;
static NOW: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static LARGEST: AtomicUsize = AtomicUsize::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let now = NOW.fetch_add(l.size(), Relaxed) + l.size();
        PEAK.fetch_max(now, Relaxed);
        LARGEST.fetch_max(l.size(), Relaxed);
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
        LARGEST.fetch_max(n, Relaxed);
        System.realloc(p, l, n)
    }
}
#[global_allocator]
static A: Counting = Counting;

fn main() {
    let mut a = std::env::args().skip(1);
    let id = a.next().unwrap();
    let seed: u32 = a.next().unwrap().parse().unwrap();
    let wood = a.next().map_or(true, |w| w != "foliage");
    let leaves = a.next().map_or(true, |w| w != "surface");
    let mut f = params::by_identity(&id).unwrap();
    f.skeleton.seed = seed;
    let built = pipeline::build(&f, pipeline::Request { wood, leaves, ..Default::default() }).unwrap();
    let nodes = built.skeleton.tree.nodes.len();
    let peak = PEAK.load(Relaxed);
    let mib = |b: usize| b as f64 / 1048576.0;
    let (p, n, c, i) = built.outputs.wood.as_ref().map_or((0, 0, 0, 0), |w| {
        (w.positions.len(), w.normals.len(), w.coords.len(), w.indices.len())
    });
    let wood_bytes = (p + n + c) * 4 + i * 4;
    println!(
        "{{\"id\":\"{id}\",\"seed\":{seed},\"nodes\":{nodes},\"peakMiB\":{:.0},\"largestMiB\":{:.0},\"woodMiB\":{:.0},\"positions\":{p},\"indices\":{i},\"heldAfterMiB\":{:.0}}}",
        mib(peak), mib(LARGEST.load(Relaxed)), mib(wood_bytes), mib(NOW.load(Relaxed))
    );
}
