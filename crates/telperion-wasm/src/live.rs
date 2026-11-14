//! The live heap, counted at the allocator, and the one request the machine
//! refused (fn-369): its size, the live bytes then and the stage the build
//! was in. A failed allocation the code cannot return aborts the build as a
//! trap; the loader reads this record after it, through exports that touch
//! no engine state, and names the refusal.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU32, AtomicUsize, Ordering::Relaxed};

/// The heap's live bytes.
pub(crate) static LIVE: AtomicUsize = AtomicUsize::new(0);

/// The refused request: failed (0 or 1), bytes asked, bytes live, the
/// stage's tag; four u32 words, read as one record.
pub(crate) static REFUSED: [AtomicU32; 4] = [const { AtomicU32::new(0) }; 4];

/// Forgets a refusal: a build begins.
pub(crate) fn begin() {
    REFUSED[0].store(0, Relaxed);
}

/// The system allocator, counting what it holds and recording the request
/// it could not give.
pub(crate) struct Counting;

impl Counting {
    fn given(&self, p: *mut u8, size: usize) -> *mut u8 {
        if p.is_null() {
            let live = LIVE.load(Relaxed) as u32;
            let stage = telperion_core::memory::stage::now();
            for (word, v) in REFUSED.iter().zip([1, size as u32, live, stage]) {
                word.store(v, Relaxed);
            }
        }
        p
    }
}

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            LIVE.fetch_add(layout.size(), Relaxed);
        }
        self.given(p, layout.size())
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc_zeroed(layout) };
        if !p.is_null() {
            LIVE.fetch_add(layout.size(), Relaxed);
        }
        self.given(p, layout.size())
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
        self.given(p, size)
    }
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

    #[test]
    fn a_request_the_machine_refuses_is_recorded_with_its_stage() {
        use telperion_core::memory::stage;
        begin();
        assert_eq!(REFUSED[0].load(Relaxed), 0);
        stage::at(stage::CONVERT);
        // Past any address space a 64-bit host maps, so no allocator gives it.
        let size = isize::MAX as usize - 7;
        let huge = Layout::from_size_align(size, 8).unwrap();
        // Held through `black_box`, so the optimiser cannot drop the call and
        // take its success for granted.
        let given = std::hint::black_box(unsafe { Counting.alloc(huge) });
        assert!(given.is_null());
        let record = REFUSED.each_ref().map(|w| w.load(Relaxed));
        assert_eq!(record[0], 1);
        assert_eq!(record[1], size as u32);
        assert_eq!(record[3], stage::CONVERT);
        begin();
        assert_eq!(REFUSED[0].load(Relaxed), 0);
    }
}
