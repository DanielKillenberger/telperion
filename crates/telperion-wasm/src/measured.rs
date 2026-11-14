//! The measurement build's allocator (fn-369 step 1, `measure-ceiling`
//! only, never shipped): the counting allocator, and beside it the live
//! high-water mark, every block of `LARGE` bytes or more with the stage that
//! asked for it, the large blocks live when the mark last rose, and the one
//! request that failed. Nothing here allocates; 32-bit Wasm runs it on one
//! thread.
use crate::live::{Counting, LIVE};
use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::sync::atomic::Ordering::Relaxed;
use telperion_core::memory::measure;

/// The smallest block recorded: 16 MiB.
const LARGE: usize = 16 << 20;
const EVENTS: usize = 4096;
const BLOCKS: usize = 256;

/// One large allocation or reallocation: its size, the old block's size
/// (0 for a fresh one), the stage and the address.
#[derive(Clone, Copy)]
#[repr(C)]
struct Event([u32; 4]);
/// One live large block: its address, size and the stage that made it.
#[derive(Clone, Copy)]
#[repr(C)]
struct Block([u32; 3]);

struct Record {
    high: usize,
    high_stage: u32,
    high_pages: u32,
    events: [Event; EVENTS],
    seen: usize,
    live: [Block; BLOCKS],
    n_live: usize,
    version: u32,
    snap: [Block; BLOCKS],
    n_snap: usize,
    snap_version: u32,
    /// The failed request: size, old size, stage, live bytes, pages, failed.
    fail: [u32; 6],
    summary: [u32; 16],
}

struct Cell(UnsafeCell<Record>);
// One thread: 32-bit Wasm without threads.
unsafe impl Sync for Cell {}

static R: Cell = Cell(UnsafeCell::new(Record {
    high: 0,
    high_stage: 0,
    high_pages: 0,
    events: [Event([0; 4]); EVENTS],
    seen: 0,
    live: [Block([0; 3]); BLOCKS],
    n_live: 0,
    version: 0,
    snap: [Block([0; 3]); BLOCKS],
    n_snap: 0,
    snap_version: 0,
    fail: [0; 6],
    summary: [0; 16],
}));

fn record() -> &'static mut Record {
    unsafe { &mut *R.0.get() }
}

fn pages() -> u32 {
    core::arch::wasm32::memory_size(0) as u32
}

impl Record {
    fn rise(&mut self) {
        let live = LIVE.load(Relaxed);
        if live <= self.high {
            return;
        }
        (self.high, self.high_stage, self.high_pages) = (live, measure::stage(), pages());
        if self.version != self.snap_version {
            self.snap = self.live;
            (self.n_snap, self.snap_version) = (self.n_live, self.version);
        }
    }
    fn event(&mut self, size: usize, old: usize, ptr: *mut u8) {
        if self.seen < EVENTS {
            let e = [size as u32, old as u32, measure::stage(), ptr as u32];
            self.events[self.seen] = Event(e);
        }
        self.seen += 1;
    }
    fn add(&mut self, ptr: *mut u8, size: usize) {
        if size >= LARGE && self.n_live < BLOCKS {
            self.live[self.n_live] = Block([ptr as u32, size as u32, measure::stage()]);
            (self.n_live, self.version) = (self.n_live + 1, self.version + 1);
        }
    }
    fn remove(&mut self, ptr: *mut u8, size: usize) {
        if size < LARGE {
            return;
        }
        if let Some(k) = self.live[..self.n_live]
            .iter()
            .position(|b| b.0[0] == ptr as u32)
        {
            self.n_live -= 1;
            self.live[k] = self.live[self.n_live];
            self.version += 1;
        }
    }
    fn failed(&mut self, size: usize, old: usize) {
        let live = LIVE.load(Relaxed) as u32;
        self.fail = [size as u32, old as u32, measure::stage(), live, pages(), 1];
    }
    fn done(&mut self, p: *mut u8, size: usize, old: Option<(*mut u8, usize)>) -> *mut u8 {
        let old_size = old.map_or(0, |o| o.1);
        if p.is_null() {
            self.failed(size, old_size);
            return p;
        }
        if let Some((q, n)) = old {
            self.remove(q, n);
        }
        if size >= LARGE || old_size >= LARGE {
            self.event(size, old_size, p);
        }
        self.add(p, size);
        self.rise();
        p
    }
}

/// The counting allocator, measured.
pub(crate) struct Measured;

unsafe impl GlobalAlloc for Measured {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { Counting.alloc(layout) };
        record().done(p, layout.size(), None)
    }
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { Counting.alloc_zeroed(layout) };
        record().done(p, layout.size(), None)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { Counting.dealloc(ptr, layout) };
        record().remove(ptr, layout.size());
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        let p = unsafe { Counting.realloc(ptr, layout, size) };
        record().done(p, size, Some((ptr, layout.size())))
    }
}

/// One of the records as u32 words: 0 the summary, 1 the large events
/// (four words each), 2 the large blocks live at the high-water mark
/// (three words each).
pub(crate) fn words(kind: u32) -> (*const u32, usize) {
    let r = record();
    match kind {
        0 => {
            let f = r.fail;
            r.summary = [
                r.high as u32,
                r.high_stage,
                r.high_pages,
                LIVE.load(Relaxed) as u32,
                pages(),
                r.seen as u32,
                r.n_snap as u32,
                f[0],
                f[1],
                f[2],
                f[3],
                f[4],
                f[5],
                measure::stage(),
                0,
                0,
            ];
            (r.summary.as_ptr(), r.summary.len())
        }
        1 => (r.events.as_ptr().cast(), 4 * r.seen.min(EVENTS)),
        2 => (r.snap.as_ptr().cast(), 3 * r.n_snap),
        _ => (std::ptr::null(), 0),
    }
}
