//! Consumer skeletons are parent-before-child with structural nodes before locals.
//! During a retained advance, slices may append structure after local nodes;
//! the specimen provides a cached packed layout when a consumer reads it.
//! Timeline nodes retain their birth identity and slot after their death year;
//! the legacy envelope builder still compacts. Consumer reads exclude dead nodes.
mod identity;
mod section;
mod shoot;
use crate::{math::Vec3, Error, Result};
pub use identity::NodeIdentity;
pub(crate) use identity::NodeKey;
pub use section::{Section, SectionRing};
pub(crate) use shoot::LocalWidth;
pub use shoot::{BudFate, ShootState};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKind {
    #[default]
    Structural,
    Branch,
    Twig,
}
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Node {
    /// Stable birth identity, assigned by the owning specimen; never a storage index.
    pub identity: NodeIdentity,
    pub shoot: ShootState,
    pub position: Vec3,
    /// None only at the root; otherwise strictly earlier than this node.
    pub parent: Option<u32>,
    /// Solved distal and proximal edge radii; zero before radius solving.
    pub radius: f64,
    pub start_radius: f64,
    /// Allocation at the origin of this entire branch run, not this edge.
    pub base_radius: f64,
    /// First node of the branch run; structural nodes use their own index.
    /// The run's stable identity is the identity of this first node.
    pub branch: u32,
    pub kind: NodeKind,
    /// Wood of a stem, an order-zero axis: the one trunk of a tree on one
    /// stem, or any stem of a clump. It is how a fork of stems is told from a
    /// limb, so it is carried through every read, the shoot-less ones too,
    /// where the bud's fate is not. The root is every stem's base and none.
    pub stem: bool,
    /// On the first node of a codominant sibling, the wood a fork parts off
    /// beside the primary that carries its axis on, the sibling's weight: how
    /// far it has grown in, 0 to 1, from the rate that switched it on and the
    /// ways' fraction. None on every other node. Carried through every read,
    /// as `stem` is, so the radius solve and the surface read one recorded
    /// fact.
    pub codominant: Option<f64>,
}
impl Node {
    pub fn root() -> Self {
        Self {
            identity: NodeIdentity::default(),
            shoot: ShootState::default(),
            position: Vec3::ZERO,
            parent: None,
            radius: 0.0,
            start_radius: 0.0,
            base_radius: 0.0,
            branch: 0,
            kind: NodeKind::Structural,
            stem: false,
            codominant: None,
        }
    }
}
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Diagnostics {
    pub node_capped: bool,
    pub level_capped: bool,
    pub attraction_capped: bool,
}
impl Diagnostics {
    pub fn complete(self) -> bool {
        !(self.node_capped || self.level_capped || self.attraction_capped)
    }
}
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "json", derive(serde::Serialize, serde::Deserialize))]
pub struct Tree {
    pub nodes: Vec<Node>,
    pub crossover: usize,
    pub diagnostics: Diagnostics,
    /// The runs drawn as a cell of their stem's lattice rather than round,
    /// ordered by the last node of each. Empty on every tree that shapes none.
    /// Drawn from the tree's own rows after it is grown and never stored: a
    /// specimen or a snapshot is the skeleton, and its bytes are the ones they
    /// always were.
    #[cfg_attr(feature = "json", serde(skip))]
    pub sections: Vec<Section>,
}
impl Tree {
    /// The wood a canopy or a twig layer measures itself against: the thickest
    /// stem leaving the root, read through whatever radius the caller trusts.
    /// The root carries every stem's pipe together, so a clump has to measure
    /// one stem rather than the combined trunk none of its shoots would ever
    /// read as slender beside; a tree on one stem measures the root itself,
    /// which IS that stem's own base, and so is left exactly where it was.
    /// A clump that parts above the ground is measured the same way, at the
    /// fork: the node two stems leave, which a limb never is, so a single stem
    /// still has no fork to find.
    pub(crate) fn stem_radius(&self, radius: impl Fn(usize) -> f64) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }
        // What each node parts into: every child of the root, as a clump born
        // there has always been counted, and stems above it.
        let parts = |i: usize, n: &Node| i == 0 || n.stem;
        let mut runs = vec![0_u8; self.nodes.len()];
        for n in self.nodes.iter().skip(1) {
            let p = n.parent.unwrap() as usize;
            if parts(p, n) {
                runs[p] = runs[p].saturating_add(1);
            }
        }
        let Some(fork) = runs.iter().position(|&r| r > 1) else {
            return radius(0);
        };
        let parted = || {
            self.nodes
                .iter()
                .enumerate()
                .skip(1)
                .filter(move |(_, n)| n.parent == Some(fork as u32) && parts(fork, n))
        };
        let widest = parted().map(|(i, _)| radius(i)).fold(0.0, f64::max);
        // A fork growing in moves the measure from the root to the fork by as
        // much as its siblings have grown; stems no fork recorded part whole.
        let grown = parted()
            .filter_map(|(_, n)| n.codominant)
            .fold(None, |most: Option<f64>, w| {
                Some(most.map_or(w, |m| m.max(w)))
            })
            .unwrap_or(1.0);
        widest - (widest - radius(0)) * (1.0 - grown)
    }
    /// The distal node of every order-zero axis, in birth order: where a stem
    /// stops carrying itself further. A stem apex may still bear laterals and
    /// a twig layer, so it is the last node of the stem run rather than a
    /// childless node.
    pub fn stem_apices(&self) -> Vec<usize> {
        let mut carried = vec![false; self.nodes.len()];
        for n in self.nodes.iter().skip(1) {
            if n.stem {
                carried[n.parent.unwrap() as usize] = true;
            }
        }
        let mut found: Vec<usize> = (1..self.nodes.len())
            .filter(|&i| self.nodes[i].stem && !carried[i])
            .collect();
        // One apex a stem, so a handful: an insertion sort on the unique
        // identity gives the library sort's order in a fraction of its code,
        // which the slim field binding carries.
        for i in 1..found.len() {
            let mut j = i;
            while j > 0 && self.nodes[found[j - 1]].identity > self.nodes[found[j]].identity {
                found.swap(j - 1, j);
                j -= 1;
            }
        }
        found
    }
    pub fn validate(&self) -> Result<()> {
        if self.nodes.len() > u32::MAX as usize || self.crossover > self.nodes.len() {
            return Err(Error::InvalidInput("tree length"));
        }
        self.validate_range(0..self.nodes.len(), false)
    }
    pub(crate) fn validate_range(&self, range: std::ops::Range<usize>, solved: bool) -> Result<()> {
        for i in range {
            let n = &self.nodes[i];
            if solved && n.radius <= 0.0 {
                return Err(Error::InvalidInput("unsolved radii"));
            }
            if !n.position.is_finite()
                || ![n.radius, n.start_radius, n.base_radius]
                    .iter()
                    .all(|r| r.is_finite() && *r >= 0.0)
                || n.start_radius < n.radius
            {
                return Err(Error::InvalidInput("node position or radius"));
            }
            if (i == 0 && n.parent.is_some())
                || (i > 0 && !n.parent.is_some_and(|p| (p as usize) < i))
            {
                return Err(Error::InvalidInput("parent order"));
            }
            if n.branch as usize > i {
                return Err(Error::InvalidInput("branch index"));
            }
        }
        Ok(())
    }
    pub fn validate_solved(&self) -> Result<()> {
        self.validate()?;
        if self.nodes.iter().any(|n| n.radius <= 0.0) {
            return Err(Error::InvalidInput("unsolved radii"));
        }
        let ordered = self.sections.windows(2).all(|w| w[0].node < w[1].node);
        let held = self
            .sections
            .iter()
            .all(|s| (s.node as usize) < self.nodes.len() && s.is_finite());
        if !ordered || !held {
            return Err(Error::InvalidInput("run sections"));
        }
        Ok(())
    }
    /// The section the run ending at `node` is drawn as, if it is shaped.
    pub fn section(&self, node: usize) -> Option<&Section> {
        if self.sections.is_empty() {
            return None;
        }
        self.sections
            .binary_search_by_key(&node, |s| s.node as usize)
            .ok()
            .map(|i| &self.sections[i])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A trunk from the root to `fork` metres, two runs above it, the second
    /// of them a stem if `second` and a limb if not, and a second stem at the
    /// root if `clump`.
    fn tree(fork: f64, second: bool, clump: bool) -> Tree {
        let node = |y: f64, parent: Option<u32>, radius: f64| Node {
            position: Vec3::new(0.0, y, 0.0),
            parent,
            radius,
            stem: true,
            ..Node::root()
        };
        let mut nodes = vec![
            node(0.0, None, 0.3),
            node(fork, Some(0), 0.28),
            node(4.0, Some(1), 0.2),
            node(4.0, Some(1), 0.15),
        ];
        nodes[3].stem = second;
        if clump {
            nodes.push(node(3.0, Some(0), 0.25));
        }
        Tree {
            crossover: nodes.len(),
            nodes,
            ..Tree::default()
        }
    }

    #[test]
    fn a_fork_growing_in_moves_the_measure_from_the_root_by_its_weight() {
        let radius = |t: &Tree| t.stem_radius(|i| t.nodes[i].radius);
        let grown = |weight: f64| {
            let mut t = tree(1.0, true, false);
            t.nodes[3].codominant = Some(weight);
            radius(&t)
        };
        // A sibling of next to no weight leaves the root's measure, a whole
        // one the fork's, and the ones between walk from one to the other.
        assert!((grown(1e-9) - 0.3).abs() < 1e-9);
        assert_eq!(grown(1.0), 0.2);
        assert!((grown(0.5) - 0.25).abs() < 1e-12);
    }

    #[test]
    fn a_clump_is_measured_by_its_largest_stem_wherever_it_parts() {
        let radius = |t: &Tree| t.stem_radius(|i| t.nodes[i].radius);
        // One stem with a limb: the root, as it always was.
        assert_eq!(radius(&tree(1.0, false, false)), 0.3);
        // Two stems at the root: the larger of them, as fn-38 measured it.
        assert_eq!(radius(&tree(1.0, false, true)), 0.28);
        assert_eq!(radius(&tree(1.0, true, true)), 0.28);
        // Two stems parting at a metre: the larger of them, not the trunk.
        assert_eq!(radius(&tree(1.0, true, false)), 0.2);
        // A read that dropped every bud's fate still tells them apart.
        let mut shootless = tree(1.0, false, false);
        for n in &mut shootless.nodes {
            n.shoot = ShootState::default();
        }
        assert_eq!(radius(&shootless), 0.3);
    }
}
