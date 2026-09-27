//! A limb system's own shell. A first-order axis that `raggedReach` stops
//! short of the crown's shell keeps everything it bears - its deeper axes and
//! the twigs on them - inside the crown's shell scaled about the station it
//! leaves by the share of its room it kept. An axis that kept all of it is
//! bound by the crown's shell itself, to the byte. A shortened system's curtain
//! stays inside its shell too: it drops through no band below it.
use super::*;

/// The shell a limb system grows in: the crown's, scaled about `station`.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Bound {
    station: Vec3,
    scale: f64,
}
impl Default for Bound {
    fn default() -> Self {
        Self::around(Vec3::ZERO, 1.0)
    }
}
impl Bound {
    pub(super) fn around(station: Vec3, scale: f64) -> Self {
        Self { station, scale }
    }
    /// Whether the system is bound more tightly than the crown.
    pub(super) fn short(self) -> bool {
        self.scale != 1.0
    }
    /// `p` where the crown's shell judges it: a point on the system's shell
    /// maps onto the crown's.
    pub(super) fn map(self, p: Vec3) -> Vec3 {
        if self.short() {
            self.station + (p - self.station) / self.scale
        } else {
            p
        }
    }
}

/// The bound of every first-order axis stopped short, by the index of its
/// first node, remapped with the scaffold whenever the tree's storage moves.
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub(super) struct Limbs(Vec<(u32, Bound)>);
impl Limbs {
    pub(super) fn record(&mut self, first: usize, bound: Bound) {
        if !bound.short() {
            return;
        }
        let first = first as u32;
        if let Err(at) = self.0.binary_search_by_key(&first, |e| e.0) {
            self.0.insert(at, (first, bound));
        }
    }
    pub(super) fn remap(&mut self, map: &[Option<u32>]) {
        self.0
            .retain_mut(|(first, _)| map[*first as usize].map(|to| *first = to).is_some());
        self.0.sort_unstable_by_key(|e| e.0);
    }
    /// The bound of the limb system node `i` belongs to: its first-order
    /// ancestor's, or the crown's for a stem and for a system kept whole.
    pub(super) fn of(&self, tree: &Tree, mut i: usize) -> Bound {
        if self.0.is_empty() {
            return Bound::default();
        }
        while let Some(parent) = tree.nodes[i].parent.map(|p| p as usize) {
            if tree.nodes[parent].stem && !tree.nodes[i].stem {
                return self
                    .0
                    .binary_search_by_key(&(i as u32), |e| e.0)
                    .map_or(Bound::default(), |at| self.0[at].1);
            }
            i = parent;
        }
        Bound::default()
    }
}
