//! Sums carried up the tree on every core (fn-211). A pass that adds each
//! child's share into its parent, children in turn from the last, is the
//! same to the bit when each parent takes its children's shares itself,
//! in that order, once they are done: so the axes of one depth are worked
//! at once, the deepest first.
use crate::cores::{map, picked, PIECE};

/// The axes by depth from the root, each depth in index order, and each
/// axis's children, the last first.
pub(crate) struct Tree {
    depths: Vec<Vec<usize>>,
    start: Vec<usize>,
    kids: Vec<usize>,
}

impl Tree {
    /// The tree of `parents`, each axis's parent or none, parents first.
    pub fn new(parents: &[Option<usize>]) -> Self {
        let n = parents.len();
        let mut depth = vec![0usize; n];
        let mut depths: Vec<Vec<usize>> = Vec::new();
        let mut count = vec![0usize; n + 1];
        for (i, parent) in parents.iter().enumerate() {
            depth[i] = parent.map_or(0, |p| depth[p] + 1);
            if depths.len() <= depth[i] {
                depths.push(Vec::new());
            }
            depths[depth[i]].push(i);
            if let Some(p) = parent {
                count[*p + 1] += 1;
            }
        }
        for i in 0..n {
            count[i + 1] += count[i];
        }
        let start = count.clone();
        let mut kids = vec![0; start[n]];
        for (i, parent) in parents.iter().enumerate().rev() {
            if let Some(p) = parent {
                kids[count[*p]] = i;
                count[*p] += 1;
            }
        }
        Self {
            depths,
            start,
            kids,
        }
    }

    /// Axis `i`'s children, the last first.
    pub fn children(&self, i: usize) -> &[usize] {
        &self.kids[self.start[i]..self.start[i + 1]]
    }

    /// `work(i, &mut items[i], done)` for every axis, the deepest first,
    /// each depth's on up to `threads` cores: what each gives, by index,
    /// where `done` holds what every deeper axis gave.
    pub fn upward<T: Send, O: Default + Clone + Send + Sync>(
        &self,
        items: &mut [T],
        threads: usize,
        work: &(dyn Fn(usize, &mut T, &[O]) -> O + Sync),
    ) -> Vec<O> {
        let mut done = vec![O::default(); items.len()];
        for depth in self.depths.iter().rev() {
            let jobs: Vec<(usize, &mut T)> = depth
                .iter()
                .copied()
                .zip(picked(items, depth.iter().copied()))
                .collect();
            let given = map(jobs, PIECE, threads, &|_, (i, item)| work(i, item, &done));
            for (&i, out) in depth.iter().zip(given) {
                done[i] = out;
            }
        }
        done
    }
}
