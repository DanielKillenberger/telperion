//! Consumer skeletons are parent-before-child with structural nodes before locals.
//! During a retained advance, slices may append structure after local nodes;
//! the specimen provides a cached packed layout when a consumer reads it.
//! Timeline nodes retain their birth identity and slot after their death year;
//! the legacy envelope builder still compacts. Consumer reads exclude dead nodes.
mod identity;
mod shoot;
use crate::{math::Vec3, Error, Result};
pub use identity::NodeIdentity;
pub(crate) use identity::NodeKey;
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
}
impl Tree {
    /// The wood a canopy or a twig layer measures itself against: the thickest
    /// stem leaving the root, read through whatever radius the caller trusts.
    /// The root carries every stem's pipe together, so a clump has to measure
    /// one stem rather than the combined trunk none of its shoots would ever
    /// read as slender beside; a tree on one stem measures the root itself,
    /// which IS that stem's own base, and so is left exactly where it was.
    pub(crate) fn stem_radius(&self, radius: impl Fn(usize) -> f64) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }
        let mut stems = 0;
        let mut largest = 0.0_f64;
        for (i, n) in self.nodes.iter().enumerate().skip(1) {
            if n.parent == Some(0) {
                stems += 1;
                largest = largest.max(radius(i));
            }
        }
        if stems > 1 {
            largest
        } else {
            radius(0)
        }
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
        Ok(())
    }
}
