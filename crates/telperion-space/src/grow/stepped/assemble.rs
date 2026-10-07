//! The tree assembled from the coarse grower and its items for today's
//! re-lay and settle (PHASES.md's plan, step 5): the coarse axes in their
//! order, each placeholder replaced by its root, then each item's other
//! axes in its order, the items in the visiting order. Every parent's
//! children keep their relative order, so every bearer-to-child reduction
//! is the whole tree's, and every tree-wide one is order-free
//! (`fixed::sum`). A re-lay's places and walks are handed back after.
use super::handover::reparent;
use super::Executor;
use crate::grow::sketch::Sketch;
use crate::grow::subtree::bare;
use crate::grow::{bud, Apex, Grower};
use crate::lineage::Key;
use crate::structure::{Axis, Origin};

/// Where each assembled axis came from (its item, none for the coarse
/// grower, and its index there) and its origin there.
pub(super) struct Layout {
    from: Vec<(Option<usize>, usize)>,
    origins: Vec<Origin>,
}

impl<'a> Executor<'a> {
    /// The tree assembled, its axes and draws moved out of the growers:
    /// with their pencils for a re-lay, or with their successors and
    /// woken buds to `settle`.
    pub(super) fn assemble(&mut self, settle: bool) -> (Grower<'a>, Layout) {
        let nc = self.coarse.axes.len();
        let mut root_of = vec![None; nc];
        for (k, item) in self.items.iter().enumerate() {
            root_of[item.at] = Some(k);
        }
        let mut offset = vec![0; self.items.len()];
        let mut from: Vec<(Option<usize>, usize)> = (0..nc)
            .map(|i| root_of[i].map_or((None, i), |k| (Some(k), 1)))
            .collect();
        for k in self.visit() {
            offset[k] = from.len();
            from.extend((2..self.items[k].g.axes.len()).map(|j| (Some(k), j)));
        }
        let at: Vec<usize> = self.items.iter().map(|i| i.at).collect();
        let global = |k: Option<usize>, j: usize| match (k, j) {
            (None, j) => j,
            (Some(k), 1) => at[k],
            (Some(k), j) => offset[k] + j - 2,
        };
        let placed: Vec<Origin> = at.iter().map(|&i| self.coarse.axes[i].origin).collect();
        let mut tree = Grower::bare(self.coarse.rules.clone(), self.coarse.threads);
        tree.rules.grown += self.items.iter().map(|i| i.g.rules.grown).sum::<u64>();
        let laid = !settle && self.coarse.sketch.is_some();
        tree.sketch = self
            .coarse
            .sketch
            .as_ref()
            .filter(|_| laid)
            .map(|s| Sketch {
                light: s.light,
                pencils: Vec::with_capacity(from.len()),
            });
        let mut origins = Vec::with_capacity(from.len());
        for &(k, j) in &from {
            let g = match k {
                None => &mut self.coarse,
                Some(k) => &mut self.items[k].g,
            };
            let mut axis = std::mem::replace(&mut g.axes[j], hollow());
            origins.push(axis.origin);
            match (k, j) {
                (Some(k), 1) => axis.origin = placed[k],
                _ => reparent(&mut axis.origin, |p| global(k, p)),
            }
            tree.axes.push(axis);
            tree.draws.push(std::mem::take(&mut g.draws[j]));
            if let (Some(t), Some(s)) = (tree.sketch.as_mut(), g.sketch.as_mut()) {
                t.pencils
                    .push(std::mem::replace(&mut s.pencils[j], bare(0.0)));
            }
            if settle {
                let next = g.successor[j].iter().map(|&s| global(k, s)).collect();
                tree.successor.push(next);
            }
        }
        let growers = std::iter::once((None, &self.coarse))
            .chain(self.items.iter().enumerate().map(|(k, i)| (Some(k), &i.g)));
        for (k, g) in growers {
            let live = g.live.iter().map(|a| Apex {
                axis: global(k, a.axis),
                ..*a
            });
            tree.live.extend(live);
            if settle {
                tree.woken.extend(g.woken.iter().map(|w| {
                    let mut w = *w;
                    (w.axis, w.bearer) = (global(k, w.axis), global(k, w.bearer));
                    w
                }));
                tree.marked.extend(&g.marked);
            }
        }
        (tree, Layout { from, origins })
    }

    /// Hands a re-laid tree's axes, draws and pencils back where they
    /// came from.
    pub(super) fn disperse(&mut self, tree: Grower<'a>, layout: Layout) {
        let mut pencils = tree.sketch.map(|s| s.pencils.into_iter());
        let back = layout.from.iter().zip(layout.origins);
        for ((&(k, j), origin), (mut axis, draws)) in
            back.zip(tree.axes.into_iter().zip(tree.draws))
        {
            axis.origin = origin;
            let g = match k {
                None => &mut self.coarse,
                Some(k) => &mut self.items[k].g,
            };
            (g.axes[j], g.draws[j]) = (axis, draws);
            if let (Some(p), Some(s)) = (pencils.as_mut(), g.sketch.as_mut()) {
                s.pencils[j] = p.next().expect("a pencil each");
            }
        }
    }
}

/// An axis of nothing, left where one was moved out.
fn hollow() -> Axis {
    bud(Key(0), 0, 0, Origin::Seed)
}
