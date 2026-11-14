//! fn-333: where the one cycle's memory goes, and the tree it grows.
//!
//! - `stores <preset> <seed> <age>[,<age>...]`: the grower at each age, by
//!   store (records, bytes, allocations), beside the heap the allocator
//!   holds then and its most so far; growth only, no settle.
//! - `tree <preset> <seed> <age>`: `grow_cycle` whole: its tree's digest,
//!   phytomers and axes, its seconds and the process's peak memory.
//!
//!   cargo run --profile ci -p telperion-core --example footprint -- stores oregon-white-oak 1 80,200
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
use telperion_core::presets::by_identity;
use telperion_space::{grow_cycle, tree_digest, Paused, Request, Retiring};

/// The system's allocator, counting the bytes it holds and their most.
struct Counting;
static HELD: AtomicUsize = AtomicUsize::new(0);
static MOST: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let held = HELD.fetch_add(layout.size(), Relaxed) + layout.size();
        MOST.fetch_max(held, Relaxed);
        System.alloc(layout)
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HELD.fetch_sub(layout.size(), Relaxed);
        System.dealloc(ptr, layout)
    }
}

#[global_allocator]
static ALLOCATOR: Counting = Counting;

const GB: f64 = 1024.0 * 1024.0 * 1024.0;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let family = by_identity(&args[1]).expect("a shipped preset");
    let species = family.space.species.expect("an engine species");
    let seed = args[2].parse().expect("a seed");
    let ages: Vec<u32> = args[3]
        .split(',')
        .map(|a| a.parse().expect("an age"))
        .collect();
    let request = |age| Request {
        age,
        seed,
        light: family.space.light,
    };
    println!("{} seed {seed}", args[1]);
    match args[0].as_str() {
        "stores" => {
            let horizon = request(*ages.iter().max().expect("an age"));
            let mut p = Paused::begin(
                std::sync::Arc::new(species.clone()),
                horizon,
                Retiring::AtAge,
                false,
            )
            .expect("begun");
            let started = std::time::Instant::now();
            for &age in &ages {
                p.grow_to(age).expect("grown");
                stores(age, &p, started.elapsed().as_secs_f64());
            }
        }
        "tree" => {
            let c = grow_cycle(&species, request(ages[0])).expect("a tree");
            let s = &c.structure;
            println!(
                "TREE age {} digest {} phytomers {} axes {} grown {} seconds {:.1} {}",
                ages[0],
                tree_digest(s),
                s.phytomer_count(),
                s.axes.len(),
                c.grown,
                c.seconds,
                peak()
            );
        }
        _ => panic!("stores or tree"),
    }
}

/// The grower at `age`, grown in `seconds`, by store, largest first.
fn stores(age: u32, p: &Paused, seconds: f64) {
    let f = p.footprint();
    let (held, most) = (HELD.load(Relaxed) as f64, MOST.load(Relaxed) as f64);
    println!(
        "AGE {age}: counted {:.3} GiB; heap held {:.3} GiB, most {:.3} GiB; {}; grown in {seconds:.1} s",
        f.bytes() as f64 / GB,
        held / GB,
        most / GB,
        peak()
    );
    println!(
        "  axes held: open {}, closed {}, retired roots {}, inside retired {}",
        f.axes[0], f.axes[1], f.axes[2], f.axes[3]
    );
    let mut s = f.stores.clone();
    s.sort_by(|a, b| b.bytes.cmp(&a.bytes));
    for s in s {
        println!(
            "  {:<24} {:>12} records {:>9.3} GiB {:>10} allocations",
            s.name,
            s.records,
            s.bytes as f64 / GB,
            s.allocations
        );
    }
}

/// The process's peak resident memory.
fn peak() -> String {
    let status = std::fs::read_to_string("/proc/self/status").unwrap_or_default();
    let line = status.lines().find(|l| l.starts_with("VmHWM"));
    line.unwrap_or("VmHWM: unknown")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}
