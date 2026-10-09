//! Consumer skeletons are parent-before-child with structural nodes before locals.
//! During a retained advance, slices may append structure after local nodes;
//! the specimen provides a cached packed layout when a consumer reads it.
//! Timeline nodes retain their birth identity and slot after their death year;
//! the legacy envelope builder still compacts. Consumer reads exclude dead nodes.
mod bloom;
mod cluster;
mod identity;
mod leafing;
mod section;
mod shoot;
mod stand;
use crate::{math::Vec3, Error, Result};
pub use bloom::Bloom;
pub use cluster::{cluster_key, Cluster};
pub use identity::NodeIdentity;
pub(crate) use identity::NodeKey;
pub use leafing::{keeps, kept_share, retention, Leafing, KEEP};
pub use section::{Section, SectionRing};
pub use shoot::{BudFate, ShootState};
pub use stand::{stand_key, Stand};

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
    /// Stable identity, never a storage index: the scaffold specimen's birth
    /// order, or the engine phytomer's lineage key (fn-220).
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
    /// The node budget left the tree incomplete: its framework alone exceeds
    /// the budget, or not even the least twig detail fits (`incomplete`). A
    /// tree that gave up detail to fit is whole and not capped (`reduced`).
    pub node_capped: bool,
    pub level_capped: bool,
    pub attraction_capped: bool,
    /// The twig generations a direct build at its node budget kept, in
    /// sixteenths of a generation: whole generations grown from the scaffold
    /// outward, then the share of the next one's laterals still branching
    /// rather than ending as twigs (below one generation, the share of the
    /// first generation's twigs grown at all). None where the budget did not
    /// bind the twigs. Chosen per build and never stored: a snapshot or a JSON
    /// read of the diagnostics reads none; the browser gets it as `twigDetail`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub twig_detail: Option<u16>,
}
impl Diagnostics {
    /// Every axis the build grew reaches its tips. A tree that gave up
    /// detail to meet its node budget is complete.
    pub fn complete(self) -> bool {
        !(self.node_capped || self.level_capped || self.attraction_capped)
    }
    /// The build gave up twig detail to finish inside its node budget, and is
    /// whole: judged as drawn.
    pub fn reduced(self) -> bool {
        !self.node_capped && self.twig_detail.is_some()
    }
    /// Why the node budget left the tree incomplete, or none where it did not.
    /// On the direct build only the framework can stop without twig detail.
    pub fn incomplete(self) -> Option<&'static str> {
        match (self.node_capped, self.twig_detail) {
            (false, _) => None,
            (true, None) => Some("the framework alone exceeds the node budget"),
            (true, Some(_)) => Some("no twig detail fits the node budget"),
        }
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
    /// The pipe model's `[start, distal]` radii of each structural node,
    /// kept where `girthHold` drew the wood thicker: what decides which wood
    /// bears leaves, so a hold adds or loses none. Empty where nothing is held.
    /// Drawn by the direct build's last radius pass and never stored, as
    /// `sections` is: a specimen's bytes are the ones they always were.
    #[cfg_attr(feature = "json", serde(skip))]
    pub pipe: Vec<[f64; 2]>,
    /// Each fine root's node drawn from wood above the cut (fn-209 F1): its
    /// first axis's first node, and any lateral its first phytomers,
    /// merged into its bearer's node, hang there; in increasing order. The
    /// seam its wood starts a run of its own at, and
    /// from which all it bears keeps its own side's run order (fn-212 D4,
    /// D5). Empty on a tree grown whole. Never stored, as `sections`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub seams: Vec<u32>,
    /// Each fine root drawn, in the order its first axis is converted
    /// (fn-212 task 4). Empty on a tree grown whole. Never stored, as
    /// `seams`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub attachments: Vec<Attachment>,
    /// Each node's wood's frame as the engine laid it, its heading and the
    /// side its first bud faces (y up), where the engine grew the tree:
    /// what a fine root's run seeds its frame from at its socket (fn-212,
    /// host decision 2026-10-07). Never stored, as `seams`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub frames: Vec<[f32; 6]>,
    /// Each node's key, from its axis's lineage and its phytomer's place
    /// along the axis (`node_key`): what a run's leaves draw their random
    /// values by, whatever order an executor walks the runs in (fn-212,
    /// host decision 2026-10-07). Empty on a tree no engine grew, whose
    /// nodes key by their index. Never stored, as `seams`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub keys: Vec<u32>,
    /// Each node's wood as the engine grew it, in metres: the internodes
    /// of the phytomers it draws, each its PA's length at its scale,
    /// summed. Record data, the same on every executor, which a run's
    /// stations are counted and split by (fn-212 D2, host decision
    /// 2026-10-07). Empty on a tree no engine grew, whose wood is its
    /// chord. Never stored, as `seams`.
    #[cfg_attr(feature = "json", serde(skip))]
    pub grown: Vec<f64>,
    /// The nodes that may flower, in node order (fn-216). Empty on every
    /// tree whose ages bear none, and never stored, as `sections` is.
    #[cfg_attr(feature = "json", serde(skip))]
    pub blooms: Vec<Bloom>,
    /// The short shoots' tips, in node order (fn-223). Empty on every tree
    /// whose ages state no cluster, and never stored, as `sections` is.
    #[cfg_attr(feature = "json", serde(skip))]
    pub clusters: Vec<Cluster>,
    /// Whether each node is a short shoot's wood, which bears no leaf along
    /// its length (fn-223). Empty where `clusters` is.
    #[cfg_attr(feature = "json", serde(skip))]
    pub clustered: Vec<bool>,
    /// Each node's leaf retention by its wood's age (fn-248). Empty on
    /// every tree whose ages bear for ever, and never stored, as
    /// `clusters` is.
    #[cfg_attr(feature = "json", serde(skip))]
    pub leafing: Vec<Leafing>,
    /// The fine roots a coarse tree holds as their expected occupancy, never
    /// grown (fn-214): empty on every tree grown whole. Never stored.
    #[cfg_attr(feature = "json", serde(skip))]
    pub stands: Vec<Stand>,
}

/// A 32-bit finaliser: the one the leaves' draws already use.
pub fn mix32(z: u32) -> u32 {
    let z = (z ^ (z >> 16)).wrapping_mul(0x21f0_aaad);
    let z = (z ^ (z >> 15)).wrapping_mul(0x735a_2d97);
    z ^ (z >> 15)
}

/// A node's key: its axis's lineage and its phytomer's place along the
/// axis, mixed; the GPU executor's drawing computes the same words.
pub fn node_key(lineage: u64, place: usize) -> u32 {
    let low = mix32(lineage as u32);
    let high = mix32(low ^ (lineage >> 32) as u32);
    mix32(high ^ place as u32)
}

/// The random stream a run's leaves draw from: the family's seed and the
/// key of the run's first node (fn-212, host decision 2026-10-07).
pub fn leaf_stream(seed: u32, key: u32) -> u32 {
    mix32(seed ^ 0x2c9e_1a7f ^ key)
}

/// A fine root as the conversion drew it (fn-212 task 4): the record an
/// executor dressing the root on its own starts from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attachment {
    /// Its first axis's lineage.
    pub lineage: u64,
    /// The node its bearer's phytomer is drawn as, which it is drawn from.
    pub node: u32,
    /// Its seams (`Tree::seams`): the first node its first axis draws, if
    /// any, then any lateral that hangs from its bearer's node.
    pub seams: Vec<u32>,
    /// Whether it forks its bearer codominantly, read from its base radius
    /// as phase 2 handed it up against its bearer's node's radius: what
    /// the sweep eases its bearer's fork by, whichever executor draws the
    /// root (fn-212 task 7, host option 3).
    pub fork: bool,
}
impl Tree {
    /// Whether node `i` is a short shoot's wood (`clustered`).
    pub fn is_clustered(&self, i: usize) -> bool {
        self.clustered.get(i).copied().unwrap_or(false)
    }
    /// Node `i`'s leaf retention (`leafing`): every station kept where the
    /// tree records none.
    pub fn leafing(&self, i: usize) -> Leafing {
        self.leafing.get(i).copied().unwrap_or(Leafing::ALL)
    }
    /// Node `i`'s key (`keys`), or its index where no engine grew it.
    pub fn key(&self, i: usize) -> u32 {
        self.keys.get(i).copied().unwrap_or(i as u32)
    }
    /// Node `i`'s grown wood (`grown`), or its chord from its parent where
    /// no engine grew it.
    pub fn grown_length(&self, i: usize) -> f64 {
        self.grown.get(i).copied().unwrap_or_else(|| {
            let n = &self.nodes[i];
            n.parent.map_or(0.0, |p| {
                self.nodes[p as usize].position.distance(n.position)
            })
        })
    }
    /// Whether each node is a fine root's wood: a seam, or borne by one
    /// (fn-212 D4); all false on a tree grown whole.
    pub(crate) fn in_fine_roots(&self) -> Vec<bool> {
        let mut fine = vec![
            false;
            if self.seams.is_empty() {
                0
            } else {
                self.nodes.len()
            }
        ];
        if fine.is_empty() {
            return fine;
        }
        for &s in &self.seams {
            fine[s as usize] = true;
        }
        for i in 1..self.nodes.len() {
            let p = self.nodes[i].parent.unwrap() as usize;
            fine[i] |= fine[p];
        }
        fine
    }
    /// The side a fine root's bearing or sweep run seeds its frame from:
    /// its attachment's laid side, where its first node is a fine root's.
    pub(crate) fn seed_side(
        &self,
        fine: &[bool],
        attach: usize,
        first: usize,
    ) -> Option<crate::math::Vec3> {
        if !fine.get(first).copied().unwrap_or(false) {
            return None;
        }
        let f = self.frames.get(attach)?.map(f64::from);
        Some(crate::math::Vec3::new(f[3], f[4], f[5]))
    }
    /// Node `i`'s radii as the pipe model solved them, `[start, distal]`,
    /// whatever `girthHold` drew: the radii leaf-bearing decisions read.
    pub(crate) fn pipe(&self, i: usize) -> [f64; 2] {
        let n = &self.nodes[i];
        self.pipe
            .get(i)
            .copied()
            .unwrap_or([n.start_radius, n.radius])
    }
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
        if !self.pipe.is_empty() && self.pipe.len() != self.crossover {
            return Err(Error::InvalidInput("held pipe radii"));
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
