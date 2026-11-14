//! What a grower holds between cycles, by what holds it (fn-333 R1): each
//! store's records and the bytes it holds, counted as its length and its
//! capacity times its record's size, its heap allocations apart, so an
//! allocator's own overhead per allocation can be read beside it.
use super::super::Grower;
use super::retire::{Fate, Retirement};
use std::mem::size_of;

/// One store: its name, its records, its bytes and its allocations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Store {
    pub name: &'static str,
    pub records: usize,
    pub bytes: usize,
    pub allocations: usize,
}

/// Every store a grower holds, and its axes held by kind: open (on the
/// way to a living apex or a sleeping bud), closed and not retired, a
/// retired subtree's root, and inside one.
#[derive(Debug, Clone, Default)]
pub struct Footprint {
    pub stores: Vec<Store>,
    pub axes: [usize; 4],
}

impl Footprint {
    /// A vector's own block.
    pub(super) fn vec<T>(&mut self, name: &'static str, v: &Vec<T>) {
        self.add(name, v.len(), v.capacity() * size_of::<T>(), 1);
    }

    /// A store of `records` holding `bytes` in `allocations`.
    pub(super) fn add(&mut self, name: &'static str, records: usize, bytes: usize, allocs: usize) {
        self.stores.push(Store {
            name,
            records,
            bytes,
            allocations: allocs,
        });
    }

    /// The vectors inside `outer`'s records, each one allocation where it
    /// holds any capacity.
    pub(super) fn inner<O, T>(
        &mut self,
        name: &'static str,
        outer: &[O],
        of: impl Fn(&O) -> &Vec<T>,
    ) {
        let (mut records, mut bytes, mut allocs) = (0, 0, 0);
        for o in outer {
            let v = of(o);
            records += v.len();
            bytes += v.capacity() * size_of::<T>();
            allocs += usize::from(v.capacity() > 0);
        }
        self.add(name, records, bytes, allocs);
    }

    /// Every store's bytes.
    pub fn bytes(&self) -> usize {
        self.stores.iter().map(|s| s.bytes).sum()
    }
}

impl Grower {
    /// What this grower holds between cycles.
    pub(super) fn footprint(&self) -> Footprint {
        let mut f = Footprint::default();
        f.vec("axes", &self.axes);
        f.inner("axes.phytomers", &self.axes, |a| &a.phytomers);
        f.inner("axes.units", &self.axes, |a| &a.units);
        let joined = self.axes.iter().filter(|a| a.joined.is_some()).count();
        f.add(
            "axes.joined",
            joined,
            joined * size_of::<crate::structure::Join>(),
            joined,
        );
        f.vec("draws", &self.draws);
        f.inner("draws.units", &self.draws, |d| &d.units);
        f.inner("draws.nodes", &self.draws, |d| &d.nodes);
        f.inner("draws.shade", &self.draws, |d| &d.shade);
        f.vec("horizon", &self.horizon);
        f.vec("units", &self.units);
        f.vec("roots", &self.roots);
        f.vec("marked", &self.marked);
        f.vec("successor", &self.successor);
        f.inner("successor.each", &self.successor, |s| s);
        f.vec("live", &self.live);
        f.vec("asleep", &self.asleep);
        f.inner("asleep.each", &self.asleep, |s| s);
        f.vec("woken", &self.woken);
        f.vec("leaves", &self.leaves);
        if let Some(sketch) = &self.sketch {
            f.vec("pencils", &sketch.pencils);
            let heap = sketch.pencils.iter().map(|p| p.layer.heap());
            let allocs = heap.clone().filter(|&b| b > 0).count();
            f.add("pencils.inherited", allocs, heap.sum(), allocs);
            f.inner("pencils.path", &sketch.pencils, |p| &p.path);
        }
        if let Some(w) = &self.working {
            w.footprint(&mut f);
            f.axes = w.tally();
            // Axes laid since the retirement table last grew are open.
            f.axes[0] += self.axes.len() - w.retirement().fate.len();
        }
        if let Some(c) = &self.census {
            c.footprint(&mut f);
        }
        f
    }
}

impl super::Working {
    /// The axes taken in, open, closed and not retired, retired roots, and
    /// inside a retired subtree.
    fn tally(&self) -> [usize; 4] {
        let mut t = [0; 4];
        for (i, f) in self.retirement().fate.iter().enumerate() {
            t[match f {
                Fate::Root(_) => 2,
                Fate::Inside => 3,
                _ if self.closed(i) => 1,
                _ => 0,
            }] += 1;
        }
        t
    }
}

impl Retirement {
    /// What it holds.
    pub(super) fn footprint(&self, f: &mut Footprint) {
        f.vec("retire.fate", &self.fate);
        f.vec("retire.woken", &self.woken);
        f.vec("retire.pending", &self.pending);
        f.vec("retire.fading", &self.fading);
        f.vec("retire.log", &self.log);
        f.vec("retire.stands", &self.stands);
    }
}
