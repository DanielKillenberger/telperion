//! Leaf-bearing wood handed on (fn-364, host): a span laid too short to
//! draw only for its stretch keeps the kind and the leaf-bearing length
//! it would have had laid whole, and hands that length to the node it
//! merges into, whose stations of it stand at that node's tip.
use super::{Leafing, Tree};

/// One span's leaf-bearing wood handed to `node`: its allotted length
/// (metres, scaled as `Tree::bearing` is), its radius, whether it lay in
/// its axis's terminal wood as allotted, and its retention now and a year
/// before (packed as `Leafing`'s, before and from one boundary). Once a
/// family resolves them (`resolve_handed`), only the spans that bear.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Handed {
    pub node: u32,
    pub length: f64,
    pub radius: f64,
    pub terminal: bool,
    pub keep: u32,
    pub was: u32,
}

impl Handed {
    /// Its retention as a node's, all of it past the boundary.
    pub fn leafing(&self) -> Leafing {
        Leafing {
            older: 0.0,
            keep: self.keep,
            was: self.was,
        }
    }
}

impl Tree {
    /// Whether wood that would be drawn with every stretch at 1 follows
    /// node `i` on its axis: a node left without a child only by its
    /// stretch is not terminal for leaves (fn-364).
    pub fn followed(&self, i: usize) -> bool {
        self.followed.get(i).copied().unwrap_or(false)
    }

    /// The spans handed to node `i`, resolved: each its own short run
    /// of stations at the node's tip.
    pub fn handed_at(&self, i: usize) -> &[Handed] {
        let from = self.handed.partition_point(|h| (h.node as usize) < i);
        let to = self.handed.partition_point(|h| (h.node as usize) <= i);
        &self.handed[from..to]
    }

    /// Keeps the handed spans that bear, by the kind their wood would have
    /// had laid whole (in terminal wood, or of a radius `slender` holds
    /// slender), and none on a node that bears no leaf along its wood, in
    /// node order. Idempotent.
    pub(crate) fn resolve_handed(&mut self, slender: &dyn Fn(f64) -> bool) {
        let mut spans = std::mem::take(&mut self.handed);
        spans.sort_by_key(|h| h.node);
        spans.retain(|h| {
            let node = h.node as usize;
            (h.terminal || slender(h.radius)) && !self.is_clustered(node) && !self.in_clothing(node)
        });
        for h in &mut spans {
            h.terminal = true;
        }
        self.handed = spans;
    }
}
