//! The ground bound's roots grown ahead (fn-209 F1 task 9): while the
//! executor scales, thickens and lays its roots (the structure above
//! the cut's own passes hold them), the spare cores grow the roots likeliest
//! to reach the ground, likeliest first, each from its seed alone and
//! scaled as `landing.rs` would. The landing takes a root grown here and
//! grows any other itself, so what is grown ahead decides only when the
//! work runs, never which roots land or what they are.
//!
//! Which roots, in what order, is `likely.rs`.
use super::expansion::RootSeed;
use super::realized::{Root, Stub};
use crate::error::Result;
use crate::grow::Rules;
use crate::species::Species;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

/// A root grown ahead, and whether it is laid on its bearer as laid
/// before bending.
pub(crate) type Ahead = (Result<Root>, bool);

/// What a root grown ahead takes: its species, its rules, the live
/// roots' seeds and their scales.
pub(crate) type Grows<'a> = (
    &'a Species,
    &'a Rules<'a>,
    &'a [&'a RootSeed],
    &'a [(f64, f64)],
);

/// The work the spare cores share: the roots to grow, in order, and the
/// next; those grown before the stubs were laid, to lay, and the next;
/// each root begun; whether the structure's passes have the cores; and
/// whether the landing is done.
#[derive(Default)]
struct Work {
    order: Arc<Vec<usize>>,
    next: usize,
    relay: Arc<Vec<usize>>,
    relaid: usize,
    begun: Vec<bool>,
    held: bool,
    stopped: bool,
}

/// A root to lay, or to grow.
enum Job {
    Lay(usize),
    Grow(usize),
}

impl Work {
    /// The next job, if any.
    fn next_job(&mut self) -> Option<Job> {
        if let Some(&j) = self.relay.get(self.relaid) {
            self.relaid += 1;
            return Some(Job::Lay(j));
        }
        while let Some(&j) = self.order.get(self.next) {
            self.next += 1;
            if !self.begun[j] {
                self.begun[j] = true;
                return Some(Job::Grow(j));
            }
        }
        None
    }

    fn idle(&self) -> bool {
        self.relaid >= self.relay.len() && self.next >= self.order.len()
    }
}

/// Roots grown ahead, by their place among the live roots, and the
/// bearers' stubs as laid before bending, once laid.
pub(crate) struct Spare {
    work: Mutex<Work>,
    changed: Condvar,
    frames: OnceLock<Arc<Vec<Stub>>>,
    slots: Vec<Mutex<Option<Ahead>>>,
}

impl Spare {
    /// For `live` live roots.
    pub fn new(live: usize) -> Self {
        Self {
            work: Mutex::new(Work {
                begun: vec![false; live],
                ..Work::default()
            }),
            changed: Condvar::new(),
            frames: OnceLock::new(),
            slots: (0..live).map(|_| Mutex::new(None)).collect(),
        }
    }

    fn change(&self, change: impl FnOnce(&mut Work)) {
        change(&mut self.work.lock().expect("the work"));
        self.changed.notify_all();
    }

    /// The roots to grow, in order, in place of any before.
    pub fn plan(&self, order: Vec<usize>) {
        self.change(|w| (w.order, w.next) = (Arc::new(order), 0));
    }

    /// The bearers' stubs as laid before bending, and the roots to grow
    /// from now, in order: every root grown ahead is laid on its stub
    /// from now, those grown before first.
    pub fn laid(&self, frames: Arc<Vec<Stub>>, order: Vec<usize>) {
        let _ = self.frames.set(frames);
        self.change(|w| {
            let begun = (0..w.begun.len()).filter(|&j| w.begun[j]);
            (w.relay, w.relaid) = (Arc::new(begun.collect()), 0);
            (w.order, w.next) = (Arc::new(order), 0);
        });
    }

    /// The structure's own passes take the cores (`true`) or give them
    /// back: no root is begun meanwhile.
    pub fn hold(&self, held: bool) {
        self.change(|w| w.held = held);
    }

    /// No more roots are needed ahead.
    pub fn stop(&self) {
        self.change(|w| w.stopped = true);
    }

    /// Grows and lays roots as planned until the landing is done.
    pub fn work(&self, (species, rules, seeds, scales): Grows) {
        loop {
            let job = {
                let w = self.work.lock().expect("the work");
                let mut w = self
                    .changed
                    .wait_while(w, |w| !w.stopped && (w.held || w.idle()))
                    .expect("the work");
                if w.stopped {
                    return;
                }
                w.next_job()
            };
            match job {
                Some(Job::Lay(j)) => self.lay(species, j),
                Some(Job::Grow(j)) => {
                    let root = super::spare::grown(species, rules, seeds[j], scales[j]);
                    let mut slot = self.slots[j].lock().expect("a slot");
                    // Laid here if the stubs were laid by now, and otherwise
                    // by the pass over the roots begun before them.
                    *slot = Some(match (root, self.frames.get()) {
                        (Ok(mut root), Some(frames)) => {
                            let laid = root.lay_on(species, frames[j].axis());
                            (laid.map(|()| root), true)
                        }
                        (root, _) => (root, false),
                    });
                }
                None => {}
            }
        }
    }

    /// The root at live place `j`, grown before the stubs were laid, laid.
    fn lay(&self, species: &Species, j: usize) {
        let Some(frames) = self.frames.get() else {
            return;
        };
        let mut slot = self.slots[j].lock().expect("a slot");
        *slot = slot.take().map(|ahead| match ahead {
            (Ok(mut root), false) => {
                let laid = root.lay_on(species, frames[j].axis());
                (laid.map(|()| root), true)
            }
            ahead => ahead,
        });
    }

    /// The root at live place `j`, if grown ahead.
    pub fn take(&self, j: usize) -> Option<Ahead> {
        self.slots[j].lock().expect("a slot").take()
    }

    /// The roots begun ahead.
    pub fn grown(&self) -> usize {
        let w = self.work.lock().expect("the work");
        w.begun.iter().filter(|&&b| b).count()
    }
}

/// A live root grown and scaled as `landing.rs` grows it.
pub(crate) fn grown(
    species: &Species,
    rules: &Rules,
    seed: &RootSeed,
    scales: (f64, f64),
) -> Result<Root> {
    let mut root = Root::grow(species, rules, &seed.handed)?;
    root.girth(species, scales);
    Ok(root)
}
