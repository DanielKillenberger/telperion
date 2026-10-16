//! fn-198 task 5 scratch: allocations a grow makes, partition and whole.
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
struct Counting;
static N: AtomicU64 = AtomicU64::new(0);
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 { N.fetch_add(1, Relaxed); System.alloc(l) }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) { System.dealloc(p, l) }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 { N.fetch_add(1, Relaxed); System.realloc(p, l, n) }
}
#[global_allocator]
static A: Counting = Counting;
use telperion_space::{grow, grow_whole, Light, Request};
fn main() {
    let a: Vec<String> = std::env::args().skip(1).collect();
    let s = match a[0].as_str() { "beech" => telperion_space::beech(), _ => telperion_space::spruce() };
    let r = Request { age: a[1].parse().unwrap(), seed: 1, budget: 40_000_000, light: Light::default() };
    let n0 = N.load(Relaxed);
    let t = grow(&s, r).unwrap();
    let n1 = N.load(Relaxed);
    drop(t);
    let n2 = N.load(Relaxed);
    let t = grow_whole(&s, r).unwrap();
    let n3 = N.load(Relaxed);
    drop(t);
    println!("{} {}: grow {} allocs, grow_whole {} allocs", a[0], a[1], n1 - n0, n3 - n2);
}
