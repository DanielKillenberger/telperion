//! The year's leaves deposited into the light (fn-354 R5): the leaf every
//! open phytomer bears that year (`girth::leaf`), the year's own first in
//! the order laying them gave, then every open axis's older phytomers
//! still in leaf, which none is where no age keeps its leaf past its year.
use super::Grower;
use crate::girth::{bearing, leaf};
use crate::structure::Node;

impl Grower {
    /// The leaves of `cycle`: `grown` names each axis laid on in it, by
    /// the phytomers it laid, `from..to`.
    pub(super) fn deposit(&mut self, cycle: u32, grown: &[(usize, usize, usize)]) {
        let species = &*self.species;
        let mut leaves = std::mem::take(&mut self.leaves);
        leaves.clear();
        let lasting = species.states.iter().any(|s| s.leaf_years > 1.0);
        let mut older = vec![usize::MAX; if lasting { self.axes.len() } else { 0 }];
        for &(i, from, to) in grown {
            if let Some(o) = older.get_mut(i) {
                *o = from;
            }
            let axis = &self.axes[i];
            let state = &species.states[axis.pa];
            for p in &axis.phytomers[from..to] {
                leaves.push((p.tip(), leaf(state, p, cycle)));
            }
        }
        let working = self.working.as_deref();
        let closed = |i: usize| working.is_some_and(|w| w.closed(i));
        for (i, axis) in self.axes.iter().enumerate().take(older.len()) {
            if closed(i) {
                continue;
            }
            let state = &species.states[axis.pa];
            let held = &axis.phytomers[..older[i].min(axis.phytomers.len())];
            for p in &held[bearing(state, held, cycle)..] {
                let area = leaf(state, p, cycle);
                if area > 0.0 {
                    leaves.push((p.tip(), area));
                }
            }
        }
        self.leaves = leaves;
    }
}
