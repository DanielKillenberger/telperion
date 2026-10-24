//! The live heap, counted at the allocator, and the bytes it leaves in the
//! 4 GiB a 32-bit Wasm memory can reach (fn-291 R2f). Freed heap counts as
//! left, which grown memory never shows.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

/// The heap's live bytes.
static LIVE: AtomicUsize = AtomicUsize::new(0);

/// The system allocator, counting what it holds.
pub(crate) struct Counting;

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            LIVE.fetch_add(layout.size(), Relaxed);
        }
        p
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            LIVE.fetch_add(layout.size(), Relaxed);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        LIVE.fetch_sub(layout.size(), Relaxed);
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let p = unsafe { System.realloc(ptr, layout, size) };
        if !p.is_null() {
            LIVE.fetch_add(size, Relaxed);
            LIVE.fetch_sub(layout.size(), Relaxed);
        }
        p
    }
}

/// The bytes a 32-bit Wasm memory can still give the heap: its 4 GiB less
/// the static data and stack below the heap and the live heap.
#[cfg(target_arch = "wasm32")]
pub(crate) fn left() -> u64 {
    extern "C" {
        static __heap_base: u8;
    }
    let base = unsafe { core::ptr::addr_of!(__heap_base) } as u64;
    (1u64 << 32).saturating_sub(base + LIVE.load(Relaxed) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_count_follows_alloc_realloc_and_dealloc() {
        let counting = Counting;
        let before = LIVE.load(Relaxed);
        let small = Layout::from_size_align(1_000, 8).unwrap();
        unsafe {
            let p = counting.alloc(small);
            assert_eq!(LIVE.load(Relaxed) - before, 1_000);
            let p = counting.realloc(p, small, 3_000);
            assert_eq!(LIVE.load(Relaxed) - before, 3_000);
            counting.dealloc(p, Layout::from_size_align(3_000, 8).unwrap());
        }
        assert_eq!(LIVE.load(Relaxed), before);
    }
}
