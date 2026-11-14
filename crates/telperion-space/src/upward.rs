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

    /// `work(i, &mut items[i], done, scratch)` for every axis, the deepest
    /// first, each depth's in pieces on up to `threads` cores: what each
    /// gives, by index, where `done` holds what every deeper axis gave.
    /// Each piece hands its axes one `scratch` in turn, so per-axis sums
    /// reuse one buffer rather than each making its own.
    pub fn upward<T: Send, O: Default + Clone + Send + Sync, S: Default>(
        &self,
        items: &mut [T],
        threads: usize,
        work: &(dyn Fn(usize, &mut T, &[O], &mut S) -> O + Sync),
    ) -> Vec<O> {
        let mut done = vec![O::default(); items.len()];
        for depth in self.depths.iter().rev() {
            let mut picked = picked(items, depth.iter().copied()).into_iter();
            let pieces: Vec<Vec<&mut T>> = depth
                .chunks(PIECE)
                .map(|c| picked.by_ref().take(c.len()).collect())
                .collect();
            let given = map(pieces, 1, threads, &|k, piece| {
                let mut scratch = S::default();
                let at = &depth[k * PIECE..];
                let each = at.iter().zip(piece);
                each.map(|(&i, item)| work(i, item, &done, &mut scratch))
                    .collect::<Vec<O>>()
            });
            for (&i, out) in depth.iter().zip(given.into_iter().flatten()) {
                done[i] = out;
            }
        }
        done
    }
}
